fn main() {
    let r = glib::Regex::new(
        "hello",
        glib::RegexCompileFlags::DEFAULT,
        glib::RegexMatchFlags::DEFAULT,
    )
    .unwrap()
    .unwrap();
    // implicit drop
    {
        let s = glib::GString::from("hello");
        let match_info = r
            .match_(s.as_gstr(), glib::RegexMatchFlags::DEFAULT)
            .expect("should match");
        assert_eq!(match_info.fetch_all(), vec!["hello"]);
        // match_info is dropped
        // s is dropped
    }
    // explicit drop
    {
        let s = glib::GString::from("hello");
        let match_info = r
            .match_(s.as_gstr(), glib::RegexMatchFlags::DEFAULT)
            .expect("should match");
        assert_eq!(match_info.fetch_all(), vec!["hello"]);
        drop(match_info);
        drop(s);
    }
}
