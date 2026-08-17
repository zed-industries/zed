use alloc::{string::String, vec::Vec};

use parking_lot::Mutex;

/// Stores the text of any validation errors that have occurred since
/// the last call to `get_and_reset`.
///
/// Each value is a validation error and a message associated with it,
/// or `None` if the error has no message from the api.
///
/// This is used for internal wgpu testing only and _must not_ be used
/// as a way to check for errors.
///
/// This works as a static because `cargo nextest` runs all of our
/// tests in separate processes, so each test gets its own canary.
///
/// This prevents the issue of one validation error terminating the
/// entire process.
pub static VALIDATION_CANARY: ValidationCanary = ValidationCanary {
    inner: Mutex::new(Vec::new()),
};

/// Flag for internal testing.
pub struct ValidationCanary {
    inner: Mutex<Vec<String>>,
}

impl ValidationCanary {
    #[allow(dead_code)] // in some configurations this function is dead
    pub(crate) fn add(&self, msg: String) {
        self.inner.lock().push(msg);
    }

    /// Returns any API validation errors that have occurred in this process
    /// since the last call to this function.
    pub fn get_and_reset(&self) -> Vec<String> {
        self.inner.lock().drain(..).collect()
    }
}
