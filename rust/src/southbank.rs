use crate::core;
use chrono::{DateTime, NaiveDate, Utc};
use core::naivedt_to_utc;
use futures::future::join_all;
use itertools::Itertools;
use log::{debug, info};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

fn get_southbank_url(page: u32) -> String {
    format!(
        "https://www.southbankcentre.co.uk/whats-on/page/{}/?artform-filter=classical-music",
        page
    )
}

pub async fn scrape(client: &reqwest::Client) -> Vec<core::ConcertData> {
    let mut concerts: Vec<core::ConcertData> = vec![];

    // TODO: Dynamically determine the number of pages to scrape
    let mut page = 1;
    loop {
        let page_concerts = scrape_page(page, client).await;
        if page_concerts.is_empty() {
            break;
        }
        else {
            concerts.extend(page_concerts);
            page += 1;
        }
    }

    info!("Scraped {} concerts from Southbank Centre", concerts.len());
    concerts
}

async fn scrape_page(page: u32, client: &reqwest::Client) -> Vec<core::ConcertData> {
    let url: String = get_southbank_url(page);
    debug!(
        "Scraping page {} of Southbank Centre from URL {}",
        page, url
    );

    let html: String = client
        .get(url)
        .send()
        .await
        .expect("Failed to fetch Southbank page")
        .text()
        .await
        .expect("Failed to parse Southbank page");
    let doc: Html = Html::parse_document(&html);

    let slc_concert_link: Selector = Selector::parse("a.c-event-card__cover-link").unwrap();

    let futures = doc
        .select(&slc_concert_link)
        .map(|link_elem| {
            link_elem
                .value()
                .attr("href")
                .expect("Concert URL is missing href attribute")
        })
        .filter(|url| {
            // https://github.com/penelopeysm/london_classical/issues/3
            !url.contains("christmas-classics")
        })
        .map(|url| scrape_concert_info(url, client));
    let concerts: Vec<core::ConcertData> = join_all(futures).await;

    // .filter_map(async |url| scrape_concert_info(url).await)
    // .collect::<Vec<_>>();

    concerts
}

async fn scrape_concert_info(concert_url: &str, client: &reqwest::Client) -> core::ConcertData {
    let html: String = client
        .get(concert_url)
        .header("User-Agent", "penelopeysm/london-classical/0.1")
        .send()
        .await
        .expect("Failed to fetch Southbank page")
        .text()
        .await
        .expect("Failed to parse Southbank page");
    let doc: Html = Html::parse_document(&html);

    let slc_title = Selector::parse("h1.c-event-masthead__title").unwrap();
    let title = doc
        .select(&slc_title)
        .next()
        .expect("Could not find concert title")
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let slc_datetime = Selector::parse("div.c-event-masthead__event-datetime").unwrap();
    let datetime_str = doc
        .select(&slc_datetime)
        .next()
        .expect("Could not find concert datetime")
        .text()
        .collect::<String>()
        .trim()
        .to_string();
    let datetime_utc = match datetime_str.split(",").collect::<Vec<&str>>().as_slice() {
        [date, time] => parse_datetime(date.trim(), time.trim()),
        [date] => {
            // No time specified, just put midnight
            // TODO: make time optional (?!!!)
            let date = NaiveDate::parse_from_str(date, "%a %e %b %Y")
                .expect("Failed to parse concert date");
            naivedt_to_utc(date, 0, 0)
        }
        _ => {
            panic!("Unexpected datetime format: {}", datetime_str);
        }
    };

    let slc_location = Selector::parse("span.c-event-masthead__event-location-label-text").unwrap();
    let venue = doc
        .select(&slc_location)
        .next()
        .expect("Could not find concert venue")
        .text()
        .collect::<Vec<&str>>()
        .pop()
        .expect("Venue text is empty")
        .trim();

    let slc_description = Selector::parse("div.c-event-section__main > p").unwrap();
    let description = doc
        .select(&slc_description)
        .map(|p_elem| {
            p_elem
                .text()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .join(" ")
        })
        .join("\n");

    // Prices are annoying -- sometimes they're listed as £15.00, sometimes £15, so we have to
    // check both. However, Southbank uniformly lists "from X price" so we don't need to check for
    // maximum price. (The only case where we need to care about the maximum price is if the
    // concert is free, in which case it's just 0.)
    let slc_price = Selector::parse("span.c-event-masthead__event-price").unwrap();
    let price_elem = doc.select(&slc_price).next();
    let (min_price, max_price) = match price_elem {
        None => {
            // no price was specified -- check if it's free
            let slc_free = Selector::parse("span.c-btn--free-no-ticket").unwrap();
            // if there is a free button, it's free, otherwise we just return None as we don't
            // know.
            match doc.select(&slc_free).next() {
                None => (None, None),
                Some(_) => (Some(0), Some(0)),
            }
        }
        Some(elem) => {
            let price_text = elem.text().collect::<String>();
            let price_re = Regex::new(r"£(\d+)\.(\d+)").unwrap();
            let mut prices: Vec<u32> = price_re
                .captures_iter(price_text.as_str())
                .map(|cap| {
                    let pounds = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let pence = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    pounds * 100 + pence
                })
                .collect::<Vec<u32>>();
            let other_price_re = Regex::new(r"£(\d+)([^.\d]|$)").unwrap();
            let other_prices: Vec<u32> = other_price_re
                .captures_iter(price_text.as_str())
                .map(|cap| {
                    let pounds = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    pounds * 100
                })
                .collect::<Vec<u32>>();
            prices.extend(other_prices);
            (prices.into_iter().min(), None)
        }
    };

    let slc_performers = Selector::parse("p.c-event-performers__item").unwrap();
    let performers: Vec<core::Performer> = doc
        .select(&slc_performers)
        .map(|p_elem| parse_performer(p_elem))
        .collect();

    let slc_pieces = Selector::parse("p.c-event-repertoire__item").unwrap();
    let pieces: Vec<core::Piece> = doc
        .select(&slc_pieces)
        .flat_map(|p_elem| parse_piece(p_elem))
        .collect();

    let concert = core::ConcertData {
        datetime: datetime_utc,
        url: concert_url.to_string(),
        performers,
        title,
        subtitle: None,
        description: Some(description),
        programme_pdf_url: None,
        pieces,
        venue: venue.to_string(),
        min_price,
        max_price,
        is_wigmore_u35: false,
        is_prom: false,
    };

    core::report_concert(&concert);
    concert
}

fn parse_datetime(date_str: &str, time_str: &str) -> DateTime<Utc> {
    let date =
        NaiveDate::parse_from_str(date_str, "%a %e %b %Y").expect("Failed to parse concert date");

    let mut time_chars_iter = time_str.chars();
    let unadjusted_hour = time_chars_iter
        .take_while_ref(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    // need to use .count() to actually consume the dots
    let _ = time_chars_iter.take_while_ref(|c| *c == '.').count();
    let minute = match time_chars_iter
        .take_while_ref(|c| c.is_ascii_digit())
        .collect::<String>()
        .as_str()
    {
        "" => 0,
        m => m.parse::<u32>().expect("Failed to parse concert minute"),
    };
    let remaining_chars: String = time_chars_iter.collect();
    let is_pm = match remaining_chars.as_str() {
        "pm" => true,
        "am" => false,
        _ => {
            panic!("Unexpected time format: {}", time_str);
        }
    };
    let hour = if is_pm && unadjusted_hour < 12 {
        unadjusted_hour + 12
    } else if !is_pm && unadjusted_hour == 12 {
        0
    } else {
        unadjusted_hour
    };

    naivedt_to_utc(date, hour, minute)
}

fn parse_performer(p_elem: ElementRef) -> core::Performer {
    let slc_name = Selector::parse("span.c-event-performers__name").unwrap();
    let name = p_elem
        .select(&slc_name)
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let slc_role = Selector::parse("span.c-event-performers__role").unwrap();
    let role = p_elem
        .select(&slc_role)
        .next()
        .map(|elem| elem.text().collect::<String>().trim().to_string());

    core::Performer {
        name,
        instrument: role,
    }
}

fn parse_piece(p_elem: ElementRef) -> Vec<core::Piece> {
    let slc_composer = Selector::parse("span.c-event-repertoire__composer").unwrap();
    let composer_elem = p_elem.select(&slc_composer).next();

    match composer_elem {
        None => vec![],
        Some(elem) => {
            let composer = elem.text().collect::<String>().trim().to_string();
            if composer == "Interval" || composer == "Programme includes" {
                return vec![];
            }
            let slc_piece = Selector::parse("span.c-event-performers__work").unwrap();
            p_elem
                .select(&slc_piece)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .split("; ")
                .map(|s| core::Piece {
                    composer: composer.clone(),
                    title: s.trim().to_string(),
                })
                .collect::<Vec<core::Piece>>()
        }
    }
}
