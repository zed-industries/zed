use crate::{
    WasmStoreRef, WasmtimeStoreContext, wasm_externkind_t, wasm_externtype_t, wasm_func_t,
    wasm_global_t, wasm_memory_t, wasm_table_t,
};
use std::mem::ManuallyDrop;
use wasmtime::{Extern, Func, Global, Memory, SharedMemory, Table};

#[derive(Clone)]
pub struct wasm_extern_t {
    pub(crate) store: WasmStoreRef,
    pub(crate) which: Extern,
}

wasmtime_c_api_macros::declare_ref!(wasm_extern_t);

#[unsafe(no_mangle)]
pub extern "C" fn wasm_extern_kind(e: &wasm_extern_t) -> wasm_externkind_t {
    match e.which {
        Extern::Func(_) => crate::WASM_EXTERN_FUNC,
        Extern::Global(_) => crate::WASM_EXTERN_GLOBAL,
        Extern::Table(_) => crate::WASM_EXTERN_TABLE,
        Extern::Memory(_) => crate::WASM_EXTERN_MEMORY,
        Extern::SharedMemory(_) => panic!(
            "Shared Memory no implemented for wasm_* types. Please use wasmtime_* types instead"
        ),
        Extern::Tag(_) => todo!(), // FIXME: #10252 C embedder API for exceptions and control tags.
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_extern_type(e: &wasm_extern_t) -> Box<wasm_externtype_t> {
    Box::new(wasm_externtype_t::from_extern_type(
        e.which.ty(&e.store.context()),
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_extern_as_func(e: &wasm_extern_t) -> Option<&wasm_func_t> {
    wasm_func_t::try_from(e)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_extern_as_func_const(e: &wasm_extern_t) -> Option<&wasm_func_t> {
    wasm_extern_as_func(e)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_extern_as_global(e: &wasm_extern_t) -> Option<&wasm_global_t> {
    wasm_global_t::try_from(e)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_extern_as_global_const(e: &wasm_extern_t) -> Option<&wasm_global_t> {
    wasm_extern_as_global(e)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_extern_as_table(e: &wasm_extern_t) -> Option<&wasm_table_t> {
    wasm_table_t::try_from(e)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_extern_as_table_const(e: &wasm_extern_t) -> Option<&wasm_table_t> {
    wasm_extern_as_table(e)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_extern_as_memory(e: &wasm_extern_t) -> Option<&wasm_memory_t> {
    wasm_memory_t::try_from(e)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_extern_as_memory_const(e: &wasm_extern_t) -> Option<&wasm_memory_t> {
    wasm_extern_as_memory(e)
}

#[repr(C)]
pub struct wasmtime_extern_t {
    pub kind: wasmtime_extern_kind_t,
    pub of: wasmtime_extern_union,
}

pub type wasmtime_extern_kind_t = u8;
pub const WASMTIME_EXTERN_FUNC: wasmtime_extern_kind_t = 0;
pub const WASMTIME_EXTERN_GLOBAL: wasmtime_extern_kind_t = 1;
pub const WASMTIME_EXTERN_TABLE: wasmtime_extern_kind_t = 2;
pub const WASMTIME_EXTERN_MEMORY: wasmtime_extern_kind_t = 3;
pub const WASMTIME_EXTERN_SHAREDMEMORY: wasmtime_extern_kind_t = 4;

#[repr(C)]
pub union wasmtime_extern_union {
    pub func: Func,
    pub table: Table,
    pub global: Global,
    pub memory: Memory,
    pub sharedmemory: ManuallyDrop<Box<SharedMemory>>,
}

impl Drop for wasmtime_extern_t {
    fn drop(&mut self) {
        if self.kind == WASMTIME_EXTERN_SHAREDMEMORY {
            unsafe {
                ManuallyDrop::drop(&mut self.of.sharedmemory);
            }
        }
    }
}

impl wasmtime_extern_t {
    pub unsafe fn to_extern(&self) -> Extern {
        match self.kind {
            WASMTIME_EXTERN_FUNC => Extern::Func(self.of.func),
            WASMTIME_EXTERN_GLOBAL => Extern::Global(self.of.global),
            WASMTIME_EXTERN_TABLE => Extern::Table(self.of.table),
            WASMTIME_EXTERN_MEMORY => Extern::Memory(self.of.memory),
            WASMTIME_EXTERN_SHAREDMEMORY => Extern::SharedMemory((**self.of.sharedmemory).clone()),
            other => panic!("unknown wasmtime_extern_kind_t: {other}"),
        }
    }
}

impl From<Extern> for wasmtime_extern_t {
    fn from(item: Extern) -> wasmtime_extern_t {
        match item {
            Extern::Func(func) => wasmtime_extern_t {
                kind: WASMTIME_EXTERN_FUNC,
                of: wasmtime_extern_union { func },
            },
            Extern::Global(global) => wasmtime_extern_t {
                kind: WASMTIME_EXTERN_GLOBAL,
                of: wasmtime_extern_union { global },
            },
            Extern::Table(table) => wasmtime_extern_t {
                kind: WASMTIME_EXTERN_TABLE,
                of: wasmtime_extern_union { table },
            },
            Extern::Memory(memory) => wasmtime_extern_t {
                kind: WASMTIME_EXTERN_MEMORY,
                of: wasmtime_extern_union { memory },
            },
            Extern::SharedMemory(sharedmemory) => wasmtime_extern_t {
                kind: WASMTIME_EXTERN_SHAREDMEMORY,
                of: wasmtime_extern_union {
                    sharedmemory: ManuallyDrop::new(Box::new(sharedmemory)),
                },
            },
            Extern::Tag(_) => todo!(), // FIXME: #10252 C embedder API for exceptions and control tags.
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_extern_delete(e: &mut ManuallyDrop<wasmtime_extern_t>) {
    ManuallyDrop::drop(e);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_extern_type(
    store: WasmtimeStoreContext<'_>,
    e: &wasmtime_extern_t,
) -> Box<wasm_externtype_t> {
    Box::new(wasm_externtype_t::from_extern_type(e.to_extern().ty(store)))
}
