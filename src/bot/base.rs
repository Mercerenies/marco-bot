
use super::message::{self, MessageHistory};
use super::passive;
use super::commands::{BotCommand, compile_default_commands};
use crate::personality::FullPersonality;
use crate::openai::DeveloperPromptConfig;
use crate::openai::responder::chat_completion;

use async_openai::Client;
use async_openai::config::OpenAIConfig;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{UserId, GuildId};
use serenity::model::channel::Channel;
use serenity::model::user::User;
use serenity::builder::CreateMessage;
use serenity::gateway::ActivityData;
use async_trait::async_trait;
use regex::Regex;

use std::sync::{Arc, Mutex, MutexGuard, LazyLock};
use std::collections::HashMap;

const BOT_NAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i:\bmarco\b)").unwrap());

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
  pub personality: FullPersonality,
  pub messages: MessageHistory,
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
  pub fn lock_state(&self) -> MutexGuard<MarcoBotState> {
    self.inner.state.lock().unwrap()
  }
}

impl MarcoBotState {
  pub const MESSAGE_HISTORY_CAPACITY: usize = 7;
  pub const MESSAGE_REFER_HISTORY_CAPACITY: usize = 4;

  pub fn new() -> Self {
    Self {
      messages: MessageHistory::new(Self::MESSAGE_REFER_HISTORY_CAPACITY, Self::MESSAGE_HISTORY_CAPACITY),
      personality: FullPersonality::default(),
      last_reference: None,
    }
  }

  pub fn refresh_activity(&self, ctx: &Context) {
    let activity_data = ActivityData::custom(&self.personality.name);
    ctx.set_activity(Some(activity_data));
  }

  pub fn set_personality(&mut self, personality: FullPersonality) {
    println!("Setting Personality: {}", personality.tagline());
    self.last_reference = None;
    self.personality = personality;
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

    // Special command checks
    if !msg.author.bot {
      if let Some(command) = self.inner.commands.get(&msg.content) {
        let res = command.run_command(self, &ctx, &msg).await;
        if let Err(err) = res {
          println!("Error while running {:?} command: {:?}", command, err);
        }
        return;
      }
    }

    if is_dm(&ctx, &msg).await {
      // Ignore DMs
      return;
    }

    let mut responder = None;
    {
      let mentioned = is_bot_mentioned(bot_user_id, &msg);
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
      state.messages.push_back(message, mentioned);
      if mentioned {
        let config = DeveloperPromptConfig {};
        state.mark_latest_reference(chrono::Utc::now());
        responder = Some(
          chat_completion(
            &state.personality,
            state.messages.messages().iter(),
            state.messages.referred_messages().iter(),
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
        let user = message::MessageUser::Marco { identity: state.personality.name.clone() };
        state.messages.push_back(message::Message {
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

  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
    {
      let state = self.lock_state();
      state.refresh_activity(&ctx);
    }
    passive::schedule_reroll_task(self.clone(), ctx);
  }
}

fn is_bot_mentioned(bot_user_id: UserId, msg: &Message) -> bool {
  BOT_NAME_RE.is_match(&msg.content) ||
    msg.mentions.iter().any(|mention| mention.id == bot_user_id)
}

async fn is_dm(ctx: &Context, msg: &Message) -> bool {
  match msg.channel(&ctx).await {
    Ok(Channel::Private(_)) => true,
    _ => false,
  }
}

async fn get_nick(ctx: &Context, user: &User, guild: Option<GuildId>) -> String {
  let Some(guild) = guild else { return user.name.clone() };
  user.nick_in(ctx, guild).await.unwrap_or_else(|| user.name.clone())
}
