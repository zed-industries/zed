fn main() {
    let (conn, _) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    println!("{:#?}", setup);
}
