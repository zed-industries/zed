use syn::{parse_macro_input, DeriveInput};

mod body_impl;
mod default_attr;
mod util;

/// # Smart Default
///
/// This crate provides a custom derive for `SmartDefault`. `SmartDefault` is not a real trait -
/// deriving it will actually `impl Default`. The difference from regular `#[derive(Default)]` is
/// that `#[derive(SmartDefault)]` allows you to use `#[default = "..."]` attributes to customize
/// the `::default()` method and to support `struct`s that don't have `Default` for all their
/// fields - and even `enum`s!
///
/// # Examples
///
/// ```
/// use smart_default::SmartDefault;
///
/// # fn main() {
/// #[derive(SmartDefault)]
/// # #[derive(PartialEq)]
/// # #[allow(dead_code)]
/// enum Foo {
///     Bar,
///     #[default]
///     Baz {
///         #[default = 12]
///         a: i32,
///         b: i32,
///         #[default(Some(Default::default()))]
///         c: Option<i32>,
///         #[default(_code = "vec![1, 2, 3]")]
///         d: Vec<u32>,
///         #[default = "four"]
///         e: String,
///     },
///     Qux(i32),
/// }
///
/// assert!(Foo::default() == Foo::Baz {
///     a: 12,
///     b: 0,
///     c: Some(0),
///     d: vec![1, 2, 3],
///     e: "four".to_owned(),
/// });
/// # }
/// ```
///
/// * `Baz` has the `#[default]` attribute. This means that the default `Foo` is a `Foo::Baz`. Only
///   one variant may have a `#[default]` attribute, and that attribute must have no value.
/// * `a` has a `#[default = 12]` attribute. This means that it's default value is `12`.
/// * `b` has no `#[default = ...]` attribute. It's default value will `i32`'s default value
///   instead - `0`.
/// * `c` is an `Option<i32>`, and it's default is `Some(Default::default())`. Rust cannot (currently)
///   parse `#[default = Some(Default::default())]` and therefore we have to use a special syntax:
///   `#[default(Some(Default::default))]`
/// * `d` has the `!` token in it, which cannot (currently) be parsed even with `#[default(...)]`,
///   so we have to encode it as a string and mark it as `_code = `.
/// * `e` is a `String`, so the string literal "four" is automatically converted to it. This
///   automatic conversion **only** happens to string (or byte string) literals - and only if
///   `_code` is not used.
/// * Documentation for the `impl Default` section is generated automatically, specifying the
///   default value returned from `::default()`.
#[proc_macro_derive(SmartDefault, attributes(default))]
pub fn derive_smart_default(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match body_impl::impl_my_derive(&input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
