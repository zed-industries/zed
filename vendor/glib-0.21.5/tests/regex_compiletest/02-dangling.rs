fn main() {
    let r = glib::Regex::new(
        "hello",
        glib::RegexCompileFlags::DEFAULT,
        glib::RegexMatchFlags::DEFAULT,
    )
    .unwrap()
    .unwrap();
    let s = glib::GString::from("hello");
    let match_info = r
        .match_(s.as_gstr(), glib::RegexMatchFlags::DEFAULT)
        .expect("should match");
    dbg!(match_info.fetch_all());
    drop(s);
}
