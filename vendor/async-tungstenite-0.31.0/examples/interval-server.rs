use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};
use futures::future::{select, Either};
use futures::prelude::*;
use log::*;
use std::net::SocketAddr;
use std::time::Duration;

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8(_) => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = async_std::stream::interval(Duration::from_millis(1000));

    // Echo incoming WebSocket messages and send a message periodically every second.

    let mut msg_fut = ws_receiver.next();
    let mut tick_fut = interval.next();
    loop {
        match select(msg_fut, tick_fut).await {
            Either::Left((msg, tick_fut_continue)) => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() || msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                        tick_fut = tick_fut_continue; // Continue waiting for tick.
                        msg_fut = ws_receiver.next(); // Receive next WebSocket message.
                    }
                    None => break, // WebSocket stream terminated.
                };
            }
            Either::Right((_, msg_fut_continue)) => {
                ws_sender.send(Message::text("tick")).await?;
                msg_fut = msg_fut_continue; // Continue receiving the WebSocket message.
                tick_fut = interval.next(); // Wait for next tick.
            }
        }
    }

    Ok(())
}

async fn run() {
    env_logger::init();

    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        task::spawn(accept_connection(peer, stream));
    }
}

fn main() {
    task::block_on(run());
}
