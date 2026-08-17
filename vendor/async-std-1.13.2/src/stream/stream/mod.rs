//! Asynchronous iteration.
//!
//! This module is an async version of [`std::iter`].
//!
//! [`std::iter`]: https://doc.rust-lang.org/std/iter/index.html
//!
//! # Examples
//!
//! ```
//! # async_std::task::block_on(async {
//! #
//! use async_std::prelude::*;
//! use async_std::stream;
//!
//! let mut s = stream::repeat(9).take(3);
//!
//! while let Some(v) = s.next().await {
//!     assert_eq!(v, 9);
//! }
//! #
//! # })
//! ```

mod all;
mod any;
mod chain;
mod cloned;
mod cmp;
mod copied;
mod cycle;
mod enumerate;
mod eq;
mod filter;
mod filter_map;
mod find;
mod find_map;
mod fold;
mod for_each;
mod fuse;
mod ge;
mod gt;
mod inspect;
mod last;
mod le;
mod lt;
mod map;
mod max;
mod max_by;
mod max_by_key;
mod min;
mod min_by;
mod min_by_key;
mod ne;
mod next;
mod nth;
mod partial_cmp;
mod position;
mod scan;
mod skip;
mod skip_while;
mod step_by;
mod take;
mod take_while;
mod try_fold;
mod try_for_each;
mod zip;

use all::AllFuture;
use any::AnyFuture;
use cmp::CmpFuture;
use cycle::Cycle;
use enumerate::Enumerate;
use eq::EqFuture;
use filter_map::FilterMap;
use find::FindFuture;
use find_map::FindMapFuture;
use fold::FoldFuture;
use for_each::ForEachFuture;
use ge::GeFuture;
use gt::GtFuture;
use last::LastFuture;
use le::LeFuture;
use lt::LtFuture;
use max::MaxFuture;
use max_by::MaxByFuture;
use max_by_key::MaxByKeyFuture;
use min::MinFuture;
use min_by::MinByFuture;
use min_by_key::MinByKeyFuture;
use ne::NeFuture;
use next::NextFuture;
use nth::NthFuture;
use partial_cmp::PartialCmpFuture;
use position::PositionFuture;
use try_fold::TryFoldFuture;
use try_for_each::TryForEachFuture;

pub use chain::Chain;
pub use cloned::Cloned;
pub use copied::Copied;
pub use filter::Filter;
pub use fuse::Fuse;
pub use inspect::Inspect;
pub use map::Map;
pub use scan::Scan;
pub use skip::Skip;
pub use skip_while::SkipWhile;
pub use step_by::StepBy;
pub use take::Take;
pub use take_while::TakeWhile;
pub use zip::Zip;

use core::cmp::Ordering;

cfg_unstable! {
    use core::future::Future;
    use core::pin::Pin;
    use core::time::Duration;

    use crate::stream::into_stream::IntoStream;
    use crate::stream::{FromStream, Product, Sum};
    use crate::stream::Extend;

    use count::CountFuture;
    use partition::PartitionFuture;
    use unzip::UnzipFuture;

    pub use merge::Merge;
    pub use flatten::Flatten;
    pub use flat_map::FlatMap;
    pub use timeout::{TimeoutError, Timeout};
    pub use throttle::Throttle;
    pub use delay::Delay;

    mod count;
    mod merge;
    mod flatten;
    mod flat_map;
    mod partition;
    mod timeout;
    mod throttle;
    mod delay;
    mod unzip;
}

pub use futures_core::stream::Stream as Stream;

#[doc = r#"
    Extension methods for [`Stream`].

    [`Stream`]: ../stream/trait.Stream.html
"#]
pub trait StreamExt: Stream {
    #[doc = r#"
        Advances the stream and returns the next value.

        Returns [`None`] when iteration is finished. Individual stream implementations may
        choose to resume iteration, and so calling `next()` again may or may not eventually
        start returning more values.

        [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::once(7);

        assert_eq!(s.next().await, Some(7));
        assert_eq!(s.next().await, None);
        #
        # }) }
        ```
    "#]
    fn next(&mut self) -> NextFuture<'_, Self>
    where
        Self: Unpin,
    {
        NextFuture { stream: self }
    }

    #[doc = r#"
        Creates a stream that yields its first `n` elements.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::repeat(9).take(3);

        while let Some(v) = s.next().await {
            assert_eq!(v, 9);
        }
        #
        # }) }
        ```
    "#]
    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }

    #[doc = r#"
        Creates a stream that yields elements based on a predicate.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1, 2, 3, 4]);
        let mut s = s.take_while(|x| x < &3 );

        assert_eq!(s.next().await, Some(1));
        assert_eq!(s.next().await, Some(2));
        assert_eq!(s.next().await, None);
        #
        # }) }
        ```
    "#]
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        TakeWhile::new(self, predicate)
    }

    #[doc = r#"
        Limit the amount of items yielded per timeslice in a stream.

        This stream does not drop any items, but will only limit the rate at which items pass through.
        # Examples
        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;
        use std::time::{Duration, Instant};

        let start = Instant::now();

        // emit value every 5 milliseconds
        let s = stream::interval(Duration::from_millis(5)).take(2);

        // throttle for 10 milliseconds
        let mut s = s.throttle(Duration::from_millis(10));

        s.next().await;
        assert!(start.elapsed().as_millis() >= 5);

        s.next().await;
        assert!(start.elapsed().as_millis() >= 15);

        s.next().await;
        assert!(start.elapsed().as_millis() >= 25);
        #
        # }) }
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn throttle(self, d: Duration) -> Throttle<Self>
    where
        Self: Sized,
    {
        Throttle::new(self, d)
    }

    #[doc = r#"
        Creates a stream that yields each `step`th element.

        # Panics

        This method will panic if the given step is `0`.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![0u8, 1, 2, 3, 4]);
        let mut stepped = s.step_by(2);

        assert_eq!(stepped.next().await, Some(0));
        assert_eq!(stepped.next().await, Some(2));
        assert_eq!(stepped.next().await, Some(4));
        assert_eq!(stepped.next().await, None);

        #
        # }) }
        ```
    "#]
    fn step_by(self, step: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        StepBy::new(self, step)
    }

    #[doc = r#"
        Takes two streams and creates a new stream over both in sequence.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let first = stream::from_iter(vec![0u8, 1]);
        let second = stream::from_iter(vec![2, 3]);
        let mut c = first.chain(second);

        assert_eq!(c.next().await, Some(0));
        assert_eq!(c.next().await, Some(1));
        assert_eq!(c.next().await, Some(2));
        assert_eq!(c.next().await, Some(3));
        assert_eq!(c.next().await, None);

        #
        # }) }
        ```
    "#]
    fn chain<U>(self, other: U) -> Chain<Self, U>
    where
        Self: Sized,
        U: Stream<Item = Self::Item> + Sized,
    {
        Chain::new(self, other)
    }

        #[doc = r#"
        Creates an stream which copies all of its elements.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let v = stream::from_iter(vec![&1, &2, &3]);

        let mut v_cloned = v.cloned();

        assert_eq!(v_cloned.next().await, Some(1));
        assert_eq!(v_cloned.next().await, Some(2));
        assert_eq!(v_cloned.next().await, Some(3));
        assert_eq!(v_cloned.next().await, None);

        #
        # }) }
        ```
    "#]
    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        Self: Sized + Stream<Item = &'a T>,
        T: Clone + 'a,
    {
        Cloned::new(self)
    }


    #[doc = r#"
        Creates an stream which copies all of its elements.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![&1, &2, &3]);
        let mut s_copied  = s.copied();

        assert_eq!(s_copied.next().await, Some(1));
        assert_eq!(s_copied.next().await, Some(2));
        assert_eq!(s_copied.next().await, Some(3));
        assert_eq!(s_copied.next().await, None);
        #
        # }) }
        ```
    "#]
    fn copied<'a, T>(self) -> Copied<Self>
    where
        Self: Sized + Stream<Item = &'a T>,
        T: Copy + 'a,
    {
        Copied::new(self)
    }

    #[doc = r#"
        Creates a stream that yields the provided values infinitely and in order.

        # Examples

        Basic usage:

        ```
        # async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::once(7).cycle();

        assert_eq!(s.next().await, Some(7));
        assert_eq!(s.next().await, Some(7));
        assert_eq!(s.next().await, Some(7));
        assert_eq!(s.next().await, Some(7));
        assert_eq!(s.next().await, Some(7));
        #
        # })
        ```
    "#]
    fn cycle(self) -> Cycle<Self>
    where
        Self: Clone + Sized,
    {
        Cycle::new(self)
    }

    #[doc = r#"
        Creates a stream that gives the current element's count as well as the next value.

        # Overflow behaviour.

        This combinator does no guarding against overflows.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec!['a', 'b', 'c']);
        let mut s = s.enumerate();

        assert_eq!(s.next().await, Some((0, 'a')));
        assert_eq!(s.next().await, Some((1, 'b')));
        assert_eq!(s.next().await, Some((2, 'c')));
        assert_eq!(s.next().await, None);
        #
        # }) }
        ```
    "#]
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }

    #[doc = r#"
        Creates a stream that is delayed before it starts yielding items.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;
        use std::time::{Duration, Instant};

        let start = Instant::now();
        let mut s = stream::from_iter(vec![0u8, 1, 2]).delay(Duration::from_millis(200));

        assert_eq!(s.next().await, Some(0));
        // The first time will take more than 200ms due to delay.
        assert!(start.elapsed().as_millis() >= 200);

        assert_eq!(s.next().await, Some(1));
        // There will be no delay after the first time.
        assert!(start.elapsed().as_millis() < 400);

        assert_eq!(s.next().await, Some(2));
        assert!(start.elapsed().as_millis() < 400);

        assert_eq!(s.next().await, None);
        assert!(start.elapsed().as_millis() < 400);
        #
        # }) }
        ```
    "#]
    #[cfg(any(feature = "unstable", feature = "docs"))]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn delay(self, dur: std::time::Duration) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, dur)
    }

    #[doc = r#"
        Takes a closure and creates a stream that calls that closure on every element of this stream.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1, 2, 3]);
        let mut s = s.map(|x| 2 * x);

        assert_eq!(s.next().await, Some(2));
        assert_eq!(s.next().await, Some(4));
        assert_eq!(s.next().await, Some(6));
        assert_eq!(s.next().await, None);

        #
        # }) }
        ```
    "#]
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }

    #[doc = r#"
        A combinator that does something with each element in the stream, passing the value
        on.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1, 2, 3, 4, 5]);

        let sum = s
           .inspect(|x| println!("about to filter {}", x))
           .filter(|x| x % 2 == 0)
           .inspect(|x| println!("made it through filter: {}", x))
           .fold(0, |sum, i| sum + i)
           .await;

        assert_eq!(sum, 6);
        #
        # }) }
        ```
    "#]
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item),
    {
        Inspect::new(self, f)
    }

    #[doc = r#"
        Returns the last element of the stream.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1, 2, 3]);

        let last  = s.last().await;
        assert_eq!(last, Some(3));
        #
        # }) }
        ```

        An empty stream will return `None`:
        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::stream;
        use crate::async_std::prelude::*;

        let s = stream::empty::<()>();

        let last  = s.last().await;
        assert_eq!(last, None);
        #
        # }) }
        ```
    "#]
    fn last(
        self,
    ) -> LastFuture<Self, Self::Item>
    where
        Self: Sized,
    {
        LastFuture::new(self)
    }

    #[doc = r#"
        Creates a stream which ends after the first `None`.

        After a stream returns `None`, future calls may or may not yield `Some(T)` again.
        `fuse()` adapts an iterator, ensuring that after a `None` is given, it will always
        return `None` forever.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::once(1).fuse();
        assert_eq!(s.next().await, Some(1));
        assert_eq!(s.next().await, None);
        assert_eq!(s.next().await, None);
        #
        # }) }
        ```
    "#]
    fn fuse(self) -> Fuse<Self>
    where
        Self: Sized,
    {
        Fuse::new(self)
    }

    #[doc = r#"
        Creates a stream that uses a predicate to determine if an element should be yielded.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1, 2, 3, 4]);
        let mut s = s.filter(|i| i % 2 == 0);

        assert_eq!(s.next().await, Some(2));
        assert_eq!(s.next().await, Some(4));
        assert_eq!(s.next().await, None);
        #
        # }) }
        ```
    "#]
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        Filter::new(self, predicate)
    }

    #[doc= r#"
        Creates an stream that works like map, but flattens nested structure.

        # Examples

        Basic usage:

        ```
        # async_std::task::block_on(async {

        use async_std::prelude::*;
        use async_std::stream;

        let words = stream::from_iter(&["alpha", "beta", "gamma"]);

        let merged: String = words
            .flat_map(|s| stream::from_iter(s.chars()))
            .collect().await;
            assert_eq!(merged, "alphabetagamma");

        let d3 = stream::from_iter(&[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]);
        let d1: Vec<_> = d3
            .flat_map(|item| stream::from_iter(item))
            .flat_map(|item| stream::from_iter(item))
            .collect().await;

        assert_eq!(d1, [&1, &2, &3, &4, &5, &6, &7, &8]);
        # });
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
        where
            Self: Sized,
            U: IntoStream,
            F: FnMut(Self::Item) -> U,
    {
        FlatMap::new(self, f)
    }

    #[doc = r#"
        Creates an stream that flattens nested structure.

        # Examples

        Basic usage:

        ```
        # async_std::task::block_on(async {

        use async_std::prelude::*;
        use async_std::stream;

        let inner1 = stream::from_iter(vec![1u8,2,3]);
        let inner2 = stream::from_iter(vec![4u8,5,6]);
        let s  = stream::from_iter(vec![inner1, inner2]);

        let v: Vec<_> = s.flatten().collect().await;

        assert_eq!(v, vec![1,2,3,4,5,6]);

        # });
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Item: IntoStream,
    {
        Flatten::new(self)
    }

    #[doc = r#"
        Both filters and maps a stream.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #

        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec!["1", "lol", "3", "NaN", "5"]);

        let mut parsed = s.filter_map(|a| a.parse::<u32>().ok());

        let one = parsed.next().await;
        assert_eq!(one, Some(1));

        let three = parsed.next().await;
        assert_eq!(three, Some(3));

        let five = parsed.next().await;
        assert_eq!(five, Some(5));

        let end = parsed.next().await;
        assert_eq!(end, None);
        #
        # }) }
        ```
    "#]
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        FilterMap::new(self, f)
    }

    #[doc = r#"
        Returns the element that gives the minimum value with respect to the
        specified key function. If several elements are equally minimum,
        the first element is returned. If the stream is empty, `None` is returned.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![-1isize, 2, -3]);

        let min = s.clone().min_by_key(|x| x.abs()).await;
        assert_eq!(min, Some(-1));

        let min = stream::empty::<isize>().min_by_key(|x| x.abs()).await;
        assert_eq!(min, None);
        #
        # }) }
        ```
    "#]
    fn min_by_key<B, F>(
        self,
        key_by: F,
    ) -> MinByKeyFuture<Self, Self::Item, F>
    where
        Self: Sized,
        B: Ord,
        F: FnMut(&Self::Item) -> B,
    {
        MinByKeyFuture::new(self, key_by)
    }

    #[doc = r#"
        Returns the element that gives the maximum value with respect to the
        specified key function. If several elements are equally maximum,
        the first element is returned. If the stream is empty, `None` is returned.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![-3_i32, 0, 1, 5, -10]);

        let max = s.clone().max_by_key(|x| x.abs()).await;
        assert_eq!(max, Some(-10));

        let max = stream::empty::<isize>().max_by_key(|x| x.abs()).await;
        assert_eq!(max, None);
        #
        # }) }
        ```
    "#]
    fn max_by_key<B, F>(
        self,
        key_by: F,
    ) -> MaxByKeyFuture<Self, Self::Item, F>
    where
        Self: Sized,
        B: Ord,
        F: FnMut(&Self::Item) -> B,
    {
        MaxByKeyFuture::new(self, key_by)
    }

    #[doc = r#"
        Returns the element that gives the minimum value with respect to the
        specified comparison function. If several elements are equally minimum,
        the first element is returned. If the stream is empty, `None` is returned.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1u8, 2, 3]);

        let min = s.clone().min_by(|x, y| x.cmp(y)).await;
        assert_eq!(min, Some(1));

        let min = s.min_by(|x, y| y.cmp(x)).await;
        assert_eq!(min, Some(3));

        let min = stream::empty::<u8>().min_by(|x, y| x.cmp(y)).await;
        assert_eq!(min, None);
        #
        # }) }
        ```
    "#]
    fn min_by<F>(
        self,
        compare: F,
    ) -> MinByFuture<Self, F, Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        MinByFuture::new(self, compare)
    }

    #[doc = r#"
        Returns the element that gives the maximum value. If several elements are equally maximum,
        the first element is returned. If the stream is empty, `None` is returned.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1usize, 2, 3]);

        let max = s.clone().max().await;
        assert_eq!(max, Some(3));

        let max = stream::empty::<usize>().max().await;
        assert_eq!(max, None);
        #
        # }) }
        ```
    "#]
    fn max(
        self,
    ) -> MaxFuture<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        MaxFuture::new(self)
    }

            #[doc = r#"
        Returns the element that gives the minimum value. If several elements are equally minimum,
        the first element is returned. If the stream is empty, `None` is returned.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1usize, 2, 3]);

        let min = s.clone().min().await;
        assert_eq!(min, Some(1));

        let min = stream::empty::<usize>().min().await;
        assert_eq!(min, None);
        #
        # }) }
        ```
    "#]
    fn min(
        self,
    ) -> MinFuture<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        MinFuture::new(self)
    }

     #[doc = r#"
        Returns the element that gives the maximum value with respect to the
        specified comparison function. If several elements are equally maximum,
        the first element is returned. If the stream is empty, `None` is returned.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1u8, 2, 3]);

        let max = s.clone().max_by(|x, y| x.cmp(y)).await;
        assert_eq!(max, Some(3));

        let max = s.max_by(|x, y| y.cmp(x)).await;
        assert_eq!(max, Some(1));

        let max = stream::empty::<usize>().max_by(|x, y| x.cmp(y)).await;
        assert_eq!(max, None);
        #
        # }) }
        ```
    "#]
    fn max_by<F>(
        self,
        compare: F,
    ) -> MaxByFuture<Self, F, Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        MaxByFuture::new(self, compare)
    }

    #[doc = r#"
        Returns the nth element of the stream.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::from_iter(vec![1u8, 2, 3]);

        let second = s.nth(1).await;
        assert_eq!(second, Some(2));
        #
        # }) }
        ```
        Calling `nth()` multiple times:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::stream;
        use async_std::prelude::*;

        let mut s = stream::from_iter(vec![1u8, 2, 3]);

        let second = s.nth(0).await;
        assert_eq!(second, Some(1));

        let second = s.nth(0).await;
        assert_eq!(second, Some(2));
        #
        # }) }
        ```
        Returning `None` if the stream finished before returning `n` elements:
        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s  = stream::from_iter(vec![1u8, 2, 3]);

        let fourth = s.nth(4).await;
        assert_eq!(fourth, None);
        #
        # }) }
        ```
    "#]
    fn nth(
        &mut self,
        n: usize,
    ) -> NthFuture<'_, Self>
    where
        Self: Unpin + Sized,
    {
        NthFuture::new(self, n)
    }

    #[doc = r#"
        Tests if every element of the stream matches a predicate.

        `all()` takes a closure that returns `true` or `false`. It applies
        this closure to each element of the stream, and if they all return
        `true`, then so does `all()`. If any of them return `false`, it
        returns `false`.

        `all()` is short-circuiting; in other words, it will stop processing
        as soon as it finds a `false`, given that no matter what else happens,
        the result will also be `false`.

        An empty stream returns `true`.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::repeat::<u32>(42).take(3);
        assert!(s.all(|x| x ==  42).await);

        #
        # }) }
        ```

        Empty stream:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::empty::<u32>();
        assert!(s.all(|_| false).await);
        #
        # }) }
        ```
    "#]
    #[inline]
    fn all<F>(
        &mut self,
        f: F,
    ) -> AllFuture<'_, Self, F, Self::Item>
    where
        Self: Unpin + Sized,
        F: FnMut(Self::Item) -> bool,
    {
        AllFuture::new(self, f)
    }

    #[doc = r#"
        Searches for an element in a stream that satisfies a predicate.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::from_iter(vec![1u8, 2, 3]);
        let res = s.find(|x| *x == 2).await;
        assert_eq!(res, Some(2));
        #
        # }) }
        ```

        Resuming after a first find:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s= stream::from_iter(vec![1, 2, 3]);
        let res = s.find(|x| *x == 2).await;
        assert_eq!(res, Some(2));

        let next = s.next().await;
        assert_eq!(next, Some(3));
        #
        # }) }
        ```
    "#]
    fn find<P>(
        &mut self,
        p: P,
    ) -> FindFuture<'_, Self, P>
    where
        Self: Unpin + Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        FindFuture::new(self, p)
    }

    #[doc = r#"
        Applies function to the elements of stream and returns the first non-none result.

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::from_iter(vec!["lol", "NaN", "2", "5"]);
        let first_number = s.find_map(|s| s.parse().ok()).await;

        assert_eq!(first_number, Some(2));
        #
        # }) }
        ```
    "#]
    fn find_map<F, B>(
        &mut self,
        f: F,
    ) -> FindMapFuture<'_, Self, F>
    where
        Self: Unpin + Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        FindMapFuture::new(self, f)
    }

    #[doc = r#"
        A combinator that applies a function to every element in a stream
        producing a single, final value.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1u8, 2, 3]);
        let sum = s.fold(0, |acc, x| acc + x).await;

        assert_eq!(sum, 6);
        #
        # }) }
        ```
    "#]
    fn fold<B, F>(
        self,
        init: B,
        f: F,
    ) -> FoldFuture<Self, F, B>
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        FoldFuture::new(self, init, f)
    }

    #[doc = r#"
        A combinator that applies a function to every element in a stream
        creating two collections from it.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let (even, odd): (Vec<i32>, Vec<i32>) = stream::from_iter(vec![1, 2, 3])
            .partition(|&n| n % 2 == 0).await;

        assert_eq!(even, vec![2]);
        assert_eq!(odd, vec![1, 3]);

        #
        # }) }
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn partition<B, F>(
        self,
        f: F,
    ) -> PartitionFuture<Self, F, B>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
        B: Default + Extend<Self::Item>,
    {
        PartitionFuture::new(self, f)
    }

    #[doc = r#"
        Call a closure on each element of the stream.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;
        use std::sync::mpsc::channel;

        let (tx, rx) = channel();

        let s = stream::from_iter(vec![1usize, 2, 3]);
        let sum = s.for_each(move |x| tx.clone().send(x).unwrap()).await;

        let v: Vec<_> = rx.iter().collect();

        assert_eq!(v, vec![1, 2, 3]);
        #
        # }) }
        ```
    "#]
    fn for_each<F>(
        self,
        f: F,
    ) -> ForEachFuture<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        ForEachFuture::new(self, f)
    }

    #[doc = r#"
        Tests if any element of the stream matches a predicate.

        `any()` takes a closure that returns `true` or `false`. It applies
        this closure to each element of the stream, and if any of them return
        `true`, then so does `any()`. If they all return `false`, it
        returns `false`.

        `any()` is short-circuiting; in other words, it will stop processing
        as soon as it finds a `true`, given that no matter what else happens,
        the result will also be `true`.

        An empty stream returns `false`.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::repeat::<u32>(42).take(3);
        assert!(s.any(|x| x ==  42).await);
        #
        # }) }
        ```

        Empty stream:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::empty::<u32>();
        assert!(!s.any(|_| false).await);
        #
        # }) }
        ```
    "#]
    #[inline]
    fn any<F>(
        &mut self,
        f: F,
    ) -> AnyFuture<'_, Self, F, Self::Item>
    where
        Self: Unpin + Sized,
        F: FnMut(Self::Item) -> bool,
    {
        AnyFuture::new(self, f)
    }

    #[doc = r#"
        Borrows an stream, rather than consuming it.

        This is useful to allow applying stream adaptors while still retaining ownership of the original stream.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let a = vec![1isize, 2, 3];

        let stream = stream::from_iter(a);

        let sum: isize = stream.take(5).sum().await;

        assert_eq!(sum, 6);

        // if we try to use stream again, it won't work. The following line
        // gives error: use of moved value: `stream`
        // assert_eq!(stream.next(), None);

        // let's try that again
        let a = vec![1isize, 2, 3];

        let mut stream = stream::from_iter(a);

        // instead, we add in a .by_ref()
        let sum: isize = stream.by_ref().take(2).sum().await;

        assert_eq!(sum, 3);

        // now this is just fine:
        assert_eq!(stream.next().await, Some(3));
        assert_eq!(stream.next().await, None);
        #
        # }) }
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn by_ref(&mut self) -> &mut Self {
        self
    }

    #[doc = r#"
        A stream adaptor similar to [`fold`] that holds internal state and produces a new
        stream.

        [`fold`]: #method.fold

        `scan()` takes two arguments: an initial value which seeds the internal state, and
        a closure with two arguments, the first being a mutable reference to the internal
        state and the second a stream element. The closure can assign to the internal state
        to share state between iterations.

        On iteration, the closure will be applied to each element of the stream and the
        return value from the closure, an `Option`, is yielded by the stream.

        ## Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1isize, 2, 3]);
        let mut s = s.scan(1, |state, x| {
            *state = *state * x;
            Some(-*state)
        });

        assert_eq!(s.next().await, Some(-1));
        assert_eq!(s.next().await, Some(-2));
        assert_eq!(s.next().await, Some(-6));
        assert_eq!(s.next().await, None);
        #
        # }) }
        ```
    "#]
    #[inline]
    fn scan<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> Option<B>,
    {
        Scan::new(self, initial_state, f)
    }

    #[doc = r#"
        Combinator that `skip`s elements based on a predicate.

        Takes a closure argument. It will call this closure on every element in
        the stream and ignore elements until it returns `false`.

        After `false` is returned, `SkipWhile`'s job is over and all further
        elements in the stream are yielded.

        ## Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let a = stream::from_iter(vec![-1i32, 0, 1]);
        let mut s = a.skip_while(|x| x.is_negative());

        assert_eq!(s.next().await, Some(0));
        assert_eq!(s.next().await, Some(1));
        assert_eq!(s.next().await, None);
        #
        # }) }
        ```
    "#]
    fn skip_while<P>(self, predicate: P) -> SkipWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        SkipWhile::new(self, predicate)
    }

    #[doc = r#"
        Creates a combinator that skips the first `n` elements.

        ## Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1u8, 2, 3]);
        let mut skipped = s.skip(2);

        assert_eq!(skipped.next().await, Some(3));
        assert_eq!(skipped.next().await, None);
        #
        # }) }
        ```
    "#]
    fn skip(self, n: usize) -> Skip<Self>
    where
        Self: Sized,
    {
        Skip::new(self, n)
    }

    #[doc=r#"
        Await a stream or times out after a duration of time.

        If you want to await an I/O future consider using
        [`io::timeout`](../io/fn.timeout.html) instead.

        # Examples

        ```
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use std::time::Duration;

        use async_std::stream;
        use async_std::prelude::*;

        let mut s = stream::repeat(1).take(3).timeout(Duration::from_secs(1));

        while let Some(v) = s.next().await {
            assert_eq!(v, Ok(1));
        }

        // when timeout
        let mut s = stream::pending::<()>().timeout(Duration::from_millis(10));
        match s.next().await {
            Some(item) => assert!(item.is_err()),
            None => panic!()
        };
        #
        # Ok(()) }) }
        ```
    "#]
    #[cfg(any(feature = "unstable", feature = "docs"))]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn timeout(self, dur: Duration) -> Timeout<Self>
    where
        Self: Stream + Sized,
    {
        Timeout::new(self, dur)
    }

    #[doc = r#"
        A combinator that applies a function as long as it returns successfully, producing a single, final value.
        Immediately returns the error when the function returns unsuccessfully.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let mut s = stream::from_iter(vec![1usize, 2, 3]);
        let sum = s.try_fold(0, |acc, v| {
            if (acc+v) % 2 == 1 {
                Ok(v+3)
            } else {
                Err("fail")
            }
        }).await;

        assert_eq!(sum, Err("fail"));
        #
        # }) }
        ```
    "#]
    fn try_fold<B, F, T, E>(
        &mut self,
        init: T,
        f: F,
    ) -> TryFoldFuture<'_, Self, F, T>
    where
        Self: Unpin + Sized,
        F: FnMut(B, Self::Item) -> Result<T, E>,
    {
        TryFoldFuture::new(self, init, f)
    }

    #[doc = r#"
        Applies a falliable function to each element in a stream, stopping at first error and returning it.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use std::sync::mpsc::channel;
        use async_std::prelude::*;
        use async_std::stream;

        let (tx, rx) = channel();

        let mut s = stream::from_iter(vec![1u8, 2, 3]);
        let s = s.try_for_each(|v| {
            if v % 2 == 1 {
                tx.clone().send(v).unwrap();
                Ok(())
            } else {
                Err("even")
            }
        });

        let res = s.await;
        drop(tx);
        let values: Vec<_> = rx.iter().collect();

        assert_eq!(values, vec![1]);
        assert_eq!(res, Err("even"));
        #
        # }) }
        ```
    "#]
    fn try_for_each<F, E>(
        &mut self,
        f: F,
    ) -> TryForEachFuture<'_, Self, F>
    where
        Self: Unpin + Sized,
        F: FnMut(Self::Item) -> Result<(), E>,
    {
        TryForEachFuture::new(self, f)
    }

    #[doc = r#"
        'Zips up' two streams into a single stream of pairs.

        `zip()` returns a new stream that will iterate over two other streams, returning a
        tuple where the first element comes from the first stream, and the second element
        comes from the second stream.

        In other words, it zips two streams together, into a single one.

        If either stream returns [`None`], [`poll_next`] from the zipped stream will return
        [`None`]. If the first stream returns [`None`], `zip` will short-circuit and
        `poll_next` will not be called on the second stream.

        [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
        [`poll_next`]: #tymethod.poll_next

        ## Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let l = stream::from_iter(vec![1u8, 2, 3]);
        let r = stream::from_iter(vec![4u8, 5, 6, 7]);
        let mut s = l.zip(r);

        assert_eq!(s.next().await, Some((1, 4)));
        assert_eq!(s.next().await, Some((2, 5)));
        assert_eq!(s.next().await, Some((3, 6)));
        assert_eq!(s.next().await, None);
        #
        # }) }
        ```
    "#]
    #[inline]
    fn zip<U>(self, other: U) -> Zip<Self, U>
    where
        Self: Sized,
        U: Stream,
    {
        Zip::new(self, other)
    }

    #[doc = r#"
        Converts an stream of pairs into a pair of containers.

        `unzip()` consumes an entire stream of pairs, producing two collections: one from the left elements of the pairs, and one from the right elements.

        This function is, in some sense, the opposite of [`zip`].

        [`zip`]: trait.Stream.html#method.zip

        # Example

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s  = stream::from_iter(vec![(1,2), (3,4)]);

        let (left, right): (Vec<_>, Vec<_>) = s.unzip().await;

        assert_eq!(left, [1, 3]);
        assert_eq!(right, [2, 4]);
        #
        # }) }
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn unzip<A, B, FromA, FromB>(self) -> UnzipFuture<Self, FromA, FromB>
    where
    FromA: Default + Extend<A>,
    FromB: Default + Extend<B>,
    Self: Stream<Item = (A, B)> + Sized,
    {
        UnzipFuture::new(self)
    }

    #[doc = r#"
        Transforms a stream into a collection.

        `collect()` can take anything streamable, and turn it into a relevant
        collection. This is one of the more powerful methods in the async
        standard library, used in a variety of contexts.

        The most basic pattern in which `collect()` is used is to turn one
        collection into another. You take a collection, call [`into_stream`] on it,
        do a bunch of transformations, and then `collect()` at the end.

        Because `collect()` is so general, it can cause problems with type
        inference. As such, `collect()` is one of the few times you'll see
        the syntax affectionately known as the 'turbofish': `::<>`. This
        helps the inference algorithm understand specifically which collection
        you're trying to collect into.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::repeat(9u8).take(3);
        let buf: Vec<u8> = s.collect().await;

        assert_eq!(buf, vec![9; 3]);

        // You can also collect streams of Result values
        // into any collection that implements FromStream
        let s = stream::repeat(Ok(9)).take(3);
        // We are using Vec here, but other collections
        // are supported as well
        let buf: Result<Vec<u8>, ()> = s.collect().await;

        assert_eq!(buf, Ok(vec![9; 3]));

        // The stream will stop on the first Err and
        // return that instead
        let s = stream::repeat(Err(5)).take(3);
        let buf: Result<Vec<u8>, u8> = s.collect().await;

        assert_eq!(buf, Err(5));
        #
        # }) }
        ```

        [`into_stream`]: trait.IntoStream.html#tymethod.into_stream
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn collect<'a, B>(
        self,
    ) -> Pin<Box<dyn Future<Output = B> + 'a + Send>>
    where
        Self: Sized + 'a + Send,
        B: FromStream<Self::Item>,
        Self::Item: Send,
    {
        FromStream::from_stream(self)
    }

    #[doc = r#"
        Combines multiple streams into a single stream of all their outputs.

        Items are yielded as soon as they're received, and the stream continues yield until
        both streams have been exhausted. The output ordering between streams is not guaranteed.

        # Examples

        ```
        # async_std::task::block_on(async {
        use async_std::prelude::*;
        use async_std::stream::{self, FromStream};

        let a = stream::once(1u8);
        let b = stream::once(2u8);
        let c = stream::once(3u8);

        let s = a.merge(b).merge(c);
        let mut lst = Vec::from_stream(s).await;

        lst.sort_unstable();
        assert_eq!(&lst, &[1u8, 2u8, 3u8]);
        # });
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn merge<U>(self, other: U) -> Merge<Self, U>
    where
        Self: Sized,
        U: Stream<Item = Self::Item> + Sized,
    {
        Merge::new(self, other)
    }

    #[doc = r#"
        Lexicographically compares the elements of this `Stream` with those
        of another.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        use std::cmp::Ordering;

        let s1 = stream::from_iter(vec![1]);
        let s2 = stream::from_iter(vec![1, 2]);
        let s3 = stream::from_iter(vec![1, 2, 3]);
        let s4 = stream::from_iter(vec![1, 2, 4]);
        assert_eq!(s1.clone().partial_cmp(s1.clone()).await, Some(Ordering::Equal));
        assert_eq!(s1.clone().partial_cmp(s2.clone()).await, Some(Ordering::Less));
        assert_eq!(s2.clone().partial_cmp(s1.clone()).await, Some(Ordering::Greater));
        assert_eq!(s3.clone().partial_cmp(s4.clone()).await, Some(Ordering::Less));
        assert_eq!(s4.clone().partial_cmp(s3.clone()).await, Some(Ordering::Greater));
        #
        # }) }
        ```
    "#]
    fn partial_cmp<S>(
       self,
       other: S
    ) -> PartialCmpFuture<Self, S>
    where
        Self: Sized + Stream,
        S: Stream,
        <Self as Stream>::Item: PartialOrd<S::Item>,
    {
        PartialCmpFuture::new(self, other)
    }

    #[doc = r#"
        Searches for an element in a Stream that satisfies a predicate, returning
        its index.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![1usize, 2, 3]);
        let res = s.clone().position(|x| x == 1).await;
        assert_eq!(res, Some(0));

        let res = s.clone().position(|x| x == 2).await;
        assert_eq!(res, Some(1));

        let res = s.clone().position(|x| x == 3).await;
        assert_eq!(res, Some(2));

        let res = s.clone().position(|x| x == 4).await;
        assert_eq!(res, None);
        #
        # }) }
        ```
    "#]
    fn position<P>(
       &mut self,
       predicate: P,
    ) -> PositionFuture<'_, Self, P>
    where
        Self: Unpin + Sized,
        P: FnMut(Self::Item) -> bool,
    {
        PositionFuture::new(self, predicate)
    }

    #[doc = r#"
        Lexicographically compares the elements of this `Stream` with those
        of another using 'Ord'.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;
        use std::cmp::Ordering;

        let s1 = stream::from_iter(vec![1]);
        let s2 = stream::from_iter(vec![1, 2]);
        let s3 = stream::from_iter(vec![1, 2, 3]);
        let s4 = stream::from_iter(vec![1, 2, 4]);

        assert_eq!(s1.clone().cmp(s1.clone()).await, Ordering::Equal);
        assert_eq!(s1.clone().cmp(s2.clone()).await, Ordering::Less);
        assert_eq!(s2.clone().cmp(s1.clone()).await, Ordering::Greater);
        assert_eq!(s3.clone().cmp(s4.clone()).await, Ordering::Less);
        assert_eq!(s4.clone().cmp(s3.clone()).await, Ordering::Greater);
        #
        # }) }
        ```
    "#]
    fn cmp<S>(
       self,
       other: S
    ) -> CmpFuture<Self, S>
    where
        Self: Sized + Stream,
        S: Stream,
        <Self as Stream>::Item: Ord
    {
        CmpFuture::new(self, other)
    }

    #[doc = r#"
        Counts the number of elements in the stream.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s1 = stream::from_iter(vec![0]);
        let s2 = stream::from_iter(vec![1, 2, 3]);

        assert_eq!(s1.count().await, 1);
        assert_eq!(s2.count().await, 3);
        #
        # }) }
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn count(self) -> CountFuture<Self>
    where
        Self: Sized,
    {
        CountFuture::new(self)
    }

    #[doc = r#"
        Determines if the elements of this `Stream` are lexicographically
        not equal to those of another.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let single     = stream::from_iter(vec![1usize]);
        let single_ne  = stream::from_iter(vec![10usize]);
        let multi      = stream::from_iter(vec![1usize,2]);
        let multi_ne   = stream::from_iter(vec![1usize,5]);

        assert_eq!(single.clone().ne(single.clone()).await, false);
        assert_eq!(single_ne.clone().ne(single.clone()).await, true);
        assert_eq!(multi.clone().ne(single_ne.clone()).await, true);
        assert_eq!(multi_ne.clone().ne(multi.clone()).await, true);
        #
        # }) }
        ```
    "#]
    fn ne<S>(
       self,
       other: S
    ) -> NeFuture<Self, S>
    where
        Self: Sized,
        S: Sized + Stream,
        <Self as Stream>::Item: PartialEq<S::Item>,
    {
        NeFuture::new(self, other)
    }

    #[doc = r#"
        Determines if the elements of this `Stream` are lexicographically
        greater than or equal to those of another.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let single    = stream::from_iter(vec![1]);
        let single_gt = stream::from_iter(vec![10]);
        let multi     = stream::from_iter(vec![1,2]);
        let multi_gt  = stream::from_iter(vec![1,5]);

        assert_eq!(single.clone().ge(single.clone()).await, true);
        assert_eq!(single_gt.clone().ge(single.clone()).await, true);
        assert_eq!(multi.clone().ge(single_gt.clone()).await, false);
        assert_eq!(multi_gt.clone().ge(multi.clone()).await, true);
        #
        # }) }
        ```
    "#]
    fn ge<S>(
       self,
       other: S
    ) -> GeFuture<Self, S>
    where
        Self: Sized + Stream,
        S: Stream,
        <Self as Stream>::Item: PartialOrd<S::Item>,
    {
        GeFuture::new(self, other)
    }

    #[doc = r#"
        Determines if the elements of this `Stream` are lexicographically
        equal to those of another.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let single     = stream::from_iter(vec![1]);
        let single_eq  = stream::from_iter(vec![10]);
        let multi      = stream::from_iter(vec![1,2]);
        let multi_eq   = stream::from_iter(vec![1,5]);

        assert_eq!(single.clone().eq(single.clone()).await, true);
        assert_eq!(single_eq.clone().eq(single.clone()).await, false);
        assert_eq!(multi.clone().eq(single_eq.clone()).await, false);
        assert_eq!(multi_eq.clone().eq(multi.clone()).await, false);
        #
        # }) }
        ```
    "#]
    fn eq<S>(
       self,
       other: S
    ) -> EqFuture<Self, S>
    where
        Self: Sized + Stream,
        S: Sized + Stream,
        <Self as Stream>::Item: PartialEq<S::Item>,
    {
        EqFuture::new(self, other)
    }

    #[doc = r#"
        Determines if the elements of this `Stream` are lexicographically
        greater than those of another.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let single = stream::from_iter(vec![1]);
        let single_gt = stream::from_iter(vec![10]);
        let multi = stream::from_iter(vec![1,2]);
        let multi_gt = stream::from_iter(vec![1,5]);

        assert_eq!(single.clone().gt(single.clone()).await, false);
        assert_eq!(single_gt.clone().gt(single.clone()).await, true);
        assert_eq!(multi.clone().gt(single_gt.clone()).await, false);
        assert_eq!(multi_gt.clone().gt(multi.clone()).await, true);
        #
        # }) }
        ```
    "#]
    fn gt<S>(
       self,
       other: S
    ) -> GtFuture<Self, S>
    where
        Self: Sized + Stream,
        S: Stream,
        <Self as Stream>::Item: PartialOrd<S::Item>,
    {
        GtFuture::new(self, other)
    }

    #[doc = r#"
        Determines if the elements of this `Stream` are lexicographically
        less or equal to those of another.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let single = stream::from_iter(vec![1]);
        let single_gt = stream::from_iter(vec![10]);
        let multi = stream::from_iter(vec![1,2]);
        let multi_gt = stream::from_iter(vec![1,5]);

        assert_eq!(single.clone().le(single.clone()).await, true);
        assert_eq!(single.clone().le(single_gt.clone()).await, true);
        assert_eq!(multi.clone().le(single_gt.clone()).await, true);
        assert_eq!(multi_gt.clone().le(multi.clone()).await, false);
        #
        # }) }
        ```
    "#]
    fn le<S>(
       self,
       other: S
    ) -> LeFuture<Self, S>
    where
        Self: Sized + Stream,
        S: Stream,
        <Self as Stream>::Item: PartialOrd<S::Item>,
    {
        LeFuture::new(self, other)
    }

    #[doc = r#"
        Determines if the elements of this `Stream` are lexicographically
        less than those of another.

        # Examples

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let single = stream::from_iter(vec![1]);
        let single_gt = stream::from_iter(vec![10]);
        let multi = stream::from_iter(vec![1,2]);
        let multi_gt = stream::from_iter(vec![1,5]);

        assert_eq!(single.clone().lt(single.clone()).await, false);
        assert_eq!(single.clone().lt(single_gt.clone()).await, true);
        assert_eq!(multi.clone().lt(single_gt.clone()).await, true);
        assert_eq!(multi_gt.clone().lt(multi.clone()).await, false);
        #
        # }) }
        ```
    "#]
    fn lt<S>(
       self,
       other: S
    ) -> LtFuture<Self, S>
    where
        Self: Sized + Stream,
        S: Stream,
        <Self as Stream>::Item: PartialOrd<S::Item>,
    {
        LtFuture::new(self, other)
    }

    #[doc = r#"
        Sums the elements of a stream.

        Takes each element, adds them together, and returns the result.

        An empty streams returns the zero value of the type.

        # Panics

        When calling `sum()` and a primitive integer type is being returned, this
        method will panic if the computation overflows and debug assertions are
        enabled.

        # Examples

        Basic usage:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let s = stream::from_iter(vec![0u8, 1, 2, 3, 4]);
        let sum: u8 = s.sum().await;

        assert_eq!(sum, 10);
        #
        # }) }
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn sum<'a, S>(
        self,
    ) -> Pin<Box<dyn Future<Output = S> + 'a>>
    where
        Self: Sized + Stream<Item = S> + 'a,
        S: Sum<Self::Item>,
    {
        Sum::sum(self)
    }

    #[doc = r#"
        Multiplies all elements of the stream.

        An empty stream returns the one value of the type.

        # Panics

        When calling `product()` and a primitive integer type is being returned,
        method will panic if the computation overflows and debug assertions are
        enabled.

        # Examples

        This example calculates the factorial of n (i.e. the product of the numbers from 1 to
        n, inclusive):

        ```
        # fn main() { async_std::task::block_on(async {
        #
        async fn factorial(n: u32) -> u32 {
            use async_std::prelude::*;
            use async_std::stream;

            let s = stream::from_iter(1..=n);
            s.product().await
        }

        assert_eq!(factorial(0).await, 1);
        assert_eq!(factorial(1).await, 1);
        assert_eq!(factorial(5).await, 120);
        #
        # }) }
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn product<'a, P>(
        self,
    ) -> Pin<Box<dyn Future<Output = P> + 'a>>
    where
        Self: Sized + Stream<Item = P> + 'a,
        P: Product,
    {
        Product::product(self)
    }
}

impl<T: Stream + ?Sized> StreamExt for T {}

