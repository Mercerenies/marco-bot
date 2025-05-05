
use super::MarcoBot;

use crate::personality::generate_personality;

use tokio_schedule::Job;
use serenity::prelude::Context;

const REROLL_TASK_MINUTES: u32 =  15;

const MINS_SINCE_LAST_MSG_TO_REROLL: i64 = 10;

/// Passive re-roll job for Marco to generate new personalities.
pub fn schedule_reroll_task(bot: MarcoBot, ctx: Context) {
  println!("Initiating reroll task...");
  let task = tokio_schedule::every(REROLL_TASK_MINUTES).minutes()
    .perform(move || {
      // I cannot wait for async closures to be stable.....
      let bot = bot.clone();
      let ctx = ctx.clone();
      async move {
        if let Err(err) = do_passive_reroll(bot, ctx).await {
          println!("Error during reroll: {:?}", err);
        }
      }
    });
  tokio::spawn(task);
}

async fn do_passive_reroll(bot: MarcoBot, ctx: Context) -> anyhow::Result<()> {
  if !should_reroll(&bot) {
    return Ok(());
  }
  println!("Passively setting personality.");
  let new_personality = generate_personality(bot.client()).await?;
  let mut state = bot.lock_state();
  state.set_personality(new_personality);
  state.refresh_activity(&ctx);
  Ok(())
}

fn should_reroll(bot: &MarcoBot) -> bool {
  let state = bot.lock_state();
  let Some(last_reference) = state.last_reference else {
    // We have never spoken to this personality. Do NOT reroll it.
    return false;
  };
  let now = chrono::Utc::now();
  (now - last_reference) > chrono::Duration::minutes(MINS_SINCE_LAST_MSG_TO_REROLL)
}
