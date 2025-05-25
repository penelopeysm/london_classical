use log::{debug, info};
use london_classical::{core, proms, southbank, wigmore};
use reqwest::header;
use std::fs::{create_dir_all, File};

fn envvar_is_empty_or_undefined(var: &str) -> bool {
    match std::env::var(var) {
        Ok(value) => value.is_empty(),
        Err(_) => true,
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent("penelopeysm/london-classical/0.1")
        .build()
        .unwrap();

    // Fetch Wigmore concerts, first reading from $LDNCLS_WIGMORE_MAX and defaulting to 220 if not
    // given. If you push it a bit more it starts to rate limit
    let max_wigmore_concerts = {
        match std::env::var("LDNCLS_WIGMORE_MAX") {
            Ok(s) => match s.as_str() {
                "all" => None,
                _ => Some(s.parse::<usize>().unwrap()),
            },
            Err(_) => Some(220),
        }
    };
    debug!("max_wigmore_concerts: {:?}", max_wigmore_concerts);
    let scrape_wigmore = envvar_is_empty_or_undefined("LDNCLS_WIGMORE_DISABLE");
    let mut wigmore_concerts = if scrape_wigmore {
        info!(
            "Scraping Wigmore Hall concerts (up to a maximum of {:?})",
            max_wigmore_concerts
        );
        let cs = wigmore::get_concerts(&client, max_wigmore_concerts).await;
        info!("Found {} Wigmore Hall concerts", cs.len());
        cs
    } else {
        info!("$LDNCLS_WIGMORE_DISABLE not empty; skipping Wigmore Hall concerts");
        vec![]
    };

    // Fetch Proms
    let scrape_proms = envvar_is_empty_or_undefined("LDNCLS_PROMS_DISABLE");
    let mut proms_concerts = if scrape_proms {
        info!("Scraping Proms");
        let cs = proms::scrape(&client).await;
        info!("Found {} Proms", cs.len());
        cs
    } else {
        info!("$LDNCLS_PROMS_DISABLE not empty; skipping Proms");
        vec![]
    };

    // Southbank Centre
    let scrape_southbank = envvar_is_empty_or_undefined("LDNCLS_SOUTHBANK_DISABLE");
    let mut southbank_concerts = if scrape_southbank {
        info!("Scraping Southbank Centre concerts");
        let cs = southbank::scrape(&client).await;
        info!("Found {} Southbank Centre concerts", cs.len());
        cs
    } else {
        info!("$LDNCLS_SOUTHBANK_DISABLE not empty; skipping Southbank Centre concerts");
        vec![]
    };

    // Concatenate and sort
    let mut full_concerts = vec![];
    full_concerts.append(&mut wigmore_concerts);
    full_concerts.append(&mut proms_concerts);
    full_concerts.append(&mut southbank_concerts);
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

    let n_concerts = all_ids.len();
    info!("Found {} concerts in total", n_concerts);
    if n_concerts > 0 {
        for i in 0..n_concerts - 1 {
            if all_ids[i] == all_ids[i + 1] {
                panic!("Duplicate ID: {}", all_ids[i]);
            }
        }
    }

    let output_dir = env!("CARGO_MANIFEST_DIR").to_string() + "/../src/assets";
    create_dir_all(&output_dir).unwrap();
    let output_file = File::create(output_dir + "/concerts.json").unwrap();
    serde_json::to_writer_pretty(output_file, &full_concerts_with_ids).unwrap();
}
