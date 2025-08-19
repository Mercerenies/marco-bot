
//! Personality template.

use super::character::BaseCharacter;
use super::tag::PersonalityTag;
use crate::openai::OPENAI_MODEL;

use async_openai::Client;
use async_openai::types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessage};
use async_openai::config::OpenAIConfig;
use regex::Regex;
use itertools::Itertools;

use std::sync::LazyLock;
use std::fmt::{self, Display};

pub const NAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Name: (.*)").unwrap());

pub const SYNOPSIS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Summary: (.*)").unwrap());

pub const BASE_DEVELOPER_PROMPT: &str = "\
  You are helping to develop characters for a roleplay session. The user will provide you with a \
  starting point and you will fill in the details.\n\
  1. Respond using the requested format.\n\
  2. All characters should have names that at least vaguely \
  resemble \"Marco\" but which fit the theme given.\n\
  3. Use the provided tags as guidance but favor creativity and \n\
  new ideas when designing characters.\
";

#[derive(Debug, Clone)]
pub struct FullPersonality {
  pub name: String,
  pub class: String,
  pub base_character: String,
  pub synopsis: String,
}

#[derive(Debug, Clone)]
pub struct PersonalityTemplate {
  pub base_character: BaseCharacter,
  pub tags: Vec<PersonalityTag>,
}

impl FullPersonality {
  pub fn tagline(&self) -> String {
    format!("{} (\"Marco\" for short) - {}, talks and behaves like {} (Quirks: {})", self.name, self.class, self.base_character, self.synopsis)
  }
}

impl PersonalityTemplate {
  fn get_user_prompt(&self) -> String {
    let base_personality = self.base_character.to_string();
    let tags = self.tags.iter()
      .map(|t| t.to_string())
      .join(", ");
    format!("\
      Base Character: {base_personality}\n\
      Tags: {tags}\n\
      \n\
      Output Format:\n\
      ```\n\
      Name: (name)\n\
      Description: (Short, a few sentences)\n\
      Summary: (At most one short sentence)\n\
      ```\
    ")
  }
}

impl Display for PersonalityTemplate {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} ({})", self.base_character.to_string(), self.tags.iter().join(", "))
  }
}

impl Default for FullPersonality {
  fn default() -> Self {
    Self {
      name: String::from("Marco"),
      base_character: String::from("ChatGPT"),
      class: String::from("AI"),
      synopsis: String::from("A helpful AI assistant"),
    }
  }
}

pub async fn flesh_out_personality(
  client: &Client<OpenAIConfig>,
  template: &PersonalityTemplate,
) -> anyhow::Result<FullPersonality> {
  let request = CreateChatCompletionRequestArgs::default()
    .model(OPENAI_MODEL)
    .n(1)
    .messages(vec![
      ChatCompletionRequestMessage::Developer(BASE_DEVELOPER_PROMPT.into()),
      ChatCompletionRequestMessage::User(template.get_user_prompt().into()),
    ])
    .build()
    .unwrap();
  println!("Chatting with OpenAI to get a new personality: {:?}", request);
  let response = client.chat().create(request).await?;
  let text = response.choices.first().unwrap().message.content.to_owned().unwrap();
  println!("OpenAI personality response: {text}");
  let name = NAME_RE.captures(&text).and_then(|c| c.get(1))
    .ok_or_else(|| anyhow::anyhow!("Failed to parse name from response"))?
    .as_str().into();
  let synopsis = SYNOPSIS_RE.captures(&text).and_then(|c| c.get(1))
    .ok_or_else(|| anyhow::anyhow!("Failed to parse synopsis from response"))?
    .as_str().trim().to_owned();
  let class = template.base_character.class().long_name().to_owned();
  let base_character = template.base_character.to_string();
  Ok(FullPersonality { name, base_character, class, synopsis })
}
