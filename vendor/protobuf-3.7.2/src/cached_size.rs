use std::hash::Hash;
use std::hash::Hasher;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// Cached size field used in generated code.
///
/// It is always equal to itself to simplify generated code.
/// (Generated code can use `#[derive(Eq)]`).
///
/// This type should rarely be used directly.
#[derive(Debug, Default)]
pub struct CachedSize {
    size: AtomicUsize,
}

impl CachedSize {
    /// Create a new `CachedSize` object.
    pub const fn new() -> CachedSize {
        CachedSize {
            size: AtomicUsize::new(0),
        }
    }

    /// Get cached size
    pub fn get(&self) -> u32 {
        self.size.load(Ordering::Relaxed) as u32
    }

    /// Set cached size
    pub fn set(&self, size: u32) {
        self.size.store(size as usize, Ordering::Relaxed)
    }
}

impl Clone for CachedSize {
    fn clone(&self) -> CachedSize {
        CachedSize {
            size: AtomicUsize::new(self.size.load(Ordering::Relaxed)),
        }
    }
}

impl PartialEq<CachedSize> for CachedSize {
    fn eq(&self, _other: &CachedSize) -> bool {
        true
    }
}

impl Eq for CachedSize {}

impl Hash for CachedSize {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // ignore cached size in cache computation
    }
}
