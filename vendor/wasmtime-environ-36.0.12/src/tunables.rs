use crate::{IndexType, Limits, Memory, TripleExt};
use anyhow::{Error, Result, anyhow, bail};
use core::{fmt, str::FromStr};
use serde_derive::{Deserialize, Serialize};
use target_lexicon::{PointerWidth, Triple};

macro_rules! define_tunables {
    (
        $(#[$outer_attr:meta])*
        pub struct $tunables:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident : $field_ty:ty,
            )*
        }

        pub struct $config_tunables:ident {
            ...
        }
    ) => {
        $(#[$outer_attr])*
        pub struct $tunables {
            $(
                $(#[$field_attr])*
                pub $field: $field_ty,
            )*
        }

        /// Optional tunable configuration options used in `wasmtime::Config`
        #[derive(Default, Clone)]
        #[expect(missing_docs, reason = "macro-generated fields")]
        pub struct $config_tunables {
            $(pub $field: Option<$field_ty>,)*
        }

        impl $config_tunables {
            /// Formats configured fields into `f`.
            pub fn format(&self, f: &mut fmt::DebugStruct<'_,'_>) {
                $(
                    if let Some(val) = &self.$field {
                        f.field(stringify!($field), val);
                    }
                )*
            }

            /// Configure the `Tunables` provided.
            pub fn configure(&self, tunables: &mut Tunables) {
                $(
                    if let Some(val) = self.$field {
                        tunables.$field = val;
                    }
                )*
            }
        }
    };
}

define_tunables! {
    /// Tunable parameters for WebAssembly compilation.
    #[derive(Clone, Hash, Serialize, Deserialize, Debug)]
    pub struct Tunables {
        /// The garbage collector implementation to use, which implies the layout of
        /// GC objects and barriers that must be emitted in Wasm code.
        pub collector: Option<Collector>,

        /// Initial size, in bytes, to be allocated for linear memories.
        pub memory_reservation: u64,

        /// The size, in bytes, of the guard page region for linear memories.
        pub memory_guard_size: u64,

        /// The size, in bytes, to allocate at the end of a relocated linear
        /// memory for growth.
        pub memory_reservation_for_growth: u64,

        /// Whether or not to generate native DWARF debug information.
        pub generate_native_debuginfo: bool,

        /// Whether or not to retain DWARF sections in compiled modules.
        pub parse_wasm_debuginfo: bool,

        /// Whether or not fuel is enabled for generated code, meaning that fuel
        /// will be consumed every time a wasm instruction is executed.
        pub consume_fuel: bool,

        /// Whether or not we use epoch-based interruption.
        pub epoch_interruption: bool,

        /// Whether or not linear memories are allowed to be reallocated after
        /// initial allocation at runtime.
        pub memory_may_move: bool,

        /// Whether or not linear memory allocations will have a guard region at the
        /// beginning of the allocation in addition to the end.
        pub guard_before_linear_memory: bool,

        /// Whether to initialize tables lazily, so that instantiation is fast but
        /// indirect calls are a little slower. If false, tables are initialized
        /// eagerly from any active element segments that apply to them during
        /// instantiation.
        pub table_lazy_init: bool,

        /// Indicates whether an address map from compiled native code back to wasm
        /// offsets in the original file is generated.
        pub generate_address_map: bool,

        /// Flag for the component module whether adapter modules have debug
        /// assertions baked into them.
        pub debug_adapter_modules: bool,

        /// Whether or not lowerings for relaxed simd instructions are forced to
        /// be deterministic.
        pub relaxed_simd_deterministic: bool,

        /// Whether or not Wasm functions target the winch abi.
        pub winch_callable: bool,

        /// Whether or not the host will be using native signals (e.g. SIGILL,
        /// SIGSEGV, etc) to implement traps.
        pub signals_based_traps: bool,

        /// Whether CoW images might be used to initialize linear memories.
        pub memory_init_cow: bool,

        /// Whether to enable inlining in Wasmtime's compilation orchestration
        /// or not.
        pub inlining: bool,

        /// Whether to inline calls within the same core Wasm module or not.
        pub inlining_intra_module: IntraModuleInlining,

        /// The size of "small callees" that can be inlined regardless of the
        /// caller's size.
        pub inlining_small_callee_size: u32,

        /// The general size threshold for the sum of the caller's and callee's
        /// sizes, past which we will generally not inline calls anymore.
        pub inlining_sum_size_threshold: u32,
    }

    pub struct ConfigTunables {
        ...
    }
}

impl Tunables {
    /// Returns a `Tunables` configuration assumed for running code on the host.
    pub fn default_host() -> Self {
        if cfg!(miri) {
            Tunables::default_miri()
        } else if cfg!(target_pointer_width = "32") {
            Tunables::default_u32()
        } else if cfg!(target_pointer_width = "64") {
            Tunables::default_u64()
        } else {
            panic!("unsupported target_pointer_width");
        }
    }

    /// Returns the default set of tunables for the given target triple.
    pub fn default_for_target(target: &Triple) -> Result<Self> {
        if cfg!(miri) {
            return Ok(Tunables::default_miri());
        }
        let mut ret = match target
            .pointer_width()
            .map_err(|_| anyhow!("failed to retrieve target pointer width"))?
        {
            PointerWidth::U32 => Tunables::default_u32(),
            PointerWidth::U64 => Tunables::default_u64(),
            _ => bail!("unsupported target pointer width"),
        };

        // Pulley targets never use signals-based-traps and also can't benefit
        // from guard pages, so disable them.
        if target.is_pulley() {
            ret.signals_based_traps = false;
            ret.memory_guard_size = 0;
        }
        Ok(ret)
    }

    /// Returns the default set of tunables for running under MIRI.
    pub const fn default_miri() -> Tunables {
        Tunables {
            collector: None,

            // No virtual memory tricks are available on miri so make these
            // limits quite conservative.
            memory_reservation: 1 << 20,
            memory_guard_size: 0,
            memory_reservation_for_growth: 0,

            // General options which have the same defaults regardless of
            // architecture.
            generate_native_debuginfo: false,
            parse_wasm_debuginfo: true,
            consume_fuel: false,
            epoch_interruption: false,
            memory_may_move: true,
            guard_before_linear_memory: true,
            table_lazy_init: true,
            generate_address_map: true,
            debug_adapter_modules: false,
            relaxed_simd_deterministic: false,
            winch_callable: false,
            signals_based_traps: false,
            memory_init_cow: true,
            inlining: false,
            inlining_intra_module: IntraModuleInlining::WhenUsingGc,
            inlining_small_callee_size: 50,
            inlining_sum_size_threshold: 2000,
        }
    }

    /// Returns the default set of tunables for running under a 32-bit host.
    pub const fn default_u32() -> Tunables {
        Tunables {
            // For 32-bit we scale way down to 10MB of reserved memory. This
            // impacts performance severely but allows us to have more than a
            // few instances running around.
            memory_reservation: 10 * (1 << 20),
            memory_guard_size: 0x1_0000,
            memory_reservation_for_growth: 1 << 20, // 1MB
            signals_based_traps: true,

            ..Tunables::default_miri()
        }
    }

    /// Returns the default set of tunables for running under a 64-bit host.
    pub const fn default_u64() -> Tunables {
        Tunables {
            // 64-bit has tons of address space to static memories can have 4gb
            // address space reservations liberally by default, allowing us to
            // help eliminate bounds checks.
            //
            // A 32MiB default guard size is then allocated so we can remove
            // explicit bounds checks if any static offset is less than this
            // value. SpiderMonkey found, for example, that in a large corpus of
            // wasm modules 20MiB was the maximum offset so this is the
            // power-of-two-rounded up from that and matches SpiderMonkey.
            memory_reservation: 1 << 32,
            memory_guard_size: 32 << 20,

            // We've got lots of address space on 64-bit so use a larger
            // grow-into-this area, but on 32-bit we aren't as lucky. Miri is
            // not exactly fast so reduce memory consumption instead of trying
            // to avoid memory movement.
            memory_reservation_for_growth: 2 << 30, // 2GB

            signals_based_traps: true,
            ..Tunables::default_miri()
        }
    }

    /// Get the GC heap's memory type, given our configured tunables.
    pub fn gc_heap_memory_type(&self) -> Memory {
        Memory {
            idx_type: IndexType::I32,
            limits: Limits { min: 0, max: None },
            shared: false,
            // We *could* try to match the target architecture's page size, but that
            // would require exercising a page size for memories that we don't
            // otherwise support for Wasm; we conservatively avoid that, and just
            // use the default Wasm page size, for now.
            page_size_log2: 16,
        }
    }
}

/// The garbage collector implementation to use.
#[derive(Clone, Copy, Hash, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Collector {
    /// The deferred reference-counting collector.
    DeferredReferenceCounting,
    /// The null collector.
    Null,
}

impl fmt::Display for Collector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Collector::DeferredReferenceCounting => write!(f, "deferred reference-counting"),
            Collector::Null => write!(f, "null"),
        }
    }
}

/// Whether to inline function calls within the same module.
#[derive(Clone, Copy, Hash, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum IntraModuleInlining {
    Yes,
    No,
    WhenUsingGc,
}

impl FromStr for IntraModuleInlining {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "y" | "yes" | "true" => Ok(Self::Yes),
            "n" | "no" | "false" => Ok(Self::No),
            "gc" => Ok(Self::WhenUsingGc),
            _ => bail!(
                "invalid intra-module inlining option string: `{s}`, \
                 only yes,no,gc accepted"
            ),
        }
    }
}
