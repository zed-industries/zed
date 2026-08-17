/// Full Windows crash context
pub struct CrashContext {
    /// The information on the exception.
    ///
    /// Note that this is a pointer into the actual memory of the crashed process,
    /// and is a pointer to an [EXCEPTION_POINTERS](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-exception_pointers)
    pub exception_pointers: *const EXCEPTION_POINTERS,
    /// The top level exception code from the `exception_pointers`. This is provided
    /// so that external processes don't need to use `ReadProcessMemory` to inspect
    /// the exception code
    pub exception_code: i32,
    /// The pid of the process that crashed
    pub process_id: u32,
    /// The thread id on which the exception occurred
    pub thread_id: u32,
}

#[link(name = "kernel32")]
extern "system" {
    #[link_name = "RtlCaptureContext"]
    pub fn capture_context(ctx_rec: *mut CONTEXT);
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        #[repr(C, align(16))]
        pub struct M128A {
            pub Low: u64,
            pub High: i64,
        }

        #[repr(C)]
        pub struct CONTEXT_0_0 {
            pub Header: [M128A; 2],
            pub Legacy: [M128A; 8],
            pub Xmm0: M128A,
            pub Xmm1: M128A,
            pub Xmm2: M128A,
            pub Xmm3: M128A,
            pub Xmm4: M128A,
            pub Xmm5: M128A,
            pub Xmm6: M128A,
            pub Xmm7: M128A,
            pub Xmm8: M128A,
            pub Xmm9: M128A,
            pub Xmm10: M128A,
            pub Xmm11: M128A,
            pub Xmm12: M128A,
            pub Xmm13: M128A,
            pub Xmm14: M128A,
            pub Xmm15: M128A,
        }

        #[repr(C, align(16))]
        pub struct XSAVE_FORMAT {
            pub ControlWord: u16,
            pub StatusWord: u16,
            pub TagWord: u8,
            pub Reserved1: u8,
            pub ErrorOpcode: u16,
            pub ErrorOffset: u32,
            pub ErrorSelector: u16,
            pub Reserved2: u16,
            pub DataOffset: u32,
            pub DataSelector: u16,
            pub Reserved3: u16,
            pub MxCsr: u32,
            pub MxCsr_Mask: u32,
            pub FloatRegisters: [M128A; 8],
            pub XmmRegisters: [M128A; 16],
            pub Reserved4: [u8; 96],
        }

        #[repr(C)]
        pub union CONTEXT_0 {
            pub FltSave: std::mem::ManuallyDrop<XSAVE_FORMAT>,
            pub Anonymous: std::mem::ManuallyDrop<CONTEXT_0_0>,
        }

        #[repr(C, align(16))]
        pub struct CONTEXT {
            pub P1Home: u64,
            pub P2Home: u64,
            pub P3Home: u64,
            pub P4Home: u64,
            pub P5Home: u64,
            pub P6Home: u64,
            pub ContextFlags: u32,
            pub MxCsr: u32,
            pub SegCs: u16,
            pub SegDs: u16,
            pub SegEs: u16,
            pub SegFs: u16,
            pub SegGs: u16,
            pub SegSs: u16,
            pub EFlags: u32,
            pub Dr0: u64,
            pub Dr1: u64,
            pub Dr2: u64,
            pub Dr3: u64,
            pub Dr6: u64,
            pub Dr7: u64,
            pub Rax: u64,
            pub Rcx: u64,
            pub Rdx: u64,
            pub Rbx: u64,
            pub Rsp: u64,
            pub Rbp: u64,
            pub Rsi: u64,
            pub Rdi: u64,
            pub R8: u64,
            pub R9: u64,
            pub R10: u64,
            pub R11: u64,
            pub R12: u64,
            pub R13: u64,
            pub R14: u64,
            pub R15: u64,
            pub Rip: u64,
            pub Anonymous: CONTEXT_0,
            pub VectorRegister: [M128A; 26],
            pub VectorControl: u64,
            pub DebugControl: u64,
            pub LastBranchToRip: u64,
            pub LastBranchFromRip: u64,
            pub LastExceptionToRip: u64,
            pub LastExceptionFromRip: u64,
        }
    } else if #[cfg(target_arch = "x86")] {
        #[repr(C)]
        pub struct FLOATING_SAVE_AREA {
            pub ControlWord: u32,
            pub StatusWord: u32,
            pub TagWord: u32,
            pub ErrorOffset: u32,
            pub ErrorSelector: u32,
            pub DataOffset: u32,
            pub DataSelector: u32,
            pub RegisterArea: [u8; 80],
            pub Spare0: u32,
        }

        #[repr(C, packed(4))]
        pub struct CONTEXT {
            pub ContextFlags: u32,
            pub Dr0: u32,
            pub Dr1: u32,
            pub Dr2: u32,
            pub Dr3: u32,
            pub Dr6: u32,
            pub Dr7: u32,
            pub FloatSave: FLOATING_SAVE_AREA,
            pub SegGs: u32,
            pub SegFs: u32,
            pub SegEs: u32,
            pub SegDs: u32,
            pub Edi: u32,
            pub Esi: u32,
            pub Ebx: u32,
            pub Edx: u32,
            pub Ecx: u32,
            pub Eax: u32,
            pub Ebp: u32,
            pub Eip: u32,
            pub SegCs: u32,
            pub EFlags: u32,
            pub Esp: u32,
            pub SegSs: u32,
            pub ExtendedRegisters: [u8; 512],
        }
    } else if #[cfg(target_arch = "aarch64")] {
        #[repr(C)]
        pub struct ARM64_NT_NEON128_0 {
            pub Low: u64,
            pub High: i64,
        }
        #[repr(C)]
        pub union ARM64_NT_NEON128 {
            pub Anonymous: std::mem::ManuallyDrop<ARM64_NT_NEON128_0>,
            pub D: [f64; 2],
            pub S: [f32; 4],
            pub H: [u16; 8],
            pub B: [u8; 16],
        }

        #[repr(C)]
        pub struct CONTEXT_0_0 {
            pub X0: u64,
            pub X1: u64,
            pub X2: u64,
            pub X3: u64,
            pub X4: u64,
            pub X5: u64,
            pub X6: u64,
            pub X7: u64,
            pub X8: u64,
            pub X9: u64,
            pub X10: u64,
            pub X11: u64,
            pub X12: u64,
            pub X13: u64,
            pub X14: u64,
            pub X15: u64,
            pub X16: u64,
            pub X17: u64,
            pub X18: u64,
            pub X19: u64,
            pub X20: u64,
            pub X21: u64,
            pub X22: u64,
            pub X23: u64,
            pub X24: u64,
            pub X25: u64,
            pub X26: u64,
            pub X27: u64,
            pub X28: u64,
            pub Fp: u64,
            pub Lr: u64,
        }

        #[repr(C)]
        pub union CONTEXT_0 {
            pub Anonymous: std::mem::ManuallyDrop<CONTEXT_0_0>,
            pub X: [u64; 31],
        }

        #[repr(C, align(16))]
        pub struct CONTEXT {
            pub ContextFlags: u32,
            pub Cpsr: u32,
            pub Anonymous: CONTEXT_0,
            pub Sp: u64,
            pub Pc: u64,
            pub V: [ARM64_NT_NEON128; 32],
            pub Fpcr: u32,
            pub Fpsr: u32,
            pub Bcr: [u32; 8],
            pub Bvr: [u64; 8],
            pub Wcr: [u32; 2],
            pub Wvr: [u64; 2],
        }
    }
}

pub type NTSTATUS = i32;
pub type BOOL = i32;

#[repr(C)]
pub struct EXCEPTION_RECORD {
    pub ExceptionCode: NTSTATUS,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ExceptionAddress: *mut std::ffi::c_void,
    pub NumberParameters: u32,
    pub ExceptionInformation: [usize; 15],
}

#[repr(C)]
pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: *mut EXCEPTION_RECORD,
    pub ContextRecord: *mut CONTEXT,
}
