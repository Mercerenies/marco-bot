
pub mod message;
pub mod nicknames;
pub mod passive;

use message::MessageHistory;
use nicknames::NicknameMap;
use crate::personality::{FullPersonality, generate_personality};
use crate::openai;

use async_openai::Client;
use async_openai::config::OpenAIConfig;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{UserId, GuildId};
use serenity::model::channel::Channel;
use serenity::model::user::User;
use serenity::http::Typing;
use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::gateway::ActivityData;
use async_trait::async_trait;
use regex::Regex;

use std::sync::{Arc, Mutex, MutexGuard, LazyLock};

pub const BOT_NAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i:\bmarco\b)").unwrap());

/// An instance of this Discord bot.
///
/// Implemented as an [`Arc`], so the [`Clone`] impl for this type is
/// cheap and reference-based.
#[derive(Debug, Clone)]
pub struct MarcoBot {
  inner: Arc<MarcoBotImpl>,
}

#[derive(Debug)]
pub struct MarcoBotImpl {
  state: Mutex<MarcoBotState>,
  client: Client<OpenAIConfig>,
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
  pub nicknames: NicknameMap,
  pub last_reference: Option<chrono::DateTime<chrono::Utc>>,
  /// Whether or not any user has acknowledged this personality. If
  /// this is false, then a new personality will not passively roll.
  pub spoken_to_latest_personality: bool, // TODO This field is equivalent to self.last_reference.is_some() I think
}

pub fn gateway_intents() -> GatewayIntents {
  GatewayIntents::all()
}

impl MarcoBot {
  pub fn new(config: MarcoBotConfig) -> Self {
    let inner = MarcoBotImpl {
      state: Mutex::new(MarcoBotState::new()),
      client: Client::new(),
      config,
    };
    Self { inner: Arc::new(inner) }
  }

  pub fn client(&self) -> &Client<OpenAIConfig> {
    &self.inner.client
  }

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
      nicknames: NicknameMap::new(),
      personality: FullPersonality::default(),
      last_reference: None,
      spoken_to_latest_personality: false,
    }
  }

  pub fn refresh_activity(&self, ctx: &Context) {
    let activity_data = ActivityData::custom(&self.personality.name);
    ctx.set_activity(Some(activity_data));
  }

  pub fn set_personality(&mut self, personality: FullPersonality) {
    println!("Setting Personality: {}", personality.tagline());
    self.last_reference = None;
    self.spoken_to_latest_personality = false;
    self.personality = personality;
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
      // Ignore all messages from the bot
      return;
    }

    // Special command checks
    if !msg.author.bot {
      if msg.content == "!marco help" {
        send_help_message(&ctx, &msg).await;
        return;
      }

      if msg.content == "!marco reroll" {
        let new_personality = match generate_personality(self.client()).await {
          Ok(p) => p,
          Err(e) => {
            println!("Error from OpenAI: {:?}", e);
            return;
          }
        };
        let name = new_personality.name.trim().to_owned();
        {
          let mut state = self.lock_state();
          state.set_personality(new_personality);
          state.refresh_activity(&ctx);
        }
        let resp = CreateMessage::default()
          .content(format!("Introducing {name}!"))
          .reference_message(&msg);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, resp).await {
          println!("Error sending reroll message: {:?}", why);
        }
        return;
      }
    }

    if is_dm(&ctx, &msg).await {
      // Ignore DMs
      return;
    }

    let mut chat_completion = None;
    let mut _typing = None; // unused variable reason: Semantically-significant drop glue
    {
      let mentioned = is_bot_mentioned(bot_user_id, &msg);
      let nick = get_nick(&ctx, &msg.author, msg.guild_id).await;
      let mut state = self.lock_state();
      state.nicknames.insert(msg.author.id, nick);
      let message = message::Message {
        user: message::MessageUser::DiscordUser { user_id: msg.author.id },
        content: msg.content.to_owned(),
      };
      state.messages.push_back(message, mentioned);
      if mentioned {
        let config = openai::DeveloperPromptConfig {};
        state.spoken_to_latest_personality = true;
        state.last_reference = Some(chrono::Utc::now());
        chat_completion = Some(openai::chat_completion(
          &state.personality,
          state.messages.messages().iter(),
          state.messages.referred_messages().iter(),
          &state.nicknames,
          &config,
        ));
        _typing = Some(Typing::start(ctx.http.clone(), msg.channel_id));
      }
    }
    // Note: Drop mutex here so we don't hold it over an await boundary.
    if let Some(chat_completion) = chat_completion {
      let resp = match openai::chat(self.client(), chat_completion).await {
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

async fn send_help_message(ctx: &Context, msg: &Message) {
  let help_embed = CreateEmbed::default()
    .title("Marco Bot Help")
    .description("Marco is a Discord bot written by Mercerenies. Check the link above for more details")
    .field("!marco help", "Displays this help message.", false)
    .field("!marco reroll", "Roll a new personality for Marco.", false)
    .url("https://github.com/Mercerenies/marco-bot")
    .footer(CreateEmbedFooter::new("Thank you for using Marco Bot!"));

  let message = CreateMessage::default()
    .embed(help_embed);

  if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
    eprintln!("Error sending help message: {:?}", why);
  }
}

async fn get_nick(ctx: &Context, user: &User, guild: Option<GuildId>) -> String {
  let Some(guild) = guild else { return user.name.clone() };
  user.nick_in(ctx, guild).await.unwrap_or_else(|| user.name.clone())
}
