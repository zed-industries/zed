//! JSON5 is a superset of [JSON][] with an expanded syntax including some productions from
//! [ECMAScript 5.1][].
//!
//! In particular, JSON5 allows comments, trailing commas, object keys without quotes, single
//! quoted strings and more. See the [JSON5 project page][] for full details.
//!
//! ```json5,ignore
//! {
//!   // comments
//!   unquoted: 'and you can quote me on that',
//!   singleQuotes: 'I can use "double quotes" here',
//!   lineBreaks: "Look, Mom! \
//! No \\n's!",
//!   hexadecimal: 0xdecaf,
//!   leadingDecimalPoint: .8675309, andTrailing: 8675309.,
//!   positiveSign: +1,
//!   trailingComma: 'in objects', andIn: ['arrays',],
//!   "backwardsCompatible": "with JSON",
//! }
//! ```
//!
//! This crate provides functions for deserializing JSON5 text into a Rust datatype and for
//! serializing a Rust datatype as JSON5 text, both via the [Serde framework][].
//!
//! # Deserialization
//!
//! Implementing Serde&rsquo;s [`Deserialize`][] trait on your type will allow you to parse JSON5
//! text into a value of that type with [`from_str`][].
//!
//! ```rust
//! use serde_derive::Deserialize;
//!
//! #[derive(Deserialize, Debug, PartialEq)]
//! struct Config {
//!     message: String,
//!     n: i32,
//! }
//!
//! let config = "
//!     {
//!       // A traditional message.
//!       message: 'hello world',
//!
//!       // A number for some reason.
//!       n: 42,
//!     }
//! ";
//!
//! assert_eq!(
//!     json5::from_str(config),
//!     Ok(Config {
//!         message: "hello world".to_string(),
//!         n: 42,
//!     }),
//! );
//! ```
//!
//! Also, you could deserialize into serde_json::Value
//!
//! ```rust
//! use json5;
//! use serde_json::{Value, json};
//!
//! let config = "
//!     {
//!       // A traditional message.
//!       message: 'hello world',
//!
//!       // A number for some reason.
//!       n: 42,
//!     }
//! ";
//!
//! assert_eq!(
//!     json5::from_str::<Value>(&config),
//!     Ok(json!({
//!         "message": "hello world",
//!         "n": 42
//!     }))
//! );
//! ```
//!
//! There are many ways to customize the deserialization (e.g. deserializing `camelCase` field
//! names into a struct with `snake_case` fields). See the Serde docs, especially the
//! [Attributes][], [Custom serialization][] and [Examples][] sections.
//!
//! # Serialization
//!
//! Similarly, implementing [`Serialize`][] on a Rust type allows you to produce a JSON5
//! serialization of values of that type with [`to_string`][]. At present the serializer will just
//! produce JSON (since it's a valid subset of JSON5), but future work will allow specifying the
//! output style (single over double quotes, trailing commas, indentation etc.).
//!
//! ```rust
//! use serde_derive::Serialize;
//! use std::collections::HashMap;
//!
//! #[derive(Serialize, Debug)]
//! #[serde(untagged)]
//! enum Val {
//!     Null,
//!     Bool(bool),
//!     Number(f64),
//!     String(String),
//!     Array(Vec<Val>),
//!     Object(HashMap<String, Val>),
//! }
//! let mut map = HashMap::new();
//! map.insert(
//!     "a".to_owned(),
//!     Val::Array(vec![
//!         Val::Null,
//!         Val::Bool(true),
//!         Val::Number(42.),
//!         Val::Number(42.42),
//!         Val::Number(f64::NAN),
//!         Val::String("hello".to_owned()),
//!     ])
//! );
//! assert_eq!(
//!     json5::to_string(&Val::Object(map)),
//!     Ok("{\"a\":[null,true,42,42.42,NaN,\"hello\"]}".to_owned()),
//! )
//! ```
//!
//! You could also build from serde_json
//!
//! ```rust
//! use serde_json::{json, Value, Map, Number};
//! assert_eq!(
//!     json5::to_string(
//!         &json!({"a": [null, true, 42, 42.42, f64::NAN, "hello"]})
//!     ),
//!     Ok("{\"a\":[null,true,42,42.42,null,\"hello\"]}".to_owned())
//! );
//! let mut map = Map::new();
//! map.insert(
//!     "a".to_owned(),
//!     Value::Array(vec![
//!         Value::Null,
//!         Value::Bool(true),
//!         Value::Number(Number::from_f64(42.).unwrap()),
//!         Value::Number(Number::from_f64(42.42).unwrap()),
//!         Value::String("hello".to_owned()),
//!     ])
//! );
//! assert_eq!(
//!     json5::to_string(&Value::Object(map)),
//!     Ok("{\"a\":[null,true,42,42.42,\"hello\"]}".to_owned()),
//! )
//! ```
//!
//! There are many ways to customize the serialization (e.g. serializing `snake_case` struct fields
//! as `camelCase`). See the Serde docs, especially the [Attributes][], [Custom serialization][]
//! and [Examples][] sections.
//!
//! # Limitations
//!
//! At the time of writing the following is unsupported:
//!
//! - deserializing into borrowed types (e.g. fields of type `&str`)
//!
//! - serializing or deserializing [byte arrays][]
//!
//! - specifying the style of JSON5 output from the serializer (single over double quotes, trailing
//! commas, indentation etc.)
//!
//! [JSON]: https://tools.ietf.org/html/rfc7159
//! [ECMAScript 5.1]: https://www.ecma-international.org/ecma-262/5.1/
//! [JSON5 project page]: https://json5.org/
//! [Serde framework]: https://serde.rs/
//! [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
//! [`from_str`]: fn.from_str.html
//! [Attributes]: https://serde.rs/attributes.html
//! [Custom serialization]: https://serde.rs/custom-serialization.html
//! [Examples]: https://serde.rs/examples.html
//! [`Serialize`]: https://docs.serde.rs/serde/ser/trait.Serialize.html
//! [`to_string`]: fn.to_string.html
//! [byte arrays]: https://serde.rs/data-model.html#types

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

mod de;
mod error;
mod ser;

pub use crate::de::{from_str, Deserializer};
pub use crate::error::{Error, Location, Result};
pub use crate::ser::to_string;
