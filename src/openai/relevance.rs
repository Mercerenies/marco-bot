
//! Helpers for determining whether a message is relevant.

use super::{DeveloperPromptConfig, OPENAI_MODEL};
use crate::personality::FullPersonality;

use async_openai::Client;
use async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
                          ChatCompletionRequestMessage};
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use regex::Regex;

use std::sync::LazyLock;

const DEVELOPER_PROMPT: &str = "\
  You are Marco, a discord bot. You are roleplaying in a Discord server. \
  The user will feed you a chat message and ask you a question. Answer with \
  a simple \"Yes\" or \"No\" and no other output.\
";

/// Regex to strip direct mentions.
const MENTION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<@\d+>\s+").unwrap());

/// Structure holding the parameters for an OpenAI question as to
/// whether or not the bot should respond.
///
/// Like [`super::responder::OpenAiResponder`], this structure splits
/// the act of asking OpenAI for a response into two parts, to
/// minimize the amount of time that the bot's state mutex must be
/// held.
#[derive(Debug)]
pub struct OpenAiRelevanceChecker {
  completion_request: CreateChatCompletionRequest,
}

impl OpenAiRelevanceChecker {
  pub async fn ask_question(self, client: &Client<OpenAIConfig>) -> Result<bool, OpenAIError> {
    println!("Chatting with OpenAI for relevance question: {:?}", &self.completion_request);
    let response = client
      .chat()
      .create(self.completion_request)
      .await?;
    let text = response.choices.first().unwrap().message.content.to_owned().unwrap();
    println!("Relevance response: {text}");
    if text.to_lowercase().contains("yes") {
      Ok(true)
    } else if text.to_lowercase().contains("no") {
      Ok(false)
    } else if text.to_lowercase().contains("y") {
      Ok(true)
    } else {
      Ok(false)
    }
  }
}

pub fn relevance_completion(
  personality: &FullPersonality,
  latest_chat_message: &str,
  _config: &DeveloperPromptConfig, // Currently unused
) -> OpenAiRelevanceChecker {
  let personality_name = &personality.name;
  let latest_chat_message = latest_chat_message.replace('\n', " ");
  let latest_chat_message = MENTION_RE.replace_all(&latest_chat_message, "");
  let user_prompt = format!("\
    Your character: {personality_name} (\"Marco\" for short)\n\
    Latest chat message: `{latest_chat_message}`\n\
    \n\
    Does the above message directly address your character \
    (\"{personality_name}\" or \"Marco\") by name? Do NOT \
    respond YES if the message is a passive or generic comment that does \
    not mention your name.
  ");
  let request = CreateChatCompletionRequestArgs::default()
    .model(OPENAI_MODEL)
    .n(1)
    .messages(vec![
      ChatCompletionRequestMessage::Developer(DEVELOPER_PROMPT.into()),
      ChatCompletionRequestMessage::User(user_prompt.into()),
    ])
    .build()
    .unwrap();
  OpenAiRelevanceChecker {
    completion_request: request,
  }
}
