use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROMPT_DATA: HashMap<String, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert(
            "Get Live Scores".to_string(),
            vec!["current event", "Live match", "transfer window", "Odds for all event scheduled"],
        );
        m.insert(
            "Get latest crypto charts".to_string(),
            vec!["red", "green", "blue"],
        );
        m.insert(
            "top trending movies".to_string(),
            vec!["red", "green", "blue"],
        );
        m
    };
}
