use async_tungstenite::{async_std::connect_async, tungstenite::Error, tungstenite::Result};
use futures::prelude::*;
use log::*;

const AGENT: &str = "Tungstenite";

async fn get_case_count() -> Result<u32> {
    let (mut socket, _) = connect_async("ws://localhost:9001/getCaseCount").await?;
    let msg = socket.next().await.expect("Can't fetch case count")?;
    socket.close(None).await?;
    Ok(msg
        .to_text()?
        .parse::<u32>()
        .expect("Can't parse case count"))
}

async fn update_reports() -> Result<()> {
    let (mut socket, _) = connect_async(&format!(
        "ws://localhost:9001/updateReports?agent={}",
        AGENT
    ))
    .await?;
    socket.close(None).await?;
    Ok(())
}

async fn run_test(case: u32) -> Result<()> {
    info!("Running test case {}", case);
    let case_url = &format!("ws://localhost:9001/runCase?case={}&agent={}", case, AGENT);

    let (mut ws_stream, _) = connect_async(case_url).await?;
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            // for Sink of futures 0.3, see autobahn-server example
            ws_stream.send(msg).await?;
        }
    }

    Ok(())
}

async fn run() {
    env_logger::init();

    let total = get_case_count().await.expect("Error getting case count");

    for case in 1..=total {
        if let Err(e) = run_test(case).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8(_) => (),
                err => error!("Testcase failed: {}", err),
            }
        }
    }

    update_reports().await.expect("Error updating reports");
}

fn main() {
    async_std::task::block_on(run());
}
