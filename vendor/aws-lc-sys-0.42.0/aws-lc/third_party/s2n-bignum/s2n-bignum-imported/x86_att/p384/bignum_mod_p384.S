// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_384
// Input x[k]; output z[6]
//
//    extern void bignum_mod_p384(uint64_t z[static 6], uint64_t k,
//                                const uint64_t *x);
//
// Standard x86-64 ABI: RDI = z, RSI = k, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = k, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p384)
        .text

#define z %rdi
#define k %rsi
#define x %rcx

#define m0 %r8
#define m1 %r9
#define m2 %r10
#define m3 %r11
#define m4 %r12
#define m5 %r13
#define d %r14

#define n0 %rax
#define n1 %rbx
#define n2 %rdx
#define q %rdx

#define n0short %eax
#define n1short %ebx
#define qshort %edx


S2N_BN_SYMBOL(bignum_mod_p384):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save extra registers

        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)

// If the input is already <= 5 words long, go to a trivial "copy" path

        cmpq    $6, k
        jc      Lbignum_mod_p384_shortinput

// Otherwise load the top 6 digits (top-down) and reduce k by 6

        subq    $6, k
        movq    40(%rdx,k,8), m5
        movq    32(%rdx,k,8), m4
        movq    24(%rdx,k,8), m3
        movq    16(%rdx,k,8), m2
        movq    8(%rdx,k,8), m1
        movq    (%rdx,k,8), m0

// Move x into another register to leave %rdx free for multiplies and use of n2

        movq    %rdx, x

// Reduce the top 6 digits mod p_384 (a conditional subtraction of p_384)

        movl    $0x00000000ffffffff, n0short
        movq    $0xffffffff00000000, n1
        movq    $0xfffffffffffffffe, n2

        subq    n0, m0
        sbbq    n1, m1
        sbbq    n2, m2
        sbbq    $-1, m3
        sbbq    $-1, m4
        sbbq    $-1, m5

        sbbq    d, d
        andq    d, n0
        andq    d, n1
        andq    d, n2
        addq    n0, m0
        adcq    n1, m1
        adcq    n2, m2
        adcq    d, m3
        adcq    d, m4
        adcq    d, m5

// Now do (k-6) iterations of 7->6 word modular reduction

        testq   k, k
        jz      Lbignum_mod_p384_writeback

Lbignum_mod_p384_loop:

// Compute q = min (m5 + 1) (2^64 - 1)

        movl    $1, qshort
        addq    m5, q
        sbbq    d, d
        orq     d, q

// Load the next digit so current m to reduce = [m5;m4;m3;m2;m1;m0;d]

        movq    -8(x,k,8), d

// Now form [m5;m4;m3;m2;m1;m0;d] = m - q * p_384. To use an addition for
// the main calculation we do (m - 2^384 * q) + q * (2^384 - p_384)
// where 2^384 - p_384 = [0;0;0;1;0x00000000ffffffff;0xffffffff00000001].
// The extra subtraction of 2^384 * q is the first instruction.

        subq    q, m5
        xorq    n0, n0
        movq    $0xffffffff00000001, n0
        mulxq   n0, n0, n1
        adcxq   n0, d
        adoxq   n1, m0
        movl    $0x00000000ffffffff, n0short
        mulxq   n0, n0, n1
        adcxq   n0, m0
        adoxq   n1, m1
        adcxq   q, m1
        movl    $0, n0short
        adoxq   n0, n0
        adcxq   n0, m2
        adcq    $0, m3
        adcq    $0, m4
        adcq    $0, m5

// Now our top word m5 is either zero or all 1s. Use it for a masked
// addition of p_384, which we can do by a *subtraction* of
// 2^384 - p_384 from our portion

        movq    $0xffffffff00000001, n0
        andq    m5, n0
        movl    $0x00000000ffffffff, n1short
        andq    m5, n1
        andq    $1, m5

        subq    n0, d
        sbbq    n1, m0
        sbbq    m5, m1
        sbbq    $0, m2
        sbbq    $0, m3
        sbbq    $0, m4

// Now shuffle registers up and loop

        movq    m4, m5
        movq    m3, m4
        movq    m2, m3
        movq    m1, m2
        movq    m0, m1
        movq    d, m0

        decq    k
        jnz     Lbignum_mod_p384_loop

// Write back

Lbignum_mod_p384_writeback:

        movq    m0, (z)
        movq    m1, 8(z)
        movq    m2, 16(z)
        movq    m3, 24(z)
        movq    m4, 32(z)
        movq    m5, 40(z)

// Restore registers and return

        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

Lbignum_mod_p384_shortinput:

        xorq    m0, m0
        xorq    m1, m1
        xorq    m2, m2
        xorq    m3, m3
        xorq    m4, m4
        xorq    m5, m5

        testq   k, k
        jz      Lbignum_mod_p384_writeback
        movq    (%rdx), m0
        decq    k
        jz      Lbignum_mod_p384_writeback
        movq    8(%rdx), m1
        decq    k
        jz      Lbignum_mod_p384_writeback
        movq    16(%rdx), m2
        decq    k
        jz      Lbignum_mod_p384_writeback
        movq    24(%rdx), m3
        decq    k
        jz      Lbignum_mod_p384_writeback
        movq    32(%rdx), m4
        jmp     Lbignum_mod_p384_writeback

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
