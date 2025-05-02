
use marco::bot::{MarcoBot, MarcoBotConfig, gateway_intents};
use marco::environ::get_discord_token;
use marco::personality::generate_personality;

use serenity::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let discord_token = get_discord_token();
  let intents = gateway_intents();

  let config = MarcoBotConfig {};
  //let args: Vec<String> = std::env::args().collect();

  let bot = MarcoBot::new(config);
  initialize_starting_personality(&bot).await?;
  let mut client = Client::builder(&discord_token, intents)
    .event_handler(bot)
    .await?;

  // Start listening for events by starting a single shard
  client.start().await?;

  Ok(())
}

async fn initialize_starting_personality(bot: &MarcoBot) -> anyhow::Result<()> {
  let new_personality = generate_personality(bot.client()).await?;
  let mut state = bot.lock_state();
  state.set_personality(new_personality);
  Ok(())
}
