use chrono::{DateTime, Utc};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WigmoreFrontPageConcert {
    pub datetime: DateTime<Utc>,
    pub url: String,
    pub performer: String,
    pub title: String,
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
        let mut performer: String = item["node"]["titleOverrideText"]
            .as_str()
            .unwrap()
            .to_string();
        let mut title: String = item["node"]["subtitleText"].as_str().unwrap().to_string();

        // If there is no title, the performer is the title, and the performer itself should be
        // empty
        if title.is_empty() {
            std::mem::swap(&mut performer, &mut title);
        }

        concerts.push(WigmoreFrontPageConcert {
            datetime,
            url,
            performer,
            title,
        });
    }

    concerts
}

/// Scrape all upcoming concerts via the Wigmore Hall API
pub async fn get_api() -> Vec<WigmoreFrontPageConcert> {
    let mut concerts = Vec::new();
    let client = reqwest::Client::new();

    // Scrape the first page and determine how many pages there are
    let (first_page, npages) = get_api_page(&client, 1).await;
    concerts.extend(first_page);
    // Then scrape the remaining pages
    for page_number in 2..=npages {
        concerts.extend(get_api_page(&client, page_number).await.0);
    }

    concerts.sort_by_key(|concert| concert.datetime);
    concerts
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
