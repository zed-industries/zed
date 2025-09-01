use std::{sync::Arc, task::Wake};

pub(crate) struct WakerFn<F> {
    f: F,
}

impl<F: Fn()> WakerFn<F> {
    pub(crate) fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F: Fn()> Wake for WakerFn<F> {
    fn wake(self: Arc<Self>) {
        (self.f)();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        (self.f)();
    }
}
