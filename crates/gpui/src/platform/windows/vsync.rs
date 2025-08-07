pub(crate) struct VsyncProvider {
    delta: Duration,
    f: Box<dyn Fn() + Send>,
}

impl VsyncProvider {
    pub fn new() -> Self {
        Self {
            delta: Duration::from_millis(16),
        }
    }
}
