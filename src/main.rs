
use marco::bot::{MarcoBot, gateway_intents};
use marco::environ::get_discord_token;

use serenity::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let discord_token = get_discord_token();
  let intents = gateway_intents();

  let bot = MarcoBot::new();
  let mut client = Client::builder(&discord_token, intents)
    .event_handler(bot)
    .await?;

  // Start listening for events by starting a single shard
  client.start().await?;

  Ok(())
}
