// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo group order, z := x mod m_25519
// Input x[k]; output z[4]
//
//    extern void bignum_mod_m25519(uint64_t z[static 4], uint64_t k,
//                                  const uint64_t *x);
//
// Reduction is modulo the group order of curve25519/edwards25519.
// This is the full group order, 8 * the standard basepoint order.
//
// Standard x86-64 ABI: RDI = z, RSI = k, RDX = x
// Microsoft x64 ABI:   RCX = z, RDX = k, R8 = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_m25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_m25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_m25519)
        .text

#define z %rdi
#define k %rsi
#define x %rcx

#define m0 %r8
#define m1 %r9
#define m2 %r10
#define m3 %r11
#define d %r12

#define q %rbx

S2N_BN_SYMBOL(bignum_mod_m25519):
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
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)

// If the input is already <= 3 words long, go to a trivial "copy" path

        cmpq    $4, k
        jc      Lbignum_mod_m25519_shortinput

// Otherwise load the top 4 digits (top-down) and reduce k by 4
// This [m3;m2;m1;m0] is the initial x where we begin reduction.

        subq    $4, k
        movq    24(%rdx,k,8), m3
        movq    16(%rdx,k,8), m2
        movq    8(%rdx,k,8), m1
        movq    (%rdx,k,8), m0

// Move x into another register to leave %rdx free for multiplies

        movq    %rdx, x

// Load a set of registers [%rbx; 0; %rdx; %rax] = m_25519

        movq    $0xc09318d2e7ae9f68, %rax
        movq    $0xa6f7cef517bce6b2, %rdx
        movq    $0x8000000000000000, %rbx

// Do the subtraction; since m_25519 > 2^255 we only need either
// subtraction of m_25519 or nothing to reduce it modulo.

        subq    %rax, m0
        sbbq    %rdx, m1
        sbbq    $0, m2
        sbbq    %rbx, m3

// Now CF is set iff x < m_25519. Create a mask for that condition and mask
// the three nontrivial digits ready to undo the previous subtraction with
// a compensating addition

        sbbq    %rbx, %rbx
        andq    %rbx, %rax
        andq    %rbx, %rdx
        shlq    $63, %rbx

// Now add mask * m_25519 again

        addq    %rax, m0
        adcq    %rdx, m1
        adcq    $0, m2
        adcq    %rbx, m3

// Now do (k-4) iterations of 5->4 word modular reduction. Each one
// is similar to the sequence above except for the more refined quotient
// estimation process.

        testq   k, k
        jz      Lbignum_mod_m25519_writeback

Lbignum_mod_m25519_loop:

// Assume that the new 5-digit x is 2^64 * previous_x + next_digit.
// Get the quotient estimate q = max (floor(x/2^255)) (2^64 - 1)
// and first compute x' = x - 2^255 * q.

        movq    m3, q
        shldq   $1, m2, q
        shrq    $63, m3
        subq    m3, q
        shlq    $1, m2
        shrdq   $1, m3, m2

// Let [%rdx;m3;%rbp] = q * (m_25519 - 2^255)

        movq    $0xc09318d2e7ae9f68, %rax
        mulq    q
        movq    %rax, %rbp
        movq    %rdx, m3

        movq    $0xa6f7cef517bce6b2, %rax
        mulq    q
        addq    %rax, m3
        adcq    $0, %rdx

// Load the next digit

        movq    -8(x,k,8), d

// Subtract to get x' - q * (m_25519 - 2^255) = x - q * m_25519

        subq    %rbp, d
        sbbq    m3, m0
        sbbq    %rdx, m1
        sbbq    $0, m2

// Get a bitmask for the borrow and create a masked version of
// non-trivial digits of [%rbx;0;%rdx;%rax] = m_25519, then add it.
// The masked n3 digit exploits the fact that bit 63 of n0 is set.

        sbbq    %rbx, %rbx

        movq    $0xc09318d2e7ae9f68, %rax
        andq    %rbx, %rax
        movq    $0xa6f7cef517bce6b2, %rdx
        andq    %rbx, %rdx
        movq    $0x8000000000000000, %rbx
        andq    %rax, %rbx

        addq    %rax, d
        adcq    %rdx, m0
        adcq    $0, m1
        adcq    %rbx, m2

// Now shuffle registers up and loop

        movq    m2, m3
        movq    m1, m2
        movq    m0, m1
        movq    d, m0

        decq    k
        jnz     Lbignum_mod_m25519_loop
        jmp     Lbignum_mod_m25519_writeback

Lbignum_mod_m25519_shortinput:

        xorq    m0, m0
        xorq    m1, m1
        xorq    m2, m2
        xorq    m3, m3

        testq   k, k
        jz      Lbignum_mod_m25519_writeback
        movq    (%rdx), m0
        decq    k
        jz      Lbignum_mod_m25519_writeback
        movq    8(%rdx), m1
        decq    k
        jz      Lbignum_mod_m25519_writeback
        movq    16(%rdx), m2

// Write back

Lbignum_mod_m25519_writeback:

        movq    m0, (z)
        movq    m1, 8(z)
        movq    m2, 16(z)
        movq    m3, 24(z)

// Restore registers and return

        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_m25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
