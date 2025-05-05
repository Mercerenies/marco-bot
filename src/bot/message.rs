
//! Message history deque.

use crate::util::CapacityDeque;

use serenity::model::id::UserId;

/// Recent chat history that the bot is aware of.
#[derive(Debug, Clone)]
pub struct MessageHistory {
  recent_referred_messages: CapacityDeque<Message>,
  recent_messages: CapacityDeque<Message>,
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
  DiscordUser { user_id: UserId, user_proper_name: String, user_nickname: String },
  /// This bot, as a message sender.
  Marco { identity: String },
}

impl MessageHistory {
  pub fn new(referred_cap: usize, regular_cap: usize) -> MessageHistory {
    MessageHistory {
      recent_referred_messages: CapacityDeque::new(referred_cap),
      recent_messages: CapacityDeque::new(regular_cap),
    }
  }

  pub fn push_back(&mut self, message: Message, referred: bool) {
    if referred {
      self.recent_referred_messages.push_back(message.clone());
    }
    self.recent_messages.push_back(message);
  }

  pub fn messages(&self) -> &CapacityDeque<Message> {
    &self.recent_messages
  }

  pub fn referred_messages(&self) -> &CapacityDeque<Message> {
    &self.recent_referred_messages
  }
}
