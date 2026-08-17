//! Traits, helpers, and type definitions for core I/O functionality.
//!
//! The `async_std::io` module contains a number of common things you'll need
//! when doing input and output. The most core part of this module is
//! the [`Read`] and [`Write`] traits, which provide the
//! most general interface for reading and writing input and output.
//!
//! This module is an async version of [`std::io`].
//!
//! [`std::io`]: https://doc.rust-lang.org/std/io/index.html
//!
//! # Read and Write
//!
//! Because they are traits, [`Read`] and [`Write`] are implemented by a number
//! of other types, and you can implement them for your types too. As such,
//! you'll see a few different types of I/O throughout the documentation in
//! this module: [`File`]s, [`TcpStream`]s, and sometimes even [`Vec<T>`]s. For
//! example, [`Read`] adds a [`read`][`Read::read`] method, which we can use on
//! [`File`]s:
//!
//! ```no_run
//! use async_std::fs::File;
//! use async_std::prelude::*;
//!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! let mut f = File::open("foo.txt").await?;
//! let mut buffer = [0; 10];
//!
//! // read up to 10 bytes
//! let n = f.read(&mut buffer).await?;
//!
//! println!("The bytes: {:?}", &buffer[..n]);
//! #
//! # Ok(()) }) }
//! ```
//!
//! [`Read`] and [`Write`] are so important, implementors of the two traits have a
//! nickname: readers and writers. So you'll sometimes see 'a reader' instead
//! of 'a type that implements the [`Read`] trait'. Much easier!
//!
//! ## Seek and BufRead
//!
//! Beyond that, there are two important traits that are provided: [`Seek`]
//! and [`BufRead`]. Both of these build on top of a reader to control
//! how the reading happens. [`Seek`] lets you control where the next byte is
//! coming from:
//!
//! ```no_run
//! use async_std::fs::File;
//! use async_std::io::SeekFrom;
//! use async_std::prelude::*;
//!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! let mut f = File::open("foo.txt").await?;
//! let mut buffer = [0; 10];
//!
//! // skip to the last 10 bytes of the file
//! f.seek(SeekFrom::End(-10)).await?;
//!
//! // read up to 10 bytes
//! let n = f.read(&mut buffer).await?;
//!
//! println!("The bytes: {:?}", &buffer[..n]);
//! #
//! # Ok(()) }) }
//! ```
//!
//! [`BufRead`] uses an internal buffer to provide a number of other ways to read, but
//! to show it off, we'll need to talk about buffers in general. Keep reading!
//!
//! ## BufReader and BufWriter
//!
//! Byte-based interfaces are unwieldy and can be inefficient, as we'd need to be
//! making near-constant calls to the operating system. To help with this,
//! `std::io` comes with two structs, [`BufReader`] and [`BufWriter`], which wrap
//! readers and writers. The wrapper uses a buffer, reducing the number of
//! calls and providing nicer methods for accessing exactly what you want.
//!
//! For example, [`BufReader`] works with the [`BufRead`] trait to add extra
//! methods to any reader:
//!
//! ```no_run
//! use async_std::fs::File;
//! use async_std::io::BufReader;
//! use async_std::prelude::*;
//!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! let f = File::open("foo.txt").await?;
//! let mut reader = BufReader::new(f);
//! let mut buffer = String::new();
//!
//! // read a line into buffer
//! reader.read_line(&mut buffer).await?;
//!
//! println!("{}", buffer);
//! #
//! # Ok(()) }) }
//! ```
//!
//! [`BufWriter`] doesn't add any new ways of writing; it just buffers every call
//! to [`write`][`Write::write`]:
//!
//! ```no_run
//! use async_std::fs::File;
//! use async_std::io::prelude::*;
//! use async_std::io::BufWriter;
//!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! let f = File::create("foo.txt").await?;
//! {
//!     let mut writer = BufWriter::new(f);
//!
//!     // write a byte to the buffer
//!     writer.write(&[42]).await?;
//! } // the buffer is flushed once writer goes out of scope
//! //
//! #
//! # Ok(()) }) }
//! ```
//!
//! ## Standard input and output
//!
//! A very common source of input is standard input:
//!
//! ```no_run
//! use async_std::io;
//!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! let mut input = String::new();
//!
//! io::stdin().read_line(&mut input).await?;
//!
//! println!("You typed: {}", input.trim());
//! #
//! # Ok(()) }) }
//! ```
//!
//! Note that you cannot use the [`?` operator] in functions that do not return
//! a [`Result<T, E>`][`Result`]. Instead, you can call [`.unwrap()`]
//! or `match` on the return value to catch any possible errors:
//!
//! ```no_run
//! use async_std::io;
//!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! let mut input = String::new();
//!
//! io::stdin().read_line(&mut input).await.unwrap();
//! #
//! # Ok(()) }) }
//! ```
//!
//! And a very common source of output is standard output:
//!
//! ```no_run
//! use async_std::io;
//! use async_std::io::prelude::*;
//!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! io::stdout().write(&[42]).await?;
//! #
//! # Ok(()) }) }
//! ```
//!
//! Of course, using [`io::stdout`] directly is less common than something like
//! [`println!`].
//!
//! ## Iterator types
//!
//! A large number of the structures provided by `std::io` are for various
//! ways of iterating over I/O. For example, [`Lines`] is used to split over
//! lines:
//!
//! ```no_run
//! use async_std::fs::File;
//! use async_std::io::BufReader;
//! use async_std::prelude::*;
//!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! let f = File::open("foo.txt").await?;
//! let reader = BufReader::new(f);
//!
//! let mut lines = reader.lines();
//! while let Some(line) = lines.next().await {
//!     println!("{}", line?);
//! }
//! #
//! # Ok(()) }) }
//! ```
//!
//! ## Functions
//!
//! There are a number of [functions][functions-list] that offer access to various
//! features. For example, we can use three of these functions to copy everything
//! from standard input to standard output:
//!
//! ```no_run
//! use async_std::io;
//!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! io::copy(&mut io::stdin(), &mut io::stdout()).await?;
//! #
//! # Ok(()) }) }
//! ```
//!
//! [functions-list]: #functions-1
//!
//! ## io::Result
//!
//! Last, but certainly not least, is [`io::Result`]. This type is used
//! as the return type of many `std::io` functions that can cause an error, and
//! can be returned from your own functions as well. Many of the examples in this
//! module use the [`?` operator]:
//!
//! ```
//! #![allow(dead_code)]
//! use async_std::io;
//!
//! async fn read_input() -> io::Result<()> {
//!     let mut input = String::new();
//!
//!     io::stdin().read_line(&mut input).await?;
//!
//!     println!("You typed: {}", input.trim());
//!
//!     Ok(())
//! }
//! ```
//!
//! The return type of `read_input`, [`io::Result<()>`][`io::Result`], is a very
//! common type for functions which don't have a 'real' return value, but do want to
//! return errors if they happen. In this case, the only purpose of this function is
//! to read the line and print it, so we use `()`.
//!
//! ## Platform-specific behavior
//!
//! Many I/O functions throughout the standard library are documented to indicate
//! what various library or syscalls they are delegated to. This is done to help
//! applications both understand what's happening under the hood as well as investigate
//! any possibly unclear semantics. Note, however, that this is informative, not a binding
//! contract. The implementation of many of these functions are subject to change over
//! time and may call fewer or more syscalls/library functions.
//!
//! [`Read`]: trait.Read.html
//! [`Write`]: trait.Write.html
//! [`Seek`]: trait.Seek.html
//! [`BufRead`]: trait.BufRead.html
//! [`File`]: ../fs/struct.File.html
//! [`TcpStream`]: ../net/struct.TcpStream.html
//! [`Vec<T>`]: ../vec/struct.Vec.html
//! [`BufReader`]: struct.BufReader.html
//! [`BufWriter`]: struct.BufWriter.html
//! [`Write::write`]: trait.Write.html#tymethod.write
//! [`io::stdout`]: fn.stdout.html
//! [`println!`]: ../macro.println.html
//! [`Lines`]: struct.Lines.html
//! [`io::Result`]: type.Result.html
//! [`?` operator]: https://doc.rust-lang.org/stable/book/appendix-02-operators.html
//! [`Read::read`]: trait.Read.html#tymethod.read
//! [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
//! [`.unwrap()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

cfg_std! {
    #[doc(inline)]
    pub use std::io::{Error, ErrorKind, IoSlice, IoSliceMut, Result, SeekFrom};

    pub use buf_read::*;
    pub use buf_reader::BufReader;
    pub use buf_writer::{BufWriter, IntoInnerError};
    pub use copy::copy;
    pub use cursor::Cursor;
    pub use empty::{empty, Empty};
    pub use read::*;
    pub use repeat::{repeat, Repeat};
    pub use seek::*;
    pub use sink::{sink, Sink};
    pub use write::*;

    pub mod prelude;

    pub(crate) mod buf_read;
    pub(crate) mod read;
    pub(crate) mod seek;
    pub(crate) mod write;
    pub(crate) mod utils;

    mod buf_reader;
    mod buf_writer;
    mod copy;
    mod cursor;
    mod empty;
    mod repeat;
    mod sink;
}

cfg_default! {
    // For use in the print macros.
    #[doc(hidden)]
    #[cfg(not(target_os = "unknown"))]
    pub use stdio::{_eprint, _print};

    #[cfg(not(target_os = "unknown"))]
    pub use stderr::{stderr, Stderr};
    #[cfg(not(target_os = "unknown"))]
    pub use stdin::{stdin, Stdin};
    #[cfg(not(target_os = "unknown"))]
    pub use stdout::{stdout, Stdout};
    pub use timeout::timeout;

    mod timeout;
    #[cfg(not(target_os = "unknown"))]
    mod stderr;
    #[cfg(not(target_os = "unknown"))]
    mod stdin;
    #[cfg(not(target_os = "unknown"))]
    mod stdio;
    #[cfg(not(target_os = "unknown"))]
    mod stdout;
}
