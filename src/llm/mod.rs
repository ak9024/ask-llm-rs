use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use std::error::Error;

pub mod entity;

/// The main LLM client that handles communication with LLM APIs.
///
/// This struct manages the HTTP client and configuration for making
/// requests to various LLM services (OpenAI, Anthropic, LocalAI, etc.).
pub struct LLM {
    config: Config,
    client: Client,
}

/// Configuration for the LLM client.
///
/// Contains the necessary information to connect to an LLM service,
/// including the API endpoint, authentication key, and request parameters.
pub struct Config {
    pub base_url: String,
    pub req: entity::Request,
    pub api_key: String,
}

/// Trait defining the interface for LLM interactions.
///
/// This trait provides a standardized way to interact with different
/// LLM providers through a common API.
#[async_trait]
pub trait LLMInterface {
    /// Creates a new LLM client instance with the given configuration.
    fn new(config: Config) -> Self;

    /// Sends a chat request to the LLM service and returns the response.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - The response cannot be parsed
    /// - The API returns an error status code
    async fn chat(&self) -> Result<entity::Response, Box<dyn Error>>;

    /// Builds the HTTP headers required for API requests.
    ///
    /// # Errors
    ///
    /// Returns an error if the API key contains invalid characters.
    fn headers(&self) -> Result<HeaderMap, Box<dyn Error>>;
}

#[async_trait]
impl LLMInterface for LLM {
    fn new(config: Config) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    async fn chat(&self) -> Result<entity::Response, Box<dyn Error>> {
        let response = self
            .client
            .post(&self.config.base_url)
            .headers(self.headers()?)
            .json(&self.config.req)
            .send()
            .await?
            .json::<entity::Response>()
            .await?;

        Ok(response)
    }

    fn headers(&self) -> Result<HeaderMap, Box<dyn Error>> {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let auth_value = HeaderValue::from_str(&format!("Bearer {}", self.config.api_key))
            .map_err(|e| format!("Invalid API key: {}", e))?;
        headers.insert(AUTHORIZATION, auth_value);

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_success() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "id": "test-id",
                    "object": "chat.completion",
                    "created": 1234567890,
                    "model": "gpt-3.5-turbo",
                    "choices": [
                        {
                            "index": 0,
                            "message": {
                                "role": "assistant",
                                "content": "Test response"
                            },
                            "finish_reason": "stop"
                        }
                    ],
                    "usage": {
                        "prompt_tokens": 10,
                        "completion_tokens": 20,
                        "total_tokens": 30
                    }
                }
            "#,
            )
            .create_async()
            .await;

        let llm: LLM = LLMInterface::new(Config {
            base_url: format!("{}/v1/chat/completions", server.url()),
            api_key: String::from("api-key"),
            req: entity::Request {
                model: "gpt-3.5-turbo".to_string(),
                messages: vec![entity::Message {
                    role: "user".to_string(),
                    content: "Test message".to_string(),
                }],
            },
        });

        let response = llm.chat().await?;

        assert_eq!(response.id, "test-id");
        assert_eq!(response.object, "chat.completion");
        assert_eq!(response.created, 1234567890);
        assert_eq!(response.model, "gpt-3.5-turbo");
        assert_eq!(response.choices.len(), 1);
        assert_eq!(response.choices[0].message.content, "Test response");
        assert_eq!(response.usage.total_tokens, 30);

        mock.assert_async().await;
        Ok(())
    }
}
