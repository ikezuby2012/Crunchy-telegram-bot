use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct TrendingMovieApiResponse {
    pub results: Vec<TrendingMovieResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingMovieResult {
    pub backdrop_path: String,
    pub id: i32,
    pub title: String,
    pub original_title: String,
    pub overview: String,
    pub poster_path: String,
    pub media_type: Option<String>,
    pub adult: bool,
    pub original_language: String,
    pub popularity: f64,
    pub release_date: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: u32,
}