/// Derives const debug formatting for a type.
///
/// Derives the [`FormatMarker`] trait, and defines an `const_debug_fmt` inherent
/// method to format a type at compile-time.
/// 
/// # Features 
/// 
/// This derive macro is only available with the "derive" feature,
/// and Rust 1.83.0, because is uses mutable references in const.
///
/// # Limitations
///
/// Compile-time formatting currently imposes these limitations on users,
/// this derive macro has some mitigations for some of them.
///
/// ### Generic impls
///
/// Because the formatting of custom types is implemented with duck typing,
/// it's not possible to format generic types, instead you must do either of these:
///
/// - Provide all the implementations ahead of time, what the [`impls attribute`] is for.
///
/// - Provide a macro that formats the type.
/// The `call_debug_fmt` macro is a version of this that formats generic std types,
/// then it can be used to format fields of the type with the 
/// [`#[cdeb(with_macro = "....")]`](#cdeb_with_macro) attribute.
///
/// These are the things that this macro does to mitigate the limitations:
///
/// - Allows users to provide a function/macro/wrapper to format a field.
///
/// - Automatically detect some builtin/standard library types that are generic.
///
/// - Allow users to ignore a field.
///
/// # Container Attributes 
///
/// These attributes go on the type itself, rather than the fields.
///
/// ### `#[cdeb(debug_print)]`
///
/// Panics with the output of the expanded derive.
///
/// ### `#[cdeb(impls(....))]`
///
/// Allows users to implement const debug formatting for multiple different
/// concrete instances of the type.
/// 
/// When this attribute is used it disables the default implementation
/// that uses the type parameters generically.
///
/// Example:
/// 
/// ```rust
/// #[derive(const_format::ConstDebug)]
/// #[cdeb(impls(
///     "Foo<u8, u64>",
///     "<T> Foo<u16, T>",
///     "<T> Foo<u32, T> where T: 'static",
/// ))]
/// struct Foo<A, B>(A, *const B);
/// ```
///
/// In this example, there's exactly three impls of 
/// the `const_debug_fmt` method and [`FormatMarker`] trait.
///
/// ### `#[cdeb(crate = "foo::bar")]`
///
/// The path to the `const_format` crate, useful if you want to reexport the ConstDebug macro,
/// or rename the `const_format` crate in the Cargo.toml .
///
/// Example of renaming the `const_format` crate in the Cargo.toml file:
/// ```toml
/// cfmt = {version = "0.*", package = "const_format"}
/// ```
///
/// # Field attributes
///
/// ### `#[cdeb(ignore)]`
///
/// Ignores the field, pretending that it doesn't exist.
///
/// ### `#[cdeb(with = "module::function")]`
///
/// Uses the function at the passed-in path to format the field.
///
/// The function is expected to have this signature:
/// ```ignored
/// const fn(&FieldType, &mut const_format::Formatter<'_>) -> Result<(), const_format::Error>
/// ```
///
/// <span id = "cdeb_with_macro"> </span>
///
/// ### `#[cdeb(with_macro = "module::the_macro")]`
///
/// Uses the macro at the passed-in path to format the field.
///
/// The macro is expected to be callable like a function with this signature: 
/// ```ignored
/// const fn(&FieldType, &mut const_format::Formatter<'_>) -> Result<(), const_format::Error>
/// ```
/// 
/// ### `#[cdeb(with_wrapper = "module::Wrapper")]`
/// 
/// Uses the wrapper type to print the field.
///
/// The wrapper is expected to wrap a reference to the field type,
/// to have an implementation of the [`FormatMarker`] trait,
/// and have a method with this signature:
/// ```ignored
/// const fn const_debug_fmt(
///     self,
///     &mut const_format::Formatter<'_>,
/// ) -> Result<(), const_format::Error>
/// ```
/// (`self` can be taken by reference or by value)
///
/// ### `#[cdeb(is_a(....))]`
/// 
/// Gives the derive macro a hint of what the type is.
///
/// For standard library types,
/// this is necessary if you're using a type alias, since the derive macro detects 
/// those types syntactically.
///
/// These are the valid ways to use this attribute:
///
/// - `#[cdeb(is_a(array))]`/`#[cdeb(is_a(slice))]`:
/// Treats the field as being a slice/array,
/// printing the elements of std or user-defined type with const debug formatting.
///
/// - `#[cdeb(is_a(Option))]`/`#[cdeb(is_a(option))]`:
/// Treats the field as being an Option, 
/// printing the contents of std or user-defined type with const debug formatting.
///
/// - `#[cdeb(is_a(newtype))]`:
/// Treats the field as being being a single field tuple struct, 
/// using the identifier of the field type as the name of the struct,
/// then printing the single field of std or user-defined type with const debug formatting.
///
/// - `#[cdeb(is_a(non_std))]`/`#[cdeb(is_a(not_std))]`:
/// This acts as an opt-out for the automatic detection of std types,
/// most likely needed for types named `Option`.
/// 
/// # Examples
/// 
/// ### Basic
/// 
/// This example demonstrates using the derive without using any helper attributes.
/// 
/// ```rust
/// 
/// use const_format::{ConstDebug, formatc};
/// 
/// use std::cmp::Ordering;
/// 
/// const E_FOO: &str = formatc!("{:?}", Enum::Foo);
/// const E_BAR: &str = formatc!("{:?}", Enum::Bar(10));
/// const E_BAZ: &str = formatc!("{:?}", Enum::Baz{order: Ordering::Less});
/// 
/// const S_UNIT: &str = formatc!("{:?}", Unit);
/// const S_BRACED: &str = formatc!("{:?}", Braced{is_true: false, optional: Some(Unit)});
/// 
/// assert_eq!(E_FOO, "Foo");
/// assert_eq!(E_BAR, "Bar(10)");
/// assert_eq!(E_BAZ, "Baz { order: Less }");
/// 
/// assert_eq!(S_UNIT, "Unit");
/// assert_eq!(S_BRACED, "Braced { is_true: false, optional: Some(Unit) }");
/// 
/// 
/// #[derive(ConstDebug)]
/// enum Enum {
///     Foo,
///     Bar(u32),
///     Baz{
///         order: Ordering,
///     },
/// }
/// 
/// #[derive(ConstDebug)]
/// struct Unit;
/// 
/// #[derive(ConstDebug)]
/// struct Braced {
///     is_true: bool,
///     optional: Option<Unit>,
/// }
/// 
/// ```
/// 
/// ### Generic type
/// 
/// This example demonstrates the `#[cdeb(impls)]` attribute,
/// a workaround for deriving this trait for generic types,
/// specifying a list of impls of types that unconditionally implement const debug formatting
/// 
/// ```rust
/// 
/// use const_format::{ConstDebug, formatc};
/// 
/// use std::marker::PhantomData;
/// 
/// 
/// const S_U32: &str = formatc!("{:?}", Foo(10));
///
/// const S_STR: &str = formatc!("{:?}", Foo("hello"));
/// 
/// const S_PHANTOM: &str = formatc!("{:?}", Foo(PhantomData::<()>));
/// 
/// assert_eq!(S_U32, r#"Foo(10)"#);
/// assert_eq!(S_STR, r#"Foo("hello")"#);
/// assert_eq!(S_PHANTOM, r#"Foo(PhantomData)"#);
/// 
/// 
/// // This type implements const debug formatting three times:
/// // - `Foo<u32>`
/// // - `Foo<&str>`
/// // - `Foo<PhantomData<T>>`: with a generic `T`
/// #[derive(ConstDebug)]
/// #[cdeb(impls(
///     "Foo<u32>",
///     "Foo<&str>",
///     "<T> Foo<PhantomData<T>>",
/// ))]
/// struct Foo<T>(T);
/// 
/// ```
/// 
/// ### `is_a` attributes
/// 
/// This example demonstrates when you would use the `is_a` attributes.
/// 
/// ```rust
/// 
/// use const_format::{ConstDebug, formatc};
/// 
/// use std::{
///     cmp::Ordering,
///     marker::PhantomData,
///     num::Wrapping,
/// };
/// 
/// const STRUCT: &Struct = &Struct {
///     arr: [Ordering::Less, Ordering::Equal, Ordering::Greater, Ordering::Less],
///     opt: Some(Unit),
///     wrap: Wrapping(21),
///     not_option: Option(PhantomData), // This is not the standard library `Option`
/// };
/// 
/// const S_STRUCT: &str = formatc!("{STRUCT:#?}");
/// 
/// const EXPECTED: &str = "\
/// Struct {
///     arr: [
///         Less,
///         Equal,
///         Greater,
///         Less,
///     ],
///     opt: Some(
///         Unit,
///     ),
///     wrap: Wrapping(
///         21,
///     ),
///     not_option: Option(
///         PhantomData,
///     ),
/// }";
/// 
/// fn main(){
///     assert_eq!(S_STRUCT, EXPECTED);
/// }
/// 
/// #[derive(ConstDebug)]
/// struct Struct {
///     // `Ordering` implements const debug formatting,
///     // but `[Ordering; 4]` does not, so this attribute is required for the 
///     // derive macro to generate code to format this array field.
///     #[cdeb(is_a(array))]
///     arr: Array,
///     
///     // Attribute is required to tell the derive macro that this is an
///     // `Option` wrapping a user-defined type,
///     // since `Option<Unit>` doesn't implement const debug formatting.
///     #[cdeb(is_a(option))]
///     opt: Opt,
///     
///     // Attribute is required because `Wrapping<usize>` is a newtype struct
///     // that doesn't implement const debug formatting,
///     // so the derive generates code to format it.
///     #[cdeb(is_a(newtype))]
///     wrap: Wrapping<usize>,
///
///     // Attribute is required for the field to be treated as a user-defined type,
///     // otherwise it'd be assumed to be `Option` from the standard library.
///     #[cdeb(is_a(not_std))]
///     not_option: Option<u32>, 
///     
/// }
/// 
/// type Array = [Ordering; 4];
/// 
/// type Opt = std::option::Option<Unit>;
/// 
/// #[derive(ConstDebug)]
/// struct Unit;
/// 
/// #[derive(ConstDebug)]
/// struct Option<T>(PhantomData<T>);
/// 
/// ```
///
/// [`FormatMarker`]: ./marker_traits/trait.FormatMarker.html
/// [`impls attribute`]: #cdebimpls
///
///
///
/// 
/// ### Renamed import
/// 
/// This example demonstrates that you can use all the macros when the `const_format`
/// crate is renamed.
/// 
/// ```rust
/// # extern crate self as const_format;
/// # extern crate const_format as cfmt;
/// # fn main() {
/// use cfmt::{
///     for_examples::Unit,
///     ConstDebug, formatc,
/// };
/// 
/// #[derive(ConstDebug)]
/// #[cdeb(crate = "cfmt")]
/// struct Foo {
///     bar: &'static str,
///     baz: Unit
/// }
/// 
/// const TEXT: &str = formatc!("{:?}", Foo{ bar: "hello", baz: Unit });
/// 
/// assert_eq!(TEXT, r#"Foo { bar: "hello", baz: Unit }"#);
/// 
/// # }
/// ```
///
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "derive")))]
#[cfg(feature = "derive")]
pub use const_format_proc_macros::ConstDebug;
