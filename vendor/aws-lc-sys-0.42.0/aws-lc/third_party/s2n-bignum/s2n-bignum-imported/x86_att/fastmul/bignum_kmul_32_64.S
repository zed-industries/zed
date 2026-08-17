// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[32], y[32]; output z[64]; temporary buffer t[>=96]
//
//    extern void bignum_kmul_32_64(uint64_t z[static 64],
//                                  const uint64_t x[static 32],
//                                  const uint64_t y[static 32],
//                                  uint64_t t[static 96]);
//
// This is a Karatsuba-style function multiplying half-sized results
// internally and using temporary buffer t for intermediate results. The size
// of 96 is an overstatement for compatibility with the ARM version; it
// actually only uses 65 elements of t (64 + 1 for a stashed sign).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y, RCX = t
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y, R9 = t
// -----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_kmul_32_64)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_kmul_32_64)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_kmul_32_64)


        .text

#define K 16

#define z %rdi
#define x %rsi
#define y %rcx

#define s %r9

// We re-use the y variable to point at t later on, when this seems clearer

#define t %rcx

S2N_BN_SYMBOL(bignum_kmul_32_64):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
#endif

// Save callee-saved registers and also push t onto the stack; we'll
// use this space to back up both t and later z. Then move the y variable
// into its longer-term home for the first few stages.

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_PUSH(%rcx)
        movq    %rdx, y

// Multiply the low halves

        CFI_CALL(Lbignum_kmul_32_64_local_bignum_kmul_16_32)

// Multiply the high halves

        leaq    16*K-0x40(%rdi), %rdi
        leaq    8*K-0x40(%rsi), %rsi
        leaq    8*K(%rcx), %rcx
        CFI_CALL(Lbignum_kmul_32_64_local_bignum_kmul_16_32)

// Establish %r8 as the t pointer and use the cell to back up z now

        movq    (%rsp), %r8
        subq    $16*K+0x40, %rdi
        movq    %rdi, (%rsp)

// Form |x_lo - x_hi| starting at t

        movq    -8*K-0x40(%rsi), %rax
        subq    -8*K-0x40+8*K(%rsi), %rax
        movq    %rax, (%r8)
        .set I,  1
        .rep K-1
        movq    -8*K-0x40+8*I(%rsi), %rax
        sbbq    -8*K-0x40+8*K+8*I(%rsi), %rax
        movq    %rax, 8*I(%r8)
        .set I,  (I+1)
        .endr

        movl    $0, %ebx
        sbbq    s, s // Maintain CF, set ZF for cmovs, record sign

        .set I,  0
        .rep K
        movq    8*I(%r8), %rdx
        movq    %rdx, %rax
        notq    %rdx
        cmovzq  %rax, %rdx
        adcxq   %rbx, %rdx
        movq    %rdx, 8*I(%r8)
        .set I,  (I+1)
        .endr

// Form |y_hi - y_lo| (note opposite order) starting at t[K]

        movq    -8*K+8*K(%rcx), %rax
        subq    -8*K(%rcx), %rax
        movq    %rax, 8*K(%r8)
        .set I,  1
        .rep K-1
        movq    -8*K+8*K+8*I(%rcx), %rax
        sbbq    -8*K+8*I(%rcx), %rax
        movq    %rax, 8*K+8*I(%r8)
        .set I,  (I+1)
        .endr

        movl    $0, %ebx
        sbbq    %rbp, %rbp // Maintain CF, set ZF for cmovs

        .set I,  0
        .rep K
        movq    8*K+8*I(%r8), %rdx
        movq    %rdx, %rax
        notq    %rdx
        cmovzq  %rax, %rdx
        adcxq   %rbx, %rdx
        movq    %rdx, 8*K+8*I(%r8)
        .set I,  (I+1)
        .endr

// Stash the final sign with which to add things at t[4*K]

        xorq    %rbp, s
        movq    s, 32*K(%r8)

// Multiply the absolute differences, putting the result at t[2*K]
// This has the side-effect of putting t in the "right" register %rcx
// so after the load of z, we have both z and t pointers straight.

        movq    %r8, %rcx
        leaq    8*K(%r8), %rsi
        leaq    16*K(%r8), %rdi
        CFI_CALL(Lbignum_kmul_32_64_local_bignum_kmul_16_32)
        movq    (%rsp), z

// Compose the middle parts [2,1] + [1,0] + [3,2], saving carry in %rbx.
// Put the sum at t, overwriting the absolute differences we no longer need.

        xorl    %ebx, %ebx
        .set I,  0
        .rep 2*K
        movq    8*K+8*I(z), %rax
        adcxq   8*I(z), %rax
        adoxq   16*K+8*I(z), %rax
        movq    %rax, 8*I(t)
        .set I,  (I+1)
        .endr
        adoxq   %rbx, %rbx
        adcq    $0, %rbx

// Sign-aware addition or subtraction of the complicated term.
// We double-negate it to set CF/ZF while not spoiling its
// actual form: note that we eventually adcx to it below.

        movq    32*K(t), s
        negq    s
        negq    s

        .set I,  0
        .rep 2*K
        movq    16*K+8*I(t), %rdx
        movq    %rdx, %rax
        notq    %rdx
        cmovzq  %rax, %rdx
        adcxq   8*I(t), %rdx
        movq    %rdx, 8*K+8*I(z)
        .set I,  (I+1)
        .endr

// Bump the accumulated carry. This must end up >= 0 because it's the top
// word of a value of the form ... + h * h' + l * l' - (h - l) * (h' - l') >= 0

        adcxq   s, %rbx

// Finally propagate the carry to the top part

        xorl    %eax, %eax
        addq    %rbx, 24*K(z)
        .set I,  1
        .rep K-1
        adcq    %rax, 24*K+8*I(z)
        .set I,  (I+1)
        .endr

// Restore and return. The first pop is not needed for the ABI but
// we need to adjust the stack anyway so it seems reasonable.

        CFI_POP(%rcx)

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

// Local copy of half-length subroutine. This has a slightly different
// interface, expecting y argument in %rcx directly, and not doing any
// save-restore of the other registers. It naturally moves z and x on by
// 0x40, which we compensate for when it is called by adjusting offsets.

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_kmul_32_64_local_bignum_kmul_16_32)

Lbignum_kmul_32_64_local_bignum_kmul_16_32:
        CFI_START
        movq   (%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %r9
        movq   %rax, (%rdi)
        mulxq  0x8(%rsi), %rax, %r10
        adcxq  %rax, %r9
        mulxq  0x10(%rsi), %rax, %r11
        adcxq  %rax, %r10
        mulxq  0x18(%rsi), %rax, %r12
        adcxq  %rax, %r11
        mulxq  0x20(%rsi), %rax, %r13
        adcxq  %rax, %r12
        mulxq  0x28(%rsi), %rax, %r14
        adcxq  %rax, %r13
        mulxq  0x30(%rsi), %rax, %r15
        adcxq  %rax, %r14
        mulxq  0x38(%rsi), %rax, %r8
        adcxq  %rax, %r15
        adcq   %rbp, %r8
        movq   0x8(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   %r9, 0x8(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x38(%rsi), %rax, %r9
        adcxq  %rax, %r8
        adoxq  %rbp, %r9
        adcq   %rbp, %r9
        movq   0x10(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   %r10, 0x10(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x38(%rsi), %rax, %r10
        adcxq  %rax, %r9
        adoxq  %rbp, %r10
        adcq   %rbp, %r10
        movq   0x18(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        movq   %r11, 0x18(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x38(%rsi), %rax, %r11
        adcxq  %rax, %r10
        adoxq  %rbp, %r11
        adcq   %rbp, %r11
        movq   0x20(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        movq   %r12, 0x20(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x38(%rsi), %rax, %r12
        adcxq  %rax, %r11
        adoxq  %rbp, %r12
        adcq   %rbp, %r12
        movq   0x28(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        movq   %r13, 0x28(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x38(%rsi), %rax, %r13
        adcxq  %rax, %r12
        adoxq  %rbp, %r13
        adcq   %rbp, %r13
        movq   0x30(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        movq   %r14, 0x30(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x38(%rsi), %rax, %r14
        adcxq  %rax, %r13
        adoxq  %rbp, %r14
        adcq   %rbp, %r14
        movq   0x38(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        movq   %r15, 0x38(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x38(%rsi), %rax, %r15
        adcxq  %rax, %r14
        adoxq  %rbp, %r15
        adcq   %rbp, %r15
        movq   0x40(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        movq   %r8, 0x40(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x38(%rsi), %rax, %r8
        adcxq  %rax, %r15
        adoxq  %rbp, %r8
        adcq   %rbp, %r8
        movq   0x48(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   %r9, 0x48(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x38(%rsi), %rax, %r9
        adcxq  %rax, %r8
        adoxq  %rbp, %r9
        adcq   %rbp, %r9
        movq   0x50(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   %r10, 0x50(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x38(%rsi), %rax, %r10
        adcxq  %rax, %r9
        adoxq  %rbp, %r10
        adcq   %rbp, %r10
        movq   0x58(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        movq   %r11, 0x58(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x38(%rsi), %rax, %r11
        adcxq  %rax, %r10
        adoxq  %rbp, %r11
        adcq   %rbp, %r11
        movq   0x60(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        movq   %r12, 0x60(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x38(%rsi), %rax, %r12
        adcxq  %rax, %r11
        adoxq  %rbp, %r12
        adcq   %rbp, %r12
        movq   0x68(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        movq   %r13, 0x68(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x38(%rsi), %rax, %r13
        adcxq  %rax, %r12
        adoxq  %rbp, %r13
        adcq   %rbp, %r13
        movq   0x70(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        movq   %r14, 0x70(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x38(%rsi), %rax, %r14
        adcxq  %rax, %r13
        adoxq  %rbp, %r14
        adcq   %rbp, %r14
        movq   0x78(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        movq   %r15, 0x78(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x38(%rsi), %rax, %r15
        adcxq  %rax, %r14
        adoxq  %rbp, %r15
        adcq   %rbp, %r15
        movq   %r8, 0x80(%rdi)
        movq   %r9, 0x88(%rdi)
        movq   %r10, 0x90(%rdi)
        movq   %r11, 0x98(%rdi)
        movq   %r12, 0xa0(%rdi)
        movq   %r13, 0xa8(%rdi)
        movq   %r14, 0xb0(%rdi)
        movq   %r15, 0xb8(%rdi)
        addq   $0x40, %rdi
        addq   $0x40, %rsi
        movq   (%rcx), %rdx
        xorl   %ebp, %ebp
        movq   (%rdi), %r8
        movq   0x8(%rdi), %r9
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        movq   %r8, (%rdi)
        movq   0x10(%rdi), %r10
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   0x18(%rdi), %r11
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   0x20(%rdi), %r12
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        movq   0x28(%rdi), %r13
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        movq   0x30(%rdi), %r14
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        movq   0x38(%rdi), %r15
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x38(%rsi), %rax, %r8
        adcxq  %rax, %r15
        adoxq  %rbp, %r8
        adcxq  %rbp, %r8
        movq   0x8(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   %r9, 0x8(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x38(%rsi), %rax, %r9
        adcxq  %rax, %r8
        adoxq  %rbp, %r9
        adcq   %rbp, %r9
        movq   0x10(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   %r10, 0x10(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x38(%rsi), %rax, %r10
        adcxq  %rax, %r9
        adoxq  %rbp, %r10
        adcq   %rbp, %r10
        movq   0x18(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        movq   %r11, 0x18(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x38(%rsi), %rax, %r11
        adcxq  %rax, %r10
        adoxq  %rbp, %r11
        adcq   %rbp, %r11
        movq   0x20(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        movq   %r12, 0x20(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x38(%rsi), %rax, %r12
        adcxq  %rax, %r11
        adoxq  %rbp, %r12
        adcq   %rbp, %r12
        movq   0x28(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        movq   %r13, 0x28(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x38(%rsi), %rax, %r13
        adcxq  %rax, %r12
        adoxq  %rbp, %r13
        adcq   %rbp, %r13
        movq   0x30(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        movq   %r14, 0x30(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x38(%rsi), %rax, %r14
        adcxq  %rax, %r13
        adoxq  %rbp, %r14
        adcq   %rbp, %r14
        movq   0x38(%rcx), %rdx
        xorl   %ebp, %ebp
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        movq   %r15, 0x38(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x38(%rsi), %rax, %r15
        adcxq  %rax, %r14
        adoxq  %rbp, %r15
        adcq   %rbp, %r15
        movq   0x40(%rcx), %rdx
        xorl   %ebp, %ebp
        adoxq  0x40(%rdi), %r8
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        movq   %r8, 0x40(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x38(%rsi), %rax, %r8
        adcxq  %rax, %r15
        adoxq  %rbp, %r8
        adcq   %rbp, %r8
        movq   0x48(%rcx), %rdx
        xorl   %ebp, %ebp
        adoxq  0x48(%rdi), %r9
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   %r9, 0x48(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x38(%rsi), %rax, %r9
        adcxq  %rax, %r8
        adoxq  %rbp, %r9
        adcq   %rbp, %r9
        movq   0x50(%rcx), %rdx
        xorl   %ebp, %ebp
        adoxq  0x50(%rdi), %r10
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   %r10, 0x50(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x38(%rsi), %rax, %r10
        adcxq  %rax, %r9
        adoxq  %rbp, %r10
        adcq   %rbp, %r10
        movq   0x58(%rcx), %rdx
        xorl   %ebp, %ebp
        adoxq  0x58(%rdi), %r11
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        movq   %r11, 0x58(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x38(%rsi), %rax, %r11
        adcxq  %rax, %r10
        adoxq  %rbp, %r11
        adcq   %rbp, %r11
        movq   0x60(%rcx), %rdx
        xorl   %ebp, %ebp
        adoxq  0x60(%rdi), %r12
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        movq   %r12, 0x60(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x38(%rsi), %rax, %r12
        adcxq  %rax, %r11
        adoxq  %rbp, %r12
        adcq   %rbp, %r12
        movq   0x68(%rcx), %rdx
        xorl   %ebp, %ebp
        adoxq  0x68(%rdi), %r13
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        movq   %r13, 0x68(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x38(%rsi), %rax, %r13
        adcxq  %rax, %r12
        adoxq  %rbp, %r13
        adcq   %rbp, %r13
        movq   0x70(%rcx), %rdx
        xorl   %ebp, %ebp
        adoxq  0x70(%rdi), %r14
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        movq   %r14, 0x70(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x38(%rsi), %rax, %r14
        adcxq  %rax, %r13
        adoxq  %rbp, %r14
        adcq   %rbp, %r14
        movq   0x78(%rcx), %rdx
        xorl   %ebp, %ebp
        adoxq  0x78(%rdi), %r15
        mulxq  (%rsi), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        movq   %r15, 0x78(%rdi)
        mulxq  0x8(%rsi), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x10(%rsi), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x18(%rsi), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x20(%rsi), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x28(%rsi), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x30(%rsi), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x38(%rsi), %rax, %r15
        adcxq  %rax, %r14
        adoxq  %rbp, %r15
        adcq   %rbp, %r15
        movq   %r8, 0x80(%rdi)
        movq   %r9, 0x88(%rdi)
        movq   %r10, 0x90(%rdi)
        movq   %r11, 0x98(%rdi)
        movq   %r12, 0xa0(%rdi)
        movq   %r13, 0xa8(%rdi)
        movq   %r14, 0xb0(%rdi)
        movq   %r15, 0xb8(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_kmul_32_64_local_bignum_kmul_16_32)

S2N_BN_SIZE_DIRECTIVE(bignum_kmul_32_64)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
