
use crate::bot::MarcoBot;
use super::BotCommand;
use crate::personality::generate_personality;

use serenity::prelude::*;
use serenity::model::channel::Message;
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

  async fn run_command(&self, bot: &MarcoBot, ctx: &Context, message: &Message) -> anyhow::Result<()> {
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
}
