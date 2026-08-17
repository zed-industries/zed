use crate::{CExternType, wasm_externtype_t, wasm_limits_t, wasm_valtype_t};
use std::cell::OnceCell;
use wasmtime::{TableType, ValType};

#[repr(transparent)]
#[derive(Clone)]
pub struct wasm_tabletype_t {
    ext: wasm_externtype_t,
}

wasmtime_c_api_macros::declare_ty!(wasm_tabletype_t);

#[derive(Clone)]
pub(crate) struct CTableType {
    pub(crate) ty: TableType,
    element_cache: OnceCell<wasm_valtype_t>,
    limits_cache: OnceCell<wasm_limits_t>,
}

impl wasm_tabletype_t {
    pub(crate) fn new(ty: TableType) -> wasm_tabletype_t {
        wasm_tabletype_t {
            ext: wasm_externtype_t::from_extern_type(ty.into()),
        }
    }

    pub(crate) fn try_from(e: &wasm_externtype_t) -> Option<&wasm_tabletype_t> {
        match &e.which {
            CExternType::Table(_) => Some(unsafe { &*(e as *const _ as *const _) }),
            _ => None,
        }
    }

    pub(crate) fn ty(&self) -> &CTableType {
        match &self.ext.which {
            CExternType::Table(f) => &f,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl CTableType {
    pub(crate) fn new(ty: TableType) -> CTableType {
        CTableType {
            ty,
            element_cache: OnceCell::new(),
            limits_cache: OnceCell::new(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_tabletype_new(
    ty: Box<wasm_valtype_t>,
    limits: &wasm_limits_t,
) -> Option<Box<wasm_tabletype_t>> {
    let ty = ty.ty.as_ref()?.clone();
    Some(Box::new(wasm_tabletype_t::new(TableType::new(
        ty,
        limits.min,
        limits.max(),
    ))))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_tabletype_element(tt: &wasm_tabletype_t) -> &wasm_valtype_t {
    let tt = tt.ty();
    tt.element_cache.get_or_init(|| wasm_valtype_t {
        ty: ValType::Ref(tt.ty.element().clone()),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_tabletype_limits(tt: &wasm_tabletype_t) -> &wasm_limits_t {
    let tt = tt.ty();
    tt.limits_cache.get_or_init(|| wasm_limits_t {
        min: u32::try_from(tt.ty.minimum()).unwrap(),
        max: u32::try_from(tt.ty.maximum().unwrap_or(u64::from(u32::MAX))).unwrap(),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_tabletype_as_externtype(ty: &wasm_tabletype_t) -> &wasm_externtype_t {
    &ty.ext
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_tabletype_as_externtype_const(ty: &wasm_tabletype_t) -> &wasm_externtype_t {
    &ty.ext
}
