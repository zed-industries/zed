#![doc = include_str!("../README.md")]
#![allow(
    clippy::redundant_pub_crate,
    clippy::wildcard_imports,
    clippy::map_unwrap_or,
    clippy::items_after_statements,
    clippy::missing_const_for_fn,
    clippy::option_option,
    clippy::option_if_let_else,
    clippy::enum_glob_use,
    clippy::too_many_lines,
    clippy::if_not_else,

    // This crate doesn't expose complex public function signatures, any occurrences
    // of `&Option<T>` are internal and they are likely just small private functions
    // that take a reference to an `Option<T>` to make the callsite cleaner avoiding
    // the unnecessary `Option::as_ref()` calls.
    clippy::ref_option,

    // We can't use the explicit captures syntax due to the MSRV
    impl_trait_overcaptures,

    // There are too many false-positives for syn::Ident
    if_let_rescope,
)]

mod bon;
mod builder;
mod collections;
mod error;
mod normalization;
mod parsing;
mod privatize;
mod util;

#[cfg(test)]
mod tests;

/// Generates a builder for the function or method it's placed on.
///
/// ## Quick examples
///
/// You can turn a function with positional parameters into a function with
/// named parameters just by placing the `#[builder]` attribute on top of it.
///
/// ```rust ignore
/// use bon::builder;
///
/// #[builder]
/// fn greet(name: &str, level: Option<u32>) -> String {
///     let level = level.unwrap_or(0);
///
///     format!("Hello {name}! Your level is {level}")
/// }
///
/// let greeting = greet()
///     .name("Bon")
///     .level(24) // <- setting `level` is optional, we could omit it
///     .call();
///
/// assert_eq!(greeting, "Hello Bon! Your level is 24");
/// ```
///
/// You can also use the `#[builder]` attribute with associated methods:
///
/// ```rust ignore
/// use bon::bon;
///
/// struct User {
///     id: u32,
///     name: String,
/// }
///
/// #[bon] // <- this attribute is required on impl blocks that contain `#[builder]`
/// impl User {
///     #[builder]
///     fn new(id: u32, name: String) -> Self {
///         Self { id, name }
///     }
///
///     #[builder]
///     fn greet(&self, target: &str, level: Option<&str>) -> String {
///         let level = level.unwrap_or("INFO");
///         let name = &self.name;
///
///         format!("[{level}] {name} says hello to {target}")
///     }
/// }
///
/// // The method named `new` generates `builder()/build()` methods
/// let user = User::builder()
///     .id(1)
///     .name("Bon".to_owned())
///     .build();
///
/// // All other methods generate `method_name()/call()` methods
/// let greeting = user
///     .greet()
///     .target("the world")
///     // `level` is optional, we can omit it here
///     .call();
///
/// assert_eq!(user.id, 1);
/// assert_eq!(user.name, "Bon");
/// assert_eq!(greeting, "[INFO] Bon says hello to the world");
/// ```
///
/// The builder never panics. Any mistakes such as missing required fields
/// or setting the same field twice will be reported as compile-time errors.
///
/// See the full documentation for more details:
/// - [Guide](https://bon-rs.com/guide/overview)
/// - [Attributes reference](https://bon-rs.com/reference/builder)
#[proc_macro_attribute]
pub fn builder(
    params: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    builder::generate_from_attr(params.into(), item.into()).into()
}

/// Derives a builder for the struct it's placed on.
///
/// ## Quick example
///
/// Add a `#[derive(Builder)]` attribute to your struct to generate a `builder()` method for it.
///
/// ```rust ignore
/// use bon::{bon, builder, Builder};
///
/// #[derive(Builder)]
/// struct User {
///     name: String,
///     is_admin: bool,
///     level: Option<u32>,
/// }
///
/// let user = User::builder()
///     .name("Bon".to_owned())
///     // `level` is optional, we could omit it here
///     .level(24)
///     // call setters in any order
///     .is_admin(true)
///     .build();
///
/// assert_eq!(user.name, "Bon");
/// assert_eq!(user.level, Some(24));
/// assert!(user.is_admin);
/// ```
///
/// The builder never panics. Any mistakes such as missing required fields
/// or setting the same field twice will be reported as compile-time errors.
///
/// See the full documentation for more details:
/// - [Guide](https://bon-rs.com/guide/overview)
/// - [Attributes reference](https://bon-rs.com/reference/builder)
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    builder::generate_from_derive(item.into()).into()
}

/// Companion macro for [`builder`]. You should place it on top of the `impl` block
/// where you want to define methods with the [`builder`] macro.
///
/// It provides the necessary context to the [`builder`] macros on top of the functions
/// inside of the `impl` block. You'll get compile errors without that context.
///
/// # Quick example
///
/// ```rust ignore
/// use bon::bon;
///
/// struct User {
///     id: u32,
///     name: String,
/// }
///
/// #[bon] // <- this attribute is required on impl blocks that contain `#[builder]`
/// impl User {
///     #[builder]
///     fn new(id: u32, name: String) -> Self {
///         Self { id, name }
///     }
///
///     #[builder]
///     fn greet(&self, target: &str, level: Option<&str>) -> String {
///         let level = level.unwrap_or("INFO");
///         let name = &self.name;
///
///         format!("[{level}] {name} says hello to {target}")
///     }
/// }
///
/// // The method named `new` generates `builder()/build()` methods
/// let user = User::builder()
///     .id(1)
///     .name("Bon".to_owned())
///     .build();
///
/// // All other methods generate `method_name()/call()` methods
/// let greeting = user
///     .greet()
///     .target("the world")
///     // `level` is optional, we can omit it here
///     .call();
///
/// assert_eq!(user.id, 1);
/// assert_eq!(user.name, "Bon");
/// assert_eq!(greeting, "[INFO] Bon says hello to the world");
/// ```
///
/// The builder never panics. Any mistakes such as missing required fields
/// or setting the same field twice will be reported as compile-time errors.
///
/// For details on this macro [see the overview](https://bon-rs.com/guide/overview).
///
/// [`builder`]: macro@builder
#[proc_macro_attribute]
pub fn bon(
    params: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    bon::generate(params.into(), item.into()).into()
}

/// Creates any map-like collection that implements [`FromIterator<(K, V)>`].
///
/// It automatically converts each key and value to the target type using [`Into`].
/// This way you can write a map of `String`s without the need to call `.to_owned()`
/// or `.to_string()` on every string literal:
///
/// ```rust
/// # use bon_macros as bon;
/// # use std::collections::HashMap;
/// let map: HashMap<String, String> = bon::map! {
///     "key1": "value1",
///     format!("key{}", 2): "value2",
///     "key3": format!("value{}", 3),
/// };
/// ```
///
/// There is no separate variant for [`BTreeMap`] and [`HashMap`]. Instead, you
/// should annotate the return type of this macro with the desired type or make
/// sure the compiler can infer the collection type from other context.
///
/// # Compile errors
///
/// The macro conservatively rejects duplicate keys in the map with a compile error.
/// This check works for very simple expressions that involve only literal values.
///
/// ```rust compile_fail
/// # use bon_macros as bon;
/// # use std::collections::HashMap;
/// let map: HashMap<String, String> = bon::map! {
///     "key1": "value1",
///     "key2": "value2"
///     "key1": "value3", // compile error: `duplicate key in the map`
/// };
/// ```
///
/// [`FromIterator<(K, V)>`]: https://doc.rust-lang.org/stable/std/iter/trait.FromIterator.html
/// [`Into`]: https://doc.rust-lang.org/stable/std/convert/trait.Into.html
/// [`BTreeMap`]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html
/// [`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
#[proc_macro]
pub fn map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let entries = syn::parse_macro_input!(input with collections::map::parse_macro_input);

    collections::map::generate(entries).into()
}

/// Creates any set-like collection that implements [`FromIterator<T>`].
///
/// It automatically converts each value to the target type using [`Into`].
/// This way you can write a set of `String`s without the need to call `.to_owned()`
/// or `.to_string()` on every string literal:
///
/// ```rust
/// # use bon_macros as bon;
/// # use std::collections::HashSet;
/// let set: HashSet<String> = bon::set![
///     "value1",
///     format!("value{}", 2),
///     "value3",
/// ];
/// ```
///
/// There is no separate variant for [`BTreeSet`] and [`HashSet`]. Instead, you
/// should annotate the return type of this macro with the desired type or make
/// sure the compiler can infer the collection type from other context.
///
/// # Compile errors
///
/// The macro conservatively rejects duplicate values in the set with a compile error.
/// This check works for very simple expressions that involve only literal values.
///
/// ```rust compile_fail
/// # use bon_macros as bon;
/// # use std::collections::HashSet;
/// let set: HashSet<String> = bon::set![
///     "value1",
///     "value2"
///     "value1", // compile error: `duplicate value in the set`
/// ];
/// ```
///
/// [`FromIterator<T>`]: https://doc.rust-lang.org/stable/std/iter/trait.FromIterator.html
/// [`Into`]: https://doc.rust-lang.org/stable/std/convert/trait.Into.html
/// [`BTreeSet`]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html
/// [`HashSet`]: https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html
#[proc_macro]
pub fn set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::punctuated::Punctuated;

    let entries = syn::parse_macro_input!(input with Punctuated::parse_terminated);

    collections::set::generate(entries).into()
}

// Private implementation detail. Don't use it directly!
// This attribute renames the function provided to it to `__orig_{fn_name}`
#[doc(hidden)]
#[proc_macro_attribute]
pub fn __privatize(
    _params: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    privatize::privatize_fn(input.into()).into()
}
