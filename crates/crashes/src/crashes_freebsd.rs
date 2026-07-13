use std::future::Future;
use std::path::Path;

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

pub static REQUESTED_MINIDUMP: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitCrashHandler {
    pub session_id: String,
    pub zed_version: String,
    pub binary: String,
    pub release_channel: String,
    pub commit_sha: String,
}

pub fn init<F: Future<Output = ()> + Send + Sync + 'static>(
    _crash_init: InitCrashHandler,
    _spawn: impl FnOnce(BoxFuture<'static, ()>),
    _wait_timer: impl (Fn(std::time::Duration) -> F) + Send + Sync + 'static,
) {
    log::info!("crash handler disabled on FreeBSD");
}

pub fn crash_server(_socket: &Path) {
    log::info!("crash server disabled on FreeBSD");
}

pub fn set_gpu_info(_specs: ()) {}

pub fn set_user_info(_info: ()) {}
