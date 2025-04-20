
pub mod message;
pub mod nicknames;

use message::MessageHistory;
use nicknames::NicknameMap;
use crate::personality::Personality;

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
}

impl Default for MarcoBotState {
  fn default() -> Self {
    Self::new()
  }
}

#[async_trait]
impl EventHandler for MarcoBot {
  async fn message(&self, ctx: Context, msg: Message) {
    println!("Got a message!");
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}
