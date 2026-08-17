#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

use std::clone::Clone;
use std::fmt::Display;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
struct Generic<T: Display>
where
    T: Clone,
{
    ipsum: &'static str,
    pub dolor: T,
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
pub struct GenericReference<'a, T: 'a + Default>
where
    T: Display,
{
    pub bar: Option<&'a T>,
}

#[test]
fn error_if_uninitialized() {
    let error = GenericBuilder::<String>::default().build().unwrap_err();
    assert_eq!(&error.to_string(), "`ipsum` must be initialized");
}

#[test]
fn generic_builder() {
    let x = GenericBuilder::default()
        .ipsum("Generic")
        .dolor(true)
        .build()
        .unwrap();

    assert_eq!(
        x,
        Generic {
            ipsum: "Generic".into(),
            dolor: true,
        }
    );
}

#[test]
fn generic_reference_builder() {
    static BAR: u32 = 42;

    let x = GenericReferenceBuilder::<'static, u32>::default()
        .bar(Some(&BAR))
        .build()
        .unwrap();

    assert_eq!(x, GenericReference { bar: Some(&BAR) });
}
