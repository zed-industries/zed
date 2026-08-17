use octocrab::{etag::Etagged, models::events::Event, Page};
use std::collections::VecDeque;

const DELAY_MS: u64 = 500;
const TRACKING_CAPACITY: usize = 200;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let mut etag = None;
    let mut seen = VecDeque::with_capacity(TRACKING_CAPACITY);
    let octo = octocrab::instance();
    loop {
        let response: Etagged<Page<Event>> = octo.events().etag(etag).per_page(100).send().await?;
        if let Some(page) = response.value {
            for event in page {
                if !seen.contains(&event.id) {
                    println!(
                        "New event : id = {:?}, type = {:?}, time = {:?}",
                        event.id, event.r#type, event.created_at,
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
