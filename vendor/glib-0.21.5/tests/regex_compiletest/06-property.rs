use glib::prelude::*;
use glib::subclass::prelude::*;
use glib_macros::Properties;
use std::cell::RefCell;
pub mod imp {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::Foo)]
    pub struct Foo {
        #[property(get, set)]
        match_info: RefCell<Option<glib::MatchInfo<'static>>>,
    }

    #[glib::derived_properties]
    impl ObjectImpl for Foo {}

    #[glib::object_subclass]
    impl ObjectSubclass for Foo {
        const NAME: &'static str = "MyFoo";
        type Type = super::Foo;
    }
}

glib::wrapper! {
    pub struct Foo(ObjectSubclass<imp::Foo>);
}

fn main() {
    let myfoo: Foo = glib::object::Object::new();

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

    myfoo.set_match_info(match_info);

    let match_info: glib::MatchInfo<'_> = myfoo.match_info().unwrap();
    assert_eq!(match_info.fetch_all(), vec!["hello"]);
}
