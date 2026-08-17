use {super::CrashContext, crate::minidump_cpu::RawContextCPU};

impl CrashContext {
    pub fn get_instruction_pointer(&self) -> usize {
        self.inner.context.uc_mcontext.arm_pc as usize
    }

    pub fn get_stack_pointer(&self) -> usize {
        self.inner.context.uc_mcontext.arm_sp as usize
    }

    pub fn fill_cpu_context(&self, out: &mut RawContextCPU) {
        out.context_flags =
            crate::minidump_format::format::ContextFlagsArm::CONTEXT_ARM_FULL.bits();

        {
            let iregs = &mut out.iregs;
            let gregs = &self.inner.context.uc_mcontext;
            iregs[0] = gregs.arm_r0;
            iregs[1] = gregs.arm_r1;
            iregs[2] = gregs.arm_r2;
            iregs[3] = gregs.arm_r3;
            iregs[4] = gregs.arm_r4;
            iregs[5] = gregs.arm_r5;
            iregs[6] = gregs.arm_r6;
            iregs[7] = gregs.arm_r7;
            iregs[8] = gregs.arm_r8;
            iregs[9] = gregs.arm_r9;
            iregs[10] = gregs.arm_r10;

            iregs[11] = gregs.arm_fp;
            iregs[12] = gregs.arm_ip;
            iregs[13] = gregs.arm_sp;
            iregs[14] = gregs.arm_lr;
            iregs[15] = gregs.arm_pc;

            out.cpsr = gregs.arm_cpsr;
        }

        // TODO: this todo has been in breakpad for years....
        // TODO: fix this after fixing ExceptionHandler
        //out.float_save.fpscr = 0;
        //out.float_save.regs = [0; MD_FLOATINGSAVEAREA_ARM_FPR_COUNT];
        //out.float_save.extra = [0; MD_FLOATINGSAVEAREA_ARM_FPEXTRA_COUNT];
    }
}
