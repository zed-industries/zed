use crate::prelude::*;
use core::convert::Infallible;
use core::{fmt, num};
type ParseIntResult<T> = Result<T, num::ParseIntError>;
use super::IntoStrRef;

#[test]
fn test_struct() {
    #[derive(Debug, Builder)]
    #[builder(derive(Clone))]
    #[allow(dead_code)]
    struct Sut<T: Default + Clone> {
        #[builder(with = |x: u32| x + 1)]
        required: u32,

        #[builder(with = |x: u32| x + 1)]
        optional: Option<u32>,

        #[builder(with = |x: u32| x + 1, default)]
        default: u32,

        #[builder(with = |value: &T| value.clone())]
        generic: T,

        #[builder(with = |value: &T| value.clone())]
        optional_generic: Option<T>,

        #[builder(with = |value: &T| value.clone(), default)]
        default_generic: T,

        #[builder(with = |value: impl Into<&'static str>| value.into())]
        impl_trait: &'static str,

        #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() })]
        try_required: u32,

        #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() })]
        try_optional: Option<u32>,

        #[builder(with = |value: &str| -> ParseIntResult<_> { value.parse() }, default)]
        try_default: u32,

        #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) })]
        try_optional_generic: Option<T>,

        #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) }, default)]
        try_default_generic: T,

        #[builder(with = |value: impl Into<&'static str>| -> Result<_, Infallible> { Ok(value.into()) })]
        try_impl_trait: &'static str,
    }

    let builder = Sut::builder()
        .required(1)
        .optional(2)
        .default(3)
        .generic(&"hello");

    let _ignore = builder.clone().optional_generic(&"hello you");
    let builder = builder.maybe_optional_generic(Some(&"littlepip"));

    let _ignore = builder.clone().default_generic(&"blackjack");
    let builder = builder.maybe_default_generic(Some(&"<3"));

    let builder = builder
        .impl_trait(IntoStrRef("morning glory"))
        .try_required("4")
        .unwrap();

    let _ignore = builder.clone().try_optional("5").unwrap();
    let builder = builder.maybe_try_optional(Some("6")).unwrap();

    let _ignore = builder.clone().try_default("7").unwrap();
    let builder = builder.maybe_try_default(Some("8")).unwrap();

    let _ignore = builder.clone().try_optional_generic(&"9").unwrap();
    let builder = builder.maybe_try_optional_generic(Some(&"10")).unwrap();

    let _ignore = builder.clone().try_default_generic(&"11").unwrap();
    let builder = builder.maybe_try_default_generic(Some(&"12")).unwrap();

    let builder = builder.try_impl_trait(IntoStrRef("morning glory")).unwrap();

    assert_debug_eq(
        builder.build(),
        expect![[r#"
            Sut {
                required: 2,
                optional: Some(
                    3,
                ),
                default: 4,
                generic: "hello",
                optional_generic: Some(
                    "littlepip",
                ),
                default_generic: "<3",
                impl_trait: "morning glory",
                try_required: 4,
                try_optional: Some(
                    6,
                ),
                try_default: 8,
                try_optional_generic: Some(
                    "10",
                ),
                try_default_generic: "12",
                try_impl_trait: "morning glory",
            }"#]],
    );
}

#[test]
fn test_function() {
    #[builder(derive(Clone))]
    fn sut<T: Default + Clone + fmt::Debug>(
        #[builder(with = |x: u32| x + 1)] required: u32,
        #[builder(with = |x: u32| x + 1)] optional: Option<u32>,
        #[builder(with = |x: u32| x + 1, default)] default: u32,
        #[builder(with = |value: &T| value.clone())] generic: T,
        #[builder(with = |value: &T| value.clone())] optional_generic: Option<T>,
        #[builder(with = |value: &T| value.clone(), default)] default_generic: T,
        #[builder(with = |value: impl Into<&'static str>| value.into())] impl_trait: &'static str,

        #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() })]
        try_required: u32,

        #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() })]
        try_optional: Option<u32>,

        #[builder(with = |value: &str| -> ParseIntResult<_> { value.parse() }, default)]
        try_default: u32,

        #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) })]
        try_optional_generic: Option<T>,

        #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) }, default)]
        try_default_generic: T,

        #[builder(with = |value: impl Into<&'static str>| -> Result<_, Infallible> { Ok(value.into()) })]
        try_impl_trait: &'static str,
    ) -> impl fmt::Debug {
        (
            (
                required,
                optional,
                default,
                generic,
                optional_generic,
                default_generic,
                impl_trait,
            ),
            (
                try_required,
                try_optional,
                try_default,
                try_optional_generic,
                try_default_generic,
                try_impl_trait,
            ),
        )
    }

    let builder = sut().required(1).optional(2).default(3).generic(&"hello");

    let _ignore = builder.clone().optional_generic(&"hello you");
    let builder = builder.maybe_optional_generic(Some(&"littlepip"));

    let _ignore = builder.clone().default_generic(&"blackjack");
    let builder = builder.maybe_default_generic(Some(&"<3"));

    let builder = builder
        .impl_trait(IntoStrRef("morning glory"))
        .try_required("4")
        .unwrap();

    let _ignore = builder.clone().try_optional("5").unwrap();
    let builder = builder.maybe_try_optional(Some("6")).unwrap();

    let _ignore = builder.clone().try_default("7").unwrap();
    let builder = builder.maybe_try_default(Some("8")).unwrap();

    let _ignore = builder.clone().try_optional_generic(&"9").unwrap();
    let builder = builder.maybe_try_optional_generic(Some(&"10")).unwrap();

    let _ignore = builder.clone().try_default_generic(&"11").unwrap();
    let builder = builder.maybe_try_default_generic(Some(&"12")).unwrap();

    let builder = builder.try_impl_trait(IntoStrRef("morning glory")).unwrap();

    assert_debug_eq(
        builder.call(),
        expect![[r#"
            (
                (
                    2,
                    Some(
                        3,
                    ),
                    4,
                    "hello",
                    Some(
                        "littlepip",
                    ),
                    "<3",
                    "morning glory",
                ),
                (
                    4,
                    Some(
                        6,
                    ),
                    8,
                    Some(
                        "10",
                    ),
                    "12",
                    "morning glory",
                ),
            )"#]],
    );
}

#[test]
fn test_method() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(derive(Clone))]
        fn sut<T: Default + Clone + fmt::Debug>(
            #[builder(with = |x: u32| x + 1)] required: u32,
            #[builder(with = |x: u32| x + 1)] optional: Option<u32>,
            #[builder(with = |x: u32| x + 1, default)] default: u32,
            #[builder(with = |value: &T| value.clone())] generic: T,
            #[builder(with = |value: &T| value.clone())] optional_generic: Option<T>,
            #[builder(with = |value: &T| value.clone(), default)] default_generic: T,
            #[builder(with = |value: impl Into<&'static str>| value.into())]
            impl_trait: &'static str,

            #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() })]
            try_required: u32,

            #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() })]
            try_optional: Option<u32>,

            #[builder(with = |value: &str| -> ParseIntResult<_> { value.parse() }, default)]
            try_default: u32,

            #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) })]
            try_optional_generic: Option<T>,

            #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) }, default)]
            try_default_generic: T,

            #[builder(with = |value: impl Into<&'static str>| -> Result<_, Infallible> { Ok(value.into()) })]
            try_impl_trait: &'static str,
        ) -> impl fmt::Debug {
            (
                (
                    required,
                    optional,
                    default,
                    generic,
                    optional_generic,
                    default_generic,
                    impl_trait,
                ),
                (
                    try_required,
                    try_optional,
                    try_default,
                    try_optional_generic,
                    try_default_generic,
                    try_impl_trait,
                ),
            )
        }

        #[builder(derive(Clone))]
        fn with_self<T: Default + Clone + fmt::Debug>(
            &self,
            #[builder(with = |x: u32| x + 1)] required: u32,
            #[builder(with = |x: u32| x + 1)] optional: Option<u32>,
            #[builder(with = |x: u32| x + 1, default)] default: u32,
            #[builder(with = |value: &T| value.clone())] generic: T,
            #[builder(with = |value: &T| value.clone())] optional_generic: Option<T>,
            #[builder(with = |value: &T| value.clone(), default)] default_generic: T,
            #[builder(with = |value: impl Into<&'static str>| value.into())]
            impl_trait: &'static str,

            #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() })]
            try_required: u32,

            #[builder(with = |value: &str| -> Result<_, num::ParseIntError> { value.parse() })]
            try_optional: Option<u32>,

            #[builder(with = |value: &str| -> ParseIntResult<_> { value.parse() }, default)]
            try_default: u32,

            #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) })]
            try_optional_generic: Option<T>,

            #[builder(with = |value: &T| -> Result<_, Infallible> { Ok(value.clone()) }, default)]
            try_default_generic: T,

            #[builder(with = |value: impl Into<&'static str>| -> Result<_, Infallible> { Ok(value.into()) })]
            try_impl_trait: &'static str,
        ) -> impl fmt::Debug {
            let _ = self;
            (
                (
                    required,
                    optional,
                    default,
                    generic,
                    optional_generic,
                    default_generic,
                    impl_trait,
                ),
                (
                    try_required,
                    try_optional,
                    try_default,
                    try_optional_generic,
                    try_default_generic,
                    try_impl_trait,
                ),
            )
        }
    }

    let builder = Sut::sut()
        .required(1)
        .optional(2)
        .default(3)
        .generic(&"hello");

    let _ignore = builder.clone().optional_generic(&"hello you");
    let builder = builder.maybe_optional_generic(Some(&"littlepip"));

    let _ignore = builder.clone().default_generic(&"blackjack");
    let builder = builder.maybe_default_generic(Some(&"<3"));

    let builder = builder
        .impl_trait(IntoStrRef("morning glory"))
        .try_required("4")
        .unwrap();

    let _ignore = builder.clone().try_optional("5").unwrap();
    let builder = builder.maybe_try_optional(Some("6")).unwrap();

    let _ignore = builder.clone().try_default("7").unwrap();
    let builder = builder.maybe_try_default(Some("8")).unwrap();

    let _ignore = builder.clone().try_optional_generic(&"9").unwrap();
    let builder = builder.maybe_try_optional_generic(Some(&"10")).unwrap();

    let _ignore = builder.clone().try_default_generic(&"11").unwrap();
    let builder = builder.maybe_try_default_generic(Some(&"12")).unwrap();

    let builder = builder.try_impl_trait(IntoStrRef("morning glory")).unwrap();

    assert_debug_eq(
        builder.call(),
        expect![[r#"
            (
                (
                    2,
                    Some(
                        3,
                    ),
                    4,
                    "hello",
                    Some(
                        "littlepip",
                    ),
                    "<3",
                    "morning glory",
                ),
                (
                    4,
                    Some(
                        6,
                    ),
                    8,
                    Some(
                        "10",
                    ),
                    "12",
                    "morning glory",
                ),
            )"#]],
    );

    let builder = Sut
        .with_self()
        .required(1)
        .optional(2)
        .default(3)
        .generic(&"hello");

    let _ignore = builder.clone().optional_generic(&"hello you");
    let builder = builder.maybe_optional_generic(Some(&"littlepip"));

    let _ignore = builder.clone().default_generic(&"blackjack");
    let builder = builder.maybe_default_generic(Some(&"<3"));

    let builder = builder
        .impl_trait(IntoStrRef("morning glory"))
        .try_required("4")
        .unwrap();

    let _ignore = builder.clone().try_optional("5").unwrap();
    let builder = builder.maybe_try_optional(Some("6")).unwrap();

    let _ignore = builder.clone().try_default("7").unwrap();
    let builder = builder.maybe_try_default(Some("8")).unwrap();

    let _ignore = builder.clone().try_optional_generic(&"9").unwrap();
    let builder = builder.maybe_try_optional_generic(Some(&"10")).unwrap();

    let _ignore = builder.clone().try_default_generic(&"11").unwrap();
    let builder = builder.maybe_try_default_generic(Some(&"12")).unwrap();

    let builder = builder.try_impl_trait(IntoStrRef("morning glory")).unwrap();

    assert_debug_eq(
        builder.call(),
        expect![[r#"
            (
                (
                    2,
                    Some(
                        3,
                    ),
                    4,
                    "hello",
                    Some(
                        "littlepip",
                    ),
                    "<3",
                    "morning glory",
                ),
                (
                    4,
                    Some(
                        6,
                    ),
                    8,
                    Some(
                        "10",
                    ),
                    "12",
                    "morning glory",
                ),
            )"#]],
    );
}
