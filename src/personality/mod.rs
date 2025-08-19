
pub mod base;
pub mod character;
mod template;
mod tag;

pub use character::BaseCharacter;
pub use tag::PersonalityTag;
pub use template::{PersonalityTemplate, FullPersonality, flesh_out_personality};

use rand::rng;
use rand::seq::IndexedRandom;
use strum::VariantArray;
use async_openai::Client;
use async_openai::config::OpenAIConfig;

pub async fn generate_personality(client: &Client<OpenAIConfig>) -> anyhow::Result<FullPersonality> {
  let base_character = *BaseCharacter::VARIANTS.choose(&mut rng()).unwrap();
  generate_personality_from(client, base_character).await
}

pub async fn generate_personality_from(
  client: &Client<OpenAIConfig>,
  base_character: BaseCharacter,
) -> anyhow::Result<FullPersonality> {
  let template = {
    let mut random = rng();
    let tags_count = [(1, 0.6), (2, 0.4)].choose_weighted(&mut random, |w| w.1).unwrap().0;
    let tags = PersonalityTag::VARIANTS.choose_multiple(&mut random, tags_count).copied().collect();
    PersonalityTemplate { base_character, tags }
  };
  println!("Generating personality starting with template: {}", template);
  flesh_out_personality(&client, &template).await
}
