use {
    super::{super::thread_info::copy_u32_registers, CrashContext},
    crate::{minidump_cpu::RawContextCPU, minidump_format::format},
    libc::{
        REG_CSGSFS, REG_EFL, REG_R10, REG_R11, REG_R12, REG_R13, REG_R14, REG_R15, REG_R8, REG_R9,
        REG_RAX, REG_RBP, REG_RBX, REG_RCX, REG_RDI, REG_RDX, REG_RIP, REG_RSI, REG_RSP,
    },
    scroll::Pwrite,
};

impl CrashContext {
    pub fn get_instruction_pointer(&self) -> usize {
        self.inner.context.uc_mcontext.gregs[REG_RIP as usize] as usize
    }

    pub fn get_stack_pointer(&self) -> usize {
        self.inner.context.uc_mcontext.gregs[REG_RSP as usize] as usize
    }

    pub fn fill_cpu_context(&self, out: &mut RawContextCPU) {
        out.context_flags = format::ContextFlagsAmd64::CONTEXT_AMD64_FULL.bits();

        {
            let gregs = &self.inner.context.uc_mcontext.gregs;
            out.cs = (gregs[REG_CSGSFS as usize] & 0xffff) as u16;

            out.fs = ((gregs[REG_CSGSFS as usize] >> 32) & 0xffff) as u16;
            out.gs = ((gregs[REG_CSGSFS as usize] >> 16) & 0xffff) as u16;

            out.eflags = gregs[REG_EFL as usize] as u32;

            out.rax = gregs[REG_RAX as usize] as u64;
            out.rcx = gregs[REG_RCX as usize] as u64;
            out.rdx = gregs[REG_RDX as usize] as u64;
            out.rbx = gregs[REG_RBX as usize] as u64;

            out.rsp = gregs[REG_RSP as usize] as u64;
            out.rbp = gregs[REG_RBP as usize] as u64;
            out.rsi = gregs[REG_RSI as usize] as u64;
            out.rdi = gregs[REG_RDI as usize] as u64;
            out.r8 = gregs[REG_R8 as usize] as u64;
            out.r9 = gregs[REG_R9 as usize] as u64;
            out.r10 = gregs[REG_R10 as usize] as u64;
            out.r11 = gregs[REG_R11 as usize] as u64;
            out.r12 = gregs[REG_R12 as usize] as u64;
            out.r13 = gregs[REG_R13 as usize] as u64;
            out.r14 = gregs[REG_R14 as usize] as u64;
            out.r15 = gregs[REG_R15 as usize] as u64;

            out.rip = gregs[REG_RIP as usize] as u64;
        }

        {
            let fs = &self.inner.float_state;

            let mut float_save = format::XMM_SAVE_AREA32 {
                control_word: fs.cwd,
                status_word: fs.swd,
                tag_word: fs.ftw as u8,
                error_opcode: fs.fop,
                error_offset: fs.rip as u32,
                data_offset: fs.rdp as u32,
                error_selector: 0, // We don't have this.
                data_selector: 0,  // We don't have this.
                mx_csr: fs.mxcsr,
                mx_csr_mask: fs.mxcr_mask,
                ..Default::default()
            };

            copy_u32_registers(&mut float_save.float_registers, &fs.st_space);
            copy_u32_registers(&mut float_save.xmm_registers, &fs.xmm_space);

            out.float_save
                .pwrite_with(float_save, 0, scroll::Endian::Little)
                .expect("this is impossible");
        }
    }
}
