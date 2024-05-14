use gpui::{Subscription, Task, WeakModel};
use language::Buffer;

pub struct RecentBuffersContext {
    pub enabled: bool,
    pub buffers: Vec<RecentBuffer>,
    pub message: String,
    pub pending_message: Option<Task<()>>,
}

pub struct RecentBuffer {
    pub buffer: WeakModel<Buffer>,
    pub _subscription: Subscription,
}

impl Default for RecentBuffersContext {
    fn default() -> Self {
        Self {
            enabled: true,
            buffers: Vec::new(),
            message: String::new(),
            pending_message: None,
        }
    }
}
