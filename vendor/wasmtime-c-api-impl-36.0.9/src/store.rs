use crate::{ForeignData, wasm_engine_t, wasmtime_error_t, wasmtime_val_t};
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::sync::Arc;
use wasmtime::{
    AsContext, AsContextMut, Caller, Store, StoreContext, StoreContextMut, StoreLimits,
    StoreLimitsBuilder, UpdateDeadline, Val,
};

// Store-related type aliases for `wasm.h` APIs. Not for use with `wasmtime.h`
// APIs!
pub type WasmStoreData = ();
pub type WasmStore = Store<WasmStoreData>;
pub type WasmStoreContext<'a> = StoreContext<'a, WasmStoreData>;
pub type WasmStoreContextMut<'a> = StoreContextMut<'a, WasmStoreData>;

/// This representation of a `Store` is used to implement the `wasm.h` API (and
/// *not* the `wasmtime.h` API!)
///
/// This is stored alongside `Func` and such for `wasm.h` so each object is
/// independently owned. The usage of `Arc` here is mostly to just get it to be
/// safe to drop across multiple threads, but otherwise acquiring the `context`
/// values from this struct is considered unsafe due to it being unknown how the
/// aliasing is working on the C side of things.
///
/// The aliasing requirements are documented in the C API `wasm.h` itself (at
/// least Wasmtime's implementation).
#[derive(Clone)]
pub struct WasmStoreRef {
    store: Arc<UnsafeCell<WasmStore>>,
}

impl WasmStoreRef {
    pub unsafe fn context(&self) -> WasmStoreContext<'_> {
        (*self.store.get()).as_context()
    }

    pub unsafe fn context_mut(&mut self) -> WasmStoreContextMut<'_> {
        (*self.store.get()).as_context_mut()
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct wasm_store_t {
    pub(crate) store: WasmStoreRef,
}

wasmtime_c_api_macros::declare_own!(wasm_store_t);

#[unsafe(no_mangle)]
pub extern "C" fn wasm_store_new(engine: &wasm_engine_t) -> Box<wasm_store_t> {
    let engine = &engine.engine;
    let store = Store::new(engine, ());
    Box::new(wasm_store_t {
        store: WasmStoreRef {
            store: Arc::new(UnsafeCell::new(store)),
        },
    })
}

// Store-related type aliases for `wasmtime.h` APIs. Not for use with `wasm.h`
// APIs!
pub type WasmtimeStore = Store<WasmtimeStoreData>;
pub type WasmtimeStoreContext<'a> = StoreContext<'a, WasmtimeStoreData>;
pub type WasmtimeStoreContextMut<'a> = StoreContextMut<'a, WasmtimeStoreData>;
pub type WasmtimeCaller<'a> = Caller<'a, WasmtimeStoreData>;

/// Representation of a `Store` for `wasmtime.h` This notably tries to move more
/// burden of aliasing on the caller rather than internally, allowing for a more
/// raw representation of contexts and such that requires less `unsafe` in the
/// implementation.
///
/// Note that this notably carries `WasmtimeStoreData` as a payload which allows
/// storing foreign data and configuring WASI as well.
#[repr(C)]
pub struct wasmtime_store_t {
    pub(crate) store: WasmtimeStore,
}

wasmtime_c_api_macros::declare_own!(wasmtime_store_t);

pub struct WasmtimeStoreData {
    foreign: crate::ForeignData,
    #[cfg(feature = "wasi")]
    pub(crate) wasi: Option<wasmtime_wasi::preview1::WasiP1Ctx>,

    /// Temporary storage for usage during a wasm->host call to store values
    /// in a slice we pass to the C API.
    pub hostcall_val_storage: Vec<wasmtime_val_t>,

    /// Temporary storage for usage during host->wasm calls, same as above but
    /// for a different direction.
    pub wasm_val_storage: Vec<Val>,

    /// Limits for the store.
    pub store_limits: StoreLimits,

    #[cfg(feature = "component-model")]
    pub(crate) resource_table: wasmtime::component::ResourceTable,

    #[cfg(all(feature = "component-model", feature = "wasi"))]
    pub(crate) wasip2: Option<wasmtime_wasi::WasiCtx>,
}

#[cfg(all(feature = "component-model", feature = "wasi"))]
impl wasmtime_wasi::WasiView for WasmtimeStoreData {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        wasmtime_wasi::WasiCtxView {
            ctx: self.wasip2.as_mut().unwrap(),
            table: &mut self.resource_table,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_store_new(
    engine: &wasm_engine_t,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut c_void)>,
) -> Box<wasmtime_store_t> {
    Box::new(wasmtime_store_t {
        store: Store::new(
            &engine.engine,
            WasmtimeStoreData {
                foreign: ForeignData { data, finalizer },
                #[cfg(feature = "wasi")]
                wasi: None,
                hostcall_val_storage: Vec::new(),
                wasm_val_storage: Vec::new(),
                store_limits: StoreLimits::default(),
                #[cfg(feature = "component-model")]
                resource_table: wasmtime::component::ResourceTable::default(),
                #[cfg(all(feature = "component-model", feature = "wasi"))]
                wasip2: None,
            },
        ),
    })
}

pub type wasmtime_update_deadline_kind_t = u8;
pub const WASMTIME_UPDATE_DEADLINE_CONTINUE: wasmtime_update_deadline_kind_t = 0;
pub const WASMTIME_UPDATE_DEADLINE_YIELD: wasmtime_update_deadline_kind_t = 1;

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_store_epoch_deadline_callback(
    store: &mut wasmtime_store_t,
    func: extern "C" fn(
        WasmtimeStoreContextMut<'_>,
        *mut c_void,
        *mut u64,
        *mut wasmtime_update_deadline_kind_t,
    ) -> Option<Box<wasmtime_error_t>>,
    data: *mut c_void,
    finalizer: Option<extern "C" fn(*mut c_void)>,
) {
    let foreign = crate::ForeignData { data, finalizer };
    store.store.epoch_deadline_callback(move |mut store_ctx| {
        let _ = &foreign; // Move foreign into this closure
        let mut delta: u64 = 0;
        let mut kind = WASMTIME_UPDATE_DEADLINE_CONTINUE;
        let result = (func)(
            store_ctx.as_context_mut(),
            foreign.data,
            &mut delta as *mut u64,
            &mut kind as *mut wasmtime_update_deadline_kind_t,
        );
        match result {
            Some(err) => Err((*err).into()),
            None if kind == WASMTIME_UPDATE_DEADLINE_CONTINUE => {
                Ok(UpdateDeadline::Continue(delta))
            }
            #[cfg(feature = "async")]
            None if kind == WASMTIME_UPDATE_DEADLINE_YIELD => Ok(UpdateDeadline::Yield(delta)),
            _ => panic!("unknown wasmtime_update_deadline_kind_t: {kind}"),
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_store_context(
    store: &mut wasmtime_store_t,
) -> WasmtimeStoreContextMut<'_> {
    store.store.as_context_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_store_limiter(
    store: &mut wasmtime_store_t,
    memory_size: i64,
    table_elements: i64,
    instances: i64,
    tables: i64,
    memories: i64,
) {
    let mut limiter = StoreLimitsBuilder::new();
    if memory_size >= 0 {
        limiter = limiter.memory_size(memory_size as usize);
    }
    if table_elements >= 0 {
        limiter = limiter.table_elements(table_elements as usize);
    }
    if instances >= 0 {
        limiter = limiter.instances(instances as usize);
    }
    if tables >= 0 {
        limiter = limiter.tables(tables as usize);
    }
    if memories >= 0 {
        limiter = limiter.memories(memories as usize);
    }
    store.store.data_mut().store_limits = limiter.build();
    store.store.limiter(|data| &mut data.store_limits);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_context_get_data(store: WasmtimeStoreContext<'_>) -> *mut c_void {
    store.data().foreign.data
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_context_set_data(
    mut store: WasmtimeStoreContextMut<'_>,
    data: *mut c_void,
) {
    store.data_mut().foreign.data = data;
}

#[cfg(feature = "wasi")]
#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_context_set_wasi(
    mut context: WasmtimeStoreContextMut<'_>,
    wasi: Box<crate::wasi_config_t>,
) -> Option<Box<wasmtime_error_t>> {
    crate::handle_result(wasi.into_wasi_ctx(), |wasi| {
        context.data_mut().wasi = Some(wasi);
    })
}

#[cfg(all(feature = "component-model", feature = "wasi"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_context_set_wasip2(
    mut context: WasmtimeStoreContextMut<'_>,
    mut config: Box<crate::wasmtime_wasip2_config_t>,
) {
    context.data_mut().wasip2 = Some(config.builder.build());
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_context_gc(mut context: WasmtimeStoreContextMut<'_>) {
    context.gc(None);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_context_set_fuel(
    mut store: WasmtimeStoreContextMut<'_>,
    fuel: u64,
) -> Option<Box<wasmtime_error_t>> {
    crate::handle_result(store.set_fuel(fuel), |()| {})
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_context_get_fuel(
    store: WasmtimeStoreContext<'_>,
    fuel: &mut u64,
) -> Option<Box<wasmtime_error_t>> {
    crate::handle_result(store.get_fuel(), |amt| {
        *fuel = amt;
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_context_set_epoch_deadline(
    mut store: WasmtimeStoreContextMut<'_>,
    ticks_beyond_current: u64,
) {
    store.set_epoch_deadline(ticks_beyond_current);
}
