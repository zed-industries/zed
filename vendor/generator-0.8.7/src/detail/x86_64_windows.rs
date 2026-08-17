use crate::detail::{align_down, mut_offset};
use crate::stack::Stack;

// first argument is task handle, second is thunk ptr
pub type InitFn = extern "sysv64" fn(usize, *mut usize) -> !;

pub extern "sysv64" fn gen_init(a1: usize, a2: *mut usize) -> ! {
    super::gen::gen_init_impl(a1, a2)
}

std::arch::global_asm!(include_str!("asm/asm_x86_64_sysv_pe.S"));

// #[cfg(not(nightly))]
//#[link(name = "asm", kind = "static")]
extern "sysv64" {
    pub fn bootstrap_green_task();
    pub fn prefetch_asm(data: *const usize);
    pub fn swap_registers(out_regs: *mut Registers, in_regs: *const Registers);
}

#[inline]
pub fn prefetch(data: *const usize) {
    unsafe { prefetch_asm(data) }
}

/*
#[cfg(nightly)]
mod asm_impl {
    use super::Registers;
    /// prefetch data
    #[inline]
    pub unsafe extern "C" fn prefetch_asm(data: *const usize) {
        llvm_asm!(
            "prefetcht1 $0"
            : // no output
            : "m"(*data)
            :
            : "volatile"
        );
    }

    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn bootstrap_green_task() {
        llvm_asm!(
            "
                mov %r12, %rcx     // setup the function arg
                mov %r13, %rdx     // setup the function arg
                and $$-16, %rsp    // align the stack pointer
                mov %r14, (%rsp)   // this is the new return address
            "
            : // no output
            : // no input
            : "memory"
            : "volatile"
        );
    }

    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn swap_registers(out_regs: *mut Registers, in_regs: *const Registers) {
        // The first argument is in %rcx, and the second one is in %rdx
        llvm_asm!(
            ""
            :
            : "{rcx}"(out_regs), "{rdx}"(in_regs)
            :
            :
        );

        // introduce this function to workaround rustc bug! (#6)
        #[naked]
        unsafe extern "C" fn _swap_reg() {
            // Save registers
            llvm_asm!(
                "
                    mov %rbx, (0*8)(%rcx)
                    mov %rsp, (1*8)(%rcx)
                    mov %rbp, (2*8)(%rcx)
                    mov %r12, (4*8)(%rcx)
                    mov %r13, (5*8)(%rcx)
                    mov %r14, (6*8)(%rcx)
                    mov %r15, (7*8)(%rcx)
                    mov %rdi, (9*8)(%rcx)
                    mov %rsi, (10*8)(%rcx)

                    // mov %rcx, %r10
                    // and $$0xf0, %r10b

                    // Save non-volatile XMM registers:
                    movapd %xmm6, (16*8)(%rcx)
                    movapd %xmm7, (18*8)(%rcx)
                    movapd %xmm8, (20*8)(%rcx)
                    movapd %xmm9, (22*8)(%rcx)
                    movapd %xmm10, (24*8)(%rcx)
                    movapd %xmm11, (26*8)(%rcx)
                    movapd %xmm12, (28*8)(%rcx)
                    movapd %xmm13, (30*8)(%rcx)
                    movapd %xmm14, (32*8)(%rcx)
                    movapd %xmm15, (34*8)(%rcx)

                    /* load NT_TIB */
                    movq  %gs:(0x30), %r10
/* save current stack base */
                    movq  0x08(%r10), %rax
                    mov  %rax, (11*8)(%rcx)
/* save current stack limit */
                    movq  0x10(%r10), %rax
                     mov  %rax, (12*8)(%rcx)
/* save current deallocation stack */
                    movq  0x1478(%r10), %rax
                    mov  %rax, (13*8)(%rcx)
/* save fiber local storage */
// movq  0x18(%r10), %rax
// mov  %rax, (14*8)(%rcx)

// mov %rcx, (3*8)(%rcx)

                    mov (0*8)(%rdx), %rbx
                    mov (1*8)(%rdx), %rsp
                    mov (2*8)(%rdx), %rbp
                    mov (4*8)(%rdx), %r12
                    mov (5*8)(%rdx), %r13
                    mov (6*8)(%rdx), %r14
                    mov (7*8)(%rdx), %r15
                    mov (9*8)(%rdx), %rdi
                    mov (10*8)(%rdx), %rsi

// Restore non-volatile XMM registers:
                    movapd (16*8)(%rdx), %xmm6
                    movapd (18*8)(%rdx), %xmm7
                    movapd (20*8)(%rdx), %xmm8
                    movapd (22*8)(%rdx), %xmm9
                    movapd (24*8)(%rdx), %xmm10
                    movapd (26*8)(%rdx), %xmm11
                    movapd (28*8)(%rdx), %xmm12
                    movapd (30*8)(%rdx), %xmm13
                    movapd (32*8)(%rdx), %xmm14
                    movapd (34*8)(%rdx), %xmm15

/* load NT_TIB */
                    movq  %gs:(0x30), %r10
/* restore fiber local storage */
// mov (14*8)(%rdx), %rax
// movq  %rax, 0x18(%r10)
/* restore deallocation stack */
                    mov (13*8)(%rdx), %rax
                    movq  %rax, 0x1478(%r10)
/* restore stack limit */
                    mov (12*8)(%rdx), %rax
                    movq  %rax, 0x10(%r10)
/* restore stack base */
                    mov  (11*8)(%rdx), %rax
                    movq  %rax, 0x8(%r10)

// mov (3*8)(%rdx), %rcx
                "
                // why save the rcx and rdx in stack? this will overwrite something!
                // the naked function should only use the asm block, debug version breaks
                // since rustc 1.27.0-nightly, we have to use O2 level optimization (#6)
                :
                : //"{rcx}"(out_regs), "{rdx}"(in_regs)
                : "memory"
                : "volatile"
            );
        }

        _swap_reg()
    }
}
#[cfg(nightly)]
pub use self::asm_impl::*;
*/

// windows need to restore xmm6~xmm15, for most cases only use two xmm registers
// so we use sysv64
#[repr(C)]
#[derive(Debug)]
pub struct Registers {
    pub(crate) gpr: [usize; 16],
}

impl Registers {
    pub fn new() -> Registers {
        Registers { gpr: [0; 16] }
    }

    #[inline]
    pub fn prefetch(&self) {
        let ptr = self.gpr[1] as *const usize;
        unsafe {
            prefetch(ptr); // RSP
            prefetch(ptr.add(8)); // RSP + 8
        }
    }
}

pub fn initialize_call_frame(
    regs: &mut Registers,
    fptr: InitFn,
    arg: usize,
    arg2: *mut usize,
    stack: &Stack,
) {
    // Redefinitions from rt/arch/x86_64/regs.h
    const RUSTRT_RSP: usize = 1;
    const RUSTRT_RBP: usize = 2;
    const RUSTRT_R12: usize = 4;
    const RUSTRT_R13: usize = 5;
    const RUSTRT_R14: usize = 6;
    const RUSTRT_STACK_BASE: usize = 11;
    const RUSTRT_STACK_LIMIT: usize = 12;
    const RUSTRT_STACK_DEALLOC: usize = 13;

    let sp = align_down(stack.end());

    // These registers are frobbed by bootstrap_green_task into the right
    // location so we can invoke the "real init function", `fptr`.
    regs.gpr[RUSTRT_R12] = arg;
    regs.gpr[RUSTRT_R13] = arg2 as usize;
    regs.gpr[RUSTRT_R14] = fptr as usize;

    // Last base pointer on the stack should be 0
    regs.gpr[RUSTRT_RBP] = 0;

    regs.gpr[RUSTRT_STACK_BASE] = stack.end() as usize;
    regs.gpr[RUSTRT_STACK_LIMIT] = stack.begin() as usize;
    regs.gpr[RUSTRT_STACK_DEALLOC] = 0; //mut_offset(sp, -8192) as usize;

    // setup the init stack
    // this is prepared for the swap context
    regs.gpr[RUSTRT_RSP] = mut_offset(sp, -2) as usize;

    unsafe {
        // leave enough space for RET
        *mut_offset(sp, -2) = bootstrap_green_task as usize;
        *mut_offset(sp, -1) = 0;
    }
}
