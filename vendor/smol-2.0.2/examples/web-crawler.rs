//! Crawls the Rust language website and prints found pages.
//!
//! Run with:
//!
//! ```
//! cargo run --example web-crawler
//! ```

use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use async_channel::{bounded, Sender};
use scraper::{Html, Selector};

const ROOT: &str = "https://www.rust-lang.org";

/// Fetches the HTML contents of a web page.
async fn fetch(url: String, sender: Sender<String>) {
    let body = surf::get(&url).recv_string().await;
    let body = body.unwrap_or_default();
    sender.send(body).await.ok();
}

/// Extracts links from a HTML body.
fn links(body: String) -> Vec<String> {
    let mut v = Vec::new();
    for elem in Html::parse_fragment(&body).select(&Selector::parse("a").unwrap()) {
        if let Some(link) = elem.value().attr("href") {
            v.push(link.to_string());
        }
    }
    v
}

fn main() -> Result<()> {
    smol::block_on(async {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        seen.insert(ROOT.to_string());
        queue.push_back(ROOT.to_string());

        let (s, r) = bounded(200);
        let mut tasks = 0;

        // Loop while the queue is not empty or tasks are fetching pages.
        while queue.len() + tasks > 0 {
            // Limit the number of concurrent tasks.
            while tasks < s.capacity().unwrap() {
                // Process URLs in the queue and fetch more pages.
                match queue.pop_front() {
                    None => break,
                    Some(url) => {
                        println!("{}", url);
                        tasks += 1;
                        smol::spawn(fetch(url, s.clone())).detach();
                    }
                }
            }

            // Get a fetched web page.
            let body = r.recv().await.unwrap();
            tasks -= 1;

            // Parse links in the web page and add them to the queue.
            for mut url in links(body) {
                // Add the site prefix if it's missing.
                if url.starts_with('/') {
                    url = format!("{}{}", ROOT, url);
                }

                // If the URL makes sense and was not seen already, push it into the queue.
                if url.starts_with(ROOT) && seen.insert(url.clone()) {
                    url = url.trim_end_matches('/').to_string();
                    queue.push_back(url);
                }
            }
        }
        Ok(())
    })
}
