//! Naming well-known routines in the runtime library.

use crate::{
    ir::{AbiParam, ExternalName, FuncRef, Function, Signature, Type, types},
    isa::CallConv,
};
use core::fmt;
use core::str::FromStr;
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// The name of a runtime library routine.
///
/// Runtime library calls are generated for Cranelift IR instructions that don't have an equivalent
/// ISA instruction or an easy macro expansion. A `LibCall` is used as a well-known name to refer to
/// the runtime library routine. This way, Cranelift doesn't have to know about the naming
/// convention in the embedding VM's runtime library.
///
/// This list is likely to grow over time.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum LibCall {
    /// probe for stack overflow. These are emitted for functions which need
    /// when the `enable_probestack` setting is true.
    Probestack,
    /// ceil.f32
    CeilF32,
    /// ceil.f64
    CeilF64,
    /// floor.f32
    FloorF32,
    /// floor.f64
    FloorF64,
    /// trunc.f32
    TruncF32,
    /// trunc.f64
    TruncF64,
    /// nearest.f32
    NearestF32,
    /// nearest.f64
    NearestF64,
    /// fma.f32
    FmaF32,
    /// fma.f64
    FmaF64,
    /// libc.memcpy
    Memcpy,
    /// libc.memset
    Memset,
    /// libc.memmove
    Memmove,
    /// libc.memcmp
    Memcmp,

    /// Elf __tls_get_addr
    ElfTlsGetAddr,
    /// Elf __tls_get_offset
    ElfTlsGetOffset,

    /// The `pshufb` on x86 when SSSE3 isn't available.
    X86Pshufb,
    // When adding a new variant make sure to add it to `all_libcalls` too.
}

impl fmt::Display for LibCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for LibCall {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Probestack" => Ok(Self::Probestack),
            "CeilF32" => Ok(Self::CeilF32),
            "CeilF64" => Ok(Self::CeilF64),
            "FloorF32" => Ok(Self::FloorF32),
            "FloorF64" => Ok(Self::FloorF64),
            "TruncF32" => Ok(Self::TruncF32),
            "TruncF64" => Ok(Self::TruncF64),
            "NearestF32" => Ok(Self::NearestF32),
            "NearestF64" => Ok(Self::NearestF64),
            "FmaF32" => Ok(Self::FmaF32),
            "FmaF64" => Ok(Self::FmaF64),
            "Memcpy" => Ok(Self::Memcpy),
            "Memset" => Ok(Self::Memset),
            "Memmove" => Ok(Self::Memmove),
            "Memcmp" => Ok(Self::Memcmp),

            "ElfTlsGetAddr" => Ok(Self::ElfTlsGetAddr),
            "ElfTlsGetOffset" => Ok(Self::ElfTlsGetOffset),

            "X86Pshufb" => Ok(Self::X86Pshufb),
            _ => Err(()),
        }
    }
}

impl LibCall {
    /// Get a list of all known `LibCall`'s.
    pub fn all_libcalls() -> &'static [LibCall] {
        use LibCall::*;
        &[
            Probestack,
            CeilF32,
            CeilF64,
            FloorF32,
            FloorF64,
            TruncF32,
            TruncF64,
            NearestF32,
            NearestF64,
            FmaF32,
            FmaF64,
            Memcpy,
            Memset,
            Memmove,
            Memcmp,
            ElfTlsGetAddr,
            ElfTlsGetOffset,
            X86Pshufb,
        ]
    }

    /// Get a [Signature] for the function targeted by this [LibCall].
    pub fn signature(&self, call_conv: CallConv, pointer_type: Type) -> Signature {
        use types::*;
        let mut sig = Signature::new(call_conv);

        match self {
            LibCall::CeilF32 | LibCall::FloorF32 | LibCall::TruncF32 | LibCall::NearestF32 => {
                sig.params.push(AbiParam::new(F32));
                sig.returns.push(AbiParam::new(F32));
            }
            LibCall::TruncF64 | LibCall::FloorF64 | LibCall::CeilF64 | LibCall::NearestF64 => {
                sig.params.push(AbiParam::new(F64));
                sig.returns.push(AbiParam::new(F64));
            }
            LibCall::FmaF32 | LibCall::FmaF64 => {
                let ty = if *self == LibCall::FmaF32 { F32 } else { F64 };

                sig.params.push(AbiParam::new(ty));
                sig.params.push(AbiParam::new(ty));
                sig.params.push(AbiParam::new(ty));
                sig.returns.push(AbiParam::new(ty));
            }
            LibCall::Memcpy | LibCall::Memmove => {
                // void* memcpy(void *dest, const void *src, size_t count);
                // void* memmove(void* dest, const void* src, size_t count);
                sig.params.push(AbiParam::new(pointer_type));
                sig.params.push(AbiParam::new(pointer_type));
                sig.params.push(AbiParam::new(pointer_type));
                sig.returns.push(AbiParam::new(pointer_type));
            }
            LibCall::Memset => {
                // void *memset(void *dest, int ch, size_t count);
                sig.params.push(AbiParam::new(pointer_type));
                sig.params.push(AbiParam::new(I32));
                sig.params.push(AbiParam::new(pointer_type));
                sig.returns.push(AbiParam::new(pointer_type));
            }
            LibCall::Memcmp => {
                // void* memcpy(void *dest, const void *src, size_t count);
                sig.params.push(AbiParam::new(pointer_type));
                sig.params.push(AbiParam::new(pointer_type));
                sig.params.push(AbiParam::new(pointer_type));
                sig.returns.push(AbiParam::new(I32))
            }

            LibCall::Probestack | LibCall::ElfTlsGetAddr | LibCall::ElfTlsGetOffset => {
                unimplemented!()
            }
            LibCall::X86Pshufb => {
                sig.params.push(AbiParam::new(I8X16));
                sig.params.push(AbiParam::new(I8X16));
                sig.returns.push(AbiParam::new(I8X16));
            }
        }

        sig
    }
}

/// Get a function reference for the probestack function in `func`.
///
/// If there is an existing reference, use it, otherwise make a new one.
pub fn get_probestack_funcref(func: &mut Function) -> Option<FuncRef> {
    find_funcref(LibCall::Probestack, func)
}

/// Get the existing function reference for `libcall` in `func` if it exists.
fn find_funcref(libcall: LibCall, func: &Function) -> Option<FuncRef> {
    // We're assuming that all libcall function decls are at the end.
    // If we get this wrong, worst case we'll have duplicate libcall decls which is harmless.
    for (fref, func_data) in func.dfg.ext_funcs.iter().rev() {
        match func_data.name {
            ExternalName::LibCall(lc) => {
                if lc == libcall {
                    return Some(fref);
                }
            }
            _ => break,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn display() {
        assert_eq!(LibCall::CeilF32.to_string(), "CeilF32");
        assert_eq!(LibCall::NearestF64.to_string(), "NearestF64");
    }

    #[test]
    fn parsing() {
        assert_eq!("FloorF32".parse(), Ok(LibCall::FloorF32));
    }

    #[test]
    fn all_libcalls_to_from_string() {
        for &libcall in LibCall::all_libcalls() {
            assert_eq!(libcall.to_string().parse(), Ok(libcall));
        }
    }
}
