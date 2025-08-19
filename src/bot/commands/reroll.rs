
use super::{BotCommand, CommandOption, get_option};
use crate::bot::MarcoBot;
use crate::personality::{generate_personality, generate_personality_from};

use serenity::prelude::*;
use serenity::model::application::{CommandInteraction, CommandOptionType, CommandDataOptionValue};
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

  fn get_command_arguments(&self) -> Vec<CommandOption> {
    vec![
      CommandOption {
        kind: CommandOptionType::String,
        name: String::from("character_name"),
        description: String::from("Name of character template to use"),
        is_required: false,
      },
    ]
  }

  async fn run_command(&self, bot: &MarcoBot, ctx: &Context, interaction: CommandInteraction) -> anyhow::Result<()> {
    let initial_response = CreateInteractionResponseMessage::default()
      .content("Rerolling...");
    interaction.create_response(&ctx.http, CreateInteractionResponse::Defer(initial_response)).await?;

    let new_personality;
    if let Some(data_value) = get_option(&interaction.data, "character_name") {
      let CommandDataOptionValue::String(data_value) = data_value else {
        panic!("Expected a string, per command arguments");
      };
      let Ok(character_name) = data_value.trim().to_lowercase().parse() else {
        let final_response = EditInteractionResponse::default()
          .content("I don't know who that is, sorry");
        interaction.edit_response(&ctx.http, final_response).await?;
        return Ok(());
      };
      new_personality = generate_personality_from(bot.client(), character_name).await?;
    } else {
      new_personality = generate_personality(bot.client()).await?;
    }
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
