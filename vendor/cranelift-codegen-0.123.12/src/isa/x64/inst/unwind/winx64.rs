//! Unwind information for Windows x64 ABI.

use crate::machinst::{Reg, RegClass};

pub(crate) struct RegisterMapper;

impl crate::isa::unwind::winx64::RegisterMapper<Reg> for RegisterMapper {
    fn map(reg: Reg) -> crate::isa::unwind::winx64::MappedRegister {
        use crate::isa::unwind::winx64::MappedRegister;
        match reg.class() {
            RegClass::Int => MappedRegister::Int(reg.to_real_reg().unwrap().hw_enc()),
            RegClass::Float => MappedRegister::Xmm(reg.to_real_reg().unwrap().hw_enc()),
            RegClass::Vector => unreachable!(),
        }
    }
}
