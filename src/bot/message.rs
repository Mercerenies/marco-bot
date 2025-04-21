
//! Message history deque.

use serenity::model::id::UserId;

use std::collections::VecDeque;

/// Recent chat history that the bot is aware of.
#[derive(Debug, Clone)]
pub struct MessageHistory {
  messages: VecDeque<Message>,
  capacity: usize,
}

#[derive(Debug, Clone)]
pub struct Message {
  pub user: MessageUser,
  pub content: String,
}

/// The sender of the message, either a traditional Discord user or
/// this bot.
#[derive(Debug, Clone)]
pub enum MessageUser {
  /// A normal Discord user. This category also includes bots other
  /// than this one.
  DiscordUser { user_id: UserId },
  /// This bot, as a message sender.
  Marco { identity: String },
}

impl MessageHistory {
  pub fn new(capacity: usize) -> MessageHistory {
    MessageHistory {
      messages: VecDeque::new(),
      capacity,
    }
  }

  pub fn len(&self) -> usize {
    self.messages.len()
  }

  pub fn is_empty(&self) -> bool {
    self.messages.is_empty()
  }

  pub fn capacity(&self) -> usize {
    self.capacity
  }

  pub fn push_back(&mut self, message: Message) {
    while self.messages.len() >= self.capacity {
      self.messages.pop_front();
    }
    self.messages.push_back(message);
  }

  pub fn iter(&self) -> impl Iterator<Item = &Message> {
    self.messages.iter()
  }
}
