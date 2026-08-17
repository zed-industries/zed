use crate::detail::{initialize_call_frame, swap_registers, InitFn, Registers};
use crate::stack::Stack;

#[derive(Debug)]
pub struct RegContext {
    /// Hold the registers while the task or scheduler is suspended
    pub(crate) regs: Registers,
}

impl RegContext {
    pub fn empty() -> RegContext {
        RegContext {
            regs: Registers::new(),
        }
    }

    #[inline]
    pub fn prefetch(&self) {
        self.regs.prefetch();
    }

    /// Create a new context, only used in tests
    #[cfg(test)]
    fn new(init: InitFn, arg: usize, start: *mut usize, stack: &Stack) -> RegContext {
        let mut ctx = RegContext::empty();
        ctx.init_with(init, arg, start, stack);
        ctx
    }

    /// init the generator register
    #[inline]
    pub fn init_with(&mut self, init: InitFn, arg: usize, start: *mut usize, stack: &Stack) {
        // Save and then immediately load the current context,
        // we will modify it to call the given function when restored back
        initialize_call_frame(&mut self.regs, init, arg, start, stack);
    }

    /// Switch contexts
    ///
    /// Suspend the current execution context and resume another by
    /// saving the registers values of the executing thread to a Context
    /// then loading the registers from a previously saved Context.
    #[inline]
    pub fn swap(out_context: &mut RegContext, in_context: &RegContext) {
        // debug!("register raw swap");
        unsafe { swap_registers(&mut out_context.regs, &in_context.regs) }
    }

    /// Load the context and switch. This function will never return.
    #[inline]
    #[cfg(test)]
    pub fn load(to_context: &RegContext) {
        let mut cur = Registers::new();
        let regs: &Registers = &to_context.regs;

        unsafe { swap_registers(&mut cur, regs) }
    }
}

#[cfg(test)]
mod test {
    use std::mem::transmute;

    use crate::reg_context::RegContext;
    use crate::stack::Stack;

    const MIN_STACK: usize = 1024;

    fn init_fn_impl(arg: usize, f: *mut usize) -> ! {
        let func: fn() = unsafe { transmute(f) };
        func();

        let ctx: &RegContext = unsafe { transmute(arg) };
        RegContext::load(ctx);

        unreachable!("Should never comeback");
    }

    #[cfg(target_arch = "x86_64")]
    extern "sysv64" fn init_fn(arg: usize, f: *mut usize) -> ! {
        init_fn_impl(arg, f)
    }

    #[cfg(target_arch = "aarch64")]
    extern "C" fn init_fn(arg: usize, f: *mut usize) -> ! {
        init_fn_impl(arg, f)
    }

    #[cfg(target_arch = "loongarch64")]
    extern "C" fn init_fn(arg: usize, f: *mut usize) -> ! {
        init_fn_impl(arg, f)
    }

    #[cfg(target_arch = "riscv64")]
    extern "C" fn init_fn(arg: usize, f: *mut usize) -> ! {
        init_fn_impl(arg, f)
    }

    #[cfg(target_arch = "powerpc64")]
    extern "C" fn init_fn(arg: usize, f: *mut usize) -> ! {
        init_fn_impl(arg, f)
    }

    #[cfg(target_arch = "arm")]
    extern "aapcs" fn init_fn(arg: usize, f: *mut usize) -> ! {
        init_fn_impl(arg, f)
    }

    #[test]
    fn test_swap_context() {
        static mut VAL: bool = false;
        let mut cur = RegContext::empty();

        fn callback() {
            unsafe { VAL = true };
        }

        let stk = Stack::new(MIN_STACK);
        let ctx = RegContext::new(
            init_fn,
            &cur as *const _ as usize,
            callback as *mut usize,
            &stk,
        );

        RegContext::swap(&mut cur, &ctx);
        unsafe {
            assert!(VAL);
        }
    }
}
