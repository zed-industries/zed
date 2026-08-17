use crate::detail::{align_down, mut_offset};
use crate::stack::Stack;

// first argument is task handle, second is thunk ptr
pub type InitFn = extern "sysv64" fn(usize, *mut usize) -> !;

pub extern "sysv64" fn gen_init(a1: usize, a2: *mut usize) -> ! {
    super::gen::gen_init_impl(a1, a2)
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        std::arch::global_asm!(include_str!("asm/asm_x86_64_sysv_macho.S"));
    } else {
        std::arch::global_asm!(include_str!("asm/asm_x86_64_sysv_elf.S"));
    }
}

// #[cfg(not(nightly))]
//#[link(name = "asm", kind = "static")]
extern "sysv64" {
    pub fn bootstrap_green_task();
    pub fn prefetch(data: *const usize);
    pub fn swap_registers(out_regs: *mut Registers, in_regs: *const Registers);
}

/*
#[cfg(nightly)]
mod asm_impl {
    use super::Registers;
    /// prefetch data
    #[inline]
    pub unsafe extern "C" fn prefetch(data: *const usize) {
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
                mov %r12, %rdi     // setup the function arg
                mov %r13, %rsi     // setup the function arg
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
        // The first argument is in %rdi, and the second one is in %rsi
        llvm_asm!(
            ""
            :
            : "{rdi}"(out_regs), "{rsi}"(in_regs)
            :
            :
        );

        // introduce this function to workaround rustc bug! (#6)
        #[naked]
        unsafe extern "C" fn _swap_reg() {
            // Save registers
            llvm_asm!(
                "
                    mov %rbx, (0*8)(%rdi)
                    mov %rsp, (1*8)(%rdi)
                    mov %rbp, (2*8)(%rdi)
                    mov %r12, (4*8)(%rdi)
                    mov %r13, (5*8)(%rdi)
                    mov %r14, (6*8)(%rdi)
                    mov %r15, (7*8)(%rdi)

                    mov (0*8)(%rsi), %rbx
                    mov (1*8)(%rsi), %rsp
                    mov (2*8)(%rsi), %rbp
                    mov (4*8)(%rsi), %r12
                    mov (5*8)(%rsi), %r13
                    mov (6*8)(%rsi), %r14
                    mov (7*8)(%rsi), %r15
                "
                :
                : //"{rdi}"(out_regs), "{rsi}"(in_regs)
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

#[repr(C)]
#[derive(Debug)]
pub struct Registers {
    gpr: [usize; 8],
}

impl Registers {
    pub fn new() -> Registers {
        Registers { gpr: [0; 8] }
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

    let sp = align_down(stack.end());

    // These registers are frobbed by bootstrap_green_task into the right
    // location so we can invoke the "real init function", `fptr`.
    regs.gpr[RUSTRT_R12] = arg;
    regs.gpr[RUSTRT_R13] = arg2 as usize;
    regs.gpr[RUSTRT_R14] = fptr as usize;

    // Last base pointer on the stack should be 0
    regs.gpr[RUSTRT_RBP] = 0;

    // setup the init stack
    // this is prepared for the swap context
    regs.gpr[RUSTRT_RSP] = mut_offset(sp, -2) as usize;

    unsafe {
        // leave enough space for RET
        *mut_offset(sp, -2) = bootstrap_green_task as usize;
        *mut_offset(sp, -1) = 0;
    }
}
