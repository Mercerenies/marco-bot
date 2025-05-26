
use crate::bot::MarcoBot;
use super::BotCommand;
use crate::personality::generate_personality;

use serenity::prelude::*;
use serenity::model::application::CommandInteraction;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage,
                        EditInteractionResponse};
use async_trait::async_trait;

use std::fmt::Debug;

/// "Reroll" command to roll a new personality.
#[derive(Debug, Clone, Default)]
pub struct RerollCommand;

#[async_trait]
impl BotCommand for RerollCommand {
  fn get_command_name(&self) -> &str {
    "reroll"
  }

  fn get_command_desc(&self) -> &str {
    "Rerolls a new personality for Marco immediately."
  }

  async fn run_command(&self, bot: &MarcoBot, ctx: &Context, interaction: CommandInteraction) -> anyhow::Result<()> {
    let initial_response = CreateInteractionResponseMessage::default()
      .content("Rerolling...");
    interaction.create_response(&ctx.http, CreateInteractionResponse::Defer(initial_response)).await?;

    let new_personality = generate_personality(bot.client()).await?;
    let name = new_personality.name.trim().to_owned();
    {
      let mut state = bot.lock_state();
      state.set_personality(new_personality);
      state.refresh_activity(&ctx);
    }

    let final_response = EditInteractionResponse::default()
      .content(format!("Introducing {name}!"));
    interaction.edit_response(&ctx.http, final_response).await?;

    Ok(())
  }
}
