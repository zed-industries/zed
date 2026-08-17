//! Composable asynchronous iteration.
//!
//! This module is an async version of [`std::iter`].
//!
//! If you've found yourself with an asynchronous collection of some kind,
//! and needed to perform an operation on the elements of said collection,
//! you'll quickly run into 'streams'. Streams are heavily used in idiomatic
//! asynchronous Rust code, so it's worth becoming familiar with them.
//!
//! Before explaining more, let's talk about how this module is structured:
//!
//! # Organization
//!
//! This module is largely organized by type:
//!
//! * [Traits] are the core portion: these traits define what kind of streams
//!   exist and what you can do with them. The methods of these traits are worth
//!   putting some extra study time into.
//! * [Functions] provide some helpful ways to create some basic streams.
//! * [Structs] are often the return types of the various methods on this
//!   module's traits. You'll usually want to look at the method that creates
//!   the `struct`, rather than the `struct` itself. For more detail about why,
//!   see '[Implementing Stream](#implementing-stream)'.
//!
//! [Traits]: #traits
//! [Functions]: #functions
//! [Structs]: #structs
//!
//! That's it! Let's dig into streams.
//!
//! # Stream
//!
//! The heart and soul of this module is the [`Stream`] trait. The core of
//! [`Stream`] looks like this:
//!
//! ```
//! #![allow(dead_code)]
//! # use async_std::task::{Context, Poll};
//! # use std::pin::Pin;
//! pub trait Stream {
//!     type Item;
//!     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
//! }
//! # impl Stream for () {
//! #   type Item = ();
//! #   fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> { Poll::Pending }
//! # }
//! ```
//!
//! A stream has a method, [`next`], which when called, returns an
//! [`Poll`]<[`Option`]`<Item>>`. [`next`] will return `Ready(Some(Item))`
//! as long as there are elements, and once they've all been exhausted, will
//! return `None` to indicate that iteration is finished. If we're waiting on
//! something asynchronous to resolve `Pending` is returned.
//!
//! Individual streams may choose to resume iteration, and so calling
//! [`next`] again may or may not eventually start returning `Ready(Some(Item))`
//! again at some point.
//!
//! [`Stream`]'s full definition includes a number of other methods as well,
//! but they are default methods, built on top of [`next`], and so you get
//! them for free.
//!
//! Streams are also composable, and it's common to chain them together to do
//! more complex forms of processing. See the [Adapters](#adapters) section
//! below for more details.
//!
//! [`Poll`]: ../task/enum.Poll.html
//! [`Stream`]: trait.Stream.html
//! [`next`]: trait.Stream.html#tymethod.next
//! [`Option`]: ../../std/option/enum.Option.html
//!
//! # The three forms of streaming
//!
//! There are three common methods which can create streams from a collection:
//!
//! * `stream()`, which iterates over `&T`.
//! * `stream_mut()`, which iterates over `&mut T`.
//! * `into_stream()`, which iterates over `T`.
//!
//! Various things in async-std may implement one or more of the
//! three, where appropriate.
//!
//! # Implementing Stream
//!
//! Creating a stream of your own involves two steps: creating a `struct` to
//! hold the stream's state, and then `impl`ementing [`Stream`] for that
//! `struct`. This is why there are so many `struct`s in this module: there is
//! one for each stream and iterator adapter.
//!
//! Let's make a stream named `Counter` which counts from `1` to `5`:
//!
//! ```
//! # use async_std::prelude::*;
//! # use async_std::task::{Context, Poll};
//! # use std::pin::Pin;
//! // First, the struct:
//!
//! /// A stream which counts from one to five
//! struct Counter {
//!     count: usize,
//! }
//!
//! // we want our count to start at one, so let's add a new() method to help.
//! // This isn't strictly necessary, but is convenient. Note that we start
//! // `count` at zero, we'll see why in `next()`'s implementation below.
//! impl Counter {
//!     fn new() -> Counter {
//!         Counter { count: 0 }
//!     }
//! }
//!
//! // Then, we implement `Stream` for our `Counter`:
//!
//! impl Stream for Counter {
//!     // we will be counting with usize
//!     type Item = usize;
//!
//!     // poll_next() is the only required method
//!     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//!         // Increment our count. This is why we started at zero.
//!         self.count += 1;
//!
//!         // Check to see if we've finished counting or not.
//!         if self.count < 6 {
//!             Poll::Ready(Some(self.count))
//!         } else {
//!             Poll::Ready(None)
//!         }
//!     }
//! }
//!
//! // And now we can use it!
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! let mut counter = Counter::new();
//!
//! let x = counter.next().await.unwrap();
//! println!("{}", x);
//!
//! let x = counter.next().await.unwrap();
//! println!("{}", x);
//!
//! let x = counter.next().await.unwrap();
//! println!("{}", x);
//!
//! let x = counter.next().await.unwrap();
//! println!("{}", x);
//!
//! let x = counter.next().await.unwrap();
//! println!("{}", x);
//! #
//! # Ok(()) }) }
//! ```
//!
//! This will print `1` through `5`, each on their own line.
//!
//! Calling `next().await` this way gets repetitive. Rust has a construct which
//! can call `next()` on your stream, until it reaches `None`. Let's go over
//! that next.
//!
//! # while let Loops and IntoStream
//!
//! Rust's `while let` loop syntax is an idiomatic way to iterate over streams. Here's a basic
//! example of `while let`:
//!
//! ```
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! # use async_std::prelude::*;
//! # use async_std::stream;
//! let mut values = stream::from_iter(1u8..6);
//!
//! while let Some(x) = values.next().await {
//!     println!("{}", x);
//! }
//! #
//! # Ok(()) }) }
//! ```
//!
//! This will print the numbers one through five, each on their own line. But
//! you'll notice something here: we never called anything on our vector to
//! produce a stream. What gives?
//!
//! There's a trait in the standard library for converting something into an
//! stream: [`IntoStream`]. This trait has one method, [`into_stream`],
//! which converts the thing implementing [`IntoStream`] into a stream.
//!
//! Unlike `std::iter::IntoIterator`, `IntoStream` does not have compiler
//! support yet. This means that automatic conversions like with `for` loops
//! doesn't occur yet, and `into_stream` or `from_iter` as above will always
//! have to be called manually.
//!
//! [`IntoStream`]: trait.IntoStream.html
//! [`into_stream`]: trait.IntoStream.html#tymethod.into_stream
//!
//! # Adapters
//!
//! Functions which take an [`Stream`] and return another [`Stream`] are
//! often called 'stream adapters', as they are a form of the 'adapter
//! pattern'.
//!
//! Common stream adapters include [`map`], [`take`], and [`filter`].
//! For more, see their documentation.
//!
//! [`map`]: trait.Stream.html#method.map
//! [`take`]: trait.Stream.html#method.take
//! [`filter`]: trait.Stream.html#method.filter
//!
//! # Laziness
//!
//! Streams (and stream [adapters](#adapters)) are *lazy*. This means that
//! just creating a stream doesn't _do_ a whole lot. Nothing really happens
//! until you call [`next`]. This is sometimes a source of confusion when
//! creating a stream solely for its side effects. For example, the [`map`]
//! method calls a closure on each element it iterates over:
//!
//! ```
//! # #![allow(unused_must_use)]
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! # use async_std::prelude::*;
//! # use async_std::stream;
//! let v = stream::repeat(1u8).take(5);
//! v.map(|x| println!("{}", x));
//! #
//! # Ok(()) }) }
//! ```
//!
//! This will not print any values, as we only created a stream, rather than
//! using it. The compiler will warn us about this kind of behavior:
//!
//! ```text
//! warning: unused result that must be used: streams are lazy and
//! do nothing unless consumed
//! ```
//!
//! The idiomatic way to write a [`map`] for its side effects is to use a
//! `while let` loop instead:
//!
//! ```
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! # use async_std::prelude::*;
//! # use async_std::stream;
//! let mut v = stream::repeat(1u8).take(5);
//!
//! while let Some(x) = &v.next().await {
//!     println!("{}", x);
//! }
//! #
//! # Ok(()) }) }
//! ```
//!
//! [`map`]: trait.Stream.html#method.map
//!
//! The two most common ways to evaluate a stream are to use a `while let` loop
//! like this, or using the [`collect`] method to produce a new collection.
//!
//! [`collect`]: trait.Stream.html#method.collect
//!
//! # Infinity
//!
//! Streams do not have to be finite. As an example, a repeat stream is
//! an infinite stream:
//!
//! ```
//! # use async_std::stream;
//! let numbers = stream::repeat(1u8);
//! ```
//!
//! It is common to use the [`take`] stream adapter to turn an infinite
//! stream into a finite one:
//!
//! ```
//! # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
//! #
//! # use async_std::prelude::*;
//! # use async_std::stream;
//! let numbers = stream::from_iter(0u8..);
//! let mut five_numbers = numbers.take(5);
//!
//! while let Some(number) = five_numbers.next().await {
//!     println!("{}", number);
//! }
//! #
//! # Ok(()) }) }
//! ```
//!
//! This will print the numbers `0` through `4`, each on their own line.
//!
//! Bear in mind that methods on infinite streams, even those for which a
//! result can be determined mathematically in finite time, may not terminate.
//! Specifically, methods such as [`min`], which in the general case require
//! traversing every element in the stream, are likely not to return
//! successfully for any infinite streams.
//!
//! ```ignore
//! let ones = async_std::stream::repeat(1);
//! let least = ones.min().await.unwrap(); // Oh no! An infinite loop!
//! // `ones.min()` causes an infinite loop, so we won't reach this point!
//! println!("The smallest number one is {}.", least);
//! ```
//!
//! [`std::iter`]: https://doc.rust-lang.org/std/iter/index.html
//! [`take`]: trait.Stream.html#method.take
//! [`min`]: trait.Stream.html#method.min

pub use empty::{empty, Empty};
pub use from_fn::{from_fn, FromFn};
pub use from_iter::{from_iter, FromIter};
pub use once::{once, Once};
pub use repeat::{repeat, Repeat};
pub use repeat_with::{repeat_with, RepeatWith};
pub use stream::*;

pub(crate) mod stream;

mod empty;
mod from_fn;
mod from_iter;
mod once;
mod repeat;
mod repeat_with;

cfg_unstable! {
    mod double_ended_stream;
    mod exact_size_stream;
    mod extend;
    mod from_stream;
    mod fused_stream;
    mod interval;
    mod into_stream;
    mod pending;
    mod product;
    mod successors;
    mod sum;

    pub use double_ended_stream::DoubleEndedStream;
    pub use exact_size_stream::ExactSizeStream;
    pub use extend::{extend, Extend};
    pub use from_stream::FromStream;
    pub use fused_stream::FusedStream;
    pub use interval::{interval, Interval};
    pub use into_stream::IntoStream;
    pub use pending::{pending, Pending};
    pub use product::Product;
    pub use stream::Merge;
    pub use successors::{successors, Successors};
    pub use sum::Sum;
}
