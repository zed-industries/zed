#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

use std::num::ParseIntError;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
pub struct Lorem {
    #[builder(field(type = "Option<usize>", build = "self.ipsum.unwrap_or(42) + 1"))]
    ipsum: usize,

    #[builder(setter(into), field(type = "String", build = "self.dolor.parse()?"))]
    dolor: u32,
}

impl From<ParseIntError> for LoremBuilderError {
    fn from(e: ParseIntError) -> LoremBuilderError {
        LoremBuilderError::ValidationError(e.to_string())
    }
}

#[test]
fn custom_fields() {
    let x = LoremBuilder::default().dolor("7").build().unwrap();

    assert_eq!(
        x,
        Lorem {
            ipsum: 43,
            dolor: 7,
        }
    );

    let x = LoremBuilder::default()
        .ipsum(Some(12))
        .dolor("66")
        .build()
        .unwrap();

    assert_eq!(
        x,
        Lorem {
            ipsum: 13,
            dolor: 66,
        }
    );

    let x = LoremBuilder::default().build().unwrap_err().to_string();
    assert_eq!(x, "cannot parse integer from empty string");
}
