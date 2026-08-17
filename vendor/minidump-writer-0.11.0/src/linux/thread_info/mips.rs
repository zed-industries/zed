use super::{errors::ThreadInfoError, Pid};

type Result<T> = std::result::Result<T, ThreadInfoError>;

#[derive(Debug)]
pub struct ThreadInfoMips {
    pub stack_pointer: libc::c_ulonglong,
    pub tgid: Pid, // thread group id
    pub ppid: Pid, // parent process
    // Use the structure defined in <sys/ucontext.h>
    pub mcontext: libc::mcontext_t,
}

impl ThreadInfoMips {
    pub fn get_instruction_pointer(&self) -> libc::c_ulonglong {
        self.mcontext.pc
    }

    pub fn fill_cpu_context(&self, out: &mut RawContextCPU) {
        // #if _MIPS_SIM == _ABI64
        //   out->context_flags = MD_CONTEXT_MIPS64_FULL;
        // #elif _MIPS_SIM == _ABIO32
        //   out->context_flags = MD_CONTEXT_MIPS_FULL;
        for idx in 0..MD_CONTEXT_MIPS_GPR_COUNT {
            out.iregs[idx] = self.mcontext.gregs[idx];
        }

        out.mdhi = self.mcontext.mdhi;
        out.mdlo = self.mcontext.mdlo;
        out.dsp_control = self.mcontext.dsp;

        out.hi[0] = self.mcontext.hi1;
        out.lo[0] = self.mcontext.lo1;
        out.hi[1] = self.mcontext.hi2;
        out.lo[1] = self.mcontext.lo2;
        out.hi[2] = self.mcontext.hi3;
        out.lo[2] = self.mcontext.lo3;

        out.epc = self.mcontext.pc;
        out.badvaddr = 0; // Not stored in mcontext
        out.status = 0; // Not stored in mcontext
        out.cause = 0; // Not stored in mcontext

        for idx in 0..MD_FLOATINGSAVEAREA_MIPS_FPR_COUNT {
            out.float_save.regs[idx] = self.mcontext.fpregs.fp_r.fp_fregs[idx]._fp_fregs;
        }

        out.float_save.fpcsr = mcontext.fpc_csr;

        // #if _MIPS_SIM == _ABIO32
        //   out.float_save.fir = self.mcontext.fpc_eir;
        // #endif
    }
}
