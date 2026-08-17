#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

#[derive(Debug, Clone)]
pub enum ContentType {
    Json,
    Xml,
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Json
    }
}

#[derive(Debug, Builder)]
#[builder(
    custom_constructor,
    create_empty = "empty",
    build_fn(private, name = "fallible_build")
)]
pub struct ApiClient {
    // To make sure `host` and `key` are not changed after creation, tell derive_builder
    // to create the fields but not to generate setters.
    #[builder(setter(custom))]
    host: String,
    #[builder(setter(custom))]
    key: String,
    // uncommenting this field will cause the unit-test below to fail
    // baz: String,
    #[builder(default)]
    content_type: ContentType,
}

impl ApiClient {
    pub fn new(host: impl Into<String>, key: impl Into<String>) -> ApiClientBuilder {
        ApiClientBuilder {
            host: Some(host.into()),
            key: Some(key.into()),
            ..ApiClientBuilder::empty()
        }
    }
}

impl ApiClientBuilder {
    pub fn build(&self) -> ApiClient {
        self.fallible_build()
            .expect("All required fields were initialized")
    }
}

fn main() {
    dbg!(ApiClient::new("hello", "world")
        .content_type(ContentType::Xml)
        .build());
}

#[cfg(test)]
mod tests {
    use super::ApiClient;

    /// If any required fields are added to ApiClient and ApiClient::new was not updated,
    /// the field will be `None` when this "infallible" build method is called, resulting
    /// in a panic. Panicking is appropriate here, since the author of the builder promised
    /// the caller that it was safe to call new(...) followed immediately by build(), and
    /// the builder author is also the person who can correct the coding error by setting
    /// a default for the new property or changing the signature of `new`
    #[test]
    fn api_client_new_collects_all_required_fields() {
        ApiClient::new("hello", "world").build();
    }
}
