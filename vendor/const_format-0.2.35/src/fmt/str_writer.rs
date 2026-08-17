use super::{Error, Formatter, FormattingFlags, StrWriterMut, Utf8Encoding};

use core::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

/// A wrapper over an array usable to build up a `&str` at compile-time.
///
/// # Calling methods
///
/// [Certain `StrWriter` methods](#certain-methods) require
/// a `StrWriter<[u8]>` to be called,
/// and since constructing `StrWriter` from an array produces a `StrWriter<[u8; N]>`,
/// it must be cast to call them.
///
/// `StrWriter`'s type parameter defaults to `[u8]`,
/// so every instance of a `StrWriter` as a *type* is a `StrWriter<[u8]>`.
///
/// Example of casting it:
///
/// ```rust
/// # use const_format::StrWriter;
/// let writer: &mut StrWriter<[u8; 8]> = &mut StrWriter::new([0; 8]);
///
/// // Casts `&StrWriter<[u8; 8]>` to `&StrWriter`
/// writer.unsize();
///
/// // Casts `&StrWriter<[u8; 8]>` to `&StrWriter`
/// writer.r();
///
/// // Coerces the `&mut StrWriter<[u8; 8]>` to `&mut StrWriter`
/// let _writer: &mut StrWriter = writer;
///
/// // Casts `&mut StrWriter<[u8; 8]>` to `StrWriterMut<'_>`,
/// // which defines methods for mutating `StrWriter`
/// let _writer = writer.as_mut();
///
/// # drop(writer);
/// ```
///
/// # StrWriterMut
///
/// `StrWriter` can be borrowed into a [`StrWriterMut`],
/// which provides methods for writing a formatted string.
///
/// Example:
///
/// ```rust
/// use const_format::StrWriter;
///
/// let mut buffer: &mut StrWriter = &mut StrWriter::new([0; 100]);
///
/// let mut writer = buffer.as_mut();
/// writer.write_str("Your password is: ");
/// writer.write_str_debug("PASSWORD");
///
/// assert_eq!(writer.as_str(), r#"Your password is: "PASSWORD""#);
///
/// ```
///
/// # Examples
///
/// ### Formatting into associated constant
///
/// This example shows how you can construct a formatted `&'static str` from associated constants.
///
/// ```rust
///
/// use const_format::{StrWriter, writec, unwrap};
///
/// trait Num {
///     const V: u32;
/// }
///
/// struct Two;
///
/// impl Num for Two {
///     const V: u32 = 2;
/// }
///
/// struct Three;
///
/// impl Num for Three {
///     const V: u32 = 3;
/// }
///
/// struct Mul<L, R>(L, R);
///
/// const fn compute_str(l: u32, r: u32) -> StrWriter<[u8; 128]> {
///     let mut writer = StrWriter::new([0; 128]);
///     unwrap!(writec!(writer, "{} * {} == {}", l, r, l * r ));
///     writer
/// }
///
/// impl<L: Num, R: Num> Mul<L, R> {
///     const __STR: &'static StrWriter<[u8]> = &compute_str(L::V, R::V);
///     const STR: &'static str = Self::__STR.as_str_alt();
/// }
///
/// assert_eq!(Mul::<Two,Three>::STR, "2 * 3 == 6");
/// assert_eq!(Mul::<Three,Three>::STR, "3 * 3 == 9");
///
/// ```
///
/// [`StrWriterMut`]: ./struct.StrWriterMut.html
///
#[derive(Debug, Copy, Clone)]
pub struct StrWriter<A: ?Sized = [u8]> {
    pub(super) len: usize,
    pub(super) buffer: A,
}

impl<A> StrWriter<A> {
    /// Constructs a `StrWriter` from a `u8` array
    pub const fn new(array: A) -> Self {
        Self {
            len: 0,
            buffer: array,
        }
    }
}

impl<A: ?Sized> StrWriter<A> {
    /// Accesses the underlying buffer immutably.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::StrWriter;
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 7]);
    /// assert_eq!(buffer.buffer(), &[0; 7]);
    ///
    /// buffer.as_mut().write_str("foo")?;
    /// assert_eq!(buffer.buffer(), b"foo\0\0\0\0");
    ///
    /// buffer.as_mut().write_str("bar")?;
    /// assert_eq!(buffer.buffer(), b"foobar\0");
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline(always)]
    pub const fn buffer(&self) -> &A {
        &self.buffer
    }

    /// How long the string this wrote is.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::StrWriter;
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    /// assert_eq!(buffer.len(), 0);
    ///
    /// buffer.as_mut().write_str("foo")?;
    /// assert_eq!(buffer.len(), 3);
    ///
    /// buffer.as_mut().write_str("bar")?;
    /// assert_eq!(buffer.len(), 6);
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Checks whether the string this wrote is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::StrWriter;
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    /// assert!( buffer.is_empty() );
    ///
    /// buffer.as_mut().write_str("foo")?;
    /// assert!( !buffer.is_empty() );
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// <span id="certain-methods"></span>
impl StrWriter {
    /// Gets the maximum length for a string written into this.
    ///
    /// Trying to write more that the capacity causes an error,
    /// returning back an `Err(Error::NotEnoughSpace)`
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{Error, StrWriter};
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    /// assert_eq!(buffer.capacity(), 64);
    ///
    /// buffer.as_mut().write_ascii_repeated(b'A', 64)?;
    /// assert_eq!(buffer.capacity(), 64);
    ///
    /// assert_eq!(buffer.as_mut().write_str("-").unwrap_err(), Error::NotEnoughSpace);
    /// assert_eq!(buffer.capacity(), 64);
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Checks how many more bytes can be written.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{Error, StrWriter};
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    /// assert_eq!(buffer.remaining_capacity(), 64);
    ///
    /// buffer.as_mut().write_str("foo")?;
    /// assert_eq!(buffer.remaining_capacity(), 61);
    ///
    /// buffer.as_mut().write_ascii_repeated(b'a', 61)?;
    /// assert_eq!(buffer.remaining_capacity(), 0);
    ///
    /// assert_eq!(buffer.as_mut().write_str(" ").unwrap_err(), Error::NotEnoughSpace);
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline]
    pub const fn remaining_capacity(&self) -> usize {
        self.buffer.len() - self.len
    }

    /// Truncates this `StrWriter` to `length`.
    ///
    /// If `length` is greater than the current length, this does nothing.
    ///
    /// # Errors
    ///
    /// Returns an `Error::NotOnCharBoundary` if `length` is not on a char boundary.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{Error, StrWriter};
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    ///
    /// buffer.as_mut().write_str("foo bâr baz");
    /// assert_eq!(buffer.as_str(), "foo bâr baz");
    ///
    /// assert_eq!(buffer.truncate(6).unwrap_err(), Error::NotOnCharBoundary);
    ///
    /// buffer.truncate(3)?;
    /// assert_eq!(buffer.as_str(), "foo");
    ///
    /// buffer.as_mut().write_str("ooooooo");
    /// assert_eq!(buffer.as_str(), "fooooooooo");
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline]
    pub const fn truncate(&mut self, length: usize) -> Result<(), Error> {
        self.as_mut().truncate(length)
    }

    /// Truncates this `StrWriter` to length 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{Error, StrWriter};
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    ///
    /// buffer.as_mut().write_str("foo")?;
    /// assert_eq!(buffer.as_str(), "foo");
    ///
    /// buffer.clear();
    /// assert_eq!(buffer.as_str(), "");
    /// assert!(buffer.is_empty());
    ///
    /// buffer.as_mut().write_str("bar");
    /// assert_eq!(buffer.as_str(), "bar");
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline]
    pub const fn clear(&mut self) {
        self.len = 0;
    }

    /// Gets the written part of this `StrWriter` as a `&[u8]`
    ///
    /// The slice is guaranteed to be valid utf8, so this is mostly for convenience.
    ///
    /// ### Runtime
    ///
    /// If the "rust_1_64" feature is disabled,
    /// this takes time proportional to `self.capacity() - self.len()`.
    ///
    /// If the "rust_1_64" feature is enabled, it takes constant time to run.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{StrWriter, StrWriterMut};
    ///
    /// const fn slice() -> StrWriter<[u8; 64]> {
    ///     let mut buffer = StrWriter::new([0; 64]);
    ///     let mut writer = StrWriterMut::new(&mut buffer);
    ///     writer.write_str("Hello, World!");
    ///     buffer
    /// }
    ///
    /// const SLICE: &[u8] = {
    ///     const PROM: &'static StrWriter = &slice();
    ///     PROM.as_bytes_alt()
    /// };
    ///
    ///
    /// assert_eq!(SLICE, "Hello, World!".as_bytes());
    ///
    /// ```
    #[inline(always)]
    pub const fn as_bytes_alt(&self) -> &[u8] {
        crate::utils::slice_up_to_len_alt(&self.buffer, self.len)
    }

    /// Gets the written part of this `StrWriter` as a `&str`
    ///
    /// ### Runtime
    ///
    /// If the "rust_1_64" feature is disabled,
    /// this takes time proportional to `self.capacity() - self.len()`.
    ///
    /// If the "rust_1_64" feature is enabled, it takes constant time to run.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::StrWriter;
    /// use const_format::{unwrap, writec};
    ///
    ///
    /// const CAP: usize = 128;
    ///
    /// const __STR: &StrWriter = &{
    ///     let mut writer =  StrWriter::new([0; CAP]);
    ///
    ///     // Writing the array with debug formatting, and the integers with hexadecimal formatting.
    ///     unwrap!(writec!(writer, "{:X}", [3u32, 5, 8, 13, 21, 34]));
    ///
    ///     writer
    /// };
    ///
    /// const STR: &str = __STR.as_str_alt();
    ///
    /// fn main() {
    ///     assert_eq!(STR, "[3, 5, 8, D, 15, 22]");
    /// }
    /// ```
    #[inline(always)]
    pub const fn as_str_alt(&self) -> &str {
        // All the methods that modify the buffer must ensure utf8 validity,
        // only methods from this module need to ensure this.
        unsafe { core::str::from_utf8_unchecked(self.as_bytes_alt()) }
    }

    conditionally_const! {
        feature = "rust_1_64";
        /// Gets the written part of this `StrWriter` as a `&str`
        ///
        /// ### Constness
        ///
        /// This can be called in const contexts by enabling the "rust_1_64" feature.
        ///
        /// ### Alternative
        ///
        /// You can also use the [`as_str_alt`](Self::as_str_alt) method,
        /// which is always available,
        /// but takes linear time to run when the "rust_1_64" feature
        /// is disabled.
        ///
        #[inline(always)]
        pub fn as_str(&self) -> &str {
            // All the methods that modify the buffer must ensure utf8 validity,
            // only methods from this module need to ensure this.
            unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
        }

        /// Gets the written part of this `StrWriter` as a `&[u8]`
        ///
        /// The slice is guaranteed to be valid utf8, so this is mostly for convenience.
        ///
        /// ### Constness
        ///
        /// This can be called in const contexts by enabling the "rust_1_64" feature.
        ///
        /// # Example
        ///
        /// ```rust
        /// use const_format::StrWriter;
        ///
        /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
        ///
        /// buffer.as_mut().write_str("Hello, World!");
        ///
        /// assert_eq!(buffer.as_bytes(), "Hello, World!".as_bytes());
        ///
        /// ```
        #[inline(always)]
        pub fn as_bytes(&self) -> &[u8] {
            crate::utils::slice_up_to_len(&self.buffer, self.len)
        }
    }

    /// Borrows this `StrWriter<[u8]>` into a `StrWriterMut`,
    /// most useful for calling the `write_*` methods.
    ///
    /// ```rust
    /// use const_format::StrWriter;
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    ///
    /// buffer.as_mut().write_str_range("trust", 1..usize::MAX);
    ///
    /// assert_eq!(buffer.as_str(), "rust");
    ///
    /// ```
    #[inline(always)]
    pub const fn as_mut(&mut self) -> StrWriterMut<'_> {
        StrWriterMut {
            len: &mut self.len,
            buffer: &mut self.buffer,
            _encoding: PhantomData,
        }
    }

    /// Constructs a [`Formatter`] that writes into this `StrWriter`,
    /// which can be passed to debug and display formatting methods.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Error, Formatter, FormattingFlags, StrWriter, call_debug_fmt};
    ///
    /// use std::ops::Range;
    ///
    /// const fn range_debug_fmt(
    ///     slice: &[Range<usize>],
    ///     f: &mut Formatter<'_>
    /// ) -> Result<(), Error> {
    ///     // We need this macro to debug format arrays of non-primitive types
    ///     // Also, it implicitly returns a `const_format::Error` on error.
    ///     call_debug_fmt!(array, slice, f);
    ///     Ok(())
    /// }
    ///
    /// fn main() -> Result<(), Error> {
    ///     let buffer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    ///
    ///     range_debug_fmt(
    ///         &[0..14, 14..31, 31..48],
    ///         &mut buffer.make_formatter(FormattingFlags::new().set_binary())
    ///     )?;
    ///    
    ///     assert_eq!(buffer.as_str(), "[0..1110, 1110..11111, 11111..110000]");
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Formatter`]: ./struct.Formatter.html
    #[inline(always)]
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter::from_sw_mut(
            StrWriterMut::<Utf8Encoding> {
                len: &mut self.len,
                buffer: &mut self.buffer,
                _encoding: PhantomData,
            },
            flags,
        )
    }
}

impl<const N: usize> StrWriter<[u8; N]> {
    /// Casts a `&StrWriter<[u8; N]>` to a `&StrWriter<[u8]>`,
    /// for calling methods defined on `StrWriter<[u8]>` (most of them).
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::StrWriter;
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    ///
    /// buffer.as_mut().write_str("Hello,");
    /// buffer.as_mut().write_str(" world!");
    ///
    /// assert_eq!(buffer.r().as_str(), "Hello, world!");
    ///
    /// ```
    ///
    #[inline(always)]
    pub const fn r(&self) -> &StrWriter<[u8]> {
        self
    }
    /// Casts a `&StrWriter<[u8; N]>` to a `&StrWriter<[u8]>`,
    /// for calling methods defined on `StrWriter<[u8]>` (most of them).
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::StrWriter;
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    ///
    /// buffer.as_mut().write_str("Hello,");
    /// buffer.as_mut().write_str(" world!");
    ///
    /// assert_eq!(buffer.unsize().as_str(), "Hello, world!");
    ///
    /// ```
    ///
    #[inline(always)]
    pub const fn unsize(&self) -> &StrWriter<[u8]> {
        self
    }

    /// Borrows this `StrWriter<[u8; N]>` into a `StrWriterMut`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::StrWriter;
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    ///
    /// buffer.as_mut().write_str_range("trust", 1..usize::MAX);
    ///
    /// assert_eq!(buffer.r().as_str(), "rust");
    ///
    /// ```
    #[inline(always)]
    pub const fn as_mut(&mut self) -> StrWriterMut<'_> {
        StrWriterMut {
            len: &mut self.len,
            buffer: &mut self.buffer,
            _encoding: PhantomData,
        }
    }
}

impl<A: ?Sized> StrWriter<A> {
    /// For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }
}
