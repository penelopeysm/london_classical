use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Europe::London;
use serde::Serialize;

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

fn from_london_time(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    London
        .with_ymd_and_hms(year, month, day, hour, minute, 0)
        .unwrap()
        .with_timezone(&Utc)
}

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
    for concert in london_classical::wigmore::get_api().await {
        println!("Concert: {}", concert.title);
        println!("Performer: {}", concert.performer);
        println!("Date and time: {}", concert.datetime.with_timezone(&London));
        println!("URL: {}", concert.url);
        println!();
    }
}
