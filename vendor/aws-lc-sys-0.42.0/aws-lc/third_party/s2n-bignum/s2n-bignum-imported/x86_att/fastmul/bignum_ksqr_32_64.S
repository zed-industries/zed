// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[32]; output z[64]; temporary buffer t[>=72]
//
//    extern void bignum_ksqr_32_64(uint64_t z[static 64],
//                                  const uint64_t x[static 32],
//                                  uint64_t t[static 72]);
//
// This is a Karatsuba-style function squaring half-sized results
// and using temporary buffer t for intermediate results. The size of 72
// is an overstatement for compatibility with the ARM version; it actually
// only uses 65 elements of t (64 + 1 for a suspended carry).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = t
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_ksqr_32_64)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_ksqr_32_64)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_ksqr_32_64)

        .text

#define K 16

#define z %rdi
#define x %rsi
#define t %rcx

S2N_BN_SYMBOL(bignum_ksqr_32_64):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save callee-preserved registers once and for all at the outset
// Later we further reshuffle the input arguments to avoid extra saves

        CFI_PUSH(%rbp)
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Move the temp space pointer since we need %rdx for multiplications

        movq    %rdx, t

// Square the low half

        CFI_CALL(Lbignum_ksqr_32_64_local_bignum_sqr_16_32)

// Square the high half; from here on x and z are modified

        leaq    8*K(x), x // input at x+8*K
        leaq    16*K(z), z // result at z+16*K
        CFI_CALL(Lbignum_ksqr_32_64_local_bignum_sqr_16_32)

// Form |x_lo - x_hi|, stored at t

        movq    -8*K(x), %rax
        subq    (x), %rax
        movq    %rax, (t)
        .set I,  1
        .rep K-1
        movq    -8*K+8*I(x), %rax
        sbbq    8*I(x), %rax
        movq    %rax, 8*I(t)
        .set I,  (I+1)
        .endr

        movl    $0, %ebx
        sbbq    %rax, %rax // Maintain CF, set ZF for cmovs

        .set I,  0
        .rep K
        movq    8*I(t), %rdx
        movq    %rdx, %rax
        notq    %rdx
        adcxq   %rbx, %rdx
        cmovzq  %rax, %rdx
        movq    %rdx, 8*I(t)
        .set I,  (I+1)
        .endr

// Compose the middle parts [2,1] + [1,0] + [3,2]
// Put the low half of this at t[K] and the top half in place at z[2*K]; a
// fully in-place version is awkward with the otherwise beneficial double
// carry chain. Stash the carry suspended from the 3k position at the end of
// the temp buffer t[4*K].

        xorl    %edx, %edx
        .set I,  0
        .rep K
        movq    -16*K+8*K+8*I(z), %rax
        adcxq   -16*K+8*I(z), %rax
        adoxq   -16*K+16*K+8*I(z), %rax
        movq    %rax, 8*K+8*I(t)
        .set I,  (I+1)
        .endr

        .rep K
        movq    -16*K+8*K+8*I(z), %rax
        adcxq   -16*K+8*I(z), %rax
        adoxq   -16*K+16*K+8*I(z), %rax
        movq    %rax, -16*K+8*K+8*I(z)
        .set I,  (I+1)
        .endr

        adoxq   %rdx, %rdx
        adcq    $0, %rdx
        movq    %rdx, 32*K(t)

// Square the absolute difference, putting the result M at t[2*K].
// This involves another shuffle so now t' = z_orig and x' = t_orig
// while z' points within the temp buffer to the product M itself

        movq    t, x
        leaq    -16*K(z), t
        leaq    16*K(x), z
        CFI_CALL(Lbignum_ksqr_32_64_local_bignum_sqr_16_32)

// Subtract M, pausing at the 3k position to bump down accumulated carry.
// The carry cannot go negative since it's the top word of a value
// of the form ... + h^2 + l^2 - (h - l)^2 >= 0

        movq    8*K(x), %rax
        subq    (z), %rax
        movq    %rax, 8*K(t)

        .set I,  1

        .rep (K-1)
        movq    8*K+8*I(x), %rax
        sbbq    8*I(z), %rax
        movq    %rax, 8*K+8*I(t)
        .set I,  (I+1)
        .endr

        .rep K
        movq    8*K+8*I(t), %rax
        sbbq    8*I(z), %rax
        movq    %rax, 8*K+8*I(t)
        .set I,  (I+1)
        .endr

        movq    32*K(x), %rdx
        sbbq    $0, %rdx

// Finally propagate the carry to the top quarter

        xorl    %eax, %eax
        addq    %rdx, 24*K(t)
        .set I,  1
        .rep K-1
        adcq    %rax, 24*K+8*I(t)
        .set I,  (I+1)
        .endr

// Restore registers and return

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
        CFI_POP(%rbp)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

// Local copy of the half-length subroutine

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_ksqr_32_64_local_bignum_sqr_16_32)

Lbignum_ksqr_32_64_local_bignum_sqr_16_32:
        CFI_START
        xorl   %ebp, %ebp
        movq   (x), %rdx
        mulxq  0x8(x), %r9, %rax
        movq   %r9, 0x8(z)
        mulxq  0x10(x), %r10, %rbx
        adcxq  %rax, %r10
        movq   %r10, 0x10(z)
        mulxq  0x18(x), %r11, %rax
        adcxq  %rbx, %r11
        mulxq  0x20(x), %r12, %rbx
        adcxq  %rax, %r12
        mulxq  0x28(x), %r13, %rax
        adcxq  %rbx, %r13
        mulxq  0x30(x), %r14, %rbx
        adcxq  %rax, %r14
        mulxq  0x38(x), %r15, %r8
        adcxq  %rbx, %r15
        adcxq  %rbp, %r8
        xorl   %ebp, %ebp
        movq   0x8(x), %rdx
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        movq   %r11, 0x18(z)
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        movq   %r12, 0x20(z)
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x38(x), %rax, %r9
        adcxq  %rax, %r8
        adoxq  %rbp, %r9
        movq   0x20(x), %rdx
        mulxq  0x28(x), %rax, %r10
        adcxq  %rax, %r9
        adoxq  %rbp, %r10
        adcxq  %rbp, %r10
        xorl   %ebp, %ebp
        movq   0x10(x), %rdx
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        movq   %r13, 0x28(z)
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        movq   %r14, 0x30(z)
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x38(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   0x30(x), %rdx
        mulxq  0x20(x), %rax, %r11
        adcxq  %rax, %r10
        adoxq  %rbp, %r11
        mulxq  0x28(x), %rax, %r12
        adcxq  %rax, %r11
        adoxq  %rbp, %r12
        adcxq  %rbp, %r12
        xorl   %ebp, %ebp
        movq   0x18(x), %rdx
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        movq   %r15, 0x38(z)
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x38(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   0x38(x), %rdx
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x28(x), %rax, %r13
        adcxq  %rax, %r12
        adoxq  %rbp, %r13
        mulxq  0x30(x), %rax, %r14
        adcxq  %rax, %r13
        adoxq  %rbp, %r14
        adcxq  %rbp, %r14
        movq   0x40(x), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        movq   %r8, 0x40(z)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x30(x), %rax, %r15
        adcxq  %rax, %r14
        adoxq  %rbp, %r15
        mulxq  0x38(x), %rax, %r8
        adcxq  %rax, %r15
        adoxq  %rbp, %r8
        adcxq  %rbp, %r8
        movq   0x48(x), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   %r9, 0x48(z)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x38(x), %rax, %r9
        adcxq  %rax, %r8
        adoxq  %rbp, %r9
        adcxq  %rbp, %r9
        movq   0x50(x), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   %r10, 0x50(z)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x38(x), %rax, %r10
        adcxq  %rax, %r9
        adoxq  %rbp, %r10
        adcxq  %rbp, %r10
        movq   0x58(x), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        movq   %r11, 0x58(z)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x38(x), %rax, %r11
        adcxq  %rax, %r10
        adoxq  %rbp, %r11
        adcxq  %rbp, %r11
        movq   0x60(x), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        movq   %r12, 0x60(z)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x38(x), %rax, %r12
        adcxq  %rax, %r11
        adoxq  %rbp, %r12
        adcxq  %rbp, %r12
        movq   0x68(x), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        movq   %r13, 0x68(z)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x38(x), %rax, %r13
        adcxq  %rax, %r12
        adoxq  %rbp, %r13
        adcxq  %rbp, %r13
        movq   0x70(x), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        movq   %r14, 0x70(z)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x38(x), %rax, %r14
        adcxq  %rax, %r13
        adoxq  %rbp, %r14
        adcxq  %rbp, %r14
        movq   0x78(x), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        movq   %r15, 0x78(z)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x38(x), %rax, %r15
        adcxq  %rax, %r14
        adoxq  %rbp, %r15
        adcxq  %rbp, %r15
        movq   %r8, 0x80(z)
        movq   %r9, 0x88(z)
        movq   %r10, 0x90(z)
        movq   %r11, 0x98(z)
        movq   %r12, 0xa0(z)
        movq   %r13, 0xa8(z)
        movq   %r14, 0xb0(z)
        movq   %r15, 0xb8(z)
        xorl   %ebp, %ebp
        movq   0x88(z), %r9
        movq   0x90(z), %r10
        movq   0x98(z), %r11
        movq   0xa0(z), %r12
        movq   0xa8(z), %r13
        movq   0xb0(z), %r14
        movq   0xb8(z), %r15
        movq   0x40(x), %rdx
        mulxq  0x48(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   %r9, 0x88(z)
        mulxq  0x50(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   %r10, 0x90(z)
        mulxq  0x58(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x60(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x68(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x70(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x78(x), %rax, %r8
        adcxq  %rax, %r15
        adoxq  %rbp, %r8
        adcxq  %rbp, %r8
        xorl   %ebp, %ebp
        movq   0x48(x), %rdx
        mulxq  0x50(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        movq   %r11, 0x98(z)
        mulxq  0x58(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        movq   %r12, 0xa0(z)
        mulxq  0x60(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x68(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x70(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x78(x), %rax, %r9
        adcxq  %rax, %r8
        adoxq  %rbp, %r9
        movq   0x60(x), %rdx
        mulxq  0x68(x), %rax, %r10
        adcxq  %rax, %r9
        adoxq  %rbp, %r10
        adcxq  %rbp, %r10
        xorl   %ebp, %ebp
        movq   0x50(x), %rdx
        mulxq  0x58(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        movq   %r13, 0xa8(z)
        mulxq  0x60(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        movq   %r14, 0xb0(z)
        mulxq  0x68(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x70(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x78(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   0x70(x), %rdx
        mulxq  0x60(x), %rax, %r11
        adcxq  %rax, %r10
        adoxq  %rbp, %r11
        mulxq  0x68(x), %rax, %r12
        adcxq  %rax, %r11
        adoxq  %rbp, %r12
        adcxq  %rbp, %r12
        xorl   %ebp, %ebp
        movq   0x58(x), %rdx
        mulxq  0x60(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        movq   %r15, 0xb8(z)
        mulxq  0x68(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x70(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x78(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   0x78(x), %rdx
        mulxq  0x60(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x68(x), %rax, %r13
        adcxq  %rax, %r12
        adoxq  %rbp, %r13
        mulxq  0x70(x), %rax, %r14
        adcxq  %rax, %r13
        adoxq  %rbp, %r14
        adcxq  %rbp, %r14
        movq   %r8, 0xc0(z)
        movq   %r9, 0xc8(z)
        movq   %r10, 0xd0(z)
        movq   %r11, 0xd8(z)
        movq   %r12, 0xe0(z)
        movq   %r13, 0xe8(z)
        movq   %r14, 0xf0(z)
        xorl   %ebp, %ebp
        movq   (x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   %rax, (z)
        movq   0x8(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x8(z)
        movq   0x8(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0x10(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0x10(z)
        movq   0x18(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x18(z)
        movq   0x10(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0x20(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0x20(z)
        movq   0x28(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x28(z)
        movq   0x18(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0x30(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0x30(z)
        movq   0x38(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x38(z)
        movq   0x20(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0x40(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0x40(z)
        movq   0x48(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x48(z)
        movq   0x28(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0x50(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0x50(z)
        movq   0x58(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x58(z)
        movq   0x30(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0x60(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0x60(z)
        movq   0x68(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x68(z)
        movq   0x38(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0x70(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0x70(z)
        movq   0x78(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x78(z)
        movq   0x40(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0x80(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0x80(z)
        movq   0x88(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x88(z)
        movq   0x48(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0x90(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0x90(z)
        movq   0x98(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0x98(z)
        movq   0x50(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0xa0(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0xa0(z)
        movq   0xa8(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0xa8(z)
        movq   0x58(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0xb0(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0xb0(z)
        movq   0xb8(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0xb8(z)
        movq   0x60(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0xc0(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0xc0(z)
        movq   0xc8(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0xc8(z)
        movq   0x68(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0xd0(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0xd0(z)
        movq   0xd8(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0xd8(z)
        movq   0x70(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0xe0(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0xe0(z)
        movq   0xe8(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rbx, %rdx
        movq   %rdx, 0xe8(z)
        movq   0x78(x), %rdx
        mulxq  %rdx, %rax, %rbx
        movq   0xf0(z), %rdx
        adcxq  %rdx, %rdx
        adoxq  %rax, %rdx
        movq   %rdx, 0xf0(z)
        adcxq  %rbp, %rbx
        adoxq  %rbp, %rbx
        movq   %rbx, 0xf8(z)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_ksqr_32_64_local_bignum_sqr_16_32)

S2N_BN_SIZE_DIRECTIVE(bignum_ksqr_32_64)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
