mod host;

use crate::p3::bindings::random;
use crate::random::{WasiRandom, WasiRandomView};
use wasmtime::component::Linker;

/// Add all WASI interfaces from this module into the `linker` provided.
///
/// This function will add all interfaces implemented by this module to the
/// [`Linker`], which corresponds to the `wasi:random/imports` world supported by
/// this crate.
///
/// This is low-level API for advanced use cases,
/// [`wasmtime_wasi::p3::add_to_linker`](crate::p3::add_to_linker) can be used instead
/// to add *all* wasip3 interfaces (including the ones from this module) to the `linker`.
///
///
/// # Example
///
/// ```
/// use wasmtime::{Engine, Result, Store, Config};
/// use wasmtime::component::Linker;
/// use wasmtime_wasi::random::{WasiRandomView, WasiRandomCtx};
///
/// fn main() -> Result<()> {
///     let mut config = Config::new();
///     config.async_support(true);
///     let engine = Engine::new(&config)?;
///
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p3::random::add_to_linker(&mut linker)?;
///     // ... add any further functionality to `linker` if desired ...
///
///     let mut store = Store::new(
///         &engine,
///         MyState {
///             random: WasiRandomCtx::default(),
///         },
///     );
///
///     // ... use `linker` to instantiate within `store` ...
///
///     Ok(())
/// }
///
/// struct MyState {
///     random: WasiRandomCtx,
/// }
///
/// impl WasiRandomView for MyState {
///     fn random(&mut self) -> &mut WasiRandomCtx { &mut self.random }
/// }
/// ```
pub fn add_to_linker<T>(linker: &mut Linker<T>) -> wasmtime::Result<()>
where
    T: WasiRandomView + 'static,
{
    random::random::add_to_linker::<_, WasiRandom>(linker, T::random)?;
    random::insecure::add_to_linker::<_, WasiRandom>(linker, T::random)?;
    random::insecure_seed::add_to_linker::<_, WasiRandom>(linker, T::random)?;
    Ok(())
}
