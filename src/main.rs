use chrono::{DateTime, FixedOffset, TimeZone, Utc};
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

fn main() {
    let jyt = Concert {
        datetime: from_london_time(2024, 1, 24, 19, 30),
        venue: "Barbican Centre".to_string(),
        title: "Debussy Préludes".to_string(),
        performers: vec![Performer {
            name: "Jean-Yves Thibaudet".to_string(),
            instrument: Some("piano".to_string()),
        }],
        pieces: vec![
            Piece {
                composer: "Claude Debussy".to_string(),
                title: "Préludes, Book I".to_string(),
            },
            Piece {
                composer: "Claude Debussy".to_string(),
                title: "Préludes, Book II".to_string(),
            },
        ],
    };

    let hh = Concert {
        datetime: from_london_time(2024, 4, 8, 19, 30),
        venue: "Wigmore Hall".to_string(),
        title: "Brahms Violin Sonatas".to_string(),
        performers: vec![
            Performer {
                name: "Hilary Hahn".to_string(),
                instrument: Some("violin".to_string()),
            },
            Performer {
                name: "Andreas Haefliger".to_string(),
                instrument: Some("piano".to_string()),
            },
        ],
        pieces: vec![
            Piece {
                composer: "Johannes Brahms".to_string(),
                title: "Violin Sonata No. 1, Op. 78".to_string(),
            },
            Piece {
                composer: "Johannes Brahms".to_string(),
                title: "Violin Sonata No. 2, Op. 100".to_string(),
            },
            Piece {
                composer: "Johannes Brahms".to_string(),
                title: "Violin Sonata No. 3, Op. 108".to_string(),
            },
        ],
    };

    display_programme(&jyt);

    println!();

    display_programme(&hh);
}
