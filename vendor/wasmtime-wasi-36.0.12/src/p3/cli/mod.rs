mod host;

use crate::cli::WasiCliCtx;
use crate::p3::bindings::cli;
use wasmtime::component::{HasData, Linker, ResourceTable};

pub struct WasiCliCtxView<'a> {
    pub ctx: &'a mut WasiCliCtx,
    pub table: &'a mut ResourceTable,
}

pub trait WasiCliView: Send {
    fn cli(&mut self) -> WasiCliCtxView<'_>;
}

/// Add all WASI interfaces from this module into the `linker` provided.
///
/// This function will add all interfaces implemented by this module to the
/// [`Linker`], which corresponds to the `wasi:cli/imports` world supported by
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
/// use wasmtime_wasi::cli::WasiCliCtx;
/// use wasmtime_wasi::p3::cli::{WasiCliView, WasiCliCtxView};
///
/// fn main() -> Result<()> {
///     let mut config = Config::new();
///     config.async_support(true);
///     config.wasm_component_model_async(true);
///     let engine = Engine::new(&config)?;
///
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p3::cli::add_to_linker(&mut linker)?;
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
///     cli: WasiCliCtx,
///     table: ResourceTable,
/// }
///
/// impl WasiCliView for MyState {
///     fn cli(&mut self) -> WasiCliCtxView<'_> {
///         WasiCliCtxView {
///             ctx: &mut self.cli,
///             table: &mut self.table,
///         }
///     }
/// }
/// ```
pub fn add_to_linker<T>(linker: &mut Linker<T>) -> wasmtime::Result<()>
where
    T: WasiCliView + 'static,
{
    let exit_options = cli::exit::LinkOptions::default();
    add_to_linker_with_options(linker, &exit_options)
}

/// Similar to [`add_to_linker`], but with the ability to enable unstable features.
pub fn add_to_linker_with_options<T>(
    linker: &mut Linker<T>,
    exit_options: &cli::exit::LinkOptions,
) -> anyhow::Result<()>
where
    T: WasiCliView + 'static,
{
    cli::exit::add_to_linker::<_, WasiCli>(linker, exit_options, T::cli)?;
    cli::environment::add_to_linker::<_, WasiCli>(linker, T::cli)?;
    cli::stdin::add_to_linker::<_, WasiCli>(linker, T::cli)?;
    cli::stdout::add_to_linker::<_, WasiCli>(linker, T::cli)?;
    cli::stderr::add_to_linker::<_, WasiCli>(linker, T::cli)?;
    cli::terminal_input::add_to_linker::<_, WasiCli>(linker, T::cli)?;
    cli::terminal_output::add_to_linker::<_, WasiCli>(linker, T::cli)?;
    cli::terminal_stdin::add_to_linker::<_, WasiCli>(linker, T::cli)?;
    cli::terminal_stdout::add_to_linker::<_, WasiCli>(linker, T::cli)?;
    cli::terminal_stderr::add_to_linker::<_, WasiCli>(linker, T::cli)?;
    Ok(())
}

struct WasiCli;

impl HasData for WasiCli {
    type Data<'a> = WasiCliCtxView<'a>;
}

pub struct TerminalInput;
pub struct TerminalOutput;
