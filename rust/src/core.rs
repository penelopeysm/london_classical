use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Piece {
    pub composer: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Performer {
    pub name: String,
    pub instrument: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Concert {
    pub datetime: DateTime<Utc>,
    pub url: String,
    pub performers: Vec<Performer>,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub is_wigmore_u35: bool,
    pub programme_pdf_url: Option<String>,
    pub pieces: Vec<Piece>,
    pub venue: String,
}
