//! Unwind information for System V ABI (s390x).

use crate::isa::unwind::systemv::RegisterMappingError;
use crate::machinst::{Reg, RegClass};
use gimli::{Encoding, Format, Register, write::CommonInformationEntry};

/// Creates a new s390x common information entry (CIE).
pub fn create_cie() -> CommonInformationEntry {
    use gimli::write::CallFrameInstruction;

    let mut entry = CommonInformationEntry::new(
        Encoding {
            address_size: 8,
            format: Format::Dwarf32,
            version: 1,
        },
        1,            // Code alignment factor
        -8,           // Data alignment factor
        Register(14), // Return address column - register %r14
    );

    // Every frame will start with the call frame address (CFA) at %r15 + 160.
    entry.add_instruction(CallFrameInstruction::Cfa(Register(15), 160));

    entry
}

/// Map Cranelift registers to their corresponding Gimli registers.
pub fn map_reg(reg: Reg) -> Result<Register, RegisterMappingError> {
    const GPR_MAP: [gimli::Register; 16] = [
        Register(0),
        Register(1),
        Register(2),
        Register(3),
        Register(4),
        Register(5),
        Register(6),
        Register(7),
        Register(8),
        Register(9),
        Register(10),
        Register(11),
        Register(12),
        Register(13),
        Register(14),
        Register(15),
    ];
    const VR_MAP: [gimli::Register; 32] = [
        Register(16),
        Register(20),
        Register(17),
        Register(21),
        Register(18),
        Register(22),
        Register(19),
        Register(23),
        Register(24),
        Register(28),
        Register(25),
        Register(29),
        Register(26),
        Register(30),
        Register(27),
        Register(31),
        Register(68),
        Register(72),
        Register(69),
        Register(73),
        Register(70),
        Register(74),
        Register(71),
        Register(75),
        Register(76),
        Register(80),
        Register(77),
        Register(81),
        Register(78),
        Register(82),
        Register(79),
        Register(83),
    ];

    match reg.class() {
        RegClass::Int => Ok(GPR_MAP[reg.to_real_reg().unwrap().hw_enc() as usize]),
        RegClass::Float => Ok(VR_MAP[reg.to_real_reg().unwrap().hw_enc() as usize]),
        RegClass::Vector => unreachable!(),
    }
}

pub(crate) struct RegisterMapper;

impl crate::isa::unwind::systemv::RegisterMapper<Reg> for RegisterMapper {
    fn map(&self, reg: Reg) -> Result<u16, RegisterMappingError> {
        Ok(map_reg(reg)?.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Context;
    use crate::cursor::{Cursor, FuncCursor};
    use crate::ir::{
        AbiParam, Function, InstBuilder, Signature, StackSlotData, StackSlotKind, types,
    };
    use crate::isa::{CallConv, lookup};
    use crate::settings::{Flags, builder};
    use gimli::write::Address;
    use target_lexicon::triple;

    #[test]
    fn test_simple_func() {
        let isa = lookup(triple!("s390x"))
            .expect("expect s390x ISA")
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
            "FrameDescriptionEntry { address: Constant(1234), length: 10, lsda: None, instructions: [(4, CfaOffset(224))] }"
        );
    }

    fn create_function(call_conv: CallConv, stack_slot: Option<StackSlotData>) -> Function {
        let mut func = Function::with_name_signature(Default::default(), Signature::new(call_conv));

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
        let isa = lookup(triple!("s390x"))
            .expect("expect s390x ISA")
            .finish(Flags::new(builder()))
            .expect("Creating compiler backend");

        let mut context = Context::for_function(create_multi_return_function(
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
                info.to_fde(Address::Constant(4321))
            }
            _ => panic!("expected unwind information"),
        };

        assert_eq!(
            format!("{fde:?}"),
            "FrameDescriptionEntry { address: Constant(4321), length: 26, lsda: None, instructions: [(4, CfaOffset(224))] }"
        );
    }

    fn create_multi_return_function(
        call_conv: CallConv,
        stack_slot: Option<StackSlotData>,
    ) -> Function {
        let mut sig = Signature::new(call_conv);
        sig.params.push(AbiParam::new(types::I32));
        let mut func = Function::with_name_signature(Default::default(), sig);

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

        if let Some(stack_slot) = stack_slot {
            func.sized_stack_slots.push(stack_slot);
        }

        func
    }
}
