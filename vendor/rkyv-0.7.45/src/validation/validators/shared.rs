//! Validators add validation capabilities by wrapping and extending basic validators.

use crate::{validation::SharedContext, Fallible};
use core::{any::TypeId, fmt};

#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

/// Errors that can occur when checking shared memory.
#[derive(Debug)]
pub enum SharedError {
    /// Multiple pointers exist to the same location with different types
    TypeMismatch {
        /// A previous type that the location was checked as
        previous: TypeId,
        /// The current type that the location is checked as
        current: TypeId,
    },
}

impl fmt::Display for SharedError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SharedError::TypeMismatch { previous, current } => write!(
                f,
                "the same memory region has been claimed as two different types ({:?} and {:?})",
                previous, current
            ),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl Error for SharedError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                SharedError::TypeMismatch { .. } => None,
            }
        }
    }
};

/// A validator that can verify shared memory.
#[derive(Debug)]
pub struct SharedValidator {
    shared: HashMap<*const u8, TypeId>,
}

// SAFETY: SharedValidator is safe to send to another thread
// This trait is not automatically implemented because the struct contains a pointer
unsafe impl Send for SharedValidator {}

// SAFETY: SharedValidator is safe to share between threads
// This trait is not automatically implemented because the struct contains a pointer
unsafe impl Sync for SharedValidator {}

impl SharedValidator {
    /// Wraps the given context and adds shared memory validation.
    #[inline]
    pub fn new() -> Self {
        Self {
            // TODO: consider deferring this to avoid the overhead of constructing
            shared: HashMap::new(),
        }
    }

    /// Shared memory validator with specific capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            shared: HashMap::with_capacity(capacity),
        }
    }
}

impl Default for SharedValidator {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Fallible for SharedValidator {
    type Error = SharedError;
}

impl SharedContext for SharedValidator {
    #[inline]
    fn register_shared_ptr(
        &mut self,
        ptr: *const u8,
        type_id: TypeId,
    ) -> Result<bool, Self::Error> {
        #[cfg(not(feature = "std"))]
        use hashbrown::hash_map::Entry;
        #[cfg(feature = "std")]
        use std::collections::hash_map::Entry;

        match self.shared.entry(ptr) {
            Entry::Occupied(previous_type_entry) => {
                let previous_type_id = previous_type_entry.get();
                if previous_type_id != &type_id {
                    Err(SharedError::TypeMismatch {
                        previous: *previous_type_id,
                        current: type_id,
                    })
                } else {
                    Ok(false)
                }
            }
            Entry::Vacant(ent) => {
                ent.insert(type_id);
                Ok(true)
            }
        }
    }
}
