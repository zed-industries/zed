use crate::detail::{align_down, mut_offset};
use crate::stack::Stack;

// first argument is task handle, second is thunk ptr
pub type InitFn = extern "C" fn(usize, *mut usize) -> !;

pub extern "C" fn gen_init(a1: usize, a2: *mut usize) -> ! {
    super::gen::gen_init_impl(a1, a2);
}

extern "C" {
    pub fn bootstrap_green_task();
    pub fn prefetch(data: *const usize);
    #[allow(improper_ctypes)] // allow declaring u128 in Registers (since f128 is not stable yet)
    pub fn swap_registers(out_regs: *mut Registers, in_regs: *const Registers);
}

#[repr(C)]
#[derive(Debug)]
#[allow(improper_ctypes)]
pub struct Registers {
    // array containing all non-volatile registers. in order:
    // 0:    lr
    // 1:    cr
    // 2:    fp
    // 3:    toc (r2)
    // 4:    r12
    // 5-22: r14-r31
    // we use r14 and r15 to store the parameters when initialising a call frame.
    // r16 is used to pass the entry point addres (GEP) of the bootstrap function.
    // similar to the x86_64 implementation
    gpr: [usize; 32],

    // all non-volatile floating point registers (14-31)
    fp: [f64; 18],

    // all non-volatile vector registers (128Bit, 20-31)
    vr: [u128; 12], // f128 is not stable on ppc64le in rust
                    // and since these are never accessed in rust, just use u128
                    // to allocate the required memory.
}

// register indices:
const REG_LR: usize = 0;
// const REG_CR: usize = 1;
const REG_FP: usize = 2;
// const REG_TOC: usize = 3;
const REG_GLOB_ENTRY: usize = 4;
const REG_R14: usize = 5; // used to pass parameters on initialisation
const REG_R15: usize = 6; // used to pass parameters on initialistaion
const REG_R16: usize = 7; // used to pass parameters on initialisation

impl Registers {
    pub fn new() -> Self {
        Self {
            gpr: [0; 32],
            fp: [0.0; 18],
            vr: [0; 12],
        }
    }

    pub fn prefetch(&self) {
        unsafe {
            prefetch(&self.gpr[0]);
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
    // stack grows towards lower addresses (downward)
    let end = stack.end();
    let sp = align_down(end);
    let sp = mut_offset(sp, -2); // allow for back chain and CR save word

    regs.gpr[REG_FP] = sp as usize;

    // parameters passed in non-volatile regs and moved to ABI regs in bootstrap_green_task
    regs.gpr[REG_R14] = arg;
    regs.gpr[REG_R15] = arg2 as usize;
    regs.gpr[REG_R16] = fptr as usize;

    regs.gpr[REG_LR] = bootstrap_green_task as usize;
    regs.gpr[REG_GLOB_ENTRY] = bootstrap_green_task as usize;
}
