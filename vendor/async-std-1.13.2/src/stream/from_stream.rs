use core::future::Future;
use core::pin::Pin;

use crate::stream::IntoStream;

/// Conversion from a `Stream`.
///
/// By implementing `FromStream` for a type, you define how it will be created from a stream.
/// This is common for types which describe a collection of some kind.
///
/// See also: [`IntoStream`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream::{self, FromStream};
///
/// let five_fives = stream::repeat(5).take(5);
///
/// let v = Vec::from_stream(five_fives).await;
///
/// assert_eq!(v, vec![5, 5, 5, 5, 5]);
/// #
/// # Ok(()) }) }
/// ```
///
/// Using `collect` to  implicitly use `FromStream`
///
/// ```
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
/// let five_fives = stream::repeat(5).take(5);
///
/// let v: Vec<i32> = five_fives.collect().await;
///
/// assert_eq!(v, vec![5, 5, 5, 5, 5]);
/// #
/// # Ok(()) }) }
/// ```
///
/// Implementing `FromStream` for your type:
///
/// ```
/// use async_std::prelude::*;
/// use async_std::stream::{self, FromStream, IntoStream};
/// use std::pin::Pin;
///
/// // A sample collection, that's just a wrapper over Vec<T>
/// #[derive(Debug)]
/// struct MyCollection(Vec<i32>);
///
/// // Let's give it some methods so we can create one and add things
/// // to it.
/// impl MyCollection {
///     fn new() -> MyCollection {
///         MyCollection(Vec::new())
///     }
///
///     fn add(&mut self, elem: i32) {
///         self.0.push(elem);
///     }
/// }
///
/// // and we'll implement FromIterator
/// impl FromStream<i32> for MyCollection {
///     fn from_stream<'a, S: IntoStream<Item = i32> + 'a>(
///         stream: S,
///     ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>>
///    where
///        <S as IntoStream>::IntoStream: Send,
///    {
///         let stream = stream.into_stream();
///
///         Box::pin(async move {
///             let mut c = MyCollection::new();
///
///             let mut v = vec![];
///             stream::extend(&mut v, stream).await;
///
///             for i in v {
///                 c.add(i);
///             }
///             c
///         })
///     }
/// }
///
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// // Now we can make a new stream...
/// let stream = stream::repeat(5).take(5);
///
/// // ...and make a MyCollection out of it
/// let c = MyCollection::from_stream(stream).await;
///
/// assert_eq!(c.0, vec![5, 5, 5, 5, 5]);
///
/// // collect works too!
///
/// let stream = stream::repeat(5).take(5);
/// let c: MyCollection = stream.collect().await;
///
/// assert_eq!(c.0, vec![5, 5, 5, 5, 5]);
/// #
/// # Ok(()) }) }
/// ```
///
/// [`IntoStream`]: trait.IntoStream.html
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
pub trait FromStream<T: Send> {
    /// Creates a value from a stream.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::stream::{self, FromStream};
    ///
    /// let five_fives = stream::repeat(5).take(5);
    ///
    /// let v = Vec::from_stream(five_fives).await;
    ///
    /// assert_eq!(v, vec![5, 5, 5, 5, 5]);
    /// #
    /// # Ok(()) }) }
    /// ```
    fn from_stream<'a, S: IntoStream<Item = T> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>>
    where
        <S as IntoStream>::IntoStream: Send;
}
