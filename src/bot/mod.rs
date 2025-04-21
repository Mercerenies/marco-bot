
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

  pub fn calculate_personality(&mut self, content: &str) {
    run_personality_shift(content, &mut self.personality);
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
      // Ignore all messages from the bot.
      return; // TODO Should also ignore (most) DMs
    }

    let mut chat_completion = None;
    {
      let mut state = self.lock_state();
      state.calculate_personality(&msg.content);
      // TODO Update nicknames map
      state.messages.push_back(message::Message {
        user: message::MessageUser::DiscordUser { user_id: msg.author.id },
        content: msg.content.to_owned(),
      });
      if is_bot_mentioned(bot_user_id, &msg) {
        chat_completion = Some(openai::chat_completion(&state.personality, state.messages.iter(), &state.nicknames));
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
      if let Err(why) = msg.channel_id.say(&ctx.http, resp).await {
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
