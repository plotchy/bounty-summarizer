use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let client = Client::new();
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let url = "https://api.openai.com/v1/chat/completions";

    // Read the system prompt from the file
    let system_prompt = fs::read_to_string("prompts/SYSTEM.md")?;
    // Read the user prompt from the file
    let user_prompt = fs::read_to_string("prompts/USER.md")?;

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ]
        }))
        .send()
        .await?;

    let body = response.text().await?;
    let json: Value = serde_json::from_str(&body)?;
    if let Some(message) = json["choices"][0]["message"]["content"].as_str() {
        println!("{}", message);
    } else {
        println!("Message not found in the response.");
    }

    Ok(())
}
