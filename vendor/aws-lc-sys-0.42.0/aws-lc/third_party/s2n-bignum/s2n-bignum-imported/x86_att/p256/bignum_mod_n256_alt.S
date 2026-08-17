// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod n_256
// Input x[k]; output z[4]
//
//    extern void bignum_mod_n256_alt(uint64_t z[static 4], uint64_t k,
//                                    const uint64_t *x);
//
// Reduction is modulo the group order of the NIST curve P-256.
//
// Standard x86-64 ABI: RDI = z, RSI = k, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = k, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_n256_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_n256_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_n256_alt)
        .text

#define z %rdi
#define k %rsi
#define x %rcx

#define m0 %r8
#define m1 %r9
#define m2 %r10
#define m3 %r11
#define d %r12

#define n0 %rax
#define n1 %rbx
#define n3 %rdx

#define q %rbx

#define n0short %eax
#define n3short %edx


S2N_BN_SYMBOL(bignum_mod_n256_alt):
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

// If the input is already <= 3 words long, go to a trivial "copy" path

        cmpq    $4, k
        jc      Lbignum_mod_n256_alt_shortinput

// Otherwise load the top 4 digits (top-down) and reduce k by 4

        subq    $4, k
        movq    24(%rdx,k,8), m3
        movq    16(%rdx,k,8), m2
        movq    8(%rdx,k,8), m1
        movq    (%rdx,k,8), m0

// Move x into another register to leave %rdx free for multiplies and use of n3

        movq    %rdx, x

// Reduce the top 4 digits mod n_256 (a conditional subtraction of n_256)

        movq    $0x0c46353d039cdaaf, n0
        movq    $0x4319055258e8617b, n1
        movl    $0x00000000ffffffff, n3short

        addq    n0, m0
        adcq    n1, m1
        adcq    $0, m2
        adcq    n3, m3
        sbbq    d, d
        notq    d
        andq    d, n0
        andq    d, n1
        andq    d, n3
        subq    n0, m0
        sbbq    n1, m1
        sbbq    $0, m2
        sbbq    n3, m3

// Now do (k-4) iterations of 5->4 word modular reduction

        testq   k, k
        jz      Lbignum_mod_n256_alt_writeback

Lbignum_mod_n256_alt_loop:

// Writing the input as z = 2^256 * m3 + 2^192 * m2 + t = 2^192 * h + t, our
// intended quotient approximation is MIN ((h + h>>32 + 1)>>64) (2^64 - 1).

        movq    m3, n0
        shldq   $32, m2, n0
        movq    m3, q
        shrq    $32, q

        xorq    %rdx, %rdx
        subq    $1, %rdx

        adcq    m2, n0
        adcq    m3, q
        sbbq    n0, n0
        orq     n0, q

// Load the next digit so current m to reduce = [m3;m2;m1;m0;d]

        movq    -8(x,k,8), d

// Now form [m3;m2;m1;m0;d] = m - q * n_256

        subq    q, m3

        movq    $0x0c46353d039cdaaf, %rax
        mulq    q
        addq    %rax, d
        adcq    %rdx, m0
        adcq    $0, m1
        adcq    $0, m2
        adcq    $0, m3

        movq    $0x4319055258e8617b, %rax
        mulq    q
        addq    %rax, m0
        adcq    %rdx, m1
        adcq    $0, m2
        adcq    $0, m3

        movq    $0x00000000ffffffff, %rax
        mulq    q
        addq    %rax, m2
        adcq    %rdx, m3

// Now our top word m3 is either zero or all 1s. Use it for a masked
// addition of n_256, which we can do by a *subtraction* of
// 2^256 - n_256 from our portion

        movq    $0x0c46353d039cdaaf, n0
        andq    m3, n0
        movq    $0x4319055258e8617b, n1
        andq    m3, n1
        movl    $0x00000000ffffffff, n3short
        andq    m3, n3

        subq    n0, d
        sbbq    n1, m0
        sbbq    $0, m1
        sbbq    n3, m2

// Now shuffle registers up and loop

        movq    m2, m3
        movq    m1, m2
        movq    m0, m1
        movq    d, m0

        decq    k
        jnz     Lbignum_mod_n256_alt_loop

// Write back

Lbignum_mod_n256_alt_writeback:

        movq    m0, (z)
        movq    m1, 8(z)
        movq    m2, 16(z)
        movq    m3, 24(z)

// Restore registers and return

        CFI_POP(%r12)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

Lbignum_mod_n256_alt_shortinput:

        xorq    m0, m0
        xorq    m1, m1
        xorq    m2, m2
        xorq    m3, m3

        testq   k, k
        jz      Lbignum_mod_n256_alt_writeback
        movq    (%rdx), m0
        decq    k
        jz      Lbignum_mod_n256_alt_writeback
        movq    8(%rdx), m1
        decq    k
        jz      Lbignum_mod_n256_alt_writeback
        movq    16(%rdx), m2
        jmp     Lbignum_mod_n256_alt_writeback

S2N_BN_SIZE_DIRECTIVE(bignum_mod_n256_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
