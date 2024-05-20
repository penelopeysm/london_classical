use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct Piece {
    pub composer: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct Performer {
    pub name: String,
    pub instrument: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct Concert {
    pub datetime: DateTime<Utc>,
    pub url: String,
    pub performers: Vec<Performer>,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub programme_pdf_url: Option<String>,
    pub pieces: Vec<Piece>,
    pub venue: String,
    pub min_price: Option<u32>, // pennies
    pub max_price: Option<u32>, // pennies

    pub is_wigmore_u35: bool,
    pub is_prom: bool,
}
