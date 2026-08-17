#[cfg(feature = "coredump")]
use super::coredump::WasmCoreDump;
use crate::prelude::*;
use crate::store::StoreOpaque;
use crate::{AsContext, Module};
use core::fmt;
use wasmtime_environ::{FilePos, demangle_function_name, demangle_function_name_or_index};

/// Representation of a WebAssembly trap and what caused it to occur.
///
/// WebAssembly traps happen explicitly for instructions such as `unreachable`
/// but can also happen as side effects of other instructions such as `i32.load`
/// loading an out-of-bounds address. Traps halt the execution of WebAssembly
/// and cause an error to be returned to the host. This enumeration is a list of
/// all possible traps that can happen in wasm, in addition to some
/// Wasmtime-specific trap codes listed here as well.
///
/// # Errors in Wasmtime
///
/// Error-handling in Wasmtime is primarily done through the
/// [`anyhow`][mod@anyhow] crate where most results are a
/// [`Result<T>`](anyhow::Result) which is an alias for [`Result<T,
/// anyhow::Error>`](std::result::Result). Errors in Wasmtime are represented
/// with [`anyhow::Error`] which acts as a container for any type of error in
/// addition to optional context for this error. The "base" error or
/// [`anyhow::Error::root_cause`] is a [`Trap`] whenever WebAssembly hits a
/// trap, or otherwise it's whatever the host created the error with when
/// returning an error for a host call.
///
/// Any error which happens while WebAssembly is executing will also, by
/// default, capture a backtrace of the wasm frames while executing. This
/// backtrace is represented with a [`WasmBacktrace`] instance and is attached
/// to the [`anyhow::Error`] return value as a
/// [`context`](anyhow::Error::context). Inspecting a [`WasmBacktrace`] can be
/// done with the [`downcast_ref`](anyhow::Error::downcast_ref) function. For
/// information on this see the [`WasmBacktrace`] documentation.
///
/// # Examples
///
/// ```
/// # use wasmtime::*;
/// # fn main() -> Result<()> {
/// let engine = Engine::default();
/// let module = Module::new(
///     &engine,
///     r#"
///         (module
///             (func (export "trap")
///                 unreachable)
///             (func $overflow (export "overflow")
///                 call $overflow)
///         )
///     "#,
/// )?;
/// let mut store = Store::new(&engine, ());
/// let instance = Instance::new(&mut store, &module, &[])?;
///
/// let trap = instance.get_typed_func::<(), ()>(&mut store, "trap")?;
/// let error = trap.call(&mut store, ()).unwrap_err();
/// assert_eq!(*error.downcast_ref::<Trap>().unwrap(), Trap::UnreachableCodeReached);
/// assert!(error.root_cause().is::<Trap>());
///
/// let overflow = instance.get_typed_func::<(), ()>(&mut store, "overflow")?;
/// let error = overflow.call(&mut store, ()).unwrap_err();
/// assert_eq!(*error.downcast_ref::<Trap>().unwrap(), Trap::StackOverflow);
/// # Ok(())
/// # }
/// ```
pub use wasmtime_environ::Trap;

#[cold] // traps are exceptional, this helps move handling off the main path
pub(crate) fn from_runtime_box(
    store: &mut StoreOpaque,
    runtime_trap: Box<crate::runtime::vm::Trap>,
) -> Error {
    let crate::runtime::vm::Trap {
        reason,
        backtrace,
        coredumpstack,
    } = *runtime_trap;
    let (mut error, pc) = match reason {
        // For user-defined errors they're already an `anyhow::Error` so no
        // conversion is really necessary here, but a `backtrace` may have
        // been captured so it's attempted to get inserted here.
        //
        // If the error is actually a `Trap` then the backtrace is inserted
        // directly into the `Trap` since there's storage there for it.
        // Otherwise though this represents a host-defined error which isn't
        // using a `Trap` but instead some other condition that was fatal to
        // wasm itself. In that situation the backtrace is inserted as
        // contextual information on error using `error.context(...)` to
        // provide useful information to debug with for the embedder/caller,
        // otherwise the information about what the wasm was doing when the
        // error was generated would be lost.
        crate::runtime::vm::TrapReason::User(error) => (error, None),
        crate::runtime::vm::TrapReason::Jit {
            pc,
            faulting_addr,
            trap,
        } => {
            let mut err: Error = trap.into();

            // If a fault address was present, for example with segfaults,
            // then simultaneously assert that it's within a known linear memory
            // and additionally translate it to a wasm-local address to be added
            // as context to the error.
            if let Some(fault) = faulting_addr.and_then(|addr| store.wasm_fault(pc, addr)) {
                err = err.context(fault);
            }
            (err, Some(pc))
        }
        crate::runtime::vm::TrapReason::Wasm(trap_code) => (trap_code.into(), None),
    };

    if let Some(bt) = backtrace {
        let bt = WasmBacktrace::from_captured(store, bt, pc);
        if !bt.wasm_trace.is_empty() {
            error = error.context(bt);
        }
    }

    let _ = &coredumpstack;
    #[cfg(feature = "coredump")]
    if let Some(coredump) = coredumpstack {
        let bt = WasmBacktrace::from_captured(store, coredump.bt, pc);
        let cd = WasmCoreDump::new(store, bt);
        error = error.context(cd);
    }

    error
}

/// Representation of a backtrace of function frames in a WebAssembly module for
/// where an error happened.
///
/// This structure is attached to the [`anyhow::Error`] returned from many
/// Wasmtime functions that execute WebAssembly such as [`Instance::new`] or
/// [`Func::call`]. This can be acquired with the [`anyhow::Error::downcast`]
/// family of methods to programmatically inspect the backtrace. Otherwise since
/// it's part of the error returned this will get printed along with the rest of
/// the error when the error is logged.
///
/// Capturing of wasm backtraces can be configured through the
/// [`Config::wasm_backtrace`](crate::Config::wasm_backtrace) method.
///
/// For more information about errors in wasmtime see the documentation of the
/// [`Trap`] type.
///
/// [`Func::call`]: crate::Func::call
/// [`Instance::new`]: crate::Instance::new
///
/// # Examples
///
/// ```
/// # use wasmtime::*;
/// # fn main() -> Result<()> {
/// let engine = Engine::default();
/// let module = Module::new(
///     &engine,
///     r#"
///         (module
///             (func $start (export "run")
///                 call $trap)
///             (func $trap
///                 unreachable)
///         )
///     "#,
/// )?;
/// let mut store = Store::new(&engine, ());
/// let instance = Instance::new(&mut store, &module, &[])?;
/// let func = instance.get_typed_func::<(), ()>(&mut store, "run")?;
/// let error = func.call(&mut store, ()).unwrap_err();
/// let bt = error.downcast_ref::<WasmBacktrace>().unwrap();
/// let frames = bt.frames();
/// assert_eq!(frames.len(), 2);
/// assert_eq!(frames[0].func_name(), Some("trap"));
/// assert_eq!(frames[1].func_name(), Some("start"));
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct WasmBacktrace {
    wasm_trace: Vec<FrameInfo>,
    hint_wasm_backtrace_details_env: bool,
    // This is currently only present for the `Debug` implementation for extra
    // context.
    _runtime_trace: crate::runtime::vm::Backtrace,
}

impl WasmBacktrace {
    /// Captures a trace of the WebAssembly frames on the stack for the
    /// provided store.
    ///
    /// This will return a [`WasmBacktrace`] which holds captured
    /// [`FrameInfo`]s for each frame of WebAssembly on the call stack of the
    /// current thread. If no WebAssembly is on the stack then the returned
    /// backtrace will have no frames in it.
    ///
    /// Note that this function will respect the [`Config::wasm_backtrace`]
    /// configuration option and will return an empty backtrace if that is
    /// disabled. To always capture a backtrace use the
    /// [`WasmBacktrace::force_capture`] method.
    ///
    /// Also note that this function will only capture frames from the
    /// specified `store` on the stack, ignoring frames from other stores if
    /// present.
    ///
    /// [`Config::wasm_backtrace`]: crate::Config::wasm_backtrace
    ///
    /// # Example
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> Result<()> {
    /// let engine = Engine::default();
    /// let module = Module::new(
    ///     &engine,
    ///     r#"
    ///         (module
    ///             (import "" "" (func $host))
    ///             (func $foo (export "f") call $bar)
    ///             (func $bar call $host)
    ///         )
    ///     "#,
    /// )?;
    ///
    /// let mut store = Store::new(&engine, ());
    /// let func = Func::wrap(&mut store, |cx: Caller<'_, ()>| {
    ///     let trace = WasmBacktrace::capture(&cx);
    ///     println!("{trace:?}");
    /// });
    /// let instance = Instance::new(&mut store, &module, &[func.into()])?;
    /// let func = instance.get_typed_func::<(), ()>(&mut store, "f")?;
    /// func.call(&mut store, ())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn capture(store: impl AsContext) -> WasmBacktrace {
        let store = store.as_context();
        if store.engine().config().wasm_backtrace {
            Self::force_capture(store)
        } else {
            WasmBacktrace {
                wasm_trace: Vec::new(),
                hint_wasm_backtrace_details_env: false,
                _runtime_trace: crate::runtime::vm::Backtrace::empty(),
            }
        }
    }

    /// Unconditionally captures a trace of the WebAssembly frames on the stack
    /// for the provided store.
    ///
    /// Same as [`WasmBacktrace::capture`] except that it disregards the
    /// [`Config::wasm_backtrace`](crate::Config::wasm_backtrace) setting and
    /// always captures a backtrace.
    pub fn force_capture(store: impl AsContext) -> WasmBacktrace {
        let store = store.as_context();
        Self::from_captured(store.0, crate::runtime::vm::Backtrace::new(store.0), None)
    }

    fn from_captured(
        store: &StoreOpaque,
        runtime_trace: crate::runtime::vm::Backtrace,
        trap_pc: Option<usize>,
    ) -> Self {
        let mut wasm_trace = Vec::<FrameInfo>::with_capacity(runtime_trace.frames().len());
        let mut hint_wasm_backtrace_details_env = false;
        let wasm_backtrace_details_env_used =
            store.engine().config().wasm_backtrace_details_env_used;

        for frame in runtime_trace.frames() {
            debug_assert!(frame.pc() != 0);

            // Note that we need to be careful about the pc we pass in
            // here to lookup frame information. This program counter is
            // used to translate back to an original source location in
            // the origin wasm module. If this pc is the exact pc that
            // the trap happened at, then we look up that pc precisely.
            // Otherwise backtrace information typically points at the
            // pc *after* the call instruction (because otherwise it's
            // likely a call instruction on the stack). In that case we
            // want to lookup information for the previous instruction
            // (the call instruction) so we subtract one as the lookup.
            let pc_to_lookup = if Some(frame.pc()) == trap_pc {
                frame.pc()
            } else {
                frame.pc() - 1
            };

            // NB: The PC we are looking up _must_ be a Wasm PC since
            // `crate::runtime::vm::Backtrace` only contains Wasm frames.
            //
            // However, consider the case where we have multiple, nested calls
            // across stores (with host code in between, by necessity, since
            // only things in the same store can be linked directly together):
            //
            //     | ...             |
            //     | Host            |  |
            //     +-----------------+  | stack
            //     | Wasm in store A |  | grows
            //     +-----------------+  | down
            //     | Host            |  |
            //     +-----------------+  |
            //     | Wasm in store B |  V
            //     +-----------------+
            //
            // In this scenario, the `crate::runtime::vm::Backtrace` will
            // contain two frames: Wasm in store B followed by Wasm in store
            // A. But `store.modules()` will only have the module information
            // for modules instantiated within this store. Therefore, we use `if
            // let Some(..)` instead of the `unwrap` you might otherwise expect
            // and we ignore frames from modules that were not registered in
            // this store's module registry.
            if let Some((info, module)) = store.modules().lookup_frame_info(pc_to_lookup) {
                wasm_trace.push(info);

                // If this frame has unparsed debug information and the
                // store's configuration indicates that we were
                // respecting the environment variable of whether to
                // do this then we will print out a helpful note in
                // `Display` to indicate that more detailed information
                // in a trap may be available.
                let has_unparsed_debuginfo = module.compiled_module().has_unparsed_debuginfo();
                if has_unparsed_debuginfo
                    && wasm_backtrace_details_env_used
                    && cfg!(feature = "addr2line")
                {
                    hint_wasm_backtrace_details_env = true;
                }
            }
        }

        Self {
            wasm_trace,
            _runtime_trace: runtime_trace,
            hint_wasm_backtrace_details_env,
        }
    }

    /// Returns a list of function frames in WebAssembly this backtrace
    /// represents.
    pub fn frames(&self) -> &[FrameInfo] {
        self.wasm_trace.as_slice()
    }
}

impl fmt::Display for WasmBacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "error while executing at wasm backtrace:")?;

        let mut needs_newline = false;
        for (i, frame) in self.wasm_trace.iter().enumerate() {
            // Avoid putting a trailing newline on the output
            if needs_newline {
                writeln!(f, "")?;
            } else {
                needs_newline = true;
            }
            let name = frame.module().name().unwrap_or("<unknown>");
            write!(f, "  {i:>3}: ")?;

            if let Some(offset) = frame.module_offset() {
                write!(f, "{offset:#8x} - ")?;
            }

            let write_raw_func_name = |f: &mut fmt::Formatter<'_>| {
                demangle_function_name_or_index(f, frame.func_name(), frame.func_index() as usize)
            };
            if frame.symbols().is_empty() {
                write!(f, "{name}!")?;
                write_raw_func_name(f)?;
            } else {
                for (i, symbol) in frame.symbols().iter().enumerate() {
                    if i > 0 {
                        if needs_newline {
                            writeln!(f, "")?;
                        } else {
                            needs_newline = true;
                        }
                        write!(f, "                - ")?;
                    } else {
                        // ...
                    }
                    match symbol.name() {
                        Some(name) => demangle_function_name(f, name)?,
                        None if i == 0 => write_raw_func_name(f)?,
                        None => write!(f, "<inlined function>")?,
                    }
                    if let Some(file) = symbol.file() {
                        writeln!(f, "")?;
                        write!(f, "                    at {file}")?;
                        if let Some(line) = symbol.line() {
                            write!(f, ":{line}")?;
                            if let Some(col) = symbol.column() {
                                write!(f, ":{col}")?;
                            }
                        }
                    }
                }
            }
        }
        if self.hint_wasm_backtrace_details_env {
            write!(
                f,
                "\nnote: using the `WASMTIME_BACKTRACE_DETAILS=1` \
                 environment variable may show more debugging information"
            )?;
        }
        Ok(())
    }
}

/// Description of a frame in a backtrace for a [`WasmBacktrace`].
///
/// Whenever an error happens while WebAssembly is executing a
/// [`WasmBacktrace`] will be attached to the error returned which can be used
/// to acquire this `FrameInfo`. For more information see [`WasmBacktrace`].
#[derive(Debug)]
pub struct FrameInfo {
    module: Module,
    func_index: u32,
    func_name: Option<String>,
    func_start: FilePos,
    instr: Option<FilePos>,
    symbols: Vec<FrameSymbol>,
}

impl FrameInfo {
    /// Fetches frame information about a program counter in a backtrace.
    ///
    /// Returns an object if this `pc` is known to this module, or returns `None`
    /// if no information can be found.
    pub(crate) fn new(module: Module, text_offset: usize) -> Option<FrameInfo> {
        let compiled_module = module.compiled_module();
        let (index, _func_offset) = compiled_module.func_by_text_offset(text_offset)?;
        let func_start = compiled_module.func_start_srcloc(index);
        let instr = wasmtime_environ::lookup_file_pos(
            compiled_module.code_memory().address_map_data(),
            text_offset,
        );
        let index = compiled_module.module().func_index(index);
        let func_index = index.as_u32();
        let func_name = compiled_module.func_name(index).map(|s| s.to_string());

        // In debug mode for now assert that we found a mapping for `pc` within
        // the function, because otherwise something is buggy along the way and
        // not accounting for all the instructions. This isn't super critical
        // though so we can omit this check in release mode.
        //
        // Note that if the module doesn't even have an address map due to
        // compilation settings then it's expected that `instr` is `None`.
        debug_assert!(
            instr.is_some() || !compiled_module.has_address_map(),
            "failed to find instruction for {text_offset:#x}"
        );

        // Use our wasm-relative pc to symbolize this frame. If there's a
        // symbolication context (dwarf debug info) available then we can try to
        // look this up there.
        //
        // Note that dwarf pcs are code-section-relative, hence the subtraction
        // from the location of `instr`. Also note that all errors are ignored
        // here for now since technically wasm modules can always have any
        // custom section contents.
        let mut symbols = Vec::new();

        let _ = &mut symbols;
        #[cfg(feature = "addr2line")]
        if let Some(s) = &compiled_module.symbolize_context().ok().and_then(|c| c) {
            if let Some(offset) = instr.and_then(|i| i.file_offset()) {
                let to_lookup = u64::from(offset) - s.code_section_offset();
                if let Ok(mut frames) = s.addr2line().find_frames(to_lookup).skip_all_loads() {
                    while let Ok(Some(frame)) = frames.next() {
                        symbols.push(FrameSymbol {
                            name: frame
                                .function
                                .as_ref()
                                .and_then(|l| l.raw_name().ok())
                                .map(|s| s.to_string()),
                            file: frame
                                .location
                                .as_ref()
                                .and_then(|l| l.file)
                                .map(|s| s.to_string()),
                            line: frame.location.as_ref().and_then(|l| l.line),
                            column: frame.location.as_ref().and_then(|l| l.column),
                        });
                    }
                }
            }
        }

        Some(FrameInfo {
            module,
            func_index,
            func_name,
            instr,
            func_start,
            symbols,
        })
    }

    /// Returns the WebAssembly function index for this frame.
    ///
    /// This function index is the index in the function index space of the
    /// WebAssembly module that this frame comes from.
    pub fn func_index(&self) -> u32 {
        self.func_index
    }

    /// Returns the module for this frame.
    ///
    /// This is the module who's code was being run in this frame.
    pub fn module(&self) -> &Module {
        &self.module
    }

    /// Returns a descriptive name of the function for this frame, if one is
    /// available.
    ///
    /// The name of this function may come from the `name` section of the
    /// WebAssembly binary, or wasmtime may try to infer a better name for it if
    /// not available, for example the name of the export if it's exported.
    ///
    /// This return value is primarily used for debugging and human-readable
    /// purposes for things like traps. Note that the exact return value may be
    /// tweaked over time here and isn't guaranteed to be something in
    /// particular about a wasm module due to its primary purpose of assisting
    /// in debugging.
    ///
    /// This function returns `None` when no name could be inferred.
    pub fn func_name(&self) -> Option<&str> {
        self.func_name.as_deref()
    }

    /// Returns the offset within the original wasm module this frame's program
    /// counter was at.
    ///
    /// The offset here is the offset from the beginning of the original wasm
    /// module to the instruction that this frame points to.
    ///
    /// Note that `None` may be returned if the original module was not
    /// compiled with mapping information to yield this information. This is
    /// controlled by the
    /// [`Config::generate_address_map`](crate::Config::generate_address_map)
    /// configuration option.
    pub fn module_offset(&self) -> Option<usize> {
        Some(self.instr?.file_offset()? as usize)
    }

    /// Returns the offset from the original wasm module's function to this
    /// frame's program counter.
    ///
    /// The offset here is the offset from the beginning of the defining
    /// function of this frame (within the wasm module) to the instruction this
    /// frame points to.
    ///
    /// Note that `None` may be returned if the original module was not
    /// compiled with mapping information to yield this information. This is
    /// controlled by the
    /// [`Config::generate_address_map`](crate::Config::generate_address_map)
    /// configuration option.
    pub fn func_offset(&self) -> Option<usize> {
        let instr_offset = self.instr?.file_offset()?;
        Some((instr_offset - self.func_start.file_offset()?) as usize)
    }

    /// Returns the debug symbols found, if any, for this function frame.
    ///
    /// When a wasm program is compiled with DWARF debug information then this
    /// function may be populated to return symbols which contain extra debug
    /// information about a frame including the filename and line number. If no
    /// debug information was found or if it was malformed then this will return
    /// an empty array.
    pub fn symbols(&self) -> &[FrameSymbol] {
        &self.symbols
    }
}

/// Debug information for a symbol that is attached to a [`FrameInfo`].
///
/// When DWARF debug information is present in a wasm file then this structure
/// can be found on a [`FrameInfo`] and can be used to learn about filenames,
/// line numbers, etc, which are the origin of a function in a stack trace.
#[derive(Debug)]
pub struct FrameSymbol {
    name: Option<String>,
    file: Option<String>,
    line: Option<u32>,
    column: Option<u32>,
}

impl FrameSymbol {
    /// Returns the function name associated with this symbol.
    ///
    /// Note that this may not be present with malformed debug information, or
    /// the debug information may not include it. Also note that the symbol is
    /// frequently mangled, so you might need to run some form of demangling
    /// over it.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the source code filename this symbol was defined in.
    ///
    /// Note that this may not be present with malformed debug information, or
    /// the debug information may not include it.
    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    /// Returns the 1-indexed source code line number this symbol was defined
    /// on.
    ///
    /// Note that this may not be present with malformed debug information, or
    /// the debug information may not include it.
    pub fn line(&self) -> Option<u32> {
        self.line
    }

    /// Returns the 1-indexed source code column number this symbol was defined
    /// on.
    ///
    /// Note that this may not be present with malformed debug information, or
    /// the debug information may not include it.
    pub fn column(&self) -> Option<u32> {
        self.column
    }
}
