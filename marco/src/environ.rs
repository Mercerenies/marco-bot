
//! Environment variables for the Marco bot.

use std::env;

pub const DISCORD_TOKEN: &str = "DISCORD_TOKEN";
pub const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

pub fn get_discord_token() -> String {
  env::var(DISCORD_TOKEN)
    .expect("Expected a Discord token in the environment")
}

pub fn get_openai_api_key() -> String {
  env::var(OPENAI_API_KEY)
    .expect("Expected an OpenAI API key in the environment")
}
