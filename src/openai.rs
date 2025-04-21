
//! OpenAI helpers.

use crate::bot::nicknames::NicknameMap;
use crate::bot::message::{Message, MessageUser};
use crate::personality::Personality;

use async_openai::Client;
use async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
                          ChatCompletionRequestMessage};
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use itertools::Itertools;
use regex::Regex;

use std::sync::LazyLock;

pub const DEVELOPER_PROMPT: &str = "\
  You are roleplaying in a Discord server. The user will feed you a chat history and \
  a role to play. Respond in-character with one to three sentences. Respond ONLY with
  character dialogue and NO other text.\
";

/// The AI seems to want to put a character name at the beginning of
/// each message, so we strip it.
pub const NAMED_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.{1,32}):\s+").unwrap());

pub const QUOTES_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^"|"$"#).unwrap());

pub fn chat_completion<'a, I>(
  personality: &Personality,
  chat_history: I,
  mapping: &NicknameMap,
) -> CreateChatCompletionRequest
where I: IntoIterator<Item = &'a Message> {
  let recent_messages = chat_history
    .into_iter()
    .map(|message| format!("{}: {}", message_user_name(&message.user, mapping), message.content))
    .join("\n");
  let user_prompt = format!("\
    Your role: {personality}\n\
    Recent Chat History:\n\
    ```\n\
    {recent_messages}\n\
    ```\
  ");
  CreateChatCompletionRequestArgs::default()
    .model("gpt-4o-mini")
    .n(1)
    .messages(vec![
      ChatCompletionRequestMessage::Developer(DEVELOPER_PROMPT.into()),
      ChatCompletionRequestMessage::User(user_prompt.into()),
    ])
    .build()
    .unwrap()
}

pub async fn chat(
  client: &Client<OpenAIConfig>,
  req: CreateChatCompletionRequest,
) -> Result<String, OpenAIError> {
  println!("Chatting with OpenAI: {:?}", &req);
  let response = client
    .chat()
    .create(req)
    .await?;
  let text = response.choices.first().unwrap().message.content.to_owned().unwrap();
  let text = NAMED_PREFIX_RE.replace_all(&text, "");
  let text = QUOTES_RE.replace_all(&text, "");
  Ok(text.to_string())
}

fn message_user_name(user: &MessageUser, mapping: &NicknameMap) -> String {
  match user {
    MessageUser::DiscordUser { user_id } =>
      mapping.get(&user_id).unwrap_or("User").to_owned(),
    MessageUser::Marco { identity } =>
      identity.clone(),
  }
}
