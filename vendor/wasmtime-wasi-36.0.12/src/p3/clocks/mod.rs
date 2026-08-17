mod host;

use crate::clocks::{WasiClocksCtx, WasiClocksView};
use crate::p3::bindings::clocks;
use wasmtime::component::{HasData, Linker};

/// Add all WASI interfaces from this module into the `linker` provided.
///
/// This function will add all interfaces implemented by this module to the
/// [`Linker`], which corresponds to the `wasi:clocks/imports` world supported by
/// this module.
///
/// This is low-level API for advanced use cases,
/// [`wasmtime_wasi::p3::add_to_linker`](crate::p3::add_to_linker) can be used instead
/// to add *all* wasip3 interfaces (including the ones from this module) to the `linker`.
///
/// # Example
///
/// ```
/// use wasmtime::{Engine, Result, Store, Config};
/// use wasmtime::component::Linker;
/// use wasmtime_wasi::clocks::{WasiClocksView, WasiClocksCtx};
///
/// fn main() -> Result<()> {
///     let mut config = Config::new();
///     config.async_support(true);
///     config.wasm_component_model_async(true);
///     let engine = Engine::new(&config)?;
///
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p3::clocks::add_to_linker(&mut linker)?;
///     // ... add any further functionality to `linker` if desired ...
///
///     let mut store = Store::new(
///         &engine,
///         MyState {
///             clocks: WasiClocksCtx::default(),
///         },
///     );
///
///     // ... use `linker` to instantiate within `store` ...
///
///     Ok(())
/// }
///
/// struct MyState {
///     clocks: WasiClocksCtx,
/// }
///
/// impl WasiClocksView for MyState {
///     fn clocks(&mut self) -> &mut WasiClocksCtx { &mut self.clocks }
/// }
/// ```
pub fn add_to_linker<T>(linker: &mut Linker<T>) -> wasmtime::Result<()>
where
    T: WasiClocksView + 'static,
{
    clocks::monotonic_clock::add_to_linker::<_, WasiClocks>(linker, T::clocks)?;
    clocks::wall_clock::add_to_linker::<_, WasiClocks>(linker, T::clocks)?;
    Ok(())
}

struct WasiClocks;

impl HasData for WasiClocks {
    type Data<'a> = &'a mut WasiClocksCtx;
}
