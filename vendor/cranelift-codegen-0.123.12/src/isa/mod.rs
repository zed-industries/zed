//! Instruction Set Architectures.
//!
//! The `isa` module provides a `TargetIsa` trait which provides the behavior specialization needed
//! by the ISA-independent code generator. The sub-modules of this module provide definitions for
//! the instruction sets that Cranelift can target. Each sub-module has it's own implementation of
//! `TargetIsa`.
//!
//! # Constructing a `TargetIsa` instance
//!
//! The target ISA is built from the following information:
//!
//! - The name of the target ISA as a string. Cranelift is a cross-compiler, so the ISA to target
//!   can be selected dynamically. Individual ISAs can be left out when Cranelift is compiled, so a
//!   string is used to identify the proper sub-module.
//! - Values for settings that apply to all ISAs. This is represented by a `settings::Flags`
//!   instance.
//! - Values for ISA-specific settings.
//!
//! The `isa::lookup()` function is the main entry point which returns an `isa::Builder`
//! appropriate for the requested ISA:
//!
//! ```
//! # #[macro_use] extern crate target_lexicon;
//! use cranelift_codegen::isa;
//! use cranelift_codegen::settings::{self, Configurable};
//! use std::str::FromStr;
//! use target_lexicon::Triple;
//!
//! let shared_builder = settings::builder();
//! let shared_flags = settings::Flags::new(shared_builder);
//!
//! match isa::lookup(triple!("x86_64")) {
//!     Err(_) => {
//!         // The x86_64 target ISA is not available.
//!     }
//!     Ok(mut isa_builder) => {
//!         isa_builder.set("use_popcnt", "on");
//!         let isa = isa_builder.finish(shared_flags);
//!     }
//! }
//! ```
//!
//! The configured target ISA trait object is a `Box<TargetIsa>` which can be used for multiple
//! concurrent function compilations.

use crate::dominator_tree::DominatorTree;
pub use crate::isa::call_conv::CallConv;

use crate::CodegenResult;
use crate::ir::{self, Function, Type};
#[cfg(feature = "unwind")]
use crate::isa::unwind::{UnwindInfoKind, systemv::RegisterMappingError};
use crate::machinst::{CompiledCode, CompiledCodeStencil, TextSectionBuilder};
use crate::settings;
use crate::settings::Configurable;
use crate::settings::SetResult;
use crate::{Reg, flowgraph};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::fmt;
use core::fmt::{Debug, Formatter};
use cranelift_control::ControlPlane;
use std::string::String;
use target_lexicon::{Architecture, PointerWidth, Triple, triple};

// This module is made public here for benchmarking purposes. No guarantees are
// made regarding API stability.
#[cfg(feature = "x86")]
pub mod x64;

#[cfg(feature = "arm64")]
pub mod aarch64;

#[cfg(feature = "riscv64")]
pub mod riscv64;

#[cfg(feature = "s390x")]
mod s390x;

#[cfg(feature = "pulley")]
mod pulley32;
#[cfg(feature = "pulley")]
mod pulley64;
#[cfg(feature = "pulley")]
mod pulley_shared;

pub mod unwind;

mod call_conv;
mod winch;

/// Returns a builder that can create a corresponding `TargetIsa`
/// or `Err(LookupError::SupportDisabled)` if not enabled.
macro_rules! isa_builder {
    ($name: ident, $cfg_terms: tt, $triple: ident) => {{
        #[cfg $cfg_terms]
        {
            Ok($name::isa_builder($triple))
        }
        #[cfg(not $cfg_terms)]
        {
            Err(LookupError::SupportDisabled)
        }
    }};
}

/// Look for an ISA for the given `triple`.
/// Return a builder that can create a corresponding `TargetIsa`.
pub fn lookup(triple: Triple) -> Result<Builder, LookupError> {
    match triple.architecture {
        Architecture::X86_64 => {
            isa_builder!(x64, (feature = "x86"), triple)
        }
        Architecture::Aarch64 { .. } => isa_builder!(aarch64, (feature = "arm64"), triple),
        Architecture::S390x { .. } => isa_builder!(s390x, (feature = "s390x"), triple),
        Architecture::Riscv64 { .. } => isa_builder!(riscv64, (feature = "riscv64"), triple),
        Architecture::Pulley32 | Architecture::Pulley32be => {
            isa_builder!(pulley32, (feature = "pulley"), triple)
        }
        Architecture::Pulley64 | Architecture::Pulley64be => {
            isa_builder!(pulley64, (feature = "pulley"), triple)
        }
        _ => Err(LookupError::Unsupported),
    }
}

/// The string names of all the supported, but possibly not enabled, architectures. The elements of
/// this slice are suitable to be passed to the [lookup_by_name] function to obtain the default
/// configuration for that architecture.
pub const ALL_ARCHITECTURES: &[&str] = &["x86_64", "aarch64", "s390x", "riscv64"];

/// Look for a supported ISA with the given `name`.
/// Return a builder that can create a corresponding `TargetIsa`.
pub fn lookup_by_name(name: &str) -> Result<Builder, LookupError> {
    lookup(triple!(name))
}

/// Describes reason for target lookup failure
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LookupError {
    /// Support for this target was disabled in the current build.
    SupportDisabled,

    /// Support for this target has not yet been implemented.
    Unsupported,
}

// This is manually implementing Error and Display instead of using thiserror to reduce the amount
// of dependencies used by Cranelift.
impl std::error::Error for LookupError {}

impl fmt::Display for LookupError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LookupError::SupportDisabled => write!(f, "Support for this target is disabled"),
            LookupError::Unsupported => {
                write!(f, "Support for this target has not been implemented yet")
            }
        }
    }
}

/// The type of a polymorphic TargetISA object which is 'static.
pub type OwnedTargetIsa = Arc<dyn TargetIsa>;

/// Type alias of `IsaBuilder` used for building Cranelift's ISAs.
pub type Builder = IsaBuilder<CodegenResult<OwnedTargetIsa>>;

/// Builder for a `TargetIsa`.
/// Modify the ISA-specific settings before creating the `TargetIsa` trait object with `finish`.
#[derive(Clone)]
pub struct IsaBuilder<T> {
    triple: Triple,
    setup: settings::Builder,
    constructor: fn(Triple, settings::Flags, &settings::Builder) -> T,
}

impl<T> IsaBuilder<T> {
    /// Creates a new ISA-builder from its components, namely the `triple` for
    /// the ISA, the ISA-specific settings builder, and a final constructor
    /// function to generate the ISA from its components.
    pub fn new(
        triple: Triple,
        setup: settings::Builder,
        constructor: fn(Triple, settings::Flags, &settings::Builder) -> T,
    ) -> Self {
        IsaBuilder {
            triple,
            setup,
            constructor,
        }
    }

    /// Creates a new [Builder] from a [TargetIsa], copying all flags in the
    /// process.
    pub fn from_target_isa(target_isa: &dyn TargetIsa) -> Builder {
        // We should always be able to find the builder for the TargetISA, since presumably we
        // also generated the previous TargetISA at some point
        let triple = target_isa.triple().clone();
        let mut builder = self::lookup(triple).expect("Could not find triple for target ISA");

        // Copy ISA Flags
        for flag in target_isa.isa_flags() {
            builder.set(&flag.name, &flag.value_string()).unwrap();
        }

        builder
    }

    /// Gets the triple for the builder.
    pub fn triple(&self) -> &Triple {
        &self.triple
    }

    /// Iterates the available settings in the builder.
    pub fn iter(&self) -> impl Iterator<Item = settings::Setting> + use<T> {
        self.setup.iter()
    }

    /// Combine the ISA-specific settings with the provided
    /// ISA-independent settings and allocate a fully configured
    /// `TargetIsa` trait object. May return an error if some of the
    /// flags are inconsistent or incompatible: for example, some
    /// platform-independent features, like general SIMD support, may
    /// need certain ISA extensions to be enabled.
    pub fn finish(&self, shared_flags: settings::Flags) -> T {
        (self.constructor)(self.triple.clone(), shared_flags, &self.setup)
    }
}

impl<T> settings::Configurable for IsaBuilder<T> {
    fn set(&mut self, name: &str, value: &str) -> SetResult<()> {
        self.setup.set(name, value)
    }

    fn enable(&mut self, name: &str) -> SetResult<()> {
        self.setup.enable(name)
    }
}

/// After determining that an instruction doesn't have an encoding, how should we proceed to
/// legalize it?
///
/// The `Encodings` iterator returns a legalization function to call.
pub type Legalize =
    fn(ir::Inst, &mut ir::Function, &mut flowgraph::ControlFlowGraph, &dyn TargetIsa) -> bool;

/// This struct provides information that a frontend may need to know about a target to
/// produce Cranelift IR for the target.
#[derive(Clone, Copy, Hash)]
pub struct TargetFrontendConfig {
    /// The default calling convention of the target.
    pub default_call_conv: CallConv,

    /// The pointer width of the target.
    pub pointer_width: PointerWidth,

    /// The log2 of the target's page size and alignment.
    ///
    /// Note that this may be an upper-bound that is larger than necessary for
    /// some platforms since it may depend on runtime configuration.
    pub page_size_align_log2: u8,
}

impl TargetFrontendConfig {
    /// Get the pointer type of this target.
    pub fn pointer_type(self) -> ir::Type {
        ir::Type::int(self.pointer_bits() as u16).unwrap()
    }

    /// Get the width of pointers on this target, in units of bits.
    pub fn pointer_bits(self) -> u8 {
        self.pointer_width.bits()
    }

    /// Get the width of pointers on this target, in units of bytes.
    pub fn pointer_bytes(self) -> u8 {
        self.pointer_width.bytes()
    }
}

/// Methods that are specialized to a target ISA.
///
/// Implies a Display trait that shows the shared flags, as well as any ISA-specific flags.
pub trait TargetIsa: fmt::Display + Send + Sync {
    /// Get the name of this ISA.
    fn name(&self) -> &'static str;

    /// Get the target triple that was used to make this trait object.
    fn triple(&self) -> &Triple;

    /// Get the ISA-independent flags that were used to make this trait object.
    fn flags(&self) -> &settings::Flags;

    /// Get the ISA-dependent flag values that were used to make this trait object.
    fn isa_flags(&self) -> Vec<settings::Value>;

    /// Get the ISA-dependent flag values as raw bytes for hashing.
    fn isa_flags_hash_key(&self) -> IsaFlagsHashKey<'_>;

    /// Get a flag indicating whether branch protection is enabled.
    fn is_branch_protection_enabled(&self) -> bool {
        false
    }

    /// Get the ISA-dependent maximum vector register size, in bytes.
    fn dynamic_vector_bytes(&self, dynamic_ty: ir::Type) -> u32;

    /// Compile the given function.
    fn compile_function(
        &self,
        func: &Function,
        domtree: &DominatorTree,
        want_disasm: bool,
        ctrl_plane: &mut ControlPlane,
    ) -> CodegenResult<CompiledCodeStencil>;

    #[cfg(feature = "unwind")]
    /// Map a regalloc::Reg to its corresponding DWARF register.
    fn map_regalloc_reg_to_dwarf(
        &self,
        _: crate::machinst::Reg,
    ) -> Result<u16, RegisterMappingError> {
        Err(RegisterMappingError::UnsupportedArchitecture)
    }

    /// Creates unwind information for the function.
    ///
    /// Returns `None` if there is no unwind information for the function.
    #[cfg(feature = "unwind")]
    fn emit_unwind_info(
        &self,
        result: &CompiledCode,
        kind: UnwindInfoKind,
    ) -> CodegenResult<Option<crate::isa::unwind::UnwindInfo>>;

    /// Creates a new System V Common Information Entry for the ISA.
    ///
    /// Returns `None` if the ISA does not support System V unwind information.
    #[cfg(feature = "unwind")]
    fn create_systemv_cie(&self) -> Option<gimli::write::CommonInformationEntry> {
        // By default, an ISA cannot create a System V CIE
        None
    }

    /// Returns an object that can be used to build the text section of an
    /// executable.
    ///
    /// This object will internally attempt to handle as many relocations as
    /// possible using relative calls/jumps/etc between functions.
    ///
    /// The `num_labeled_funcs` argument here is the number of functions which
    /// will be "labeled" or might have calls between them, typically the number
    /// of defined functions in the object file.
    fn text_section_builder(&self, num_labeled_funcs: usize) -> Box<dyn TextSectionBuilder>;

    /// Returns the minimum function alignment and the preferred function
    /// alignment, for performance, required by this ISA.
    fn function_alignment(&self) -> FunctionAlignment;

    /// The log2 of the target's page size and alignment.
    ///
    /// Note that this may be an upper-bound that is larger than necessary for
    /// some platforms since it may depend on runtime configuration.
    fn page_size_align_log2(&self) -> u8;

    /// Create a polymorphic TargetIsa from this specific implementation.
    fn wrapped(self) -> OwnedTargetIsa
    where
        Self: Sized + 'static,
    {
        Arc::new(self)
    }

    /// Generate a `Capstone` context for disassembling bytecode for this architecture.
    #[cfg(feature = "disas")]
    fn to_capstone(&self) -> Result<capstone::Capstone, capstone::Error> {
        Err(capstone::Error::UnsupportedArch)
    }

    /// Return the string representation of "reg" accessed as "size" bytes.
    /// The returned string will match the usual disassemly view of "reg".
    fn pretty_print_reg(&self, reg: Reg, size: u8) -> String;

    /// Returns whether this ISA has a native fused-multiply-and-add instruction
    /// for floats.
    ///
    /// Currently this only returns false on x86 when some native features are
    /// not detected.
    fn has_native_fma(&self) -> bool;

    /// Returns whether this ISA has instructions for `ceil`, `floor`, etc.
    fn has_round(&self) -> bool;

    /// Returns whether the CLIF `x86_blendv` instruction is implemented for
    /// this ISA for the specified type.
    fn has_x86_blendv_lowering(&self, ty: Type) -> bool;

    /// Returns whether the CLIF `x86_pshufb` instruction is implemented for
    /// this ISA.
    fn has_x86_pshufb_lowering(&self) -> bool;

    /// Returns whether the CLIF `x86_pmulhrsw` instruction is implemented for
    /// this ISA.
    fn has_x86_pmulhrsw_lowering(&self) -> bool;

    /// Returns whether the CLIF `x86_pmaddubsw` instruction is implemented for
    /// this ISA.
    fn has_x86_pmaddubsw_lowering(&self) -> bool;

    /// Returns the mode of extension used for integer arguments smaller than
    /// the pointer width in function signatures.
    ///
    /// Some platform ABIs require that smaller-than-pointer-width values are
    /// either zero or sign-extended to the full register width. This value is
    /// propagated to the `AbiParam` value created for signatures. Note that not
    /// all ABIs for all platforms require extension of any form, so this is
    /// generally only necessary for the `default_call_conv`.
    fn default_argument_extension(&self) -> ir::ArgumentExtension;
}

/// A wrapper around the ISA-dependent flags types which only implements `Hash`.
#[derive(Hash)]
pub struct IsaFlagsHashKey<'a>(&'a [u8]);

/// Function alignment specifications as required by an ISA, returned by
/// [`TargetIsa::function_alignment`].
#[derive(Copy, Clone)]
pub struct FunctionAlignment {
    /// The minimum alignment required by an ISA, where all functions must be
    /// aligned to at least this amount.
    pub minimum: u32,
    /// A "preferred" alignment which should be used for more
    /// performance-sensitive situations. This can involve cache-line-aligning
    /// for example to get more of a small function into fewer cache lines.
    pub preferred: u32,
}

/// Methods implemented for free for target ISA!
impl<'a> dyn TargetIsa + 'a {
    /// Get the default calling convention of this target.
    pub fn default_call_conv(&self) -> CallConv {
        CallConv::triple_default(self.triple())
    }

    /// Get the endianness of this ISA.
    pub fn endianness(&self) -> ir::Endianness {
        match self.triple().endianness().unwrap() {
            target_lexicon::Endianness::Little => ir::Endianness::Little,
            target_lexicon::Endianness::Big => ir::Endianness::Big,
        }
    }

    /// Returns the minimum symbol alignment for this ISA.
    pub fn symbol_alignment(&self) -> u64 {
        match self.triple().architecture {
            // All symbols need to be aligned to at least 2 on s390x.
            Architecture::S390x => 2,
            _ => 1,
        }
    }

    /// Get the pointer type of this ISA.
    pub fn pointer_type(&self) -> ir::Type {
        ir::Type::int(self.pointer_bits() as u16).unwrap()
    }

    /// Get the width of pointers on this ISA.
    pub(crate) fn pointer_width(&self) -> PointerWidth {
        self.triple().pointer_width().unwrap()
    }

    /// Get the width of pointers on this ISA, in units of bits.
    pub fn pointer_bits(&self) -> u8 {
        self.pointer_width().bits()
    }

    /// Get the width of pointers on this ISA, in units of bytes.
    pub fn pointer_bytes(&self) -> u8 {
        self.pointer_width().bytes()
    }

    /// Get the information needed by frontends producing Cranelift IR.
    pub fn frontend_config(&self) -> TargetFrontendConfig {
        TargetFrontendConfig {
            default_call_conv: self.default_call_conv(),
            pointer_width: self.pointer_width(),
            page_size_align_log2: self.page_size_align_log2(),
        }
    }
}

impl Debug for &dyn TargetIsa {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TargetIsa {{ triple: {:?}, pointer_width: {:?}}}",
            self.triple(),
            self.pointer_width()
        )
    }
}
