
use super::MarcoBot;
use crate::personality::generate_personality;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use async_trait::async_trait;

use std::collections::HashMap;
use std::fmt::Debug;

/// "Help" command.
#[derive(Debug, Clone, Default)]
pub struct HelpCommand;

/// "Reroll" command to roll a new personality.
#[derive(Debug, Clone, Default)]
pub struct RerollCommand;

/// Very basic implementation of Discord bot commands, via simple text
/// matching in the message content.
///
/// Derives from [`Debug`] so that it can be shown conveniently in
/// debug output.
#[async_trait]
pub trait BotCommand: Debug + Send + Sync {
  fn get_command_name(&self) -> &str;

  async fn run_command(&self, bot: &MarcoBot, ctx: &Context, message: &Message) -> anyhow::Result<()>;

  fn get_command_text(&self) -> String {
    format!("!marco {}", self.get_command_name())
  }
}

#[async_trait]
impl BotCommand for HelpCommand {
  fn get_command_name(&self) -> &str {
    "help"
  }

  async fn run_command(&self, _bot: &MarcoBot, ctx: &Context, message: &Message) -> anyhow::Result<()> {
    let help_embed = CreateEmbed::default()
      .title("Marco Bot Help")
      .description("Marco is a Discord bot written by Mercerenies. Check the link above for more details")
      .field("!marco help", "Displays this help message.", false)
      .field("!marco reroll", "Roll a new personality for Marco.", false)
      .url("https://github.com/Mercerenies/marco-bot")
      .footer(CreateEmbedFooter::new("Thank you for using Marco Bot!"));

    let response_message = CreateMessage::default()
      .embed(help_embed);

    message.channel_id.send_message(&ctx.http, response_message).await?;
    Ok(())
  }
}

#[async_trait]
impl BotCommand for RerollCommand {
  fn get_command_name(&self) -> &str {
    "reroll"
  }

  async fn run_command(&self, bot: &MarcoBot, ctx: &Context, message: &Message) -> anyhow::Result<()> {
    let new_personality = generate_personality(bot.client()).await?;
    let name = new_personality.name.trim().to_owned();
    {
      let mut state = bot.lock_state();
      state.set_personality(new_personality);
    }
    let resp = CreateMessage::default()
      .content(format!("Introducing {name}!"))
      .reference_message(message);
    message.channel_id.send_message(&ctx.http, resp).await?;
    Ok(())
  }
}

pub fn compile_commands_map<I>(commands: I) -> HashMap<String, Box<dyn BotCommand>>
where I: IntoIterator<Item = Box<dyn BotCommand>> {
  commands.into_iter()
    .map(|c| (c.get_command_text(), c))
    .collect()
}

pub fn compile_default_commands() -> HashMap<String, Box<dyn BotCommand>> {
  let default_commands_list: [Box<dyn BotCommand>; 2] = [
    Box::new(HelpCommand),
    Box::new(RerollCommand),
  ];
  compile_commands_map(default_commands_list)
}
