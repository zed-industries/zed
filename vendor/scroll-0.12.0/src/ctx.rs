//! Generic context-aware conversion traits, for automatic _downstream_ extension of `Pread`, et. al
//!
//! The context traits are arguably the center piece of the scroll crate. In simple terms they
//! define how to actually read and write, respectively, a data type from a container, being able to
//! take context into account.
//!
//! ### Reading
//!
//! Types implementing [TryFromCtx](trait.TryFromCtx.html) and it's infallible cousin [FromCtx](trait.FromCtx.html)
//! allow a user of [Pread::pread](../trait.Pread.html#method.pread) or respectively
//! [Cread::cread](../trait.Cread.html#method.cread) and
//! [IOread::ioread](../trait.IOread.html#method.ioread) to read that data type from a data source one
//! of the `*read` traits has been implemented for.
//!
//! Implementations of `TryFromCtx` specify a source (called `This`) and an `Error` type for failed
//! reads. The source defines the kind of container the type can be read from, and defaults to
//! `[u8]` for any type that implements `AsRef<[u8]>`.
//!
//! `FromCtx` is slightly more restricted; it requires the implementer to use `[u8]` as source and
//! never fail, and thus does not have an `Error` type.
//!
//! Types chosen here are of relevance to `Pread` implementations; of course only a container which
//! can produce a source of the type `This` can be used to read a `TryFromCtx` requiring it and the
//! `Error` type returned in `Err` of `Pread::pread`'s Result.
//!
//! ### Writing
//!
//! [TryIntoCtx](trait.TryIntoCtx.html) and the infallible [IntoCtx](trait.IntoCtx.html) work
//! similarly to the above traits, allowing [Pwrite::pwrite](../trait.Pwrite.html#method.pwrite) or
//! respectively [Cwrite::cwrite](../trait.Cwrite.html#method.cwrite) and
//! [IOwrite::iowrite](../trait.IOwrite.html#method.iowrite) to write data into a byte sink for
//! which one of the `*write` traits has been implemented for.
//!
//! `IntoCtx` is similarly restricted as `FromCtx` is to `TryFromCtx`. And equally the types chosen
//! affect usable `Pwrite` implementation.
//!
//! ### Context
//!
//! Each of the traits passes along a `Ctx` to the marshalling logic. This context type contains
//! any additional information that may be required to successfully parse or write the data:
//! Examples would be endianness to use, field lengths of a serialized struct, or delimiters to use
//! when reading/writing `&str`. The context type can be any type but must derive
//! [Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html). In addition if you want to use
//! the `*read`-methods instead of the `*read_with` ones you must also implement
//! [default::Default](https://doc.rust-lang.org/std/default/trait.Default.html).
//!
//! # Example
//!
//! Let's expand on the [previous example](../index.html#complex-use-cases).
//!
//! ```rust
//! use scroll::{self, ctx, Pread, Endian};
//! use scroll::ctx::StrCtx;
//!
//! #[derive(Copy, Clone, PartialEq, Eq)]
//! enum FieldSize {
//!     U32,
//!     U64
//! }
//!
//! // Our custom context type. As said above it has to derive Copy.
//! #[derive(Copy, Clone)]
//! struct Context {
//!     fieldsize: FieldSize,
//!     endianess: Endian,
//! }
//!
//! // Our custom data type
//! struct Data<'b> {
//!   // These u64 are encoded either as 32-bit or 64-bit wide ints. Which one it is is defined in
//!   // the Context.
//!   // Also, let's imagine they have a strict relationship: A < B < C otherwise the struct is
//!   // invalid.
//!   field_a: u64,
//!   field_b: u64,
//!   field_c: u64,
//!
//!   // Both of these are marshalled with a prefixed length.
//!   name: &'b str,
//!   value: &'b [u8],
//! }
//!
//! #[derive(Debug)]
//! enum Error {
//!     // We'll return this custom error if the field* relationship doesn't hold
//!     BadFieldMatchup,
//!     Scroll(scroll::Error),
//! }
//!
//! impl<'a> ctx::TryFromCtx<'a, Context> for Data<'a> {
//!   type Error = Error;
//!
//!   // Using the explicit lifetime specification again you ensure that read data doesn't outlife
//!   // its source buffer without having to resort to copying.
//!   fn try_from_ctx (src: &'a [u8], ctx: Context)
//!     // the `usize` returned here is the amount of bytes read.
//!     -> Result<(Self, usize), Self::Error>
//!   {
//!     // The offset counter; gread and gread_with increment a given counter automatically so we
//!     // don't have to manually care.
//!     let offset = &mut 0;
//!
//!     let field_a;
//!     let field_b;
//!     let field_c;
//!
//!     // Switch the amount of bytes read depending on the parsing context
//!     if ctx.fieldsize == FieldSize::U32 {
//!       field_a = src.gread_with::<u32>(offset, ctx.endianess)? as u64;
//!       field_b = src.gread_with::<u32>(offset, ctx.endianess)? as u64;
//!       field_c = src.gread_with::<u32>(offset, ctx.endianess)? as u64;
//!     } else {
//!       field_a = src.gread_with::<u64>(offset, ctx.endianess)?;
//!       field_b = src.gread_with::<u64>(offset, ctx.endianess)?;
//!       field_c = src.gread_with::<u64>(offset, ctx.endianess)?;
//!     }
//!
//!     // You can use type ascribition or turbofish operators, whichever you prefer.
//!     let namelen = src.gread_with::<u16>(offset, ctx.endianess)? as usize;
//!     let name: &str = src.gread_with(offset, scroll::ctx::StrCtx::Length(namelen))?;
//!
//!     let vallen = src.gread_with::<u16>(offset, ctx.endianess)? as usize;
//!     let value = &src[*offset..(*offset+vallen)];
//!
//!     // Let's sanity check those fields, shall we?
//!     if ! (field_a < field_b && field_b < field_c) {
//!       return Err(Error::BadFieldMatchup);
//!     }
//!
//!     Ok((Data { field_a, field_b, field_c, name, value }, *offset))
//!   }
//! }
//!
//! // In lieu of a complex byte buffer we hearken back to the venerable &[u8]; do note however
//! // that the implementation of TryFromCtx did not specify such. In fact any type that implements
//! // Pread can now read `Data` as it implements TryFromCtx.
//! let bytes = b"\x00\x02\x03\x04\x01\x02\x03\x04\xde\xad\xbe\xef\x00\x08UserName\x00\x02\xCA\xFE";
//!
//! // We define an appropiate context, and get going
//! let contextA = Context {
//!     fieldsize: FieldSize::U32,
//!     endianess: Endian::Big,
//! };
//! let data: Data = bytes.pread_with(0, contextA).unwrap();
//!
//! assert_eq!(data.field_a, 0x00020304);
//! assert_eq!(data.field_b, 0x01020304);
//! assert_eq!(data.field_c, 0xdeadbeef);
//! assert_eq!(data.name, "UserName");
//! assert_eq!(data.value, [0xCA, 0xFE]);
//!
//! // Here we have a context with a different FieldSize, changing parsing information at runtime.
//! let contextB = Context {
//!     fieldsize: FieldSize::U64,
//!     endianess: Endian::Big,
//! };
//!
//! // Which will of course error with a malformed input for the context
//! let err: Result<Data, Error> = bytes.pread_with(0, contextB);
//! assert!(err.is_err());
//!
//! let bytes_long = [0x00,0x00,0x00,0x00,0x00,0x02,0x03,0x04,0x00,0x00,0x00,0x00,0x01,0x02,0x03,
//!                   0x04,0x00,0x00,0x00,0x00,0xde,0xad,0xbe,0xef,0x00,0x08,0x55,0x73,0x65,0x72,
//!                   0x4e,0x61,0x6d,0x65,0x00,0x02,0xCA,0xFE];
//!
//! let data: Data = bytes_long.pread_with(0, contextB).unwrap();
//!
//! assert_eq!(data.field_a, 0x00020304);
//! assert_eq!(data.field_b, 0x01020304);
//! assert_eq!(data.field_c, 0xdeadbeef);
//! assert_eq!(data.name, "UserName");
//! assert_eq!(data.value, [0xCA, 0xFE]);
//!
//! // Ergonomic conversion, not relevant really.
//! use std::convert::From;
//! impl From<scroll::Error> for Error {
//!   fn from(error: scroll::Error) -> Error {
//!     Error::Scroll(error)
//!   }
//! }
//! ```

use core::mem::{size_of, MaybeUninit};
use core::ptr::copy_nonoverlapping;
use core::{result, str};
#[cfg(feature = "std")]
use std::ffi::{CStr, CString};

use crate::endian::Endian;
use crate::{error, Pread, Pwrite};

/// A trait for measuring how large something is; for a byte sequence, it will be its length.
pub trait MeasureWith<Ctx> {
    /// How large is `Self`, given the `ctx`?
    fn measure_with(&self, ctx: &Ctx) -> usize;
}

impl<Ctx> MeasureWith<Ctx> for [u8] {
    #[inline]
    fn measure_with(&self, _ctx: &Ctx) -> usize {
        self.len()
    }
}

impl<Ctx, T: AsRef<[u8]>> MeasureWith<Ctx> for T {
    #[inline]
    fn measure_with(&self, _ctx: &Ctx) -> usize {
        self.as_ref().len()
    }
}

/// The parsing context for converting a byte sequence to a `&str`
///
/// `StrCtx` specifies what byte delimiter to use, and defaults to C-style null terminators. Be careful.
#[derive(Debug, Copy, Clone)]
pub enum StrCtx {
    Delimiter(u8),
    DelimiterUntil(u8, usize),
    Length(usize),
}

/// A C-style, null terminator based delimiter
pub const NULL: u8 = 0;
/// A space-based delimiter
pub const SPACE: u8 = 0x20;
/// A newline-based delimiter
pub const RET: u8 = 0x0a;
/// A tab-based delimiter
pub const TAB: u8 = 0x09;

impl Default for StrCtx {
    #[inline]
    fn default() -> Self {
        StrCtx::Delimiter(NULL)
    }
}

impl StrCtx {
    pub fn len(&self) -> usize {
        match self {
            StrCtx::Delimiter(_) | StrCtx::DelimiterUntil(_, _) => 1,
            StrCtx::Length(_) => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, StrCtx::Length(_))
    }
}

/// Reads `Self` from `This` using the context `Ctx`; must _not_ fail
pub trait FromCtx<Ctx: Copy = (), This: ?Sized = [u8]> {
    fn from_ctx(this: &This, ctx: Ctx) -> Self;
}

/// Tries to read `Self` from `This` using the context `Ctx`
///
/// # Implementing Your Own Reader
/// If you want to implement your own reader for a type `Foo` from some kind of buffer (say
/// `[u8]`), then you need to implement this trait
///
/// ```rust
/// ##[cfg(feature = "std")] {
/// use scroll::{self, ctx, Pread};
/// #[derive(Debug, PartialEq, Eq)]
/// pub struct Foo(u16);
///
/// impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for Foo {
///      type Error = scroll::Error;
///      fn try_from_ctx(this: &'a [u8], le: scroll::Endian) -> Result<(Self, usize), Self::Error> {
///          if this.len() < 2 { return Err((scroll::Error::Custom("whatever".to_string())).into()) }
///          let n = this.pread_with(0, le)?;
///          Ok((Foo(n), 2))
///      }
/// }
///
/// let bytes: [u8; 4] = [0xde, 0xad, 0, 0];
/// let foo = bytes.pread_with::<Foo>(0, scroll::LE).unwrap();
/// assert_eq!(Foo(0xadde), foo);
///
/// let foo2 = bytes.pread_with::<Foo>(0, scroll::BE).unwrap();
/// assert_eq!(Foo(0xdeadu16), foo2);
/// # }
/// ```
///
/// # Advanced: Using Your Own Error in `TryFromCtx`
/// ```rust
///  use scroll::{self, ctx, Pread};
///  use std::error;
///  use std::fmt::{self, Display};
///  // make some kind of normal error which also can transformed from a scroll error
///  #[derive(Debug)]
///  pub struct ExternalError {}
///
///  impl Display for ExternalError {
///      fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
///          write!(fmt, "ExternalError")
///      }
///  }
///
///  impl error::Error for ExternalError {
///      fn description(&self) -> &str {
///          "ExternalError"
///      }
///      fn cause(&self) -> Option<&dyn error::Error> { None}
///  }
///
///  impl From<scroll::Error> for ExternalError {
///      fn from(err: scroll::Error) -> Self {
///          match err {
///              _ => ExternalError{},
///          }
///      }
///  }
///  #[derive(Debug, PartialEq, Eq)]
///  pub struct Foo(u16);
///
///  impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for Foo {
///      type Error = ExternalError;
///      fn try_from_ctx(this: &'a [u8], le: scroll::Endian) -> Result<(Self, usize), Self::Error> {
///          if this.len() <= 2 { return Err((ExternalError {}).into()) }
///          let offset = &mut 0;
///          let n = this.gread_with(offset, le)?;
///          Ok((Foo(n), *offset))
///      }
///  }
///
/// let bytes: [u8; 4] = [0xde, 0xad, 0, 0];
/// let foo: Result<Foo, ExternalError> = bytes.pread(0);
/// ```
pub trait TryFromCtx<'a, Ctx: Copy = (), This: ?Sized = [u8]>
where
    Self: 'a + Sized,
{
    type Error;
    fn try_from_ctx(from: &'a This, ctx: Ctx) -> Result<(Self, usize), Self::Error>;
}

/// Writes `Self` into `This` using the context `Ctx`
pub trait IntoCtx<Ctx: Copy = (), This: ?Sized = [u8]>: Sized {
    fn into_ctx(self, _: &mut This, ctx: Ctx);
}

/// Tries to write `Self` into `This` using the context `Ctx`
/// To implement writing into an arbitrary byte buffer, implement `TryIntoCtx`
/// # Example
/// ```rust
/// ##[cfg(feature = "std")] {
/// use scroll::{self, ctx, LE, Endian, Pwrite};
/// #[derive(Debug, PartialEq, Eq)]
/// pub struct Foo(u16);
///
/// // this will use the default `DefaultCtx = scroll::Endian`
/// impl ctx::TryIntoCtx<Endian> for Foo {
///     // you can use your own error here too, but you will then need to specify it in fn generic parameters
///     type Error = scroll::Error;
///     // you can write using your own context type, see `leb128.rs`
///     fn try_into_ctx(self, this: &mut [u8], le: Endian) -> Result<usize, Self::Error> {
///         if this.len() < 2 { return Err((scroll::Error::Custom("whatever".to_string())).into()) }
///         this.pwrite_with(self.0, 0, le)?;
///         Ok(2)
///     }
/// }
/// // now we can write a `Foo` into some buffer (in this case, a byte buffer, because that's what we implemented it for above)
///
/// let mut bytes: [u8; 4] = [0, 0, 0, 0];
/// bytes.pwrite_with(Foo(0x7f), 1, LE).unwrap();
/// # }
/// ```
pub trait TryIntoCtx<Ctx: Copy = (), This: ?Sized = [u8]>: Sized {
    type Error;
    fn try_into_ctx(self, _: &mut This, ctx: Ctx) -> Result<usize, Self::Error>;
}

/// Gets the size of `Self` with a `Ctx`, and in `Self::Units`. Implementors can then call `Gread` related functions
///
/// The rationale behind this trait is to:
///
/// 1. Prevent `gread` from being used, and the offset being modified based on simply the sizeof the value, which can be a misnomer, e.g., for Leb128, etc.
/// 2. Allow a context based size, which is useful for 32/64 bit variants for various containers, etc.
pub trait SizeWith<Ctx = ()> {
    fn size_with(ctx: &Ctx) -> usize;
}

#[rustfmt::skip]
macro_rules! signed_to_unsigned {
    (i8) =>  {u8 };
    (u8) =>  {u8 };
    (i16) => {u16};
    (u16) => {u16};
    (i32) => {u32};
    (u32) => {u32};
    (i64) => {u64};
    (u64) => {u64};
    (i128) => {u128};
    (u128) => {u128};
    (f32) => {u32};
    (f64) => {u64};
}

macro_rules! write_into {
    ($typ:ty, $size:expr, $n:expr, $dst:expr, $endian:expr) => {{
        assert!($dst.len() >= $size);
        let bytes = if $endian.is_little() {
            $n.to_le()
        } else {
            $n.to_be()
        }
        .to_ne_bytes();
        unsafe {
            copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
        }
    }};
}

macro_rules! into_ctx_impl {
    ($typ:tt, $size:expr) => {
        impl IntoCtx<Endian> for $typ {
            #[inline]
            fn into_ctx(self, dst: &mut [u8], le: Endian) {
                assert!(dst.len() >= $size);
                write_into!($typ, $size, self, dst, le);
            }
        }
        impl<'a> IntoCtx<Endian> for &'a $typ {
            #[inline]
            fn into_ctx(self, dst: &mut [u8], le: Endian) {
                (*self).into_ctx(dst, le)
            }
        }
        impl TryIntoCtx<Endian> for $typ
        where
            $typ: IntoCtx<Endian>,
        {
            type Error = error::Error;
            #[inline]
            fn try_into_ctx(self, dst: &mut [u8], le: Endian) -> error::Result<usize> {
                if $size > dst.len() {
                    Err(error::Error::TooBig {
                        size: $size,
                        len: dst.len(),
                    })
                } else {
                    <$typ as IntoCtx<Endian>>::into_ctx(self, dst, le);
                    Ok($size)
                }
            }
        }
        impl<'a> TryIntoCtx<Endian> for &'a $typ {
            type Error = error::Error;
            #[inline]
            fn try_into_ctx(self, dst: &mut [u8], le: Endian) -> error::Result<usize> {
                (*self).try_into_ctx(dst, le)
            }
        }
    };
}

macro_rules! from_ctx_impl {
    ($typ:tt, $size:expr) => {
        impl<'a> FromCtx<Endian> for $typ {
            #[inline]
            fn from_ctx(src: &[u8], le: Endian) -> Self {
                assert!(src.len() >= $size);
                let mut data: signed_to_unsigned!($typ) = 0;
                unsafe {
                    copy_nonoverlapping(
                        src.as_ptr(),
                        &mut data as *mut signed_to_unsigned!($typ) as *mut u8,
                        $size,
                    );
                }
                (if le.is_little() {
                    data.to_le()
                } else {
                    data.to_be()
                }) as $typ
            }
        }

        impl<'a> TryFromCtx<'a, Endian> for $typ
        where
            $typ: FromCtx<Endian>,
        {
            type Error = error::Error;
            #[inline]
            fn try_from_ctx(
                src: &'a [u8],
                le: Endian,
            ) -> result::Result<(Self, usize), Self::Error> {
                if $size > src.len() {
                    Err(error::Error::TooBig {
                        size: $size,
                        len: src.len(),
                    })
                } else {
                    Ok((FromCtx::from_ctx(&src, le), $size))
                }
            }
        }
        // as ref
        impl<'a, T> FromCtx<Endian, T> for $typ
        where
            T: AsRef<[u8]>,
        {
            #[inline]
            fn from_ctx(src: &T, le: Endian) -> Self {
                let src = src.as_ref();
                assert!(src.len() >= $size);
                let mut data: signed_to_unsigned!($typ) = 0;
                unsafe {
                    copy_nonoverlapping(
                        src.as_ptr(),
                        &mut data as *mut signed_to_unsigned!($typ) as *mut u8,
                        $size,
                    );
                }
                (if le.is_little() {
                    data.to_le()
                } else {
                    data.to_be()
                }) as $typ
            }
        }

        impl<'a, T> TryFromCtx<'a, Endian, T> for $typ
        where
            $typ: FromCtx<Endian, T>,
            T: AsRef<[u8]>,
        {
            type Error = error::Error;
            #[inline]
            fn try_from_ctx(src: &'a T, le: Endian) -> result::Result<(Self, usize), Self::Error> {
                let src = src.as_ref();
                Self::try_from_ctx(src, le)
            }
        }
    };
}

macro_rules! ctx_impl {
    ($typ:tt, $size:expr) => {
        from_ctx_impl!($typ, $size);
    };
}

ctx_impl!(u8, 1);
ctx_impl!(i8, 1);
ctx_impl!(u16, 2);
ctx_impl!(i16, 2);
ctx_impl!(u32, 4);
ctx_impl!(i32, 4);
ctx_impl!(u64, 8);
ctx_impl!(i64, 8);
ctx_impl!(u128, 16);
ctx_impl!(i128, 16);

macro_rules! from_ctx_float_impl {
    ($typ:tt, $size:expr) => {
        impl<'a> FromCtx<Endian> for $typ {
            #[inline]
            fn from_ctx(src: &[u8], le: Endian) -> Self {
                assert!(src.len() >= ::core::mem::size_of::<Self>());
                let mut data: signed_to_unsigned!($typ) = 0;
                unsafe {
                    copy_nonoverlapping(
                        src.as_ptr(),
                        &mut data as *mut signed_to_unsigned!($typ) as *mut u8,
                        $size,
                    );
                }
                $typ::from_bits(if le.is_little() {
                    data.to_le()
                } else {
                    data.to_be()
                })
            }
        }
        impl<'a> TryFromCtx<'a, Endian> for $typ
        where
            $typ: FromCtx<Endian>,
        {
            type Error = error::Error;
            #[inline]
            fn try_from_ctx(
                src: &'a [u8],
                le: Endian,
            ) -> result::Result<(Self, usize), Self::Error> {
                if $size > src.len() {
                    Err(error::Error::TooBig {
                        size: $size,
                        len: src.len(),
                    })
                } else {
                    Ok((FromCtx::from_ctx(src, le), $size))
                }
            }
        }
    };
}

from_ctx_float_impl!(f32, 4);
from_ctx_float_impl!(f64, 8);

into_ctx_impl!(u8, 1);
into_ctx_impl!(i8, 1);
into_ctx_impl!(u16, 2);
into_ctx_impl!(i16, 2);
into_ctx_impl!(u32, 4);
into_ctx_impl!(i32, 4);
into_ctx_impl!(u64, 8);
into_ctx_impl!(i64, 8);
into_ctx_impl!(u128, 16);
into_ctx_impl!(i128, 16);

macro_rules! into_ctx_float_impl {
    ($typ:tt, $size:expr) => {
        impl IntoCtx<Endian> for $typ {
            #[inline]
            fn into_ctx(self, dst: &mut [u8], le: Endian) {
                assert!(dst.len() >= $size);
                write_into!(signed_to_unsigned!($typ), $size, self.to_bits(), dst, le);
            }
        }
        impl<'a> IntoCtx<Endian> for &'a $typ {
            #[inline]
            fn into_ctx(self, dst: &mut [u8], le: Endian) {
                (*self).into_ctx(dst, le)
            }
        }
        impl TryIntoCtx<Endian> for $typ
        where
            $typ: IntoCtx<Endian>,
        {
            type Error = error::Error;
            #[inline]
            fn try_into_ctx(self, dst: &mut [u8], le: Endian) -> error::Result<usize> {
                if $size > dst.len() {
                    Err(error::Error::TooBig {
                        size: $size,
                        len: dst.len(),
                    })
                } else {
                    <$typ as IntoCtx<Endian>>::into_ctx(self, dst, le);
                    Ok($size)
                }
            }
        }
        impl<'a> TryIntoCtx<Endian> for &'a $typ {
            type Error = error::Error;
            #[inline]
            fn try_into_ctx(self, dst: &mut [u8], le: Endian) -> error::Result<usize> {
                (*self).try_into_ctx(dst, le)
            }
        }
    };
}

into_ctx_float_impl!(f32, 4);
into_ctx_float_impl!(f64, 8);

impl<'a> TryFromCtx<'a, StrCtx> for &'a str {
    type Error = error::Error;
    #[inline]
    /// Read a `&str` from `src` using `delimiter`
    fn try_from_ctx(src: &'a [u8], ctx: StrCtx) -> Result<(Self, usize), Self::Error> {
        let len = match ctx {
            StrCtx::Length(len) => len,
            StrCtx::Delimiter(delimiter) => src.iter().take_while(|c| **c != delimiter).count(),
            StrCtx::DelimiterUntil(delimiter, len) => {
                if len > src.len() {
                    return Err(error::Error::TooBig {
                        size: len,
                        len: src.len(),
                    });
                };
                src.iter()
                    .take_while(|c| **c != delimiter)
                    .take(len)
                    .count()
            }
        };

        if len > src.len() {
            return Err(error::Error::TooBig {
                size: len,
                len: src.len(),
            });
        };

        match str::from_utf8(&src[..len]) {
            Ok(res) => Ok((res, len + ctx.len())),
            Err(_) => Err(error::Error::BadInput {
                size: src.len(),
                msg: "invalid utf8",
            }),
        }
    }
}

impl<'a, T> TryFromCtx<'a, StrCtx, T> for &'a str
where
    T: AsRef<[u8]>,
{
    type Error = error::Error;
    #[inline]
    fn try_from_ctx(src: &'a T, ctx: StrCtx) -> result::Result<(Self, usize), Self::Error> {
        let src = src.as_ref();
        TryFromCtx::try_from_ctx(src, ctx)
    }
}

impl<'a> TryIntoCtx for &'a [u8] {
    type Error = error::Error;
    #[inline]
    fn try_into_ctx(self, dst: &mut [u8], _ctx: ()) -> error::Result<usize> {
        let src_len = self.len() as isize;
        let dst_len = dst.len() as isize;
        // if src_len < 0 || dst_len < 0 || offset < 0 {
        //     return Err(error::Error::BadOffset(format!("requested operation has negative casts: src len: {src_len} dst len: {dst_len} offset: {offset}")).into())
        // }
        if src_len > dst_len {
            Err(error::Error::TooBig {
                size: self.len(),
                len: dst.len(),
            })
        } else {
            unsafe { copy_nonoverlapping(self.as_ptr(), dst.as_mut_ptr(), src_len as usize) };
            Ok(self.len())
        }
    }
}

// TODO: make TryIntoCtx use StrCtx for awesomeness
impl<'a> TryIntoCtx for &'a str {
    type Error = error::Error;
    #[inline]
    fn try_into_ctx(self, dst: &mut [u8], _ctx: ()) -> error::Result<usize> {
        let bytes = self.as_bytes();
        TryIntoCtx::try_into_ctx(bytes, dst, ())
    }
}

// TODO: we can make this compile time without size_of call, but compiler probably does that anyway
macro_rules! sizeof_impl {
    ($ty:ty) => {
        impl SizeWith<Endian> for $ty {
            #[inline]
            fn size_with(_ctx: &Endian) -> usize {
                size_of::<$ty>()
            }
        }
    };
}

sizeof_impl!(u8);
sizeof_impl!(i8);
sizeof_impl!(u16);
sizeof_impl!(i16);
sizeof_impl!(u32);
sizeof_impl!(i32);
sizeof_impl!(u64);
sizeof_impl!(i64);
sizeof_impl!(u128);
sizeof_impl!(i128);
sizeof_impl!(f32);
sizeof_impl!(f64);

impl<'a> TryFromCtx<'a, usize> for &'a [u8] {
    type Error = error::Error;
    #[inline]
    fn try_from_ctx(src: &'a [u8], size: usize) -> result::Result<(Self, usize), Self::Error> {
        if size > src.len() {
            Err(error::Error::TooBig {
                size,
                len: src.len(),
            })
        } else {
            Ok((&src[..size], size))
        }
    }
}

impl<'a, Ctx: Copy, T: TryFromCtx<'a, Ctx, Error = error::Error>, const N: usize>
    TryFromCtx<'a, Ctx> for [T; N]
{
    type Error = error::Error;
    fn try_from_ctx(src: &'a [u8], ctx: Ctx) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        let mut buf: [MaybeUninit<T>; N] = core::array::from_fn(|_| MaybeUninit::uninit());

        let mut error_ctx = None;
        for (idx, element) in buf.iter_mut().enumerate() {
            match src.gread_with::<T>(&mut offset, ctx) {
                Ok(val) => {
                    *element = MaybeUninit::new(val);
                }
                Err(e) => {
                    error_ctx = Some((e, idx));
                    break;
                }
            }
        }
        if let Some((e, idx)) = error_ctx {
            for element in &mut buf[0..idx].iter_mut() {
                // SAFETY: Any element upto idx must have already been initialized, since
                // we iterate until we encounter an error.
                unsafe {
                    element.assume_init_drop();
                }
            }
            Err(e)
        } else {
            // SAFETY: we initialized each element above by preading them out, correctness
            // of the initialized element is guaranted by pread itself
            Ok((buf.map(|element| unsafe { element.assume_init() }), offset))
        }
    }
}
impl<Ctx: Copy, T: TryIntoCtx<Ctx, Error = error::Error>, const N: usize> TryIntoCtx<Ctx>
    for [T; N]
{
    type Error = error::Error;
    fn try_into_ctx(self, buf: &mut [u8], ctx: Ctx) -> Result<usize, Self::Error> {
        let mut offset = 0;
        for element in self {
            buf.gwrite_with(element, &mut offset, ctx)?;
        }
        Ok(offset)
    }
}

#[cfg(feature = "std")]
impl<'a> TryFromCtx<'a> for &'a CStr {
    type Error = error::Error;
    #[inline]
    fn try_from_ctx(src: &'a [u8], _ctx: ()) -> result::Result<(Self, usize), Self::Error> {
        let null_byte = match src.iter().position(|b| *b == 0) {
            Some(ix) => ix,
            None => {
                return Err(error::Error::BadInput {
                    size: 0,
                    msg: "The input doesn't contain a null byte",
                })
            }
        };

        let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(&src[..=null_byte]) };
        Ok((cstr, null_byte + 1))
    }
}

#[cfg(feature = "std")]
impl<'a> TryFromCtx<'a> for CString {
    type Error = error::Error;
    #[inline]
    fn try_from_ctx(src: &'a [u8], _ctx: ()) -> result::Result<(Self, usize), Self::Error> {
        let (raw, bytes_read) = <&CStr as TryFromCtx>::try_from_ctx(src, _ctx)?;
        Ok((raw.to_owned(), bytes_read))
    }
}

#[cfg(feature = "std")]
impl<'a> TryIntoCtx for &'a CStr {
    type Error = error::Error;
    #[inline]
    fn try_into_ctx(self, dst: &mut [u8], _ctx: ()) -> error::Result<usize> {
        let data = self.to_bytes_with_nul();

        if dst.len() < data.len() {
            Err(error::Error::TooBig {
                size: dst.len(),
                len: data.len(),
            })
        } else {
            unsafe {
                copy_nonoverlapping(data.as_ptr(), dst.as_mut_ptr(), data.len());
            }

            Ok(data.len())
        }
    }
}

#[cfg(feature = "std")]
impl TryIntoCtx for CString {
    type Error = error::Error;
    #[inline]
    fn try_into_ctx(self, dst: &mut [u8], _ctx: ()) -> error::Result<usize> {
        self.as_c_str().try_into_ctx(dst, ())
    }
}

// example of marshalling to bytes, let's wait until const is an option
// impl FromCtx for [u8; 10] {
//     fn from_ctx(bytes: &[u8], _ctx: Endian) -> Self {
//         let mut dst: Self = [0; 10];
//         assert!(bytes.len() >= dst.len());
//         unsafe {
//             copy_nonoverlapping(bytes.as_ptr(), dst.as_mut_ptr(), dst.len());
//         }
//         dst
//     }
// }

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;

    #[test]
    fn parse_a_cstr() {
        let src = CString::new("Hello World").unwrap();
        let as_bytes = src.as_bytes_with_nul();

        let (got, bytes_read) = <&CStr as TryFromCtx>::try_from_ctx(as_bytes, ()).unwrap();

        assert_eq!(bytes_read, as_bytes.len());
        assert_eq!(got, src.as_c_str());
    }

    #[test]
    fn round_trip_a_c_str() {
        let src = CString::new("Hello World").unwrap();
        let src = src.as_c_str();
        let as_bytes = src.to_bytes_with_nul();

        let mut buffer = vec![0; as_bytes.len()];
        let bytes_written = src.try_into_ctx(&mut buffer, ()).unwrap();
        assert_eq!(bytes_written, as_bytes.len());

        let (got, bytes_read) = <&CStr as TryFromCtx>::try_from_ctx(&buffer, ()).unwrap();

        assert_eq!(bytes_read, as_bytes.len());
        assert_eq!(got, src);
    }
}
