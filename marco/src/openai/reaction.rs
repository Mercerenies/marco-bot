
//! Helpers for determining whether the bot should react with an emoji
//! to the message.

use super::{DeveloperPromptConfig, OPENAI_MODEL};

use async_openai::Client;
use async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
                          ChatCompletionRequestMessage};
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;

const DEVELOPER_PROMPT: &str = "\
  You are Marco, a discord bot. You are roleplaying in a Discord server.

  The user will feed you a chat message. If you feel strongly about the message, \
  reply with a single emoji. Otherwise, simply say \"No reaction\".
";

/// Structure holding the parameters for an OpenAI question as to
/// whether or not the bot should react with an emoji.
///
/// Like [`super::responder::OpenAiResponder`], this structure splits
/// the act of asking OpenAI for a response into two parts, to
/// minimize the amount of time that the bot's state mutex must be
/// held.
#[derive(Debug)]
pub struct OpenAiReactionChecker {
  completion_request: CreateChatCompletionRequest,
}

impl OpenAiReactionChecker {
  pub async fn ask_question(self, client: &Client<OpenAIConfig>) -> Result<Option<String>, OpenAIError> {
    println!("Chatting with OpenAI for emoji reaction: {:?}", &self.completion_request);
    let response = client
      .chat()
      .create(self.completion_request)
      .await?;
    let text = response.choices.first().unwrap().message.content.to_owned().unwrap();
    println!("Reaction response: {text}");
    if text.to_lowercase().contains("reaction") {
      Ok(None)
    } else {
      Ok(Some(text))
    }
  }
}

pub fn emoji_reaction_completion(
  latest_chat_message: &str,
  _config: &DeveloperPromptConfig, // Currently unused
) -> OpenAiReactionChecker {
  let latest_chat_message = latest_chat_message.replace('\n', " ");
  let user_prompt = format!("\
    Latest chat message: `{latest_chat_message}`\
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
  OpenAiReactionChecker {
    completion_request: request,
  }
}
