
use crate::bot::message::{Message, MessageUser};
use crate::personality::FullPersonality;
use super::{DeveloperPromptConfig, BASE_DEVELOPER_PROMPT, OPENAI_MODEL};

use async_openai::Client;
use async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
                          ChatCompletionRequestMessage};
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use itertools::Itertools;
use regex::Regex;
use serenity::prelude::*;
use serenity::http::Typing;
use serenity::model::id::ChannelId;

use std::sync::LazyLock;

/// The AI seems to want to put a character name at the beginning of
/// each message, so we strip it.
pub const NAMED_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.{1,32}):\s+").unwrap());

pub const QUOTES_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^["“]|["”]$"#).unwrap());

/// Structure holding the parameters for an OpenAI response to a chat
/// message.
///
/// This structure splits the act of asking OpenAI for a response into
/// two parts. The first part uses the bot's current state (and
/// requires an exclusive lock on said state). The second part merely
/// queries OpenAI's chat completion framework and does NOT require a
/// lock on the bot's state.
///
/// An [`OpenAiResponder`] can also optionally hold a Discord typing
/// notification.
#[derive(Debug)]
pub struct OpenAiResponder {
  completion_request: CreateChatCompletionRequest,
  typing: Option<Typing>,
}

impl OpenAiResponder {
  pub fn with_typing_notification(mut self, ctx: &Context, channel_id: ChannelId) -> Self {
    if !self.typing.is_some() {
      self.typing = Some(Typing::start(ctx.http.clone(), channel_id));
    }
    self
  }

  pub async fn chat(self, client: &Client<OpenAIConfig>) -> Result<String, OpenAIError> {
    println!("Chatting with OpenAI: {:?}", &self.completion_request);
    let response = client
      .chat()
      .create(self.completion_request)
      .await?;
    let text = response.choices.first().unwrap().message.content.to_owned().unwrap();
    let text = NAMED_PREFIX_RE.replace_all(&text, "");
    let text = QUOTES_RE.replace_all(&text, "");
    Ok(text.to_string())
  }
}

pub fn chat_completion<'a, 'b, I1, I2>(
  marco_id: usize,
  personality: &FullPersonality,
  chat_history: I1,
  referred_chat_history: I2,
  config: &DeveloperPromptConfig,
) -> OpenAiResponder
where I1: IntoIterator<Item = &'a Message>,
      I2: IntoIterator<Item = &'b Message> {
  let personality_tagline = personality.tagline();
  let recent_messages = chat_history
    .into_iter()
    .map(|message| format!("{}: {}", message_user_name(marco_id, &message.user), message.content))
    .join("\n");
  let recent_referred_messages = referred_chat_history
    .into_iter()
    .map(|message| format!("{}: {}", message_user_name(marco_id, &message.user), message.content))
    .join("\n");
  let user_prompt = format!("\
    Your role: {personality_tagline}\n\
    \n\
    Recent Chat History:\n\
    ```\n\
    {recent_messages}\n\
    ```\
    \n\
    Recent Messages that Refer to You:\n\
    ```\n\
    {recent_referred_messages}\n\
    ```\
  ");
  let request = CreateChatCompletionRequestArgs::default()
    .model(OPENAI_MODEL)
    .n(1)
    .messages(vec![
      ChatCompletionRequestMessage::Developer(get_developer_prompt(config).into()),
      ChatCompletionRequestMessage::User(user_prompt.into()),
    ])
    .build()
    .unwrap();
  OpenAiResponder {
    completion_request: request,
    typing: None,
  }
}

fn get_developer_prompt(#[expect(unused_variables)] config: &DeveloperPromptConfig) -> String {
  String::from(BASE_DEVELOPER_PROMPT)
}

fn message_user_name(marco_id: usize, user: &MessageUser) -> String {
  match user {
    MessageUser::DiscordUser { user_id: _, user_proper_name, user_nickname } => {
      if user_nickname == user_proper_name {
        format!("User {}", user_nickname)
      } else {
        format!("User {} ({})", user_nickname, user_proper_name)
      }
    }
    MessageUser::Marco { identity_id, identity } => {
      if *identity_id == marco_id {
        // This is his current personality
        String::from("You")
      } else {
        // This is a past personality
        identity.to_owned()
      }
    }
  }
}
