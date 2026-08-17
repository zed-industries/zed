#![deny(rust_2018_idioms)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/z-galaxy/zbus/9f7a90d2b594ddc48b7a5f39fda5e00cd56a7dfb/logo.png"
)]
#![doc = include_str!("../README.md")]
#![doc(test(attr(
    warn(unused),
    deny(warnings),
    allow(dead_code),
    // W/o this, we seem to get some bogus warning about `extern crate zbus`.
    allow(unused_extern_crates),
)))]

use proc_macro::TokenStream;
use syn::DeriveInput;

mod dict;
mod signature;
mod r#type;
mod utils;
mod value;

/// Derive macro to add [`Type`] implementation to structs and enums.
///
/// # Examples
///
/// For structs it works just like serde's [`Serialize`] and [`Deserialize`] macros:
///
/// ```
/// use zvariant::{serialized::Context, to_bytes, Type, LE};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
/// struct Struct<'s> {
///     field1: u16,
///     field2: i64,
///     field3: &'s str,
/// }
///
/// assert_eq!(Struct::SIGNATURE, "(qxs)");
/// let s = Struct {
///     field1: 42,
///     field2: i64::max_value(),
///     field3: "hello",
/// };
/// let ctxt = Context::new_dbus(LE, 0);
/// let encoded = to_bytes(ctxt, &s).unwrap();
/// let decoded: Struct = encoded.deserialize().unwrap().0;
/// assert_eq!(decoded, s);
/// ```
///
/// Same with enum, except that all variants of the enum must have the same number and types of
/// fields (if any). If you want the encoding size of the (unit-type) enum to be dictated by
/// `repr` attribute (like in the example below), you'll also need [serde_repr] crate.
///
/// ```
/// use zvariant::{serialized::Context, to_bytes, Type, LE};
/// use serde::{Deserialize, Serialize};
/// use serde_repr::{Deserialize_repr, Serialize_repr};
///
/// #[repr(u8)]
/// #[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
/// enum Enum {
///     Variant1,
///     Variant2,
/// }
/// assert_eq!(Enum::SIGNATURE, u8::SIGNATURE);
/// let ctxt = Context::new_dbus(LE, 0);
/// let encoded = to_bytes(ctxt, &Enum::Variant2).unwrap();
/// let decoded: Enum = encoded.deserialize().unwrap().0;
/// assert_eq!(decoded, Enum::Variant2);
///
/// #[repr(i64)]
/// #[derive(Deserialize_repr, Serialize_repr, Type)]
/// enum Enum2 {
///     Variant1,
///     Variant2,
/// }
/// assert_eq!(Enum2::SIGNATURE, i64::SIGNATURE);
///
/// // w/o repr attribute, u32 representation is chosen
/// #[derive(Deserialize, Serialize, Type)]
/// enum NoReprEnum {
///     Variant1,
///     Variant2,
/// }
/// assert_eq!(NoReprEnum::SIGNATURE, u32::SIGNATURE);
///
/// // Not-unit enums are represented as a structure, with the first field being a u32 denoting the
/// // variant and the second as the actual value.
/// #[derive(Deserialize, Serialize, Type)]
/// enum NewType {
///     Variant1(f64),
///     Variant2(f64),
/// }
/// assert_eq!(NewType::SIGNATURE, "(ud)");
///
/// #[derive(Deserialize, Serialize, Type)]
/// enum StructFields {
///     Variant1(u16, i64, &'static str),
///     Variant2 { field1: u16, field2: i64, field3: &'static str },
/// }
/// assert_eq!(StructFields::SIGNATURE, "(u(qxs))");
/// ```
///
/// # Custom signatures
///
/// There are times when you'd find yourself wanting to specify a hardcoded signature yourself for
/// the type. The `signature` attribute exists for this purpose. A typical use case is when you'd
/// need to encode your type as a dictionary (signature `a{sv}`) type. For convenience, `dict` is
/// an alias for `a{sv}`. Here is an example:
///
/// ```
/// use zvariant::{
///     serialized::Context, as_value, to_bytes, Type, LE,
/// };
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
/// // `#[zvariant(signature = "a{sv}")]` would be the same.
/// #[zvariant(signature = "dict")]
/// struct Struct {
///     #[serde(with = "as_value")]
///     field1: u16,
///     #[serde(with = "as_value")]
///     field2: i64,
///     #[serde(with = "as_value")]
///     field3: String,
/// }
///
/// assert_eq!(Struct::SIGNATURE, "a{sv}");
/// let s = Struct {
///     field1: 42,
///     field2: i64::max_value(),
///     field3: "hello".to_string(),
/// };
/// let ctxt = Context::new_dbus(LE, 0);
/// let encoded = to_bytes(ctxt, &s).unwrap();
/// let decoded: Struct = encoded.deserialize().unwrap().0;
/// assert_eq!(decoded, s);
/// ```
///
/// Another common use for custom signatures is (de)serialization of unit enums as strings:
///
/// ```
/// use zvariant::{serialized::Context, to_bytes, Type, LE};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
/// #[zvariant(signature = "s")]
/// enum StrEnum {
///     Variant1,
///     Variant2,
///     Variant3,
/// }
///
/// assert_eq!(StrEnum::SIGNATURE, "s");
/// let ctxt = Context::new_dbus(LE, 0);
/// let encoded = to_bytes(ctxt, &StrEnum::Variant2).unwrap();
/// assert_eq!(encoded.len(), 13);
/// let decoded: StrEnum = encoded.deserialize().unwrap().0;
/// assert_eq!(decoded, StrEnum::Variant2);
/// ```
///
/// # Custom crate path
///
/// If you've renamed `zvariant` in your `Cargo.toml` or are using it through a re-export,
/// you can specify the crate path using the `crate` attribute:
///
/// ```
/// use zvariant::Type;
///
/// #[derive(Type)]
/// #[zvariant(crate = "zvariant")]
/// struct MyStruct {
///     field: String,
/// }
/// ```
///
/// [`Type`]: https://docs.rs/zvariant/latest/zvariant/trait.Type.html
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
/// [serde_repr]: https://crates.io/crates/serde_repr
#[proc_macro_derive(Type, attributes(zbus, zvariant))]
pub fn type_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    r#type::expand_derive(ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Adds [`Serialize`] implementation to structs to be serialized as a D-Bus dictionary type.
///
/// The dictionary type is determined by the `signature` attribute. The default is `a{sv}`
/// (string keys, variant values), but nested forms like `a{sa{sv}}` and `a{oa{sv}}` are also
/// supported — fields whose value type is itself a dict (or any non-`Variant` type) are
/// serialized directly through their own `Serialize` impl rather than wrapped as a variant.
///
/// Such dictionary types are very commonly used with
/// [D-Bus](https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces-properties)
/// and GVariant.
///
/// # Alternative Approaches
///
/// There are two approaches to serializing structs as dictionaries:
///
/// 1. Using this macro (simpler, but less control).
/// 2. Using the `Serialize` derive with `zvariant::as_value` (more verbose, but more control).
///
/// See the example below and the relevant [FAQ entry] in our book for more details on the
/// alternative approach.
///
/// # Example
///
/// ## Approach #1
///
/// ```
/// use zvariant::{SerializeDict, Type};
///
/// #[derive(Debug, Default, SerializeDict, Type)]
/// #[zvariant(signature = "a{sv}", rename_all = "PascalCase")]
/// pub struct MyStruct {
///     field1: Option<u32>,
///     field2: String,
/// }
/// ```
///
/// ## Approach #2
///
/// ```
/// use serde::Serialize;
/// use zvariant::{Type, as_value};
///
/// #[derive(Debug, Default, Serialize, Type)]
/// #[zvariant(signature = "a{sv}")]
/// #[serde(default, rename_all = "PascalCase")]
/// pub struct MyStruct {
///     #[serde(with = "as_value::optional", skip_serializing_if = "Option::is_none")]
///     field1: Option<u32>,
///     #[serde(with = "as_value")]
///     field2: String,
/// }
/// ```
///
/// ## Nested dictionaries
///
/// To represent shapes like `a{sa{sv}}` (the body type of
/// `org.freedesktop.DBus.ObjectManager.GetManagedObjects` and similar APIs), nest one
/// `SerializeDict`/`DeserializeDict` struct inside another:
///
/// ```
/// use zvariant::{DeserializeDict, SerializeDict, Type};
///
/// #[derive(SerializeDict, DeserializeDict, Type, Default)]
/// #[zvariant(signature = "a{sv}", rename_all = "PascalCase")]
/// pub struct AdapterProperties {
///     address: Option<String>,
///     name: Option<String>,
/// }
///
/// #[derive(SerializeDict, DeserializeDict, Type, Default)]
/// #[zvariant(signature = "a{sa{sv}}")]
/// pub struct InterfaceProperties {
///     #[zvariant(rename = "org.bluez.Adapter1")]
///     adapter: Option<AdapterProperties>,
/// }
/// ```
///
/// # Custom crate path
///
/// If you've renamed `zvariant` in your `Cargo.toml` or are using it through a re-export,
/// you can specify the crate path using the `crate` attribute:
///
/// ```
/// use zvariant::{SerializeDict, Type};
///
/// #[derive(SerializeDict, Type)]
/// #[zvariant(signature = "a{sv}", crate = "zvariant")]
/// struct MyStruct {
///     field: String,
/// }
/// ```
///
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
/// [FAQ entry]: https://z-galaxy.github.io/zbus/faq.html#how-to-use-a-struct-as-a-dictionary
#[proc_macro_derive(SerializeDict, attributes(zbus, zvariant))]
pub fn serialize_dict_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    dict::expand_serialize_derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Adds [`Deserialize`] implementation to structs to be deserialized from a D-Bus dictionary type.
///
/// The dictionary type is determined by the `signature` attribute. The default is `a{sv}`
/// (string keys, variant values), but nested forms like `a{sa{sv}}` and `a{oa{sv}}` are also
/// supported — fields whose value type is itself a dict (or any non-`Variant` type) are
/// deserialized directly through their own `Deserialize` impl rather than unwrapped from a
/// variant. See [`SerializeDict`] for a nested example.
///
/// Such dictionary types are very commonly used with
/// [D-Bus](https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces-properties)
/// and GVariant.
///
/// # Alternative Approaches
///
/// There are two approaches to deserializing dictionaries as structs:
///
/// 1. Using this macro (simpler, but less control).
/// 2. Using the `Deserialize` derive with `zvariant::as_value` (more verbose, but more control).
///
/// See the example below and the relevant [FAQ entry] in our book for more details on the
/// alternative approach.
///
/// # Example
///
/// ## Approach #1
///
/// ```
/// use zvariant::{DeserializeDict, Type};
///
/// #[derive(Debug, Default, DeserializeDict, Type)]
/// #[zvariant(signature = "a{sv}", rename_all = "PascalCase")]
/// pub struct MyStruct {
///     field1: Option<u32>,
///     field2: String,
/// }
/// ```
///
/// ## Approach #2
///
/// ```
/// use serde::Deserialize;
/// use zvariant::{Type, as_value};
///
/// #[derive(Debug, Default, Deserialize, Type)]
/// #[zvariant(signature = "a{sv}")]
/// #[serde(default, rename_all = "PascalCase")]
/// pub struct MyStruct {
///     #[serde(with = "as_value::optional")]
///     field1: Option<u32>,
///     #[serde(with = "as_value")]
///     field2: String,
/// }
/// ```
///
/// # Custom crate path
///
/// If you've renamed `zvariant` in your `Cargo.toml` or are using it through a re-export,
/// you can specify the crate path using the `crate` attribute:
///
/// ```
/// use zvariant::{DeserializeDict, Type};
///
/// #[derive(DeserializeDict, Type)]
/// #[zvariant(signature = "a{sv}", crate = "zvariant")]
/// struct MyStruct {
///     field: String,
/// }
/// ```
///
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
/// [FAQ entry]: https://z-galaxy.github.io/zbus/faq.html#how-to-use-a-struct-as-a-dictionary
#[proc_macro_derive(DeserializeDict, attributes(zbus, zvariant))]
pub fn deserialize_dict_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    dict::expand_deserialize_derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Implements conversions for your type to/from [`Value`].
///
/// Implements `TryFrom<Value>` and `Into<Value>` for your type.
///
/// # Examples
///
/// Simple owned strutures:
///
/// ```
/// use zvariant::{OwnedObjectPath, OwnedValue, Value};
///
/// #[derive(Clone, Value, OwnedValue)]
/// struct OwnedStruct {
///     owned_str: String,
///     owned_path: OwnedObjectPath,
/// }
///
/// let s = OwnedStruct {
///     owned_str: String::from("hi"),
///     owned_path: OwnedObjectPath::try_from("/blah").unwrap(),
/// };
/// let value = Value::from(s.clone());
/// let _ = OwnedStruct::try_from(value).unwrap();
/// let value = OwnedValue::try_from(s).unwrap();
/// let s = OwnedStruct::try_from(value).unwrap();
/// assert_eq!(s.owned_str, "hi");
/// assert_eq!(s.owned_path.as_str(), "/blah");
/// ```
///
/// Now for the more exciting case of unowned structures:
///
/// ```
/// use zvariant::{ObjectPath, Str};
/// # use zvariant::{OwnedValue, Value};
/// #
/// #[derive(Clone, Value, OwnedValue)]
/// struct UnownedStruct<'a> {
///     s: Str<'a>,
///     path: ObjectPath<'a>,
/// }
///
/// let hi = String::from("hi");
/// let s = UnownedStruct {
///     s: Str::from(&hi),
///     path: ObjectPath::try_from("/blah").unwrap(),
/// };
/// let value = Value::from(s.clone());
/// let s = UnownedStruct::try_from(value).unwrap();
///
/// let value = OwnedValue::try_from(s).unwrap();
/// let s = UnownedStruct::try_from(value).unwrap();
/// assert_eq!(s.s, "hi");
/// assert_eq!(s.path, "/blah");
/// ```
///
/// Generic structures also supported:
///
/// ```
/// # use zvariant::{OwnedObjectPath, OwnedValue, Value};
/// #
/// #[derive(Clone, Value, OwnedValue)]
/// struct GenericStruct<S, O> {
///     field1: S,
///     field2: O,
/// }
///
/// let s = GenericStruct {
///     field1: String::from("hi"),
///     field2: OwnedObjectPath::try_from("/blah").unwrap(),
/// };
/// let value = Value::from(s.clone());
/// let _ = GenericStruct::<String, OwnedObjectPath>::try_from(value).unwrap();
/// let value = OwnedValue::try_from(s).unwrap();
/// let s = GenericStruct::<String, OwnedObjectPath>::try_from(value).unwrap();
/// assert_eq!(s.field1, "hi");
/// assert_eq!(s.field2.as_str(), "/blah");
/// ```
///
/// Enums also supported but currently only with unit variants:
///
/// ```
/// # use zvariant::{OwnedValue, Value};
/// #
/// #[derive(Debug, PartialEq, Value, OwnedValue)]
/// // Default representation is `u32`.
/// #[repr(u8)]
/// enum Enum {
///     Variant1 = 0,
///     Variant2,
/// }
///
/// let value = Value::from(Enum::Variant1);
/// let e = Enum::try_from(value).unwrap();
/// assert_eq!(e, Enum::Variant1);
/// assert_eq!(e as u8, 0);
/// let value = OwnedValue::try_from(Enum::Variant2).unwrap();
/// let e = Enum::try_from(value).unwrap();
/// assert_eq!(e, Enum::Variant2);
/// ```
///
/// String-encoded enums are also supported:
///
/// ```
/// # use zvariant::{OwnedValue, Value};
/// #
/// #[derive(Debug, PartialEq, Value, OwnedValue)]
/// #[zvariant(signature = "s")]
/// enum StrEnum {
///     Variant1,
///     Variant2,
/// }
///
/// let value = Value::from(StrEnum::Variant1);
/// let e = StrEnum::try_from(value).unwrap();
/// assert_eq!(e, StrEnum::Variant1);
/// let value = OwnedValue::try_from(StrEnum::Variant2).unwrap();
/// let e = StrEnum::try_from(value).unwrap();
/// assert_eq!(e, StrEnum::Variant2);
/// ```
///
/// # Renaming fields
///
/// ## Auto Renaming
///
/// The macro supports specifying a Serde-like `#[zvariant(rename_all = "case")]` attribute on
/// structures. The attribute allows to rename all the fields from snake case to another case
/// automatically.
///
/// Currently the macro supports the following values for `case`:
///
/// * `"lowercase"`
/// * `"UPPERCASE"`
/// * `"PascalCase"`
/// * `"camelCase"`
/// * `"snake_case"`
/// * `"kebab-case"`
///
/// ## Individual Fields
///
/// It's still possible to specify custom names for individual fields using the
/// `#[zvariant(rename = "another-name")]` attribute even when the `rename_all` attribute is
/// present.
///
/// Here is an example using both `rename` and `rename_all`:
///
/// ```
/// # use zvariant::{OwnedValue, Value, Dict};
/// # use std::collections::HashMap;
/// #
/// #[derive(Clone, Value, OwnedValue)]
/// #[zvariant(signature = "dict", rename_all = "PascalCase")]
/// struct RenamedStruct {
///     #[zvariant(rename = "MyValue")]
///     field1: String,
///     field2: String,
/// }
///
/// let s = RenamedStruct {
///     field1: String::from("hello"),
///     field2: String::from("world")
/// };
/// let v = Value::from(s);
/// let d = Dict::try_from(v).unwrap();
/// let hm: HashMap<String, String> = HashMap::try_from(d).unwrap();
/// assert_eq!(hm.get("MyValue").unwrap().as_str(), "hello");
/// assert_eq!(hm.get("Field2").unwrap().as_str(), "world");
/// ```
///
/// # Dictionary encoding
///
/// For treating your type as a dictionary, you can use the `signature = "dict"` attribute. See
/// [`Type`] for more details and an example use. Please note that this macro can only handle
/// `dict` or `a{sv}` values. All other values will be ignored.
///
/// # Custom crate path
///
/// If you've renamed `zvariant` in your `Cargo.toml` or are using it through a re-export,
/// you can specify the crate path using the `crate` attribute:
///
/// ```
/// use zvariant::Value;
///
/// #[derive(Clone, Value)]
/// #[zvariant(crate = "zvariant")]
/// struct MyStruct {
///     field: String,
/// }
/// ```
///
/// [`Value`]: https://docs.rs/zvariant/latest/zvariant/enum.Value.html
/// [`Type`]: crate::Type#custom-signatures
#[proc_macro_derive(Value, attributes(zbus, zvariant))]
pub fn value_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    value::expand_derive(ast, value::ValueType::Value)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Implements conversions for your type to/from [`OwnedValue`].
///
/// Implements `TryFrom<OwnedValue>` and `TryInto<OwnedValue>` for your type.
///
/// See [`Value`] documentation for examples.
///
/// [`OwnedValue`]: https://docs.rs/zvariant/latest/zvariant/struct.OwnedValue.html
#[proc_macro_derive(OwnedValue, attributes(zbus, zvariant))]
pub fn owned_value_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    value::expand_derive(ast, value::ValueType::OwnedValue)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Constructs a const [`Signature`] with compile-time validation.
///
/// This macro creates a `Signature` from a string literal at compile time, validating
/// that the signature string is valid D-Bus signature. Invalid signatures will cause
/// a compilation error.
///
/// # Examples
///
/// ## Basic usage
///
/// ```
/// use zvariant::signature;
///
/// // Create signatures for basic types
/// let sig = signature!("s");  // String signature
/// assert_eq!(sig.to_string(), "s");
///
/// let sig = signature!("i");  // 32-bit integer signature
/// assert_eq!(sig.to_string(), "i");
/// ```
///
/// ## Container types
///
/// ```
/// use zvariant::signature;
///
/// // Array of strings
/// let sig = signature!("as");
/// assert_eq!(sig.to_string(), "as");
///
/// // Dictionary mapping strings to variants
/// let sig = signature!("a{sv}");
/// assert_eq!(sig.to_string(), "a{sv}");
///
/// // Structures
/// let sig = signature!("(isx)");
/// assert_eq!(sig.to_string(), "(isx)");
/// ```
///
/// ## Const signatures
///
/// The macro can be used to create const signatures, which is especially useful
/// for defining signatures at compile time:
///
/// ```
/// use zvariant::{signature, Signature};
///
/// const MY_SIGNATURE: Signature = signature!("a{sv}");
///
/// fn process_data(_data: &str) {
///     assert_eq!(MY_SIGNATURE.to_string(), "a{sv}");
/// }
/// ```
///
/// ## Using the `dict` alias
///
/// For convenience, `dict` is an alias for `a{sv}` (string-to-variant dictionary):
///
/// ```
/// use zvariant::signature;
///
/// let sig = signature!("dict");
/// assert_eq!(sig.to_string(), "a{sv}");
/// ```
///
/// ## Compile-time validation
///
/// Invalid signatures will be caught at compile time:
///
/// ```compile_fail
/// use zvariant::signature;
///
/// // This will fail to compile because 'z' is not a valid D-Bus type
/// let sig = signature!("z");
/// ```
///
/// [`Signature`]: https://docs.rs/zvariant/latest/zvariant/enum.Signature.html
#[proc_macro]
pub fn signature(input: TokenStream) -> TokenStream {
    signature::expand_signature_macro(input.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
