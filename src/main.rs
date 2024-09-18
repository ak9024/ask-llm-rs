use ask_llm_rs::llm::{
    entity::{Message, Request},
    Config, LLMInterface, LLM,
};
use std::{
    env,
    io::{self, Write},
};

#[dotenvy::load]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!(">> You: ");
        io::stdout().flush().unwrap();

        let mut message = String::new();

        io::stdin().read_line(&mut message).unwrap();

        let mut messages: Vec<Message> = vec![];
        messages.push(Message {
            role: String::from("user"),
            content: message,
        });

        let config = Config {
            base_url: env::var("LLM_BASE_URL").unwrap(),
            api_key: env::var("LLM_API_KEY").unwrap(),
            req: Request {
                model: env::var("LLM_MODEL").unwrap(),
                messages,
            },
        };

        let llm: LLM = LLMInterface::new(config);
        let response = llm.chat().await?;

        for resp in response.choices {
            println!(">> AI: {}", resp.message.content)
        }
    }
}
