use super::IntoStrRef;
use crate::prelude::*;
use core::convert::Infallible;
use core::{fmt, num};
type ParseIntResult<T> = Result<T, num::ParseIntError>;

#[test]
fn test_struct() {
    #[derive(Debug, Builder)]
    #[builder(derive(Clone))]
    #[allow(dead_code)]
    struct Sut<T: Clone + Default> {
        #[builder(with = |x: u32, y: u32| x + y)]
        required: u32,

        #[builder(with = |x: u32, y: u32| x + y)]
        optional: Option<u32>,

        #[builder(with = |x: u32, y: u32| x + y, default)]
        default: u32,

        #[builder(with = |value: &T, _value2: &T| value.clone())]
        generic: T,

        #[builder(with = |value: &T, _value2: &T| value.clone())]
        optional_generic: Option<T>,

        #[builder(with = |value: &T, _value2: &T| value.clone(), default)]
        default_generic: T,

        #[builder(with = |value: impl Into<&'static str>| value.into())]
        impl_trait: &'static str,

        #[builder(with = |value: &str, _value2: u32| -> Result<_, num::ParseIntError> { value.parse() })]
        try_required: u32,

        #[builder(with = |value: &str, _value2: u32| -> Result<_, num::ParseIntError> { value.parse() })]
        try_optional: Option<u32>,

        #[builder(with = |value: &str, _value2: u32| -> ParseIntResult<_> { value.parse() }, default)]
        try_default: u32,

        #[builder(with = |value: &T, _value2: &T| -> Result<_, Infallible> { Ok(value.clone()) })]
        try_optional_generic: Option<T>,

        #[builder(with = |value: &T, _value2: &T| -> Result<_, Infallible> { Ok(value.clone()) }, default)]
        try_default_generic: T,

        #[builder(with = |value: impl Into<&'static str>, _value2: impl fmt::Debug| -> Result<_, Infallible> { Ok(value.into()) })]
        try_impl_trait: &'static str,
    }

    let builder = Sut::builder()
        .required(1, 2)
        .optional(3, 4)
        .default(5, 6)
        .generic(&"hello", &"world");

    let _ignore = builder.clone().optional_generic(&"hello", &"you");
    let builder = builder.maybe_optional_generic(Some((&"littlepip", &"blackjack")));

    let _ignore = builder.clone().default_generic(&"p21", &"rampage");
    let builder = builder.maybe_default_generic(Some((&"<3", &"glory")));

    let builder = builder
        .impl_trait(IntoStrRef("p21"))
        .try_required("4", 99)
        .unwrap();

    let _ignore = builder.clone().try_optional("5", 99).unwrap();
    let builder = builder.maybe_try_optional(Some(("6", 99))).unwrap();

    let _ignore = builder.clone().try_default("7", 99).unwrap();
    let builder = builder.maybe_try_default(Some(("8", 99))).unwrap();

    let _ignore = builder.clone().try_optional_generic(&"9", &"99").unwrap();
    let builder = builder
        .maybe_try_optional_generic(Some((&"10", &"99")))
        .unwrap();

    let _ignore = builder.clone().try_default_generic(&"11", &"99").unwrap();
    let builder = builder
        .maybe_try_default_generic(Some((&"12", &"99")))
        .unwrap();

    let builder = builder.try_impl_trait(IntoStrRef("p21"), true).unwrap();

    assert_debug_eq(
        builder.build(),
        expect![[r#"
            Sut {
                required: 3,
                optional: Some(
                    7,
                ),
                default: 11,
                generic: "hello",
                optional_generic: Some(
                    "littlepip",
                ),
                default_generic: "<3",
                impl_trait: "p21",
                try_required: 4,
                try_optional: Some(
                    6,
                ),
                try_default: 8,
                try_optional_generic: Some(
                    "10",
                ),
                try_default_generic: "12",
                try_impl_trait: "p21",
            }"#]],
    );
}
