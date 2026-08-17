cfg_unstable! {
    mod delay;
    mod flatten;
    mod race;
    mod try_race;
    mod join;
    mod try_join;

    use std::time::Duration;
    use delay::DelayFuture;
    use flatten::FlattenFuture;
    use crate::future::IntoFuture;
    use race::Race;
    use try_race::TryRace;
    use join::Join;
    use try_join::TryJoin;
}

cfg_unstable_default! {
    use crate::future::timeout::TimeoutFuture;
}

pub use core::future::Future as Future;

#[doc = r#"
    Extension methods for [`Future`].

    [`Future`]: ../future/trait.Future.html
"#]
#[cfg(any(feature = "std", feature = "docs"))]
pub trait FutureExt: Future {
    /// Returns a Future that delays execution for a specified time.
    ///
    /// # Examples
    ///
    /// ```
    /// # async_std::task::block_on(async {
    /// use async_std::prelude::*;
    /// use async_std::future;
    /// use std::time::Duration;
    ///
    /// let a = future::ready(1).delay(Duration::from_millis(2000));
    /// dbg!(a.await);
    /// # })
    /// ```
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn delay(self, dur: Duration) -> DelayFuture<Self>
    where
        Self: Sized,
    {
        DelayFuture::new(self, dur)
    }

    /// Flatten out the execution of this future when the result itself
    /// can be converted into another future.
    ///
    /// # Examples
    ///
    /// ```
    /// # async_std::task::block_on(async {
    /// use async_std::prelude::*;
    ///
    /// let nested_future = async { async { 1 } };
    /// let future = nested_future.flatten();
    /// assert_eq!(future.await, 1);
    /// # })
    /// ```
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn flatten(
        self,
    ) -> FlattenFuture<Self, <Self::Output as IntoFuture>::Future>
    where
        Self: Sized,
        <Self as Future>::Output: IntoFuture,
    {
       FlattenFuture::new(self)
    }

    #[doc = r#"
        Waits for one of two similarly-typed futures to complete.

        Awaits multiple futures simultaneously, returning the output of the
        first future that completes.

        This function will return a new future which awaits for either one of both
        futures to complete. If multiple futures are completed at the same time,
        resolution will occur in the order that they have been passed.

        Note that this function consumes all futures passed, and once a future is
        completed, all other futures are dropped.

        # Examples

        ```
        # async_std::task::block_on(async {
        use async_std::prelude::*;
        use async_std::future;

        let a = future::pending();
        let b = future::ready(1u8);
        let c = future::ready(2u8);

        let f = a.race(b).race(c);
        assert_eq!(f.await, 1u8);
        # });
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn race<F>(
        self,
        other: F,
    ) -> Race<Self, F>
    where
        Self: std::future::Future + Sized,
        F: std::future::Future<Output = <Self as std::future::Future>::Output>,
    {
        Race::new(self, other)
    }

    #[doc = r#"
        Waits for one of two similarly-typed fallible futures to complete.

        Awaits multiple futures simultaneously, returning all results once complete.

        `try_race` is similar to [`race`], but keeps going if a future
        resolved to an error until all futures have been resolved. In which case
        an error is returned.

        The ordering of which value is yielded when two futures resolve
        simultaneously is intentionally left unspecified.

        [`race`]: #method.race

        # Examples

        ```
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::future;
        use std::io::{Error, ErrorKind};

        let a = future::pending::<Result<_, Error>>();
        let b = future::ready(Err(Error::from(ErrorKind::Other)));
        let c = future::ready(Ok(1u8));

        let f = a.try_race(b).try_race(c);
        assert_eq!(f.await?, 1u8);
        #
        # Ok(()) }) }
        ```
    "#]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn try_race<F, T, E>(
        self,
        other: F
    ) -> TryRace<Self, F>
    where
        Self: std::future::Future<Output = Result<T, E>> + Sized,
        F: std::future::Future<Output = <Self as std::future::Future>::Output>,
    {
        TryRace::new(self, other)
    }

    #[doc = r#"
        Waits for two similarly-typed futures to complete.

        Awaits multiple futures simultaneously, returning the output of the
        futures once both complete.

        This function returns a new future which polls both futures
        concurrently.

        # Examples

        ```
        # async_std::task::block_on(async {
        use async_std::prelude::*;
        use async_std::future;

        let a = future::ready(1u8);
        let b = future::ready(2u16);

        let f = a.join(b);
        assert_eq!(f.await, (1u8, 2u16));
        # });
        ```
    "#]
    #[cfg(any(feature = "unstable", feature = "docs"))]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn join<F>(
        self,
        other: F
    ) -> Join<Self, F>
    where
        Self: std::future::Future + Sized,
        F: std::future::Future,
    {
        Join::new(self, other)
    }

    #[doc = r#"
        Waits for two similarly-typed fallible futures to complete.

        Awaits multiple futures simultaneously, returning all results once
        complete.

        `try_join` is similar to [`join`], but returns an error immediately
        if a future resolves to an error.

        [`join`]: #method.join

        # Examples

        ```
        # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::future;

        let a = future::ready(Err::<u8, &str>("Error"));
        let b = future::ready(Ok(1u8));

        let f = a.try_join(b);
        assert_eq!(f.await, Err("Error"));

        let a = future::ready(Ok::<u8, String>(1u8));
        let b = future::ready(Ok::<u16, String>(2u16));

        let f = a.try_join(b);
        assert_eq!(f.await, Ok((1u8, 2u16)));
        #
        # Ok(()) }) }
        ```
    "#]
    #[cfg(any(feature = "unstable", feature = "docs"))]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn try_join<F, A, B, E>(
        self,
        other: F
    ) -> TryJoin<Self, F>
    where
        Self: std::future::Future<Output = Result<A, E>> + Sized,
        F: std::future::Future<Output = Result<B, E>>,
    {
        TryJoin::new(self, other)
    }

    #[doc = r#"
        Waits for both the future and a timeout, if the timeout completes before
        the future, it returns a TimeoutError.

        # Example
        ```
        # async_std::task::block_on(async {
        #
        use std::time::Duration;

        use async_std::prelude::*;
        use async_std::future;

        let fut = future::ready(0);
        let dur = Duration::from_millis(100);
        let res = fut.timeout(dur).await;
        assert!(res.is_ok());

        let fut = future::pending::<()>();
        let dur = Duration::from_millis(100);
        let res = fut.timeout(dur).await;
        assert!(res.is_err())
        #
        # });
        ```
    "#]
    #[cfg(any(all(feature = "default", feature = "unstable"), feature = "docs"))]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    fn timeout(self, dur: Duration) -> TimeoutFuture<Self>
        where Self: Sized
    {
        TimeoutFuture::new(self, dur)
    }
}

#[cfg(any(feature = "std", feature = "docs"))]
impl<T: Future + ?Sized> FutureExt for T {}

