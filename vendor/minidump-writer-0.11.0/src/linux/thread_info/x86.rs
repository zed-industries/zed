use {
    super::{CommonThreadInfo, NT_Elf, Pid, ThreadInfoError},
    crate::{minidump_cpu::RawContextCPU, minidump_format::format},
    core::mem::size_of_val,
    nix::sys::ptrace,
    scroll::Pwrite,
};

#[cfg(all(not(target_os = "android"), target_arch = "x86"))]
use libc::user_fpxregs_struct;
#[cfg(not(all(target_os = "android", target_arch = "x86")))]
use libc::{user, user_fpregs_struct, user_regs_struct};

type Result<T> = std::result::Result<T, ThreadInfoError>;

// Not defined by libc on Android
#[cfg(all(target_os = "android", target_arch = "x86"))]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct user_regs_struct {
    pub ebx: libc::c_long,
    pub ecx: libc::c_long,
    pub edx: libc::c_long,
    pub esi: libc::c_long,
    pub edi: libc::c_long,
    pub ebp: libc::c_long,
    pub eax: libc::c_long,
    pub xds: libc::c_long,
    pub xes: libc::c_long,
    pub xfs: libc::c_long,
    pub xgs: libc::c_long,
    pub orig_eax: libc::c_long,
    pub eip: libc::c_long,
    pub xcs: libc::c_long,
    pub eflags: libc::c_long,
    pub esp: libc::c_long,
    pub xss: libc::c_long,
}

// Not defined by libc on Android
#[cfg(all(target_os = "android", target_arch = "x86"))]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct user_fpxregs_struct {
    pub cwd: libc::c_ushort,
    pub swd: libc::c_ushort,
    pub twd: libc::c_ushort,
    pub fop: libc::c_ushort,
    pub fip: libc::c_long,
    pub fcs: libc::c_long,
    pub foo: libc::c_long,
    pub fos: libc::c_long,
    pub mxcsr: libc::c_long,
    __reserved: libc::c_long,
    pub st_space: [libc::c_long; 32],
    pub xmm_space: [libc::c_long; 32],
    padding: [libc::c_long; 56],
}

// Not defined by libc on Android
#[cfg(all(target_os = "android", target_arch = "x86"))]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct user_fpregs_struct {
    pub cwd: libc::c_long,
    pub swd: libc::c_long,
    pub twd: libc::c_long,
    pub fip: libc::c_long,
    pub fcs: libc::c_long,
    pub foo: libc::c_long,
    pub fos: libc::c_long,
    pub st_space: [libc::c_long; 20],
}

#[cfg(all(target_os = "android", target_arch = "x86"))]
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct user {
    pub regs: user_regs_struct,
    pub u_fpvalid: libc::c_long,
    pub i387: user_fpregs_struct,
    pub u_tsize: libc::c_ulong,
    pub u_dsize: libc::c_ulong,
    pub u_ssize: libc::c_ulong,
    pub start_code: libc::c_ulong,
    pub start_stack: libc::c_ulong,
    pub signal: libc::c_long,
    __reserved: libc::c_int,
    pub u_ar0: *mut user_regs_struct,
    pub u_fpstate: *mut user_fpregs_struct,
    pub magic: libc::c_ulong,
    pub u_comm: [libc::c_char; 32],
    pub u_debugreg: [libc::c_int; 8],
}

const NUM_DEBUG_REGISTERS: usize = 8;

pub struct ThreadInfoX86 {
    pub stack_pointer: usize,
    pub tgid: Pid, // thread group id
    pub ppid: Pid, // parent process
    pub regs: user_regs_struct,
    pub fpregs: user_fpregs_struct,
    #[cfg(target_arch = "x86_64")]
    pub dregs: [libc::c_ulonglong; NUM_DEBUG_REGISTERS],
    #[cfg(target_arch = "x86")]
    pub dregs: [libc::c_int; NUM_DEBUG_REGISTERS],
    #[cfg(target_arch = "x86")]
    pub fpxregs: user_fpxregs_struct,
}

impl CommonThreadInfo for ThreadInfoX86 {}

impl ThreadInfoX86 {
    // nix currently doesn't support PTRACE_GETREGSET, so we have to do it ourselves
    fn getregset(pid: Pid) -> Result<user_regs_struct> {
        Self::ptrace_get_data_via_io(
            0x4204 as ptrace::RequestType, // PTRACE_GETREGSET
            Some(NT_Elf::NT_PRSTATUS),
            nix::unistd::Pid::from_raw(pid),
        )
    }

    pub fn getregs(pid: Pid) -> Result<user_regs_struct> {
        // TODO: nix restricts PTRACE_GETREGS to arm android for some reason
        Self::ptrace_get_data(
            12 as ptrace::RequestType, // PTRACE_GETREGS
            None,
            nix::unistd::Pid::from_raw(pid),
        )
    }

    // nix currently doesn't support PTRACE_GETREGSET, so we have to do it ourselves
    fn getfpregset(pid: Pid) -> Result<user_fpregs_struct> {
        Self::ptrace_get_data_via_io(
            0x4204 as ptrace::RequestType, // PTRACE_GETREGSET
            Some(NT_Elf::NT_PRFPREGSET),
            nix::unistd::Pid::from_raw(pid),
        )
    }

    // nix currently doesn't support PTRACE_GETFPREGS, so we have to do it ourselves
    fn getfpregs(pid: Pid) -> Result<user_fpregs_struct> {
        Self::ptrace_get_data(
            14 as ptrace::RequestType, // PTRACE_GETFPREGS
            None,
            nix::unistd::Pid::from_raw(pid),
        )
    }

    // nix currently doesn't support PTRACE_GETFPXREGS, so we have to do it ourselves
    #[cfg(target_arch = "x86")]
    fn getfpxregs(pid: Pid) -> Result<user_fpxregs_struct> {
        Self::ptrace_get_data(
            18 as ptrace::RequestType, // PTRACE_GETFPXREGS
            None,
            nix::unistd::Pid::from_raw(pid),
        )
    }

    fn peek_user(pid: Pid, addr: ptrace::AddressType) -> nix::Result<libc::c_long> {
        Self::ptrace_peek(
            ptrace::Request::PTRACE_PEEKUSER as ptrace::RequestType,
            nix::unistd::Pid::from_raw(pid),
            addr,
            std::ptr::null_mut(),
        )
    }

    pub fn create_impl(_pid: Pid, tid: Pid) -> Result<Self> {
        let (ppid, tgid) = Self::get_ppid_and_tgid(tid)?;
        let regs = Self::getregset(tid).or_else(|_| Self::getregs(tid))?;
        let fpregs = Self::getfpregset(tid).or_else(|_| Self::getfpregs(tid))?;
        #[cfg(target_arch = "x86")]
        let fpxregs: user_fpxregs_struct;
        #[cfg(target_arch = "x86")]
        {
            if cfg!(target_feature = "fxsr") {
                fpxregs = Self::getfpxregs(tid)?;
            } else {
                fpxregs = unsafe { std::mem::zeroed() };
            }
        }

        #[cfg(target_arch = "x86_64")]
        let mut dregs: [libc::c_ulonglong; NUM_DEBUG_REGISTERS] = [0; NUM_DEBUG_REGISTERS];
        #[cfg(target_arch = "x86")]
        let mut dregs: [libc::c_int; NUM_DEBUG_REGISTERS] = [0; NUM_DEBUG_REGISTERS];

        let debug_offset = memoffset::offset_of!(user, u_debugreg);
        let elem_offset = size_of_val(&dregs[0]);
        for (idx, dreg) in dregs.iter_mut().enumerate() {
            let chunk = Self::peek_user(
                tid,
                (debug_offset + idx * elem_offset) as ptrace::AddressType,
            )?;
            #[cfg(target_arch = "x86_64")]
            {
                *dreg = chunk as u64; // libc / ptrace is very messy wrt int types used...
            }
            #[cfg(target_arch = "x86")]
            {
                *dreg = chunk as i32; // libc / ptrace is very messy wrt int types used...
            }
        }

        #[cfg(target_arch = "x86_64")]
        let stack_pointer = regs.rsp as usize;
        #[cfg(target_arch = "x86")]
        let stack_pointer = regs.esp as usize;

        Ok(Self {
            stack_pointer,
            tgid,
            ppid,
            regs,
            fpregs,
            dregs,
            #[cfg(target_arch = "x86")]
            fpxregs,
        })
    }

    #[cfg(target_arch = "x86_64")]
    pub fn get_instruction_pointer(&self) -> usize {
        self.regs.rip as usize
    }

    #[cfg(target_arch = "x86")]
    pub fn get_instruction_pointer(&self) -> usize {
        self.regs.eip as usize
    }

    #[cfg(target_arch = "x86_64")]
    pub fn fill_cpu_context(&self, out: &mut RawContextCPU) {
        use format::ContextFlagsAmd64;

        out.context_flags = ContextFlagsAmd64::CONTEXT_AMD64_FULL.bits()
            | ContextFlagsAmd64::CONTEXT_AMD64_SEGMENTS.bits();

        out.cs = self.regs.cs as u16; // TODO: This is u64, do we loose information by doing this?

        out.ds = self.regs.ds as u16; // TODO: This is u64, do we loose information by doing this?
        out.es = self.regs.es as u16; // TODO: This is u64, do we loose information by doing this?
        out.fs = self.regs.fs as u16; // TODO: This is u64, do we loose information by doing this?
        out.gs = self.regs.gs as u16; // TODO: This is u64, do we loose information by doing this?

        out.ss = self.regs.ss as u16; // TODO: This is u64, do we loose information by doing this?
        out.eflags = self.regs.eflags as u32; // TODO: This is u64, do we loose information by doing this?

        out.dr0 = self.dregs[0];
        out.dr1 = self.dregs[1];
        out.dr2 = self.dregs[2];
        out.dr3 = self.dregs[3];
        // 4 and 5 deliberatly omitted because they aren't included in the minidump
        // format.
        out.dr6 = self.dregs[6];
        out.dr7 = self.dregs[7];

        out.rax = self.regs.rax;
        out.rcx = self.regs.rcx;
        out.rdx = self.regs.rdx;
        out.rbx = self.regs.rbx;

        out.rsp = self.regs.rsp;

        out.rbp = self.regs.rbp;
        out.rsi = self.regs.rsi;
        out.rdi = self.regs.rdi;
        out.r8 = self.regs.r8;
        out.r9 = self.regs.r9;
        out.r10 = self.regs.r10;
        out.r11 = self.regs.r11;
        out.r12 = self.regs.r12;
        out.r13 = self.regs.r13;
        out.r14 = self.regs.r14;
        out.r15 = self.regs.r15;

        out.rip = self.regs.rip;

        {
            let fs = &self.fpregs;
            let mut float_save = crate::minidump_cpu::FloatStateCPU {
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

            super::copy_u32_registers(&mut float_save.float_registers, &fs.st_space);
            super::copy_u32_registers(&mut float_save.xmm_registers, &fs.xmm_space);

            out.float_save
                .pwrite_with(float_save, 0, scroll::Endian::Little)
                .expect("this is impossible");
        }
    }

    #[cfg(target_arch = "x86")]
    pub fn fill_cpu_context(&self, out: &mut RawContextCPU) {
        out.context_flags = format::ContextFlagsX86::CONTEXT_X86_ALL.bits();

        out.dr0 = self.dregs[0] as u32;
        out.dr3 = self.dregs[3] as u32;
        out.dr1 = self.dregs[1] as u32;
        out.dr2 = self.dregs[2] as u32;
        // 4 and 5 deliberatly omitted because they aren't included in the minidump
        // format.
        out.dr6 = self.dregs[6] as u32;
        out.dr7 = self.dregs[7] as u32;

        out.gs = self.regs.xgs as u32;
        out.fs = self.regs.xfs as u32;
        out.es = self.regs.xes as u32;
        out.ds = self.regs.xds as u32;

        out.edi = self.regs.edi as u32;
        out.esi = self.regs.esi as u32;
        out.ebx = self.regs.ebx as u32;
        out.edx = self.regs.edx as u32;
        out.ecx = self.regs.ecx as u32;
        out.eax = self.regs.eax as u32;

        out.ebp = self.regs.ebp as u32;
        out.eip = self.regs.eip as u32;
        out.cs = self.regs.xcs as u32;
        out.eflags = self.regs.eflags as u32;
        out.esp = self.regs.esp as u32;
        out.ss = self.regs.xss as u32;

        out.float_save.control_word = self.fpregs.cwd as u32;
        out.float_save.status_word = self.fpregs.swd as u32;
        out.float_save.tag_word = self.fpregs.twd as u32;
        out.float_save.error_offset = self.fpregs.fip as u32;
        out.float_save.error_selector = self.fpregs.fcs as u32;
        out.float_save.data_offset = self.fpregs.foo as u32;
        out.float_save.data_selector = self.fpregs.fos as u32;

        {
            let ra = &mut out.float_save.register_area;
            // 8 registers * 10 bytes per register.
            for (idx, block) in self.fpregs.st_space.iter().enumerate() {
                let offset = idx * std::mem::size_of::<u32>();
                if offset >= ra.len() {
                    break;
                }

                ra.pwrite_with(block, offset, scroll::Endian::Little)
                    .expect("this is impossible");
            }
        }

        #[allow(unused_assignments)]
        {
            let mut offset = 0;
            macro_rules! write_er {
                ($reg:expr) => {
                    offset += out
                        .extended_registers
                        .pwrite_with($reg, offset, scroll::Endian::Little)
                        .unwrap()
                };
            }

            // This matches the Intel fpsave format.
            write_er!(self.fpregs.cwd as u16);
            write_er!(self.fpregs.swd as u16);
            write_er!(self.fpregs.twd as u16);
            write_er!(self.fpxregs.fop);
            write_er!(self.fpxregs.fip);
            write_er!(self.fpxregs.fcs);
            write_er!(self.fpregs.foo);
            write_er!(self.fpregs.fos);
            write_er!(self.fpxregs.mxcsr);

            offset = 32;

            for val in &self.fpxregs.st_space {
                write_er!(val);
            }

            debug_assert_eq!(offset, 160);

            for val in &self.fpxregs.xmm_space {
                write_er!(val);
            }
        }
    }
}
