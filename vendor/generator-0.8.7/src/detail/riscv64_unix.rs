use crate::detail::{align_down, gen};
use crate::stack::Stack;

// first argument is task handle, second is thunk ptr
pub type InitFn = extern "C" fn(usize, *mut usize) -> !;

pub extern "C" fn gen_init(a1: usize, a2: *mut usize) -> ! {
    gen::gen_init_impl(a1, a2)
}

std::arch::global_asm!(include_str!("asm/asm_riscv64_c_elf.S"));

extern "C" {
    pub fn bootstrap_green_task();
    pub fn prefetch(data: *const usize);
    pub fn swap_registers(out_regs: *mut Registers, in_regs: *const Registers);
}

#[repr(C)]
#[derive(Debug)]
pub struct Registers {
    // We save the 13 callee-saved registers:
    //  x18~x27(s2~s11), fp (s0), s1, sp, ra
    // and the 12 callee-saved floating point registers:
    //  f8~f9(fs0~fs1), f18~f27(fs2~fs11)
    gpr: [usize; 32],
}

impl Registers {
    pub fn new() -> Registers {
        Registers { gpr: [0; 32] }
    }

    #[inline]
    pub fn prefetch(&self) {
        let ptr = self.gpr[12] as *const usize;
        unsafe {
            prefetch(ptr); // SP
            prefetch(ptr.add(1)); // SP + 8
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
    const S2: usize = 18 - 18;
    const S3: usize = 19 - 18;
    const S4: usize = 20 - 18;

    const FP: usize = 28 - 18; // S0
    const S1: usize = 29 - 18;
    const SP: usize = 30 - 18;
    const RA: usize = 31 - 18;

    let sp = align_down(stack.end());

    // These registers are frobbed by bootstrap_green_task into the right
    // location so we can invoke the "real init function", `fptr`.
    regs.gpr[S2] = arg;
    regs.gpr[S3] = arg2 as usize;
    regs.gpr[S4] = fptr as usize;

    regs.gpr[FP] = sp as usize;
    regs.gpr[S1] = 0;
    regs.gpr[SP] = sp as usize;
    regs.gpr[RA] = bootstrap_green_task as usize;
}
