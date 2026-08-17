use crate::{Enumerable, Listable, Mappable, Structable, Tuplable, Valuable, Visit};

use core::fmt;

macro_rules! value {
    (
        $(
            $(#[$attrs:meta])*
            $variant:ident($ty:ty),
        )*
    ) => {
        /// Any Rust value
        ///
        /// The `Value` enum is used to pass single values to the
        /// [visitor][`Visit`]. Primitive types are enumerated and other types
        /// are represented at trait objects.
        ///
        /// Values are converted to `Value` instances using
        /// [`Valuable::as_value()`].
        ///
        /// # Examples
        ///
        /// Convert a primitive type
        ///
        /// ```
        /// use valuable::{Value, Valuable};
        ///
        /// let num = 123;
        /// let val = num.as_value();
        ///
        /// assert!(matches!(val, Value::I32(v) if v == 123));
        /// ```
        ///
        /// Converting a struct
        ///
        /// ```
        /// use valuable::{Value, Valuable};
        ///
        /// #[derive(Valuable, Debug)]
        /// struct HelloWorld {
        ///     message: String,
        /// }
        ///
        /// let hello = HelloWorld {
        ///     message: "greetings".to_string(),
        /// };
        ///
        /// let val = hello.as_value();
        ///
        /// assert!(matches!(val, Value::Structable(_v)));
        ///
        /// // The Value `Debug` output matches the struct's
        /// assert_eq!(
        ///     format!("{:?}", val),
        ///     format!("{:?}", hello),
        /// );
        /// ```
        ///
        /// [visitor]: Visit
        #[non_exhaustive]
        #[derive(Clone, Copy)]
        pub enum Value<'a> {
            $(
                $(#[$attrs])*
                $variant($ty),
            )*

            /// A Rust `()` or `None` value.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            ///
            /// let v = Value::Unit;
            /// ```
            Unit,
        }

        $(
            $(#[$attrs])*
            impl<'a> From<$ty> for Value<'a> {
                fn from(src: $ty) -> Value<'a> {
                    Value::$variant(src)
                }
            }
        )*

        impl<'a> From<()> for Value<'a> {
            fn from(_: ()) -> Value<'a> {
                Value::Tuplable(&())
            }
        }

        impl fmt::Debug for Value<'_> {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                use Value::*;

                // Doc comments are expanded into the branch arms, which results
                // in a warning. It isn't a big deal, so silence it.
                #[allow(unused_doc_comments)]
                match self {
                    $(
                        $(#[$attrs])*
                        $variant(v) => fmt::Debug::fmt(v, fmt),
                    )*
                    Unit => ().fmt(fmt),
                }
            }
        }
    }
}

value! {
    /// A Rust `bool` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::Bool(true);
    /// ```
    Bool(bool),

    /// A Rust `char` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::Char('h');
    /// ```
    Char(char),

    /// A Rust `f32` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::F32(3.1415);
    /// ```
    F32(f32),

    /// A Rust `f64` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::F64(3.1415);
    /// ```
    F64(f64),

    /// A Rust `i8` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I8(42);
    /// ```
    I8(i8),

    /// A Rust `i16` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I16(42);
    /// ```
    I16(i16),

    /// A Rust `i32` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I32(42);
    /// ```
    I32(i32),

    /// A Rust `i64` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I64(42);
    /// ```
    I64(i64),

    /// A Rust `i128` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I128(42);
    /// ```
    I128(i128),

    /// A Rust `isize` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::Isize(42);
    /// ```
    Isize(isize),

    /// A Rust `&str` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::String("hello");
    /// ```
    String(&'a str),

    /// A Rust `u8` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U8(42);
    /// ```
    U8(u8),

    /// A Rust `u16` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U16(42);
    /// ```
    U16(u16),

    /// A Rust `u32` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U32(42);
    /// ```
    U32(u32),

    /// A Rust `u64` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U64(42);
    /// ```
    U64(u64),

    /// A Rust `u128` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U128(42);
    /// ```
    U128(u128),

    /// A Rust `usize` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::Usize(42);
    /// ```
    Usize(usize),

    /// A Rust `&Path` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    /// use std::path::Path;
    ///
    /// let path = Path::new("a.txt");
    /// let v = Value::Path(path);
    /// ```
    #[cfg(feature = "std")]
    Path(&'a std::path::Path),

    /// A Rust error value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    /// use std::io;
    ///
    /// let err: io::Error = io::ErrorKind::Other.into();
    /// let v = Value::Error(&err);
    /// ```
    #[cfg(feature = "std")]
    Error(&'a (dyn std::error::Error +'static)),

    /// A Rust list value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let vals = vec![1, 2, 3, 4, 5];
    /// let v = Value::Listable(&vals);
    /// ```
    Listable(&'a dyn Listable),

    /// A Rust map value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("foo", 1);
    /// map.insert("bar", 2);
    ///
    /// let v = Value::Mappable(&map);
    /// ```
    Mappable(&'a dyn Mappable),

    /// A Rust struct value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Value, Valuable};
    ///
    /// #[derive(Valuable)]
    /// struct MyStruct {
    ///     field: u32,
    /// }
    ///
    /// let my_struct = MyStruct {
    ///     field: 123,
    /// };
    ///
    /// let v = Value::Structable(&my_struct);
    /// ```
    Structable(&'a dyn Structable),

    /// A Rust enum value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Value, Valuable};
    ///
    /// #[derive(Valuable)]
    /// enum MyEnum {
    ///     Foo,
    ///     Bar,
    /// }
    ///
    /// let my_enum = MyEnum::Foo;
    /// let v = Value::Enumerable(&my_enum);
    /// ```
    Enumerable(&'a dyn Enumerable),

    /// A tuple value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Value, Valuable};
    ///
    /// let my_tuple = (123, 456);
    /// let v = Value::Tuplable(&my_tuple);
    /// ```
    Tuplable(&'a dyn Tuplable),
}

impl Valuable for Value<'_> {
    fn as_value(&self) -> Value<'_> {
        *self
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(*self);
    }
}

impl Default for Value<'_> {
    fn default() -> Self {
        Value::Unit
    }
}

macro_rules! convert {
    (
        $(
            $(#[$attrs:meta])*
            $ty:ty => $as:ident,
        )*
    ) => {
        impl<'a> Value<'a> {
            /// Return a `bool` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            ///
            /// assert_eq!(Value::Bool(true).as_bool(), Some(true));
            /// assert_eq!(Value::Char('c').as_bool(), None);
            /// ```
            pub fn as_bool(&self) -> Option<bool> {
                match *self {
                    Value::Bool(v) => Some(v),
                    _ => None,
                }
            }

            /// Return a `char` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            ///
            /// assert_eq!(Value::Char('c').as_char(), Some('c'));
            /// assert_eq!(Value::Bool(true).as_char(), None);
            /// ```
            pub fn as_char(&self) -> Option<char> {
                match *self {
                    Value::Char(v) => Some(v),
                    _ => None,
                }
            }

            /// Return a `f32` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            ///
            /// assert_eq!(Value::F32(3.1415).as_f32(), Some(3.1415));
            /// assert_eq!(Value::Bool(true).as_f32(), None);
            /// ```
            pub fn as_f32(&self) -> Option<f32> {
                match *self {
                    Value::F32(v) => Some(v),
                    _ => None,
                }
            }

            /// Return a `f64` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            ///
            /// assert_eq!(Value::F64(3.1415).as_f64(), Some(3.1415));
            /// assert_eq!(Value::Bool(true).as_f64(), None);
            /// ```
            pub fn as_f64(&self) -> Option<f64> {
                match *self {
                    Value::F64(v) => Some(v),
                    _ => None,
                }
            }

            $(
                $(#[$attrs])*
                pub fn $as(&self) -> Option<$ty> {
                    use Value::*;

                    match *self {
                        I8(v) => v.try_into().ok(),
                        I16(v) => v.try_into().ok(),
                        I32(v) => v.try_into().ok(),
                        I64(v) => v.try_into().ok(),
                        I128(v) => v.try_into().ok(),
                        Isize(v) => v.try_into().ok(),
                        U8(v) => v.try_into().ok(),
                        U16(v) => v.try_into().ok(),
                        U32(v) => v.try_into().ok(),
                        U64(v) => v.try_into().ok(),
                        U128(v) => v.try_into().ok(),
                        Usize(v) => v.try_into().ok(),
                        _ => None,
                    }
                }
            )*

            /// Return a `&str` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            ///
            /// assert_eq!(Value::String("hello").as_str(), Some("hello"));
            /// assert_eq!(Value::Bool(true).as_str(), None);
            /// ```
            pub fn as_str(&self) -> Option<&str> {
                match *self {
                    Value::String(v) => Some(v),
                    _ => None,
                }
            }

            /// Return a `&Path` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            /// use std::path::Path;
            ///
            /// let path = Path::new("a.txt");
            ///
            /// assert!(Value::Path(path).as_path().is_some());
            /// assert!(Value::Bool(true).as_path().is_none());
            /// ```
            #[cfg(feature = "std")]
            pub fn as_path(&self) -> Option<&std::path::Path> {
                match *self {
                    Value::Path(v) => Some(v),
                    _ => None,
                }
            }

            /// Return a `&dyn Error` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            /// use std::io;
            ///
            /// let err: io::Error = io::ErrorKind::Other.into();
            ///
            /// assert!(Value::Error(&err).as_error().is_some());
            /// assert!(Value::Bool(true).as_error().is_none());
            /// ```
            #[cfg(feature = "std")]
            pub fn as_error(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match *self {
                    Value::Error(v) => Some(v),
                    _ => None,
                }
            }


            /// Return a `&dyn Listable` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            ///
            /// let list = vec![1, 2, 3, 4];
            ///
            /// assert!(Value::Listable(&list).as_listable().is_some());
            /// assert!(Value::Bool(true).as_listable().is_none());
            /// ```
            pub fn as_listable(&self) -> Option<&dyn Listable> {
                match *self {
                    Value::Listable(v) => Some(v),
                    _ => None,
                }
            }

            /// Return a `&dyn Mappable` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            /// use std::collections::HashMap;
            ///
            /// let mut map = HashMap::new();
            /// map.insert("foo", 123);
            /// map.insert("bar", 456);
            ///
            /// assert!(Value::Mappable(&map).as_mappable().is_some());
            /// assert!(Value::Bool(true).as_mappable().is_none());
            /// ```
            pub fn as_mappable(&self) -> Option<&dyn Mappable> {
                match *self {
                    Value::Mappable(v) => Some(v),
                    _ => None,
                }
            }

            /// Return a `&dyn Structable` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::{Value, Valuable};
            ///
            /// #[derive(Valuable)]
            /// struct Hello {
            ///     message: &'static str,
            /// }
            ///
            /// let hello = Hello { message: "Hello world" };
            ///
            /// assert!(Value::Structable(&hello).as_structable().is_some());
            /// assert!(Value::Bool(true).as_structable().is_none());
            /// ```
            pub fn as_structable(&self) -> Option<&dyn Structable> {
                match *self {
                    Value::Structable(v) => Some(v),
                    _ => None,
                }
            }

            /// Return a `&dyn Enumerable` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::{Value, Valuable};
            ///
            /// #[derive(Valuable)]
            /// enum Greet {
            ///     Hello,
            ///     World,
            /// }
            ///
            /// let greet = Greet::Hello;
            ///
            /// assert!(Value::Enumerable(&greet).as_enumerable().is_some());
            /// assert!(Value::Bool(true).as_enumerable().is_none());
            /// ```
            pub fn as_enumerable(&self) -> Option<&dyn Enumerable> {
                match *self {
                    Value::Enumerable(v) => Some(v),
                    _ => None,
                }
            }

            /// Return a `&dyn Tuplable` representation of `self`, if possible.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Value;
            ///
            /// let my_tuple = (123, 456);
            ///
            /// assert!(Value::Tuplable(&my_tuple).as_tuplable().is_some());
            /// assert!(Value::Bool(true).as_tuplable().is_none());
            /// ```
            pub fn as_tuplable(&self) -> Option<&dyn Tuplable> {
                match *self {
                    Value::Tuplable(v) => Some(v),
                    _ => None,
                }
            }
        }
    }
}

convert! {
    /// Return a `i8` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::I8(42).as_i8(), Some(42));
    /// assert_eq!(Value::I32(42).as_i8(), Some(42));
    ///
    /// assert_eq!(Value::I64(i64::MAX).as_i8(), None);
    /// assert_eq!(Value::Bool(true).as_i8(), None);
    /// ```
    i8 => as_i8,

    /// Return a `i16` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::I16(42).as_i16(), Some(42));
    /// assert_eq!(Value::I32(42).as_i16(), Some(42));
    ///
    /// assert_eq!(Value::I64(i64::MAX).as_i16(), None);
    /// assert_eq!(Value::Bool(true).as_i16(), None);
    /// ```
    i16 => as_i16,

    /// Return a `i32` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::I32(42).as_i32(), Some(42));
    /// assert_eq!(Value::I64(42).as_i32(), Some(42));
    ///
    /// assert_eq!(Value::I64(i64::MAX).as_i32(), None);
    /// assert_eq!(Value::Bool(true).as_i32(), None);
    /// ```
    i32 => as_i32,

    /// Return a `i64` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::I64(42).as_i64(), Some(42));
    /// assert_eq!(Value::I128(42).as_i64(), Some(42));
    ///
    /// assert_eq!(Value::I128(i128::MAX).as_i64(), None);
    /// assert_eq!(Value::Bool(true).as_i64(), None);
    /// ```
    i64 => as_i64,

    /// Return a `i128` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::I128(42).as_i128(), Some(42));
    /// assert_eq!(Value::U128(42).as_i128(), Some(42));
    ///
    /// assert_eq!(Value::U128(u128::MAX).as_i128(), None);
    /// assert_eq!(Value::Bool(true).as_i128(), None);
    /// ```
    i128 => as_i128,

    /// Return a `isize` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::Isize(42).as_isize(), Some(42));
    /// assert_eq!(Value::Usize(42).as_isize(), Some(42));
    ///
    /// assert_eq!(Value::Usize(usize::MAX).as_isize(), None);
    /// assert_eq!(Value::Bool(true).as_isize(), None);
    /// ```
    isize => as_isize,

    /// Return a `u8` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::U8(42).as_u8(), Some(42));
    /// assert_eq!(Value::U32(42).as_u8(), Some(42));
    ///
    /// assert_eq!(Value::U32(u32::MAX).as_u8(), None);
    /// assert_eq!(Value::Bool(true).as_u8(), None);
    /// ```
    u8 => as_u8,

    /// Return a `u16` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::U16(42).as_u16(), Some(42));
    /// assert_eq!(Value::U32(42).as_u16(), Some(42));
    ///
    /// assert_eq!(Value::U32(u32::MAX).as_u16(), None);
    /// assert_eq!(Value::Bool(true).as_u16(), None);
    /// ```
    u16 => as_u16,

    /// Return a `u32` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::U32(42).as_u32(), Some(42));
    /// assert_eq!(Value::U64(42).as_u32(), Some(42));
    ///
    /// assert_eq!(Value::U64(u64::MAX).as_u32(), None);
    /// assert_eq!(Value::Bool(true).as_u32(), None);
    /// ```
    u32 => as_u32,

    /// Return a `u64` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::U64(42).as_u64(), Some(42));
    /// assert_eq!(Value::U128(42).as_u64(), Some(42));
    ///
    /// assert_eq!(Value::U128(u128::MAX).as_u64(), None);
    /// assert_eq!(Value::Bool(true).as_u64(), None);
    /// ```
    u64 => as_u64,

    /// Return a `u128` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::U128(42).as_u128(), Some(42));
    /// assert_eq!(Value::I32(42).as_u128(), Some(42));
    ///
    /// assert_eq!(Value::I32(-5).as_u128(), None);
    /// assert_eq!(Value::Bool(true).as_u128(), None);
    /// ```
    u128 => as_u128,

    /// Return a `usize` representation of `self`, if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// assert_eq!(Value::Usize(42).as_usize(), Some(42));
    /// assert_eq!(Value::I8(42).as_usize(), Some(42));
    ///
    /// assert_eq!(Value::I8(-5).as_usize(), None);
    /// assert_eq!(Value::Bool(true).as_usize(), None);
    /// ```
    usize => as_usize,
}
