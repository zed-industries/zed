#![no_std]

use core::future;
use futures_concurrency::{array::AggregateError, prelude::*};
use futures_lite::future::block_on;
use futures_lite::prelude::*;
use futures_lite::stream;

// These tests ensure that the traits provided by `futures-concurrency` work in a no std environment.

#[test]
fn join() {
    futures_lite::future::block_on(async {
        let fut = [future::ready("hello"), future::ready("world")].join();
        assert_eq!(fut.await, ["hello", "world"]);
    });
}

#[test]
fn try_join() {
    futures_lite::future::block_on(async {
        let res: Result<[&str; 2], &str> = [future::ready(Ok("hello")), future::ready(Ok("world"))]
            .try_join()
            .await;
        assert_eq!(res.unwrap(), ["hello", "world"]);
    })
}

#[test]
fn race() {
    futures_lite::future::block_on(async {
        let res = [future::ready("hello"), future::ready("world")]
            .race()
            .await;
        assert!(matches!(res, "hello" | "world"));
    });
}

#[test]
fn race_ok() {
    futures_lite::future::block_on(async {
        let res: Result<&str, AggregateError<&str, 2>> =
            [future::ready(Ok("hello")), future::ready(Ok("world"))]
                .race_ok()
                .await;
        assert!(res.is_ok());
    })
}

#[test]
fn chain_3() {
    block_on(async {
        let a = stream::once(1);
        let b = stream::once(2);
        let c = stream::once(3);
        let mut s = [a, b, c].chain();

        assert_eq!(s.next().await, Some(1));
        assert_eq!(s.next().await, Some(2));
        assert_eq!(s.next().await, Some(3));
        assert_eq!(s.next().await, None);
    })
}

#[test]
fn merge_array_4() {
    block_on(async {
        let a = stream::once(1);
        let b = stream::once(2);
        let c = stream::once(3);
        let d = stream::once(4);
        let mut s = [a, b, c, d].merge();

        let mut counter = 0;
        while let Some(n) = s.next().await {
            counter += n;
        }
        assert_eq!(counter, 10);
    })
}

#[test]
fn zip_array_3() {
    use futures_concurrency::stream::Zip;

    block_on(async {
        let a = stream::repeat(1).take(2);
        let b = stream::repeat(2).take(2);
        let c = stream::repeat(3).take(2);
        let mut s = Zip::zip([a, b, c]);

        assert_eq!(s.next().await, Some([1, 2, 3]));
        assert_eq!(s.next().await, Some([1, 2, 3]));
        assert_eq!(s.next().await, None);
    })
}
