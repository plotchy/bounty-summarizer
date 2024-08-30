use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::fs;
use twitter_v2::TwitterApi;
use twitter_v2::authorization::Oauth1aToken;
use std::fmt;

#[derive(Debug)]
pub enum Diff {
    Added,
    Removed,
    MaxPayoutChanged(u64, u64), // New, Previous
    LastUpdatedChanged(String), // Date
    StartDateChanged(String), // Date
    EndDateChanged(String), // Date
    AssetsAdded(String), // Asset Name
    AssetsRemoved(String), // Asset Name
}

pub const CHUNK_SIZE: usize = 2;

pub struct DiffEvent {
    name: String,
    diffs: Vec<Diff>,
}

impl fmt::Display for DiffEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for diff in &self.diffs {
            let message = match diff {
                Diff::Added => format!("🎉📈 {} has a new bounty", self.name),
                Diff::Removed => format!("🚨❌ {} removed their bounty", self.name),
                Diff::MaxPayoutChanged(new, prev) => {
                    if new > prev {
                        format!(
                            "💵🤑 Max payout increased to ${} for {}",
                            new, self.name
                        )
                    } else {
                        format!(
                            "✂️💰 Max payout reduced to ${} for {}",
                            new, self.name
                        )
                    }
                }
                Diff::LastUpdatedChanged(date) => format!(
                    "📅🔄 {} was last updated on {}",
                    self.name, date
                ),
                Diff::StartDateChanged(date) => format!(
                    "🚀📅 {} start date has been changed to {}",
                    self.name, date
                ),
                Diff::EndDateChanged(date) => format!(
                    "⏳📅 {} end date has been changed to {}",
                    self.name, date
                ),
                Diff::AssetsAdded(asset) => format!(
                    "🆕💎 {} added {} to their bounty",
                    self.name, asset
                ),
                Diff::AssetsRemoved(asset) => format!(
                    "⚠️💔 {} removed {} from their bounty",
                    self.name, asset
                ),
            };
            writeln!(f, "{}", message)?;
        }
        Ok(())
    }
}


pub fn generate_diff_events() -> Vec<DiffEvent> {
    let events = vec![
        DiffEvent {
            name: "Moonbeam Network".to_string(),
            diffs: vec![
                Diff::LastUpdatedChanged("2024-08-29T08:43:24.547".to_string()),
                Diff::MaxPayoutChanged(500000, 300000),
            ],
        },
        DiffEvent {
            name: "Teahouse Finance".to_string(),
            diffs: vec![
                Diff::LastUpdatedChanged("2024-08-28T14:25:50.055".to_string()),
                Diff::Added,
            ],
        },
        DiffEvent {
            name: "MUX".to_string(),
            diffs: vec![
                Diff::LastUpdatedChanged("2024-08-28T12:03:22.374".to_string()),
                Diff::Removed,
            ],
        },
        DiffEvent {
            name: "Morpho".to_string(),
            diffs: vec![
                Diff::MaxPayoutChanged(1944445, 100000),
                Diff::LastUpdatedChanged("2024-08-28T10:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "furucombo".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::LastUpdatedChanged("2024-08-27T09:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "rangeprotocol".to_string(),
            diffs: vec![
                Diff::Added,
                Diff::StartDateChanged("2024-08-26T08:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "delv".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::EndDateChanged("2024-08-25T07:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "eckoDEX".to_string(),
            diffs: vec![
                Diff::AssetsAdded("0x0000000000000000000000000000000000000000".to_string()),
                Diff::LastUpdatedChanged("2024-08-24T06:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "Octopus Network".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::MaxPayoutChanged(60000, 50000),
            ],
        },
        DiffEvent {
            name: "lendr".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::AssetsRemoved("0x0000000000000000000000000000000000000000".to_string()),
            ],
        },
        DiffEvent {
            name: "Defly".to_string(),
            diffs: vec![
                Diff::Added,
                Diff::LastUpdatedChanged("2024-08-23T05:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "hydradx".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::StartDateChanged("2024-08-22T04:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "Overlay".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::EndDateChanged("2024-08-21T03:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "idex".to_string(),
            diffs: vec![
                Diff::AssetsAdded("0x0000000000000000000000000000000000000000".to_string()),
                Diff::LastUpdatedChanged("2024-08-20T02:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "SpookySwap".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::MaxPayoutChanged(70000, 60000),
            ],
        },
        DiffEvent {
            name: "thorchain".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::AssetsRemoved("0x0000000000000000000000000000000000000000".to_string()),
            ],
        },
        DiffEvent {
            name: "lybrafinance".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::LastUpdatedChanged("2024-08-19T01:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "print3r".to_string(),
            diffs: vec![
                Diff::Added,
                Diff::StartDateChanged("2024-08-18T00:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "Hourglass".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::EndDateChanged("2024-08-17T23:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "meanfinance".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::AssetsAdded("0x0000000000000000000000000000000000000000".to_string()),
            ],
        },
        DiffEvent {
            name: "LandX".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::LastUpdatedChanged("2024-08-16T22:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "Composable Finance".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::MaxPayoutChanged(80000, 70000),
            ],
        },
        DiffEvent {
            name: "aloeprotocol".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::AssetsRemoved("0x0000000000000000000000000000000000000000".to_string()),
            ],
        },
        DiffEvent {
            name: "darwinia".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::LastUpdatedChanged("2024-08-15T21:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "CoW Protocol".to_string(),
            diffs: vec![
                Diff::LastUpdatedChanged("2024-08-28T21:09:20.425".to_string()),
                Diff::MaxPayoutChanged(90000, 80000),
            ],
        },
        DiffEvent {
            name: "Blackwing".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::AssetsAdded("0x0000000000000000000000000000000000000000".to_string()),
            ],
        },
        DiffEvent {
            name: "ribbon".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::LastUpdatedChanged("2024-08-14T20:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "bonfida".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::MaxPayoutChanged(100000, 90000),
            ],
        },
        DiffEvent {
            name: "Decentraland".to_string(),
            diffs: vec![
                Diff::LastUpdatedChanged("2024-08-28T20:41:14.448".to_string()),
                Diff::AssetsRemoved("0x0000000000000000000000000000000000000000".to_string()),
            ],
        },
        DiffEvent {
            name: "biconomy".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::LastUpdatedChanged("2024-08-13T19:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "Threshold".to_string(),
            diffs: vec![
                Diff::Added,
                Diff::StartDateChanged("2024-08-12T18:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "metis".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::EndDateChanged("2024-08-11T17:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "Jito".to_string(),
            diffs: vec![
                Diff::Added,
                Diff::AssetsAdded("0x0000000000000000000000000000000000000000".to_string()),
            ],
        },
        DiffEvent {
            name: "securefi".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::LastUpdatedChanged("2024-08-10T16:00:00.000".to_string()),
            ],
        },
        DiffEvent {
            name: "Swappi".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::MaxPayoutChanged(110000, 100000),
            ],
        },
        DiffEvent {
            name: "Tidal".to_string(),
            diffs: vec![
                Diff::Removed,
                Diff::AssetsRemoved("0x0000000000000000000000000000000000000000".to_string()),
            ],
        },
    ];
    events
}


/// Not working atm for auth reasons
async fn send_tweet(content: String) -> Result<(), Box<dyn std::error::Error>> {
    
    // Initialize Twitter API client
    let twitter_api_key = std::env::var("API_KEY")?;
    let twitter_api_key_secret = std::env::var("API_KEY_SECRET")?;
    let twitter_access_token = std::env::var("ACCESS_TOKEN")?;
    let twitter_access_token_secret = std::env::var("ACCESS_TOKEN_SECRET")?;
    let auth = Oauth1aToken::new(twitter_api_key, twitter_api_key_secret, twitter_access_token, twitter_access_token_secret);
    let twitter_client = TwitterApi::new(auth);
    
    let tweet = twitter_client
        .post_tweet()
        .text(content)
        .send()
        .await?;

    let tweet_data = tweet.clone().into_data();
    match tweet_data {
        Some(_) => println!("Tweet sent successfully!"),
        None => println!("Failed to send tweet. Errors: {:?}", tweet.into_errors()),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // Initialize OpenAI API client
    let client = Client::new();
    let openai_api_key = std::env::var("OPENAI_API_KEY")?;
    let url = "https://api.openai.com/v1/chat/completions";

    // Read the system prompt from the file
    let system_prompt = fs::read_to_string("prompts/SYSTEM.md")?;

    // Generate and process diff events in batches of 5
    let diff_events = generate_diff_events();
    println!("diff_events len: {}", diff_events.len());
    for chunk in diff_events.chunks(CHUNK_SIZE).take(3) { // Limiting to 3 requests for current 3 RPM limits
        let user_prompt = chunk.iter()
            .map(|event| format!("{}", event))
            .collect::<Vec<String>>()
            .join("\n");
        // println!("user_prompt: {}", user_prompt);
        let response = client.post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", openai_api_key))
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
            println!("{}\n", message);
            for event in chunk {
                println!("{}", event);
            }
            // Send the LLM message as a tweet
            // send_tweet(message.to_string()).await?; // broken atm for auth reasons
        } else {
            println!("Message not found in the response.\n");
            println!("Full response: {:?}", json);
        }

    }

    Ok(())
}
