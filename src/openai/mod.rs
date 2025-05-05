
//! OpenAI helpers.

pub mod responder;

// Currently unused
#[derive(Debug, Clone)]
pub struct DeveloperPromptConfig {}

pub const BASE_DEVELOPER_PROMPT: &str = "\
  You are Marco, a Discord bot. You are roleplaying in a Discord server.\n\
  1. The user will feed you a chat history and a role to play.\n\
  2. Respond in-character with one to three sentences.\n\
  3. Respond ONLY in-character with dialogue and NO other text.\n\
";

pub const OPENAI_MODEL: &str = "gpt-4o-mini";
