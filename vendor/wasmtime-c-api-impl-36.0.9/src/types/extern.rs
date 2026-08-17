use crate::{CFuncType, CGlobalType, CMemoryType, CTableType};
use crate::{wasm_functype_t, wasm_globaltype_t, wasm_memorytype_t, wasm_tabletype_t};
use wasmtime::ExternType;

#[repr(C)]
#[derive(Clone)]
pub struct wasm_externtype_t {
    pub(crate) which: CExternType,
}

wasmtime_c_api_macros::declare_ty!(wasm_externtype_t);

#[derive(Clone)]
pub(crate) enum CExternType {
    Func(CFuncType),
    Global(CGlobalType),
    Memory(CMemoryType),
    Table(CTableType),
}

impl CExternType {
    pub(crate) fn new(ty: ExternType) -> CExternType {
        match ty {
            ExternType::Func(f) => CExternType::Func(CFuncType::new(f)),
            ExternType::Global(f) => CExternType::Global(CGlobalType::new(f)),
            ExternType::Memory(f) => CExternType::Memory(CMemoryType::new(f)),
            ExternType::Table(f) => CExternType::Table(CTableType::new(f)),
            ExternType::Tag(_) => todo!(), // FIXME: #10252 C embedder API for exceptions and control tags.
        }
    }
}

pub type wasm_externkind_t = u8;

pub const WASM_EXTERN_FUNC: wasm_externkind_t = 0;
pub const WASM_EXTERN_GLOBAL: wasm_externkind_t = 1;
pub const WASM_EXTERN_TABLE: wasm_externkind_t = 2;
pub const WASM_EXTERN_MEMORY: wasm_externkind_t = 3;

impl wasm_externtype_t {
    pub(crate) fn from_extern_type(ty: ExternType) -> wasm_externtype_t {
        wasm_externtype_t {
            which: CExternType::new(ty),
        }
    }

    pub(crate) fn from_cextern_type(ty: CExternType) -> wasm_externtype_t {
        wasm_externtype_t { which: ty }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_externtype_kind(et: &wasm_externtype_t) -> wasm_externkind_t {
    match &et.which {
        CExternType::Func(_) => WASM_EXTERN_FUNC,
        CExternType::Table(_) => WASM_EXTERN_TABLE,
        CExternType::Global(_) => WASM_EXTERN_GLOBAL,
        CExternType::Memory(_) => WASM_EXTERN_MEMORY,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_externtype_as_functype(et: &wasm_externtype_t) -> Option<&wasm_functype_t> {
    wasm_externtype_as_functype_const(et)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_externtype_as_functype_const(
    et: &wasm_externtype_t,
) -> Option<&wasm_functype_t> {
    wasm_functype_t::try_from(et)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_externtype_as_globaltype(
    et: &wasm_externtype_t,
) -> Option<&wasm_globaltype_t> {
    wasm_externtype_as_globaltype_const(et)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_externtype_as_globaltype_const(
    et: &wasm_externtype_t,
) -> Option<&wasm_globaltype_t> {
    wasm_globaltype_t::try_from(et)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_externtype_as_tabletype(
    et: &wasm_externtype_t,
) -> Option<&wasm_tabletype_t> {
    wasm_externtype_as_tabletype_const(et)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_externtype_as_tabletype_const(
    et: &wasm_externtype_t,
) -> Option<&wasm_tabletype_t> {
    wasm_tabletype_t::try_from(et)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_externtype_as_memorytype(
    et: &wasm_externtype_t,
) -> Option<&wasm_memorytype_t> {
    wasm_externtype_as_memorytype_const(et)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_externtype_as_memorytype_const(
    et: &wasm_externtype_t,
) -> Option<&wasm_memorytype_t> {
    wasm_memorytype_t::try_from(et)
}
