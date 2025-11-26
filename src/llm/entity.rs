use serde::{Deserialize, Serialize};

/// Request payload sent to the LLM API.
///
/// This structure represents a chat completion request that will be
/// serialized to JSON and sent to the LLM service.
#[derive(Serialize, Deserialize)]
pub struct Request {
    /// The model identifier to use (e.g., "gpt-3.5-turbo", "claude-3-5-sonnet-20240620")
    pub model: String,
    /// The list of messages in the conversation
    pub messages: Vec<Message>,
}

/// A single message in a conversation.
///
/// Represents one turn in the chat, which could be from a user,
/// assistant, or system.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// The role of the message sender (e.g., "user", "assistant", "system")
    pub role: String,
    /// The content of the message
    pub content: String,
}

/// Response received from the LLM API.
///
/// Contains the model's response along with metadata about the
/// completion request.
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    /// Unique identifier for this completion
    pub id: String,
    /// The object type (typically "chat.completion")
    pub object: String,
    /// Unix timestamp of when the completion was created
    pub created: u64,
    /// The model that was used for this completion
    pub model: String,
    /// List of completion choices returned by the model
    pub choices: Vec<Choice>,
    /// Token usage statistics for this request
    pub usage: Usage,
}

/// A single completion choice from the model.
///
/// The API may return multiple choices if requested, each representing
/// a different possible completion.
#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    /// The index of this choice in the list of choices
    pub index: u32,
    /// The message content of this choice
    pub message: Message,
    /// The reason the model stopped generating (e.g., "stop", "length")
    pub finish_reason: String,
}

/// Token usage statistics for a completion request.
///
/// Tracks how many tokens were used in both the prompt and the
/// completion, which is useful for billing and rate limiting.
#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    /// Total tokens used (prompt + completion)
    pub total_tokens: u32,
}
