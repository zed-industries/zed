//! The engine's own async runtime.
//!
//! `workflow_engine` stays free of any `gpui` dependency, but its executors
//! drive `tokio::process::Command`, `tokio::net::TcpListener`, timers and
//! `tokio::spawn` — all of which need an ambient tokio reactor. Forge gave
//! this code `backend::on_tokio`, a bridge over `reqwest_client`'s runtime;
//! here the bridge owns its own lazily-created multi-threaded runtime
//! instead. Engine callers (the Designer canvas) run on GPUI's own executor;
//! each `on_tokio(...)` call parks the caller's task and runs the future on
//! this runtime, mirroring the original `F: Future + Send + 'static,
//! F::Output: Send + 'static` contract exactly.

use std::future::Future;
use std::sync::LazyLock;

use tokio::runtime::Runtime;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build the workflow engine's tokio runtime")
});

/// Runs `fut` to completion on the dedicated tokio runtime and awaits the result.
pub(crate) async fn on_tokio<F>(fut: F) -> F::Output
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    RUNTIME.spawn(fut).await.expect("tokio task panicked")
}

/// `CREATE_NO_WINDOW` (0x0800_0000), so child tool processes spawned by
/// actions don't flash a console window. Windows only: no other platform
/// opens one.
#[cfg(target_os = "windows")]
pub(crate) const CREATE_NO_WINDOW: u32 = 0x0800_0000;
