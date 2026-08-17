use {
    super::CrashContext,
    crate::{minidump_cpu::RawContextCPU, minidump_format::format::ContextFlagsX86},
    libc::{
        REG_CS, REG_DS, REG_EAX, REG_EBP, REG_EBX, REG_ECX, REG_EDI, REG_EDX, REG_EFL, REG_EIP,
        REG_ES, REG_ESI, REG_ESP, REG_FS, REG_GS, REG_SS, REG_UESP,
    },
};
impl CrashContext {
    pub fn get_instruction_pointer(&self) -> usize {
        self.inner.context.uc_mcontext.gregs[REG_EIP as usize] as usize
    }

    pub fn get_stack_pointer(&self) -> usize {
        self.inner.context.uc_mcontext.gregs[REG_ESP as usize] as usize
    }

    pub fn fill_cpu_context(&self, out: &mut RawContextCPU) {
        out.context_flags = ContextFlagsX86::CONTEXT_X86_FULL.bits()
            | ContextFlagsX86::CONTEXT_X86_FLOATING_POINT.bits();

        {
            let gregs = &self.inner.context.uc_mcontext.gregs;
            out.gs = gregs[REG_GS as usize] as u32;
            out.fs = gregs[REG_FS as usize] as u32;
            out.es = gregs[REG_ES as usize] as u32;
            out.ds = gregs[REG_DS as usize] as u32;

            out.edi = gregs[REG_EDI as usize] as u32;
            out.esi = gregs[REG_ESI as usize] as u32;
            out.ebx = gregs[REG_EBX as usize] as u32;
            out.edx = gregs[REG_EDX as usize] as u32;
            out.ecx = gregs[REG_ECX as usize] as u32;
            out.eax = gregs[REG_EAX as usize] as u32;

            out.ebp = gregs[REG_EBP as usize] as u32;
            out.eip = gregs[REG_EIP as usize] as u32;
            out.cs = gregs[REG_CS as usize] as u32;
            out.eflags = gregs[REG_EFL as usize] as u32;
            out.esp = gregs[REG_UESP as usize] as u32;
            out.ss = gregs[REG_SS as usize] as u32;
        }

        {
            let fs = &self.inner.float_state;
            let out = &mut out.float_save;
            out.control_word = fs.cw;
            out.status_word = fs.sw;
            out.tag_word = fs.tag;
            out.error_offset = fs.ipoff;
            out.error_selector = fs.cssel;
            out.data_offset = fs.dataoff;
            out.data_selector = fs.datasel;

            debug_assert_eq!(fs._st.len() * std::mem::size_of::<libc::_libc_fpreg>(), 80);
            out.register_area.copy_from_slice(unsafe {
                std::slice::from_raw_parts(
                    fs._st.as_ptr().cast(),
                    fs._st.len() * std::mem::size_of::<libc::_libc_fpreg>(),
                )
            });
        }
    }
}
