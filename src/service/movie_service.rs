use reqwest::{self, Error as ReqwestError, Response};
use serde_json::Value;
use std::{env, fmt::format};

use crate::models::movie::TrendingMovieApiResponse;

const MOVIE_BASE_URL: &str = "https://api.themoviedb.org";

async fn fetch_movies(endpoint: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let movie_api_token = env::var("MOVIE_ACCESS_TOKEN")
        .map_err(|_| "MOVIE_ACCESS_TOKEN must be set")?;

    let client = reqwest::Client::new();
    let url = format!("{}/3{}", MOVIE_BASE_URL, endpoint);

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", movie_api_token))
        .send()
        .await?;

    let body: Value = response.json().await?;
    let response_object: TrendingMovieApiResponse = serde_json::from_value(body)?;

    let mut response: Vec<String> = Vec::new();
    
    for (index, movie) in response_object.results.iter().enumerate() {
        let mut message = String::new();
        message.push_str(&format!("Movie {}:\n", index + 1));
        message.push_str(&format!("Title: {}\n", movie.title));
        message.push_str(&format!("Original Title: {}\n", movie.original_title));
        message.push_str(&format!("Overview: {}\n", movie.overview));
        message.push_str(&format!("Adult: {}\n", movie.adult));
        message.push_str(&format!("Original Language: {}\n", movie.original_language));
        message.push_str(&format!("Release Date: {}\n", movie.release_date));
        message.push_str("--------------------\n");

        response.push(message);
    }

    Ok(response)
}

pub async fn trending_movie() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Fetching trending movie");
    let movies = fetch_movies("/trending/movie/day?language=en-US").await?;
    Ok(movies)
}

pub async fn popular_movie() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Fetching popular movie");
    let movies = fetch_movies("/movie/popular").await?;
    Ok(movies)
}

pub async fn get_movies_in_theatres() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Get a list of movies that are currently in theatres.");
    let movies = fetch_movies("/movie/now_playing").await?;
    Ok(movies)
}

pub async fn upcoming_movie() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Get a list of movies that are being released soon..");
    let movies = fetch_movies("/movie/upcoming").await?;
    Ok(movies)
}