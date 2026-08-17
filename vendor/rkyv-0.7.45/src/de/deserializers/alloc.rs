//! Adapters wrap deserializers and add support for deserializer traits.

use crate::{
    de::{SharedDeserializeRegistry, SharedPointer},
    Fallible,
};
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::fmt;
#[cfg(not(feature = "std"))]
use hashbrown::hash_map;
#[cfg(feature = "std")]
use std::collections::hash_map;

/// An error that can occur while deserializing shared pointers.
#[derive(Debug)]
pub enum SharedDeserializeMapError {
    /// A shared pointer was added multiple times
    DuplicateSharedPointer(*const u8),
}

// SAFETY: SharedDeserializeMapError is safe to send to another thread
// This trait is not automatically implemented because the enum contains a pointer
unsafe impl Send for SharedDeserializeMapError {}

// SAFETY: SharedDeserializeMapError is safe to share between threads
// This trait is not automatically implemented because the enum contains a pointer
unsafe impl Sync for SharedDeserializeMapError {}

impl fmt::Display for SharedDeserializeMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateSharedPointer(p) => write!(f, "duplicate shared pointer: {:p}", p),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl Error for SharedDeserializeMapError {}
};

/// An adapter that adds shared deserialization support to a deserializer.
pub struct SharedDeserializeMap {
    shared_pointers: hash_map::HashMap<*const u8, Box<dyn SharedPointer>>,
}

impl SharedDeserializeMap {
    /// Wraps the given deserializer and adds shared memory support.
    #[inline]
    pub fn new() -> Self {
        Self {
            shared_pointers: hash_map::HashMap::new(),
        }
    }

    /// Wraps the given deserializer and adds shared memory support, with initial capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            shared_pointers: hash_map::HashMap::with_capacity(capacity),
        }
    }
}

impl fmt::Debug for SharedDeserializeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(
                self.shared_pointers
                    .iter()
                    .map(|(s, p)| (s, &**p as *const _)),
            )
            .finish()
    }
}

impl Default for SharedDeserializeMap {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Fallible for SharedDeserializeMap {
    type Error = SharedDeserializeMapError;
}

impl SharedDeserializeRegistry for SharedDeserializeMap {
    fn get_shared_ptr(&mut self, ptr: *const u8) -> Option<&dyn SharedPointer> {
        self.shared_pointers.get(&ptr).map(|p| p.as_ref())
    }

    fn add_shared_ptr(
        &mut self,
        ptr: *const u8,
        shared: Box<dyn SharedPointer>,
    ) -> Result<(), Self::Error> {
        match self.shared_pointers.entry(ptr) {
            hash_map::Entry::Occupied(_) => {
                Err(SharedDeserializeMapError::DuplicateSharedPointer(ptr))
            }
            hash_map::Entry::Vacant(e) => {
                e.insert(shared);
                Ok(())
            }
        }
    }
}
