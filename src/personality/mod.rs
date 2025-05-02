
pub mod base;
mod template;
mod tag;

pub use base::BasePersonality;
pub use tag::PersonalityTag;
pub use template::{PersonalityTemplate, FullPersonality, flesh_out_personality};

use rand::rng;
use rand::seq::IndexedRandom;
use strum::VariantArray;
use async_openai::Client;
use async_openai::config::OpenAIConfig;

pub async fn generate_personality(client: &Client<OpenAIConfig>) -> anyhow::Result<FullPersonality> {
  let mut random = rng();
  let base_personality = *BasePersonality::VARIANTS.choose(&mut random).unwrap();
  let tags_count = [(2, 0.3), (3, 0.6), (4, 0.1)].choose_weighted(&mut random, |w| w.1).unwrap().0;
  let tags = PersonalityTag::VARIANTS.choose_multiple(&mut random, tags_count).copied().collect();
  let template = PersonalityTemplate { base_personality, tags };
  flesh_out_personality(&client, &template).await
}
