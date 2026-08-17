use std::io::{Read, Result, Write};

use crate::ctx::{FromCtx, IntoCtx, SizeWith};

/// An extension trait to `std::io::Read` streams; mainly targeted at reading primitive types with
/// a known size.
///
/// Requires types to implement [`FromCtx`](ctx/trait.FromCtx.html) and [`SizeWith`](ctx/trait.SizeWith.html).
///
/// **NB** You should probably add `repr(C)` and be very careful how you implement
/// [`SizeWith`](ctx/trait.SizeWith.html), otherwise you will get IO errors failing to fill entire
/// buffer (the size you specified in `SizeWith`), or out of bound errors (depending on your impl)
/// in `from_ctx`.
///
/// Warning: Currently ioread/write uses a small 256-byte buffer and can not read/write larger types
///
/// # Example
/// ```rust
/// use std::io::Cursor;
/// use scroll::{self, ctx, LE, Pread, IOread};
///
/// #[repr(packed)]
/// struct Foo {
///     foo: i64,
///     bar: u32,
/// }
///
/// impl ctx::FromCtx<scroll::Endian> for Foo {
///     fn from_ctx(bytes: &[u8], ctx: scroll::Endian) -> Self {
///         Foo { foo: bytes.pread_with::<i64>(0, ctx).unwrap(), bar: bytes.pread_with::<u32>(8, ctx).unwrap() }
///     }
/// }
///
/// impl ctx::SizeWith<scroll::Endian> for Foo {
///     // our parsing context doesn't influence our size
///     fn size_with(_: &scroll::Endian) -> usize {
///         ::std::mem::size_of::<Foo>()
///     }
/// }
///
/// let bytes_ = [0x0b,0x0b,0x00,0x00,0x00,0x00,0x00,0x00, 0xef,0xbe,0x00,0x00,];
/// let mut bytes = Cursor::new(bytes_);
/// let foo = bytes.ioread_with::<i64>(LE).unwrap();
/// let bar = bytes.ioread_with::<u32>(LE).unwrap();
/// assert_eq!(foo, 0xb0b);
/// assert_eq!(bar, 0xbeef);
/// let error = bytes.ioread_with::<f64>(LE);
/// assert!(error.is_err());
/// let mut bytes = Cursor::new(bytes_);
/// let foo_ = bytes.ioread_with::<Foo>(LE).unwrap();
/// // Remember that you need to copy out fields from packed structs
/// // with a `{}` block instead of borrowing them directly
/// // ref: https://github.com/rust-lang/rust/issues/46043
/// assert_eq!({foo_.foo}, foo);
/// assert_eq!({foo_.bar}, bar);
/// ```
///
pub trait IOread<Ctx: Copy>: Read {
    /// Reads the type `N` from `Self`, with a default parsing context.
    /// For the primitive numeric types, this will be at the host machine's endianness.
    ///
    /// # Example
    /// ```rust
    /// use scroll::IOread;
    /// use std::io::Cursor;
    /// let bytes = [0xef, 0xbe];
    /// let mut bytes = Cursor::new(&bytes[..]);
    /// let beef = bytes.ioread::<u16>().unwrap();
    ///
    /// #[cfg(target_endian = "little")]
    /// assert_eq!(0xbeef, beef);
    /// #[cfg(target_endian = "big")]
    /// assert_eq!(0xefbe, beef);
    /// ```
    #[inline]
    fn ioread<N: FromCtx<Ctx> + SizeWith<Ctx>>(&mut self) -> Result<N>
    where
        Ctx: Default,
    {
        let ctx = Ctx::default();
        self.ioread_with(ctx)
    }

    /// Reads the type `N` from `Self`, with the parsing context `ctx`.
    /// **NB**: this will panic if the type you're reading has a size greater than 256. Plans are to have this allocate in larger cases.
    ///
    /// For the primitive numeric types, this will be at the host machine's endianness.
    ///
    /// # Example
    /// ```rust
    /// use scroll::{IOread, LE, BE};
    /// use std::io::Cursor;
    /// let bytes = [0xef, 0xbe, 0xb0, 0xb0, 0xfe, 0xed, 0xde, 0xad];
    /// let mut bytes = Cursor::new(&bytes[..]);
    /// let beef = bytes.ioread_with::<u16>(LE).unwrap();
    /// assert_eq!(0xbeef, beef);
    /// let b0 = bytes.ioread::<u8>().unwrap();
    /// assert_eq!(0xb0, b0);
    /// let b0 = bytes.ioread::<u8>().unwrap();
    /// assert_eq!(0xb0, b0);
    /// let feeddead = bytes.ioread_with::<u32>(BE).unwrap();
    /// assert_eq!(0xfeeddead, feeddead);
    /// ```
    #[inline]
    fn ioread_with<N: FromCtx<Ctx> + SizeWith<Ctx>>(&mut self, ctx: Ctx) -> Result<N> {
        let mut scratch = [0u8; 256];
        let size = N::size_with(&ctx);
        let buf = &mut scratch[0..size];
        self.read_exact(buf)?;
        Ok(N::from_ctx(buf, ctx))
    }
}

/// Types that implement `Read` get methods defined in `IOread`
/// for free.
impl<Ctx: Copy, R: Read + ?Sized> IOread<Ctx> for R {}

/// An extension trait to `std::io::Write` streams; this only serializes simple types, like `u8`, `i32`, `f32`, `usize`, etc.
///
/// To write custom types with a single `iowrite::<YourType>` call, implement [`IntoCtx`](ctx/trait.IntoCtx.html) and [`SizeWith`](ctx/trait.SizeWith.html) for `YourType`.
pub trait IOwrite<Ctx: Copy>: Write {
    /// Writes the type `N` into `Self`, with the parsing context `ctx`.
    /// **NB**: this will panic if the type you're writing has a size greater than 256. Plans are to have this allocate in larger cases.
    ///
    /// For the primitive numeric types, this will be at the host machine's endianness.
    ///
    /// # Example
    /// ```rust
    /// use scroll::IOwrite;
    /// use std::io::Cursor;
    ///
    /// let mut bytes = [0x0u8; 4];
    /// let mut bytes = Cursor::new(&mut bytes[..]);
    /// bytes.iowrite(0xdeadbeef as u32).unwrap();
    ///
    /// #[cfg(target_endian = "little")]
    /// assert_eq!(bytes.into_inner(), [0xef, 0xbe, 0xad, 0xde,]);
    /// #[cfg(target_endian = "big")]
    /// assert_eq!(bytes.into_inner(), [0xde, 0xad, 0xbe, 0xef,]);
    /// ```
    #[inline]
    fn iowrite<N: SizeWith<Ctx> + IntoCtx<Ctx>>(&mut self, n: N) -> Result<()>
    where
        Ctx: Default,
    {
        let ctx = Ctx::default();
        self.iowrite_with(n, ctx)
    }

    /// Writes the type `N` into `Self`, with the parsing context `ctx`.
    /// **NB**: this will panic if the type you're writing has a size greater than 256. Plans are to have this allocate in larger cases.
    ///
    /// For the primitive numeric types, this will be at the host machine's endianness.
    ///
    /// # Example
    /// ```rust
    /// use scroll::{IOwrite, LE, BE};
    /// use std::io::{Write, Cursor};
    ///
    /// let mut bytes = [0x0u8; 10];
    /// let mut cursor = Cursor::new(&mut bytes[..]);
    /// cursor.write_all(b"hello").unwrap();
    /// cursor.iowrite_with(0xdeadbeef as u32, BE).unwrap();
    /// assert_eq!(cursor.into_inner(), [0x68, 0x65, 0x6c, 0x6c, 0x6f, 0xde, 0xad, 0xbe, 0xef, 0x0]);
    /// ```
    #[inline]
    fn iowrite_with<N: SizeWith<Ctx> + IntoCtx<Ctx>>(&mut self, n: N, ctx: Ctx) -> Result<()> {
        let mut buf = [0u8; 256];
        let size = N::size_with(&ctx);
        let buf = &mut buf[0..size];
        n.into_ctx(buf, ctx);
        self.write_all(buf)?;
        Ok(())
    }
}

/// Types that implement `Write` get methods defined in `IOwrite`
/// for free.
impl<Ctx: Copy, W: Write + ?Sized> IOwrite<Ctx> for W {}
