use crate::{
    formatting::{
        hex_as_ascii, ForEscaping, FormattingFlags, HexFormatting, NumberFormatting, FOR_ESCAPING,
    },
    utils::{min_usize, saturate_range, Constructor},
    wrapper_types::{AsciiStr, PWrapper},
};

use super::{Error, Formatter, StrWriter};

use core::{marker::PhantomData, ops::Range};

/// For writing a formatted string into a `[u8]`.
///
/// # Construction
///
/// This type can be constructed in these ways:
///
/// - From a `&mut StrWriter`, with the [`StrWriter::as_mut`] method.
///
/// - From a `&mut StrWriter<_>`, with the [`StrWriterMut::new`] constructor.
///
/// - From a pair of `usize` and `[u8]` mutable references,
/// with the [`from_custom_cleared`] constructor,
/// or the [`from_custom`] constructor.
///
/// # Relation to `Formatter`
///
/// This is the type that [`Formatter`] uses to write formatted text to a slice,
/// sharing all the `write_*` methods,
/// the difference is that this doesn't store `FormattingFlags`,
/// so you must pass them to the `write_*_debug` methods.
///
/// # Errors
///
/// Every single `write_*` method returns an [`Error::NotEnoughSpace`] if
/// there is not enough space to write the argument, leaving the string itself unmodified.
///
/// # Encoding type parameter
///
/// The `E` type parameter represents the encoding of the buffer that this
/// StrWriterMut writes into,
/// currently only [`Utf8Encoding`] and [`NoEncoding`] are supported.
///
/// # Example
///
/// This example demonstrates how you can write a formatted string to a `&mut [u8]`,
/// using a `StrWriterMut`.
///
/// ```rust
///
/// use const_format::{Error, StrWriterMut, try_, writec};
///
/// const fn format_number(number: u32,slice: &mut [u8]) -> Result<usize, Error> {
///     let mut len = 0;
///     let mut writer = StrWriterMut::from_custom_cleared(slice, &mut len);
///     
///     try_!(writec!(writer, "{0} in binary is {0:#b}", number));
///
///     Ok(len)
/// }
///
/// let mut slice = [0; 32];
///
/// let len = format_number(100, &mut slice)?;
///
/// assert_eq!(&slice[..len], "100 in binary is 0b1100100".as_bytes());
///
/// # Ok::<(), const_format::Error>(())
/// ```
///
/// [`from_custom_cleared`]: #method.from_custom_cleared
/// [`from_custom`]: #method.from_custom
///
/// [`Utf8Encoding`]: crate::fmt::Utf8Encoding
/// [`NoEncoding`]: crate::fmt::NoEncoding
/// [`Formatter`]: crate::fmt::Formatter
/// [`Error::NotEnoughSpace`]: crate::fmt::Error::NotEnoughSpace
///
pub struct StrWriterMut<'w, E = Utf8Encoding> {
    pub(super) len: &'w mut usize,
    pub(super) buffer: &'w mut [u8],
    pub(super) _encoding: PhantomData<Constructor<E>>,
}

macro_rules! borrow_fields {
    ($self:ident, $len:ident, $buffer:ident) => {
        let $len = &mut *$self.len;
        let $buffer = &mut *$self.buffer;
    };
}

/// Marker type indicating that the [`StrWriterMut`] is valid utf8,
/// enabling the `as_str` method.
///
/// [`StrWriterMut`]: ./struct.StrWriterMut.html
pub enum Utf8Encoding {}

/// Marker type indicating that the [`StrWriterMut`] is arbitrary bytes,
/// disabling the `as_str` method.
///
/// [`StrWriterMut`]: ./struct.StrWriterMut.html
pub enum NoEncoding {}

impl<'w> StrWriterMut<'w, Utf8Encoding> {
    /// Constructs a `StrWriterMut` from a mutable reference to a `StrWriter`
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{StrWriter, StrWriterMut};
    ///
    /// let buffer: &mut StrWriter = &mut StrWriter::new([0; 128]);
    /// {
    ///     let mut writer = StrWriterMut::new(buffer);
    ///
    ///     let _ = writer.write_str("Number: ");
    ///     let _ = writer.write_u8_display(1);
    /// }
    /// assert_eq!(buffer.as_str(), "Number: 1");
    ///
    /// ```
    pub const fn new(writer: &'w mut StrWriter) -> Self {
        Self {
            len: &mut writer.len,
            buffer: &mut writer.buffer,
            _encoding: PhantomData,
        }
    }
}

impl<'w> StrWriterMut<'w, NoEncoding> {
    /// Construct a `StrWriterMut` from length and byte slice mutable references.
    ///
    /// If `length > buffer.len()` is passed, it's simply assigned the length of the buffer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::StrWriterMut;
    ///
    /// let mut len = 6;
    /// let mut buffer = *b"Hello,       ";
    ///
    /// let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);
    ///
    /// writer.write_str(" world!")?;
    ///
    /// assert_eq!(writer.as_bytes(), b"Hello, world!");
    /// assert_eq!(buffer, "Hello, world!".as_bytes());
    /// assert_eq!(len, "Hello, world!".len());
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    pub const fn from_custom(buffer: &'w mut [u8], length: &'w mut usize) -> Self {
        *length = min_usize(*length, buffer.len());

        Self {
            len: length,
            buffer,
            _encoding: PhantomData,
        }
    }
}

impl<'w> StrWriterMut<'w, Utf8Encoding> {
    /// Construct a `StrWriterMut` from length and byte slice mutable references,
    /// truncating the length to `0`.
    ///
    /// Using this instead of [`from_custom`](StrWriterMut::from_custom) allows
    /// safely casting this to a `&str` with [`as_str_alt`]/[`as_str`]
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::StrWriterMut;
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 13];
    ///
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// writer.write_str("Hello, world!")?;
    ///
    /// assert_eq!(writer.as_str(), "Hello, world!");
    /// assert_eq!(buffer, "Hello, world!".as_bytes());
    /// assert_eq!(len, "Hello, world!".len());
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    ///
    /// [`as_str_alt`]: Self::as_str_alt
    /// [`as_str`]: Self::as_str
    ///
    pub const fn from_custom_cleared(buffer: &'w mut [u8], length: &'w mut usize) -> Self {
        *length = 0;

        Self {
            len: length,
            buffer,
            _encoding: PhantomData,
        }
    }
}

impl<'w, E> StrWriterMut<'w, E> {
    /// Accesses the underlying buffer immutably.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{StrWriter, StrWriterMut};
    ///
    /// let mut buffer = StrWriter::new([0; 7]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    /// assert_eq!(writer.buffer(), &[0; 7]);
    ///
    /// writer.write_str("foo")?;
    /// assert_eq!(writer.buffer(), b"foo\0\0\0\0");
    ///
    /// writer.write_str("bar")?;
    /// assert_eq!(writer.buffer(), b"foobar\0");
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline(always)]
    pub const fn buffer(&self) -> &[u8] {
        self.buffer
    }

    /// The byte length of the string this is writing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{StrWriter, StrWriterMut};
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    ///
    /// assert_eq!(writer.len(), 0);
    ///
    /// writer.write_str("foo")?;
    /// assert_eq!(writer.len(), 3);
    ///
    /// writer.write_str("bar")?;
    /// assert_eq!(writer.len(), 6);
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline(always)]
    pub const fn len(&self) -> usize {
        *self.len
    }

    /// Whether the string this is writing is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{StrWriter, StrWriterMut};
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    ///
    /// assert!( writer.is_empty() );
    ///
    /// writer.write_str("foo")?;
    /// assert!( !writer.is_empty() );
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        *self.len == 0
    }

    /// The maximum byte length of the formatted text for this `StrWriterMut`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{Error, StrWriter, StrWriterMut};
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    ///
    /// assert_eq!(writer.capacity(), 64);
    ///
    /// writer.write_ascii_repeated(b'A', 64)?;
    /// assert_eq!(writer.capacity(), 64);
    ///
    /// assert_eq!(writer.write_str("-").unwrap_err(), Error::NotEnoughSpace);
    /// assert_eq!(writer.capacity(), 64);
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
    /// use const_format::{Error, StrWriter, StrWriterMut};
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    ///
    /// assert_eq!(writer.remaining_capacity(), 64);
    ///
    /// writer.write_str("foo")?;
    /// assert_eq!(writer.remaining_capacity(), 61);
    ///
    /// writer.write_ascii_repeated(b'a', 61)?;
    /// assert_eq!(writer.remaining_capacity(), 0);
    ///
    /// assert_eq!(writer.write_str(" ").unwrap_err(), Error::NotEnoughSpace);
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline]
    pub const fn remaining_capacity(&self) -> usize {
        self.buffer.len() - *self.len
    }
}

impl<'w> StrWriterMut<'w, Utf8Encoding> {
    /// Truncates this `StrWriterMut` to `length`.
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
    /// use const_format::{Error, StrWriter, StrWriterMut};
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    ///
    /// writer.write_str("foo bâr baz");
    /// assert_eq!(writer.as_str(), "foo bâr baz");
    ///
    /// assert_eq!(writer.truncate(6).unwrap_err(), Error::NotOnCharBoundary);
    ///
    /// writer.truncate(3)?;
    /// assert_eq!(writer.as_str(), "foo");
    ///
    /// writer.write_str("ooooooo");
    /// assert_eq!(writer.as_str(), "fooooooooo");
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```
    #[inline]
    pub const fn truncate(&mut self, length: usize) -> Result<(), Error> {
        if length <= *self.len {
            if !is_valid_str_index(self.buffer, length) {
                return Err(Error::NotOnCharBoundary);
            }

            *self.len = length;
        }
        Ok(())
    }
}

impl<'w> StrWriterMut<'w, NoEncoding> {
    /// Truncates this `StrWriterMut<'w, NoEncoding>` to `length`.
    ///
    /// If `length` is greater than the current length, this does nothing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{Error, StrWriter, StrWriterMut};
    ///
    /// let mut buffer = [0; 32];
    /// let mut len = 0;
    /// let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);
    ///
    /// writer.write_str("foo bar baz");
    /// assert_eq!(writer.as_bytes(), b"foo bar baz");
    ///
    /// // Truncating to anything larger than the length is a no-op.
    /// writer.truncate(usize::MAX / 2);
    /// assert_eq!(writer.as_bytes(), b"foo bar baz");
    ///
    /// writer.truncate(3);
    /// assert_eq!(writer.as_bytes(), b"foo");
    ///
    /// writer.write_str("ooooooo");
    /// assert_eq!(writer.as_bytes(), b"fooooooooo");
    ///
    /// ```
    #[inline]
    pub const fn truncate(&mut self, length: usize) {
        if length < *self.len {
            *self.len = length;
        }
    }
}

impl<'w, E> StrWriterMut<'w, E> {
    /// Truncates this `StrWriterMut` to length 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{Error, StrWriter, StrWriterMut};
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    ///
    /// writer.write_str("foo")?;
    /// assert_eq!(writer.as_str(), "foo");
    ///
    /// writer.clear();
    /// assert_eq!(writer.as_str(), "");
    /// assert!(writer.is_empty());
    ///
    /// writer.write_str("bar");
    /// assert_eq!(writer.as_str(), "bar");
    ///
    /// # Ok::<(), const_format::Error>(())
    /// ```

    #[inline]
    pub const fn clear(&mut self) {
        *self.len = 0;
    }

    /// Gets the written part of this `StrWriterMut` as a `&[u8]`
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
    /// let mut buffer = StrWriter::new([0; 64]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    ///
    /// writer.write_str("Hello, World!");
    ///
    /// assert_eq!(writer.as_bytes_alt(), "Hello, World!".as_bytes());
    ///
    /// ```
    #[inline(always)]
    pub const fn as_bytes_alt(&self) -> &[u8] {
        crate::utils::slice_up_to_len_alt(self.buffer, *self.len)
    }
}

impl<'w> StrWriterMut<'w, Utf8Encoding> {
    /// Gets the written part of this StrWriterMut as a `&str`
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
    /// use const_format::{unwrap, writec};
    ///
    ///
    /// const CAP: usize = 128;
    ///
    /// const __STR: &StrWriter = &{
    ///     let mut buffer =  StrWriter::new([0; CAP]);
    ///     let mut writer = StrWriterMut::new(&mut buffer);
    ///
    ///     // Writing the array with debug formatting, and the integers with hexadecimal formatting.
    ///     unwrap!(writec!(writer, "{:X}", [3u32, 5, 8, 13, 21, 34]));
    ///
    ///     buffer
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
        /// Gets the written part of this StrWriterMut as a `&str`
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
        /// # Example
        ///
        /// ```rust
        ///
        /// use const_format::{StrWriter, StrWriterMut};
        ///
        /// let mut buffer = StrWriter::new([0; 64]);
        /// let mut writer = StrWriterMut::new(&mut buffer);
        ///
        /// writer.write_str("Hello, how are you?");
        ///
        /// assert_eq!(writer.as_str(), "Hello, how are you?");
        ///
        /// ```
        #[inline(always)]
        pub fn as_str(&self) -> &str {
            // All the methods that modify the buffer must ensure utf8 validity,
            // only methods from this module need to ensure this.
            unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
        }
    }
}

impl<'w, E> StrWriterMut<'w, E> {
    conditionally_const! {
        feature = "rust_1_64";

        /// Gets the written part of this `StrWriterMut` as a `&[u8]`
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
        ///
        /// use const_format::{StrWriter, StrWriterMut};
        ///
        /// let mut buffer = StrWriter::new([0; 64]);
        /// let mut writer = StrWriterMut::new(&mut buffer);
        ///
        /// writer.write_str("Hello, World!");
        ///
        /// assert_eq!(writer.as_bytes_alt(), "Hello, World!".as_bytes());
        ///
        /// ```
        #[inline(always)]
        pub fn as_bytes(&self) -> &[u8] {
            crate::utils::slice_up_to_len(self.buffer, *self.len)
        }
    }
}

impl<'w, E> StrWriterMut<'w, E> {
    /// Constructs a [`Formatter`] that writes into this `StrWriterMut`,
    /// which can be passed to debug and display formatting methods.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Error, Formatter, FormattingFlags, StrWriter, StrWriterMut};
    /// use const_format::call_debug_fmt;
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
    /// let mut buffer = StrWriter::new([0; 64]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    ///
    /// range_debug_fmt(
    ///     &[0..14, 14..31, 31..48],
    ///     &mut writer.make_formatter(FormattingFlags::new().set_binary())
    /// )?;
    ///    
    /// assert_eq!(writer.as_str(), "[0..1110, 1110..11111, 11111..110000]");
    ///
    /// # Ok::<(), Error>(())
    ///
    /// ```
    ///
    /// [`Formatter`]: ./struct.Formatter.html
    #[inline(always)]
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter::from_sw_mut(
            StrWriterMut::<NoEncoding> {
                len: self.len,
                buffer: self.buffer,
                _encoding: PhantomData,
            },
            flags,
        )
    }

    /// For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut StrWriterMut<'w, E> {
        self
    }

    /// For passing a reborrow of this `StrWriterMut` into functions,
    /// without this you'd need to pass a mutable reference instead.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Error, FormattingFlags, StrWriter, StrWriterMut, call_debug_fmt};
    ///
    /// use std::ops::Range;
    ///
    /// const fn range_debug_fmt(
    ///     slice: &[[u32; 2]],
    ///     mut writer: StrWriterMut<'_>
    /// ) -> Result<(), Error> {
    ///     let mut formatter = writer.make_formatter(FormattingFlags::new().set_binary());
    ///
    ///     // We need this macro to debug format arrays of non-primitive types
    ///     // Also, it implicitly returns a `const_format::Error` on error.
    ///     call_debug_fmt!(array, slice, formatter);
    ///     Ok(())
    /// }
    ///
    /// let mut buffer = StrWriter::new([0; 64]);
    /// let mut writer = StrWriterMut::new(&mut buffer);
    ///
    /// range_debug_fmt(&[[3, 5], [8, 13]], writer.reborrow())?;
    ///    
    /// assert_eq!(writer.as_str(), "[[11, 101], [1000, 1101]]");
    ///
    /// # Ok::<(), Error>(())
    ///
    /// ```
    #[inline(always)]
    pub const fn reborrow(&mut self) -> StrWriterMut<'_, E> {
        StrWriterMut {
            len: self.len,
            buffer: self.buffer,
            _encoding: PhantomData,
        }
    }

    // Safety: You must not write invalid utf8 bytes with the returned StrWriterMut.
    pub(crate) const unsafe fn into_byte_encoding(self) -> StrWriterMut<'w, NoEncoding> {
        StrWriterMut {
            len: self.len,
            buffer: self.buffer,
            _encoding: PhantomData,
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////

macro_rules! write_integer_fn {
    (
        display_attrs $display_attrs:tt
        debug_attrs $debug_attrs:tt
        $(($display_fn:ident, $debug_fn:ident, $sign:ident, $ty:ident, $Unsigned:ident))*
    )=>{
        impl<'w,E> StrWriterMut<'w,E>{
            $(
                write_integer_fn!{
                    @methods
                    display_attrs $display_attrs
                    debug_attrs $debug_attrs
                    $display_fn, $debug_fn, $sign, ($ty, $Unsigned), stringify!($ty)
                }
            )*
        }

        $(
            write_integer_fn!{
                @pwrapper
                $display_fn, $debug_fn, $sign, ($ty, $Unsigned), stringify!($ty)
            }
        )*
    };
    (@pwrapper
        $display_fn:ident,
        $debug_fn:ident,
        $sign:ident,
        ($ty:ident, $Unsigned:ident),
        $ty_name:expr
    )=>{
        impl PWrapper<$ty> {
            /// Writes a
            #[doc = $ty_name]
            /// with Display formatting.
            pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                f.$display_fn(self.0)
            }

            /// Writes a
            #[doc = $ty_name]
            /// with Debug formatting.
            pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                f.$debug_fn(self.0)
            }
        }
    };
    (@methods
        display_attrs( $(#[$display_attrs:meta])* )
        debug_attrs( $(#[$debug_attrs:meta])* )
        $display_fn:ident,
        $debug_fn:ident,
        $sign:ident,
        ($ty:ident, $Unsigned:ident),
        $ty_name:expr
    )=>{
        $(#[$display_attrs])*
        pub const fn $display_fn(&mut self, number: $ty) -> Result<(), Error> {
            borrow_fields!(self, this_len, this_buffer);

            let n = PWrapper(number);
            let len = n.compute_display_len(FormattingFlags::DEFAULT);

            let mut cursor = *this_len + len;

            if cursor > this_buffer.len() {
                return Err(Error::NotEnoughSpace);
            }

            write_integer_fn!(@unsigned_abs $sign, n);

            loop {
                cursor-=1;
                let digit = (n % 10) as u8;
                this_buffer[cursor] = b'0' + digit;
                n/=10;
                if n == 0 { break }
            }

            write_integer_fn!(@write_sign $sign, this_len, this_buffer, number);

            *this_len+=len;
            Ok(())
        }

        $(#[$debug_attrs])*
        pub const fn $debug_fn(
            &mut self,
            number: $ty,
            flags: FormattingFlags,
        ) -> Result<(), Error> {
            const fn hex<E>(
                this: &mut StrWriterMut<'_, E>,
                n: $ty,
                f: FormattingFlags,
            ) -> Result<(), Error> {
                borrow_fields!(this, this_len, this_buffer);

                let is_alternate = f.is_alternate();
                let len = PWrapper(n).hexadecimal_len(f);

                let mut cursor = *this_len + len;

                if cursor > this_buffer.len() {
                    return Err(Error::NotEnoughSpace);
                }

                if is_alternate {
                    this_buffer[*this_len] = b'0';
                    this_buffer[*this_len + 1] = b'x';
                }

                write_integer_fn!(@as_unsigned $sign, n, $Unsigned);

                loop {
                    cursor-=1;
                    let digit = (n & 0b1111) as u8;
                    this_buffer[cursor] = hex_as_ascii(digit, f.hex_fmt());
                    n >>= 4;
                    if n == 0 { break }
                }

                *this_len+=len;
                Ok(())
            }

            const fn binary<E>(
                this: &mut StrWriterMut<'_, E>,
                n: $ty,
                f: FormattingFlags,
            ) -> Result<(), Error> {
                borrow_fields!(this, this_len, this_buffer);

                let is_alternate = f.is_alternate();
                let len = PWrapper(n).binary_len(f);

                let mut cursor = *this_len + len;

                if cursor > this_buffer.len() {
                    return Err(Error::NotEnoughSpace);
                }

                if is_alternate {
                    this_buffer[*this_len] = b'0';
                    this_buffer[*this_len + 1] = b'b';
                }

                write_integer_fn!(@as_unsigned $sign, n, $Unsigned);

                loop {
                    cursor-=1;
                    let digit = (n & 1) as u8;
                    this_buffer[cursor] = hex_as_ascii(digit, f.hex_fmt());
                    n >>= 1;
                    if n == 0 { break }
                }

                *this_len+=len;
                Ok(())
            }

            match flags.num_fmt() {
                NumberFormatting::Decimal=>self.$display_fn(number),
                NumberFormatting::Hexadecimal=>hex(self, number, flags),
                NumberFormatting::Binary=>binary(self, number, flags),
            }
        }
    };
    (@unsigned_abs signed, $n:ident) => (
        let mut $n = $n.unsigned_abs();
    );
    (@unsigned_abs unsigned, $n:ident) => (
        let mut $n = $n.0;
    );
    (@as_unsigned signed, $n:ident, $Unsigned:ident) => (
        let mut $n = $n as $Unsigned;
    );
    (@as_unsigned unsigned, $n:ident, $Unsigned:ident) => (
        let mut $n = $n;
    );
    (@write_sign signed, $self_len:ident, $self_buffer:ident, $n:ident) => ({
        if $n < 0 {
            $self_buffer[*$self_len] = b'-';
        }
    });
    (@write_sign unsigned, $self_len:ident, $self_buffer:ident, $n:ident) => ({});
}

/// Checks that a range is valid for indexing a string,
/// assuming that the range is in-bounds, and start <= end.
#[inline]
const fn is_valid_str_range(s: &[u8], Range { start, end }: Range<usize>) -> bool {
    let len = s.len();

    (end == len || ((s[end] as i8) >= -0x40)) && (start == len || ((s[start] as i8) >= -0x40))
}

/// Checks that an index is valid for indexing a string,
/// assuming that the index is in-bounds.
#[inline]
const fn is_valid_str_index(s: &[u8], index: usize) -> bool {
    let len = s.len();

    index == len || ((s[index] as i8) >= -0x40)
}

impl<'w, E> StrWriterMut<'w, E> {
    /// Writes a subslice of `s` with Display formatting.
    ///
    /// This is a workaround for being unable to do `&foo[start..end]` at compile time.
    ///
    /// # Additional Errors
    ///
    /// This method returns `Error::NotOnCharBoundary` if the range is not
    /// on a character boundary.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`,
    /// this only returns an error on an in-bounds index that is not on a character boundary.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{FormattingFlags, StrWriterMut};
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_str_range("FOO BAR BAZ", 4..7);
    ///
    /// assert_eq!(writer.as_str(), "BAR");
    ///
    /// ```
    ///
    pub const fn write_str_range(&mut self, s: &str, range: Range<usize>) -> Result<(), Error> {
        let bytes = s.as_bytes();
        let Range { start, end } = saturate_range(bytes, &range);

        if !is_valid_str_range(bytes, start..end) {
            return Err(Error::NotOnCharBoundary);
        }

        self.write_str_inner(bytes, start, end)
    }

    /// Writes `s` with Display formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{FormattingFlags, StrWriterMut};
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_str("FOO BAR BAZ");
    ///
    /// assert_eq!(writer.as_str(), "FOO BAR BAZ");
    ///
    /// ```
    ///
    pub const fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let bytes = s.as_bytes();

        self.write_str_inner(bytes, 0, s.len())
    }

    /// Writes `character` with Display formatting
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::StrWriterMut;
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_char('3');
    /// let _ = writer.write_char('5');
    /// let _ = writer.write_char('8');
    ///
    /// assert_eq!(writer.as_str(), "358");
    ///
    /// ```
    ///
    pub const fn write_char(&mut self, character: char) -> Result<(), Error> {
        let fmt = crate::char_encoding::char_to_display(character);
        self.write_str_inner(fmt.encoded(), 0, fmt.len())
    }

    /// Writes a subslice of `ascii` with Display formatting.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`.
    ///
    /// This is a workaround for being unable to do `&foo[start..end]` at compile time.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{FormattingFlags, StrWriterMut, ascii_str};
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_ascii_range(ascii_str!("FOO BAR BAZ"), 4..7);
    ///
    /// assert_eq!(writer.as_str(), "BAR");
    ///
    /// ```
    ///
    pub const fn write_ascii_range(
        &mut self,
        ascii: AsciiStr<'_>,
        range: Range<usize>,
    ) -> Result<(), Error> {
        let bytes = ascii.as_bytes();
        let Range { start, end } = saturate_range(bytes, &range);

        self.write_str_inner(bytes, start, end)
    }

    /// Writes `ascii` with Display formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{FormattingFlags, StrWriterMut, ascii_str};
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_ascii(ascii_str!("FOO BAR BAZ"));
    ///
    /// assert_eq!(writer.as_str(), "FOO BAR BAZ");
    ///
    /// ```
    ///
    pub const fn write_ascii(&mut self, ascii: AsciiStr<'_>) -> Result<(), Error> {
        let bytes = ascii.as_bytes();

        self.write_str_inner(bytes, 0, bytes.len())
    }

    /// Writes an ascii `character`, `repeated` times.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{FormattingFlags, StrWriterMut};
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_ascii_repeated(b'A', 10);
    ///
    /// assert_eq!(writer.as_str(), "AAAAAAAAAA");
    ///
    /// ```
    ///
    pub const fn write_ascii_repeated(
        &mut self,
        mut character: u8,
        repeated: usize,
    ) -> Result<(), Error> {
        borrow_fields!(self, self_len, self_buffer);

        // Truncating non-ascii u8s
        character &= 0b111_1111;

        let end = *self_len + repeated;

        if end > self_buffer.len() {
            return Err(Error::NotEnoughSpace);
        }

        while *self_len < end {
            self_buffer[*self_len] = character;
            *self_len += 1;
        }

        Ok(())
    }

    #[inline(always)]
    const fn write_str_inner(
        &mut self,
        bytes: &[u8],
        mut start: usize,
        end: usize,
    ) -> Result<(), Error> {
        borrow_fields!(self, self_len, self_buffer);

        let len = end - start;

        if *self_len + len > self_buffer.len() {
            return Err(Error::NotEnoughSpace);
        }

        while start < end {
            self_buffer[*self_len] = bytes[start];
            *self_len += 1;
            start += 1;
        }

        Ok(())
    }
}

/// Debug-formatted string writing
impl<'w, E> StrWriterMut<'w, E> {
    /// Writes a subslice of `s` with  Debug-like formatting.
    ///
    /// This is a workaround for being unable to do `&foo[start..end]` at compile time.
    ///
    /// # Additional Errors
    ///
    /// This method returns `Error::NotOnCharBoundary` if the range is not
    /// on a character boundary.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`,
    /// this only returns an error on an in-bounds index that is not on a character boundary.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{FormattingFlags, StrWriterMut};
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_str_range_debug("FOO\nBAR\tBAZ", 3..8);
    ///
    /// assert_eq!(writer.as_str(), r#""\nBAR\t""#);
    ///
    /// ```
    ///
    pub const fn write_str_range_debug(
        &mut self,
        s: &str,
        range: Range<usize>,
    ) -> Result<(), Error> {
        let bytes = s.as_bytes();
        let Range { start, end } = saturate_range(bytes, &range);

        if !is_valid_str_range(bytes, start..end) {
            return Err(Error::NotOnCharBoundary);
        }

        self.write_str_debug_inner(bytes, start, end)
    }

    /// Writes `s` with Debug-like formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{FormattingFlags, StrWriterMut};
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_str_debug("FOO\nBAR\tBAZ");
    ///
    /// assert_eq!(writer.as_str(), r#""FOO\nBAR\tBAZ""#);
    ///
    /// ```
    ///
    pub const fn write_str_debug(&mut self, str: &str) -> Result<(), Error> {
        let bytes = str.as_bytes();
        self.write_str_debug_inner(bytes, 0, str.len())
    }

    /// Writes `character` with Debug formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::StrWriterMut;
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_str(" ");
    /// let _ = writer.write_char_debug('\\');
    /// let _ = writer.write_str(" ");
    /// let _ = writer.write_char_debug('A');
    /// let _ = writer.write_str(" ");
    /// let _ = writer.write_char_debug('0');
    /// let _ = writer.write_str(" ");
    /// let _ = writer.write_char_debug('\'');
    /// let _ = writer.write_str(" ");
    ///
    /// assert_eq!(writer.as_str(), r#" '\\' 'A' '0' '\'' "#);
    ///
    /// ```
    ///
    pub const fn write_char_debug(&mut self, character: char) -> Result<(), Error> {
        let fmt = crate::char_encoding::char_to_debug(character);
        self.write_str_inner(fmt.encoded(), 0, fmt.len())
    }

    /// Writes a subslice of `ascii` with Debug-like formatting.
    ///
    /// Out of bounds range bounds are treated as being at `s.len()`.
    ///
    /// This is a workaround for being unable to do `&foo[start..end]` at compile time.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{FormattingFlags, StrWriterMut, ascii_str};
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_ascii_range_debug(ascii_str!("FOO\nBAR\tBAZ"), 3..8);
    ///
    /// assert_eq!(writer.as_str(), r#""\nBAR\t""#);
    ///
    /// ```
    ///
    pub const fn write_ascii_range_debug(
        &mut self,
        ascii: AsciiStr<'_>,
        range: Range<usize>,
    ) -> Result<(), Error> {
        let bytes = ascii.as_bytes();
        let Range { start, end } = saturate_range(bytes, &range);

        self.write_str_debug_inner(bytes, start, end)
    }

    /// Writes `ascii` with Debug-like formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{FormattingFlags, StrWriterMut, ascii_str};
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 64];
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// let _ = writer.write_ascii_debug(ascii_str!("FOO\nBAR\tBAZ"));
    ///
    /// assert_eq!(writer.as_str(), r#""FOO\nBAR\tBAZ""#);
    ///
    /// ```
    ///
    pub const fn write_ascii_debug(&mut self, ascii: AsciiStr<'_>) -> Result<(), Error> {
        let bytes = ascii.as_bytes();
        self.write_str_debug_inner(bytes, 0, bytes.len())
    }

    #[inline(always)]
    const fn write_str_debug_inner(
        &mut self,
        bytes: &[u8],
        mut start: usize,
        end: usize,
    ) -> Result<(), Error> {
        borrow_fields!(self, self_len, self_buffer);

        let len = end - start;

        // + 2 for the quote characters around the string.
        if *self_len + len + 2 > self_buffer.len() {
            return Err(Error::NotEnoughSpace);
        }

        // The amount of bytes available for escapes,
        // not counting the `writte_c`.
        let mut remaining_for_escapes = (self_buffer.len() - 2 - len - *self_len) as isize;
        let mut written = *self_len;

        self_buffer[written] = b'"';
        written += 1;

        while start != end {
            let c = bytes[start];
            let mut written_c = c;

            let mut shifted = 0;

            if c < 128
                && ({
                    shifted = 1 << c;
                    (FOR_ESCAPING.is_escaped & shifted) != 0
                })
            {
                self_buffer[written] = b'\\';
                written += 1;

                if (FOR_ESCAPING.is_backslash_escaped & shifted) != 0 {
                    remaining_for_escapes -= 1;
                    if remaining_for_escapes < 0 {
                        return Err(Error::NotEnoughSpace);
                    }
                    written_c = ForEscaping::get_backslash_escape(c);
                } else {
                    remaining_for_escapes -= 3;
                    if remaining_for_escapes < 0 {
                        return Err(Error::NotEnoughSpace);
                    }
                    self_buffer[written] = b'x';
                    written += 1;
                    self_buffer[written] = hex_as_ascii(c >> 4, HexFormatting::Upper);
                    written += 1;
                    written_c = hex_as_ascii(c & 0xF, HexFormatting::Upper);
                };
            }

            self_buffer[written] = written_c;
            written += 1;
            start += 1;
        }

        self_buffer[written] = b'"';
        written += 1;

        *self_len = written;

        Ok(())
    }
}

write_integer_fn! {
    display_attrs(
        /// Write `number` with display formatting.
        ///
        /// # Example
        ///
        /// ```rust
        ///
        /// use const_format::{Formatter, FormattingFlags, StrWriterMut, ascii_str};
        ///
        /// let mut len = 0;
        /// let mut buffer = [0; 64];
        /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
        ///
        /// let _ = writer.write_u8_display(137);
        ///
        /// assert_eq!(writer.as_str(), "137");
        ///
        /// ```
        ///
    )
    debug_attrs(
        /// Writes `number` with debug formatting.
        ///
        /// # Example
        ///
        /// ```rust
        ///
        /// use const_format::{FormattingFlags, StrWriterMut};
        ///
        /// const fn debug_fmt<'a>(
        ///     writer: &'a mut StrWriterMut<'_>,
        ///     flag: FormattingFlags
        /// ) -> &'a str {
        ///     writer.clear();
        ///     let _ = writer.write_u8_debug(63, flag);
        ///     writer.as_str_alt()
        /// }
        ///
        /// let reg_flag = FormattingFlags::NEW.set_alternate(false);
        /// let alt_flag = FormattingFlags::NEW.set_alternate(true);
        ///
        /// let mut len = 0;
        /// let mut buffer = [0; 64];
        /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
        ///
        /// assert_eq!(debug_fmt(&mut writer, reg_flag),                   "63"     );
        /// assert_eq!(debug_fmt(&mut writer, reg_flag.set_hexadecimal()), "3F"     );
        /// assert_eq!(debug_fmt(&mut writer, reg_flag.set_lower_hexadecimal()), "3f");
        /// assert_eq!(debug_fmt(&mut writer, reg_flag.set_binary()),      "111111" );
        /// assert_eq!(debug_fmt(&mut writer, alt_flag),                   "63"     );
        /// assert_eq!(debug_fmt(&mut writer, alt_flag.set_hexadecimal()), "0x3F"   );
        /// assert_eq!(debug_fmt(&mut writer, alt_flag.set_lower_hexadecimal()), "0x3f");
        /// assert_eq!(debug_fmt(&mut writer, alt_flag.set_binary()),      "0b111111");
        ///
        /// ```
        ///
    )
    (write_u8_display, write_u8_debug, unsigned, u8, u8)
}
write_integer_fn! {
    display_attrs(
        /// Writes `number` with display formatting
        ///
        /// For an example,
        /// you can look at the one for the [`write_u8_display`] method.
        ///
        /// [`write_u8_display`]: #method.write_u8_display
    )
    debug_attrs(
        /// Writes `number` with debug formatting.
        ///
        /// For an example,
        /// you can look at the one for the [`write_u8_debug`] method.
        ///
        /// [`write_u8_debug`]: #method.write_u8_debug
    )
    (write_u16_display, write_u16_debug, unsigned, u16, u16)
    (write_u32_display, write_u32_debug, unsigned, u32, u32)
    (write_u64_display, write_u64_debug, unsigned, u64, u64)
    (write_u128_display, write_u128_debug, unsigned, u128, u128)
    (write_usize_display, write_usize_debug, unsigned, usize, usize)

    (write_i8_display, write_i8_debug, signed, i8, u8)
    (write_i16_display, write_i16_debug, signed, i16, u16)
    (write_i32_display, write_i32_debug, signed, i32, u32)
    (write_i64_display, write_i64_debug, signed, i64, u64)
    (write_i128_display, write_i128_debug, signed, i128, u128)
    (write_isize_display, write_isize_debug, signed, isize, usize)
}
