//! Implementation of the side table for `externref` host data.
//!
//! The actual host data is kept in a side table, rather than inside the GC
//! heap, because we do not trust any data coming from the GC heap. If we
//! constructed `&dyn Any`s from GC heap data and called any function loaded
//! from the `dyn Any`'s vtable, then any bug in our collector could lead to
//! corrupted vtables, which could lead to security vulnerabilities and sandbox
//! escapes.
//!
//! Much better to store host data IDs inside the GC heap, and then do checked
//! accesses into the host data table from those untrusted IDs. At worst, we can
//! return the wrong (but still valid) host data object or panic. This is way
//! less catastrophic than doing an indirect call to an attacker-controlled
//! function pointer.

use crate::prelude::*;
use core::any::Any;
use wasmtime_slab::{Id, Slab};

/// Side table for each `externref`'s host data value.
#[derive(Default)]
pub struct ExternRefHostDataTable {
    slab: Slab<Box<dyn Any + Send + Sync>>,
}

/// ID into the `externref` host data table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ExternRefHostDataId(Id);

fn deref_box<T: ?Sized>(b: &Box<T>) -> &T {
    &**b
}

fn deref_box_mut<T: ?Sized>(b: &mut Box<T>) -> &mut T {
    &mut **b
}

impl ExternRefHostDataTable {
    /// Allocate a new `externref` host data value.
    pub fn alloc(&mut self, value: Box<dyn Any + Send + Sync>) -> ExternRefHostDataId {
        let id = self.slab.alloc(value);
        let id = ExternRefHostDataId(id);
        log::trace!("allocated new externref host data: {id:?}");
        id
    }

    /// Deallocate an `externref` host data value.
    pub fn dealloc(&mut self, id: ExternRefHostDataId) -> Box<dyn Any + Send + Sync> {
        log::trace!("deallocated externref host data: {id:?}");
        self.slab.dealloc(id.0)
    }

    /// Get a shared borrow of the host data associated with the given ID.
    pub fn get(&self, id: ExternRefHostDataId) -> &(dyn Any + Send + Sync) {
        let data: &Box<dyn Any + Send + Sync> = self.slab.get(id.0).unwrap();
        deref_box(data)
    }

    /// Get a mutable borrow of the host data associated with the given ID.
    pub fn get_mut(&mut self, id: ExternRefHostDataId) -> &mut (dyn Any + Send + Sync) {
        let data: &mut Box<dyn Any + Send + Sync> = self.slab.get_mut(id.0).unwrap();
        deref_box_mut(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_dyn_object() {
        let mut table = ExternRefHostDataTable::default();

        let x = 42_u32;
        let id = table.alloc(Box::new(x));
        assert!(table.get(id).is::<u32>());
        assert_eq!(*table.get(id).downcast_ref::<u32>().unwrap(), 42);
        assert!(table.get_mut(id).is::<u32>());
        assert_eq!(*table.get_mut(id).downcast_ref::<u32>().unwrap(), 42);
    }
}
