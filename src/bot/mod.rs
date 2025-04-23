
pub mod message;
pub mod nicknames;

use message::MessageHistory;
use nicknames::NicknameMap;
use crate::personality::{Personality, run_personality_shift};
use crate::openai;

use async_openai::Client;
use async_openai::config::OpenAIConfig;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::model::channel::Channel;
use serenity::http::Typing;
use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use async_trait::async_trait;
use regex::Regex;

use std::sync::{Mutex, MutexGuard, LazyLock};

pub const BOT_NAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i:\bmarco\b)").unwrap());

/// An instance of this Discord bot.
#[derive(Debug)]
pub struct MarcoBot {
  state: Mutex<MarcoBotState>,
  client: Client<OpenAIConfig>,
}

/// An instance of this Discord bot's current state.
#[derive(Debug)]
pub struct MarcoBotState {
  pub personality: Personality,
  pub messages: MessageHistory,
  pub nicknames: NicknameMap,
}

pub fn gateway_intents() -> GatewayIntents {
  GatewayIntents::all()
}

impl MarcoBot {
  pub fn new() -> Self {
    Self {
      state: Mutex::new(MarcoBotState::new()),
      client: Client::new(),
    }
  }

  pub fn new_random() -> Self {
    Self {
      state: Mutex::new(MarcoBotState::new_random()),
      client: Client::new(),
    }
  }

  pub fn lock_state(&self) -> MutexGuard<MarcoBotState> {
    self.state.lock().unwrap()
  }
}

impl MarcoBotState {
  pub const MESSAGE_HISTORY_CAPACITY: usize = 10;

  pub fn new() -> Self {
    Self {
      messages: MessageHistory::new(Self::MESSAGE_HISTORY_CAPACITY),
      nicknames: NicknameMap::new(),
      personality: Personality::default(),
    }
  }

  pub fn new_random() -> Self {
    let personality = Personality::generate_random();
    println!("Starting Personality: {}", personality);
    Self {
      messages: MessageHistory::new(Self::MESSAGE_HISTORY_CAPACITY),
      nicknames: NicknameMap::new(),
      personality,
    }
  }

  pub fn calculate_personality(&mut self, content: &str) {
    let changed = run_personality_shift(content, &mut self.personality);
    if changed {
      println!("New Personality: {}", self.personality);
    }
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

    if msg.content == "!marco help" {
      send_help_message(&ctx, &msg).await;
      return;
    }

    if is_dm(&ctx, &msg).await {
      // Ignore DMs
      return;
    }

    let mut chat_completion = None;
    let mut typing = None;
    {
      let mut state = self.lock_state();
      state.calculate_personality(&msg.content);
      state.nicknames.insert(msg.author.id, msg.author.name.clone());
      state.messages.push_back(message::Message {
        user: message::MessageUser::DiscordUser { user_id: msg.author.id },
        content: msg.content.to_owned(),
      });
      if is_bot_mentioned(bot_user_id, &msg) {
        chat_completion = Some(openai::chat_completion(&state.personality, state.messages.iter(), &state.nicknames));
        typing = Some(Typing::start(ctx.http.clone(), msg.channel_id));
      }
    }
    // Note: Drop mutex here so we don't hold it over an await boundary.
    if let Some(chat_completion) = chat_completion {
      let resp = match openai::chat(&self.client, chat_completion).await {
        Ok(resp) => resp,
        Err(e) => {
          println!("Error from OpenAI: {:?}", e);
          return;
        }
      };
      {
        let mut state = self.lock_state();
        let user = message::MessageUser::Marco { identity: state.personality.marco_name().to_owned() };
        state.messages.push_back(message::Message {
          user,
          content: resp.clone(),
        });
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

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
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
    .url("https://github.com/Mercerenies/marco-bot")
    .footer(CreateEmbedFooter::new("Thank you for using Marco Bot!"));

  let message = CreateMessage::default()
    .embed(help_embed);

  if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
    eprintln!("Error sending help message: {:?}", why);
  }
}
