//! System V ABI unwind information.

use crate::isa::unwind::UnwindInst;
use crate::machinst::Reg;
use crate::result::CodegenResult;
use crate::{CodegenError, binemit::CodeOffset};
use alloc::vec::Vec;
use gimli::write::{Address, FrameDescriptionEntry};

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

type Register = u16;

/// Enumerate the errors possible in mapping Cranelift registers to their DWARF equivalent.
#[expect(missing_docs, reason = "self-describing variants")]
#[derive(Debug, PartialEq, Eq)]
pub enum RegisterMappingError {
    MissingBank,
    UnsupportedArchitecture,
    UnsupportedRegisterBank(&'static str),
}

// This is manually implementing Error and Display instead of using thiserror to reduce the amount
// of dependencies used by Cranelift.
impl std::error::Error for RegisterMappingError {}

impl std::fmt::Display for RegisterMappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RegisterMappingError::MissingBank => write!(f, "unable to find bank for register info"),
            RegisterMappingError::UnsupportedArchitecture => write!(
                f,
                "register mapping is currently only implemented for x86_64"
            ),
            RegisterMappingError::UnsupportedRegisterBank(bank) => {
                write!(f, "unsupported register bank: {bank}")
            }
        }
    }
}

// This mirrors gimli's CallFrameInstruction, but is serializable
// This excludes CfaExpression, Expression, ValExpression due to
// https://github.com/gimli-rs/gimli/issues/513.
// TODO: if gimli ever adds serialization support, remove this type
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub(crate) enum CallFrameInstruction {
    Cfa(Register, i32),
    CfaRegister(Register),
    CfaOffset(i32),
    Restore(Register),
    Undefined(Register),
    SameValue(Register),
    Offset(Register, i32),
    ValOffset(Register, i32),
    Register(Register, Register),
    RememberState,
    RestoreState,
    ArgsSize(u32),
    /// Enables or disables pointer authentication on aarch64 platforms post ARMv8.3.  This
    /// particular item maps to gimli::ValExpression(RA_SIGN_STATE, lit0/lit1).
    Aarch64SetPointerAuth {
        return_addresses: bool,
    },
}

impl From<gimli::write::CallFrameInstruction> for CallFrameInstruction {
    fn from(cfi: gimli::write::CallFrameInstruction) -> Self {
        use gimli::write::CallFrameInstruction;

        match cfi {
            CallFrameInstruction::Cfa(reg, offset) => Self::Cfa(reg.0, offset),
            CallFrameInstruction::CfaRegister(reg) => Self::CfaRegister(reg.0),
            CallFrameInstruction::CfaOffset(offset) => Self::CfaOffset(offset),
            CallFrameInstruction::Restore(reg) => Self::Restore(reg.0),
            CallFrameInstruction::Undefined(reg) => Self::Undefined(reg.0),
            CallFrameInstruction::SameValue(reg) => Self::SameValue(reg.0),
            CallFrameInstruction::Offset(reg, offset) => Self::Offset(reg.0, offset),
            CallFrameInstruction::ValOffset(reg, offset) => Self::ValOffset(reg.0, offset),
            CallFrameInstruction::Register(reg1, reg2) => Self::Register(reg1.0, reg2.0),
            CallFrameInstruction::RememberState => Self::RememberState,
            CallFrameInstruction::RestoreState => Self::RestoreState,
            CallFrameInstruction::ArgsSize(size) => Self::ArgsSize(size),
            _ => {
                // Cranelift's unwind support does not generate `CallFrameInstruction`s with
                // Expression at this moment, and it is not trivial to
                // serialize such instructions.
                panic!("CallFrameInstruction with Expression not supported");
            }
        }
    }
}

impl From<CallFrameInstruction> for gimli::write::CallFrameInstruction {
    fn from(cfi: CallFrameInstruction) -> gimli::write::CallFrameInstruction {
        use CallFrameInstruction as ClifCfi;
        use gimli::{Register, write::CallFrameInstruction as GimliCfi, write::Expression};

        match cfi {
            ClifCfi::Cfa(reg, offset) => GimliCfi::Cfa(Register(reg), offset),
            ClifCfi::CfaRegister(reg) => GimliCfi::CfaRegister(Register(reg)),
            ClifCfi::CfaOffset(offset) => GimliCfi::CfaOffset(offset),
            ClifCfi::Restore(reg) => GimliCfi::Restore(Register(reg)),
            ClifCfi::Undefined(reg) => GimliCfi::Undefined(Register(reg)),
            ClifCfi::SameValue(reg) => GimliCfi::SameValue(Register(reg)),
            ClifCfi::Offset(reg, offset) => GimliCfi::Offset(Register(reg), offset),
            ClifCfi::ValOffset(reg, offset) => GimliCfi::ValOffset(Register(reg), offset),
            ClifCfi::Register(reg1, reg2) => GimliCfi::Register(Register(reg1), Register(reg2)),
            ClifCfi::RememberState => GimliCfi::RememberState,
            ClifCfi::RestoreState => GimliCfi::RestoreState,
            ClifCfi::ArgsSize(size) => GimliCfi::ArgsSize(size),
            ClifCfi::Aarch64SetPointerAuth { return_addresses } => {
                // To enable pointer authentication for return addresses in dwarf directives, we
                // use a small dwarf expression that sets the value of the pseudo-register
                // RA_SIGN_STATE (RA stands for return address) to 0 or 1. This behavior is
                // documented in
                // https://github.com/ARM-software/abi-aa/blob/master/aadwarf64/aadwarf64.rst#41dwarf-register-names.
                let mut expr = Expression::new();
                expr.op(if return_addresses {
                    gimli::DW_OP_lit1
                } else {
                    gimli::DW_OP_lit0
                });
                const RA_SIGN_STATE: Register = Register(34);
                GimliCfi::ValExpression(RA_SIGN_STATE, expr)
            }
        }
    }
}

/// Maps UnwindInfo register to gimli's index space.
pub(crate) trait RegisterMapper<Reg> {
    /// Maps Reg.
    fn map(&self, reg: Reg) -> Result<Register, RegisterMappingError>;
    /// Gets the frame pointer register, if any.
    fn fp(&self) -> Option<Register> {
        None
    }
    /// Gets the link register, if any.
    fn lr(&self) -> Option<Register> {
        None
    }
    /// What is the offset from saved FP to saved LR?
    fn lr_offset(&self) -> Option<u32> {
        None
    }
}

/// Represents unwind information for a single System V ABI function.
///
/// This representation is not ISA specific.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct UnwindInfo {
    instructions: Vec<(u32, CallFrameInstruction)>,
    len: u32,
}

/// Offset from the caller's SP to CFA as we define it.
pub(crate) fn caller_sp_to_cfa_offset() -> u32 {
    // Currently we define them to always be equal.
    0
}

pub(crate) fn create_unwind_info_from_insts<MR: RegisterMapper<Reg>>(
    insts: &[(CodeOffset, UnwindInst)],
    code_len: usize,
    mr: &MR,
) -> CodegenResult<UnwindInfo> {
    let mut instructions = vec![];

    let mut cfa_offset = 0;
    let mut clobber_offset_to_cfa = 0;
    for &(instruction_offset, ref inst) in insts {
        match inst {
            &UnwindInst::PushFrameRegs {
                offset_upward_to_caller_sp,
            } => {
                // Define CFA in terms of current SP (SP changed and we haven't
                // set FP yet).
                instructions.push((
                    instruction_offset,
                    CallFrameInstruction::CfaOffset(offset_upward_to_caller_sp as i32),
                ));
                // Note that we saved the old FP value on the stack.  Use of this
                // operation implies that the target defines a FP register.
                instructions.push((
                    instruction_offset,
                    CallFrameInstruction::Offset(
                        mr.fp().unwrap(),
                        -(offset_upward_to_caller_sp as i32),
                    ),
                ));
                // If there is a link register on this architecture, note that
                // we saved it as well.
                if let Some(lr) = mr.lr() {
                    instructions.push((
                        instruction_offset,
                        CallFrameInstruction::Offset(
                            lr,
                            -(offset_upward_to_caller_sp as i32)
                                + mr.lr_offset().expect("LR offset not provided") as i32,
                        ),
                    ));
                }
            }
            &UnwindInst::DefineNewFrame {
                offset_upward_to_caller_sp,
                offset_downward_to_clobbers,
            } => {
                // Define CFA in terms of FP. Note that we assume it was already
                // defined correctly in terms of the current SP, and FP has just
                // been set to the current SP, so we do not need to change the
                // offset, only the register.  (This is done only if the target
                // defines a frame pointer register.)
                if let Some(fp) = mr.fp() {
                    instructions.push((instruction_offset, CallFrameInstruction::CfaRegister(fp)));
                }
                // Record initial CFA offset.  This will be used with later
                // StackAlloc calls if we do not have a frame pointer.
                cfa_offset = offset_upward_to_caller_sp;
                // Record distance from CFA downward to clobber area so we can
                // express clobber offsets later in terms of CFA.
                clobber_offset_to_cfa = offset_upward_to_caller_sp + offset_downward_to_clobbers;
            }
            &UnwindInst::StackAlloc { size } => {
                // If we do not use a frame pointer, we need to update the
                // CFA offset whenever the stack pointer changes.
                if mr.fp().is_none() {
                    cfa_offset += size;
                    instructions.push((
                        instruction_offset,
                        CallFrameInstruction::CfaOffset(cfa_offset as i32),
                    ));
                }
            }
            &UnwindInst::SaveReg {
                clobber_offset,
                reg,
            } => {
                let reg = mr
                    .map(reg.into())
                    .map_err(|e| CodegenError::RegisterMappingError(e))?;
                let off = (clobber_offset as i32) - (clobber_offset_to_cfa as i32);
                instructions.push((instruction_offset, CallFrameInstruction::Offset(reg, off)));
            }
            &UnwindInst::RegStackOffset {
                clobber_offset,
                reg,
            } => {
                let reg = mr
                    .map(reg.into())
                    .map_err(|e| CodegenError::RegisterMappingError(e))?;
                let off = (clobber_offset as i32) - (clobber_offset_to_cfa as i32);
                instructions.push((
                    instruction_offset,
                    CallFrameInstruction::ValOffset(reg, off),
                ));
            }
            &UnwindInst::Aarch64SetPointerAuth { return_addresses } => {
                instructions.push((
                    instruction_offset,
                    CallFrameInstruction::Aarch64SetPointerAuth { return_addresses },
                ));
            }
        }
    }

    Ok(UnwindInfo {
        instructions,
        len: code_len as u32,
    })
}

impl UnwindInfo {
    /// Converts the unwind information into a `FrameDescriptionEntry`.
    pub fn to_fde(&self, address: Address) -> gimli::write::FrameDescriptionEntry {
        let mut fde = FrameDescriptionEntry::new(address, self.len);

        for (offset, inst) in &self.instructions {
            fde.add_instruction(*offset, inst.clone().into());
        }

        fde
    }
}
