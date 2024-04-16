use crate::core;
use chrono::{DateTime, Utc};
use futures::future::join_all;
use html_escape::decode_html_entities;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WigmoreFrontPageConcert {
    pub datetime: DateTime<Utc>,
    pub url: String,
    pub title: String,
    pub subtitle: Option<String>,
}

async fn get_api_page(
    client: &reqwest::Client,
    page_number: u64,
) -> (Vec<WigmoreFrontPageConcert>, u64) {
    let url = format!(
        "https://www.wigmore-hall.org.uk/api/v1/listings/whats-on?page={}",
        page_number
    );
    let json: serde_json::Value = client.get(url).send().await.unwrap().json().await.unwrap();
    let concerts = parse_api(&json["items"].as_array().unwrap());
    (concerts, json["totalPages"].as_u64().unwrap())
}

fn parse_api(json_items: &Vec<serde_json::Value>) -> Vec<WigmoreFrontPageConcert> {
    let mut concerts = Vec::new();

    for item in json_items {
        let datetime_str = item["node"]["date"].as_str().unwrap();
        let datetime = DateTime::parse_from_rfc3339(datetime_str).unwrap().into();
        let url: String = format!(
            "https://wigmore-hall.org.uk{}",
            item["node"]["url"].as_str().unwrap()
        );
        let title: String = item["node"]["titleOverrideText"]
            .as_str()
            .unwrap()
            .to_string();
        let subtitle: Option<String> = item["node"]["subtitleText"].as_str().map(|s| s.to_string());

        concerts.push(WigmoreFrontPageConcert {
            datetime,
            url,
            title,
            subtitle,
        });
    }

    concerts
}

/// Retrieve all upcoming concerts via the Wigmore Hall API
pub async fn get_api(client: &reqwest::Client) -> Vec<WigmoreFrontPageConcert> {
    let mut concerts = Vec::new();

    // Scrape the first page and determine how many pages there are
    let (first_page, npages) = get_api_page(client, 1).await;
    concerts.extend(first_page);
    // Then scrape the remaining pages
    let futures = (2..=npages).map(|i| get_api_page(client, i));
    let remaining_concerts: Vec<WigmoreFrontPageConcert> = join_all(futures)
        .await
        .into_iter()
        .map(|(concerts, _)| concerts)
        .flatten()
        .collect();
    concerts.extend(remaining_concerts);

    concerts.sort_by_key(|concert| concert.datetime);
    concerts
}

/// Retrieve details of an individual concert by scraping the URL
pub async fn get_concert(
    fp_entry: &WigmoreFrontPageConcert,
    client: &reqwest::Client,
) -> Option<core::Concert> {
    eprintln!("Scraping concert at {}", fp_entry.url);

    // Wigmore's website actually seems to give us all the data in JSON format, but curiously, it's
    // in a script tag in the HTML. Not complaining though as it is still so much easier than
    // parsing the HTML itself.
    let html = client
        .get(&fp_entry.url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let doc = Html::parse_document(&html);
    let json_script_text = doc
        .select(&Selector::parse("script#props").unwrap())
        .next()
        .unwrap()
        .inner_html()
        .replace("&lt;\\!--", "&lt;!--"); // Fix invalid escape sequence
    let json_result = serde_json::from_str::<serde_json::Value>(&json_script_text);
    match json_result {
        Ok(json) => Some(parse_concert_json(fp_entry, json)),
        Err(e) => {
            eprintln!("Error parsing JSON for concert at {}: {}", fp_entry.url, e);
            None
        }
    }
}

fn parse_concert_json(
    fp_entry: &WigmoreFrontPageConcert,
    json: serde_json::Value,
) -> core::Concert {
    // println!("{}", serde_json::to_string_pretty(&json).unwrap());

    // Parse repertoire
    let mut pieces = Vec::new();
    let opt_repertoire = json["data"]["page"]["repertoire"].as_array();
    match opt_repertoire {
        None => eprintln!("No repertoire found for concert at {}", fp_entry.url),
        Some(repertoire) => {
            for piece in repertoire {
                let opt_cycle = piece["cycle"].as_str().map(decode_html_entities);
                let opt_piece_name = piece["title"].as_str().map(decode_html_entities);
                let opt_title = match (opt_cycle, opt_piece_name) {
                    (Some(cycle), Some(piece_name)) => Some(format!("{}: {}", cycle, piece_name)),
                    (Some(cycle), None) => Some(cycle.to_string()),
                    (None, Some(piece_name)) => Some(piece_name.to_string()),
                    _ => None,
                };
                let opt_composer = piece["composers"]
                    .as_array()
                    .and_then(|arr| arr[0]["title"].as_str())
                    .or(Some(""))
                    .map(|s| decode_html_entities(s).to_string());
                match (opt_title, opt_composer) {
                    (Some(title), Some(composer)) => pieces.push(core::Piece { title, composer }),
                    _ => (),
                }
            }
        }
    }

    // Check for U35 discount
    let booking_text = json["data"]["page"]["bookingInformationText"]
        .as_str()
        .unwrap()
        .to_lowercase();
    let is_wigmore_u35 = booking_text.contains("tickets for under 35s available");

    // Parse artists
    let mut performers = Vec::new();
    let opt_credits = json["data"]["page"]["credits"].as_array();
    match opt_credits {
        None => eprintln!("No performer credits found for concert at {}", fp_entry.url),
        Some(credits) => {
            for credit in credits {
                let opt_artist_name = credit["artist"]["title"].as_str().map(decode_html_entities);
                let opt_role = credit["role"].as_str().map(decode_html_entities);
                match (opt_artist_name, opt_role) {
                    (Some(artist_name), Some(role)) => performers.push(core::Performer {
                        name: artist_name.to_string(),
                        instrument: Some(role.to_string()),
                    }),
                    _ => (),
                }
            }
        }
    }

    // Figure out prices
    let prices_text = json["data"]["page"]["pricesText"].as_str().unwrap();
    let (min_price, max_price) = match prices_text {
        _ if prices_text.to_lowercase() == "free" => (Some(0), Some(0)),
        _ => {
            let re = Regex::new(r"£(\d+)").unwrap();
            let all_prices_in_pennies: Vec<u32> = re
                .captures_iter(prices_text)
                .map(|cap| cap[1].parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (
                all_prices_in_pennies.iter().min().map(|&x| x * 100),
                all_prices_in_pennies.iter().max().map(|&x| x * 100),
            )
        }
    };

    fn clean_up_description(s: &str) -> String {
        // Split paragraphs
        let s = decode_html_entities(s)
            .to_string()
            .replace("</p><p>", "\n")
            .replace("<p>", "")
            .replace("</p>", "");
        // Then remove any remaining HTML tags, it just seems excessive
        // to keep them in the description
        let fragments = scraper::Html::parse_fragment(&s);
        let mut cleaned = String::new();
        for node in fragments.tree {
            if let scraper::Node::Text(text_node) = node {
                cleaned.push_str(&text_node.text);
            }
        }
        cleaned
    }

    core::Concert {
        datetime: fp_entry.datetime,
        url: fp_entry.url.clone(),
        title: fp_entry.title.clone(),
        subtitle: fp_entry.subtitle.clone(),
        description: json["data"]["page"]["overviewText"]
            .as_str()
            .map(clean_up_description),
        programme_pdf_url: json["data"]["page"]["programmeDocument"]["url"]
            .as_str()
            .map(|s| s.to_string()),
        venue: "Wigmore Hall".to_string(),
        is_wigmore_u35,
        performers,
        pieces,
        min_price,
        max_price,
    }
}

/// This function scrapes the Wigmore Hall website to get concerts. As it turns out, Wigmore Hall
/// provides a public REST API which is much less fragile. This code is kept here in case the API
/// is ever removed.
/// {{{1
#[allow(dead_code)]
async fn scrape() {
    let client = reqwest::Client::new();
    let html: String = client
        .get("https://wigmore-hall.org.uk/whats-on/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let doc: Html = Html::parse_document(&html);

    let selector: Selector =
        Selector::parse("section[aria-label=\"Performances by date\"]").unwrap();
    let date_group_selector = Selector::parse("div.bg-white.col-12.sm-pb3").unwrap();

    // TODO: Parse dates, etc. into Concert struct (or a pre-struct)
    for perf_section in doc.select(&selector) {
        println!("found performance section!");
        for date_group in perf_section.select(&date_group_selector) {
            // Get date
            let date_h3_selector = Selector::parse("div > h3").unwrap();
            let date_h3 = date_group.select(&date_h3_selector).next().unwrap();
            println!("found date: {:?}", date_h3.inner_html());
            // Get concerts on that date
            let concert_selector = Selector::parse("article.px5.py6").unwrap();
            for concert in date_group.select(&concert_selector) {
                let url_selector = Selector::parse("a.black.text-decoration-reset").unwrap();
                let url_relative = concert
                    .select(&url_selector)
                    .next()
                    .unwrap()
                    .value()
                    .attr("href")
                    .unwrap();
                let url: String = format!("https://wigmore-hall.org.uk{}", url_relative);
                let artist_selector =
                    Selector::parse("div.type-style-3.sm-type-style-4.pb4.sm-pb0.sm-pr9.md-pr5")
                        .unwrap();
                let artist = concert
                    .select(&artist_selector)
                    .next()
                    .unwrap()
                    .inner_html();
                let title_selector = Selector::parse("div.type-style-5.hide.pt5.md-hide").unwrap();
                let title = concert
                    .select(&title_selector)
                    .next()
                    .map(|node| node.inner_html());
                println!("found concert: {:?} by {}\n  url: {}", title, artist, url);
            }
        }
    }
}
// }}}1

// vim: fdm=marker
