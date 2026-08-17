#![deny(warnings)]

use std::convert::Infallible;
use std::str::FromStr;
use std::time::Duration;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Match `/:Seconds`...
    let routes = warp::path::param()
        // and_then create a `Future` that will simply wait N seconds...
        .and_then(sleepy);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn sleepy(Seconds(seconds): Seconds) -> Result<impl warp::Reply, Infallible> {
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    Ok(format!("I waited {} seconds!", seconds))
}

/// A newtype to enforce our maximum allowed seconds.
struct Seconds(u64);

impl FromStr for Seconds {
    type Err = ();
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        src.parse::<u64>().map_err(|_| ()).and_then(|num| {
            if num <= 5 {
                Ok(Seconds(num))
            } else {
                Err(())
            }
        })
    }
}
