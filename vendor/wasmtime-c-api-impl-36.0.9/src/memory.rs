use crate::{
    WasmtimeStoreContext, WasmtimeStoreContextMut, handle_result, wasm_extern_t, wasm_memorytype_t,
    wasm_store_t, wasmtime_error_t,
};
use std::convert::TryFrom;
use wasmtime::{Extern, Memory};

#[derive(Clone)]
#[repr(transparent)]
pub struct wasm_memory_t {
    ext: wasm_extern_t,
}

wasmtime_c_api_macros::declare_ref!(wasm_memory_t);

pub type wasm_memory_pages_t = u32;

impl wasm_memory_t {
    pub(crate) fn try_from(e: &wasm_extern_t) -> Option<&wasm_memory_t> {
        match &e.which {
            Extern::Memory(_) => Some(unsafe { &*(e as *const _ as *const _) }),
            _ => None,
        }
    }

    fn memory(&self) -> Memory {
        match self.ext.which {
            Extern::Memory(m) => m,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_memory_new(
    store: &mut wasm_store_t,
    mt: &wasm_memorytype_t,
) -> Option<Box<wasm_memory_t>> {
    let memory = Memory::new(store.store.context_mut(), mt.ty().ty.clone()).ok()?;
    Some(Box::new(wasm_memory_t {
        ext: wasm_extern_t {
            store: store.store.clone(),
            which: memory.into(),
        },
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_memory_as_extern(m: &mut wasm_memory_t) -> &mut wasm_extern_t {
    &mut m.ext
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_memory_as_extern_const(m: &wasm_memory_t) -> &wasm_extern_t {
    &m.ext
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_memory_type(m: &wasm_memory_t) -> Box<wasm_memorytype_t> {
    let ty = m.memory().ty(m.ext.store.context());
    Box::new(wasm_memorytype_t::new(ty))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_memory_data(m: &wasm_memory_t) -> *mut u8 {
    m.memory().data_ptr(m.ext.store.context())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_memory_data_size(m: &wasm_memory_t) -> usize {
    m.memory().data_size(m.ext.store.context())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_memory_size(m: &wasm_memory_t) -> wasm_memory_pages_t {
    u32::try_from(m.memory().size(m.ext.store.context())).unwrap()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_memory_grow(
    m: &mut wasm_memory_t,
    delta: wasm_memory_pages_t,
) -> bool {
    let memory = m.memory();
    let mut store = m.ext.store.context_mut();
    memory.grow(&mut store, u64::from(delta)).is_ok()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memory_new(
    store: WasmtimeStoreContextMut<'_>,
    ty: &wasm_memorytype_t,
    ret: &mut Memory,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(Memory::new(store, ty.ty().ty.clone()), |mem| *ret = mem)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memory_type(
    store: WasmtimeStoreContext<'_>,
    mem: &Memory,
) -> Box<wasm_memorytype_t> {
    Box::new(wasm_memorytype_t::new(mem.ty(store)))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memory_data(store: WasmtimeStoreContext<'_>, mem: &Memory) -> *const u8 {
    mem.data(store).as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memory_data_size(
    store: WasmtimeStoreContext<'_>,
    mem: &Memory,
) -> usize {
    mem.data(store).len()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memory_size(store: WasmtimeStoreContext<'_>, mem: &Memory) -> u64 {
    mem.size(store)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memory_grow(
    store: WasmtimeStoreContextMut<'_>,
    mem: &Memory,
    delta: u64,
    prev_size: &mut u64,
) -> Option<Box<wasmtime_error_t>> {
    handle_result(mem.grow(store, delta), |prev| *prev_size = prev)
}
