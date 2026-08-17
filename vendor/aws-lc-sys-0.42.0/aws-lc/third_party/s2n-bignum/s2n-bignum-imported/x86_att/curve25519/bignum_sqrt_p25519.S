// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square root modulo p_25519 = 2^255 - 19
// Input x[4]; output function return (Legendre symbol) and z[4]
//
// extern int64_t bignum_sqrt_p25519(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Given a 4-digit input x, returns a modular square root mod p_25519, i.e.
// a z such that z^2 == x (mod p_25519), whenever one exists. The square
// root z is chosen so that its LSB is even (note that p_25519 - z is
// another square root). The function return is the Legendre/Jacobi symbol
// (x//p_25519), which indicates whether indeed x has a modular square root
// and hence whether the result is meaningful:
//
//   0: x is divisible by p_25519 and z is the square root 0
//  +1: x is coprime to p_25519 and z is a square root
//  -1: x is coprime to p_25519 but not a quadratic residue
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqrt_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqrt_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqrt_p25519)


        .text

// Size in bytes of a 64-bit word

#define N 8

// Pointer-offset pairs for temporaries on stack

#define a 0(%rsp)
#define b (4*N)(%rsp)
#define s (8*N)(%rsp)
#define t (12*N)(%rsp)
#define u (16*N)(%rsp)
#define res  (20*N)(%rsp)

// Total size to reserve on the stack

#define NSPACE 22*N

// Corrupted versions when stack is down 8 more

#define u8 (17*N)(%rsp)

// Syntactic variants to make x86_att version simpler to generate

#define A 0
#define B (4*N)
#define S (8*N)
#define T (12*N)
#define U (16*N)
#define U8 (17*N)

S2N_BN_SYMBOL(bignum_sqrt_p25519):
        CFI_START
        _CET_ENDBR

// In this case the Windows form literally makes a subroutine call.
// This avoids hassle arising from subroutine offsets

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        CFI_CALL(Lbignum_sqrt_p25519_standard)
        CFI_POP(%rsi)
        CFI_POP(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqrt_p25519)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_sqrt_p25519_standard)

Lbignum_sqrt_p25519_standard:
        CFI_START
#endif

// Save registers and make room for temporaries

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

// Save the return pointer for the end so we can overwrite %rdi later

        movq    %rdi, res

// Set up reduced version of the input argument a = x mod p_25519. Then
// get the candidate square root s = a^{252-2}

        movq    (%rsi), %rdx
        movq    0x8(%rsi), %rcx
        movq    0x10(%rsi), %r8
        movq    0x18(%rsi), %r9
        movl    $0x1, %eax
        xorl    %r10d, %r10d
        bts     $0x3f, %r9
        adcq    %r10, %rax
        imulq   $0x13, %rax, %rax
        addq    %rax, %rdx
        adcq    %r10, %rcx
        adcq    %r10, %r8
        adcq    %r10, %r9
        movl    $0x13, %eax
        cmovbq  %r10, %rax
        subq    %rax, %rdx
        sbbq    %r10, %rcx
        sbbq    %r10, %r8
        sbbq    %r10, %r9
        btr     $0x3f, %r9
        movq    %rdx, A(%rsp)
        movq    %rcx, A+0x8(%rsp)
        movq    %r8, A+0x10(%rsp)
        movq    %r9, A+0x18(%rsp)

  // Power 2^2 - 1 = 3

        leaq    T(%rsp), %rdi
        movq    $1, %rsi
        leaq    A(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    T(%rsp), %rsi
        leaq    A(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^4 - 1 = 15

        leaq    S(%rsp), %rdi
        movq    $2, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^5 - 1 = 31

        leaq    S(%rsp), %rdi
        movq    $1, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    B(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    A(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^10 - 1

        leaq    S(%rsp), %rdi
        movq    $5, %rsi
        leaq    B(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    B(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^20 - 1

        leaq    S(%rsp), %rdi
        movq    $10, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^25 - 1

        leaq    S(%rsp), %rdi
        movq    $5, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    B(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    B(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^50 - 1

        leaq    S(%rsp), %rdi
        movq    $25, %rsi
        leaq    B(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    B(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^100 - 1

        leaq    S(%rsp), %rdi
        movq    $50, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^125 - 1

        leaq    S(%rsp), %rdi
        movq    $25, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    B(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    B(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^250 - 1

        leaq    S(%rsp), %rdi
        movq    $125, %rsi
        leaq    B(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    B(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    B(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^251 - 1

        leaq    S(%rsp), %rdi
        movq    $1, %rsi
        leaq    B(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    A(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

  // Power 2^252 - 2

        leaq    S(%rsp), %rdi
        movq    $1, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

// s is now one candidate square root. Generate the other one t = s * j_25519

        movq    $0xc4ee1b274a0ea0b0, %rax
        movq    %rax, T(%rsp)
        movq    $0x2f431806ad2fe478, %rax
        movq    %rax, T+8(%rsp)
        movq    $0x2b4d00993dfbd7a7, %rax
        movq    %rax, T+16(%rsp)
        movq    $0x2b8324804fc1df0b, %rax
        movq    %rax, T+24(%rsp)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_mul_p25519)

// Now multiplex between them according to whether s^2 = a

        leaq    B(%rsp), %rdi
        movq    $1, %rsi
        leaq    S(%rsp), %rdx
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        movq    A(%rsp), %rax
        xorq    B(%rsp), %rax
        movq    A+8(%rsp), %rbx
        xorq    B+8(%rsp), %rbx
        orq     %rbx, %rax
        movq    A+16(%rsp), %rcx
        xorq    B+16(%rsp), %rcx
        movq    A+24(%rsp), %rdx
        xorq    B+24(%rsp), %rdx
        orq     %rdx, %rcx
        orq     %rcx, %rax

        movq    S(%rsp), %rax
        movq    T(%rsp), %rbx
        cmovnzq %rbx, %rax
        movq    S+8(%rsp), %rbx
        movq    T+8(%rsp), %rcx
        cmovnzq %rcx, %rbx
        movq    S+16(%rsp), %rcx
        movq    T+16(%rsp), %rdx
        cmovnzq %rdx, %rcx
        movq    S+24(%rsp), %rbp
        movq    T+24(%rsp), %rdx
        cmovnzq %rdx, %rbp

// For definiteness, choose "positive" (LSB=0) square root

        xorl    %edx, %edx
        leaq    -19(%rdx), %r8
        leaq    -1(%rdx), %r11
        movq    %r11, %r9
        movq    %r11, %r10
        btr     $63, %r11

        subq    %rax, %r8
        sbbq    %rbx, %r9
        sbbq    %rcx, %r10
        sbbq    %rbp, %r11

        movq    res, %rdx
        testq   $1, %rax
        cmovnzq %r8, %rax
        movq    %rax, (%rdx)
        cmovnzq %r9, %rbx
        movq    %rbx, 8(%rdx)
        cmovnzq %r10, %rcx
        movq    %rcx, 16(%rdx)
        cmovnzq %r11, %rbp
        movq    %rbp, 24(%rdx)

// Determine if it is is indeed a square root and also if a = 0
// Hence return the Legendre-Jacobi symbol as required.

        leaq    B(%rsp), %rdi
        movq    $1, %rsi
        CFI_CALL(Lbignum_sqrt_p25519_nsqr_p25519)

        movq    A(%rsp), %rax
        movq    %rax, %rbp
        xorq    B(%rsp), %rax
        movq    A+8(%rsp), %rbx
        orq     %rbx, %rbp
        xorq    B+8(%rsp), %rbx
        orq     %rbx, %rax
        movq    A+16(%rsp), %rcx
        orq     %rcx, %rbp
        xorq    B+16(%rsp), %rcx
        movq    A+24(%rsp), %rdx
        orq     %rdx, %rbp
        xorq    B+24(%rsp), %rdx
        orq     %rdx, %rcx
        orq     %rcx, %rax
        negq    %rax
        sbbq    %rax, %rax
        leaq    1(%rax,%rax,1), %rax

        testq   %rbp, %rbp
        cmovzq  %rbp, %rax

// Restore stack and registers

        CFI_INC_RSP(NSPACE)

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

#if WINDOWS_ABI
S2N_BN_SIZE_DIRECTIVE(Lbignum_sqrt_p25519_standard)
#else
S2N_BN_SIZE_DIRECTIVE(bignum_sqrt_p25519)
#endif

// *************************************************************
// Local z = x * y
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_sqrt_p25519_mul_p25519)

Lbignum_sqrt_p25519_mul_p25519:
        CFI_START
        movq    %rdx, %rcx
        xorl    %ebp, %ebp
        movq    (%rcx), %rdx
        mulxq   (%rsi), %r8, %r9
        mulxq   0x8(%rsi), %rax, %r10
        addq    %rax, %r9
        mulxq   0x10(%rsi), %rax, %r11
        adcq    %rax, %r10
        mulxq   0x18(%rsi), %rax, %r12
        adcq    %rax, %r11
        adcq    %rbp, %r12
        xorl    %ebp, %ebp
        movq    0x8(%rcx), %rdx
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x18(%rsi), %rax, %r13
        adcxq   %rax, %r12
        adoxq   %rbp, %r13
        adcq    %rbp, %r13
        xorl    %ebp, %ebp
        movq    0x10(%rcx), %rdx
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x18(%rsi), %rax, %r14
        adcxq   %rax, %r13
        adoxq   %rbp, %r14
        adcq    %rbp, %r14
        xorl    %ebp, %ebp
        movq    0x18(%rcx), %rdx
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x18(%rsi), %rcx, %r15
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        movl    $0x26, %edx
        mulxq   %r15, %rax, %rbx
        adcxq   %rcx, %r14
        adoxq   %rbp, %r15
        adcq    %rbp, %r15
        addq    %r11, %rax
        adcq    %rbp, %rbx
        btq     $0x3f, %rax
        adcq    %rbx, %rbx
        leaq    0x1(%rbx), %rcx
        imulq   $0x13, %rcx, %rcx
        xorl    %ebp, %ebp
        adoxq   %rcx, %r8
        mulxq   %r12, %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   %r13, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r14, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   %r15, %rax, %rbx
        adcq    %rax, %r11
        shlq    $0x3f, %rcx
        cmpq    %rcx, %r11
        movl    $0x13, %eax
        cmovns  %rbp, %rax
        subq    %rax, %r8
        sbbq    %rbp, %r9
        sbbq    %rbp, %r10
        sbbq    %rbp, %r11
        btr     $0x3f, %r11
        movq    %r8, (%rdi)
        movq    %r9, 0x8(%rdi)
        movq    %r10, 0x10(%rdi)
        movq    %r11, 0x18(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_sqrt_p25519_mul_p25519)

// *************************************************************
// Local z = 2^n * x
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_sqrt_p25519_nsqr_p25519)

Lbignum_sqrt_p25519_nsqr_p25519:
        CFI_START

// Copy input argument into u

        movq    (%rdx), %rax
        movq    8(%rdx), %rbx
        movq    16(%rdx), %rcx
        movq    24(%rdx), %rdx
        movq    %rax, U8(%rsp)
        movq    %rbx, U8+8(%rsp)
        movq    %rcx, U8+16(%rsp)
        movq    %rdx, U8+24(%rsp)

// Main squaring loop, accumulating in u consistently  and
// only ensuring the intermediates are < 2 * p_25519 = 2^256 - 38

Lbignum_sqrt_p25519_loop:
        movq   U8(%rsp), %rdx
        mulxq  %rdx, %r8, %r15
        mulxq  U8+0x8(%rsp), %r9, %r10
        mulxq  U8+0x18(%rsp), %r11, %r12
        movq   U8+0x10(%rsp), %rdx
        mulxq  U8+0x18(%rsp), %r13, %r14
        xorl   %ebx, %ebx
        mulxq  U8(%rsp), %rax, %rcx
        adcxq  %rax, %r10
        adoxq  %rcx, %r11
        mulxq  U8+0x8(%rsp), %rax, %rcx
        adcxq  %rax, %r11
        adoxq  %rcx, %r12
        movq   U8+0x18(%rsp), %rdx
        mulxq  U8+0x8(%rsp), %rax, %rcx
        adcxq  %rax, %r12
        adoxq  %rcx, %r13
        adcxq  %rbx, %r13
        adoxq  %rbx, %r14
        adcq   %rbx, %r14
        xorl   %ebx, %ebx
        adcxq  %r9, %r9
        adoxq  %r15, %r9
        movq   U8+0x8(%rsp), %rdx
        mulxq  %rdx, %rax, %rdx
        adcxq  %r10, %r10
        adoxq  %rax, %r10
        adcxq  %r11, %r11
        adoxq  %rdx, %r11
        movq   U8+0x10(%rsp), %rdx
        mulxq  %rdx, %rax, %rdx
        adcxq  %r12, %r12
        adoxq  %rax, %r12
        adcxq  %r13, %r13
        adoxq  %rdx, %r13
        movq   U8+0x18(%rsp), %rdx
        mulxq  %rdx, %rax, %r15
        adcxq  %r14, %r14
        adoxq  %rax, %r14
        adcxq  %rbx, %r15
        adoxq  %rbx, %r15
        movl   $0x26, %edx
        xorl   %ebx, %ebx
        mulxq  %r12, %rax, %rcx
        adcxq  %rax, %r8
        adoxq  %rcx, %r9
        mulxq  %r13, %rax, %rcx
        adcxq  %rax, %r9
        adoxq  %rcx, %r10
        mulxq  %r14, %rax, %rcx
        adcxq  %rax, %r10
        adoxq  %rcx, %r11
        mulxq  %r15, %rax, %r12
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        adcxq  %rbx, %r12
        shldq  $0x1, %r11, %r12
        btr    $0x3f, %r11
        movl   $0x13, %edx
        imulq  %r12, %rdx
        addq   %rdx, %r8
        adcq   %rbx, %r9
        adcq   %rbx, %r10
        adcq   %rbx, %r11
        movq   %r8, U8(%rsp)
        movq   %r9, U8+0x8(%rsp)
        movq   %r10, U8+0x10(%rsp)
        movq   %r11, U8+0x18(%rsp)

// Loop as applicable

        decq    %rsi
        jnz     Lbignum_sqrt_p25519_loop

// We know the intermediate result x < 2^256 - 38, and now we do strict
// modular reduction mod 2^255 - 19. Note x < 2^255 - 19 <=> x + 19 < 2^255
// which is equivalent to a "ns" condition. We just use the results where
// they were in registers [%r11;%r10;%r9;%r8] instead of re-loading them.

        movl    $19, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        addq    %r8, %rax
        adcq    %r9, %rbx
        adcq    %r10, %rcx
        adcq    %r11, %rdx

        cmovns  %r8, %rax
        cmovns  %r9, %rbx
        cmovns  %r10, %rcx
        cmovns  %r11, %rdx
        btr     $63, %rdx
        movq    %rax, (%rdi)
        movq    %rbx, 8(%rdi)
        movq    %rcx, 16(%rdi)
        movq    %rdx, 24(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_sqrt_p25519_nsqr_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
