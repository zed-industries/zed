// GREGS_OFFSET = 184
// REGISTER_SIZE = 8
// SIMD_REGISTER_SIZE = 16

std::arch::global_asm! {
    ".text",
    ".global crash_context_getcontext",
    ".hidden crash_context_getcontext",
    ".type crash_context_getcontext, #function",
    ".align 4",
    ".cfi_startproc",
"crash_context_getcontext:",

    // The saved context will return to the getcontext() call point
    // with a return value of 0
    "str     xzr,      [x0, 184]", // GREGS_OFFSET

    "stp     x18, x19, [x0, 328]", // GREGS_OFFSET + 18 * REGISTER_SIZE
    "stp     x20, x21, [x0, 344]", // GREGS_OFFSET + 20 * REGISTER_SIZE
    "stp     x22, x23, [x0, 360]", // GREGS_OFFSET + 22 * REGISTER_SIZE
    "stp     x24, x25, [x0, 376]", // GREGS_OFFSET + 24 * REGISTER_SIZE
    "stp     x26, x27, [x0, 392]", // GREGS_OFFSET + 26 * REGISTER_SIZE
    "stp     x28, x29, [x0, 408]", // GREGS_OFFSET + 28 * REGISTER_SIZE
    "str     x30,      [x0, 424]", // GREGS_OFFSET + 30 * REGISTER_SIZE

    // Place LR into the saved PC, this will ensure that when switching to this
    // saved context with setcontext() control will pass back to the caller of
    // getcontext(), we have already arranged to return the appropriate return
    // value in x0 above.
    "str     x30, [x0, 440]",

    // Save the current SP
    "mov     x2, sp",
    "str     x2, [x0, 432]",

    // Initialize the pstate.
    "str     xzr, [x0, 448]",

    // Figure out where to place the first context extension block.
    "add     x2, x0, #464",

    // Write the context extension fpsimd header.
    // FPSIMD_MAGIC = 0x46508001
    "mov     w3, #(0x46508001 & 0xffff)",
    "movk    w3, #(0x46508001 >> 16), lsl #16",
    "str     w3, [x2, #0]", // FPSIMD_CONTEXT_MAGIC_OFFSET
    "mov     w3, #528", // FPSIMD_CONTEXT_SIZE
    "str     w3, [x2, #4]", // FPSIMD_CONTEXT_SIZE_OFFSET

    // Fill in the FP SIMD context.
    "add     x3, x2, #144", // VREGS_OFFSET + 8 * SIMD_REGISTER_SIZE
    "stp     d8,  d9, [x3], #32",
    "stp     d10, d11, [x3], #32",
    "stp     d12, d13, [x3], #32",
    "stp     d14, d15, [x3], #32",

    "add     x3, x2, 8", // FPSR_OFFSET

    "mrs     x4, fpsr",
    "str     w4, [x3]",

    "mrs     x4, fpcr",
    "str     w4, [x3, 4]", // FPCR_OFFSET - FPSR_OFFSET

    // Write the termination context extension header.
    "add     x2, x2, #528", // FPSIMD_CONTEXT_SIZE

    "str     xzr, [x2, #0]", // FPSIMD_CONTEXT_MAGIC_OFFSET
    "str     xzr, [x2, #4]", // FPSIMD_CONTEXT_SIZE_OFFSET

    // Grab the signal mask
    // rt_sigprocmask (SIG_BLOCK, NULL, &ucp->uc_sigmask, _NSIG8)
    "add     x2, x0, #40", // UCONTEXT_SIGMASK_OFFSET
    "mov     x0, #0",  // SIG_BLOCK
    "mov     x1, #0",  // NULL
    "mov     x3, #(64 / 8)", // _NSIG / 8
    "mov     x8, #135", // __NR_rt_sigprocmask
    "svc     0",

    // Return x0 for success
    "mov     x0, 0",
    "ret",

    ".cfi_endproc",
    ".size crash_context_getcontext, . - crash_context_getcontext",
}
