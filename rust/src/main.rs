use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Europe::London;
use london_classical::core;
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
    use london_classical::proms;
    let client = reqwest::Client::new();

    use london_classical::wigmore;

    // Fetch Wigmore concerts
    let mut wigmore_concerts = wigmore::get_concerts(&client).await;

    // Fetch Proms
    let mut proms_concerts =
        proms::scrape("https://www.bbc.co.uk/events/rfbp5v/series", &client).await;

    // Concatenate and sort
    let mut full_concerts = vec![];
    full_concerts.append(&mut wigmore_concerts);
    full_concerts.append(&mut proms_concerts);
    full_concerts.sort_by_key(|concert| concert.datetime);

    create_dir_all("../src/assets").unwrap();
    let output_file = File::create("../src/assets/concerts.json").unwrap();
    serde_json::to_writer_pretty(output_file, &full_concerts).unwrap();
}
