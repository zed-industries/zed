use octocrab::{etag::Etagged, models::events::Event, Page};
use std::collections::VecDeque;

const DELAY_MS: u64 = 500;
const TRACKING_CAPACITY: usize = 20;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let mut etag = None;
    let mut seen = VecDeque::with_capacity(TRACKING_CAPACITY);
    let octo = octocrab::instance();
    loop {
        let response: Etagged<Page<Event>> = octo
            .orgs("nixos")
            .events()
            .etag(etag)
            .per_page(10)
            .send()
            .await?;
        if let Some(page) = response.value {
            for event in page {
                // If an etag changes and we get a new page, this page may contain events we have
                // already seen along with new events. So, keep track of the ones we have seen for
                // each page, this will be at most 20 events - the current page of 10 events and
                // the last page.
                if !seen.contains(&event.id) {
                    println!(
                        "New event : id = {:?}, repo = {:?}, type = {:?}, time = {:?}",
                        event.id, event.repo.name, event.r#type, event.created_at
                    );
                    if seen.len() == TRACKING_CAPACITY {
                        seen.pop_back();
                    }
                    seen.push_front(event.id);
                }
            }
        }
        etag = response.etag;
        tokio::time::sleep(tokio::time::Duration::from_millis(DELAY_MS)).await;
    }
}
