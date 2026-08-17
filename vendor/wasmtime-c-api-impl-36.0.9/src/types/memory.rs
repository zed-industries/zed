use crate::{CExternType, wasm_externtype_t, wasm_limits_t};
use std::cell::OnceCell;
use std::convert::TryFrom;
use wasmtime::{MemoryType, MemoryTypeBuilder};

#[repr(transparent)]
#[derive(Clone)]
pub struct wasm_memorytype_t {
    ext: wasm_externtype_t,
}

wasmtime_c_api_macros::declare_ty!(wasm_memorytype_t);

#[derive(Clone)]
pub(crate) struct CMemoryType {
    pub(crate) ty: MemoryType,
    limits_cache: OnceCell<wasm_limits_t>,
}

impl wasm_memorytype_t {
    pub(crate) fn new(ty: MemoryType) -> wasm_memorytype_t {
        wasm_memorytype_t {
            ext: wasm_externtype_t::from_extern_type(ty.into()),
        }
    }

    pub(crate) fn try_from(e: &wasm_externtype_t) -> Option<&wasm_memorytype_t> {
        match &e.which {
            CExternType::Memory(_) => Some(unsafe { &*(e as *const _ as *const _) }),
            _ => None,
        }
    }

    pub(crate) fn ty(&self) -> &CMemoryType {
        match &self.ext.which {
            CExternType::Memory(f) => &f,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl CMemoryType {
    pub(crate) fn new(ty: MemoryType) -> CMemoryType {
        CMemoryType {
            ty,
            limits_cache: OnceCell::new(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_memorytype_new(limits: &wasm_limits_t) -> Box<wasm_memorytype_t> {
    Box::new(wasm_memorytype_t::new(MemoryType::new(
        limits.min,
        limits.max(),
    )))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_memorytype_limits(mt: &wasm_memorytype_t) -> &wasm_limits_t {
    let mt = mt.ty();
    mt.limits_cache.get_or_init(|| wasm_limits_t {
        min: u32::try_from(mt.ty.minimum()).unwrap(),
        max: u32::try_from(mt.ty.maximum().unwrap_or(u64::from(u32::max_value()))).unwrap(),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memorytype_new(
    minimum: u64,
    maximum_specified: bool,
    maximum: u64,
    memory64: bool,
    shared: bool,
) -> Box<wasm_memorytype_t> {
    let maximum = if maximum_specified {
        Some(maximum)
    } else {
        None
    };

    Box::new(wasm_memorytype_t::new(
        MemoryTypeBuilder::default()
            .min(minimum)
            .max(maximum)
            .memory64(memory64)
            .shared(shared)
            .build()
            .unwrap(),
    ))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memorytype_minimum(mt: &wasm_memorytype_t) -> u64 {
    mt.ty().ty.minimum()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memorytype_maximum(mt: &wasm_memorytype_t, out: &mut u64) -> bool {
    match mt.ty().ty.maximum() {
        Some(max) => {
            *out = max;
            true
        }
        None => false,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memorytype_is64(mt: &wasm_memorytype_t) -> bool {
    mt.ty().ty.is_64()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_memorytype_isshared(mt: &wasm_memorytype_t) -> bool {
    mt.ty().ty.is_shared()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_memorytype_as_externtype(ty: &wasm_memorytype_t) -> &wasm_externtype_t {
    &ty.ext
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_memorytype_as_externtype_const(
    ty: &wasm_memorytype_t,
) -> &wasm_externtype_t {
    &ty.ext
}
