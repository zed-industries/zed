use std::sync::Arc;

pub struct Client;

pub fn force_backtrace() {}

pub fn init<F, C, S, P>(
    _crash_init: InitCrashHandler,
    _spawn: S,
    _socket_path: P,
    _wait_timer: C,
) -> impl std::future::Future<Output = Arc<Client>>
where
    F: std::future::Future<Output = ()> + Send + Sync + 'static,
    C: (Fn(std::time::Duration) -> F) + Send + Sync + 'static,
    S: FnOnce(std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>),
    P: FnOnce(u32) -> std::path::PathBuf,
{
    async { Arc::new(Client) }
}

pub fn crash_server(_socket: &std::path::Path, _logs_dir: std::path::PathBuf) {}

pub fn panic_hook(_client: Arc<Client>, _message: &str, _location: Option<&std::panic::Location>) {}

pub fn set_gpu_info(_client: &Arc<Client>, _specs: gpui::GpuSpecs) {}
pub fn set_user_info(_client: &Arc<Client>, _info: UserInfo) {}

pub struct CrashInfo {
    pub init: InitCrashHandler,
    pub panic: Option<CrashPanic>,
    pub minidump_error: Option<String>,
    pub gpus: Vec<String>,
    pub active_gpu: Option<String>,
    pub user_info: Option<UserInfo>,
}

pub struct InitCrashHandler {
    pub session_id: String,
    pub zed_version: String,
    pub binary: String,
    pub release_channel: String,
    pub commit_sha: String,
}

pub struct CrashPanic {
    pub message: String,
    pub span: String,
}

pub struct UserInfo {
    pub metrics_id: Option<String>,
    pub is_staff: Option<bool>,
}
