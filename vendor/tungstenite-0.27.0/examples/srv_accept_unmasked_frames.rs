use std::{net::TcpListener, thread::spawn};
use tungstenite::{
    accept_hdr_with_config,
    handshake::server::{Request, Response},
    protocol::WebSocketConfig,
};

fn main() {
    env_logger::init();
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (header, _value) in req.headers() {
                    println!("* {header}");
                }

                // Let's add an additional header to our response to the client.
                let headers = response.headers_mut();
                headers.append("MyCustomHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };

            let config = Some(
                WebSocketConfig::default()
                    // This setting allows to accept client frames which are not masked
                    // This is not in compliance with RFC 6455 but might be handy in some
                    // rare cases where it is necessary to integrate with existing/legacy
                    // clients which are sending unmasked frames
                    .accept_unmasked_frames(true),
            );

            let mut websocket = accept_hdr_with_config(stream.unwrap(), callback, config).unwrap();

            loop {
                let msg = websocket.read().unwrap();
                if msg.is_binary() || msg.is_text() {
                    println!("received message {msg}");
                }
            }
        });
    }
}
