fn main() {
    let dpy = ":0";
    if let Ok((_, _)) = xcb::Connection::connect(Some(&dpy)) {
        println!("Connected to X on display \"{}\"!", dpy);
    } else {
        println!("Could not connect to X!");
    }
}
