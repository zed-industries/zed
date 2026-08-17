use std::fmt;
use std::num::NonZeroU64;

/// An opaque ID that uniquely identifies a runtime relative to all other currently
/// running runtimes.
///
/// # Notes
///
/// - Runtime IDs are unique relative to other *currently running* runtimes.
///   When a runtime completes, the same ID may be used for another runtime.
/// - Runtime IDs are *not* sequential, and do not indicate the order in which
///   runtimes are started or any other data.
/// - The runtime ID of the currently running task can be obtained from the
///   Handle.
///
/// # Examples
///
/// ```
/// # #[cfg(not(target_family = "wasm"))]
/// # {
/// use tokio::runtime::Handle;
///
/// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
/// async fn main() {
///   println!("Current runtime id: {}", Handle::current().id());
/// }
/// # }
/// ```
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Id(NonZeroU64);

impl Id {
    pub(crate) fn new(integer: impl Into<NonZeroU64>) -> Self {
        Self(integer.into())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
