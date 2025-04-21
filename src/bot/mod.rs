
pub mod message;
pub mod nicknames;

use message::MessageHistory;
use nicknames::NicknameMap;
use crate::personality::{Personality, run_personality_shift};

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use async_trait::async_trait;

use std::sync::{Mutex, MutexGuard};

/// An instance of this Discord bot.
#[derive(Debug)]
pub struct MarcoBot {
  state: Mutex<MarcoBotState>,
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
      return;
    }

    {
      let mut state = self.lock_state();
      state.calculate_personality(&msg.content);
    }

    // TODO Respond
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}
