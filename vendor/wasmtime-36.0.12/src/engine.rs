use crate::Config;
use crate::prelude::*;
#[cfg(feature = "runtime")]
pub use crate::runtime::code_memory::CustomCodeMemory;
#[cfg(feature = "runtime")]
use crate::runtime::type_registry::TypeRegistry;
#[cfg(feature = "runtime")]
use crate::runtime::vm::GcRuntime;
use alloc::sync::Arc;
use core::ptr::NonNull;
#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{AtomicU64, Ordering};
#[cfg(any(feature = "cranelift", feature = "winch"))]
use object::write::{Object, StandardSegment};
#[cfg(feature = "std")]
use std::{fs::File, path::Path};
use wasmparser::WasmFeatures;
use wasmtime_environ::{FlagValue, ObjectKind, TripleExt, Tunables};

mod serialization;

/// An `Engine` which is a global context for compilation and management of wasm
/// modules.
///
/// An engine can be safely shared across threads and is a cheap cloneable
/// handle to the actual engine. The engine itself will be deallocated once all
/// references to it have gone away.
///
/// Engines store global configuration preferences such as compilation settings,
/// enabled features, etc. You'll likely only need at most one of these for a
/// program.
///
/// ## Engines and `Clone`
///
/// Using `clone` on an `Engine` is a cheap operation. It will not create an
/// entirely new engine, but rather just a new reference to the existing engine.
/// In other words it's a shallow copy, not a deep copy.
///
/// ## Engines and `Default`
///
/// You can create an engine with default configuration settings using
/// `Engine::default()`. Be sure to consult the documentation of [`Config`] for
/// default settings.
#[derive(Clone)]
pub struct Engine {
    inner: Arc<EngineInner>,
}

struct EngineInner {
    config: Config,
    features: WasmFeatures,
    tunables: Tunables,
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    compiler: Box<dyn wasmtime_environ::Compiler>,
    #[cfg(feature = "runtime")]
    allocator: Box<dyn crate::runtime::vm::InstanceAllocator + Send + Sync>,
    #[cfg(feature = "runtime")]
    gc_runtime: Option<Arc<dyn GcRuntime>>,
    #[cfg(feature = "runtime")]
    profiler: Box<dyn crate::profiling_agent::ProfilingAgent>,
    #[cfg(feature = "runtime")]
    signatures: TypeRegistry,
    #[cfg(all(feature = "runtime", target_has_atomic = "64"))]
    epoch: AtomicU64,

    /// One-time check of whether the compiler's settings, if present, are
    /// compatible with the native host.
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    compatible_with_native_host: crate::sync::OnceLock<Result<(), String>>,
}

impl core::fmt::Debug for Engine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Engine")
            .field(&Arc::as_ptr(&self.inner))
            .finish()
    }
}

impl Default for Engine {
    fn default() -> Engine {
        Engine::new(&Config::default()).unwrap()
    }
}

impl Engine {
    /// Creates a new [`Engine`] with the specified compilation and
    /// configuration settings.
    ///
    /// # Errors
    ///
    /// This method can fail if the `config` is invalid or some
    /// configurations are incompatible.
    ///
    /// For example, feature `reference_types` will need to set
    /// the compiler setting `enable_safepoints` and `unwind_info`
    /// to `true`, but explicitly disable these two compiler settings
    /// will cause errors.
    pub fn new(config: &Config) -> Result<Engine> {
        let config = config.clone();
        let (tunables, features) = config.validate()?;

        #[cfg(feature = "runtime")]
        if tunables.signals_based_traps {
            // Ensure that crate::runtime::vm's signal handlers are
            // configured. This is the per-program initialization required for
            // handling traps, such as configuring signals, vectored exception
            // handlers, etc.
            #[cfg(has_native_signals)]
            crate::runtime::vm::init_traps(config.macos_use_mach_ports);
            if !cfg!(miri) {
                #[cfg(all(has_host_compiler_backend, feature = "debug-builtins"))]
                crate::runtime::vm::debug_builtins::init();
            }
        }

        #[cfg(any(feature = "cranelift", feature = "winch"))]
        let (config, compiler) = config.build_compiler(&tunables, features)?;

        Ok(Engine {
            inner: Arc::new(EngineInner {
                #[cfg(any(feature = "cranelift", feature = "winch"))]
                compiler,
                #[cfg(feature = "runtime")]
                allocator: {
                    let allocator = config.build_allocator(&tunables)?;
                    #[cfg(feature = "gc")]
                    {
                        let mem_ty = tunables.gc_heap_memory_type();
                        allocator.validate_memory(&mem_ty).context(
                            "instance allocator cannot support configured GC heap memory",
                        )?;
                    }
                    allocator
                },
                #[cfg(feature = "runtime")]
                gc_runtime: config.build_gc_runtime()?,
                #[cfg(feature = "runtime")]
                profiler: config.build_profiler()?,
                #[cfg(feature = "runtime")]
                signatures: TypeRegistry::new(),
                #[cfg(all(feature = "runtime", target_has_atomic = "64"))]
                epoch: AtomicU64::new(0),
                #[cfg(any(feature = "cranelift", feature = "winch"))]
                compatible_with_native_host: Default::default(),
                config,
                tunables,
                features,
            }),
        })
    }

    /// Returns the configuration settings that this engine is using.
    #[inline]
    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    #[inline]
    pub(crate) fn features(&self) -> WasmFeatures {
        self.inner.features
    }

    pub(crate) fn run_maybe_parallel<
        A: Send,
        B: Send,
        E: Send,
        F: Fn(A) -> Result<B, E> + Send + Sync,
    >(
        &self,
        input: Vec<A>,
        f: F,
    ) -> Result<Vec<B>, E> {
        if self.config().parallel_compilation {
            #[cfg(feature = "parallel-compilation")]
            {
                use rayon::prelude::*;
                // If we collect into Result<Vec<B>, E> directly, the returned error is not
                // deterministic, because any error could be returned early. So we first materialize
                // all results in order and then return the first error deterministically, or Ok(_).
                return input
                    .into_par_iter()
                    .map(|a| f(a))
                    .collect::<Vec<Result<B, E>>>()
                    .into_iter()
                    .collect::<Result<Vec<B>, E>>();
            }
        }

        // In case the parallel-compilation feature is disabled or the parallel_compilation config
        // was turned off dynamically fallback to the non-parallel version.
        input
            .into_iter()
            .map(|a| f(a))
            .collect::<Result<Vec<B>, E>>()
    }

    #[cfg(any(feature = "cranelift", feature = "winch"))]
    pub(crate) fn run_maybe_parallel_mut<
        T: Send,
        E: Send,
        F: Fn(&mut T) -> Result<(), E> + Send + Sync,
    >(
        &self,
        input: &mut [T],
        f: F,
    ) -> Result<(), E> {
        if self.config().parallel_compilation {
            #[cfg(feature = "parallel-compilation")]
            {
                use rayon::prelude::*;
                // If we collect into `Result<(), E>` directly, the returned
                // error is not deterministic, because any error could be
                // returned early. So we first materialize all results in order
                // and then return the first error deterministically, or
                // `Ok(_)`.
                return input
                    .into_par_iter()
                    .map(|a| f(a))
                    .collect::<Vec<Result<(), E>>>()
                    .into_iter()
                    .collect::<Result<(), E>>();
            }
        }

        // In case the parallel-compilation feature is disabled or the
        // parallel_compilation config was turned off dynamically fallback to
        // the non-parallel version.
        input.into_iter().map(|a| f(a)).collect::<Result<(), E>>()
    }

    /// Take a weak reference to this engine.
    pub fn weak(&self) -> EngineWeak {
        EngineWeak {
            inner: Arc::downgrade(&self.inner),
        }
    }

    #[inline]
    pub(crate) fn tunables(&self) -> &Tunables {
        &self.inner.tunables
    }

    /// Returns whether the engine `a` and `b` refer to the same configuration.
    #[inline]
    pub fn same(a: &Engine, b: &Engine) -> bool {
        Arc::ptr_eq(&a.inner, &b.inner)
    }

    /// Returns whether the engine is configured to support async functions.
    #[cfg(feature = "async")]
    #[inline]
    pub fn is_async(&self) -> bool {
        self.config().async_support
    }

    /// Detects whether the bytes provided are a precompiled object produced by
    /// Wasmtime.
    ///
    /// This function will inspect the header of `bytes` to determine if it
    /// looks like a precompiled core wasm module or a precompiled component.
    /// This does not validate the full structure or guarantee that
    /// deserialization will succeed, instead it helps higher-levels of the
    /// stack make a decision about what to do next when presented with the
    /// `bytes` as an input module.
    ///
    /// If the `bytes` looks like a precompiled object previously produced by
    /// [`Module::serialize`](crate::Module::serialize),
    /// [`Component::serialize`](crate::component::Component::serialize),
    /// [`Engine::precompile_module`], or [`Engine::precompile_component`], then
    /// this will return `Some(...)` indicating so. Otherwise `None` is
    /// returned.
    pub fn detect_precompiled(bytes: &[u8]) -> Option<Precompiled> {
        serialization::detect_precompiled_bytes(bytes)
    }

    /// Like [`Engine::detect_precompiled`], but performs the detection on a file.
    #[cfg(feature = "std")]
    pub fn detect_precompiled_file(path: impl AsRef<Path>) -> Result<Option<Precompiled>> {
        serialization::detect_precompiled_file(path)
    }

    /// Returns the target triple which this engine is compiling code for
    /// and/or running code for.
    pub(crate) fn target(&self) -> target_lexicon::Triple {
        return self.config().compiler_target();
    }

    /// Verify that this engine's configuration is compatible with loading
    /// modules onto the native host platform.
    ///
    /// This method is used as part of `Module::new` to ensure that this
    /// engine can indeed load modules for the configured compiler (if any).
    /// Note that if cranelift is disabled this trivially returns `Ok` because
    /// loaded serialized modules are checked separately.
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    pub(crate) fn check_compatible_with_native_host(&self) -> Result<()> {
        self.inner
            .compatible_with_native_host
            .get_or_init(|| self._check_compatible_with_native_host())
            .clone()
            .map_err(anyhow::Error::msg)
    }

    #[cfg(any(feature = "cranelift", feature = "winch"))]
    fn _check_compatible_with_native_host(&self) -> Result<(), String> {
        use target_lexicon::Triple;

        let compiler = self.compiler();

        let target = compiler.triple();
        let host = Triple::host();
        let target_matches_host = || {
            // If the host target and target triple match, then it's valid
            // to run results of compilation on this host.
            if host == *target {
                return true;
            }

            // If there's a mismatch and the target is a compatible pulley
            // target, then that's also ok to run.
            if cfg!(feature = "pulley")
                && target.is_pulley()
                && target.pointer_width() == host.pointer_width()
                && target.endianness() == host.endianness()
            {
                return true;
            }

            // ... otherwise everything else is considered not a match.
            false
        };

        if !target_matches_host() {
            return Err(format!(
                "target '{target}' specified in the configuration does not match the host"
            ));
        }

        // Also double-check all compiler settings
        for (key, value) in compiler.flags().iter() {
            self.check_compatible_with_shared_flag(key, value)?;
        }
        for (key, value) in compiler.isa_flags().iter() {
            self.check_compatible_with_isa_flag(key, value)?;
        }

        // Double-check that this configuration isn't requesting capabilities
        // that this build of Wasmtime doesn't support.
        if !cfg!(has_native_signals) && self.tunables().signals_based_traps {
            return Err("signals-based-traps disabled at compile time -- cannot be enabled".into());
        }
        if !cfg!(has_virtual_memory) && self.tunables().memory_init_cow {
            return Err("virtual memory disabled at compile time -- cannot enable CoW".into());
        }
        if !cfg!(target_has_atomic = "64") && self.tunables().epoch_interruption {
            return Err("epochs currently require 64-bit atomics".into());
        }
        Ok(())
    }

    /// Checks to see whether the "shared flag", something enabled for
    /// individual compilers, is compatible with the native host platform.
    ///
    /// This is used both when validating an engine's compilation settings are
    /// compatible with the host as well as when deserializing modules from
    /// disk to ensure they're compatible with the current host.
    ///
    /// Note that most of the settings here are not configured by users that
    /// often. While theoretically possible via `Config` methods the more
    /// interesting flags are the ISA ones below. Typically the values here
    /// represent global configuration for wasm features. Settings here
    /// currently rely on the compiler informing us of all settings, including
    /// those disabled. Settings then fall in a few buckets:
    ///
    /// * Some settings must be enabled, such as `preserve_frame_pointers`.
    /// * Some settings must have a particular value, such as
    ///   `libcall_call_conv`.
    /// * Some settings do not matter as to their value, such as `opt_level`.
    pub(crate) fn check_compatible_with_shared_flag(
        &self,
        flag: &str,
        value: &FlagValue,
    ) -> Result<(), String> {
        let target = self.target();
        let ok = match flag {
            // These settings must all have be enabled, since their value
            // can affect the way the generated code performs or behaves at
            // runtime.
            "libcall_call_conv" => *value == FlagValue::Enum("isa_default"),
            "preserve_frame_pointers" => *value == FlagValue::Bool(true),
            "enable_probestack" => *value == FlagValue::Bool(true),
            "probestack_strategy" => *value == FlagValue::Enum("inline"),
            "enable_multi_ret_implicit_sret" => *value == FlagValue::Bool(true),

            // Features wasmtime doesn't use should all be disabled, since
            // otherwise if they are enabled it could change the behavior of
            // generated code.
            "enable_llvm_abi_extensions" => *value == FlagValue::Bool(false),
            "enable_pinned_reg" => *value == FlagValue::Bool(false),
            "use_colocated_libcalls" => *value == FlagValue::Bool(false),
            "use_pinned_reg_as_heap_base" => *value == FlagValue::Bool(false),

            // If reference types (or anything that depends on reference types,
            // like typed function references and GC) are enabled this must be
            // enabled, otherwise this setting can have any value.
            "enable_safepoints" => {
                if self.features().contains(WasmFeatures::REFERENCE_TYPES) {
                    *value == FlagValue::Bool(true)
                } else {
                    return Ok(())
                }
            }

            // Windows requires unwind info as part of its ABI.
            "unwind_info" => {
                if target.operating_system == target_lexicon::OperatingSystem::Windows {
                    *value == FlagValue::Bool(true)
                } else {
                    return Ok(())
                }
            }

            // stack switch model must match the current OS
            "stack_switch_model" => {
                if self.features().contains(WasmFeatures::STACK_SWITCHING) {
                    use target_lexicon::OperatingSystem;
                    let expected =
                    match target.operating_system  {
                        OperatingSystem::Windows => "update_windows_tib",
                        OperatingSystem::Linux
                        | OperatingSystem::MacOSX(_)
                        | OperatingSystem::Darwin(_)  => "basic",
                        _ => { return Err(String::from("stack-switching feature not supported on this platform")); }
                    };
                    *value == FlagValue::Enum(expected)
                } else {
                    return Ok(())
                }
            }

            // These settings don't affect the interface or functionality of
            // the module itself, so their configuration values shouldn't
            // matter.
            "enable_heap_access_spectre_mitigation"
            | "enable_table_access_spectre_mitigation"
            | "enable_nan_canonicalization"
            | "enable_jump_tables"
            | "enable_float"
            | "enable_verifier"
            | "enable_pcc"
            | "regalloc_checker"
            | "regalloc_verbose_logs"
            | "regalloc_algorithm"
            | "is_pic"
            | "bb_padding_log2_minus_one"
            | "log2_min_function_alignment"
            | "machine_code_cfg_info"
            | "tls_model" // wasmtime doesn't use tls right now
            | "opt_level" // opt level doesn't change semantics
            | "enable_alias_analysis" // alias analysis-based opts don't change semantics
            | "probestack_size_log2" // probestack above asserted disabled
            | "regalloc" // shouldn't change semantics
            | "enable_incremental_compilation_cache_checks" // shouldn't change semantics
            | "enable_atomics" => return Ok(()),

            // Everything else is unknown and needs to be added somewhere to
            // this list if encountered.
            _ => {
                return Err(format!("unknown shared setting {flag:?} configured to {value:?}"))
            }
        };

        if !ok {
            return Err(format!(
                "setting {flag:?} is configured to {value:?} which is not supported",
            ));
        }
        Ok(())
    }

    /// Same as `check_compatible_with_native_host` except used for ISA-specific
    /// flags. This is used to test whether a configured ISA flag is indeed
    /// available on the host platform itself.
    pub(crate) fn check_compatible_with_isa_flag(
        &self,
        flag: &str,
        value: &FlagValue,
    ) -> Result<(), String> {
        match value {
            // ISA flags are used for things like CPU features, so if they're
            // disabled then it's compatible with the native host.
            FlagValue::Bool(false) => return Ok(()),

            // Fall through below where we test at runtime that features are
            // available.
            FlagValue::Bool(true) => {}

            // Pulley's pointer_width must match the host.
            FlagValue::Enum("pointer32") => {
                return if cfg!(target_pointer_width = "32") {
                    Ok(())
                } else {
                    Err("wrong host pointer width".to_string())
                };
            }
            FlagValue::Enum("pointer64") => {
                return if cfg!(target_pointer_width = "64") {
                    Ok(())
                } else {
                    Err("wrong host pointer width".to_string())
                };
            }

            // Only `bool` values are supported right now, other settings would
            // need more support here.
            _ => {
                return Err(format!(
                    "isa-specific feature {flag:?} configured to unknown value {value:?}"
                ));
            }
        }

        let host_feature = match flag {
            // aarch64 features to detect
            "has_lse" => "lse",
            "has_pauth" => "paca",
            "has_fp16" => "fp16",

            // aarch64 features which don't need detection
            // No effect on its own.
            "sign_return_address_all" => return Ok(()),
            // The pointer authentication instructions act as a `NOP` when
            // unsupported, so it is safe to enable them.
            "sign_return_address" => return Ok(()),
            // No effect on its own.
            "sign_return_address_with_bkey" => return Ok(()),
            // The `BTI` instruction acts as a `NOP` when unsupported, so it
            // is safe to enable it regardless of whether the host supports it
            // or not.
            "use_bti" => return Ok(()),

            // s390x features to detect
            "has_vxrs_ext2" => "vxrs_ext2",
            "has_vxrs_ext3" => "vxrs_ext3",
            "has_mie3" => "mie3",
            "has_mie4" => "mie4",

            // x64 features to detect
            "has_cmpxchg16b" => "cmpxchg16b",
            "has_sse3" => "sse3",
            "has_ssse3" => "ssse3",
            "has_sse41" => "sse4.1",
            "has_sse42" => "sse4.2",
            "has_popcnt" => "popcnt",
            "has_avx" => "avx",
            "has_avx2" => "avx2",
            "has_fma" => "fma",
            "has_bmi1" => "bmi1",
            "has_bmi2" => "bmi2",
            "has_avx512bitalg" => "avx512bitalg",
            "has_avx512dq" => "avx512dq",
            "has_avx512f" => "avx512f",
            "has_avx512vl" => "avx512vl",
            "has_avx512vbmi" => "avx512vbmi",
            "has_lzcnt" => "lzcnt",

            // pulley features
            "big_endian" if cfg!(target_endian = "big") => return Ok(()),
            "big_endian" if cfg!(target_endian = "little") => {
                return Err("wrong host endianness".to_string());
            }

            _ => {
                // FIXME: should enumerate risc-v features and plumb them
                // through to the `detect_host_feature` function.
                if cfg!(target_arch = "riscv64") && flag != "not_a_flag" {
                    return Ok(());
                }
                return Err(format!(
                    "don't know how to test for target-specific flag {flag:?} at runtime"
                ));
            }
        };

        let detect = match self.config().detect_host_feature {
            Some(detect) => detect,
            None => {
                return Err(format!(
                    "cannot determine if host feature {host_feature:?} is \
                     available at runtime, configure a probing function with \
                     `Config::detect_host_feature`"
                ));
            }
        };

        match detect(host_feature) {
            Some(true) => Ok(()),
            Some(false) => Err(format!(
                "compilation setting {flag:?} is enabled, but not \
                 available on the host",
            )),
            None => Err(format!(
                "failed to detect if target-specific flag {flag:?} is \
                 available at runtime"
            )),
        }
    }

    /// Returns whether this [`Engine`] is configured to execute with Pulley,
    /// Wasmtime's interpreter.
    ///
    /// Note that Pulley is the default for host platforms that do not have a
    /// Cranelift backend to support them. For example at the time of this
    /// writing 32-bit x86 is not supported in Cranelift so the
    /// `i686-unknown-linux-gnu` target would by default return `true` here.
    pub fn is_pulley(&self) -> bool {
        self.target().is_pulley()
    }
}

#[cfg(any(feature = "cranelift", feature = "winch"))]
impl Engine {
    pub(crate) fn compiler(&self) -> &dyn wasmtime_environ::Compiler {
        &*self.inner.compiler
    }

    /// Ahead-of-time (AOT) compiles a WebAssembly module.
    ///
    /// The `bytes` provided must be in one of two formats:
    ///
    /// * A [binary-encoded][binary] WebAssembly module. This is always supported.
    /// * A [text-encoded][text] instance of the WebAssembly text format.
    ///   This is only supported when the `wat` feature of this crate is enabled.
    ///   If this is supplied then the text format will be parsed before validation.
    ///   Note that the `wat` feature is enabled by default.
    ///
    /// This method may be used to compile a module for use with a different target
    /// host. The output of this method may be used with
    /// [`Module::deserialize`](crate::Module::deserialize) on hosts compatible
    /// with the [`Config`](crate::Config) associated with this [`Engine`].
    ///
    /// The output of this method is safe to send to another host machine for later
    /// execution. As the output is already a compiled module, translation and code
    /// generation will be skipped and this will improve the performance of constructing
    /// a [`Module`](crate::Module) from the output of this method.
    ///
    /// [binary]: https://webassembly.github.io/spec/core/binary/index.html
    /// [text]: https://webassembly.github.io/spec/core/text/index.html
    pub fn precompile_module(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        crate::CodeBuilder::new(self)
            .wasm_binary_or_text(bytes, None)?
            .compile_module_serialized()
    }

    /// Same as [`Engine::precompile_module`] except for a
    /// [`Component`](crate::component::Component)
    #[cfg(feature = "component-model")]
    pub fn precompile_component(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        crate::CodeBuilder::new(self)
            .wasm_binary_or_text(bytes, None)?
            .compile_component_serialized()
    }

    /// Produces a blob of bytes by serializing the `engine`'s configuration data to
    /// be checked, perhaps in a different process, with the `check_compatible`
    /// method below.
    ///
    /// The blob of bytes is inserted into the object file specified to become part
    /// of the final compiled artifact.
    pub(crate) fn append_compiler_info(&self, obj: &mut Object<'_>) {
        serialization::append_compiler_info(self, obj, &serialization::Metadata::new(&self))
    }

    #[cfg(any(feature = "cranelift", feature = "winch"))]
    pub(crate) fn append_bti(&self, obj: &mut Object<'_>) {
        let section = obj.add_section(
            obj.segment_name(StandardSegment::Data).to_vec(),
            wasmtime_environ::obj::ELF_WASM_BTI.as_bytes().to_vec(),
            object::SectionKind::ReadOnlyData,
        );
        let contents = if self.compiler().is_branch_protection_enabled() {
            1
        } else {
            0
        };
        obj.append_section_data(section, &[contents], 1);
    }
}

/// Return value from the [`Engine::detect_precompiled`] API.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Precompiled {
    /// The input bytes look like a precompiled core wasm module.
    Module,
    /// The input bytes look like a precompiled wasm component.
    Component,
}

#[cfg(feature = "runtime")]
impl Engine {
    /// Eagerly initialize thread-local functionality shared by all [`Engine`]s.
    ///
    /// Wasmtime's implementation on some platforms may involve per-thread
    /// setup that needs to happen whenever WebAssembly is invoked. This setup
    /// can take on the order of a few hundred microseconds, whereas the
    /// overhead of calling WebAssembly is otherwise on the order of a few
    /// nanoseconds. This setup cost is paid once per-OS-thread. If your
    /// application is sensitive to the latencies of WebAssembly function
    /// calls, even those that happen first on a thread, then this function
    /// can be used to improve the consistency of each call into WebAssembly
    /// by explicitly frontloading the cost of the one-time setup per-thread.
    ///
    /// Note that this function is not required to be called in any embedding.
    /// Wasmtime will automatically initialize thread-local-state as necessary
    /// on calls into WebAssembly. This is provided for use cases where the
    /// latency of WebAssembly calls are extra-important, which is not
    /// necessarily true of all embeddings.
    pub fn tls_eager_initialize() {
        crate::runtime::vm::tls_eager_initialize();
    }

    pub(crate) fn allocator(&self) -> &dyn crate::runtime::vm::InstanceAllocator {
        self.inner.allocator.as_ref()
    }

    pub(crate) fn gc_runtime(&self) -> Option<&Arc<dyn GcRuntime>> {
        self.inner.gc_runtime.as_ref()
    }

    pub(crate) fn profiler(&self) -> &dyn crate::profiling_agent::ProfilingAgent {
        self.inner.profiler.as_ref()
    }

    #[cfg(all(feature = "cache", any(feature = "cranelift", feature = "winch")))]
    pub(crate) fn cache(&self) -> Option<&wasmtime_cache::Cache> {
        self.config().cache.as_ref()
    }

    pub(crate) fn signatures(&self) -> &TypeRegistry {
        &self.inner.signatures
    }

    #[cfg(feature = "runtime")]
    pub(crate) fn custom_code_memory(&self) -> Option<&Arc<dyn CustomCodeMemory>> {
        self.config().custom_code_memory.as_ref()
    }

    #[cfg(target_has_atomic = "64")]
    pub(crate) fn epoch_counter(&self) -> &AtomicU64 {
        &self.inner.epoch
    }

    #[cfg(target_has_atomic = "64")]
    pub(crate) fn current_epoch(&self) -> u64 {
        self.epoch_counter().load(Ordering::Relaxed)
    }

    /// Increments the epoch.
    ///
    /// When using epoch-based interruption, currently-executing Wasm
    /// code within this engine will trap or yield "soon" when the
    /// epoch deadline is reached or exceeded. (The configuration, and
    /// the deadline, are set on the `Store`.) The intent of the
    /// design is for this method to be called by the embedder at some
    /// regular cadence, for example by a thread that wakes up at some
    /// interval, or by a signal handler.
    ///
    /// See [`Config::epoch_interruption`](crate::Config::epoch_interruption)
    /// for an introduction to epoch-based interruption and pointers
    /// to the other relevant methods.
    ///
    /// When performing `increment_epoch` in a separate thread, consider using
    /// [`Engine::weak`] to hold an [`EngineWeak`](crate::EngineWeak) and
    /// performing [`EngineWeak::upgrade`](crate::EngineWeak::upgrade) on each
    /// tick, so that the epoch ticking thread does not keep an [`Engine`] alive
    /// longer than any of its consumers.
    ///
    /// ## Signal Safety
    ///
    /// This method is signal-safe: it does not make any syscalls, and
    /// performs only an atomic increment to the epoch value in
    /// memory.
    #[cfg(target_has_atomic = "64")]
    pub fn increment_epoch(&self) {
        self.inner.epoch.fetch_add(1, Ordering::Relaxed);
    }

    /// Returns a [`std::hash::Hash`] that can be used to check precompiled WebAssembly compatibility.
    ///
    /// The outputs of [`Engine::precompile_module`] and [`Engine::precompile_component`]
    /// are compatible with a different [`Engine`] instance only if the two engines use
    /// compatible [`Config`]s. If this Hash matches between two [`Engine`]s then binaries
    /// from one are guaranteed to deserialize in the other.
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    pub fn precompile_compatibility_hash(&self) -> impl std::hash::Hash + '_ {
        crate::compile::HashedEngineCompileEnv(self)
    }

    /// Returns the required alignment for a code image, if we
    /// allocate in a way that is not a system `mmap()` that naturally
    /// aligns it.
    fn required_code_alignment(&self) -> usize {
        self.custom_code_memory()
            .map(|c| c.required_alignment())
            .unwrap_or(1)
    }

    /// Loads a `CodeMemory` from the specified in-memory slice, copying it to a
    /// uniquely owned mmap.
    ///
    /// The `expected` marker here is whether the bytes are expected to be a
    /// precompiled module or a component.
    pub(crate) fn load_code_bytes(
        &self,
        bytes: &[u8],
        expected: ObjectKind,
    ) -> Result<Arc<crate::CodeMemory>> {
        self.load_code(
            crate::runtime::vm::MmapVec::from_slice_with_alignment(
                bytes,
                self.required_code_alignment(),
            )?,
            expected,
        )
    }

    /// Loads a `CodeMemory` from the specified memory region without copying
    ///
    /// The `expected` marker here is whether the bytes are expected to be
    /// a precompiled module or a component.  The `memory` provided is expected
    /// to be a serialized module (.cwasm) generated by `[Module::serialize]`
    /// or [`Engine::precompile_module] or their `Component` counterparts
    /// [`Component::serialize`] or `[Engine::precompile_component]`.
    ///
    /// The memory provided is guaranteed to only be immutably by the runtime.
    ///
    /// # Safety
    ///
    /// As there is no copy here, the runtime will be making direct readonly use
    /// of the provided memory. As such, outside writes to this memory region
    /// will result in undefined and likely very undesirable behavior.
    pub(crate) unsafe fn load_code_raw(
        &self,
        memory: NonNull<[u8]>,
        expected: ObjectKind,
    ) -> Result<Arc<crate::CodeMemory>> {
        self.load_code(crate::runtime::vm::MmapVec::from_raw(memory)?, expected)
    }

    /// Like `load_code_bytes`, but creates a mmap from a file on disk.
    #[cfg(feature = "std")]
    pub(crate) fn load_code_file(
        &self,
        file: File,
        expected: ObjectKind,
    ) -> Result<Arc<crate::CodeMemory>> {
        self.load_code(
            crate::runtime::vm::MmapVec::from_file(file)
                .with_context(|| "Failed to create file mapping".to_string())?,
            expected,
        )
    }

    pub(crate) fn load_code(
        &self,
        mmap: crate::runtime::vm::MmapVec,
        expected: ObjectKind,
    ) -> Result<Arc<crate::CodeMemory>> {
        serialization::check_compatible(self, &mmap, expected)?;
        let mut code = crate::CodeMemory::new(self, mmap)?;
        code.publish()?;
        Ok(Arc::new(code))
    }

    /// Unload process-related trap/signal handlers and destroy this engine.
    ///
    /// This method is not safe and is not widely applicable. It is not required
    /// to be called and is intended for use cases such as unloading a dynamic
    /// library from a process. It is difficult to invoke this method correctly
    /// and it requires careful coordination to do so.
    ///
    /// # Panics
    ///
    /// This method will panic if this `Engine` handle is not the last remaining
    /// engine handle.
    ///
    /// # Aborts
    ///
    /// This method will abort the process on some platforms in some situations
    /// where unloading the handler cannot be performed and an unrecoverable
    /// state is reached. For example on Unix platforms with signal handling
    /// the process will be aborted if the current signal handlers are not
    /// Wasmtime's.
    ///
    /// # Unsafety
    ///
    /// This method is not generally safe to call and has a number of
    /// preconditions that must be met to even possibly be safe. Even with these
    /// known preconditions met there may be other unknown invariants to uphold
    /// as well.
    ///
    /// * There must be no other instances of `Engine` elsewhere in the process.
    ///   Note that this isn't just copies of this `Engine` but it's any other
    ///   `Engine` at all. This unloads global state that is used by all
    ///   `Engine`s so this instance must be the last.
    ///
    /// * On Unix platforms no other signal handlers could have been installed
    ///   for signals that Wasmtime catches. In this situation Wasmtime won't
    ///   know how to restore signal handlers that Wasmtime possibly overwrote
    ///   when Wasmtime was initially loaded. If possible initialize other
    ///   libraries first and then initialize Wasmtime last (e.g. defer creating
    ///   an `Engine`).
    ///
    /// * All existing threads which have used this DLL or copy of Wasmtime may
    ///   no longer use this copy of Wasmtime. Per-thread state is not iterated
    ///   and destroyed. Only future threads may use future instances of this
    ///   Wasmtime itself.
    ///
    /// If other crashes are seen from using this method please feel free to
    /// file an issue to update the documentation here with more preconditions
    /// that must be met.
    #[cfg(has_native_signals)]
    pub unsafe fn unload_process_handlers(self) {
        assert_eq!(Arc::weak_count(&self.inner), 0);
        assert_eq!(Arc::strong_count(&self.inner), 1);

        #[cfg(not(miri))]
        crate::runtime::vm::deinit_traps();
    }
}

/// A weak reference to an [`Engine`].
#[derive(Clone)]
pub struct EngineWeak {
    inner: alloc::sync::Weak<EngineInner>,
}

impl EngineWeak {
    /// Upgrade this weak reference into an [`Engine`]. Returns `None` if
    /// strong references (the [`Engine`] type itself) no longer exist.
    pub fn upgrade(&self) -> Option<Engine> {
        alloc::sync::Weak::upgrade(&self.inner).map(|inner| Engine { inner })
    }
}
