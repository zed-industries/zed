//! Debug Support Protocol
//!
//! It provides the services to allow the debug agent to register callback functions that are
//! called either periodically or when specific processor exceptions occur.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x2755590c,
    0x6f3c,
    0x42fa,
    0x9e,
    0xa4,
    &[0xa3, 0xba, 0x54, 0x3c, 0xda, 0x25],
);

pub type InstructionSetArchitecture = u32;

pub const ISA_IA32: InstructionSetArchitecture = 0x014c;
pub const ISA_X64: InstructionSetArchitecture = 0x8664;
pub const ISA_IPF: InstructionSetArchitecture = 0x0200;
pub const ISA_EBC: InstructionSetArchitecture = 0x0ebc;
pub const ISA_ARM: InstructionSetArchitecture = 0x1c2;
pub const ISA_AARCH64: InstructionSetArchitecture = 0xaa64;
pub const ISA_RISCV32: InstructionSetArchitecture = 0x5032;
pub const ISA_RISCV64: InstructionSetArchitecture = 0x5064;
pub const ISA_RISCV128: InstructionSetArchitecture = 0x5128;

#[repr(C)]
#[derive(Clone, Copy)]
pub union SystemContext {
    pub system_context_ebc: *mut SystemContextEbc,
    pub system_context_ia32: *mut SystemContextIa32,
    pub system_context_x64: *mut SystemContextX64,
    pub system_context_ipf: *mut SystemContextIpf,
    pub system_context_arm: *mut SystemContextArm,
    pub system_context_aarch64: *mut SystemContextAArch64,
    pub system_context_riscv32: *mut SystemContextRiscV32,
    pub system_context_riscv64: *mut SystemContextRiscV64,
    pub system_context_riscv128: *mut SystemContextRiscV128,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemContextEbc {
    pub r0: u64,
    pub r1: u64,
    pub r2: u64,
    pub r3: u64,
    pub r4: u64,
    pub r5: u64,
    pub r6: u64,
    pub r7: u64,
    pub flags: u64,
    pub control_flags: u64,
    pub ip: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemContextRiscV32 {
    // Integer registers
    pub zero: u32,
    pub ra: u32,
    pub sp: u32,
    pub gp: u32,
    pub tp: u32,
    pub t0: u32,
    pub t1: u32,
    pub t2: u32,
    pub s0fp: u32,
    pub s1: u32,
    pub a0: u32,
    pub a1: u32,
    pub a2: u32,
    pub a3: u32,
    pub a4: u32,
    pub a5: u32,
    pub a6: u32,
    pub a7: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
    pub s5: u32,
    pub s6: u32,
    pub s7: u32,
    pub s8: u32,
    pub s9: u32,
    pub s10: u32,
    pub s11: u32,
    pub t3: u32,
    pub t4: u32,
    pub t5: u32,
    pub t6: u32,
    // Floating registers for F, D and Q Standard Extensions
    pub ft0: u128,
    pub ft1: u128,
    pub ft2: u128,
    pub ft3: u128,
    pub ft4: u128,
    pub ft5: u128,
    pub ft6: u128,
    pub ft7: u128,
    pub fs0: u128,
    pub fs1: u128,
    pub fa0: u128,
    pub fa1: u128,
    pub fa2: u128,
    pub fa3: u128,
    pub fa4: u128,
    pub fa5: u128,
    pub fa6: u128,
    pub fa7: u128,
    pub fs2: u128,
    pub fs3: u128,
    pub fs4: u128,
    pub fs5: u128,
    pub fs6: u128,
    pub fs7: u128,
    pub fs8: u128,
    pub fs9: u128,
    pub fs10: u128,
    pub fs11: u128,
    pub ft8: u128,
    pub ft9: u128,
    pub ft10: u128,
    pub ft11: u128,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemContextRiscV64 {
    // Integer registers
    pub zero: u64,
    pub ra: u64,
    pub sp: u64,
    pub gp: u64,
    pub tp: u64,
    pub t0: u64,
    pub t1: u64,
    pub t2: u64,
    pub s0fp: u64,
    pub s1: u64,
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
    pub a4: u64,
    pub a5: u64,
    pub a6: u64,
    pub a7: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
    pub t3: u64,
    pub t4: u64,
    pub t5: u64,
    pub t6: u64,
    // Floating registers for F, D and Q Standard Extensions
    pub ft0: u128,
    pub ft1: u128,
    pub ft2: u128,
    pub ft3: u128,
    pub ft4: u128,
    pub ft5: u128,
    pub ft6: u128,
    pub ft7: u128,
    pub fs0: u128,
    pub fs1: u128,
    pub fa0: u128,
    pub fa1: u128,
    pub fa2: u128,
    pub fa3: u128,
    pub fa4: u128,
    pub fa5: u128,
    pub fa6: u128,
    pub fa7: u128,
    pub fs2: u128,
    pub fs3: u128,
    pub fs4: u128,
    pub fs5: u128,
    pub fs6: u128,
    pub fs7: u128,
    pub fs8: u128,
    pub fs9: u128,
    pub fs10: u128,
    pub fs11: u128,
    pub ft8: u128,
    pub ft9: u128,
    pub ft10: u128,
    pub ft11: u128,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemContextRiscV128 {
    // Integer registers
    pub zero: u128,
    pub ra: u128,
    pub sp: u128,
    pub gp: u128,
    pub tp: u128,
    pub t0: u128,
    pub t1: u128,
    pub t2: u128,
    pub s0fp: u128,
    pub s1: u128,
    pub a0: u128,
    pub a1: u128,
    pub a2: u128,
    pub a3: u128,
    pub a4: u128,
    pub a5: u128,
    pub a6: u128,
    pub a7: u128,
    pub s2: u128,
    pub s3: u128,
    pub s4: u128,
    pub s5: u128,
    pub s6: u128,
    pub s7: u128,
    pub s8: u128,
    pub s9: u128,
    pub s10: u128,
    pub s11: u128,
    pub t3: u128,
    pub t4: u128,
    pub t5: u128,
    pub t6: u128,
    // Floating registers for F, D and Q Standard Extensions
    pub ft0: u128,
    pub ft1: u128,
    pub ft2: u128,
    pub ft3: u128,
    pub ft4: u128,
    pub ft5: u128,
    pub ft6: u128,
    pub ft7: u128,
    pub fs0: u128,
    pub fs1: u128,
    pub fa0: u128,
    pub fa1: u128,
    pub fa2: u128,
    pub fa3: u128,
    pub fa4: u128,
    pub fa5: u128,
    pub fa6: u128,
    pub fa7: u128,
    pub fs2: u128,
    pub fs3: u128,
    pub fs4: u128,
    pub fs5: u128,
    pub fs6: u128,
    pub fs7: u128,
    pub fs8: u128,
    pub fs9: u128,
    pub fs10: u128,
    pub fs11: u128,
    pub ft8: u128,
    pub ft9: u128,
    pub ft10: u128,
    pub ft11: u128,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemContextIa32 {
    // ExceptionData is additional data pushed on the stack by some types of IA-32 exceptions
    pub exception_data: u32,
    pub fx_save_state: FxSaveStateIA32,
    pub dr0: u32,
    pub dr1: u32,
    pub dr2: u32,
    pub dr3: u32,
    pub dr6: u32,
    pub dr7: u32,
    pub cr0: u32,
    // Reserved
    pub cr1: u32,
    pub cr2: u32,
    pub cr3: u32,
    pub cr4: u32,
    pub eflags: u32,
    pub ldtr: u32,
    pub tr: u32,
    pub gdtr: [u32; 2],
    pub idtr: [u32; 2],
    pub eip: u32,
    pub gs: u32,
    pub fs: u32,
    pub es: u32,
    pub ds: u32,
    pub cs: u32,
    pub ss: u32,
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
}

// FXSAVE_STATE - FP / MMX / XMM registers
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FxSaveStateIA32 {
    pub fcw: u16,
    pub fsw: u16,
    pub ftw: u16,
    pub opcode: u16,
    pub eip: u32,
    pub cs: u16,
    pub reserved_1: u16,
    pub data_offset: u32,
    pub ds: u16,
    pub reserved_2: [u8; 10],
    pub st0mm0: [u8; 10],
    pub reserved_3: [u8; 6],
    pub st1mm1: [u8; 10],
    pub reserved_4: [u8; 6],
    pub st2mm2: [u8; 10],
    pub reserved_5: [u8; 6],
    pub st3mm3: [u8; 10],
    pub reserved_6: [u8; 6],
    pub st4mm4: [u8; 10],
    pub reserved_7: [u8; 6],
    pub st5mm5: [u8; 10],
    pub reserved_8: [u8; 6],
    pub st6mm6: [u8; 10],
    pub reserved_9: [u8; 6],
    pub st7mm7: [u8; 10],
    pub reserved_10: [u8; 6],
    pub xmm0: [u8; 16],
    pub xmm1: [u8; 16],
    pub xmm2: [u8; 16],
    pub xmm3: [u8; 16],
    pub xmm4: [u8; 16],
    pub xmm5: [u8; 16],
    pub xmm6: [u8; 16],
    pub xmm7: [u8; 16],
    pub reserved_11: [u8; 14 * 16],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemContextX64 {
    // ExceptionData is additional data pushed on the stack by some types of x64 64-bit mode exceptions
    pub exception_data: u64,
    pub fx_save_state: FxSaveStateX64,
    pub dr0: u64,
    pub dr1: u64,
    pub dr2: u64,
    pub dr3: u64,
    pub dr6: u64,
    pub dr7: u64,
    pub cr0: u64,
    // Reserved
    pub cr1: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub rflags: u64,
    pub ldtr: u64,
    pub tr: u64,
    pub gdtr: [u64; 2],
    pub idtr: [u64; 2],
    pub rip: u64,
    pub gs: u64,
    pub fs: u64,
    pub es: u64,
    pub ds: u64,
    pub cs: u64,
    pub ss: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rbx: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rax: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

// FXSAVE_STATE – FP / MMX / XMM registers
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FxSaveStateX64 {
    pub fcw: u16,
    pub fsw: u16,
    pub ftw: u16,
    pub opcode: u16,
    pub rip: u64,
    pub data_offset: u64,
    pub reserved_1: [u8; 8],
    pub st0mm0: [u8; 10],
    pub reserved_2: [u8; 6],
    pub st1mm1: [u8; 10],
    pub reserved_3: [u8; 6],
    pub st2mm2: [u8; 10],
    pub reserved_4: [u8; 6],
    pub st3mm3: [u8; 10],
    pub reserved_5: [u8; 6],
    pub st4mm4: [u8; 10],
    pub reserved_6: [u8; 6],
    pub st5mm5: [u8; 10],
    pub reserved_7: [u8; 6],
    pub st6mm6: [u8; 10],
    pub reserved_8: [u8; 6],
    pub st7mm7: [u8; 10],
    pub reserved_9: [u8; 6],
    pub xmm0: [u8; 16],
    pub xmm1: [u8; 16],
    pub xmm2: [u8; 16],
    pub xmm3: [u8; 16],
    pub xmm4: [u8; 16],
    pub xmm5: [u8; 16],
    pub xmm6: [u8; 16],
    pub xmm7: [u8; 16],
    pub reserved_11: [u8; 14 * 16],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemContextIpf {
    pub reserved: u64,
    pub r1: u64,
    pub r2: u64,
    pub r3: u64,
    pub r4: u64,
    pub r5: u64,
    pub r6: u64,
    pub r7: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub r16: u64,
    pub r17: u64,
    pub r18: u64,
    pub r19: u64,
    pub r20: u64,
    pub r21: u64,
    pub r22: u64,
    pub r23: u64,
    pub r24: u64,
    pub r25: u64,
    pub r26: u64,
    pub r27: u64,
    pub r28: u64,
    pub r29: u64,
    pub r30: u64,
    pub r31: u64,
    pub f2: [u64; 2],
    pub f3: [u64; 2],
    pub f4: [u64; 2],
    pub f5: [u64; 2],
    pub f6: [u64; 2],
    pub f7: [u64; 2],
    pub f8: [u64; 2],
    pub f9: [u64; 2],
    pub f10: [u64; 2],
    pub f11: [u64; 2],
    pub f12: [u64; 2],
    pub f13: [u64; 2],
    pub f14: [u64; 2],
    pub f15: [u64; 2],
    pub f16: [u64; 2],
    pub f17: [u64; 2],
    pub f18: [u64; 2],
    pub f19: [u64; 2],
    pub f20: [u64; 2],
    pub f21: [u64; 2],
    pub f22: [u64; 2],
    pub f23: [u64; 2],
    pub f24: [u64; 2],
    pub f25: [u64; 2],
    pub f26: [u64; 2],
    pub f27: [u64; 2],
    pub f28: [u64; 2],
    pub f29: [u64; 2],
    pub f30: [u64; 2],
    pub f31: [u64; 2],
    pub pr: u64,
    pub b0: u64,
    pub b1: u64,
    pub b2: u64,
    pub b3: u64,
    pub b4: u64,
    pub b5: u64,
    pub b6: u64,
    pub b7: u64,
    // application registers
    pub ar_rsc: u64,
    pub ar_bsp: u64,
    pub ar_bspstore: u64,
    pub ar_rnat: u64,
    pub ar_fcr: u64,
    pub ar_eflag: u64,
    pub ar_csd: u64,
    pub ar_ssd: u64,
    pub ar_cflg: u64,
    pub ar_fsr: u64,
    pub ar_fir: u64,
    pub ar_fdr: u64,
    pub ar_ccv: u64,
    pub ar_unat: u64,
    pub ar_fpsr: u64,
    pub ar_pfs: u64,
    pub ar_lc: u64,
    pub ar_ec: u64,
    // control registers
    pub cr_dcr: u64,
    pub cr_itm: u64,
    pub cr_iva: u64,
    pub cr_pta: u64,
    pub cr_ipsr: u64,
    pub cr_isr: u64,
    pub cr_iip: u64,
    pub cr_ifa: u64,
    pub cr_itir: u64,
    pub cr_iipa: u64,
    pub cr_ifs: u64,
    pub cr_iim: u64,
    pub cr_iha: u64,
    // debug registers
    pub dbr0: u64,
    pub dbr1: u64,
    pub dbr2: u64,
    pub dbr3: u64,
    pub dbr4: u64,
    pub dbr5: u64,
    pub dbr6: u64,
    pub dbr7: u64,
    pub ibr0: u64,
    pub ibr1: u64,
    pub ibr2: u64,
    pub ibr3: u64,
    pub ibr4: u64,
    pub ibr5: u64,
    pub ibr6: u64,
    pub ibr7: u64,
    // virtual Registers
    pub int_nat: u64, // nat bits for r1-r31
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemContextArm {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
    pub cpsr: u32,
    pub dfsr: u32,
    pub dfar: u32,
    pub ifsr: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemContextAArch64 {
    // General Purpose Registers
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub fp: u64, // x29 - Frame Pointer
    pub lr: u64, // x30 - Link Register
    pub sp: u64, // x31 - Stack Pointer
    // FP/SIMD Registers
    pub v0: [u64; 2],
    pub v1: [u64; 2],
    pub v2: [u64; 2],
    pub v3: [u64; 2],
    pub v4: [u64; 2],
    pub v5: [u64; 2],
    pub v6: [u64; 2],
    pub v7: [u64; 2],
    pub v8: [u64; 2],
    pub v9: [u64; 2],
    pub v10: [u64; 2],
    pub v11: [u64; 2],
    pub v12: [u64; 2],
    pub v13: [u64; 2],
    pub v14: [u64; 2],
    pub v15: [u64; 2],
    pub v16: [u64; 2],
    pub v17: [u64; 2],
    pub v18: [u64; 2],
    pub v19: [u64; 2],
    pub v20: [u64; 2],
    pub v21: [u64; 2],
    pub v22: [u64; 2],
    pub v23: [u64; 2],
    pub v24: [u64; 2],
    pub v25: [u64; 2],
    pub v26: [u64; 2],
    pub v27: [u64; 2],
    pub v28: [u64; 2],
    pub v29: [u64; 2],
    pub v30: [u64; 2],
    pub v31: [u64; 2],
    pub elr: u64,  // Exception Link Register
    pub spsr: u64, // Saved Processor Status Register
    pub fpsr: u64, // Floating Point Status Register
    pub esr: u64,  // Exception Syndrome Register
    pub far: u64,  // Fault Address Register
}

pub type ExceptionType = isize;

// EBC Exception types
pub const EXCEPT_EBC_UNDEFINED: ExceptionType = 0;
pub const EXCEPT_EBC_DIVIDE_ERROR: ExceptionType = 1;
pub const EXCEPT_EBC_DEBUG: ExceptionType = 2;
pub const EXCEPT_EBC_BREAKPOINT: ExceptionType = 3;
pub const EXCEPT_EBC_OVERFLOW: ExceptionType = 4;
pub const EXCEPT_EBC_INVALID_OPCODE: ExceptionType = 5;
pub const EXCEPT_EBC_STACK_FAULT: ExceptionType = 6;
pub const EXCEPT_EBC_ALIGNMENT_CHECK: ExceptionType = 7;
pub const EXCEPT_EBC_INSTRUCTION_ENCODING: ExceptionType = 8;
pub const EXCEPT_EBC_BAD_BREAK: ExceptionType = 9;
pub const EXCEPT_EBC_SINGLE_STEP: ExceptionType = 10;

// IA-32 Exception types
pub const EXCEPT_IA32_DIVIDE_ERROR: ExceptionType = 0;
pub const EXCEPT_IA32_DEBUG: ExceptionType = 1;
pub const EXCEPT_IA32_NMI: ExceptionType = 2;
pub const EXCEPT_IA32_BREAKPOINT: ExceptionType = 3;
pub const EXCEPT_IA32_OVERFLOW: ExceptionType = 4;
pub const EXCEPT_IA32_BOUND: ExceptionType = 5;
pub const EXCEPT_IA32_INVALID_OPCODE: ExceptionType = 6;
pub const EXCEPT_IA32_DOUBLE_FAULT: ExceptionType = 8;
pub const EXCEPT_IA32_INVALID_TSS: ExceptionType = 10;
pub const EXCEPT_IA32_SEG_NOT_PRESENT: ExceptionType = 11;
pub const EXCEPT_IA32_STACK_FAULT: ExceptionType = 12;
pub const EXCEPT_IA32_GP_FAULT: ExceptionType = 13;
pub const EXCEPT_IA32_PAGE_FAULT: ExceptionType = 14;
pub const EXCEPT_IA32_FP_ERROR: ExceptionType = 16;
pub const EXCEPT_IA32_ALIGNMENT_CHECK: ExceptionType = 17;
pub const EXCEPT_IA32_MACHINE_CHECK: ExceptionType = 18;
pub const EXCEPT_IA32_SIMD: ExceptionType = 19;

// X64 Exception types
pub const EXCEPT_X64_DIVIDE_ERROR: ExceptionType = 0;
pub const EXCEPT_X64_DEBUG: ExceptionType = 1;
pub const EXCEPT_X64_NMI: ExceptionType = 2;
pub const EXCEPT_X64_BREAKPOINT: ExceptionType = 3;
pub const EXCEPT_X64_OVERFLOW: ExceptionType = 4;
pub const EXCEPT_X64_BOUND: ExceptionType = 5;
pub const EXCEPT_X64_INVALID_OPCODE: ExceptionType = 6;
pub const EXCEPT_X64_DOUBLE_FAULT: ExceptionType = 8;
pub const EXCEPT_X64_INVALID_TSS: ExceptionType = 10;
pub const EXCEPT_X64_SEG_NOT_PRESENT: ExceptionType = 11;
pub const EXCEPT_X64_STACK_FAULT: ExceptionType = 12;
pub const EXCEPT_X64_GP_FAULT: ExceptionType = 13;
pub const EXCEPT_X64_PAGE_FAULT: ExceptionType = 14;
pub const EXCEPT_X64_FP_ERROR: ExceptionType = 16;
pub const EXCEPT_X64_ALIGNMENT_CHECK: ExceptionType = 17;
pub const EXCEPT_X64_MACHINE_CHECK: ExceptionType = 18;
pub const EXCEPT_X64_SIMD: ExceptionType = 19;

// Itanium Processor Family Exception types
pub const EXCEPT_IPF_VHTP_TRANSLATION: ExceptionType = 0;
pub const EXCEPT_IPF_INSTRUCTION_TLB: ExceptionType = 1;
pub const EXCEPT_IPF_DATA_TLB: ExceptionType = 2;
pub const EXCEPT_IPF_ALT_INSTRUCTION_TLB: ExceptionType = 3;
pub const EXCEPT_IPF_ALT_DATA_TLB: ExceptionType = 4;
pub const EXCEPT_IPF_DATA_NESTED_TLB: ExceptionType = 5;
pub const EXCEPT_IPF_INSTRUCTION_KEY_MISSED: ExceptionType = 6;
pub const EXCEPT_IPF_DATA_KEY_MISSED: ExceptionType = 7;
pub const EXCEPT_IPF_DIRTY_BIT: ExceptionType = 8;
pub const EXCEPT_IPF_INSTRUCTION_ACCESS_BIT: ExceptionType = 9;
pub const EXCEPT_IPF_DATA_ACCESS_BIT: ExceptionType = 10;
pub const EXCEPT_IPF_BREAKPOINT: ExceptionType = 11;
pub const EXCEPT_IPF_EXTERNAL_INTERRUPT: ExceptionType = 12;
// 13 - 19 reserved
pub const EXCEPT_IPF_PAGE_NOT_PRESENT: ExceptionType = 20;
pub const EXCEPT_IPF_KEY_PERMISSION: ExceptionType = 21;
pub const EXCEPT_IPF_INSTRUCTION_ACCESS_RIGHTS: ExceptionType = 22;
pub const EXCEPT_IPF_DATA_ACCESS_RIGHTS: ExceptionType = 23;
pub const EXCEPT_IPF_GENERAL_EXCEPTION: ExceptionType = 24;
pub const EXCEPT_IPF_DISABLED_FP_REGISTER: ExceptionType = 25;
pub const EXCEPT_IPF_NAT_CONSUMPTION: ExceptionType = 26;
pub const EXCEPT_IPF_SPECULATION: ExceptionType = 27;
// 28 reserved
pub const EXCEPT_IPF_DEBUG: ExceptionType = 29;
pub const EXCEPT_IPF_UNALIGNED_REFERENCE: ExceptionType = 30;
pub const EXCEPT_IPF_UNSUPPORTED_DATA_REFERENCE: ExceptionType = 31;
pub const EXCEPT_IPF_FP_FAULT: ExceptionType = 32;
pub const EXCEPT_IPF_FP_TRAP: ExceptionType = 33;
pub const EXCEPT_IPF_LOWER_PRIVILEGE_TRANSFER_TRAP: ExceptionType = 34;
pub const EXCEPT_IPF_TAKEN_BRANCH: ExceptionType = 35;
pub const EXCEPT_IPF_SINGLE_STEP: ExceptionType = 36;
// 37 - 44 reserved
pub const EXCEPT_IPF_IA32_EXCEPTION: ExceptionType = 45;
pub const EXCEPT_IPF_IA32_INTERCEPT: ExceptionType = 46;
pub const EXCEPT_IPF_IA32_INTERRUPT: ExceptionType = 47;

// ARM processor exception types
pub const EXCEPT_ARM_RESET: ExceptionType = 0;
pub const EXCEPT_ARM_UNDEFINED_INSTRUCTION: ExceptionType = 1;
pub const EXCEPT_ARM_SOFTWARE_INTERRUPT: ExceptionType = 2;
pub const EXCEPT_ARM_PREFETCH_ABORT: ExceptionType = 3;
pub const EXCEPT_ARM_DATA_ABORT: ExceptionType = 4;
pub const EXCEPT_ARM_RESERVED: ExceptionType = 5;
pub const EXCEPT_ARM_IRQ: ExceptionType = 6;
pub const EXCEPT_ARM_FIQ: ExceptionType = 7;
pub const MAX_ARM_EXCEPTION: ExceptionType = EXCEPT_ARM_FIQ;

// AARCH64 processor exception types.
pub const EXCEPT_AARCH64_SYNCHRONOUS_EXCEPTIONS: ExceptionType = 0;
pub const EXCEPT_AARCH64_IRQ: ExceptionType = 1;
pub const EXCEPT_AARCH64_FIQ: ExceptionType = 2;
pub const EXCEPT_AARCH64_SERROR: ExceptionType = 3;
pub const MAX_AARCH64_EXCEPTION: ExceptionType = EXCEPT_AARCH64_SERROR;

// RISC-V processor exception types.
pub const EXCEPT_RISCV_INST_MISALIGNED: ExceptionType = 0;
pub const EXCEPT_RISCV_INST_ACCESS_FAULT: ExceptionType = 1;
pub const EXCEPT_RISCV_ILLEGAL_INST: ExceptionType = 2;
pub const EXCEPT_RISCV_BREAKPOINT: ExceptionType = 3;
pub const EXCEPT_RISCV_LOAD_ADDRESS_MISALIGNED: ExceptionType = 4;
pub const EXCEPT_RISCV_LOAD_ACCESS_FAULT: ExceptionType = 5;
pub const EXCEPT_RISCV_STORE_AMO_ADDRESS_MISALIGNED: ExceptionType = 6;
pub const EXCEPT_RISCV_STORE_AMO_ACCESS_FAULT: ExceptionType = 7;
pub const EXCEPT_RISCV_ENV_CALL_FROM_UMODE: ExceptionType = 8;
pub const EXCEPT_RISCV_ENV_CALL_FROM_SMODE: ExceptionType = 9;
pub const EXCEPT_RISCV_ENV_CALL_FROM_MMODE: ExceptionType = 11;
pub const EXCEPT_RISCV_INST_PAGE_FAULT: ExceptionType = 12;
pub const EXCEPT_RISCV_LOAD_PAGE_FAULT: ExceptionType = 13;
pub const EXCEPT_RISCV_STORE_AMO_PAGE_FAULT: ExceptionType = 15;

// RISC-V processor interrupt types.
pub const EXCEPT_RISCV_SUPERVISOR_SOFTWARE_INT: ExceptionType = 1;
pub const EXCEPT_RISCV_MACHINE_SOFTWARE_INT: ExceptionType = 3;
pub const EXCEPT_RISCV_SUPERVISOR_TIMER_INT: ExceptionType = 5;
pub const EXCEPT_RISCV_MACHINE_TIMER_INT: ExceptionType = 7;
pub const EXCEPT_RISCV_SUPERVISOR_EXTERNAL_INT: ExceptionType = 9;
pub const EXCEPT_RISCV_MACHINE_EXTERNAL_INT: ExceptionType = 11;

pub type GetMaximumProcessorIndex = eficall! {fn(
    *mut Protocol,
    *mut usize,
) -> crate::base::Status};

pub type PeriodicCallback = eficall! {fn(SystemContext)};

pub type RegisterPeriodicCallback = eficall! {fn(
    *mut Protocol,
    usize,
    Option<PeriodicCallback>,
) -> crate::base::Status};

pub type ExceptionCallback = eficall! {fn(ExceptionType, SystemContext)};

pub type RegisterExceptionCallback = eficall! {fn(
    *mut Protocol,
    usize,
    Option<ExceptionCallback>,
    ExceptionType,
) -> crate::base::Status};

pub type InvalidateInstructionCache = eficall! {fn(
    *mut Protocol,
    usize,
    *mut core::ffi::c_void,
    u64,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub isa: InstructionSetArchitecture,
    pub get_maximum_processor_index: GetMaximumProcessorIndex,
    pub register_periodic_callback: RegisterPeriodicCallback,
    pub register_exception_callback: RegisterExceptionCallback,
    pub invalidate_instruction_cache: InvalidateInstructionCache,
}
