#![allow(unused)]

use glib::prelude::*;

fn main() {
    let r = glib::Regex::new(
        "hello",
        glib::RegexCompileFlags::DEFAULT,
        glib::RegexMatchFlags::DEFAULT,
    )
    .unwrap()
    .unwrap();
    let s = glib::GStr::from_str_until_nul("hello\0").unwrap();
    let match_info = r
        .match_(s, glib::RegexMatchFlags::DEFAULT)
        .expect("should match");
    assert_eq!(match_info.fetch_all(), vec!["hello"]);
    let v: glib::Value = match_info.to_value();
    drop(match_info);
    let match_info = v.get::<glib::MatchInfo<'_>>().unwrap();
}
