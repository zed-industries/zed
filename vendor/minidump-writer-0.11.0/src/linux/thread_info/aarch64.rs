use {
    super::{CommonThreadInfo, NT_Elf, Pid, ThreadInfoError},
    crate::minidump_cpu::{RawContextCPU, FP_REG_COUNT, GP_REG_COUNT},
    nix::sys::ptrace,
};

/// https://github.com/rust-lang/libc/pull/2719
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct user_fpsimd_struct {
    pub vregs: [u128; 32],
    pub fpsr: u32,
    pub fpcr: u32,
}

type Result<T> = std::result::Result<T, ThreadInfoError>;

#[cfg(target_arch = "aarch64")]
#[derive(Debug)]
pub struct ThreadInfoAarch64 {
    pub stack_pointer: usize,
    pub tgid: Pid, // thread group id
    pub ppid: Pid, // parent process
    pub regs: libc::user_regs_struct,
    pub fpregs: user_fpsimd_struct,
}

impl CommonThreadInfo for ThreadInfoAarch64 {}

impl ThreadInfoAarch64 {
    pub fn get_instruction_pointer(&self) -> usize {
        self.regs.pc as usize
    }

    // nix currently doesn't support PTRACE_GETREGSET, so we have to do it ourselves
    fn getregset(pid: Pid) -> Result<libc::user_regs_struct> {
        Self::ptrace_get_data_via_io(
            0x4204 as ptrace::RequestType, // PTRACE_GETREGSET
            Some(NT_Elf::NT_PRSTATUS),
            nix::unistd::Pid::from_raw(pid),
        )
    }

    fn getregs(pid: Pid) -> Result<libc::user_regs_struct> {
        // TODO: nix restricts PTRACE_GETREGS to arm android for some reason
        Self::ptrace_get_data(
            12 as ptrace::RequestType, // PTRACE_GETREGS
            None,
            nix::unistd::Pid::from_raw(pid),
        )
    }

    // nix currently doesn't support PTRACE_GETREGSET, so we have to do it ourselves
    fn getfpregset(pid: Pid) -> Result<user_fpsimd_struct> {
        Self::ptrace_get_data_via_io(
            0x4204 as ptrace::RequestType, // PTRACE_GETREGSET
            Some(NT_Elf::NT_PRFPREGSET),
            nix::unistd::Pid::from_raw(pid),
        )
    }

    // nix currently doesn't support PTRACE_GETFPREGS, so we have to do it ourselves
    fn getfpregs(pid: Pid) -> Result<user_fpsimd_struct> {
        Self::ptrace_get_data(
            14 as ptrace::RequestType, // PTRACE_GETFPREGS
            None,
            nix::unistd::Pid::from_raw(pid),
        )
    }

    pub fn fill_cpu_context(&self, out: &mut RawContextCPU) {
        out.context_flags =
            minidump_common::format::ContextFlagsArm64Old::CONTEXT_ARM64_OLD_FULL.bits() as u64;

        out.cpsr = self.regs.pstate as u32;
        out.iregs[..GP_REG_COUNT].copy_from_slice(&self.regs.regs[..GP_REG_COUNT]);
        out.sp = self.regs.sp;
        // Note that in breakpad this was the last member of the iregs field
        // which was 33 in length, but in rust-minidump it is its own separate
        // field instead
        out.pc = self.regs.pc;

        out.fpsr = self.fpregs.fpsr;
        out.fpcr = self.fpregs.fpcr;
        out.float_regs[..FP_REG_COUNT].copy_from_slice(&self.fpregs.vregs[..FP_REG_COUNT]);
    }

    pub fn create_impl(_pid: Pid, tid: Pid) -> Result<Self> {
        let (ppid, tgid) = Self::get_ppid_and_tgid(tid)?;
        let regs = Self::getregset(tid).or_else(|_| Self::getregs(tid))?;
        let fpregs = Self::getfpregset(tid).or_else(|_| Self::getfpregs(tid))?;

        let stack_pointer = regs.sp as usize;

        Ok(Self {
            stack_pointer,
            tgid,
            ppid,
            regs,
            fpregs,
        })
    }
}
