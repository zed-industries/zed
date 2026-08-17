use crate::detail::align_down;
use crate::stack::Stack;

std::arch::global_asm!(include_str!("asm/asm_loongarch64_sysv_elf.S"));

// first argument is task handle, second is thunk ptr
pub type InitFn = extern "C" fn(usize, *mut usize) -> !;

pub extern "C" fn gen_init(a1: usize, a2: *mut usize) -> ! {
    super::gen::gen_init_impl(a1, a2)
}

//#[link(name = "asm", kind = "static")]
extern "C" {
    pub fn bootstrap_green_task();
    pub fn prefetch(data: *const usize);
    pub fn swap_registers(out_regs: *mut Registers, in_regs: *const Registers);
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct Registers {
    // We save the 12 callee-saved registers:
    //  0: ra
    //  1: sp
    //  2: fp
    //  3: s0
    //  4: s1
    //  5: s2
    //  6: s3
    //  7: s4
    //  8: s5
    //  9: s6
    // 10: s7
    // 11: s8
    // and the 8 callee-saved floating point registers:
    // 12: fs0
    // 13: fs1
    // 14: fs2
    // 15: fs3
    // 16: fs4
    // 17: fs5
    // 18: fs6
    // 19: fs7
    gpr: [usize; 20],
}

impl Registers {
    pub fn new() -> Registers {
        Registers { gpr: [0; 20] }
    }

    #[inline]
    pub fn prefetch(&self) {
        let ptr = self.gpr[1] as *const usize;
        unsafe {
            prefetch(ptr); // SP
            prefetch(ptr.add(8)); // SP + 8
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
    const RA: usize = 0;
    const SP: usize = 1;
    const FP: usize = 2;
    const S0: usize = 3;
    const S1: usize = 4;
    const S2: usize = 5;

    let sp = align_down(stack.end());

    // These registers are frobbed by bootstrap_green_task into the right
    // location so we can invoke the "real init function", `fptr`.
    regs.gpr[S0] = arg;
    regs.gpr[S1] = arg2 as usize;
    regs.gpr[S2] = fptr as usize;

    // LoongArch64 current stack frame pointer
    regs.gpr[FP] = sp as usize;

    regs.gpr[RA] = bootstrap_green_task as usize;

    // setup the init stack
    // this is prepared for the swap context
    regs.gpr[SP] = sp as usize;
}
