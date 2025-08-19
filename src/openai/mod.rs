
//! OpenAI helpers.

pub mod reaction;
pub mod relevance;
pub mod responder;

// Currently unused
#[derive(Debug, Clone)]
pub struct DeveloperPromptConfig {}

pub const BASE_DEVELOPER_PROMPT: &str = "\
  You are Marco, a Discord bot. You are roleplaying in a Discord server.\n\
  1. The user will feed you a chat history and a role to play.\n\
  2. Each user message lists the user's nickname first (if they \
     have one), followed by their real name in parentheses.\n
  3. Respond in-character with a short reply. Your response should be \
     at most two short paragraphs.\n\
  4. Your voice should be immediately recognizable as belonging to \
     your character.\n\
  5. Avoid unnecessarily flowery language, unless it suits your \
     character. Remain direct and to-the-point.\n\
  6. Reply on-topic to the conversation happening in the chat\n\
  7. Respond ONLY in-character with dialogue and NO other text.\n\
";

pub const OPENAI_MODEL: &str = "gpt-4o-mini";
