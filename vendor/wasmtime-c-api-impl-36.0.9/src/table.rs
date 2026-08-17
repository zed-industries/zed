use crate::{
    WasmtimeStoreContext, WasmtimeStoreContextMut, handle_result, wasm_extern_t, wasm_ref_t,
    wasm_store_t, wasm_tabletype_t, wasmtime_error_t, wasmtime_val_t,
};
use anyhow::anyhow;
use std::mem::MaybeUninit;
use wasmtime::{Extern, Ref, RootScope, Table, TableType};

#[derive(Clone)]
#[repr(transparent)]
pub struct wasm_table_t {
    ext: wasm_extern_t,
}

wasmtime_c_api_macros::declare_ref!(wasm_table_t);

pub type wasm_table_size_t = u32;

impl wasm_table_t {
    pub(crate) fn try_from(e: &wasm_extern_t) -> Option<&wasm_table_t> {
        match &e.which {
            Extern::Table(_) => Some(unsafe { &*(e as *const _ as *const _) }),
            _ => None,
        }
    }

    fn table(&self) -> Table {
        match self.ext.which {
            Extern::Table(t) => t,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

fn option_wasm_ref_t_to_ref(r: Option<&wasm_ref_t>, table_ty: &TableType) -> Ref {
    r.map(|r| r.r.clone())
        .unwrap_or_else(|| Ref::null(table_ty.element().heap_type()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_table_new(
    store: &mut wasm_store_t,
    tt: &wasm_tabletype_t,
    init: Option<&wasm_ref_t>,
) -> Option<Box<wasm_table_t>> {
    let tt = tt.ty().ty.clone();
    let init = option_wasm_ref_t_to_ref(init, &tt);
    let table = Table::new(store.store.context_mut(), tt, init).ok()?;
    Some(Box::new(wasm_table_t {
        ext: wasm_extern_t {
            store: store.store.clone(),
            which: table.into(),
        },
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_table_type(t: &wasm_table_t) -> Box<wasm_tabletype_t> {
    let table = t.table();
    let store = t.ext.store.context();
    Box::new(wasm_tabletype_t::new(table.ty(&store)))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_table_get(
    t: &mut wasm_table_t,
    index: wasm_table_size_t,
) -> Option<Box<wasm_ref_t>> {
    let table = t.table();
    let r = table.get(t.ext.store.context_mut(), u64::from(index))?;
    wasm_ref_t::new(r)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_table_set(
    t: &mut wasm_table_t,
    index: wasm_table_size_t,
    r: Option<&wasm_ref_t>,
) -> bool {
    let table = t.table();
    let val = option_wasm_ref_t_to_ref(r, &table.ty(t.ext.store.context()));
    table
        .set(t.ext.store.context_mut(), u64::from(index), val)
        .is_ok()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_table_size(t: &wasm_table_t) -> wasm_table_size_t {
    let table = t.table();
    let store = t.ext.store.context();
    u32::try_from(table.size(&store)).unwrap()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_table_grow(
    t: &mut wasm_table_t,
    delta: wasm_table_size_t,
    init: Option<&wasm_ref_t>,
) -> bool {
    let table = t.table();
    let init = option_wasm_ref_t_to_ref(init, &table.ty(t.ext.store.context()));
    table
        .grow(t.ext.store.context_mut(), u64::from(delta), init)
        .is_ok()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_table_as_extern(t: &mut wasm_table_t) -> &mut wasm_extern_t {
    &mut t.ext
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_table_as_extern_const(t: &wasm_table_t) -> &wasm_extern_t {
    &t.ext
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_table_new(
    mut store: WasmtimeStoreContextMut<'_>,
    tt: &wasm_tabletype_t,
    init: &wasmtime_val_t,
    out: &mut Table,
) -> Option<Box<wasmtime_error_t>> {
    let mut scope = RootScope::new(&mut store);
    handle_result(
        init.to_val(&mut scope)
            .ref_()
            .ok_or_else(|| anyhow!("wasmtime_table_new init value is not a reference"))
            .and_then(|init| Table::new(scope, tt.ty().ty.clone(), init)),
        |table| *out = table,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_table_type(
    store: WasmtimeStoreContext<'_>,
    table: &Table,
) -> Box<wasm_tabletype_t> {
    Box::new(wasm_tabletype_t::new(table.ty(store)))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_table_get(
    store: WasmtimeStoreContextMut<'_>,
    table: &Table,
    index: u64,
    ret: &mut MaybeUninit<wasmtime_val_t>,
) -> bool {
    let mut scope = RootScope::new(store);
    match table.get(&mut scope, index) {
        Some(r) => {
            crate::initialize(ret, wasmtime_val_t::from_val(&mut scope, r.into()));
            true
        }
        None => false,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_table_set(
    mut store: WasmtimeStoreContextMut<'_>,
    table: &Table,
    index: u64,
    val: &wasmtime_val_t,
) -> Option<Box<wasmtime_error_t>> {
    let mut scope = RootScope::new(&mut store);
    handle_result(
        val.to_val(&mut scope)
            .ref_()
            .ok_or_else(|| anyhow!("wasmtime_table_set value is not a reference"))
            .and_then(|val| table.set(scope, index, val)),
        |()| {},
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_table_size(store: WasmtimeStoreContext<'_>, table: &Table) -> u64 {
    table.size(store)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_table_grow(
    mut store: WasmtimeStoreContextMut<'_>,
    table: &Table,
    delta: u64,
    val: &wasmtime_val_t,
    prev_size: &mut u64,
) -> Option<Box<wasmtime_error_t>> {
    let mut scope = RootScope::new(&mut store);
    handle_result(
        val.to_val(&mut scope)
            .ref_()
            .ok_or_else(|| anyhow!("wasmtime_table_grow value is not a reference"))
            .and_then(|val| table.grow(scope, delta, val)),
        |prev| *prev_size = prev,
    )
}
