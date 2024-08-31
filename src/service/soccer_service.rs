use chrono::{DateTime, NaiveDate, Utc};
use reqwest::{self, Error as ReqwestError};
use serde_json::Value;
use std::time::SystemTime;
use std::{env, result};

use crate::models::assets::MessageError;
use crate::utils::data::PROMPT_DATA;

pub async fn handle_message(message: &String) -> Result<Vec<&'static str>, MessageError> {
    PROMPT_DATA
        .get(message)
        .map(|result| {
            log::info!("data for {}: {:?}", message, result);
            result.to_vec()
        })
        .ok_or_else(|| MessageError::NoDataFound(message.to_string()))
}

pub async fn fetch_scheduled_events() -> Result<Value, ReqwestError> {
    let rapidapi_key = env::var("RAPIDAPI_KEY").expect("expect Rapid API key to be set");
    let sys_time = SystemTime::now();
    let date_time: DateTime<Utc> = sys_time.into();
    let formatted_date = date_time.format("%Y-%m-%d").to_string();

    let client = reqwest::Client::new();
    let url = format!(
        "https://sportapi7.p.rapidapi.com/api/v1/sport/football/scheduled-events/{}",
        formatted_date
    );

    let response = client
        .get(url)
        .header("x-rapidapi-key", rapidapi_key)
        .header("x-rapidapi-host", "sportapi7.p.rapidapi.com")
        .send()
        .await?;

    let body = response.json::<Value>().await?;
    Ok(body)
}
