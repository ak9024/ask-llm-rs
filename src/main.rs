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
    // Load environment variables and validate them upfront
    let base_url = env::var("LLM_BASE_URL")
        .map_err(|_| "LLM_BASE_URL environment variable is not set")?;
    let api_key = env::var("LLM_API_KEY")
        .map_err(|_| "LLM_API_KEY environment variable is not set")?;
    let model = env::var("LLM_MODEL")
        .map_err(|_| "LLM_MODEL environment variable is not set")?;

    println!("Welcome to ask-llm-rs! Type 'quit' or 'exit' to quit.");

    loop {
        print!(">> You: ");
        io::stdout().flush()?;

        let mut message = String::new();
        io::stdin().read_line(&mut message)?;

        // Trim whitespace from user input
        let message = message.trim();

        // Allow user to exit gracefully
        if message.eq_ignore_ascii_case("quit") || message.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Skip empty messages
        if message.is_empty() {
            continue;
        }

        let messages: Vec<Message> = vec![Message {
            role: String::from("user"),
            content: message.to_string(),
        }];

        let config = Config {
            base_url: base_url.clone(),
            api_key: api_key.clone(),
            req: Request {
                model: model.clone(),
                messages,
            },
        };

        let llm: LLM = LLMInterface::new(config);
        let response = llm.chat().await?;

        for resp in response.choices {
            println!(">> AI: {}", resp.message.content)
        }
    }

    Ok(())
}
