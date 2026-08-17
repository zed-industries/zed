#![cfg(all(tokio_unstable, feature = "tracing"))]

use std::rc::Rc;
use tokio::task::{Builder, LocalSet};

#[tokio::test]
async fn spawn_with_name() {
    let result = Builder::new()
        .name("name")
        .spawn(async { "task executed" })
        .unwrap()
        .await;

    assert_eq!(result.unwrap(), "task executed");
}

#[tokio::test(flavor = "local")]
async fn spawn_local_on_local_runtime() {
    let result = Builder::new()
        .spawn_local(async { "task executed" })
        .unwrap()
        .await;

    assert_eq!(result.unwrap(), "task executed");
}

#[tokio::test]
#[should_panic = "`spawn_local` called from outside of a `task::LocalSet` or `runtime::LocalRuntime`"]
async fn spawn_local_panics_outside_local_set_or_local_runtime() {
    let _ = Builder::new()
        .spawn_local(async { "task executed" })
        .unwrap()
        .await;
}

#[tokio::test(flavor = "multi_thread")]
#[should_panic = "`spawn_local` called from outside of a `task::LocalSet` or `runtime::LocalRuntime`"]
async fn spawn_local_panics_in_multi_thread_runtime() {
    let _ = Builder::new()
        .spawn_local(async { "task executed" })
        .unwrap()
        .await;
}

#[tokio::test]
async fn spawn_blocking_with_name() {
    let result = Builder::new()
        .name("name")
        .spawn_blocking(|| "task executed")
        .unwrap()
        .await;

    assert_eq!(result.unwrap(), "task executed");
}

#[tokio::test]
async fn spawn_local_with_name() {
    let unsend_data = Rc::new("task executed");
    let result = LocalSet::new()
        .run_until(async move {
            Builder::new()
                .name("name")
                .spawn_local(async move { unsend_data })
                .unwrap()
                .await
        })
        .await;

    assert_eq!(*result.unwrap(), "task executed");
}

#[tokio::test]
async fn spawn_without_name() {
    let result = Builder::new()
        .spawn(async { "task executed" })
        .unwrap()
        .await;

    assert_eq!(result.unwrap(), "task executed");
}

#[tokio::test]
async fn spawn_blocking_without_name() {
    let result = Builder::new()
        .spawn_blocking(|| "task executed")
        .unwrap()
        .await;

    assert_eq!(result.unwrap(), "task executed");
}

#[tokio::test]
async fn spawn_local_without_name() {
    let unsend_data = Rc::new("task executed");
    let result = LocalSet::new()
        .run_until(async move {
            Builder::new()
                .spawn_local(async move { unsend_data })
                .unwrap()
                .await
        })
        .await;

    assert_eq!(*result.unwrap(), "task executed");
}
