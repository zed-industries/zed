//! Deserialization traits, deserializers, and adapters.

pub mod deserializers;

#[cfg(feature = "alloc")]
use crate::{ArchiveUnsized, DeserializeUnsized, Fallible};
#[cfg(all(feature = "alloc", not(feature = "std")))]
use ::alloc::boxed::Box;
#[cfg(feature = "alloc")]
use ::core::alloc::Layout;

/// A deserializable shared pointer type.
#[cfg(feature = "alloc")]
pub trait SharedPointer {
    /// Returns the data address for this shared pointer.
    fn data_address(&self) -> *const ();
}

/// A registry that tracks deserialized shared memory.
///
/// This trait is required to deserialize shared pointers.
#[cfg(feature = "alloc")]
pub trait SharedDeserializeRegistry: Fallible {
    /// Gets the data pointer of a previously-deserialized shared pointer.
    fn get_shared_ptr(&mut self, ptr: *const u8) -> Option<&dyn SharedPointer>;

    /// Adds the data address of a deserialized shared pointer to the registry.
    fn add_shared_ptr(
        &mut self,
        ptr: *const u8,
        shared: Box<dyn SharedPointer>,
    ) -> Result<(), Self::Error>;

    /// Checks whether the given reference has been deserialized and either uses the existing shared
    /// pointer to it, or deserializes it and converts it to a shared pointer with `to_shared`.
    #[inline]
    fn deserialize_shared<T, P, F, A>(
        &mut self,
        value: &T::Archived,
        to_shared: F,
        alloc: A,
    ) -> Result<*const T, Self::Error>
    where
        T: ArchiveUnsized + ?Sized,
        P: SharedPointer + 'static,
        F: FnOnce(*mut T) -> P,
        A: FnMut(Layout) -> *mut u8,
        T::Archived: DeserializeUnsized<T, Self>,
    {
        let ptr = value as *const T::Archived as *const u8;
        let metadata = T::Archived::deserialize_metadata(value, self)?;

        if let Some(shared_pointer) = self.get_shared_ptr(ptr) {
            Ok(ptr_meta::from_raw_parts(
                shared_pointer.data_address(),
                metadata,
            ))
        } else {
            let deserialized_data = unsafe { value.deserialize_unsized(self, alloc)? };
            let shared_ptr = to_shared(ptr_meta::from_raw_parts_mut(deserialized_data, metadata));
            let data_address = shared_ptr.data_address();

            self.add_shared_ptr(ptr, Box::new(shared_ptr) as Box<dyn SharedPointer>)?;
            Ok(ptr_meta::from_raw_parts(data_address, metadata))
        }
    }
}
