
use crate::bot::MarcoBot;
use super::BotCommand;
use crate::personality::generate_personality;

use serenity::prelude::*;
use serenity::model::application::CommandInteraction;
use serenity::builder::CreateMessage;
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
    todo!()
  }
/*
    let new_personality = generate_personality(bot.client()).await?;
    let name = new_personality.name.trim().to_owned();
    {
      let mut state = bot.lock_state();
      state.set_personality(new_personality);
      state.refresh_activity(&ctx);
    }
    let resp = CreateMessage::default()
      .content(format!("Introducing {name}!"))
      .reference_message(message);
    message.channel_id.send_message(&ctx.http, resp).await?;
    Ok(())
  }
*/
}
