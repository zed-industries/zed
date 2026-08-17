use crate::detail::align_down;
use crate::stack::Stack;

cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        std::arch::global_asm!(include_str!("asm/asm_aarch64_aapcs_macho.S"));
    } else if #[cfg(target_os = "ios")] {
        std::arch::global_asm!(include_str!("asm/asm_aarch64_aapcs_macho.S"));
    } else {
        std::arch::global_asm!(include_str!("asm/asm_aarch64_aapcs_elf.S"));
    }
}

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

#[repr(C)]
#[derive(Debug)]
pub struct Registers {
    // We save the 13 callee-saved registers:
    //  x19--x28, fp (x29), lr (x30), sp
    // and the 8 callee-saved floating point registers:
    //  d8--d15
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
    // Callee-saved registers start at x19
    const X19: usize = 19 - 19;
    const X20: usize = 20 - 19;
    const X21: usize = 21 - 19;

    const FP: usize = 29 - 19;
    const LR: usize = 30 - 19;
    const SP: usize = 31 - 19;

    let sp = align_down(stack.end());

    // These registers are frobbed by bootstrap_green_task into the right
    // location so we can invoke the "real init function", `fptr`.
    regs.gpr[X19] = arg;
    regs.gpr[X20] = arg2 as usize;
    regs.gpr[X21] = fptr as usize;

    // Aarch64 current stack frame pointer
    regs.gpr[FP] = sp as usize;

    regs.gpr[LR] = bootstrap_green_task as usize;

    // setup the init stack
    // this is prepared for the swap context
    regs.gpr[SP] = sp as usize;
}
