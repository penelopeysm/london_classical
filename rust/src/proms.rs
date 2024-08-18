use crate::core;
use chrono::{NaiveDate, TimeZone, Utc};
use chrono_tz::Europe::London;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::cmp::min;

pub const PROMS_2024_URL: &str = "https://www.bbc.co.uk/events/rfbp5v/series";

// Scrapes concerts from BBC Proms website
pub async fn scrape(url: &str, client: &reqwest::Client) -> Vec<core::ConcertData> {
    println!("----------------------------------------");
    println!("Scraping BBC Proms from URL: {}", url);
    println!("----------------------------------------");

    let html: String = client
        .get(url)
        // it 500's with default user-agent
        .header("User-Agent", "penelopeysm/london-classical/0.1")
        .send()
        .await
        .expect("Failed to fetch Proms page")
        .text()
        .await
        .expect("Failed to parse Proms page");
    let doc: Html = Html::parse_document(&html);

    let mut concerts: Vec<core::ConcertData> = vec![];

    let date_selector: Selector = Selector::parse("li.ev-event-calendar__single-date-events")
        .expect("Invalid overall date selector");

    for this_date_performances in doc.select(&date_selector) {
        let (date, metadatas) = scrape_one_date(this_date_performances).await;
        metadatas
            .into_iter()
            .map(|metadata| make_full_concert(date, metadata))
            .for_each(|concert| concerts.push(concert));
    }

    concerts
}

/// Intermediate struct which contains information about a single concert entry. This does not
/// contain the date because of the way information is grouped on the BBC Proms webpage: concerts
/// on the same date are grouped together, so the date information is parsed separately and then
/// joined together with this to form the full core::Concert.
#[derive(Debug)]
struct PromsConcertMetadata {
    /// (hour, minute) in London timezone
    london_time: (u32, u32),
    title: String,
    description: Option<String>,
    venue: String,
    url: String,
    pieces: Vec<core::Piece>,
    performers: Vec<core::Performer>,
    min_price: Option<u32>,
    max_price: Option<u32>,
}

/// Scrapes a single date's worth of concerts from the BBC Proms website
async fn scrape_one_date(date_fragment: ElementRef<'_>) -> (NaiveDate, Vec<PromsConcertMetadata>) {
    let date_selector = Selector::parse("h3.ev-event-calendar__date").unwrap();
    let date_str = date_fragment
        .select(&date_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .trim();
    // BBC's website reports dates as e.g. "Fri 23 Aug 2024"
    let date = NaiveDate::parse_from_str(date_str, "%a %e %b %Y").unwrap();

    // The BBC website retains concerts from the past too, so we filter them out right here
    if date < Utc::now().date_naive() {
        return (date, vec![]);
    }

    // For any dates that aren't in the past, get the concerts themselves
    let mut intermediate_concerts: Vec<PromsConcertMetadata> = vec![];
    let concert_details_selector =
        Selector::parse("li.ev-event-calendar__event-summary-container").unwrap();
    for concert_elem in date_fragment.select(&concert_details_selector) {
        intermediate_concerts.push(parse_single_concert(concert_elem));
    }

    (date, intermediate_concerts)
}

/// Parses a single concert entry within a date fragment
fn parse_single_concert(elem: ElementRef<'_>) -> PromsConcertMetadata {
    let title = elem
        .select(&Selector::parse("div.ev-event-calendar__name").unwrap())
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .trim()
        .to_string();

    let time_string: &str = elem
        .select(&Selector::parse("div.ev-event-calendar__time").unwrap())
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .trim();
    let parsed_time: (u32, u32) = match time_string {
        "26 –  27 Jul 2024" => (23, 00), // hack for a specific concert
        _ => {
            let time_string2: Vec<&str> = time_string.split(':').collect();
            (
                time_string2[0].parse().unwrap(),
                time_string2[1].parse().unwrap(),
            )
        }
    };

    let pieces_selector =
        Selector::parse("li.ev-act-schedule__performance-composer-segments").unwrap();
    let pieces: Vec<core::Piece> = elem
        .select(&pieces_selector)
        .filter_map(parse_piece)
        .collect();

    let performer_selector = Selector::parse(
        "div[data-id-for-tests=\"event-schedule-artists\"] li.ev-act-schedule__artist",
    )
    .unwrap();
    let performers: Vec<core::Performer> = elem
        .select(&performer_selector)
        .map(|performer_elem| parse_performer(performer_elem))
        .collect();

    let price_selector =
        Selector::parse("div.ev-event-calendar__ticket-link-subtitle--desktop").unwrap();
    let price_text = elem
        .select(&price_selector)
        .next()
        .map(|elem| elem.text().next().unwrap().trim());
    // Regexes are hacky, but it works fine for now ... otherwise the website text is very
    // inconsistent and hard to parse.
    let price_re = Regex::new(r"£(\d+)").unwrap();
    let prices: Vec<u32> = match price_text {
        None => vec![],
        Some(t) => price_re
            .captures_iter(t)
            .map(|cap| cap.get(1).unwrap().as_str().parse().unwrap())
            .collect(),
    };
    let (min_price, max_price) = match prices[..] {
        [] => (None, None),
        [price] => (Some(price * 100), Some(price * 100)),
        [min_price, max_price] => (Some(min_price * 100), Some(max_price * 100)),
        _ => panic!("couldn't parse prices from {:?}", price_text),
    };

    PromsConcertMetadata {
        london_time: parsed_time,
        title,
        description: elem
            .select(&Selector::parse("p.ev-event-calendar__event-description").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .map(|s| s.trim().to_string()),
        url: "https://bbc.co.uk".to_string()
            + elem
                .select(&Selector::parse("div.ev-event-calendar__name>a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap(),
        venue: elem
            .select(&Selector::parse("span.ev-event-calendar__event-location").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string(),
        pieces,
        performers,
        min_price,
        max_price,
    }
}

/// Combines the date and the concert metadata to form a full core::Concert
fn make_full_concert(date: NaiveDate, metadata: PromsConcertMetadata) -> core::ConcertData {
    let naive_datetime = date
        .and_hms_opt(metadata.london_time.0, metadata.london_time.1, 0)
        .unwrap();
    let tz_datetime = London.from_local_datetime(&naive_datetime).unwrap();

    let is_rah_prom = metadata.venue == "Royal Albert Hall"
        && (metadata.title.starts_with("Prom") || metadata.title.starts_with("First Night"));
    let promming_price = 800;

    let concert = core::ConcertData {
        datetime: tz_datetime.with_timezone(&Utc),
        url: metadata.url,
        venue: metadata.venue,
        title: metadata.title,
        description: metadata.description,
        pieces: metadata.pieces,
        performers: metadata.performers,

        // Proms concerts don't have subtitles or programme PDFs available
        subtitle: None,
        programme_pdf_url: None,

        // Day promming prices aren't shown on the website so we add them in here
        min_price: if is_rah_prom {
            match metadata.min_price {
                Some(price) => Some(min(price, promming_price)),
                None => Some(promming_price),
            }
        } else {
            metadata.min_price
        },
        max_price: metadata.max_price,

        // By definition
        is_wigmore_u35: false,
        is_prom: true,
    };

    core::report_concert(&concert);
    concert
}

/// Helper function to parse a piece from a concert
fn parse_piece(piece_elem: ElementRef<'_>) -> Option<core::Piece> {
    // This is kind of hacky but it works
    let all_texts = piece_elem.text().collect::<Vec<&str>>();
    match all_texts[..] {
        ["interval"] => None,
        _ => Some(core::Piece {
            composer: all_texts[0].to_string(),
            title: all_texts[1..].join(" ").to_string(),
        }),
    }
}

/// Helper function to parse a performer from a concert
fn parse_performer(performer_elem: ElementRef<'_>) -> core::Performer {
    let name = performer_elem
        .select(&Selector::parse("div.ev-act-schedule__artist-details-container").unwrap())
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap();
    let role_texts = performer_elem
        .select(&Selector::parse("div.ev-act-schedule__artist-role-container").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<Vec<&str>>();
    core::Performer {
        name: name.to_string(),
        instrument: match &role_texts[..] {
            [] => None,
            _ => Some(role_texts.join(" ")),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_proms_2024() {
        let client = reqwest::Client::new();
        let concerts = scrape(PROMS_2024_URL, &client).await;
        let json = serde_json::to_string(&concerts);
        assert!(json.is_ok());
    }
}
