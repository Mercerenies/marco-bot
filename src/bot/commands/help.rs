
use crate::bot::MarcoBot;
use super::BotCommand;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use async_trait::async_trait;

use std::fmt::Debug;

/// "Help" command.
#[derive(Debug, Clone, Default)]
pub struct HelpCommand;

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
