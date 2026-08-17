use crate::detail::{align_down, gen};
use crate::stack::Stack;

// first argument is task handle, second is thunk ptr
pub type InitFn = extern "aapcs" fn(usize, *mut usize) -> !;

pub extern "aapcs" fn gen_init(a1: usize, a2: *mut usize) -> ! {
    gen::gen_init_impl(a1, a2)
}

std::arch::global_asm!(include_str!("asm/asm_arm_aapcs_elf.S"));

extern "aapcs" {
    pub fn bootstrap_green_task();
    pub fn prefetch(data: *const usize);
    pub fn swap_registers(out_regs: *mut Registers, in_regs: *const Registers);
}

#[repr(C)]
#[derive(Debug)]
pub struct Registers {
    // We save the 10 callee-saved registers:
    //  r4~r10(v1~v7), fp (r11), lr (r14), sp
    // and the 16 callee-saved floating point registers:
    //  s16~s31
    gpr: [usize; 32],
}

impl Registers {
    pub fn new() -> Registers {
        Registers { gpr: [0; 32] }
    }

    #[inline]
    pub fn prefetch(&self) {
        let ptr = self.gpr[8 /* SP */] as *const usize;
        unsafe {
            prefetch(ptr); // SP
            prefetch(ptr.add(1)); // SP + 4
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
    // Callee-saved registers start at r4
    const R4: usize = 4 - 4;
    const R5: usize = 5 - 4;
    const R6: usize = 6 - 4;

    const FP: usize = 11 - 4; // R11
    const SP: usize = 12 - 4; // R13
    const LR: usize = 13 - 4; // R14

    let sp = align_down(stack.end());

    // These registers are frobbed by bootstrap_green_task into the right
    // location so we can invoke the "real init function", `fptr`.
    regs.gpr[R4] = arg;
    regs.gpr[R5] = arg2 as usize;
    regs.gpr[R6] = fptr as usize;

    // arm current stack frame pointer
    regs.gpr[FP] = sp as usize;

    regs.gpr[LR] = bootstrap_green_task as usize;

    // setup the init stack
    // this is prepared for the swap context
    regs.gpr[SP] = sp as usize;
}
