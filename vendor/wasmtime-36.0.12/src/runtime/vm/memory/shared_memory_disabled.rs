use crate::prelude::*;
use crate::runtime::vm::memory::LocalMemory;
use crate::runtime::vm::{VMMemoryDefinition, VMStore, WaitResult};
use core::ops::Range;
use core::ptr::NonNull;
use core::time::Duration;
use wasmtime_environ::Trap;

#[derive(Clone)]
pub enum SharedMemory {}

impl SharedMemory {
    pub fn wrap(_ty: &wasmtime_environ::Memory, _memory: LocalMemory) -> Result<Self> {
        bail!("support for shared memories was disabled at compile time");
    }

    pub fn ty(&self) -> wasmtime_environ::Memory {
        match *self {}
    }

    pub fn as_memory(self) -> crate::runtime::vm::Memory {
        match self {}
    }

    pub fn vmmemory_ptr(&self) -> NonNull<VMMemoryDefinition> {
        match *self {}
    }

    pub fn grow(
        &self,
        _delta_pages: u64,
        _store: Option<&mut dyn VMStore>,
    ) -> Result<Option<(usize, usize)>> {
        match *self {}
    }

    pub fn atomic_notify(&self, _addr_index: u64, _count: u32) -> Result<u32, Trap> {
        match *self {}
    }

    pub fn atomic_wait32(
        &self,
        _addr_index: u64,
        _expected: u32,
        _timeout: Option<Duration>,
    ) -> Result<WaitResult, Trap> {
        match *self {}
    }

    pub fn atomic_wait64(
        &self,
        _addr_index: u64,
        _expected: u64,
        _timeout: Option<Duration>,
    ) -> Result<WaitResult, Trap> {
        match *self {}
    }

    pub(crate) fn page_size(&self) -> u64 {
        match *self {}
    }

    pub(crate) fn byte_size(&self) -> usize {
        match *self {}
    }

    pub(crate) fn needs_init(&self) -> bool {
        match *self {}
    }

    pub(crate) fn wasm_accessible(&self) -> Range<usize> {
        match *self {}
    }
}
