//! External function calls.
//!
//! To a Cranelift function, all functions are "external". Directly called functions must be
//! declared in the preamble, and all function calls must have a signature.
//!
//! This module declares the data types used to represent external functions and call signatures.

use crate::ir::{ExternalName, SigRef, Type};
use crate::isa::CallConv;
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

use super::function::FunctionParameters;

/// Function signature.
///
/// The function signature describes the types of formal parameters and return values along with
/// other details that are needed to call a function correctly.
///
/// A signature can optionally include ISA-specific ABI information which specifies exactly how
/// arguments and return values are passed.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Signature {
    /// The arguments passed to the function.
    pub params: Vec<AbiParam>,
    /// Values returned from the function.
    pub returns: Vec<AbiParam>,

    /// Calling convention.
    pub call_conv: CallConv,
}

impl Signature {
    /// Create a new blank signature.
    pub fn new(call_conv: CallConv) -> Self {
        Self {
            params: Vec::new(),
            returns: Vec::new(),
            call_conv,
        }
    }

    /// Clear the signature so it is identical to a fresh one returned by `new()`.
    pub fn clear(&mut self, call_conv: CallConv) {
        self.params.clear();
        self.returns.clear();
        self.call_conv = call_conv;
    }

    /// Find the index of a presumed unique special-purpose parameter.
    pub fn special_param_index(&self, purpose: ArgumentPurpose) -> Option<usize> {
        self.params.iter().rposition(|arg| arg.purpose == purpose)
    }

    /// Find the index of a presumed unique special-purpose parameter.
    pub fn special_return_index(&self, purpose: ArgumentPurpose) -> Option<usize> {
        self.returns.iter().rposition(|arg| arg.purpose == purpose)
    }

    /// Does this signature have a parameter whose `ArgumentPurpose` is
    /// `purpose`?
    pub fn uses_special_param(&self, purpose: ArgumentPurpose) -> bool {
        self.special_param_index(purpose).is_some()
    }

    /// Does this signature have a return whose `ArgumentPurpose` is `purpose`?
    pub fn uses_special_return(&self, purpose: ArgumentPurpose) -> bool {
        self.special_return_index(purpose).is_some()
    }

    /// How many special parameters does this function have?
    pub fn num_special_params(&self) -> usize {
        self.params
            .iter()
            .filter(|p| p.purpose != ArgumentPurpose::Normal)
            .count()
    }

    /// How many special returns does this function have?
    pub fn num_special_returns(&self) -> usize {
        self.returns
            .iter()
            .filter(|r| r.purpose != ArgumentPurpose::Normal)
            .count()
    }

    /// Does this signature take an struct return pointer parameter?
    pub fn uses_struct_return_param(&self) -> bool {
        self.uses_special_param(ArgumentPurpose::StructReturn)
    }

    /// Does this return more than one normal value? (Pre-struct return
    /// legalization)
    pub fn is_multi_return(&self) -> bool {
        self.returns
            .iter()
            .filter(|r| r.purpose == ArgumentPurpose::Normal)
            .count()
            > 1
    }
}

fn write_list(f: &mut fmt::Formatter, args: &[AbiParam]) -> fmt::Result {
    match args.split_first() {
        None => {}
        Some((first, rest)) => {
            write!(f, "{first}")?;
            for arg in rest {
                write!(f, ", {arg}")?;
            }
        }
    }
    Ok(())
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        write_list(f, &self.params)?;
        write!(f, ")")?;
        if !self.returns.is_empty() {
            write!(f, " -> ")?;
            write_list(f, &self.returns)?;
        }
        write!(f, " {}", self.call_conv)
    }
}

/// Function parameter or return value descriptor.
///
/// This describes the value type being passed to or from a function along with flags that affect
/// how the argument is passed.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct AbiParam {
    /// Type of the argument value.
    pub value_type: Type,
    /// Special purpose of argument, or `Normal`.
    pub purpose: ArgumentPurpose,
    /// Method for extending argument to a full register.
    pub extension: ArgumentExtension,
}

impl AbiParam {
    /// Create a parameter with default flags.
    pub fn new(vt: Type) -> Self {
        Self {
            value_type: vt,
            extension: ArgumentExtension::None,
            purpose: ArgumentPurpose::Normal,
        }
    }

    /// Create a special-purpose parameter that is not (yet) bound to a specific register.
    pub fn special(vt: Type, purpose: ArgumentPurpose) -> Self {
        Self {
            value_type: vt,
            extension: ArgumentExtension::None,
            purpose,
        }
    }

    /// Convert `self` to a parameter with the `uext` flag set.
    pub fn uext(self) -> Self {
        debug_assert!(self.value_type.is_int(), "uext on {} arg", self.value_type);
        Self {
            extension: ArgumentExtension::Uext,
            ..self
        }
    }

    /// Convert `self` to a parameter type with the `sext` flag set.
    pub fn sext(self) -> Self {
        debug_assert!(self.value_type.is_int(), "sext on {} arg", self.value_type);
        Self {
            extension: ArgumentExtension::Sext,
            ..self
        }
    }
}

impl fmt::Display for AbiParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value_type)?;
        match self.extension {
            ArgumentExtension::None => {}
            ArgumentExtension::Uext => write!(f, " uext")?,
            ArgumentExtension::Sext => write!(f, " sext")?,
        }
        if self.purpose != ArgumentPurpose::Normal {
            write!(f, " {}", self.purpose)?;
        }
        Ok(())
    }
}

/// Function argument extension options.
///
/// On some architectures, small integer function arguments and/or return values are extended to
/// the width of a general-purpose register.
///
/// This attribute specifies how an argument or return value should be extended *if the platform
/// and ABI require it*. Because the frontend (CLIF generator) does not know anything about the
/// particulars of the target's ABI, and the CLIF should be platform-independent, these attributes
/// specify *how* to extend (according to the signedness of the original program) rather than
/// *whether* to extend.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum ArgumentExtension {
    /// No extension, high bits are indeterminate.
    None,
    /// Unsigned extension: high bits in register are 0.
    Uext,
    /// Signed extension: high bits in register replicate sign bit.
    Sext,
}

/// The special purpose of a function argument.
///
/// Function arguments and return values are used to pass user program values between functions,
/// but they are also used to represent special registers with significance to the ABI such as
/// frame pointers and callee-saved registers.
///
/// The argument purpose is used to indicate any special meaning of an argument or return value.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum ArgumentPurpose {
    /// A normal user program value passed to or from a function.
    Normal,

    /// A C struct passed as argument.
    ///
    /// Note that this should only be used when interacting with code following
    /// a C ABI which is expecting a struct passed *by value*.
    StructArgument(
        /// The size, in bytes, of the struct.
        u32,
    ),

    /// Struct return pointer.
    ///
    /// When a function needs to return more data than will fit in registers, the caller passes a
    /// pointer to a memory location where the return value can be written. In some ABIs, this
    /// struct return pointer is passed in a specific register.
    ///
    /// This argument kind can also appear as a return value for ABIs that require a function with
    /// a `StructReturn` pointer argument to also return that pointer in a register.
    StructReturn,

    /// A VM context pointer.
    ///
    /// This is a pointer to a context struct containing details about the current sandbox. It is
    /// used as a base pointer for `vmctx` global values.
    VMContext,
}

impl fmt::Display for ArgumentPurpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Normal => "normal",
            Self::StructArgument(size) => return write!(f, "sarg({size})"),
            Self::StructReturn => "sret",
            Self::VMContext => "vmctx",
        })
    }
}

impl FromStr for ArgumentPurpose {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "normal" => Ok(Self::Normal),
            "sret" => Ok(Self::StructReturn),
            "vmctx" => Ok(Self::VMContext),
            _ if s.starts_with("sarg(") => {
                if !s.ends_with(")") {
                    return Err(());
                }
                // Parse 'sarg(size)'
                let size: u32 = s["sarg(".len()..s.len() - 1].parse().map_err(|_| ())?;
                Ok(Self::StructArgument(size))
            }
            _ => Err(()),
        }
    }
}

/// An external function.
///
/// Information about a function that can be called directly with a direct `call` instruction.
#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ExtFuncData {
    /// Name of the external function.
    pub name: ExternalName,
    /// Call signature of function.
    pub signature: SigRef,
    /// Will this function be defined nearby, such that it will always be a certain distance away,
    /// after linking? If so, references to it can avoid going through a GOT or PLT. Note that
    /// symbols meant to be preemptible cannot be considered colocated.
    ///
    /// If `true`, some backends may use relocation forms that have limited range. The exact
    /// distance depends on the code model in use. Currently on AArch64, for example, Cranelift
    /// uses a custom code model supporting up to +/- 128MB displacements. If it is unknown how
    /// far away the target will be, it is best not to set the `colocated` flag; in general, this
    /// flag is best used when the target is known to be in the same unit of code generation, such
    /// as a Wasm module.
    ///
    /// See the documentation for `RelocDistance` for more details. A `colocated` flag value of
    /// `true` implies `RelocDistance::Near`.
    pub colocated: bool,
}

impl ExtFuncData {
    /// Returns a displayable version of the `ExtFuncData`, with or without extra context to
    /// prettify the output.
    pub fn display<'a>(
        &'a self,
        params: Option<&'a FunctionParameters>,
    ) -> DisplayableExtFuncData<'a> {
        DisplayableExtFuncData {
            ext_func: self,
            params,
        }
    }
}

/// A displayable `ExtFuncData`, with extra context to prettify the output.
pub struct DisplayableExtFuncData<'a> {
    ext_func: &'a ExtFuncData,
    params: Option<&'a FunctionParameters>,
}

impl<'a> fmt::Display for DisplayableExtFuncData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.ext_func.colocated {
            write!(f, "colocated ")?;
        }
        write!(
            f,
            "{} {}",
            self.ext_func.name.display(self.params),
            self.ext_func.signature
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{F32, I8, I32};
    use alloc::string::ToString;

    #[test]
    fn argument_type() {
        let t = AbiParam::new(I32);
        assert_eq!(t.to_string(), "i32");
        let mut t = t.uext();
        assert_eq!(t.to_string(), "i32 uext");
        assert_eq!(t.sext().to_string(), "i32 sext");
        t.purpose = ArgumentPurpose::StructReturn;
        assert_eq!(t.to_string(), "i32 uext sret");
    }

    #[test]
    fn argument_purpose() {
        let all_purpose = [
            (ArgumentPurpose::Normal, "normal"),
            (ArgumentPurpose::StructReturn, "sret"),
            (ArgumentPurpose::VMContext, "vmctx"),
            (ArgumentPurpose::StructArgument(42), "sarg(42)"),
        ];
        for &(e, n) in &all_purpose {
            assert_eq!(e.to_string(), n);
            assert_eq!(Ok(e), n.parse());
        }
    }

    #[test]
    fn call_conv() {
        for &cc in &[
            CallConv::Fast,
            CallConv::Cold,
            CallConv::SystemV,
            CallConv::WindowsFastcall,
        ] {
            assert_eq!(Ok(cc), cc.to_string().parse())
        }
    }

    #[test]
    fn signatures() {
        let mut sig = Signature::new(CallConv::WindowsFastcall);
        assert_eq!(sig.to_string(), "() windows_fastcall");
        sig.params.push(AbiParam::new(I32));
        assert_eq!(sig.to_string(), "(i32) windows_fastcall");
        sig.returns.push(AbiParam::new(F32));
        assert_eq!(sig.to_string(), "(i32) -> f32 windows_fastcall");
        sig.params.push(AbiParam::new(I32.by(4).unwrap()));
        assert_eq!(sig.to_string(), "(i32, i32x4) -> f32 windows_fastcall");
        sig.returns.push(AbiParam::new(I8));
        assert_eq!(sig.to_string(), "(i32, i32x4) -> f32, i8 windows_fastcall");
    }
}
