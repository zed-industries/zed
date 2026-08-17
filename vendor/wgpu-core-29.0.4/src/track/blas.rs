use crate::{
    resource::{Blas, Trackable},
    track::metadata::ResourceMetadata,
};
use alloc::sync::Arc;

/// A tracker that holds tracks BLASes.
///
/// This is mostly a safe shell around [`ResourceMetadata`]
#[derive(Debug)]
pub(crate) struct BlasTracker {
    metadata: ResourceMetadata<Arc<Blas>>,
    size: usize,
}

impl BlasTracker {
    pub fn new() -> Self {
        Self {
            metadata: ResourceMetadata::new(),
            size: 0,
        }
    }

    /// Inserts a single resource into the resource tracker.
    ///
    /// Returns a reference to the newly inserted resource.
    /// (This allows avoiding a clone/reference count increase in many cases.)
    pub fn insert_single(&mut self, resource: Arc<Blas>) -> &Arc<Blas> {
        let index = resource.tracker_index().as_usize();
        self.allow_index(index);

        unsafe {
            // # SAFETY: we just allowed this resource, which makes the metadata object larger if
            // it's not in bounds
            self.metadata.insert(index, resource)
        }
    }

    /// Sets the size of all the vectors inside the tracker.
    ///
    /// Must be called with the highest possible Texture ID before
    /// all unsafe functions are called.
    pub fn set_size(&mut self, size: usize) {
        self.size = size;
        self.metadata.set_size(size)
    }

    /// Extend the vectors to let the given index be valid.
    fn allow_index(&mut self, index: usize) {
        if index >= self.size {
            self.set_size(index + 1);
        }
    }

    /// Returns true if the tracker owns the given texture.
    pub fn contains(&self, blas: &Blas) -> bool {
        self.metadata.contains(blas.tracker_index().as_usize())
    }
}
