use crate::{CExternType, wasm_externtype_t, wasm_valtype_t};
use std::cell::OnceCell;
use wasmtime::GlobalType;

pub type wasm_mutability_t = u8;

pub const WASM_CONST: wasm_mutability_t = 0;
pub const WASM_VAR: wasm_mutability_t = 1;

#[repr(transparent)]
#[derive(Clone)]
pub struct wasm_globaltype_t {
    ext: wasm_externtype_t,
}

wasmtime_c_api_macros::declare_ty!(wasm_globaltype_t);

#[derive(Clone)]
pub(crate) struct CGlobalType {
    pub(crate) ty: GlobalType,
    content_cache: OnceCell<wasm_valtype_t>,
}

impl wasm_globaltype_t {
    pub(crate) fn new(ty: GlobalType) -> wasm_globaltype_t {
        wasm_globaltype_t {
            ext: wasm_externtype_t::from_extern_type(ty.into()),
        }
    }

    pub(crate) fn try_from(e: &wasm_externtype_t) -> Option<&wasm_globaltype_t> {
        match &e.which {
            CExternType::Global(_) => Some(unsafe { &*(e as *const _ as *const _) }),
            _ => None,
        }
    }

    pub(crate) fn ty(&self) -> &CGlobalType {
        match &self.ext.which {
            CExternType::Global(f) => &f,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl CGlobalType {
    pub(crate) fn new(ty: GlobalType) -> CGlobalType {
        CGlobalType {
            ty,
            content_cache: OnceCell::new(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_globaltype_new(
    ty: Box<wasm_valtype_t>,
    mutability: wasm_mutability_t,
) -> Option<Box<wasm_globaltype_t>> {
    use wasmtime::Mutability::*;
    let mutability = match mutability {
        WASM_CONST => Const,
        WASM_VAR => Var,
        _ => return None,
    };
    let ty = GlobalType::new(ty.ty.clone(), mutability);
    Some(Box::new(wasm_globaltype_t::new(ty)))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_globaltype_content(gt: &wasm_globaltype_t) -> &wasm_valtype_t {
    let gt = gt.ty();
    gt.content_cache.get_or_init(|| wasm_valtype_t {
        ty: gt.ty.content().clone(),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_globaltype_mutability(gt: &wasm_globaltype_t) -> wasm_mutability_t {
    use wasmtime::Mutability::*;
    let gt = gt.ty();
    match gt.ty.mutability() {
        Const => WASM_CONST,
        Var => WASM_VAR,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_globaltype_as_externtype(ty: &wasm_globaltype_t) -> &wasm_externtype_t {
    &ty.ext
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_globaltype_as_externtype_const(
    ty: &wasm_globaltype_t,
) -> &wasm_externtype_t {
    &ty.ext
}
