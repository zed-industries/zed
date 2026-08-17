//! # Scroll
//!
//! ```text, no_run
//!         _______________
//!    ()==(              (@==()
//!         '______________'|
//!           |             |
//!           |   ἀρετή     |
//!         __)_____________|
//!    ()==(               (@==()
//!         '--------------'
//!
//! ```
//!
//! Scroll is a library for easily and efficiently reading/writing types from data containers like
//! byte arrays.
//!
//! ## Easily:
//!
//! Scroll sets down a number of traits:
//!
//! [FromCtx](ctx/trait.FromCtx.html), [IntoCtx](ctx/trait.IntoCtx.html),
//! [TryFromCtx](ctx/trait.TryFromCtx.html) and [TryIntoCtx](ctx/trait.TryIntoCtx.html) — further
//! explained in the [ctx module](ctx/index.html); to be implemented on custom types to allow
//! reading, writing, and potentially fallible reading/writing respectively.
//!
//! [Pread](trait.Pread.html) and [Pwrite](trait.Pwrite.html) which are implemented on data
//! containers such as byte arrays to define how to read or respectively write types implementing
//! the *Ctx traits above.
//! In addition scroll also defines [IOread](trait.IOread.html) and
//! [IOwrite](trait.IOwrite.html) with additional constraits that then allow reading and writing
//! from `std::io` [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html) and
//! [Write](https://doc.rust-lang.org/nightly/std/io/trait.Write.html).
//!
//!
//! In most cases you can use [scroll_derive](https://docs.rs/scroll_derive) to derive sensible
//! defaults for `Pread`, `Pwrite`, their IO counterpart and `SizeWith`.  More complex situations
//! call for manual implementation of those traits; refer to [the ctx module](ctx/index.html) for
//! details.
//!
//!
//! ## Efficiently:
//!
//! Reading Slices — including [&str](https://doc.rust-lang.org/std/primitive.str.html) — supports
//! zero-copy. Scroll is designed with a `no_std` context in mind; every dependency on `std` is
//! cfg-gated and errors need not allocate.
//!
//! Reads by default take only immutable references wherever possible, allowing for trivial
//! parallelization.
//!
//! # Examples
//!
//! Let's start with a simple example
//!
//! ```rust
//! use scroll::{ctx, Pread};
//!
//! // Let's first define some data, cfg-gated so our assertions later on hold.
//! #[cfg(target_endian = "little")]
//! let bytes: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];
//! #[cfg(target_endian = "big")]
//! let bytes: [u8; 4] = [0xef, 0xbe, 0xad, 0xde];
//!
//! // We can read a u32 from the array `bytes` at offset 0.
//! // This will use a default context for the type being parsed;
//! // in the case of u32 this defines to use the host's endianess.
//! let number = bytes.pread::<u32>(0).unwrap();
//! assert_eq!(number, 0xefbeadde);
//!
//!
//! // Similarly we can also read a single byte at offset 2
//! // This time using type ascription instead of the turbofish (::<>) operator.
//! let byte: u8 = bytes.pread(2).unwrap();
//! #[cfg(target_endian = "little")]
//! assert_eq!(byte, 0xbe);
//! #[cfg(target_endian = "big")]
//! assert_eq!(byte, 0xad);
//!
//!
//! // If required we can also provide a specific parsing context; e.g. if we want to explicitly
//! // define the endianess to use:
//! let be_number: u32 = bytes.pread_with(0, scroll::BE).unwrap();
//! #[cfg(target_endian = "little")]
//! assert_eq!(be_number, 0xdeadbeef);
//! #[cfg(target_endian = "big")]
//! assert_eq!(be_number, 0xefbeadde);
//!
//! let be_number16 = bytes.pread_with::<u16>(1, scroll::BE).unwrap();
//! #[cfg(target_endian = "little")]
//! assert_eq!(be_number16, 0xadbe);
//! #[cfg(target_endian = "big")]
//! assert_eq!(be_number16, 0xbead);
//!
//!
//! // Reads may fail; in this example due to a too large read for the given container.
//! // Scroll's error type does not by default allocate to work in environments like no_std.
//! let byte_err: scroll::Result<i64> = bytes.pread(0);
//! assert!(byte_err.is_err());
//!
//!
//! // We can parse out custom datatypes, or types with lifetimes, as long as they implement
//! // the conversion traits `TryFromCtx/FromCtx`.
//! // Here we use the default context for &str which parses are C-style '\0'-delimited string.
//! let hello: &[u8] = b"hello world\0more words";
//! let hello_world: &str = hello.pread(0).unwrap();
//! assert_eq!("hello world", hello_world);
//!
//! // We can again provide a custom context; for example to parse Space-delimited strings.
//! // As you can see while we still call `pread` changing the context can influence the output —
//! // instead of splitting at '\0' we split at spaces
//! let hello2: &[u8] = b"hello world\0more words";
//! let world: &str = hello2.pread_with(6, ctx::StrCtx::Delimiter(ctx::SPACE)).unwrap();
//! assert_eq!("world\0more", world);
//! ```
//!
//! ## `std::io` API
//!
//! Scroll also allows reading from `std::io`. For this the types to read need to implement
//! [FromCtx](ctx/trait.FromCtx.html) and [SizeWith](ctx/trait.SizeWith.html).
//!
//! ```rust
//! ##[cfg(feature = "std")] {
//! use std::io::Cursor;
//! use scroll::{IOread, ctx, Endian};
//! let bytes = [0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00, 0xef,0xbe,0x00,0x00,];
//! let mut cursor = Cursor::new(bytes);
//!
//! // IOread uses std::io::Read methods, thus the Cursor will be incremented on these reads:
//! let prev = cursor.position();
//!
//! let integer = cursor.ioread_with::<u64>(Endian::Little).unwrap();
//!
//! let after = cursor.position();
//!
//! assert!(prev < after);
//!
//! // SizeWith allows us to define a context-sensitive size of a read type:
//! // Contexts can have different instantiations; e.g. the `Endian` context can be either Little or
//! // Big. This is useful if for example the context contains the word-size of fields to be
//! // read/written, e.g. switching between ELF32 or ELF64 at runtime.
//! let size = <u64 as ctx::SizeWith<Endian>>::size_with(&Endian::Little) as u64;
//! assert_eq!(prev + size, after);
//! # }
//! ```
//!
//! In the same vein as IOread we can use IOwrite to write a type to anything implementing
//! `std::io::Write`:
//!
//! ```rust
//! ##[cfg(feature = "std")] {
//! use std::io::Cursor;
//! use scroll::{IOwrite};
//!
//! let mut bytes = [0x0u8; 5];
//! let mut cursor = Cursor::new(&mut bytes[..]);
//!
//! // This of course once again increments the cursor position
//! cursor.iowrite_with(0xdeadbeef as u32, scroll::BE).unwrap();
//!
//! assert_eq!(cursor.into_inner(), [0xde, 0xad, 0xbe, 0xef, 0x0]);
//! # }
//! ```
//!
//! ## Complex use cases
//!
//! Scoll is designed to be highly adaptable while providing a strong abstraction between the types
//! being read/written and the data container containing them.
//!
//! In this example we'll define a custom Data and allow it to be read from an arbitrary byte
//! buffer.
//!
//! ```rust
//! use scroll::{self, ctx, Pread, Endian};
//! use scroll::ctx::StrCtx;
//!
//! // Our custom context type. In a more complex situation you could for example store details on
//! // how to write or read your type, field-sizes or other information.
//! // In this simple example we could also do without using a custom context in the first place.
//! #[derive(Copy, Clone)]
//! struct Context(Endian);
//!
//! // Our custom data type
//! struct Data<'zerocopy> {
//!   // This is only a reference to the actual data; we make use of scroll's zero-copy capability
//!   name: &'zerocopy str,
//!   id: u32,
//! }
//!
//! // To allow for safe zero-copying scroll allows to specify lifetimes explicitly:
//! // The context
//! impl<'a> ctx::TryFromCtx<'a, Context> for Data<'a> {
//!   // If necessary you can set a custom error type here, which will be returned by Pread/Pwrite
//!   type Error = scroll::Error;
//!
//!   // Using the explicit lifetime specification again you ensure that read data doesn't outlife
//!   // its source buffer without having to resort to copying.
//!   fn try_from_ctx (src: &'a [u8], ctx: Context)
//!     // the `usize` returned here is the amount of bytes read.
//!     -> Result<(Self, usize), Self::Error>
//!   {
//!     let offset = &mut 0;
//!
//!     let id = src.gread_with(offset, ctx.0)?;
//!
//!     // In a more serious application you would validate data here of course.
//!     let namelen: u16 = src.gread_with(offset, ctx.0)?;
//!     let name = src.gread_with::<&str>(offset, StrCtx::Length(namelen as usize))?;
//!
//!     Ok((Data { name: name, id: id }, *offset))
//!   }
//! }
//!
//! // In lieu of a complex byte buffer we hearken back to a simple &[u8]; the default source
//! // of TryFromCtx. However, any type that implements Pread to produce a &[u8] can now read
//! // `Data` thanks to it's implementation of TryFromCtx.
//! let bytes = b"\x01\x02\x03\x04\x00\x08UserName";
//! let data: Data = bytes.pread_with(0, Context(Endian::Big)).unwrap();
//!
//! assert_eq!(data.id, 0x01020304);
//! assert_eq!(data.name.to_string(), "UserName".to_string());
//! ```
//!
//! For further explanation of the traits and how to implement them manually refer to
//! [Pread](trait.Pread.html) and [TryFromCtx](ctx/trait.TryFromCtx.html).

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "derive")]
#[allow(unused_imports)]
pub use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[cfg(feature = "std")]
extern crate core;

pub mod ctx;
mod endian;
mod error;
mod greater;
mod leb128;
#[cfg(feature = "std")]
mod lesser;
mod pread;
mod pwrite;

pub use crate::endian::*;
pub use crate::error::*;
pub use crate::greater::*;
pub use crate::leb128::*;
#[cfg(feature = "std")]
pub use crate::lesser::*;
pub use crate::pread::*;
pub use crate::pwrite::*;

#[doc(hidden)]
pub mod export {
    pub use ::core::{mem, result};
}

#[allow(unused)]
macro_rules! doc_comment {
    ($x:expr) => {
        #[doc = $x]
        #[doc(hidden)]
        mod readme_tests {}
    };
}

#[cfg(feature = "derive")]
doc_comment!(include_str!("../README.md"));

#[cfg(test)]
mod tests {
    use super::LE;

    #[test]
    fn test_measure_with_bytes() {
        use super::ctx::MeasureWith;
        let bytes: [u8; 4] = [0xef, 0xbe, 0xad, 0xde];
        assert_eq!(bytes.measure_with(&()), 4);
    }

    #[test]
    fn test_measurable() {
        use super::ctx::SizeWith;
        assert_eq!(8, u64::size_with(&LE));
    }

    //////////////////////////////////////////////////////////////
    // begin pread_with
    //////////////////////////////////////////////////////////////

    macro_rules! pwrite_test {
        ($write:ident, $read:ident, $deadbeef:expr) => {
            #[test]
            fn $write() {
                use super::{Pread, Pwrite, BE};
                let mut bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
                let b = &mut bytes[..];
                b.pwrite_with::<$read>($deadbeef, 0, LE).unwrap();
                assert_eq!(b.pread_with::<$read>(0, LE).unwrap(), $deadbeef);
                b.pwrite_with::<$read>($deadbeef, 0, BE).unwrap();
                assert_eq!(b.pread_with::<$read>(0, BE).unwrap(), $deadbeef);
            }
        };
    }

    pwrite_test!(pwrite_and_pread_roundtrip_u16, u16, 0xbeef);
    pwrite_test!(pwrite_and_pread_roundtrip_i16, i16, 0x7eef);
    pwrite_test!(pwrite_and_pread_roundtrip_u32, u32, 0xbeefbeef);
    pwrite_test!(pwrite_and_pread_roundtrip_i32, i32, 0x7eefbeef);
    pwrite_test!(pwrite_and_pread_roundtrip_u64, u64, 0xbeefbeef7eef7eef);
    pwrite_test!(pwrite_and_pread_roundtrip_i64, i64, 0x7eefbeef7eef7eef);

    #[test]
    fn pread_with_be() {
        use super::Pread;
        let bytes: [u8; 2] = [0x7e, 0xef];
        let b = &bytes[..];
        let byte: u16 = b.pread_with(0, super::BE).unwrap();
        assert_eq!(0x7eef, byte);
        let bytes: [u8; 2] = [0xde, 0xad];
        let dead: u16 = bytes.pread_with(0, super::BE).unwrap();
        assert_eq!(0xdead, dead);
    }

    #[test]
    fn pread() {
        use super::Pread;
        let bytes: [u8; 2] = [0x7e, 0xef];
        let b = &bytes[..];
        let byte: u16 = b.pread(0).unwrap();
        #[cfg(target_endian = "little")]
        assert_eq!(0xef7e, byte);
        #[cfg(target_endian = "big")]
        assert_eq!(0x7eef, byte);
    }

    #[test]
    fn pread_slice() {
        use super::ctx::StrCtx;
        use super::Pread;
        let bytes: [u8; 2] = [0x7e, 0xef];
        let b = &bytes[..];
        let iserr: Result<&str, _> = b.pread_with(0, StrCtx::Length(3));
        assert!(iserr.is_err());
        // let bytes2: &[u8]  = b.pread_with(0, 2).unwrap();
        // assert_eq!(bytes2.len(), bytes[..].len());
        // for i in 0..bytes2.len() {
        //     assert_eq!(bytes2[i], bytes[i])
        // }
    }

    #[test]
    fn pread_str() {
        use super::ctx::*;
        use super::Pread;
        let bytes: [u8; 2] = [0x2e, 0x0];
        let b = &bytes[..];
        let s: &str = b.pread(0).unwrap();
        #[cfg(feature = "std")]
        println!("str: {s}");
        assert_eq!(s.len(), bytes[..].len() - 1);
        let bytes: &[u8] = b"hello, world!\0some_other_things";
        let hello_world: &str = bytes.pread_with(0, StrCtx::Delimiter(NULL)).unwrap();
        #[cfg(feature = "std")]
        println!("{hello_world:?}");
        assert_eq!(hello_world.len(), 13);
        let hello: &str = bytes.pread_with(0, StrCtx::Delimiter(SPACE)).unwrap();
        #[cfg(feature = "std")]
        println!("{hello:?}");
        assert_eq!(hello.len(), 6);
        // this could result in underflow so we just try it
        let _error = bytes.pread_with::<&str>(6, StrCtx::Delimiter(SPACE));
        let error = bytes.pread_with::<&str>(7, StrCtx::Delimiter(SPACE));
        #[cfg(feature = "std")]
        println!("{error:?}");
        assert!(error.is_ok());
    }

    /// In this test, we are testing preading
    /// at length boundaries.
    /// In the past, this test was supposed to test failures for `hello_world`.
    /// Since PR#94, this test is unwrapping as we exploit
    /// the fact that if you do &x[x.len()..] you get an empty slice.
    #[test]
    fn pread_str_weird() {
        use super::ctx::*;
        use super::Pread;
        let bytes: &[u8] = b"";
        let hello_world = bytes.pread_with::<&str>(0, StrCtx::Delimiter(NULL));
        #[cfg(feature = "std")]
        println!("1 {hello_world:?}");
        assert!(hello_world.unwrap().is_empty());
        let error = bytes.pread_with::<&str>(7, StrCtx::Delimiter(SPACE));
        #[cfg(feature = "std")]
        println!("2 {error:?}");
        assert!(error.is_err());
        let bytes: &[u8] = b"\0";
        let null = bytes.pread::<&str>(0).unwrap();
        #[cfg(feature = "std")]
        println!("3 {null:?}");
        assert_eq!(null.len(), 0);
    }

    #[test]
    fn pwrite_str_and_bytes() {
        use super::ctx::*;
        use super::{Pread, Pwrite};
        let astring: &str = "lol hello_world lal\0ala imabytes";
        let mut buffer = [0u8; 33];
        buffer.pwrite(astring, 0).unwrap();
        {
            let hello_world = buffer
                .pread_with::<&str>(4, StrCtx::Delimiter(SPACE))
                .unwrap();
            assert_eq!(hello_world, "hello_world");
        }
        let bytes: &[u8] = b"more\0bytes";
        buffer.pwrite(bytes, 0).unwrap();
        let more = bytes
            .pread_with::<&str>(0, StrCtx::Delimiter(NULL))
            .unwrap();
        assert_eq!(more, "more");
        let bytes = bytes
            .pread_with::<&str>(more.len() + 1, StrCtx::Delimiter(NULL))
            .unwrap();
        assert_eq!(bytes, "bytes");
    }

    use core::fmt::{self, Display};

    #[derive(Debug)]
    pub struct ExternalError {}

    impl Display for ExternalError {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "ExternalError")
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for ExternalError {
        fn description(&self) -> &str {
            "ExternalError"
        }
        fn cause(&self) -> Option<&dyn std::error::Error> {
            None
        }
    }

    impl From<super::Error> for ExternalError {
        fn from(err: super::Error) -> Self {
            //use super::Error::*;
            match err {
                _ => ExternalError {},
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Foo(u16);

    impl super::ctx::TryIntoCtx<super::Endian> for Foo {
        type Error = ExternalError;
        fn try_into_ctx(self, this: &mut [u8], le: super::Endian) -> Result<usize, Self::Error> {
            use super::Pwrite;
            if this.len() < 2 {
                return Err(ExternalError {});
            }
            this.pwrite_with(self.0, 0, le)?;
            Ok(2)
        }
    }

    impl<'a> super::ctx::TryFromCtx<'a, super::Endian> for Foo {
        type Error = ExternalError;
        fn try_from_ctx(this: &'a [u8], le: super::Endian) -> Result<(Self, usize), Self::Error> {
            use super::Pread;
            if this.len() > 2 {
                return Err(ExternalError {});
            }
            let n = this.pread_with(0, le)?;
            Ok((Foo(n), 2))
        }
    }

    #[test]
    fn pread_with_iter_bytes() {
        use super::Pread;
        let mut bytes_to: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let bytes_from: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let bytes_to = &mut bytes_to[..];
        let bytes_from = &bytes_from[..];
        for i in 0..bytes_from.len() {
            bytes_to[i] = bytes_from.pread(i).unwrap();
        }
        assert_eq!(bytes_to, bytes_from);
    }

    //////////////////////////////////////////////////////////////
    // end pread_with
    //////////////////////////////////////////////////////////////

    //////////////////////////////////////////////////////////////
    // begin gread_with
    //////////////////////////////////////////////////////////////
    macro_rules! g_test {
        ($read:ident, $deadbeef:expr, $typ:ty) => {
            #[test]
            fn $read() {
                use super::Pread;
                let bytes: [u8; 8] = [0xf, 0xe, 0xe, 0xb, 0xd, 0xa, 0xe, 0xd];
                let mut offset = 0;
                let deadbeef: $typ = bytes.gread_with(&mut offset, LE).unwrap();
                assert_eq!(deadbeef, $deadbeef as $typ);
                assert_eq!(offset, ::core::mem::size_of::<$typ>());
            }
        };
    }

    g_test!(simple_gread_u16, 0xe0f, u16);
    g_test!(simple_gread_u32, 0xb0e0e0f, u32);
    g_test!(simple_gread_u64, 0xd0e0a0d0b0e0e0f, u64);
    g_test!(simple_gread_i64, 940700423303335439, i64);

    macro_rules! simple_float_test {
        ($read:ident, $deadbeef:expr, $typ:ty) => {
            #[test]
            fn $read() {
                use super::Pread;
                let bytes: [u8; 8] = [0u8, 0, 0, 0, 0, 0, 224, 63];
                let mut offset = 0;
                let deadbeef: $typ = bytes.gread_with(&mut offset, LE).unwrap();
                assert_eq!(deadbeef, $deadbeef as $typ);
                assert_eq!(offset, ::core::mem::size_of::<$typ>());
            }
        };
    }

    simple_float_test!(gread_f32, 0.0, f32);
    simple_float_test!(gread_f64, 0.5, f64);

    macro_rules! g_read_write_test {
        ($read:ident, $val:expr, $typ:ty) => {
            #[test]
            fn $read() {
                use super::{Pread, Pwrite, BE, LE};
                let mut buffer = [0u8; 16];
                let offset = &mut 0;
                buffer.gwrite_with($val.clone(), offset, LE).unwrap();
                let o2 = &mut 0;
                let val: $typ = buffer.gread_with(o2, LE).unwrap();
                assert_eq!(val, $val);
                assert_eq!(*offset, ::core::mem::size_of::<$typ>());
                assert_eq!(*o2, ::core::mem::size_of::<$typ>());
                assert_eq!(*o2, *offset);
                buffer.gwrite_with($val.clone(), offset, BE).unwrap();
                let val: $typ = buffer.gread_with(o2, BE).unwrap();
                assert_eq!(val, $val);
            }
        };
    }

    g_read_write_test!(gread_gwrite_f64_1, 0.25f64, f64);
    g_read_write_test!(gread_gwrite_f64_2, 0.5f64, f64);
    g_read_write_test!(gread_gwrite_f64_3, 0.064, f64);

    g_read_write_test!(gread_gwrite_f32_1, 0.25f32, f32);
    g_read_write_test!(gread_gwrite_f32_2, 0.5f32, f32);
    g_read_write_test!(gread_gwrite_f32_3, 0.0f32, f32);

    g_read_write_test!(gread_gwrite_i64_1, 0i64, i64);
    g_read_write_test!(gread_gwrite_i64_2, -1213213211111i64, i64);
    g_read_write_test!(gread_gwrite_i64_3, -3000i64, i64);

    g_read_write_test!(gread_gwrite_i32_1, 0i32, i32);
    g_read_write_test!(gread_gwrite_i32_2, -1213213232, i32);
    g_read_write_test!(gread_gwrite_i32_3, -3000i32, i32);

    // useful for ferreting out problems with impls
    #[test]
    fn gread_with_iter_bytes() {
        use super::Pread;
        let mut bytes_to: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let bytes_from: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let bytes_to = &mut bytes_to[..];
        let bytes_from = &bytes_from[..];
        let mut offset = &mut 0;
        for i in 0..bytes_from.len() {
            bytes_to[i] = bytes_from.gread(&mut offset).unwrap();
        }
        assert_eq!(bytes_to, bytes_from);
        assert_eq!(*offset, bytes_to.len());
    }

    #[test]
    fn gread_inout() {
        use super::Pread;
        let mut bytes_to: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        let bytes_from: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let bytes = &bytes_from[..];
        let offset = &mut 0;
        bytes.gread_inout(offset, &mut bytes_to[..]).unwrap();
        assert_eq!(bytes_to, bytes_from);
        assert_eq!(*offset, bytes_to.len());
    }

    #[test]
    fn gread_with_byte() {
        use super::Pread;
        let bytes: [u8; 1] = [0x7f];
        let b = &bytes[..];
        let offset = &mut 0;
        let byte: u8 = b.gread(offset).unwrap();
        assert_eq!(0x7f, byte);
        assert_eq!(*offset, 1);
    }

    #[test]
    fn gread_slice() {
        use super::ctx::StrCtx;
        use super::Pread;
        let bytes: [u8; 2] = [0x7e, 0xef];
        let b = &bytes[..];
        let offset = &mut 0;
        let res = b.gread_with::<&str>(offset, StrCtx::Length(3));
        assert!(res.is_err());
        *offset = 0;
        let astring: [u8; 3] = [0x45, 0x42, 0x44];
        let string = astring.gread_with::<&str>(offset, StrCtx::Length(2));
        match &string {
            Ok(_) => {}
            Err(_err) => {
                #[cfg(feature = "std")]
                println!("{_err}");
                panic!();
            }
        }
        assert_eq!(string.unwrap(), "EB");
        *offset = 0;
        let bytes2: &[u8] = b.gread_with(offset, 2).unwrap();
        assert_eq!(*offset, 2);
        assert_eq!(bytes2.len(), bytes[..].len());
        for i in 0..bytes2.len() {
            assert_eq!(bytes2[i], bytes[i])
        }
    }

    /////////////////////////////////////////////////////////////////
    // end gread_with
    /////////////////////////////////////////////////////////////////
}
