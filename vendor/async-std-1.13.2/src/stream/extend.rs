use core::pin::Pin;
use core::future::Future;

use crate::stream::IntoStream;

/// Extends a collection with the contents of a stream.
///
/// Streams produce a series of values asynchronously, and collections can also be thought of as a
/// series of values. The `Extend` trait bridges this gap, allowing you to extend a collection
/// asynchronously by including the contents of that stream. When extending a collection with an
/// already existing key, that entry is updated or, in the case of collections that permit multiple
/// entries with equal keys, that entry is inserted.
///
/// ## Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut v: Vec<usize> = vec![1, 2];
/// let s = stream::repeat(3usize).take(3);
/// stream::Extend::extend(&mut v, s).await;
///
/// assert_eq!(v, vec![1, 2, 3, 3, 3]);
/// #
/// # })
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
pub trait Extend<A> {
    /// Extends a collection with the contents of a stream.
    fn extend<'a, T: IntoStream<Item = A> + 'a>(
        &'a mut self,
        stream: T,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>
    where
        <T as IntoStream>::IntoStream: Send;
}

/// Extends a collection with the contents of a stream.
///
/// Streams produce a series of values asynchronously, and collections can also be thought of as a
/// series of values. The [`Extend`] trait bridges this gap, allowing you to extend a collection
/// asynchronously by including the contents of that stream. When extending a collection with an
/// already existing key, that entry is updated or, in the case of collections that permit multiple
/// entries with equal keys, that entry is inserted.
///
/// [`Extend`]: trait.Extend.html
///
/// ## Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut v: Vec<usize> = vec![1, 2];
/// let s = stream::repeat(3usize).take(3);
/// stream::extend(&mut v, s).await;
///
/// assert_eq!(v, vec![1, 2, 3, 3, 3]);
/// #
/// # })
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
pub async fn extend<'a, C, T, S>(collection: &mut C, stream: S)
where
    C: Extend<T>,
    S: IntoStream<Item = T> + 'a,
    <S as IntoStream>::IntoStream: Send,
{
    Extend::extend(collection, stream).await
}
