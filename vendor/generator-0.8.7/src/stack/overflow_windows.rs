use crate::rt::{guard, Context, ContextStack};
use std::sync::Once;
use windows::Win32::Foundation::EXCEPTION_STACK_OVERFLOW;
use windows::Win32::System::Diagnostics::Debug::{
    AddVectoredExceptionHandler, CONTEXT, EXCEPTION_POINTERS,
};

unsafe extern "system" fn vectored_handler(exception_info: *mut EXCEPTION_POINTERS) -> i32 {
    const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
    const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;

    let info = &*exception_info;
    let rec = &(*info.ExceptionRecord);
    let context = &mut (*info.ContextRecord);

    if rec.ExceptionCode == EXCEPTION_STACK_OVERFLOW
        && guard::current().contains(&(context.Rsp as usize))
    {
        eprintln!(
            "\ncoroutine in thread '{}' has overflowed its stack\n",
            std::thread::current().name().unwrap_or("<unknown>")
        );

        let env = ContextStack::current();
        let cur = env.top();
        cur.err = Some(Box::new(crate::Error::StackErr));

        context_init(env.pop_context(cur as *mut _), context);

        //yield_now();

        EXCEPTION_CONTINUE_EXECUTION
    } else {
        EXCEPTION_CONTINUE_SEARCH
    }
}

unsafe fn init() {
    AddVectoredExceptionHandler(1, Some(vectored_handler));
}

pub fn init_once() {
    static INIT_ONCE: Once = Once::new();

    INIT_ONCE.call_once(|| unsafe {
        init();
    })
}

#[cfg(target_arch = "x86_64")]
unsafe fn context_init(parent: &mut Context, context: &mut CONTEXT) {
    let [rbx, rsp, rbp, _, r12, r13, r14, r15, _, _, _, stack_base, stack_limit, dealloc_stack, ..] =
        parent.regs.regs.gpr;

    let rip = *(rsp as *const usize);
    let rsp = rsp + std::mem::size_of::<usize>();

    context.Rbx = rbx as u64;
    context.Rsp = rsp as u64;
    context.Rbp = rbp as u64;
    context.R12 = r12 as u64;
    context.R13 = r13 as u64;
    context.R14 = r14 as u64;
    context.R15 = r15 as u64;
    context.Rip = rip as u64;

    let teb: usize;

    unsafe {
        std::arch::asm!(
        "mov {0}, gs:[0x30]",
        out(reg) teb
        );
    }

    *((teb + 0x08) as *mut usize) = stack_base;
    *((teb + 0x10) as *mut usize) = stack_limit;
    *((teb + 0x1478) as *mut usize) = dealloc_stack;
}
