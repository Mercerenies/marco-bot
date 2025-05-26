
mod help;
mod reroll;

pub use help::HelpCommand;
pub use reroll::RerollCommand;

use super::MarcoBot;

use serenity::prelude::*;
use serenity::model::application::CommandInteraction;
use async_trait::async_trait;

use std::collections::HashMap;
use std::fmt::Debug;

/// Very basic implementation of Discord bot commands, via simple text
/// matching in the message content.
///
/// Derives from [`Debug`] so that it can be shown conveniently in
/// debug output.
#[async_trait]
pub trait BotCommand: Debug + Send + Sync {
  fn get_command_name(&self) -> &str;

  fn get_command_desc(&self) -> &str;

  async fn run_command(&self, bot: &MarcoBot, ctx: &Context, interaction: CommandInteraction) -> anyhow::Result<()>;
}

pub fn compile_commands_map<I>(commands: I) -> HashMap<String, Box<dyn BotCommand>>
where I: IntoIterator<Item = Box<dyn BotCommand>> {
  commands.into_iter()
    .map(|c| (c.get_command_name().to_owned(), c))
    .collect()
}

pub fn compile_default_commands() -> HashMap<String, Box<dyn BotCommand>> {
  let default_commands_list: [Box<dyn BotCommand>; 2] = [
    Box::new(HelpCommand),
    Box::new(RerollCommand),
  ];
  compile_commands_map(default_commands_list)
}
