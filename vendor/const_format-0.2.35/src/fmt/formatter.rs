use crate::{
    fmt::{Error, FormattingFlags, NoEncoding, StrWriter, StrWriterMut},
    utils::saturate_range,
    wrapper_types::{AsciiStr, PWrapper},
};

use core::ops::Range;

////////////////////////////////////////////////////////////////////////////////

/// For computing how long a formatted string would be.
///
/// This is what the [`formatc`] macro uses to precalculate the length of its returned `&str`.
///
/// # Example
///
/// ```rust
///
/// use const_format::fmt::{ComputeStrLength, Error, Formatter, FormattingFlags, StrWriter};
/// use const_format::{try_, writec, unwrap};
///
/// const fn write_sum(mut f: Formatter<'_>) -> Result<(), Error> {
///     let l = 7u8;
///     let r = 8u8;
///     writec!(f, "{} + {} = {}", l, r, l + r)
/// }
///
/// const LEN: usize = {
///     let mut computer = ComputeStrLength::new();
///     unwrap!(write_sum(computer.make_formatter(FormattingFlags::NEW)));
///     computer.len()
/// };
///
/// // The type annotation coerces a `&mut StrWriter<[u8; LEN]>`
/// // to a `&mut StrWriter<[u8]>` (the type parameter defaults to `[u8]`)
/// let writer: &mut StrWriter = &mut StrWriter::new([0; LEN]);
///
/// write_sum(writer.make_formatter(FormattingFlags::NEW)).unwrap();
///
/// assert_eq!(writer.as_str(), "7 + 8 = 15");
/// assert_eq!(writer.len(), LEN);
/// assert_eq!(writer.capacity(), LEN);
///
/// ```
///
/// [`formatc`]: ../macro.formatc.html
///
///
pub struct ComputeStrLength {
    len: usize,
}

impl ComputeStrLength {
    /// Constructs a ComputeStrLength of length 0.
    pub const fn new() -> Self {
        Self { len: 0 }
    }

    /// Constructs a `Formatter`,
    /// which instead of writing to a buffer it adds the computed length into this.
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter {
            margin: 0,
            flags,
            writer: WriterBackend::Length(self),
        }
    }

    /// Adds `len` to the calculated length.
    pub const fn add_len(&mut self, len: usize) {
        self.len += len;
    }

    /// The length of the string when formatted.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Whether the length of the computed string is zero.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// For borrowing this mutably in macros,just takes and returns a `&mut Self`.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

enum WriterBackend<'w> {
    Str(StrWriterMut<'w, NoEncoding>),
    Length(&'w mut ComputeStrLength),
}

////////////////////////////////////////////////////////////////////////////////

/// A handle for writing formatted output.
///
/// `Formatter` writes utf8 encoded text, it can't be used to write arbitrary bytes.
///
/// # FormattingFlags
///
/// Types can change how they're formatted based on the value returned by `.flags()`,
/// for more details on that you can read the documentation for [`FormattingFlags`].
///
/// # Construction
///
/// This type can be constructed in these ways:
///
/// - From a pair of mutable reference to a [`StrWriter`] and a [`FormattingFlags`],
/// with the [`from_sw`] constructor.
///
/// - From a pair of [`StrWriterMut`] and [`FormattingFlags`],
/// with the [`from_sw_mut`] constructor.
///
/// - From a [`ComputeStrLength`], by calling its
/// [`make_formatter`](./struct.ComputeStrLength.html#method.make_formatter) method.
/// This allows computing the length of formatted output without writing to anything.
///
/// - From a triple of `[u8]` and `usize` mutable references, and a [`FormattingFlags`],
/// with the [`from_custom_cleared`] constructor,
/// or the [`from_custom`] constructor.
///
/// # Errors
///
/// The `write_*` methods can only return an `Error::NotEnoughSpace`,
/// when they do, the formatter was not written to, so you can try again with a shorter input.
///
/// In the case of the `debug_*` methods / the `Debug*` structs,
/// they can return a `Error::NotEnoughSpace` when their `finish` method is called,
/// not as soon as it happens.
///
/// # Examples
///
/// ### Display formatting
///
/// This example demonstrates how you can do display formatting with a Formatter.
///
/// If you want to write a braced struct/variant you can use [`DebugStruct`],
/// or [`DebugTuple`] for tuple structs/variants.
///
/// ```rust
///
/// use const_format::{Error, Formatter, FormattingFlags, StrWriter};
/// use const_format::{impl_fmt, try_};
///
/// struct Foo;
///
/// impl_fmt!{
///     impl[] Foo;
///     
///     const fn const_display_fmt(&self, mut f: Formatter<'_>) -> Result<(), Error> {
///         let string = "foo bar baz";
///         try_!(f.write_u8_display(100));
///         try_!(f.write_str(" "));
///         try_!(f.write_str_range(string, 4..7));
///         try_!(f.write_str("\n\n\n...figters"));
///         Ok(())
///     }
/// }
///
/// // We have to coerce `&mut StrWriter<[u8; 256]>` to `&mut StrWriter` to call the
/// // `make_formatter` method.
/// let writer: &mut StrWriter = &mut StrWriter::new([0; 256]);
///
/// let flags = FormattingFlags::NEW.set_binary();
///
/// // The Display formatters from this crate don't care which NumberFormatting you pass,
/// // they'll just write integers as decimal.
/// Foo.const_display_fmt(writer.make_formatter(flags));
///
/// assert_eq!(writer.as_str(), "100 bar\n\n\n...figters");
/// ```
///
/// <span id = "write_array_example"></span>
/// ### Writing to an array
///
/// This example demonstrates how you can use a Formatter to write to a byte slice.
///
/// You can use the [`from_custom`] constructor if you need to start writing from
/// anywhere other than 0.
///
/// ```rust
///
/// use const_format::{Error, Formatter, FormattingFlags, StrWriter};
/// use const_format::{impl_fmt, try_, writec};
///
/// const fn write_int(int: u32, buffer: &mut [u8]) -> Result<usize, Error> {
///     let mut len = 0;
///     let mut f = Formatter::from_custom_cleared(buffer, &mut len, FormattingFlags::NEW);
///     try_!(writec!(f, "{0},{0:x},{0:b}", int));
///     Ok(len)
/// }
///
/// let mut buffer = [0;64];
///
/// let written = write_int(17, &mut buffer).unwrap();
///
/// let string = std::str::from_utf8(&buffer[..written])
///     .expect("Formatter only writes valid UTF8");
///
/// assert_eq!(string, "17,11,10001");
///
/// ```
///
///
/// [`DebugStruct`]: crate::fmt::DebugStruct
/// [`DebugTuple`]: crate::fmt::DebugTuple
/// [`StrWriter`]: crate::fmt::StrWriter
/// [`StrWriterMut`]: crate::fmt::StrWriterMut
/// [`ComputeStrLength`]: crate::fmt::ComputeStrLength
/// [`from_sw`]: #method.from_sw
/// [`from_sw_mut`]: #method.from_sw_mut
/// [`from_custom_cleared`]: #method.from_custom_cleared
/// [`from_custom`]:  #method.from_custom
/// [`NumberFormatting`]: crate::fmt::NumberFormatting
/// [`FormattingFlags`]: crate::fmt::FormattingFlags
///
pub struct Formatter<'w> {
    margin: u16,
    flags: FormattingFlags,
    writer: WriterBackend<'w>,
}

const MARGIN_STEP: u16 = 4;

impl<'w> Formatter<'w> {
    /// Constructs a `Formatter`.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Error, Formatter, FormattingFlags, StrWriter};
    /// use const_format::try_;
    ///
    /// const fn inner(mut f: Formatter<'_>) -> Result<(), Error> {
    ///     try_!(f.write_str_range("ABCDEF", 2..4));
    ///     try_!(f.write_str(" N"));
    ///     try_!(f.write_ascii_repeated(b'o', 10));
    ///     Ok(())
    /// }
    ///
    /// // We have to coerce `&mut StrWriter<[u8; 128]>` to `&mut StrWriter` to call the
    /// // `as_str` method.
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 128]);
    /// inner(Formatter::from_sw(writer, FormattingFlags::NEW)).unwrap();
    ///
    /// assert_eq!(writer.as_str(), "CD Noooooooooo");
    ///
    /// ```
    #[inline]
    pub const fn from_sw(writer: &'w mut StrWriter, flags: FormattingFlags) -> Self {
        Self {
            margin: 0,
            flags,
            // safety:
            // Formatter only writes valid utf8, which is valid for both
            // encoding type parameters that StrWriterMut can have(Utf8Encoding / NoEncoding).
            writer: WriterBackend::Str(unsafe { writer.as_mut().into_byte_encoding() }),
        }
    }

    /// Constructs a `Formatter`.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Error, Formatter, FormattingFlags, StrWriterMut};
    /// use const_format::try_;
    ///
    /// const fn inner(mut f: Formatter<'_>) -> Result<(), Error> {
    ///     try_!(f.write_str_range("DVDVDVD", 2..5));
    ///     try_!(f.write_str(" N"));
    ///     try_!(f.write_ascii_repeated(b'o', 10));
    ///     Ok(())
    /// }
    ///
    /// let mut len = 0;
    /// let mut buffer = [0; 128];
    ///
    /// let mut writer = StrWriterMut::from_custom_cleared(&mut buffer, &mut len);
    ///
    /// // We need to call `.reborrow()`, because otherwise the `StrWriterMut` is moved.
    /// inner(Formatter::from_sw_mut(writer.reborrow(), FormattingFlags::NEW)).unwrap();
    ///
    /// assert_eq!(writer.as_str(), "DVD Noooooooooo");
    ///
    /// ```
    #[inline]
    pub const fn from_sw_mut<E: 'static>(
        writer: StrWriterMut<'w, E>,
        flags: FormattingFlags,
    ) -> Self {
        Self {
            margin: 0,
            flags,
            // safety:
            // Formatter only writes valid utf8, which is valid for both
            // encoding type parameters that StrWriterMut can have(Utf8Encoding / NoEncoding).
            writer: WriterBackend::Str(unsafe { writer.into_byte_encoding() }),
        }
    }

    /// Construct a `Formatter` from a byte slice.
    ///
    /// `Formatter` only writes utf8, which means that if `&buffer[..length]` is valid utf8,
    /// then `buffer` will continue to be `utf8` after being written by the `Formatter`.
    ///
    /// # Example
    ///
    /// This example demonstrates how you can use a Formatter to write to a byte slice
    /// that had some text written to it already.
    ///
    /// ```rust
    ///
    /// use const_format::{Error, Formatter, FormattingFlags, StrWriter};
    /// use const_format::{impl_fmt, try_, writec};
    ///
    /// ///
    /// /// # Safety
    /// ///
    /// /// `&buffer[..start]` must be valid utf8.
    /// const fn write_int(
    ///     int: u32,
    ///     buffer: &mut [u8],
    ///     start: usize,
    /// ) -> Result<usize, Error> {
    ///     let mut len = start;
    ///     let mut f = Formatter::from_custom(buffer, &mut len, FormattingFlags::NEW);
    ///     try_!(writec!(f, "{0},{0:x},{0:b}", int));
    ///     Ok(len)
    /// }
    ///
    /// let start_str = "The number is ";
    /// let mut buffer = [0;64];
    /// buffer[..start_str.len()].copy_from_slice(start_str.as_bytes());
    ///
    /// let new_len = write_int(20, &mut buffer, start_str.len()).unwrap();
    ///
    /// let string = std::str::from_utf8(&buffer[..new_len])
    ///     .expect("Formatter only writes valid UTF8");
    ///
    /// assert_eq!(string, "The number is 20,14,10100");
    ///
    /// ```
    #[inline]
    pub const fn from_custom(
        buffer: &'w mut [u8],
        length: &'w mut usize,
        flags: FormattingFlags,
    ) -> Self {
        Self {
            margin: 0,
            flags,
            writer: WriterBackend::Str(StrWriterMut::from_custom(buffer, length)),
        }
    }

    /// Construct a `Formatter`from a byte slice.
    ///
    /// # Example
    ///
    /// For an example of using this method you can look at
    /// [the type level docs](#write_array_example)
    ///
    #[inline]
    pub const fn from_custom_cleared(
        buffer: &'w mut [u8],
        length: &'w mut usize,
        flags: FormattingFlags,
    ) -> Self {
        *length = 0;
        Self {
            margin: 0,
            flags,
            writer: WriterBackend::Str(StrWriterMut::from_custom(buffer, length)),
        }
    }

    /// Gets the formatting flags associated with this `Formatter`.
    #[inline(always)]
    pub const fn flags(&self) -> FormattingFlags {
        self.flags
    }

    /// Gets how much indentation a data structure is printed with.
    pub const fn margin(&self) -> usize {
        self.margin as usize
    }

    #[inline(always)]
    const fn increment_margin(&mut self) -> &mut Self {
        self.margin += 4;
        self
    }

    #[inline(always)]
    const fn decrement_margin(&mut self) {
        self.margin -= 4;
    }
}

impl<'w> Formatter<'w> {
    /// For borrowing this mutably in macros,just takes and returns a `&mut Self`.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }

    /// Constructs a reborrow of this formatter, using `flags` as the formatting flags.
    ///
    /// The return value inherits the margin from this Formatter.
    ///
    /// This method exists because the [`writec`] macro gets a formatter from any writer
    /// by calling a `make_formatter` method.
    ///
    /// # Example
    ///
    /// This example demonstrates how you can change the flags when writing a field.
    ///
    /// ```rust
    ///
    /// use const_format::{Error, Formatter, PWrapper};
    /// use const_format::{coerce_to_fmt, formatc, impl_fmt, try_};
    ///
    /// use std::ops::RangeInclusive;
    ///
    /// struct Foo{
    ///     x: u32,
    ///     y: RangeInclusive<usize>,
    ///     z: u32,
    /// }
    ///
    /// impl_fmt!{
    ///     impl Foo;
    ///
    ///     pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    ///         let mut f = f.debug_struct("Foo");
    ///         try_!(PWrapper(self.x).const_debug_fmt(f.field("x")));
    ///         
    ///         let mut fmt_y = f.field("y");
    ///         let flags = fmt_y.flags().set_binary();
    ///         try_!(coerce_to_fmt!(&self.y).const_debug_fmt(&mut fmt_y.make_formatter(flags)));
    ///
    ///         try_!(PWrapper(self.z).const_debug_fmt(f.field("z")));
    ///         f.finish()
    ///     }
    /// }
    ///
    /// const FOO: Foo = Foo {
    ///     x: 15,
    ///     y: 16..=31,
    ///     z: 32,
    /// };
    /// const S: &str = formatc!("{FOO:#?}");
    ///
    /// const EXPECTED: &str = "\
    /// Foo {
    ///     x: 15,
    ///     y: 0b10000..=0b11111,
    ///     z: 32,
    /// }\
    /// ";
    ///
    /// assert_eq!(S, EXPECTED);
    /// ```
    ///
    /// [`writec`]: ../macro.writec.html
    ///
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter {
            margin: self.margin,
            flags,
            writer: match &mut self.writer {
                WriterBackend::Str(x) => WriterBackend::Str(x.reborrow()),
                WriterBackend::Length(x) => WriterBackend::Length(x),
            },
        }
    }

    /// For debug writing a braced struct, or braced variant,
    /// taking its name as a parameter
    ///
    /// # Examples
    ///
    /// For examples of using this method, you can look at the docs for [`DebugStruct`]
    ///
    /// [`DebugStruct`]: ./struct.DebugStruct.html
    ///
    #[inline]
    pub const fn debug_struct(&mut self, name: &str) -> DebugStruct<'_, 'w> {
        let err = self.write_str(name);
        DebugStruct {
            fmt: self.increment_margin(),
            wrote_field: false,
            err,
        }
    }

    /// For debug writing a tuple struct, or tuple variant,taking its name as a parameter
    ///
    /// # Examples
    ///
    /// For examples of using this method, you can look at the docs for [`DebugTuple`]
    ///
    /// [`DebugTuple`]: ./struct.DebugTuple.html
    ///
    #[inline]
    pub const fn debug_tuple(&mut self, name: &str) -> DebugTuple<'_, 'w> {
        let err = self.write_str(name);
        DebugTuple {
            fmt: self.increment_margin(),
            wrote_field: false,
            err,
        }
    }

    /// For debug writing a list/array.
    ///
    /// # Examples
    ///
    /// For examples of using this method, you can look at the docs for [`DebugList`]
    ///
    /// [`DebugList`]: ./struct.DebugList.html
    ///
    #[inline]
    pub const fn debug_list(&mut self) -> DebugList<'_, 'w> {
        DebugList {
            fmt: self.increment_margin(),
            wrote_field: false,
            err: Ok(()),
        }
    }

    /// For debug writing a set.
    ///
    /// # Examples
    ///
    /// For examples of using this method, you can look at the docs for [`DebugSet`]
    ///
    /// [`DebugSet`]: ./struct.DebugSet.html
    ///
    #[inline]
    pub const fn debug_set(&mut self) -> DebugSet<'_, 'w> {
        DebugSet {
            fmt: self.increment_margin(),
            wrote_field: false,
            err: Ok(()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! trys {
    ($e:expr,$self:ident) => {
        if let result @ Err(_) = $e {
            $self.err = result;
        }
    };
}

const COLON_SPACE_LEN: usize = ": ".len();
const COMMA_SPACE_LEN: usize = ", ".len();
const COMMA_NL_LEN: usize = ",\n".len();

macro_rules! field_method_impl {
    ($
        self: ident, $open_space:expr, $open_newline:expr;
        len(|$fmt_len:ident| $($write_name_len:tt)*)
        fmt(|$writer:ident| $($write_name_fmt:tt)*)
    ) => ({
        match &mut $self.fmt.writer {
            WriterBackend::Length($fmt_len)=>{
                let $fmt_len = &mut **$fmt_len;

                const OPEN_SPACE: usize = $open_space.len();
                const OPEN_NEWLINE: usize = $open_newline.len();

                let is_alternate = $self.fmt.flags.is_alternate();
                $fmt_len.add_len(match ($self.wrote_field, is_alternate) {
                    (false, false) => OPEN_SPACE,
                    (false, true) => OPEN_NEWLINE + $self.fmt.margin as usize,
                    (true , false) => COMMA_SPACE_LEN,
                    (true , true) => COMMA_NL_LEN + $self.fmt.margin as usize,
                });
                $($write_name_len)*
            }
            WriterBackend::Str($writer)=>{
                let $writer = &mut *$writer;

                let is_alternate = $self.fmt.flags.is_alternate();
                let sep = match ($self.wrote_field, is_alternate) {
                    (false, false)=>$open_space,
                    (false, true)=>$open_newline,
                    (true, false)=>", ",
                    (true, true)=>",\n",
                };
                trys!($writer.write_str(sep), $self);
                if is_alternate {
                    trys!($writer.write_ascii_repeated(b' ', $self.fmt.margin as usize), $self);
                }
                $($write_name_fmt)*
            }
        }
        $self.wrote_field = true;

        $self.fmt
    })
}

macro_rules! finish_method_impl {
    ($self: ident, $close_token:expr, $space_close:expr) => {{
        if let result @ Err(_) = $self.err {
            return result;
        }

        $self.fmt.decrement_margin();
        if $self.wrote_field {
            match &mut $self.fmt.writer {
                WriterBackend::Length(fmt_len) => {
                    let fmt_len = &mut **fmt_len;

                    const CLOSE_TOKEN: usize = $close_token.len();
                    const SPACE_CLOSE: usize = $space_close.len();

                    if $self.fmt.flags.is_alternate() {
                        fmt_len.add_len(COMMA_NL_LEN + $self.fmt.margin as usize + CLOSE_TOKEN);
                    } else {
                        fmt_len.add_len(SPACE_CLOSE);
                    }
                    Ok(())
                }
                WriterBackend::Str(writer) => {
                    let writer = &mut *writer;

                    if $self.fmt.flags.is_alternate() {
                        try_!(writer.write_str(",\n"));
                        try_!(writer.write_ascii_repeated(b' ', $self.fmt.margin as usize));
                        writer.write_str($close_token)
                    } else {
                        writer.write_str($space_close)
                    }
                }
            }
        } else {
            Ok(())
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

/// A helper struct for debug formatting a braced struct, or braced variant.
///
/// # Example
///
/// This example demonstrates how you can debug format a struct,
/// and a braced variant.
///
/// ```rust
///
/// use const_format::{Error, Formatter};
/// use const_format::{call_debug_fmt, coerce_to_fmt, formatc, impl_fmt, try_};
///
/// fn main() {
///     const STRUC: &str = formatc!("{:?}", Foo { a: 5, b: [8, 13, 21], c: "34" });
///     const ENUM_: &str = formatc!("{:?}", Bar::Baz { d: false, e: None });
///     
///     assert_eq!(STRUC, "Foo { a: 5, b: [8, 13, 21], c: \"34\" }");
///     assert_eq!(ENUM_, "Baz { d: false, e: None }");
/// }
///
/// struct Foo{
///     a: u32,
///     b: [u32; 3],
///     c: &'static str,
/// }
///
/// enum Bar {
///     Baz{
///         d: bool,
///         e: Option<bool>,
///     }
/// }
///
/// impl_fmt!{
///     impl Foo;
///     
///     const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         let mut f = f.debug_struct("Foo");
///         try_!(coerce_to_fmt!(&self.a).const_debug_fmt(f.field("a")));
///         try_!(coerce_to_fmt!(&self.b).const_debug_fmt(f.field("b")));
///         try_!(coerce_to_fmt!(&self.c).const_debug_fmt(f.field("c")));
///         f.finish()
///     }
/// }
///
/// impl_fmt!{
///     impl Bar;
///     
///     const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         match self {
///             Bar::Baz{d, e} => {
///                 let mut f = f.debug_struct("Baz");
///                 
///                 // This macro allows debug formatting some generic types that
///                 // don't have a const_debug_fmt fn, like Options which wrap non-std types.
///                 call_debug_fmt!(std, d, f.field("d"));
///                 call_debug_fmt!(Option, e, f.field("e"));
///                 
///                 f.finish()
///             }
///         }
///     }
/// }
///
///
///
/// ```
pub struct DebugStruct<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugStruct<'f, 'w> {
    /// Adds a field to the formatted output.
    pub const fn field(&mut self, name: &str) -> &mut Formatter<'w> {
        field_method_impl!(
            self, " { ", " {\n";
            len(|fmt_len|
                fmt_len.add_len(name.len() + COLON_SPACE_LEN);
            )
            fmt(|writer|
                trys!(writer.write_str(name), self);
                trys!(writer.write_str(": "), self);
            )
        )
    }

    /// Finishes writing the struct/variant,
    /// and if anything went wrong in the `field` method,returns an error.
    pub const fn finish(self) -> Result<(), Error> {
        finish_method_impl!(self, "}", " }")
    }
}

////////////////////////////////////////////////////////////////////////////////

/// For debug formatting a tuple struct, or tuple variant.
///
/// # Example
///
/// This example demonstrates how you can debug format a tuple struct,
/// and an enum of tuple variants.
///
/// ```rust
///
/// use const_format::{Error, Formatter};
/// use const_format::{call_debug_fmt, coerce_to_fmt, formatc, impl_fmt, try_};
///
/// fn main() {
///     const STRUC: &str = formatc!("{:?}", Foo(5, [8, 13, 21], "34"));
///     const ENUM_: &str = formatc!("{:?}", Bar::Baz(false, None));
///     
///     assert_eq!(STRUC, "Foo(5, [8, 13, 21], \"34\")");
///     assert_eq!(ENUM_, "Baz(false, None)");
/// }
///
/// struct Foo(u32, [u32; 3], &'static str);
///
/// enum Bar {
///     Baz(bool, Option<bool>),
/// }
///
/// impl_fmt!{
///     impl Foo;
///     
///     const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         let mut f = f.debug_tuple("Foo");
///         try_!(coerce_to_fmt!(&self.0).const_debug_fmt(f.field()));
///         try_!(coerce_to_fmt!(&self.1).const_debug_fmt(f.field()));
///         try_!(coerce_to_fmt!(&self.2).const_debug_fmt(f.field()));
///         f.finish()
///     }
/// }
///
/// impl_fmt!{
///     impl Bar;
///     
///     const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         match self {
///             Bar::Baz(f0, f1) => {
///                 let mut f = f.debug_tuple("Baz");
///                 
///                 // This macro allows debug formatting some generic types that
///                 // don't have a const_debug_fmt fn, like Options which wrap non-std types.
///                 call_debug_fmt!(std, f0, f.field());
///                 call_debug_fmt!(Option, f1, f.field());
///                 
///                 f.finish()
///             }
///         }
///     }
/// }
///
///
///
/// ```
pub struct DebugTuple<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugTuple<'f, 'w> {
    /// Adds a field to the formatted output.
    pub const fn field(&mut self) -> &mut Formatter<'w> {
        field_method_impl!(self, "(", "(\n"; len(|fmt_len|) fmt(|writer|) )
    }

    /// Finishes writing the tuple struct/variant,
    /// and if anything went wrong in the `field` method,returns an error.
    pub const fn finish(self) -> Result<(), Error> {
        finish_method_impl!(self, ")", ")")
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! finish_listset_method_impl {
    ($self: ident, $close_token:expr, $open_close:expr) => {{
        if let result @ Err(_) = $self.err {
            return result;
        }

        match &mut $self.fmt.writer {
            WriterBackend::Length(fmt_len) => {
                let fmt_len = &mut **fmt_len;
                const CLOSE_TOKEN: usize = $close_token.len();
                const OPEN_CLOSE: usize = $open_close.len();

                $self.fmt.margin -= MARGIN_STEP;
                if $self.wrote_field {
                    if $self.fmt.flags.is_alternate() {
                        fmt_len.add_len(COMMA_NL_LEN + $self.fmt.margin as usize);
                    }
                    fmt_len.add_len(CLOSE_TOKEN);
                } else {
                    fmt_len.add_len(OPEN_CLOSE);
                }
                Ok(())
            }
            WriterBackend::Str(writer) => {
                let writer = &mut *writer;

                $self.fmt.margin -= MARGIN_STEP;
                let margin = $self.fmt.margin as usize;
                if $self.wrote_field {
                    if $self.fmt.flags.is_alternate() {
                        try_!(writer.write_str(",\n"));
                        try_!(writer.write_ascii_repeated(b' ', margin));
                    }
                    writer.write_str($close_token)
                } else {
                    writer.write_str($open_close)
                }
            }
        }
    }};
}

////////////////////////////////////////////////////////////////////////////////

/// For debug formatting a list/array.
///
/// # Example
///
/// This example demonstrates how you can debug format a custom type as a list.
///
/// ```rust
///
/// use const_format::{Error, Formatter};
/// use const_format::{formatc, impl_fmt, try_};
///
/// use std::ops::Range;
///
/// fn main() {
///     const LIST: &str = formatc!("{:?}", RangeList(0..5));
///     
///     assert_eq!(LIST, "[0, 1, 2, 3, 4]");
/// }
///
/// struct RangeList(Range<usize>);
///
/// impl_fmt!{
///     impl RangeList;
///     
///     const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         let mut f = f.debug_list();
///         let mut i = self.0.start;
///         while i < self.0.end {
///             try_!(f.entry().write_usize_display(i));
///             i+=1;
///         }
///         f.finish()
///     }
/// }
///
/// ```
///
pub struct DebugList<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugList<'f, 'w> {
    /// Adds a list entry to the formatted output
    pub const fn entry(&mut self) -> &mut Formatter<'w> {
        field_method_impl!(self, "[", "[\n"; len(|fmt_len|) fmt(|writer|) )
    }

    /// Finishes writing the list,
    /// and if anything went wrong in the `entry` method,returns an error.
    pub const fn finish(self) -> Result<(), Error> {
        finish_listset_method_impl!(self, "]", "[]")
    }
}

////////////////////////////////////////////////////////////////////////////////

/// For debug formatting a set.
///
/// # Example
///
/// This example demonstrates how you can debug format a custom type as a set.
///
/// ```rust
///
/// use const_format::{Error, Formatter};
/// use const_format::{formatc, impl_fmt, try_};
///
/// use std::ops::Range;
///
/// fn main() {
///     const SET: &str = formatc!("{:?}", StrSet(&["foo", "bar", "baz"]));
///     
///     assert_eq!(SET, r#"{"foo", "bar", "baz"}"#);
/// }
///
/// struct StrSet(&'static [&'static str]);
///
/// impl_fmt!{
///     impl StrSet;
///     
///     const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         let mut f = f.debug_set();
///         let mut i = 0;
///         while i < self.0.len() {
///             try_!(f.entry().write_str_debug(self.0[i]));
///             i+=1;
///         }
///         f.finish()
///     }
/// }
///
/// ```
///
pub struct DebugSet<'f, 'w> {
    fmt: &'f mut Formatter<'w>,
    wrote_field: bool,
    err: Result<(), Error>,
}

impl<'f, 'w> DebugSet<'f, 'w> {
    /// Adds a set entry to the formatted output
    pub const fn entry(&mut self) -> &mut Formatter<'w> {
        field_method_impl!(self, "{", "{\n"; len(|fmt_len|) fmt(|writer|) )
    }

    /// Finishes writing the set,
    /// and if anything went wrong in the `entry` method,returns an error.
    pub const fn finish(self) -> Result<(), Error> {
        finish_listset_method_impl!(self, "}", "{}")
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! delegate_write_methods {
    (
        shared_attrs $shared_attrs:tt
        $(
            $(#[$attrs:meta])*
            fn $method:ident($($arg:ident: $arg_ty:ty ),* $(,)* )
            length = $len:expr;
        )*
    ) => (
        impl Formatter<'_>{
            $(
                delegate_write_methods!{
                    @inner
                    shared_attrs $shared_attrs
                    $(#[$attrs])*
                    fn $method($($arg: $arg_ty ),* )
                    length = $len;
                }
            )*
        }
    );
    (
        @inner
        shared_attrs (
            $( #[$shared_attrs:meta] )*
        )
        $(#[$attrs:meta])*
        fn $method:ident($($arg:ident: $arg_ty:ty ),* $(,)* )
        length = $len:expr;
    ) => (
        $( #[$shared_attrs] )*
        $(#[$attrs])*
        pub const fn $method(&mut self, $($arg: $arg_ty ),*  ) -> Result<(), Error> {
            match &mut self.writer {
                WriterBackend::Length(fmt_len)=>{
                    fmt_len.add_len($len);
                    Ok(())
                }
                WriterBackend::Str(writer)=>{
                    writer.$method($($arg,)*)
                }
            }
        }
    )
}

delegate_write_methods! {
    shared_attrs()

    /// Writes `&string[range]` into this Formatter.
    ///
    /// This is a workaround for being unable to do `&foo[start..end]` at compile time.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_str_range("FOO BAR BAZ", 4..7);
    ///
    /// assert_eq!(writer.as_str(), "BAR");
    ///
    /// ```
    ///
    fn write_str_range(string: &str, range: Range<usize>)
    length = calculate_display_len(string.as_bytes(), &range);

    /// Writes `string` into this Formatter.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_str("FOO BAR BAZ");
    ///
    /// assert_eq!(writer.as_str(), "FOO BAR BAZ");
    ///
    /// ```
    ///
    fn write_str(string: &str)
    length = string.len();

    /// Writes `character` into this Formatter.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 4]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_char('a');
    /// let _ = fmt.write_char('b');
    /// let _ = fmt.write_char('c');
    ///
    /// assert_eq!(writer.as_str(), "abc");
    ///
    /// ```
    ///
    fn write_char(character: char)
    length = crate::char_encoding::char_display_len(character);

    /// Writes `&ascii[range]` into this formatter.
    ///
    /// This is a workaround for being unable to do `&foo[start..end]` at compile time.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter, ascii_str};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_ascii_range(ascii_str!("FOO BAR BAZ"), 4..7);
    ///
    /// assert_eq!(writer.as_str(), "BAR");
    ///
    /// ```
    ///
    fn write_ascii_range(ascii: AsciiStr<'_>, range: Range<usize>)
    length = calculate_display_len(ascii.as_bytes(), &range);

    /// Writes `ascii` into this formatter.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter, ascii_str};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_ascii(ascii_str!("FOO BAR BAZ"));
    ///
    /// assert_eq!(writer.as_str(), "FOO BAR BAZ");
    ///
    /// ```
    ///
    fn write_ascii(ascii: AsciiStr<'_>)
    length = ascii.len();

    /// Writes the ascii `character` into this formatter `repeated` times.
    ///
    /// If `character` is greater than 127,
    /// this writes `character - 128` as an ascii character.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_ascii_repeated(b'A', 10);
    ///
    /// assert_eq!(writer.as_str(), "AAAAAAAAAA");
    ///
    /// ```
    ///
    fn write_ascii_repeated(character: u8,repeated: usize)
    length = repeated;

    /// Writes `string` into this formatter, with debug formatting.
    ///
    /// This is a workaround for being unable to do `&foo[start..end]` at compile time.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_str_range_debug("FOO\nBAR\tBAZ", 3..8);
    ///
    /// assert_eq!(writer.as_str(), r#""\nBAR\t""#);
    ///
    /// ```
    ///
    fn write_str_range_debug(string: &str, range: Range<usize>)
    length = calculate_display_len_debug_range(string.as_bytes(), &range);

    /// Writes `string` into this formatter, with debug formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_str_debug("FOO\nBAR\tBAZ");
    ///
    /// assert_eq!(writer.as_str(), r#""FOO\nBAR\tBAZ""#);
    ///
    /// ```
    ///
    fn write_str_debug(string: &str)
    length = PWrapper(string.as_bytes()).compute_utf8_debug_len();

    /// Writes `character` into this Formatter, with debug formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_str(" ");
    /// let _ = fmt.write_char_debug('\\');
    /// let _ = fmt.write_str(" ");
    /// let _ = fmt.write_char_debug('A');
    /// let _ = fmt.write_str(" ");
    /// let _ = fmt.write_char_debug('0');
    /// let _ = fmt.write_str(" ");
    /// let _ = fmt.write_char_debug('\'');
    /// let _ = fmt.write_str(" ");
    ///
    /// assert_eq!(writer.as_str(), r#" '\\' 'A' '0' '\'' "#);
    ///
    /// ```
    ///
    fn write_char_debug(character: char)
    length = crate::char_encoding::char_debug_len(character);

    /// Writes `&ascii[range]` into this formatter, with debug formatting.
    ///
    /// This is a workaround for being unable to do `&foo[start..end]` at compile time.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter, ascii_str};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_ascii_range_debug(ascii_str!("FOO\nBAR\tBAZ"), 3..8);
    ///
    /// assert_eq!(writer.as_str(), r#""\nBAR\t""#);
    ///
    /// ```
    ///
    fn write_ascii_range_debug(ascii: AsciiStr<'_>,range: Range<usize>)
    length = calculate_display_len_debug_range(ascii.as_bytes(), &range);

    /// Writes `ascii` into this formatter, with debug formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter, ascii_str};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_ascii_debug(ascii_str!("FOO\nBAR\tBAZ"));
    ///
    /// assert_eq!(writer.as_str(), r#""FOO\nBAR\tBAZ""#);
    ///
    /// ```
    ///
    fn write_ascii_debug(ascii: AsciiStr<'_>)
    length = PWrapper(ascii.as_bytes()).compute_utf8_debug_len();


    /// Write `n` with display formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter, ascii_str};
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 16]);
    /// let mut fmt = writer.make_formatter(FormattingFlags::NEW);
    ///
    /// let _ = fmt.write_u8_display(13);
    /// let _ = fmt.write_u8_display(21);
    /// let _ = fmt.write_u8_display(34);
    ///
    /// assert_eq!(writer.as_str(), "132134");
    ///
    /// ```
    ///
    fn write_u8_display(n: u8)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);
}

delegate_write_methods! {
    shared_attrs(
        /// Writes `n` with display formatting
        ///
        /// For an example,
        /// you can look at the one for the [`write_u8_display`] method.
        ///
        /// [`write_u8_display`]: #method.write_u8_display
    )

    fn write_u16_display(n: u16)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u32_display(n: u32)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u64_display(n: u64)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_u128_display(n: u128)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_usize_display(n: usize)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i8_display(n: i8)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i16_display(n: i16)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i32_display(n: i32)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i64_display(n: i64)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_i128_display(n: i128)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);

    fn write_isize_display(n: isize)
    length = PWrapper(n).compute_display_len(FormattingFlags::NEW);
}

macro_rules! delegate_integer_debug_methods {
    (
        shared_attrs $shared_attrs:tt
        $(
            $(#[$attrs:meta])*
            fn $method:ident($($arg:ident: $arg_ty:ty ),* $(,)* )
            length = |$flags:ident| $len:expr;
        )*
    ) => (
        impl Formatter<'_>{
            $(
                delegate_integer_debug_methods!{
                    @inner
                    shared_attrs $shared_attrs
                    $(#[$attrs])*
                    fn $method($($arg: $arg_ty ),*)
                    length = |$flags| $len;
                }
            )*
        }
    );
    (
        @inner
        shared_attrs (
            $( #[$shared_attrs:meta] )*
        )
        $(#[$attrs:meta])*
        fn $method:ident($($arg:ident: $arg_ty:ty ),* $(,)* )
        length = |$flags:ident| $len:expr;
    ) => (
        $( #[$shared_attrs] )*
        $(#[$attrs])*
        pub const fn $method(&mut self, $($arg: $arg_ty ),*  ) -> Result<(), Error> {
            let $flags = self.flags;

            match &mut self.writer {
                WriterBackend::Length(fmt_len)=>{
                    fmt_len.add_len($len);
                    Ok(())
                }
                WriterBackend::Str(writer)=>{
                    writer.$method($($arg,)* $flags)
                }
            }
        }
    )
}

delegate_integer_debug_methods! {
    shared_attrs()

    /// Writes `n` with debug formatting.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use const_format::{Formatter, FormattingFlags, StrWriter};
    ///
    /// fn debug_fmt(writer: &mut StrWriter, flag: FormattingFlags) -> &str {
    ///     writer.clear();
    ///     let mut fmt = Formatter::from_sw(writer, flag);
    ///     let _ = fmt.write_u8_debug(63);
    ///     writer.as_str()
    /// }
    ///
    /// let reg_flag = FormattingFlags::NEW.set_alternate(false);
    /// let alt_flag = FormattingFlags::NEW.set_alternate(true);
    ///
    /// let writer: &mut StrWriter = &mut StrWriter::new([0; 64]);
    ///
    /// assert_eq!(debug_fmt(writer, reg_flag),                   "63"     );
    /// assert_eq!(debug_fmt(writer, reg_flag.set_hexadecimal()), "3F"     );
    /// assert_eq!(debug_fmt(writer, reg_flag.set_lower_hexadecimal()), "3f"     );
    /// assert_eq!(debug_fmt(writer, reg_flag.set_binary()),      "111111" );
    /// assert_eq!(debug_fmt(writer, alt_flag),                   "63"     );
    /// assert_eq!(debug_fmt(writer, alt_flag.set_hexadecimal()), "0x3F"   );
    /// assert_eq!(debug_fmt(writer, alt_flag.set_lower_hexadecimal()), "0x3f"   );
    /// assert_eq!(debug_fmt(writer, alt_flag.set_binary()),      "0b111111");
    ///
    /// ```
    ///
    fn write_u8_debug(n: u8)
    length = |flags| PWrapper(n).compute_debug_len(flags);
}

delegate_integer_debug_methods! {
    shared_attrs(
        /// Writes `n` with debug formatting.
        ///
        /// For an example,
        /// you can look at the one for the [`write_u8_debug`] method.
        ///
        /// [`write_u8_debug`]: #method.write_u8_debug
    )

    fn write_u16_debug(n: u16)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_u32_debug(n: u32)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_u64_debug(n: u64)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_u128_debug(n: u128)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_usize_debug(n: usize)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i8_debug(n: i8)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i16_debug(n: i16)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i32_debug(n: i32)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i64_debug(n: i64)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_i128_debug(n: i128)
    length = |flags| PWrapper(n).compute_debug_len(flags);

    fn write_isize_debug(n: isize)
    length = |flags| PWrapper(n).compute_debug_len(flags);
}

#[inline(always)]
const fn calculate_display_len(b: &[u8], range: &Range<usize>) -> usize {
    let Range { start, end } = saturate_range(b, range);
    end - start
}

#[inline(always)]
const fn calculate_display_len_debug_range(b: &[u8], range: &Range<usize>) -> usize {
    let Range { start, end } = saturate_range(b, range);
    PWrapper(b).compute_utf8_debug_len_in_range(start..end)
}
