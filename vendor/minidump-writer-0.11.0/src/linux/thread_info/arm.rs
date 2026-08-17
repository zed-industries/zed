use {
    super::{CommonThreadInfo, NT_Elf, Pid, ThreadInfoError},
    crate::minidump_cpu::RawContextCPU,
    nix::sys::ptrace,
};

type Result<T> = std::result::Result<T, ThreadInfoError>;

// Not defined by libc because this works only for cores support VFP
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, Default)]
pub struct user_fpregs_struct {
    pub fpregs: [u64; 32],
    pub fpscr: u32,
}

#[repr(C)]
#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, Default)]
pub struct user_regs_struct {
    uregs: [u32; 18],
}

#[derive(Debug)]
pub struct ThreadInfoArm {
    pub stack_pointer: usize,
    pub tgid: Pid, // thread group id
    pub ppid: Pid, // parent process
    pub regs: user_regs_struct,
    pub fpregs: user_fpregs_struct,
}

impl CommonThreadInfo for ThreadInfoArm {}

impl ThreadInfoArm {
    // nix currently doesn't support PTRACE_GETFPREGS, so we have to do it ourselves
    fn getfpregs(pid: Pid) -> Result<user_fpregs_struct> {
        Self::ptrace_get_data_via_io(
            0x4204 as ptrace::RequestType, // PTRACE_GETREGSET
            Some(NT_Elf::NT_ARM_VFP),
            nix::unistd::Pid::from_raw(pid),
        )
    }

    // nix currently doesn't support PTRACE_GETREGS, so we have to do it ourselves
    fn getregs(pid: Pid) -> Result<user_regs_struct> {
        Self::ptrace_get_data::<user_regs_struct>(
            ptrace::Request::PTRACE_GETREGS as ptrace::RequestType,
            None,
            nix::unistd::Pid::from_raw(pid),
        )
    }

    pub fn get_instruction_pointer(&self) -> usize {
        self.regs.uregs[15] as usize
    }

    pub fn fill_cpu_context(&self, out: &mut RawContextCPU) {
        out.context_flags =
            crate::minidump_format::format::ContextFlagsArm::CONTEXT_ARM_FULL.bits();

        out.iregs.copy_from_slice(&self.regs.uregs[..16]);
        out.cpsr = self.regs.uregs[16];
        out.float_save.fpscr = self.fpregs.fpscr as u64;
        out.float_save.regs = self.fpregs.fpregs;
    }

    pub fn create_impl(_pid: Pid, tid: Pid) -> Result<Self> {
        let (ppid, tgid) = Self::get_ppid_and_tgid(tid)?;
        let regs = Self::getregs(tid)?;
        let fpregs = Self::getfpregs(tid).unwrap_or(Default::default());

        let stack_pointer = regs.uregs[13] as usize;

        Ok(ThreadInfoArm {
            stack_pointer,
            tgid,
            ppid,
            regs,
            fpregs,
        })
    }
}
