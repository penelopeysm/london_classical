use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Europe::London;
use london_classical::core;
use log::debug;
use serde::Serialize;
use std::fs::{create_dir_all, File};

#[derive(Debug, Serialize)]
struct Concert {
    datetime: DateTime<Utc>,
    url: String,
    venue: String,
    title: String,
    performers: Vec<core::Performer>,
    pieces: Vec<core::Piece>,
}

#[allow(dead_code)]
fn from_london_time(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    London
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .unwrap()
        .with_timezone(&Utc)
}

#[allow(dead_code)]
fn display_programme(concert: &Concert) {
    println!("Concert: {}", concert.title);
    println!("Venue: {}", concert.venue);
    println!("Date and time: {}", concert.datetime.with_timezone(&London));
    println!("Performers:");
    for performer in &concert.performers {
        match performer.instrument {
            Some(ref instrument) => println!("  {}, {}", performer.name, instrument),
            None => println!("  {}", performer.name),
        }
    }
    println!("Pieces:");
    for piece in &concert.pieces {
        println!("  {} - {}", piece.composer, piece.title);
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let client = reqwest::Client::new();

    // Fetch Wigmore concerts, first reading from $WIGMORE_MAX and defaulting to 220 if not given.
    // If you push it a bit more it starts to rate limit
    use london_classical::wigmore;
    let max_wigmore_concerts = {
        match std::env::var("WIGMORE_MAX") {
            Ok(s) => match s.as_str() {
                "all" => None,
                _ => Some(s.parse::<usize>().unwrap()),
            },
            Err(_) => Some(220),
        }
    };
    debug!("max_wigmore_concerts: {:?}", max_wigmore_concerts);
    let mut wigmore_concerts = wigmore::get_concerts(&client, max_wigmore_concerts).await;

    // Fetch Proms
    use london_classical::proms;
    let mut proms_concerts = proms::scrape(proms::PROMS_2024_URL, &client).await;

    // Concatenate and sort
    let mut full_concerts = vec![];
    full_concerts.append(&mut wigmore_concerts);
    full_concerts.append(&mut proms_concerts);
    full_concerts.sort_by_key(|concert| concert.datetime);

    // Add IDs in
    let full_concerts_with_ids: Vec<core::Concert> = full_concerts
        .into_iter()
        .map(core::add_id_to_concert)
        .collect();

    // Check uniqueness of IDs
    let mut all_ids: Vec<&str> = full_concerts_with_ids
        .iter()
        .map(|c| c.id.as_str())
        .collect();
    all_ids.sort();
    for i in 0..all_ids.len() - 1 {
        if all_ids[i] == all_ids[i + 1] {
            panic!("Duplicate ID: {}", all_ids[i]);
        }
    }

    let output_dir = env!("CARGO_MANIFEST_DIR").to_string() + "/../src/assets";
    create_dir_all(&output_dir).unwrap();
    let output_file = File::create(output_dir + "/concerts.json").unwrap();
    serde_json::to_writer_pretty(output_file, &full_concerts_with_ids).unwrap();
}
