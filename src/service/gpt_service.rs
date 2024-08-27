use chatgpt::prelude::*;
use reqwest::Response;
use teloxide::net::client_from_env;
use std::env;

#[tokio::main]
pub async fn main() -> Result<()> {
    let client = ChatGPT::new(env::var("GPT_API_KEY").expect("expect Open AI Key to be set"))?;

    let resonse = client.send_message("Describe the best programing language!").await?;

    println!("response is {}", resonse.message().content);

    Ok(())
}

pub async fn gpt_quick_reply(message: &String) -> Result<String> {
    println!("this is {message}");

    let client = ChatGPT::new(env::var("GPT_API_KEY").expect("expect Open AI Key to be set"))?;

    let resonse = client.send_message(message).await?;

    Ok(resonse.message().content.to_string())
}

pub async fn maintain_conversation(message: &String) -> Result<String> {
    let client = ChatGPT::new(env::var("GPT_API_KEY").expect("expect Open AI Key to be set"))?;

    let mut conversation = client.new_conversation();
    let response = conversation.send_message(message).await?;

    Ok(response.message().content.clone())
}