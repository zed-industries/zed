mod getcontext;

pub use getcontext::crash_context_getcontext;

/// The full context for a Linux/Android crash
#[repr(C)]
#[derive(Clone)]
pub struct CrashContext {
    /// Crashing thread context.
    ///
    /// Note that we use [`crate::ucontext_t`] instead of [`libc::ucontext_t`]
    /// as libc's differs between glibc and musl <https://github.com/rust-lang/libc/pull/1646>
    /// even though the `ucontext_t` received from a signal will be the same
    /// regardless of the libc implementation used as it is only arch specific
    /// and not libc specific
    ///
    /// Note that we hide `ucontext_t::uc_link` as it is a pointer and thus can't
    /// be accessed in a process other than the one the `CrashContext` was created
    /// in. This is a just a self-reference so is not useful in practice.
    ///
    /// Note that the same applies to [`mcontext_t::fpregs`], but since that points
    /// to floating point registers and _is_ interesting to read in another process,
    /// those registers available as [`Self::float_state`], except on the `arm`
    /// architecture since they aren't part of `mcontext_t` at all.
    pub context: ucontext_t,
    /// State of floating point registers.
    ///
    /// This isn't part of the user ABI for Linux arm
    #[cfg(not(target_arch = "arm"))]
    pub float_state: fpregset_t,
    /// The signal info for the crash
    pub siginfo: libc::signalfd_siginfo,
    /// The id of the crashing process
    pub pid: libc::pid_t,
    /// The id of the crashing thread
    pub tid: libc::pid_t,
}

unsafe impl Send for CrashContext {}

impl CrashContext {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let size = std::mem::size_of_val(self);
            let ptr = (self as *const Self).cast();
            std::slice::from_raw_parts(ptr, size)
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != std::mem::size_of::<Self>() {
            return None;
        }

        unsafe { Some((*bytes.as_ptr().cast::<Self>()).clone()) }
    }
}

#[repr(C)]
#[derive(Clone)]
#[doc(hidden)]
pub struct sigset_t {
    #[cfg(target_pointer_width = "32")]
    __val: [u32; 32],
    #[cfg(target_pointer_width = "64")]
    __val: [u64; 16],
}

#[repr(C)]
#[derive(Clone)]
#[doc(hidden)]
pub struct stack_t {
    pub ss_sp: *mut std::ffi::c_void,
    pub ss_flags: i32,
    pub ss_size: usize,
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct ucontext_t {
            pub uc_flags: u64,
            uc_link: *mut ucontext_t,
            pub uc_stack: stack_t,
            pub uc_mcontext: mcontext_t,
            pub uc_sigmask: sigset_t,
            __private: [u8; 512],
        }

        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct mcontext_t {
            pub gregs: [i64; 23],
            pub fpregs: *mut fpregset_t,
            __reserved: [u64; 8],
        }

        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct fpregset_t {
            pub cwd: u16,
            pub swd: u16,
            pub ftw: u16,
            pub fop: u16,
            pub rip: u64,
            pub rdp: u64,
            pub mxcsr: u32,
            pub mxcr_mask: u32,
            pub st_space: [u32; 32],
            pub xmm_space: [u32; 64],
            __padding: [u64; 12],
        }
    } else if #[cfg(target_arch = "x86")] {
        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct ucontext_t {
            pub uc_flags: u32,
            uc_link: *mut ucontext_t,
            pub uc_stack: stack_t,
            pub uc_mcontext: mcontext_t,
            pub uc_sigmask: sigset_t,
            pub __fpregs_mem: [u32; 28],
        }

        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct mcontext_t {
            pub gregs: [i32; 23],
            pub fpregs: *mut fpregset_t,
            pub oldmask: u32,
            pub cr2: u32,
        }

        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct fpreg_t {
            pub significand: [u16; 4],
            pub exponent: u16,
        }

        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct fpregset_t {
            pub cw: u32,
            pub sw: u32,
            pub tag: u32,
            pub ipoff: u32,
            pub cssel: u32,
            pub dataoff: u32,
            pub datasel: u32,
            pub _st: [fpreg_t; 8],
            pub status: u32,
        }
    } else if #[cfg(target_arch = "aarch64")] {
        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct ucontext_t {
            pub uc_flags: u64,
            uc_link: *mut ucontext_t,
            pub uc_stack: stack_t,
            pub uc_sigmask: sigset_t,
            pub uc_mcontext: mcontext_t,
        }

        // Note you might see this defined in C with `unsigned long` or
        // `unsigned long long` and think, WTF, those aren't the same! Except
        // `long` means either 32-bit _or_ 64-bit depending on the data model,
        // and the default data model for C/C++ is LP64 which means long is
        // 64-bit. I had forgotten what a trash type long was.
        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct mcontext_t {
            // Note in the kernel this is just part of regs which is length 32
            pub fault_address: u64,
            pub regs: [u64; 31],
            pub sp: u64,
            pub pc: u64,
            pub pstate: u64,
            // Note that u128 is ABI safe on aarch64, this is actually a
            // `long double` in C which Rust doesn't have native support
            pub __reserved: [u128; 256],
        }

        /// Magic value written by the kernel and our custom getcontext
        #[doc(hidden)]
        pub const FPSIMD_MAGIC: u32 = 0x46508001;

        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct _aarch64_ctx {
            pub magic: u32,
            pub size: u32,
        }

        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct fpsimd_context {
            pub head: _aarch64_ctx,
            pub fpsr: u32,
            pub fpcr: u32,
            pub vregs: [u128; 32],
        }

        #[doc(hidden)]
        pub type fpregset_t = fpsimd_context;
    } else if #[cfg(target_arch = "arm")] {
        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct ucontext_t {
            pub uc_flags: u32,
            uc_link: *mut ucontext_t,
            pub uc_stack: stack_t,
            // Note that the mcontext_t and sigset_t are swapped compared to
            // all of the other arches currently supported :p
            pub uc_mcontext: mcontext_t,
            pub uc_sigmask: sigset_t,
            pub uc_regspace: [u64; 64],
        }

        #[repr(C)]
        #[derive(Clone)]
        #[doc(hidden)]
        pub struct mcontext_t {
            pub trap_no: u32,
            pub error_code: u32,
            pub oldmask: u32,
            pub arm_r0: u32,
            pub arm_r1: u32,
            pub arm_r2: u32,
            pub arm_r3: u32,
            pub arm_r4: u32,
            pub arm_r5: u32,
            pub arm_r6: u32,
            pub arm_r7: u32,
            pub arm_r8: u32,
            pub arm_r9: u32,
            pub arm_r10: u32,
            pub arm_fp: u32,
            pub arm_ip: u32,
            pub arm_sp: u32,
            pub arm_lr: u32,
            pub arm_pc: u32,
            pub arm_cpsr: u32,
            pub fault_address: u32,
        }
    }
}

#[cfg(test)]
mod test {
    // Musl doesn't contain fpregs in libc because reasons https://github.com/rust-lang/libc/pull/1646
    #[cfg(not(target_env = "musl"))]
    #[test]
    fn matches_libc() {
        assert_eq!(
            std::mem::size_of::<libc::ucontext_t>(),
            std::mem::size_of::<super::ucontext_t>()
        );
    }
}
