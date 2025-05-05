
// Note: Nickname map is no longer used, as we simply grab the
// nickname from the Discord API directly.

use serenity::model::id::UserId;

use std::collections::HashMap;

/// Mapping from Discord user IDs to most recently known nicknames.
#[derive(Debug, Clone, Default)]
pub struct NicknameMap {
  mapping: HashMap<UserId, String>,
}

impl NicknameMap {
  /// Creates a new, empty [`NicknameMap`].
  pub fn new() -> Self {
    Self { mapping: HashMap::new() }
  }

  pub fn get(&self, id: &UserId) -> Option<&str> {
    self.mapping.get(id).map(|s| s.as_str())
  }

  pub fn insert(&mut self, id: UserId, name: String) {
    self.mapping.insert(id, name);
  }
}
