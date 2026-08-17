fn main() {
    if let Ok((_, screen_num)) = xcb::Connection::connect(None) {
        println!("Connected to X on screen \"{}\"!", screen_num);
    } else {
        println!("Could not connect to X!");
    }
}
