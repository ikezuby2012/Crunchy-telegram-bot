use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct TodayApiResponse {
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "awayScore")]
    pub away_score: Option<Score>,
    #[serde(rename = "awayTeam")]
    pub away_team: Team,
    #[serde(rename = "homeScore")]
    pub home_score: Option<Score>,
    #[serde(rename = "homeTeam")]
    pub home_team: Team,
    pub changes: Option<Changes>,
    #[serde(rename = "customId")]
    pub custom_id: String,
    #[serde(rename = "crowdsourcingDataDisplayEnabled")]
    pub crowdsourcing_data_display_enabled: bool,
    #[serde(rename = "crowdsourcingEnabled")]
    pub crowdsourcing_enabled: bool,
    #[serde(rename = "feedLocked")]
    pub feed_locked: bool,
    #[serde(rename = "finalResultOnly")]
    pub final_result_only: bool,
    #[serde(rename = "hasGlobalHighlights")]
    pub has_global_highlights: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    // Add fields as needed, the JSON snippet shows an empty object
    current: Option<i32>,
    display: Option<i32>,
    normaltime: Option<i32>,
    period1: Option<i32>,
    period2: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub country: Country,
    #[serde(rename = "fieldTranslations")]
    pub field_translations: FieldTranslations,
    pub id: i32,
    pub name: String,
    #[serde(rename = "nameCode")]
    pub name_code: String,
    pub national: bool,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub slug: String,
    pub sport: Sport,
    #[serde(rename = "subTeams")]
    pub sub_teams: Vec<String>,  // Assuming it's a list of strings, adjust if needed
    #[serde(rename = "teamColors")]
    pub team_colors: TeamColors,
    #[serde(rename = "type")]
    pub team_type: i32,
    #[serde(rename = "userCount")]
    pub user_count: Option<i32>,  // Made optional as it's not present for all teams
    pub disabled: Option<bool>,  // Made optional as it's not present for all teams
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub alpha2: String,
    pub alpha3: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldTranslations {
    #[serde(rename = "nameTranslation")]
    pub name_translation: NameTranslation,
    #[serde(rename = "shortNameTranslation")]
    pub short_name_translation: ShortNameTranslation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameTranslation {
    pub ar: String,
    pub ru: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortNameTranslation {
    // Add fields if needed, the JSON snippet shows an empty object
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sport {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamColors {
    pub primary: String,
    pub secondary: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Changes {
    #[serde(rename = "changeTimestamp")]
    pub change_timestamp: i64,
    pub changes: Vec<String>,
}
