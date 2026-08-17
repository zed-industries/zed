//! Regression test for: https://github.com/yoshuawuyts/futures-concurrency/issues/155
//!
//! We accidentally were marking a value as "ready" in `try_join`on the error
//! path. This meant that when we returned, the destructor assumed a value was
//! initialized when it wasn't, causing it to dereference uninitialized memory.

#![cfg(feature = "alloc")]

use futures_concurrency::prelude::*;
use futures_core::Future;
use std::{future::ready, pin::Pin};
use tokio::time::{sleep, Duration};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

async fn process_not_fail() -> Result<Vec<i32>, ()> {
    sleep(Duration::from_millis(100)).await;
    Ok(vec![ready(1), ready(2)].join().await)
}

async fn process_fail() -> Result<Vec<i32>, ()> {
    Err(())
}

#[tokio::test]
async fn array() {
    let a: BoxFuture<'static, _> = Box::pin(process_fail());
    let b: BoxFuture<'static, _> = Box::pin(process_not_fail());
    let res = [a, b].try_join().await;
    assert!(res.is_err());
}

#[tokio::test]
async fn vec() {
    let a: BoxFuture<'static, _> = Box::pin(process_fail());
    let b: BoxFuture<'static, _> = Box::pin(process_not_fail());
    let res = vec![a, b].try_join().await;
    assert!(res.is_err());
}

#[tokio::test]
async fn tuple() {
    let a = process_fail();
    let b = process_not_fail();
    let res = (a, b).try_join().await;
    assert!(res.is_err());
}
