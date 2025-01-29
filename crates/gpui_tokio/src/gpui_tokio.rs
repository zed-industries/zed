use std::future::Future;

use gpui::{App, Global, ReadGlobal, Task};
use tokio::task::JoinError;
use util::defer;

pub fn init(cx: &mut App) {
    cx.set_global(GlobalTokio::new());
}

struct GlobalTokio {
    runtime: tokio::runtime::Runtime,
}

impl Global for GlobalTokio {}

impl GlobalTokio {
    fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            // Since we now have two executors, let's try to keep our footprint small
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("Failed to initialize Tokio");

        Self { runtime }
    }
}

pub struct Tokio {}

impl Tokio {
    /// Spawns the given future on Tokio's thread pool, and returns it via a GPUI task
    /// Note that the Tokio task will be cancelled if the GPUI task is dropped
    pub fn spawn<Fut, R>(cx: &mut App, f: Fut) -> Task<Result<R, JoinError>>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let join_handle = GlobalTokio::global(cx).runtime.spawn(f);
        let abort_handle = join_handle.abort_handle();
        let cancel = defer(move || {
            abort_handle.abort();
        });
        cx.background_executor().spawn(async move {
            let result = join_handle.await;
            drop(cancel);
            result
        })
    }

    pub fn handle(cx: &mut App) -> tokio::runtime::Handle {
        GlobalTokio::global(cx).runtime.handle().clone()
    }
}
