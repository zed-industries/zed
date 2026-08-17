//! Read/Write headers on server example
//!
//! Run with logs:
//! Linux:
//! ```sh
//! RUST_LOG=debug cargo run --example server-headers
//! ```
//! Windows
//! ```sh
//! cmd /c "set RUST_LOG=debug && cargo run --example server-headers"
//! ```
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{
        connect,
        handshake::server::{Request, Response},
        Message,
    },
};
use url::Url;
#[macro_use]
extern crate log;
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();

    tokio::spawn(async move {
        server().await;
    });
    client();
}

async fn server() {
    let server = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((stream, _)) = server.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) {
    let callback = |req: &Request, mut response: Response| {
        debug!("Received a new ws handshake");
        debug!("The request's path is: {}", req.uri().path());
        debug!("The request's headers are:");
        for (ref header, _value) in req.headers() {
            debug!("* {}: {:?}", header, _value);
        }

        let headers = response.headers_mut();
        headers.append("MyCustomHeader", ":)".parse().unwrap());

        Ok(response)
    };
    let mut ws_stream = accept_hdr_async(stream, callback)
        .await
        .expect("Error during the websocket handshake occurred");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg.unwrap();
        if msg.is_text() || msg.is_binary() {
            debug!("Server on message: {:?}", &msg);
            ws_stream.send(msg).await.unwrap();
        }
    }
}

fn client() {
    let (mut socket, response) =
        connect(Url::parse("ws://localhost:8080/socket").unwrap()).expect("Can't connect");
    debug!("Connected to the server");
    debug!("Response HTTP code: {}", response.status());
    debug!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        debug!("* {}: {:?}", header, _value);
    }

    socket.send(Message::Text("Hello WebSocket".into())).unwrap();
    loop {
        let msg = socket.read().expect("Error reading message");
        debug!("Received: {}", msg);
    }
}
