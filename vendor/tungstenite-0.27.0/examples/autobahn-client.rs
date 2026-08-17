use log::*;

use tungstenite::{connect, Error, Message, Result};

const AGENT: &str = "Tungstenite";

fn get_case_count() -> Result<u32> {
    let (mut socket, _) = connect("ws://localhost:9001/getCaseCount")?;
    let msg = socket.read()?;
    socket.close(None)?;
    Ok(msg.into_text()?.as_str().parse::<u32>().unwrap())
}

fn update_reports() -> Result<()> {
    let (mut socket, _) = connect(format!("ws://localhost:9001/updateReports?agent={AGENT}"))?;
    socket.close(None)?;
    Ok(())
}

fn run_test(case: u32) -> Result<()> {
    info!("Running test case {}", case);
    let case_url = format!("ws://localhost:9001/runCase?case={case}&agent={AGENT}");
    let (mut socket, _) = connect(case_url)?;
    loop {
        match socket.read()? {
            msg @ Message::Text(_) | msg @ Message::Binary(_) => {
                socket.send(msg)?;
            }
            Message::Ping(_) | Message::Pong(_) | Message::Close(_) | Message::Frame(_) => {}
        }
    }
}

fn main() {
    env_logger::init();

    let total = get_case_count().unwrap();

    for case in 1..=total {
        if let Err(e) = run_test(case) {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8(_) => (),
                err => error!("test: {}", err),
            }
        }
    }

    update_reports().unwrap();
}
