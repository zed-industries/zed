use {
    super::CrashContext,
    crate::{
        minidump_cpu::{RawContextCPU, FP_REG_COUNT, GP_REG_COUNT},
        minidump_format::format,
    },
};

impl CrashContext {
    pub fn get_instruction_pointer(&self) -> usize {
        self.inner.context.uc_mcontext.pc as usize
    }

    pub fn get_stack_pointer(&self) -> usize {
        self.inner.context.uc_mcontext.sp as usize
    }

    pub fn fill_cpu_context(&self, out: &mut RawContextCPU) {
        out.context_flags = format::ContextFlagsArm64Old::CONTEXT_ARM64_OLD_FULL.bits() as u64;

        {
            let gregs = &self.inner.context.uc_mcontext;
            out.cpsr = gregs.pstate as u32;
            out.iregs[..GP_REG_COUNT].copy_from_slice(&gregs.regs[..GP_REG_COUNT]);
            out.sp = gregs.sp;
            out.pc = gregs.pc;
        }

        {
            let fs = &self.inner.float_state;
            out.fpsr = fs.fpsr;
            out.fpcr = fs.fpcr;
            out.float_regs[..FP_REG_COUNT].copy_from_slice(&fs.vregs[..FP_REG_COUNT]);
        }
    }
}
