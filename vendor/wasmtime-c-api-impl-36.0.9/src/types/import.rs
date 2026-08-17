use crate::{CExternType, wasm_externtype_t, wasm_name_t};
use std::cell::OnceCell;

#[repr(C)]
#[derive(Clone)]
pub struct wasm_importtype_t {
    pub(crate) module: String,
    pub(crate) name: String,
    pub(crate) ty: CExternType,
    module_cache: OnceCell<wasm_name_t>,
    name_cache: OnceCell<wasm_name_t>,
    type_cache: OnceCell<wasm_externtype_t>,
}

wasmtime_c_api_macros::declare_ty!(wasm_importtype_t);

impl wasm_importtype_t {
    pub(crate) fn new(module: String, name: String, ty: CExternType) -> wasm_importtype_t {
        wasm_importtype_t {
            module,
            name,
            ty,
            module_cache: OnceCell::new(),
            name_cache: OnceCell::new(),
            type_cache: OnceCell::new(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_importtype_new(
    module: &mut wasm_name_t,
    name: &mut wasm_name_t,
    ty: Box<wasm_externtype_t>,
) -> Option<Box<wasm_importtype_t>> {
    let module = module.take();
    let name = name.take();
    let module = String::from_utf8(module).ok()?;
    let name = String::from_utf8(name).ok()?;
    Some(Box::new(wasm_importtype_t::new(
        module,
        name,
        ty.which.clone(),
    )))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_importtype_module(it: &wasm_importtype_t) -> &wasm_name_t {
    it.module_cache
        .get_or_init(|| wasm_name_t::from_name(it.module.clone()))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_importtype_name(it: &wasm_importtype_t) -> &wasm_name_t {
    it.name_cache
        .get_or_init(|| wasm_name_t::from_name(it.name.to_string()))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_importtype_type(it: &wasm_importtype_t) -> &wasm_externtype_t {
    it.type_cache
        .get_or_init(|| wasm_externtype_t::from_cextern_type(it.ty.clone()))
}
