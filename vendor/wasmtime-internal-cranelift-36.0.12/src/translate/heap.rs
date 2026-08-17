//! Heaps to implement WebAssembly linear memories.

use cranelift_codegen::ir::{self, GlobalValue, MemoryType, Type};
use cranelift_entity::entity_impl;
use wasmtime_environ::{IndexType, Memory};

/// An opaque reference to a [`HeapData`][crate::HeapData].
///
/// While the order is stable, it is arbitrary.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Heap(u32);
entity_impl!(Heap, "heap");

/// A heap implementing a WebAssembly linear memory.
///
/// Code compiled from WebAssembly runs in a sandbox where it can't access all
/// process memory. Instead, it is given a small set of memory areas to work in,
/// and all accesses are bounds checked. Wasmtime models this through
/// the concept of *heaps*.
///
/// Heap addresses can be smaller than the native pointer size, for example
/// unsigned `i32` offsets on a 64-bit architecture.
///
/// A heap appears as three consecutive ranges of address space:
///
/// 1. The *mapped pages* are the accessible memory range in the heap. A heap
///    may have a minimum guaranteed size which means that some mapped pages are
///    always present.
///
/// 2. The *unmapped pages* is a possibly empty range of address space that may
///    be mapped in the future when the heap is grown. They are addressable but
///    not accessible.
///
/// 3. The *offset-guard pages* is a range of address space that is guaranteed
///    to always cause a trap when accessed. It is used to optimize bounds
///    checking for heap accesses with a shared base pointer. They are
///    addressable but not accessible.
#[derive(Clone, PartialEq, Hash)]
pub struct HeapData {
    /// The address of the start of the heap's storage.
    pub base: GlobalValue,

    /// The dynamic byte length of this heap, if needed.
    pub bound: GlobalValue,

    /// The type of wasm memory that this heap is operating on.
    pub memory: Memory,

    /// The memory type for the pointed-to memory, if using proof-carrying code.
    pub pcc_memory_type: Option<MemoryType>,
}

impl HeapData {
    pub fn index_type(&self) -> Type {
        match self.memory.idx_type {
            IndexType::I32 => ir::types::I32,
            IndexType::I64 => ir::types::I64,
        }
    }
}
