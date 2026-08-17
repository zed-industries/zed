std::arch::global_asm! {
    ".text",
    ".global crash_context_getcontext",
    ".hidden crash_context_getcontext",
    ".type crash_context_getcontext, #function",
    ".align 0",
    ".fnstart",
"crash_context_getcontext:",

    // First, save r4-r11
    "add   r1, r0, #(32 + 4 * 4)",
    "stm   r1, {{r4-r11}}",

    // r12 is a scratch register, don't save it

    // Save sp and lr explicitly.
    // - sp can't be stored with stmia in Thumb-2
    // - STM instructions that store sp and pc are deprecated in ARM
    "str   sp, [r0, #(32 + 13 * 4)]",
    "str   lr, [r0, #(32 + 14 * 4)]",

    // Save the caller's address in 'pc'
    "str   lr, [r0, #(32 + 15 * 4)]",

    // Save ucontext_t* pointer across next call
    "mov   r4, r0",

    // Call sigprocmask(SIG_BLOCK, NULL, &(ucontext->uc_sigmask))
    "mov   r0, #0",  // SIG_BLOCK
    "mov   r1, #0",  // NULL
    "add   r2, r4, #104", // UCONTEXT_SIGMASK_OFFSET
    "bl    sigprocmask(PLT)",

    /* Intentionally do not save the FPU state here. This is because on
    * Linux/ARM, one should instead use ptrace(PTRACE_GETFPREGS) or
    * ptrace(PTRACE_GETVFPREGS) to get it.
    *
    * Note that a real implementation of getcontext() would need to save
    * this here to allow setcontext()/swapcontext() to work correctly.
    */

    // Restore the values of r4 and lr
    "mov   r0, r4",
    "ldr   lr, [r0, #(32 + 14 * 4)]",
    "ldr   r4, [r0, #(32 +  4 * 4)]",

    // Return 0
    "mov   r0, #0",
    "mov   pc, lr",

    ".fnend",
    ".size crash_context_getcontext, . - crash_context_getcontext",
}
