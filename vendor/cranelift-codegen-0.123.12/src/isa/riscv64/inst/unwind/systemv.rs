//! Unwind information for System V ABI (Riscv64).

use crate::isa::riscv64::inst::regs;
use crate::isa::unwind::systemv::RegisterMappingError;
use crate::machinst::Reg;
use gimli::{Encoding, Format, Register, write::CommonInformationEntry};
use regalloc2::RegClass;

/// Creates a new riscv64 common information entry (CIE).
pub fn create_cie() -> CommonInformationEntry {
    use gimli::write::CallFrameInstruction;

    let mut entry = CommonInformationEntry::new(
        Encoding {
            address_size: 8,
            format: Format::Dwarf32,
            version: 1,
        },
        2,  // Code alignment factor
        -8, // Data alignment factor
        Register(regs::link_reg().to_real_reg().unwrap().hw_enc() as u16),
    );

    // Every frame will start with the call frame address (CFA) at SP
    let sp = Register(regs::stack_reg().to_real_reg().unwrap().hw_enc().into());
    entry.add_instruction(CallFrameInstruction::Cfa(sp, 0));

    entry
}

/// Map Cranelift registers to their corresponding Gimli registers.
pub fn map_reg(reg: Reg) -> Result<Register, RegisterMappingError> {
    let reg_offset = match reg.class() {
        RegClass::Int => 0,
        RegClass::Float => 32,
        RegClass::Vector => 64,
    };

    let reg = reg.to_real_reg().unwrap().hw_enc() as u16;
    Ok(Register(reg_offset + reg))
}

pub(crate) struct RegisterMapper;

impl crate::isa::unwind::systemv::RegisterMapper<Reg> for RegisterMapper {
    fn map(&self, reg: Reg) -> Result<u16, RegisterMappingError> {
        Ok(map_reg(reg)?.0)
    }
    fn fp(&self) -> Option<u16> {
        Some(regs::fp_reg().to_real_reg().unwrap().hw_enc() as u16)
    }
    fn lr(&self) -> Option<u16> {
        Some(regs::link_reg().to_real_reg().unwrap().hw_enc() as u16)
    }
    fn lr_offset(&self) -> Option<u32> {
        Some(8)
    }
}

#[cfg(test)]
mod tests {
    use crate::cursor::{Cursor, FuncCursor};

    use crate::Context;
    use crate::ir::{
        AbiParam, Function, InstBuilder, Signature, StackSlotData, StackSlotKind, UserFuncName,
        types,
    };
    use crate::isa::{CallConv, lookup};
    use crate::settings::{Flags, builder};
    use gimli::write::Address;
    use target_lexicon::triple;

    #[test]
    fn test_simple_func() {
        let isa = lookup(triple!("riscv64"))
            .expect("expect riscv64 ISA")
            .finish(Flags::new(builder()))
            .expect("Creating compiler backend");

        let mut context = Context::for_function(create_function(
            CallConv::SystemV,
            Some(StackSlotData::new(StackSlotKind::ExplicitSlot, 64, 0)),
        ));

        let code = context
            .compile(&*isa, &mut Default::default())
            .expect("expected compilation");

        let fde = match code
            .create_unwind_info(isa.as_ref())
            .expect("can create unwind info")
        {
            Some(crate::isa::unwind::UnwindInfo::SystemV(info)) => {
                info.to_fde(Address::Constant(1234))
            }
            _ => panic!("expected unwind information"),
        };

        assert_eq!(
            format!("{fde:?}"),
            "FrameDescriptionEntry { address: Constant(1234), length: 40, lsda: None, instructions: [(12, CfaOffset(16)), (12, Offset(Register(8), -16)), (12, Offset(Register(1), -8)), (16, CfaRegister(Register(8)))] }"
        );
    }

    fn create_function(call_conv: CallConv, stack_slot: Option<StackSlotData>) -> Function {
        let mut func =
            Function::with_name_signature(UserFuncName::user(0, 0), Signature::new(call_conv));

        let block0 = func.dfg.make_block();
        let mut pos = FuncCursor::new(&mut func);
        pos.insert_block(block0);
        pos.ins().return_(&[]);

        if let Some(stack_slot) = stack_slot {
            func.sized_stack_slots.push(stack_slot);
        }

        func
    }

    #[test]
    fn test_multi_return_func() {
        let isa = lookup(triple!("riscv64"))
            .expect("expect riscv64 ISA")
            .finish(Flags::new(builder()))
            .expect("Creating compiler backend");

        let mut context = Context::for_function(create_multi_return_function(CallConv::SystemV));

        let code = context
            .compile(&*isa, &mut Default::default())
            .expect("expected compilation");

        let fde = match code
            .create_unwind_info(isa.as_ref())
            .expect("can create unwind info")
        {
            Some(crate::isa::unwind::UnwindInfo::SystemV(info)) => {
                info.to_fde(Address::Constant(4321))
            }
            _ => panic!("expected unwind information"),
        };

        assert_eq!(
            format!("{fde:?}"),
            "FrameDescriptionEntry { address: Constant(4321), length: 16, lsda: None, instructions: [] }"
        );
    }

    fn create_multi_return_function(call_conv: CallConv) -> Function {
        let mut sig = Signature::new(call_conv);
        sig.params.push(AbiParam::new(types::I32));
        let mut func = Function::with_name_signature(UserFuncName::user(0, 0), sig);

        let block0 = func.dfg.make_block();
        let v0 = func.dfg.append_block_param(block0, types::I32);
        let block1 = func.dfg.make_block();
        let block2 = func.dfg.make_block();

        let mut pos = FuncCursor::new(&mut func);
        pos.insert_block(block0);
        pos.ins().brif(v0, block2, &[], block1, &[]);

        pos.insert_block(block1);
        pos.ins().return_(&[]);

        pos.insert_block(block2);
        pos.ins().return_(&[]);

        func
    }
}
