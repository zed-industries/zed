#![warn(
    missing_docs, unused,
    trivial_numeric_casts,
    future_incompatible,
    rust_2018_compatibility,
    rust_2018_idioms,
    clippy::all
)]

#![doc(html_root_url = "https://docs.rs/lebe/0.5.0")]

//! Dead simple endianness conversions.
//! The following operations are implemented on
//! `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `f32`, `f64`:
//!
//!
//! ### Read Numbers
//! ```rust
//! use lebe::prelude::*;
//! let mut reader: &[u8] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
//!
//! let number : u64 = reader.read_from_little_endian()?;
//! let number = u64::read_from_big_endian(&mut reader)?;
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ### Read Slices
//! ```rust
//! use std::io::Read;
//! use lebe::prelude::*;
//! let mut reader: &[u8] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
//!
//! let mut numbers: &mut [u64] = &mut [0, 0];
//! reader.read_from_little_endian_into(numbers)?;
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ### Write Numbers
//! ```rust
//! use std::io::Read;
//! use lebe::prelude::*;
//! let mut writer: Vec<u8> = Vec::new();
//!
//! let number: u64 = 1237691;
//! writer.write_as_big_endian(&number)?;
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ### Write Slices
//! ```rust
//! use std::io::Write;
//! use lebe::prelude::*;
//! let mut writer: Vec<u8> = Vec::new();
//!
//! let numbers: &[u64] = &[1_u64, 234545_u64];
//! writer.write_as_little_endian(numbers)?;
//! # Ok::<(), std::io::Error>(())
//! ```
//!


/// Exports some of the most common types.
pub mod prelude {
    pub use super::Endian;
    pub use super::io::{ WriteEndian, ReadEndian, ReadPrimitive };
}

/// Represents values that can swap their bytes to reverse their endianness.
///
/// Supports converting values in-place using [`swap_bytes`] or [`convert_current_to_little_endian`]:
/// Supports converting while transferring ownership using
/// [`from_little_endian_into_current`] or [`from_current_into_little_endian`].
///
///
/// For the types `u8`, `i8`, `&[u8]` and `&[i8]`, this trait will never transform any data,
/// as they are just implemented for completeness.
pub trait Endian {

    /// Swaps all bytes in this value, inverting its endianness.
    fn swap_bytes(&mut self);

    /// On a little endian machine, this does nothing.
    /// On a big endian machine, the bytes of this value are reversed.
    #[inline] fn convert_current_to_little_endian(&mut self) {
        #[cfg(target_endian = "big")] {
            self.swap_bytes();
        }
    }

    /// On a big endian machine, this does nothing.
    /// On a little endian machine, the bytes of this value are reversed.
    #[inline] fn convert_current_to_big_endian(&mut self) {
        #[cfg(target_endian = "little")] {
            self.swap_bytes();
        }
    }

    /// On a little endian machine, this does nothing.
    /// On a big endian machine, the bytes of this value are reversed.
    #[inline] fn convert_little_endian_to_current(&mut self) {
        #[cfg(target_endian = "big")] {
            self.swap_bytes();
        }
    }

    /// On a big endian machine, this does nothing.
    /// On a little endian machine, the bytes of this value are reversed.
    #[inline] fn convert_big_endian_to_current(&mut self) {
        #[cfg(target_endian = "little")] {
            self.swap_bytes();
        }
    }

    /// On a little endian machine, this does nothing.
    /// On a big endian machine, the bytes of this value are reversed.
    #[inline] fn from_current_into_little_endian(mut self) -> Self where Self: Sized {
        self.convert_current_to_little_endian();
        self
    }

    /// On a big endian machine, this does nothing.
    /// On a little endian machine, the bytes of this value are reversed.
    #[inline] fn from_current_into_big_endian(mut self) -> Self where Self: Sized {
        self.convert_current_to_big_endian();
        self
    }

    /// On a little endian machine, this does nothing.
    /// On a big endian machine, the bytes of this value are reversed.
    #[inline] fn from_little_endian_into_current(mut self) -> Self where Self: Sized {
        self.convert_little_endian_to_current();
        self
    }

    /// On a big endian machine, this does nothing.
    /// On a little endian machine, the bytes of this value are reversed.
    #[inline] fn from_big_endian_into_current(mut self) -> Self where Self: Sized {
        self.convert_big_endian_to_current();
        self
    }
}


// call a macro for each argument
macro_rules! call_single_arg_macro_for_each {
    ($macro: ident, $( $arguments: ident ),* ) => {
        $( $macro! { $arguments }  )*
    };
}

// implement this interface for primitive signed and unsigned integers
macro_rules! implement_simple_primitive_endian {
    ($type: ident) => {
        impl Endian for $type {
            fn swap_bytes(&mut self) {
                *self = $type::swap_bytes(*self);
            }
        }
    };
}


call_single_arg_macro_for_each! {
    implement_simple_primitive_endian,
    u16, u32, u64, u128, i16, i32, i64, i128
}

// no-op implementations
impl Endian for u8 { fn swap_bytes(&mut self) {} }
impl Endian for i8 { fn swap_bytes(&mut self) {} }
impl Endian for [u8] { fn swap_bytes(&mut self) {} }
impl Endian for [i8] { fn swap_bytes(&mut self) {} }

// implement this interface for primitive floats, because they do not have a `swap_bytes()` in `std`
macro_rules! implement_float_primitive_by_bits {
    ($type: ident) => {
        impl Endian for $type {
            fn swap_bytes(&mut self) {
                *self = Self::from_bits(self.to_bits().swap_bytes());
            }
        }
    };
}


implement_float_primitive_by_bits!(f32);
implement_float_primitive_by_bits!(f64);

macro_rules! implement_slice_by_element {
    ($type: ident) => {
        impl Endian for [$type] {
            fn swap_bytes(&mut self) {
                for number in self.iter_mut() { // TODO SIMD?
                    number.swap_bytes();
                }
            }
        }
    };
}

call_single_arg_macro_for_each! {
    implement_slice_by_element,
    u16, u32, u64, u128,
    i16, i32, i64, i128,
    f64, f32
}

/// Easily write primitives and slices of primitives to
/// binary `std::io::Write` streams and easily read from binary `std::io::Read` streams.
///
/// Also contains the unsafe `bytes` module for reinterpreting values as byte slices and vice versa.
pub mod io {
    use super::Endian;
    use std::io::{Read, Write, Result};

    /// Reinterpret values as byte slices and byte slices as values unsafely.
    pub mod bytes {
        use std::io::{Read, Write, Result};

        /// View this slice of values as a slice of bytes.
        #[inline]
        pub unsafe fn slice_as_bytes<T>(value: &[T]) -> &[u8] {
            std::slice::from_raw_parts(
                value.as_ptr() as *const u8,
                value.len() * std::mem::size_of::<T>()
            )
        }

        /// View this slice of values as a mutable slice of bytes.
        #[inline]
        pub unsafe fn slice_as_bytes_mut<T>(value: &mut [T]) -> &mut [u8] {
            std::slice::from_raw_parts_mut(
                value.as_mut_ptr() as *mut u8,
                value.len() * std::mem::size_of::<T>()
            )
        }

        /// View this reference as a slice of bytes.
        #[inline]
        pub unsafe fn value_as_bytes<T: Sized>(value: &T) -> &[u8] {
            std::slice::from_raw_parts(
                value as *const T as *const u8,
                std::mem::size_of::<T>()
            )
        }

        /// View this reference as a mutable slice of bytes.
        #[inline]
        pub unsafe fn value_as_bytes_mut<T: Sized>(value: &mut T) ->&mut [u8] {
            std::slice::from_raw_parts_mut(
                value as *mut T as *mut u8,
                std::mem::size_of::<T>()
            )
        }

        /// View this slice as a mutable slice of bytes and write it.
        #[inline]
        pub unsafe fn write_slice<T>(write: &mut impl Write, value: &[T]) -> Result<()> {
            write.write_all(slice_as_bytes(value))
        }

        /// Read a slice of bytes into the specified slice.
        #[inline]
        pub unsafe fn read_slice<T>(read: &mut impl Read, value: &mut [T]) -> Result<()> {
            read.read_exact(slice_as_bytes_mut(value))
        }

        /// View this reference as a mutable slice of bytes and write it.
        #[inline]
        pub unsafe fn write_value<T: Sized>(write: &mut impl Write, value: &T) -> Result<()> {
            write.write_all(value_as_bytes(value))
        }

        /// Read a slice of bytes into the specified reference.
        #[inline]
        pub unsafe fn read_value<T: Sized>(read: &mut impl Read, value: &mut T) -> Result<()> {
            read.read_exact(value_as_bytes_mut(value))
        }
    }

    /// A `std::io::Write` output stream which supports writing any primitive values as bytes.
    /// Will encode the values to be either little endian or big endian, as desired.
    ///
    /// This extension trait is implemented for all `Write` types.
    /// Add `use lebe::io::WriteEndian;` to your code
    /// to automatically unlock this functionality for all types that implement `Write`.
    pub trait WriteEndian<T: ?Sized> {

        /// Write the byte value of the specified reference, converting it to little endianness
        fn write_as_little_endian(&mut self, value: &T) -> Result<()>;

        /// Write the byte value of the specified reference, converting it to big endianness
        fn write_as_big_endian(&mut self, value: &T) -> Result<()>;

        /// Write the byte value of the specified reference, not converting it
        fn write_as_native_endian(&mut self, value: &T) -> Result<()> {
            #[cfg(target_endian = "little")] { self.write_as_little_endian(value) }
            #[cfg(target_endian = "big")] { self.write_as_big_endian(value) }
        }
    }

    /// A `std::io::Read` input stream which supports reading any primitive values from bytes.
    /// Will decode the values from either little endian or big endian, as desired.
    ///
    /// This extension trait is implemented for all `Read` types.
    /// Add `use lebe::io::ReadEndian;` to your code
    /// to automatically unlock this functionality for all types that implement `Read`.
    pub trait ReadEndian<T: ?Sized> {

        /// Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.
        fn read_from_little_endian_into(&mut self, value: &mut T) -> Result<()>;

        /// Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.
        fn read_from_big_endian_into(&mut self, value: &mut T) -> Result<()>;

        /// Read into the supplied reference. Acts the same as `std::io::Read::read_exact`.
        fn read_from_native_endian_into(&mut self, value: &mut T) -> Result<()> {
            #[cfg(target_endian = "little")] { self.read_from_little_endian_into(value) }
            #[cfg(target_endian = "big")] { self.read_from_big_endian_into(value) }
        }

        /// Read the byte value of the inferred type
        #[inline]
        fn read_from_little_endian(&mut self) -> Result<T> where T: Sized + Default {
            let mut value = T::default();
            self.read_from_little_endian_into(&mut value)?;
            Ok(value)
        }

        /// Read the byte value of the inferred type
        #[inline]
        fn read_from_big_endian(&mut self) -> Result<T> where T: Sized + Default {
            let mut value = T::default();
            self.read_from_big_endian_into(&mut value)?;
            Ok(value)
        }

        /// Read the byte value of the inferred type
        #[inline]
        fn read_from_native_endian(&mut self) -> Result<T> where T: Sized + Default {
            #[cfg(target_endian = "little")] { self.read_from_little_endian() }
            #[cfg(target_endian = "big")] { self.read_from_big_endian() }
        }
    }

    // implement primitive for all types that are implemented by `Read`
    impl<R: Read + ReadEndian<P>, P: Default> ReadPrimitive<R> for P {}


    /// Offers a prettier versions of reading a primitive number.
    ///
    /// The default way of reading a value is:
    /// ```rust
    /// # use std::io::Read;
    /// # use lebe::prelude::*;
    /// # let mut reader : &[u8] = &[2, 1];
    ///
    /// let number: u16 = reader.read_from_little_endian()?;
    /// println!("{}", number);
    /// # Ok::<(), std::io::Error>(())
    ///
    /// ```
    ///
    /// This trait enables you to use expressions:
    /// ```rust
    /// # use std::io::Read;
    /// # use lebe::prelude::*;
    /// # let mut reader : &[u8] = &[2, 1];
    ///
    /// println!("{}", u16::read_from_little_endian(&mut reader)?);
    /// # Ok::<(), std::io::Error>(())
    /// ```
    /// .
    ///
    pub trait ReadPrimitive<R: Read + ReadEndian<Self>> : Sized + Default {
        /// Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.
        fn read_from_little_endian(read: &mut R) -> Result<Self> {
            read.read_from_little_endian()
        }

        /// Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.
        fn read_from_big_endian(read: &mut R) -> Result<Self> {
            read.read_from_big_endian()
        }

        /// Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.
        fn read_from_native_endian(read: &mut R) -> Result<Self> {
            read.read_from_native_endian()
        }
    }

    macro_rules! implement_simple_primitive_write {
        ($type: ident) => {
            impl<W: Write> WriteEndian<$type> for W {
                fn write_as_little_endian(&mut self, value: &$type) -> Result<()> {
                    unsafe { bytes::write_value(self, &value.from_current_into_little_endian()) }
                }

                fn write_as_big_endian(&mut self, value: &$type) -> Result<()> {
                    unsafe { bytes::write_value(self, &value.from_current_into_big_endian()) }
                }
            }

            impl<R: Read> ReadEndian<$type> for R {
                #[inline]
                fn read_from_little_endian_into(&mut self, value: &mut $type) -> Result<()> {
                    unsafe { bytes::read_value(self, value)?; }
                    value.convert_little_endian_to_current();
                    Ok(())
                }

                #[inline]
                fn read_from_big_endian_into(&mut self, value: &mut $type) -> Result<()> {
                    unsafe { bytes::read_value(self, value)?; }
                    value.convert_big_endian_to_current();
                    Ok(())
                }
            }
        };
    }

    call_single_arg_macro_for_each! {
        implement_simple_primitive_write,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    }


    macro_rules! implement_slice_io {
        ($type: ident) => {
            impl<W: Write> WriteEndian<[$type]> for W {
                fn write_as_little_endian(&mut self, value: &[$type]) -> Result<()> {
                    #[cfg(target_endian = "big")] {
                        for number in value { // TODO SIMD!
                            self.write_as_little_endian(number)?;
                        }
                    }

                    // else write whole slice
                    #[cfg(target_endian = "little")]
                    unsafe { bytes::write_slice(self, value)?; }

                    Ok(())
                }

                fn write_as_big_endian(&mut self, value: &[$type]) -> Result<()> {
                    #[cfg(target_endian = "little")] {
                        for number in value { // TODO SIMD!
                            self.write_as_big_endian(number)?;
                        }
                    }

                    // else write whole slice
                    #[cfg(target_endian = "big")]
                    unsafe { bytes::write_slice(self, value)?; }

                    Ok(())
                }
            }

            impl<R: Read> ReadEndian<[$type]> for R {
                fn read_from_little_endian_into(&mut self, value: &mut [$type]) -> Result<()> {
                    unsafe { bytes::read_slice(self, value)? };
                    value.convert_little_endian_to_current();
                    Ok(())
                }

                fn read_from_big_endian_into(&mut self, value: &mut [$type]) -> Result<()> {
                    unsafe { bytes::read_slice(self, value)? };
                    value.convert_big_endian_to_current();
                    Ok(())
                }
            }
        };
    }

    call_single_arg_macro_for_each! {
        implement_slice_io,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f64, f32
    }



    // TODO: SIMD
    /*impl<R: Read> ReadEndian<[f32]> for R {
        fn read_from_little_endian_into(&mut self, value: &mut [f32]) -> Result<()> {
            unsafe { bytes::read_slice(self, value)? };
            value.convert_little_endian_to_current();
            Ok(())
        }

        fn read_from_big_endian_into(&mut self, value: &mut [f32]) -> Result<()> {
            unsafe { bytes::read_slice(self, value)? };
            value.convert_big_endian_to_current();
            Ok(())
        }
    }

    impl<W: Write> WriteEndian<[f32]> for W {
        fn write_as_big_endian(&mut self, value: &[f32]) -> Result<()> {
            if cfg!(target_endian = "little") {

                // FIX ME this SIMD optimization makes no difference ... why? like, ZERO difference, not even worse
//                #[cfg(feature = "simd")]
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                unsafe {
                    if is_x86_feature_detected!("avx2") {
                        write_bytes_avx(self, value);
                        return Ok(());
                    }
                }

                // otherwise (no avx2 available)
//                for number in value {
//                    self.write_as_little_endian(number);
//                }
//
//                return Ok(());
                unimplemented!();

                #[target_feature(enable = "avx2")]
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                unsafe fn write_bytes_avx(write: &mut impl Write, slice: &[f32]) -> Result<()> {
                    #[cfg(target_arch = "x86")] use std::arch::x86 as mm;
                    #[cfg(target_arch = "x86_64")] use std::arch::x86_64 as mm;

                    let bytes: &[u8] = crate::io::bytes::slice_as_bytes(slice);
                    let mut chunks = bytes.chunks_exact(32);

                    let indices = mm::_mm256_set_epi8(
                        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,
                        0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
//                        3,2,1,0, 7,6,5,4, 11,10,9,8, 15,14,13,12,
//                        3,2,1,0, 7,6,5,4, 11,10,9,8, 15,14,13,12
                    );

                    for chunk in &mut chunks {
                        let data = mm::_mm256_loadu_si256(chunk.as_ptr() as _);
                        let result = mm::_mm256_shuffle_epi8(data, indices);
                        let mut out = [0_u8; 32];
                        mm::_mm256_storeu_si256(out.as_mut_ptr() as _, result);
                        write.write_all(&out)?;
                    }

                    let remainder = chunks.remainder();

                    { // copy remainder into larger slice, with zeroes at the end
                        let mut last_chunk = [0_u8; 32];
                        last_chunk[0..remainder.len()].copy_from_slice(remainder);
                        let data = mm::_mm256_loadu_si256(last_chunk.as_ptr() as _);
                        let result = mm::_mm256_shuffle_epi8(data, indices);
                        mm::_mm256_storeu_si256(last_chunk.as_mut_ptr() as _, result);
                        write.write_all(&last_chunk[0..remainder.len()])?;
                    }

                    Ok(())
                }
            }

            else {
                unsafe { bytes::write_slice(self, value)?; }
                Ok(())
            }
        }

        fn write_as_little_endian(&mut self, value: &[f32]) -> Result<()> {
            for number in value {
                self.write_as_little_endian(number)?;
            }

            Ok(())
        }
    }*/
}

