use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Europe::London;
use serde::Serialize;
use std::fs::{File, create_dir_all};

#[derive(Debug, Serialize)]
struct Piece {
    composer: String,
    title: String,
}

#[derive(Debug, Serialize)]
struct Performer {
    name: String,
    instrument: Option<String>,
}

#[derive(Debug, Serialize)]
struct Concert {
    datetime: DateTime<Utc>,
    url: String,
    venue: String,
    title: String,
    performers: Vec<Performer>,
    pieces: Vec<Piece>,
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
    let concerts = london_classical::wigmore::get_api().await;

    create_dir_all("../src/assets").unwrap();
    let output_file = File::create("../src/assets/concerts.json").unwrap();
    serde_json::to_writer_pretty(output_file, &concerts).unwrap();
}
