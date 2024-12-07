use chrono::{DateTime, Utc};
use chrono_tz::Europe::London;
use deunicode::deunicode;
use log::info;
use regex::Regex;
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
pub struct ConcertData {
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

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Concert {
    pub id: String,
    #[serde(flatten)]
    pub concert: ConcertData,
}

pub fn add_id_to_concert(c: ConcertData) -> Concert {
    let id = format!("{}__{}", c.datetime.timestamp(), c.venue);
    // This is overkill but just in case I guess
    let id = deunicode(&id).replace(' ', "_").to_lowercase();
    let id = Regex::new(r"[^a-zA-Z0-9_]")
        .unwrap()
        .replace_all(&id, "")
        .to_string();
    Concert { id, concert: c }
}

pub fn report_concert(c: &ConcertData) {
    let london_datetime = c.datetime.with_timezone(&London);
    info!("Found {}: {}", london_datetime, c.title);
}
