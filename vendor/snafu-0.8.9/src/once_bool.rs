use std::sync::{
    atomic::{AtomicBool, Ordering},
    Once,
};

pub struct OnceBool {
    start: Once,
    enabled: AtomicBool,
}

impl OnceBool {
    pub const fn new() -> Self {
        Self {
            start: Once::new(),
            enabled: AtomicBool::new(false),
        }
    }

    pub fn get<F: FnOnce() -> bool>(&self, f: F) -> bool {
        self.start.call_once(|| {
            let enabled = f();
            self.enabled.store(enabled, Ordering::SeqCst);
        });

        self.enabled.load(Ordering::SeqCst)
    }
}
