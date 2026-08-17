#[macro_use]
extern crate derive_builder;
extern crate serde;
extern crate serde_json;

#[derive(Builder)]
#[builder(setter(into), derive(serde::Serialize))]
#[builder_struct_attr(serde(rename_all = "camelCase"))]
#[allow(dead_code)]
struct Example {
    first_name: String,
    middle_name: String,
    #[builder_field_attr(serde(rename = "familyName"))]
    last_name: String,
}

#[test]
fn serialize_builder() {
    assert_eq!(
        serde_json::to_string(
            &ExampleBuilder::default()
                .first_name("Jane")
                .middle_name("Alice")
                .last_name("Doe"),
        )
        .unwrap(),
        r#"{"firstName":"Jane","middleName":"Alice","familyName":"Doe"}"#
    );
}
