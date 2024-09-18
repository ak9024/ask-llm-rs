use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use std::error::Error;

pub mod entity;

pub struct LLM {
    pub config: Config,
    pub client: Client,
}

pub struct Config {
    pub base_url: String,
    pub req: entity::Request,
    pub api_key: String,
}

#[async_trait]
pub trait LLMInterface {
    fn new(config: Config) -> Self;
    async fn chat(&self) -> Result<entity::Response, Box<dyn Error>>;
    fn headers(&self) -> HeaderMap;
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
            .headers(self.headers())
            .json(&self.config.req)
            .send()
            .await?
            .json::<entity::Response>()
            .await?;

        Ok(response)
    }

    fn headers(&self) -> HeaderMap {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.config.api_key)).unwrap(),
        );

        headers
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
