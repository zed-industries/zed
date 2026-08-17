use tungstenite::{connect, Message};

fn main() {
    env_logger::init();

    let (mut socket, response) = connect("ws://localhost:3012/socket").expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (header, _value) in response.headers() {
        println!("* {header}");
    }

    socket.send(Message::Text("Hello WebSocket".into())).unwrap();
    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {msg}");
    }
    // socket.close(None);
}
