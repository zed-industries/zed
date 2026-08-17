use crate::TrappableError;
use crate::p3::bindings::sockets;
use crate::sockets::{WasiSocketsCtxView, WasiSocketsView};
use wasmtime::component::{HasData, Linker};

mod conv;
mod host;
pub mod tcp;
pub mod udp;

pub type SocketResult<T> = Result<T, SocketError>;
pub type SocketError = TrappableError<sockets::types::ErrorCode>;

/// Add all WASI interfaces from this module into the `linker` provided.
///
/// This function will add all interfaces implemented by this module to the
/// [`Linker`], which corresponds to the `wasi:sockets/imports` world supported by
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
/// use wasmtime::component::{Linker, ResourceTable};
/// use wasmtime_wasi::sockets::{WasiSocketsCtx, WasiSocketsCtxView, WasiSocketsView};
///
/// fn main() -> Result<()> {
///     let mut config = Config::new();
///     config.async_support(true);
///     config.wasm_component_model_async(true);
///     let engine = Engine::new(&config)?;
///
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p3::sockets::add_to_linker(&mut linker)?;
///     // ... add any further functionality to `linker` if desired ...
///
///     let mut store = Store::new(
///         &engine,
///         MyState::default(),
///     );
///
///     // ... use `linker` to instantiate within `store` ...
///
///     Ok(())
/// }
///
/// #[derive(Default)]
/// struct MyState {
///     sockets: WasiSocketsCtx,
///     table: ResourceTable,
/// }
///
/// impl WasiSocketsView for MyState {
///     fn sockets(&mut self) -> WasiSocketsCtxView<'_> {
///         WasiSocketsCtxView {
///             ctx: &mut self.sockets,
///             table: &mut self.table,
///         }
///     }
/// }
/// ```
pub fn add_to_linker<T>(linker: &mut Linker<T>) -> wasmtime::Result<()>
where
    T: WasiSocketsView + 'static,
{
    sockets::ip_name_lookup::add_to_linker::<_, WasiSockets>(linker, T::sockets)?;
    sockets::types::add_to_linker::<_, WasiSockets>(linker, T::sockets)?;
    Ok(())
}

struct WasiSockets;

impl HasData for WasiSockets {
    type Data<'a> = WasiSocketsCtxView<'a>;
}
