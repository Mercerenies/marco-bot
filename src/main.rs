
use marco::bot::{MarcoBot, MarcoBotConfig, gateway_intents};
use marco::environ::get_discord_token;

use serenity::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let discord_token = get_discord_token();
  let intents = gateway_intents();

  let mut config = MarcoBotConfig {
    pet_name_mode: false,
  };
  let args: Vec<String> = std::env::args().collect();
  config.pet_name_mode = args.contains(&"--pet-name".to_string());

  let bot = MarcoBot::new_random(config);
  let mut client = Client::builder(&discord_token, intents)
    .event_handler(bot)
    .await?;

  // Start listening for events by starting a single shard
  client.start().await?;

  Ok(())
}
