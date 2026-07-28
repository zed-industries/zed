// Tests for the `async_block_without_await` lint.

#![allow(unused)]

async fn returns_42() -> i32 {
    42
}

fn sync_fn() -> i32 {
    42
}

fn main() {
    // --- Should warn ---

    // Empty async block.
    let _f = async {};

    // Async block with a pure expression.
    let _f = async { 42 };

    // Async move block without await.
    let _f = async move {
        let x = 1;
        x + 2
    };

    // Async block calling a sync function.
    let _f = async { sync_fn() };

    // Nested: the *inner* async block has no await (should warn on it).
    // The outer block awaits the inner future, so the outer is fine.
    let _f = async {
        let inner = async { 42 };
        inner.await
    };

    // --- Should NOT warn ---

    // Async block with an await.
    let _f = async { returns_42().await };

    // Async block with await in a let binding.
    let _f = async {
        let x = returns_42().await;
        x + 1
    };

    // Async move block with await.
    let _f = async move { returns_42().await };

    // Regular closure (not async at all).
    let _f = || 42;
}

// --- Trait impl cases ---

use std::future::Future;

trait AsyncWork {
    fn do_work(&self) -> impl Future<Output = i32>;
}

struct Worker;

// Should NOT warn: the trait requires returning a future, so the
// implementor has no choice but to use `async { ... }`.
impl AsyncWork for Worker {
    fn do_work(&self) -> impl Future<Output = i32> {
        async { 42 }
    }
}

// Should warn: inherent impl — the author chose `async` freely.
impl Worker {
    fn compute(&self) -> impl Future<Output = i32> {
        async { 42 }
    }
}
