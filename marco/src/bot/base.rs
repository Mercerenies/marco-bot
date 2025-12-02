
use super::message::{self, MessageHistory};
use super::passive;
use super::commands::{BotCommand, compile_default_commands};
use crate::personality::FullPersonality;
use crate::openai::DeveloperPromptConfig;
use crate::openai::responder::chat_completion;
use crate::openai::relevance::relevance_completion;
use crate::openai::reaction::emoji_reaction_completion;

use async_openai::Client;
use async_openai::config::OpenAIConfig;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{UserId, GuildId, ChannelId, MessageId};
use serenity::model::channel::{Channel, ReactionType};
use serenity::model::user::User;
use serenity::model::application::{Command, Interaction, CommandInteraction};
use serenity::builder::{CreateMessage, CreateCommand, CreateCommandOption,
                        CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::gateway::ActivityData;
use async_trait::async_trait;

use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::HashMap;
use std::env;

/// An instance of this Discord bot.
///
/// Implemented as an [`Arc`], so the [`Clone`] impl for this type is
/// cheap and reference-based.
#[derive(Debug, Clone)]
pub struct MarcoBot {
  inner: Arc<MarcoBotImpl>,
}

#[derive(Debug)]
struct MarcoBotImpl {
  state: Mutex<MarcoBotState>,
  client: Client<OpenAIConfig>,
  commands: HashMap<String, Box<dyn BotCommand>>,
  #[expect(dead_code)] // Currently, config is not used :)
  config: MarcoBotConfig,
}

// Currently unused
#[derive(Debug)]
pub struct MarcoBotConfig {}

/// An instance of this Discord bot's current state.
#[derive(Debug)]
pub struct MarcoBotState {
  /// Incremented each time Marco generates a new personality.
  pub personality_id: usize,
  pub personality: FullPersonality,
  pub messages: HashMap<ChannelId, MessageHistory>,
  pub last_reference: Option<chrono::DateTime<chrono::Utc>>,
}

pub fn gateway_intents() -> GatewayIntents {
  GatewayIntents::all()
}

impl MarcoBot {
  /// Creates a new instance of this Discord bot, with the given
  /// configuration.
  pub fn new(config: MarcoBotConfig) -> Self {
    let inner = MarcoBotImpl {
      state: Mutex::new(MarcoBotState::new()),
      client: Client::new(),
      commands: compile_default_commands(),
      config,
    };
    Self { inner: Arc::new(inner) }
  }

  pub fn client(&self) -> &Client<OpenAIConfig> {
    &self.inner.client
  }

  /// Locks the mutex for the bot's state and returns the guard.
  ///
  /// This method will panic if the mutex is poisoned.
  pub fn lock_state(&self) -> MutexGuard<'_, MarcoBotState> {
    self.inner.state.lock().unwrap()
  }

  async fn is_message_relevant(
    &self,
    bot_user_id: UserId,
    msg: &Message,
  ) -> bool {
    if msg.mentions.iter().any(|mention| mention.id == bot_user_id) {
      return true;
    }
    let relevance_checker = {
      let state = self.lock_state();
      let config = DeveloperPromptConfig {};
      relevance_completion(&state.personality, &msg.content, &config)
    };
    match relevance_checker.ask_question(self.client()).await {
      Ok(response) => response,
      Err(err) => {
        println!("Error occurred while checking message relevance: {:?}", err);
        false
      }
    }
  }

  async fn register_commands(&self, ctx: &Context) {
    fn compile_command(command: &dyn BotCommand) -> CreateCommand {
      let args = command.get_command_arguments()
        .into_iter()
        .map(CreateCommandOption::from)
        .collect();
      CreateCommand::new(command.get_command_name())
        .description(command.get_command_desc())
        .set_options(args)
    }

    let commands: Vec<_> = self.inner.commands.values()
      .map(|cmd| compile_command(cmd.as_ref()))
      .collect();
    if cfg!(debug_assertions) {
      // Use guild commands for debug purposes.
      let guild_id = GuildId::new(
        env::var("DISCORD_DEBUG_GUILD_ID").ok()
          .and_then(|id| id.parse().ok())
          .expect("DISCORD_DEBUG_GUILD_ID must be set to a valid u64"),
      );
      let Ok(guild) = ctx.http.get_guild(guild_id).await else {
        panic!("Guild ID {} not found", guild_id);
      };
      let Ok(_) = guild.set_commands(&ctx.http, commands).await else {
        panic!("Failed to register commands for guild {}", guild_id);
      };
    } else {
      let Ok(_) = Command::set_global_commands(&ctx.http, commands).await else {
        panic!("Failed to register global commands");
      };
    }
  }
}

impl MarcoBotState {
  pub const MESSAGE_HISTORY_CAPACITY: usize = 7;
  pub const MESSAGE_REFER_HISTORY_CAPACITY: usize = 4;

  pub fn new() -> Self {
    Self {
      messages: HashMap::new(),
      personality_id: 0,
      personality: FullPersonality::default(),
      last_reference: None,
    }
  }

  fn make_new_message_history() -> MessageHistory {
    MessageHistory::new(Self::MESSAGE_REFER_HISTORY_CAPACITY, Self::MESSAGE_HISTORY_CAPACITY)
  }

  pub fn refresh_activity(&self, ctx: &Context) {
    let activity_data = ActivityData::custom(&self.personality.name);
    ctx.set_activity(Some(activity_data));
  }

  pub fn set_personality(&mut self, personality: FullPersonality) {
    println!("Setting Personality: {}", personality.tagline());
    self.last_reference = None;
    self.personality_id = self.personality_id.wrapping_add(1);
    self.personality = personality;
    for message_history in self.messages.values_mut() {
      message_history.referred_messages_mut().clear();
    }
  }

  pub fn mark_latest_reference(&mut self, date: chrono::DateTime<chrono::Utc>) {
    self.last_reference = Some(date);
  }

  pub fn last_reference(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
    self.last_reference.as_ref()
  }

  pub fn spoken_to_latest_personality(&self) -> bool {
    self.last_reference.is_some()
  }
}

impl Default for MarcoBotState {
  fn default() -> Self {
    Self::new()
  }
}

#[async_trait]
impl EventHandler for MarcoBot {
  async fn message(&self, ctx: Context, msg: Message) {
    let bot_user_id = ctx.cache.current_user().id;
    if msg.author.id == bot_user_id {
      // Ignore all messages from the bot itself
      return;
    }

    if is_dm(&ctx, &msg).await {
      // Ignore DMs
      return;
    }

    // Spawn up an independent task to see if the bot should react
    // (via Discord emoji) to the message.
    tokio::spawn(do_reaction_flow(self.clone(), ctx.clone(), msg.content.to_owned(), msg.channel_id, msg.id));

    if is_thread(&ctx, &msg).await {
      // Ignore thread messages (except for emoji reacts)
      return;
    }

    let mut responder = None;
    {
      let relevant = self.is_message_relevant(bot_user_id, &msg).await;
      let nick = get_nick(&ctx, &msg.author, msg.guild_id).await;
      let mut state = self.lock_state();
      let message = message::Message {
        user: message::MessageUser::DiscordUser {
          user_id: msg.author.id,
          user_proper_name: msg.author.name.clone(),
          user_nickname: nick,
        },
        content: msg.content.to_owned(),
      };
      let message_history = state.messages.entry(msg.channel_id)
        .or_insert_with(MarcoBotState::make_new_message_history);
      message_history.push_back(message, relevant);
      if relevant {
        let config = DeveloperPromptConfig {};
        state.mark_latest_reference(chrono::Utc::now());
        // Re-borrow as immutable.
        let message_history = state.messages.get(&msg.channel_id).unwrap();
        responder = Some(
          chat_completion(
            state.personality_id,
            &state.personality,
            message_history.messages().iter(),
            message_history.referred_messages().iter(),
            &config,
          ).with_typing_notification(&ctx, msg.channel_id),
        );
      }
    }
    // Note: Drop mutex here so we don't hold it over an OpenAI await boundary.
    if let Some(responder) = responder {
      let resp = match responder.chat(self.client()).await {
        Ok(resp) => resp,
        Err(e) => {
          println!("Error from OpenAI: {:?}", e);
          return;
        }
      };
      {
        let mut state = self.lock_state();
        let user = message::MessageUser::Marco {
          identity_id: state.personality_id,
          identity: state.personality.name.clone(),
        };
        let messages = state.messages.entry(msg.channel_id)
          .or_insert_with(MarcoBotState::make_new_message_history);
        messages.push_back(message::Message {
          user,
          content: resp.clone(),
        }, true);
      }
      let mut resp = CreateMessage::default()
        .content(resp);
      // I would love to reply to all messages, but replying to bots
      // causes an infinite loop WAY too often. This is a stop-gap.
      if !msg.author.bot {
        resp = resp.reference_message(&msg);
      }
      if let Err(why) = msg.channel_id.send_message(&ctx.http, resp).await {
        println!("Error sending message: {:?}", why);
      }
    }
  }

  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    let Interaction::Command(interaction) = interaction else {
      eprintln!("Got unknown interaction {:?}... ignoring", interaction);
      return;
    };
    let Some(relevant_command) = self.inner.commands.get(&interaction.data.name) else {
      eprintln!("Got unknown command {:?}... ignoring", interaction);
      if let Err(why) = send_invalid_command_response(&ctx, interaction).await {
        println!("Error sending invalid command response: {:?}", why);
      }
      return;
    };
    if let Err(why) = relevant_command.run_command(self, &ctx, interaction).await {
      println!("Error in command {:?}: {}", relevant_command, why);
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
    {
      let state = self.lock_state();
      state.refresh_activity(&ctx);
    }
    self.register_commands(&ctx).await;
    passive::schedule_reroll_task(self.clone(), ctx);
  }
}

async fn is_dm(ctx: &Context, msg: &Message) -> bool {
  match msg.channel(&ctx).await {
    Ok(Channel::Private(_)) => true,
    _ => false,
  }
}

async fn is_thread(ctx: &Context, msg: &Message) -> bool {
  match msg.channel(&ctx).await {
    Ok(Channel::Guild(ch)) => ch.thread_metadata.is_some(),
    _ => false,
  }
}

async fn get_nick(ctx: &Context, user: &User, guild: Option<GuildId>) -> String {
  let Some(guild) = guild else { return user.name.clone() };
  user.nick_in(ctx, guild).await.unwrap_or_else(|| user.name.clone())
}

async fn do_reaction_flow(
  bot: MarcoBot,
  ctx: Context,
  message_content: String,
  channel_id: ChannelId,
  message_id: MessageId,
) {
  async fn do_reaction_flow_impl(
    bot: MarcoBot,
    ctx: Context,
    message_content: String,
    channel_id: ChannelId,
    message_id: MessageId,
  ) -> anyhow::Result<()> {
    let reaction_checker = emoji_reaction_completion(&message_content, &DeveloperPromptConfig {});
    let emoji_response = reaction_checker.ask_question(bot.client()).await?;
    let Some(emoji_response) = emoji_response else {
      return Ok(()); // Nothing to react with.
    };
    ctx.http
      .create_reaction(channel_id, message_id, &ReactionType::Unicode(emoji_response))
      .await?;
    Ok(())
  }
  if let Err(err) = do_reaction_flow_impl(bot, ctx, message_content, channel_id, message_id).await {
    println!("Error while doing reaction flow: {:?}", err);
  }
}

async fn send_invalid_command_response(ctx: &Context, interaction: CommandInteraction) -> serenity::Result<()> {
  interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
    CreateInteractionResponseMessage::new().content("I don't understand that command."))
  ).await
}
