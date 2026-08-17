mod attr_impl;
mod config;
mod derive_impl;
pub(crate) mod util;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error};

use crate::{
    attr_impl::docs_const_impl,
    derive_impl::{documented_fields_impl, documented_impl, documented_variants_impl, DocType},
};

/// Derive proc-macro for `Documented` trait.
///
/// # Example
///
/// ```rust
/// use documented::Documented;
///
/// /// Nice.
/// /// Multiple single-line doc comments are supported.
/// ///
/// /** Multi-line doc comments are supported too.
///     Each line of the multi-line block is individually trimmed by default.
///     Note the lack of spaces in front of this line.
/// */
/// #[doc = "Attribute-style documentation is supported too."]
/// #[derive(Documented)]
/// struct BornIn69;
///
/// let doc_str = "Nice.
/// Multiple single-line doc comments are supported.
///
/// Multi-line doc comments are supported too.
/// Each line of the multi-line block is individually trimmed by default.
/// Note the lack of spaces in front of this line.
///
/// Attribute-style documentation is supported too.";
/// assert_eq!(BornIn69::DOCS, doc_str);
/// ```
///
/// # Configuration
///
/// With the `customise` feature enabled, you can customise this macro's
/// behaviour using the `#[documented(...)]` attribute.
///
/// Currently, you can:
///
/// ## 1. set a default value when doc comments are absent like so:
///
/// ```rust
/// # use documented::Documented;
/// #[derive(Documented)]
/// #[documented(default = "The answer is fries.")]
/// struct WhosTurnIsIt;
///
/// assert_eq!(WhosTurnIsIt::DOCS, "The answer is fries.");
/// ```
///
/// This option is primarily designed for [`DocumentedFields`] and
/// [`DocumentedVariants`], so it's probably not very useful here. But it could
/// conceivably come in handy in some niche meta-programming contexts.
///
/// ## 2. disable line-trimming like so:
///
/// ```rust
/// # use documented::Documented;
/// ///     Terrible.
/// #[derive(Documented)]
/// #[documented(trim = false)]
/// struct Frankly;
///
/// assert_eq!(Frankly::DOCS, "     Terrible.");
/// ```
///
/// If there are other configuration options you wish to have, please submit an
/// issue or a PR.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(Documented))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(Documented, attributes(documented))
)]
pub fn documented(input: TokenStream) -> TokenStream {
    documented_impl(parse_macro_input!(input), DocType::Str)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedOpt` trait.
///
/// See [`Documented`] for usage.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedOpt))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedOpt, attributes(documented))
)]
pub fn documented_opt(input: TokenStream) -> TokenStream {
    documented_impl(parse_macro_input!(input), DocType::OptStr)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedFields` trait.
///
/// # Example
///
/// ```rust
/// use documented::DocumentedFields;
///
/// #[derive(DocumentedFields)]
/// struct BornIn69 {
///     /// Cry like a grandmaster.
///     rawr: String,
///     /// Before what?
///     explosive: usize,
/// };
///
/// assert_eq!(
///     BornIn69::FIELD_DOCS,
///     ["Cry like a grandmaster.", "Before what?"]
/// );
/// ```
///
/// You can also use `DocumentedFields::get_field_docs` to access the fields'
/// documentation using their names.
///
/// ```rust
/// # use documented::{DocumentedFields, Error};
/// #
/// # #[derive(DocumentedFields)]
/// # struct BornIn69 {
/// #     /// Cry like a grandmaster.
/// #     rawr: String,
/// #     /// Before what?
/// #     explosive: usize,
/// # };
/// #
/// assert_eq!(
///     BornIn69::get_field_docs("rawr"),
///     Ok("Cry like a grandmaster.")
/// );
/// assert_eq!(BornIn69::get_field_docs("explosive"), Ok("Before what?"));
/// assert_eq!(
///     BornIn69::get_field_docs("gotcha"),
///     Err(Error::NoSuchField("gotcha".to_string()))
/// );
/// ```
///
/// # Configuration
///
/// With the `customise` feature enabled, you can customise this macro's
/// behaviour using the `#[documented_fields(...)]` attribute. Note that this
/// attribute works on both the container and each individual field, with the
/// per-field configurations overriding container configurations, which
/// override the default.
///
/// Currently, you can:
///
/// ## 1. set a different case convention for `get_field_docs` like so:
///
/// ```rust
/// # use documented::DocumentedFields;
/// #[derive(DocumentedFields)]
/// #[documented_fields(rename_all = "kebab-case")]
/// struct BooksYouShouldWrite {
///     /// It's my move?
///     whose_turn_is_it: String,
///     /// Isn't it checkmate?
///     #[documented_fields(rename_all = "PascalCase")]
///     how_many_legal_moves_do_you_have: String,
/// }
///
/// assert_eq!(
///     BooksYouShouldWrite::get_field_docs("whose-turn-is-it"),
///     Ok("It's my move?")
/// );
/// assert_eq!(
///     BooksYouShouldWrite::get_field_docs("HowManyLegalMovesDoYouHave"),
///     Ok("Isn't it checkmate?")
/// );
/// ```
///
/// ## 2. set a custom name for a specific field for `get_field_docs` like so:
///
/// ```rust
/// # use documented::DocumentedFields;
/// #[derive(DocumentedFields)]
/// // #[documented_field(rename = "fries")] // this is not allowed
/// struct ThisPosition {
///     /// I'm guessing, but I'm not really guessing.
///     #[documented_fields(rename = "knows")]
///     esserman_knows: bool,
///     /// And that's true if you're van Wely.
///     #[documented_fields(rename = "doesnt_know")]
///     van_wely_doesnt: bool,
/// }
///
/// assert_eq!(
///     ThisPosition::get_field_docs("knows"),
///     Ok("I'm guessing, but I'm not really guessing.")
/// );
/// assert_eq!(
///     ThisPosition::get_field_docs("doesnt_know"),
///     Ok("And that's true if you're van Wely.")
/// );
/// ```
///
/// Obviously this option is only available on each individual field.
/// It makes no sense on the container.
///
/// This option also always takes priority over `rename_all`.
///
/// ## 3. set a default value when doc comments are absent like so:
///
/// ```rust
/// # use documented::DocumentedFields;
/// #[derive(DocumentedFields)]
/// #[documented_fields(default = "Confusing the audience.")]
/// struct SettingUpForTheNextGame {
///     rh8: bool,
///     ng8: bool,
///     /// Always play:
///     bf8: bool,
/// }
///
/// assert_eq!(
///     SettingUpForTheNextGame::FIELD_DOCS,
///     [
///         "Confusing the audience.",
///         "Confusing the audience.",
///         "Always play:"
///     ]
/// );
/// ```
///
/// ## 4. (selectively) disable line-trimming like so:
///
/// ```rust
/// # use documented::DocumentedFields;
/// #[derive(DocumentedFields)]
/// #[documented_fields(trim = false)]
/// struct Frankly {
///     ///     Delicious.
///     perrier: usize,
///     ///     I'm vegan.
///     #[documented_fields(trim = true)]
///     fried_liver: bool,
/// }
///
/// assert_eq!(Frankly::FIELD_DOCS, ["     Delicious.", "I'm vegan."]);
/// ```
///
/// If there are other configuration options you wish to have, please
/// submit an issue or a PR.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedFields))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedFields, attributes(documented_fields))
)]
pub fn documented_fields(input: TokenStream) -> TokenStream {
    documented_fields_impl(parse_macro_input!(input), DocType::Str)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedFieldsOpt` trait.
///
/// See [`DocumentedFields`] for usage.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedFieldsOpt))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedFieldsOpt, attributes(documented_fields))
)]
pub fn documented_fields_opt(input: TokenStream) -> TokenStream {
    documented_fields_impl(parse_macro_input!(input), DocType::OptStr)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedVariants` trait.
///
/// # Example
///
/// ```rust
/// use documented::{DocumentedVariants, Error};
///
/// #[derive(DocumentedVariants)]
/// enum NeverPlay {
///     /// Terrible.
///     F3,
///     /// I fell out of my chair.
///     F6,
/// }
///
/// assert_eq!(NeverPlay::F3.get_variant_docs(), "Terrible.");
/// assert_eq!(NeverPlay::F6.get_variant_docs(), "I fell out of my chair.");
/// ```
///
/// # Configuration
///
/// With the `customise` feature enabled, you can customise this macro's
/// behaviour using the `#[documented_variants(...)]` attribute. Note that this
/// attribute works on both the container and each individual variant, with the
/// per-variant configurations overriding container configurations, which
/// override the default.
///
/// Currently, you can:
///
/// ## 1. set a default value when doc comments are absent like so:
///
/// ```rust
/// # use documented::DocumentedVariants;
/// #[derive(DocumentedVariants)]
/// #[documented_variants(default = "Still theory.")]
/// enum OurHeroPlayed {
///     G4Mate,
///     OOOMate,
///     /// Frankly ridiculous.
///     Bf1g2Mate,
/// }
///
/// assert_eq!(OurHeroPlayed::G4Mate.get_variant_docs(), "Still theory.");
/// assert_eq!(OurHeroPlayed::OOOMate.get_variant_docs(), "Still theory.");
/// assert_eq!(
///     OurHeroPlayed::Bf1g2Mate.get_variant_docs(),
///     "Frankly ridiculous."
/// );
/// ```
///
/// ## 2. (selectively) disable line-trimming like so:
///
/// ```rust
/// # use documented::DocumentedVariants;
/// #[derive(DocumentedVariants)]
/// #[documented_variants(trim = false)]
/// enum Always {
///     ///     Or the quality.
///     SacTheExchange,
///     ///     Like a Frenchman.
///     #[documented_variants(trim = true)]
///     Retreat,
/// }
/// assert_eq!(
///     Always::SacTheExchange.get_variant_docs(),
///     "     Or the quality."
/// );
/// assert_eq!(Always::Retreat.get_variant_docs(), "Like a Frenchman.");
/// ```
///
/// If there are other configuration options you wish to have, please
/// submit an issue or a PR.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedVariants))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedVariants, attributes(documented_variants))
)]
pub fn documented_variants(input: TokenStream) -> TokenStream {
    documented_variants_impl(parse_macro_input!(input), DocType::Str)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive proc-macro for `DocumentedVariantsOpt` trait.
///
/// See [`DocumentedVariants`] for usage.
#[cfg_attr(not(feature = "customise"), proc_macro_derive(DocumentedVariantsOpt))]
#[cfg_attr(
    feature = "customise",
    proc_macro_derive(DocumentedVariantsOpt, attributes(documented_variants))
)]
pub fn documented_variants_opt(input: TokenStream) -> TokenStream {
    documented_variants_impl(parse_macro_input!(input), DocType::OptStr)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Macro to extract the documentation on any item that accepts doc comments
/// and store it in a const variable.
///
/// By default, this const variable inherits visibility from its parent item.
/// This can be manually configured; see configuration section below.
///
/// # Examples
///
/// ```rust
/// use documented::docs_const;
///
/// /// This is a test function
/// #[docs_const]
/// fn test_fn() {}
///
/// assert_eq!(TEST_FN_DOCS, "This is a test function");
/// ```
///
/// # Configuration
///
/// With the `customise` feature enabled, you can customise this macro's
/// behaviour using attribute arguments.
///
/// Currently, you can:
///
/// ## 1. set a custom constant visibility like so:
///
/// ```rust
/// mod submodule {
///     # use documented::docs_const;
///     /// Boo!
///     #[docs_const(vis = pub)]
///     struct Wooooo;
/// }
///
/// // notice how the constant can be seen from outside
/// assert_eq!(submodule::WOOOOO_DOCS, "Boo!");
/// ```
///
/// ## 2. set a custom constant name like so:
///
/// ```rust
/// # use documented::docs_const;
/// /// If you have a question raise your hand
/// #[docs_const(rename = "DONT_RAISE_YOUR_HAND")]
/// mod whatever {}
///
/// assert_eq!(DONT_RAISE_YOUR_HAND, "If you have a question raise your hand");
/// ```
///
/// ## 3. set a default value when doc comments are absent like so:
///
/// ```rust
/// use documented::docs_const;
///
/// #[docs_const(default = "In this position many of you blunder.")]
/// trait StartingPosition {}
///
/// assert_eq!(
///     STARTING_POSITION_DOCS,
///     "In this position many of you blunder."
/// );
/// ```
///
/// This option is primarily designed for [`DocumentedFields`] and
/// [`DocumentedVariants`], so it's probably not very useful here. But it could
/// conceivably come in handy in some niche meta-programming contexts.
///
/// ## 4. disable line-trimming like so:
///
/// ```rust
/// # use documented::docs_const;
/// ///     This is a test constant
/// #[docs_const(trim = false)]
/// const test_const: u8 = 0;
///
/// assert_eq!(TEST_CONST_DOCS, "     This is a test constant");
/// ```
///
/// ---
///
/// Multiple option can be specified in a list like so:
/// `name = "FOO", trim = false`.
///
/// If there are other configuration options you wish to have, please
/// submit an issue or a PR.
#[proc_macro_attribute]
pub fn docs_const(#[allow(unused_variables)] attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(not(feature = "customise"))]
    let ts = docs_const_impl(parse_macro_input!(item));
    #[cfg(feature = "customise")]
    let ts = docs_const_impl(parse_macro_input!(item), parse_macro_input!(attr));

    ts.unwrap_or_else(Error::into_compile_error).into()
}
