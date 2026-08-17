#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Eq, Builder)]
#[builder(custom_constructor, build_fn(private, name = "fallible_build"))]
struct Request {
    url: &'static str,
    username: &'static str,
    #[builder(default, setter(into))]
    password: Option<&'static str>,
}

impl RequestBuilder {
    pub fn new(url: &'static str, username: &'static str) -> Self {
        Self {
            url: Some(url),
            username: Some(username),
            ..Self::create_empty()
        }
    }

    pub fn build(&self) -> Request {
        self.fallible_build()
            .expect("All required fields set upfront")
    }
}

#[test]
fn new_then_build_succeeds() {
    assert_eq!(
        RequestBuilder::new("...", "!!!").build(),
        Request {
            url: "...",
            username: "!!!",
            password: None
        }
    );
}

#[test]
fn new_then_set_succeeds() {
    assert_eq!(
        RequestBuilder::new("...", "!!!").password("test").build(),
        Request {
            url: "...",
            username: "!!!",
            password: Some("test")
        }
    );
}
