/*!
# `serde` -> `std::fmt`

This library lets you take any `Serialize` and format it as if it's `Debug`.
The format produced is the same as if the type derived `Debug`, and any
formatting flags will be preserved.

# Getting started

Add `serde_fmt` to your `Cargo.toml`:

```toml,ignore
[dependencies.serde_fmt]
version = "1.0.3"
```

By default, this library doesn't depend on the standard library.
You can enable support with the `std` Cargo feature:

```toml,ignore
[dependencies.serde_fmt]
version = "1.0.3"
features = ["std"]
```

# Formatting a `Serialize`

Use the [`to_debug`] function to treat a [`serde::Serialize`] like a [`std::fmt::Debug`]:

```rust
# use serde::Serialize;
fn takes_serialize(v: impl Serialize) {
    // You can dump any `Serialize` using the
    // standard `dbg!` macro
    dbg!(serde_fmt::to_debug(&v));

    // do something with `v`
}
```
*/

#![doc(html_root_url = "https://docs.rs/serde_fmt/1.0.3")]
#![cfg_attr(not(test), no_std)]

#[cfg(all(not(test), not(feature = "std")))]
extern crate core as std;

#[cfg(any(test, feature = "std"))]
extern crate std;

use crate::std::fmt::{self, Debug, Display};

use serde::ser::{
    self, Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

/**
Format a [`serde::Serialize`] into a [`std::fmt::Write`].
*/
pub fn to_writer(v: impl Serialize, mut w: impl fmt::Write) -> fmt::Result {
    w.write_fmt(format_args!("{:?}", to_debug(v)))
}

/**
Treat a type implementing [`serde::Serialize`] like a type implementing [`std::fmt::Debug`].
*/
pub fn to_debug<T>(v: T) -> ToDebug<T>
where
    T: Serialize,
{
    ToDebug(v)
}

/**
The result of calling [`to_debug`] .
*/
#[derive(Clone, Copy)]
pub struct ToDebug<T>(T);

impl<T> Debug for ToDebug<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

// Even though it's not specified, since we treat `fmt::Debug`
// as the canonical format for `ToDebug` we can also think of
// it as the human-readable `Display`able format in the same
// way that `fmt::Arguments` does.
impl<T> Display for ToDebug<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // If the `Serialize` impl fails then swallow the error rather than
        // propagate it; Traits like `ToString` expect formatting to be
        // infallible unless the writer itself fails
        match self.0.serialize(Formatter::new(f)) {
            Ok(()) => Ok(()),
            Err(e) => write!(f, "<{}>", e),
        }
    }
}

// Surface the original `Serialize` implementation.
impl<T> Serialize for ToDebug<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

struct Formatter<'a, 'b: 'a>(&'a mut fmt::Formatter<'b>);

impl<'a, 'b: 'a> Formatter<'a, 'b> {
    fn new(fmt: &'a mut fmt::Formatter<'b>) -> Self {
        Formatter(fmt)
    }

    fn fmt(self, v: impl Debug) -> Result<(), Error> {
        v.fmt(self.0).map_err(Into::into)
    }
}

impl<'a, 'b: 'a> Serializer for Formatter<'a, 'b> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = DebugSeq<'a, 'b>;
    type SerializeTuple = DebugTuple<'a, 'b>;
    type SerializeTupleStruct = DebugTupleStruct<'a, 'b>;
    type SerializeTupleVariant = DebugTupleVariant<'a, 'b>;
    type SerializeMap = DebugMap<'a, 'b>;
    type SerializeStruct = DebugStruct<'a, 'b>;
    type SerializeStructVariant = DebugStructVariant<'a, 'b>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn collect_str<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        self.fmt(format_args!("{}", v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.fmt(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "None")?;
        Ok(())
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_newtype_struct("Some", v)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "()")?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_tuple_struct(name, 0)?.end()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_tuple_struct(variant, 0)?.end()
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut tuple = self.serialize_tuple_struct(name, 1)?;
        tuple.serialize_field(v)?;
        tuple.end()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        v: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut tuple = self.serialize_tuple_struct(variant, 1)?;
        tuple.serialize_field(v)?;
        tuple.end()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(DebugSeq(self.0.debug_list()))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(DebugTuple(self.0.debug_tuple("")))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(DebugTupleStruct(self.0.debug_tuple(name)))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(DebugTupleVariant(self.0.debug_tuple(variant)))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(DebugMap(self.0.debug_map()))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(DebugStruct(self.0.debug_struct(name)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(DebugStructVariant(self.0.debug_struct(variant)))
    }
}

struct DebugSeq<'a, 'b: 'a>(fmt::DebugList<'a, 'b>);

impl<'a, 'b: 'a> SerializeSeq for DebugSeq<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.entry(&to_debug(v));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugTuple<'a, 'b: 'a>(fmt::DebugTuple<'a, 'b>);

impl<'a, 'b: 'a> SerializeTuple for DebugTuple<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&to_debug(v));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugTupleStruct<'a, 'b: 'a>(fmt::DebugTuple<'a, 'b>);

impl<'a, 'b: 'a> SerializeTupleStruct for DebugTupleStruct<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&to_debug(v));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugTupleVariant<'a, 'b: 'a>(fmt::DebugTuple<'a, 'b>);

impl<'a, 'b: 'a> SerializeTupleVariant for DebugTupleVariant<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&to_debug(v));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugStruct<'a, 'b: 'a>(fmt::DebugStruct<'a, 'b>);

impl<'a, 'b: 'a> SerializeStruct for DebugStruct<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, k: &'static str, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(k, &to_debug(v));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugStructVariant<'a, 'b: 'a>(fmt::DebugStruct<'a, 'b>);

impl<'a, 'b: 'a> SerializeStructVariant for DebugStructVariant<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, k: &'static str, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(k, &to_debug(v));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

struct DebugMap<'a, 'b: 'a>(fmt::DebugMap<'a, 'b>);

impl<'a, 'b: 'a> SerializeMap for DebugMap<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_entry<K, V>(&mut self, k: &K, v: &V) -> Result<Self::Ok, Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        self.0.entry(&to_debug(k), &to_debug(v));
        Ok(())
    }

    fn serialize_key<T>(&mut self, k: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.key(&to_debug(k));
        Ok(())
    }

    fn serialize_value<T>(&mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.value(&to_debug(v));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish().map_err(Into::into)
    }
}

#[derive(Debug)]
struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to serialize to a standard formatter")
    }
}

impl From<Error> for fmt::Error {
    fn from(_: Error) -> fmt::Error {
        fmt::Error
    }
}

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Error {
        Error
    }
}

impl ser::StdError for Error {}

impl ser::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: Display,
    {
        Error
    }
}

#[cfg(test)]
extern crate serde_derive;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::ser::Error as _;
    use serde_derive::*;

    fn check_fmt(v: (impl fmt::Debug + Serialize)) {
        assert_eq!(format!("{:?}", v), format!("{:?}", to_debug(v)));
    }

    #[test]
    fn failing_serialize_does_not_panic_to_string() {
        struct Kaboom;

        impl Serialize for Kaboom {
            fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(S::Error::custom("kaboom!"))
            }
        }

        #[derive(Serialize)]
        struct NestedKaboom {
            a: i32,
            b: Kaboom,
            c: i32,
        }

        assert_eq!("<failed to serialize to a standard formatter>", to_debug(Kaboom).to_string());
        assert_eq!("NestedKaboom { a: 1, b: <failed to serialize to a standard formatter>, c: 2 }", to_debug(NestedKaboom { a: 1, b: Kaboom, c: 2 }).to_string());
    }

    #[test]
    fn struct_fmt_is_consitent() {
        #[derive(Serialize, Debug)]
        struct Struct {
            a: Signed,
            b: Unsigned,
            c: char,
            d: &'static str,
            e: &'static [u8],
            f: (),
        }

        #[derive(Serialize, Debug)]
        struct Signed {
            a: i8,
            b: i16,
            c: i32,
            d: i64,
        }

        #[derive(Serialize, Debug)]
        struct Unsigned {
            a: u8,
            b: u16,
            c: u32,
            d: u64,
        }

        check_fmt(Struct {
            a: Signed {
                a: -1,
                b: 42,
                c: -42,
                d: 42,
            },
            b: Unsigned {
                a: 1,
                b: 42,
                c: 1,
                d: 42,
            },
            c: 'a',
            d: "a string",
            e: &[1, 2, 3],
            f: (),
        });
    }

    #[test]
    fn fmt_flags_are_consistent() {
        use crate::std::format;

        #[derive(Serialize, Debug)]
        struct Struct {
            a: i32,
            b: i32,
        }

        assert_eq!(format!("{:03?}", 42), format!("{:03?}", to_debug(42)));
        assert_eq!(format!("{:x?}", 42), format!("{:x?}", to_debug(42)));
        assert_eq!(format!("{:X?}", 42), format!("{:X?}", to_debug(42)));
        assert_eq!(
            format!("{:#?}", Struct { a: 42, b: 17 }),
            format!("{:#?}", to_debug(Struct { a: 42, b: 17 }))
        );
    }

    #[test]
    fn option_fmt_is_consistent() {
        check_fmt(Option::Some::<i32>(42));
        check_fmt(Option::None::<i32>);
    }

    #[test]
    fn result_fmt_is_consistent() {
        check_fmt(Result::Ok::<i32, i32>(42));
        check_fmt(Result::Err::<i32, i32>(42));
    }

    #[test]
    fn tuple_fmt_is_consistent() {
        check_fmt((42, 17));
    }

    #[test]
    fn tagged_fmt_is_consistent() {
        #[derive(Serialize, Debug)]
        enum Tagged {
            Unit,
            NewType(i32),
            Tuple(i32, i32),
            Struct { a: i32, b: i32 },
        }

        check_fmt(Tagged::Unit);
        check_fmt(Tagged::NewType(42));
        check_fmt(Tagged::Tuple(42, 17));
        check_fmt(Tagged::Struct { a: 42, b: 17 });
    }
}
