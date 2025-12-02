
use crate::bot::MarcoBot;
use super::{BotCommand, CommandOption};

use serenity::prelude::*;
use serenity::model::application::CommandInteraction;
use serenity::builder::{CreateEmbed, CreateEmbedFooter,
                        CreateInteractionResponse, CreateInteractionResponseMessage};
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

  fn get_command_desc(&self) -> &str {
    "Displays help message."
  }

  fn get_command_arguments(&self) -> Vec<CommandOption> {
    Vec::new()
  }

  async fn run_command(&self, _bot: &MarcoBot, ctx: &Context, interaction: CommandInteraction) -> anyhow::Result<()> {
    let help_embed = CreateEmbed::default()
      .title("Marco Bot Help")
      .description("Marco is a Discord bot written by Mercerenies. Check the link above for more details")
      .field("/help", "Displays this help message.", false)
      .field("/reroll [base]", "Roll a new personality for Marco.", false)
      .url("https://github.com/Mercerenies/marco-bot")
      .footer(CreateEmbedFooter::new("Thank you for using Marco Bot!"));

    let response_message = CreateInteractionResponseMessage::default()
      .embed(help_embed);

    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(response_message)).await?;
    Ok(())
  }
}
