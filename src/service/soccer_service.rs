use chrono::{DateTime, Utc};
use reqwest::{self, Error as ReqwestError};
use serde_json::Value;
use std::env;
use std::time::SystemTime;

use crate::models::soccer::TodayApiResponse;

pub async fn today_events() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Fetching today's events");
    let rapidapi_key = env::var("RAPIDAPI_KEY").expect("RAPIDAPI_KEY must be set");
    let sys_time = SystemTime::now();
    let date_time: DateTime<Utc> = sys_time.into();
    let formatted_date = date_time.format("%Y-%m-%d").to_string();

    let client = reqwest::Client::new();
    let url = format!(
        "https://sportapi7.p.rapidapi.com/api/v1/sport/football/scheduled-events/{}",
        formatted_date
    );

    log::info!("{url}");

    let response = client
        .get(url)
        .header("x-rapidapi-key", rapidapi_key)
        .header("x-rapidapi-host", "sportapi7.p.rapidapi.com")
        .send()
        .await?;

    let body: Value = response.json().await?;
    log::info!("this is the data: {:?}", body);

    let response_object: TodayApiResponse = serde_json::from_value(body)?;

    let mut message = String::from("Today's events:\n\n");
    // for (index, event) in body.events.iter().enumerate() {
    //     message.push_str(&format!("Event {}:\n", index + 1));
    //     message.push_str(&format!("Tournament: {}\n", event.tournament.name));
    //     message.push_str(&format!(
    //         "Season: {} {}\n",
    //         event.season.name, event.season.year
    //     ));
    //     message.push_str(&format!("Status: {}\n", event.status.status_type));
    //     message.push_str(&format!(
    //         "{} vs {}\n",
    //         event.home_team.name, event.away_team.name
    //     ));
    //     message.push_str(&format!(
    //         "Score: {} - {}\n",
    //         event.home_score.current, event.away_score.current
    //     ));
    //     message.push_str("--------------------\n");
    // }

    Ok(message)
}

pub async fn transfer_window() -> Result<(), ReqwestError> {
    Ok(())
}

pub async fn current_live_match() -> Result<(), ReqwestError> {
    Ok(())
}

pub async fn events_old() -> Result<(), ReqwestError> {
    Ok(())
}
