/* The x64 implementation of breakpad_getcontext was derived in part
from the implementation of libunwind which requires the following
notice. */
/* libunwind - a platform-independent unwind library
   Copyright (C) 2008 Google, Inc
    Contributed by Paul Pluzhnikov <ppluzhnikov@google.com>
   Copyright (C) 2010 Konstantin Belousov <kib@freebsd.org>

This file is part of libunwind.

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.  */

// Oof, unfortunately android libc and normal linux libc don't have the same
// fpregs offset in ucontext_t, but asm! in Rust is...not convenient for only
// adding certain lines/blocks of asm based using cfg https://github.com/rust-lang/rust/issues/15701
// and they're not really inputs, just literals, so...yah

#[cfg(any(target_os = "linux", target_os = "android"))]

// Unfortunately, the asm! macro has a few really annoying limitations at the
// moment
//
// 1. const operands are unstable
// 2. cfg attributes can't be used inside the asm macro at all
//
// and the worst part is we need it for literally only 1 thing, using a different
// offset to the fpstate in ucontext depending on whether we are targeting android
// or not :(
macro_rules! asm_func {
    ($offset:expr) => {
        std::arch::global_asm! {
            ".text",
            ".global crash_context_getcontext",
            ".hidden crash_context_getcontext",
            ".align 4",
            ".type crash_context_getcontext, @function",
        "crash_context_getcontext:",
            ".cfi_startproc",
            // Callee saved: RBX, RBP, R12-R15
            "movq %r12, 0x48(%rdi)",
            "movq %r13, 0x50(%rdi)",
            "movq %r14, 0x58(%rdi)",
            "movq %r15, 0x60(%rdi)",
            "movq %rbp, 0x78(%rdi)",
            "movq %rbx, 0x80(%rdi)",

            // Save argument registers
            "movq %r8,  0x28(%rdi)",
            "movq %r9,  0x30(%rdi)",
            "movq %rdi, 0x68(%rdi)",
            "movq %rsi, 0x70(%rdi)",
            "movq %rdx, 0x88(%rdi)",
            "movq %rax, 0x90(%rdi)",
            "movq %rcx, 0x98(%rdi)",

            // Save fp state
            stringify!(leaq $offset(%rdi),%r8),
            "movq %r8, 0xe0(%rdi)",
            "fnstenv (%r8)",
            "stmxcsr 0x18(%r8)",

            // Exclude this call
            "leaq 8(%rsp), %rax",
            "movq %rax, 0xa0(%rdi)",

            "movq 0(%rsp), %rax",
            "movq %rax, 0xa8(%rdi)",

            // Save signal mask: sigprocmask(SIGBLOCK, NULL, &uc->uc_sigmask)
            "leaq 0x128(%rdi), %rdx",  // arg3
            "xorq %rsi, %rsi",  // arg2 NULL
            "xorq %rdi, %rdi",  // arg1 SIGBLOCK == 0
            "call sigprocmask@PLT",

            // Always return 0 for success, even if sigprocmask failed.
            "xorl %eax, %eax",
            "ret",
            ".cfi_endproc",
            ".size crash_context_getcontext, . - crash_context_getcontext",
            options(att_syntax)
        }
    };
}

#[cfg(target_os = "linux")]
asm_func!(0x1a8);
#[cfg(target_os = "android")]
asm_func!(0x130);
