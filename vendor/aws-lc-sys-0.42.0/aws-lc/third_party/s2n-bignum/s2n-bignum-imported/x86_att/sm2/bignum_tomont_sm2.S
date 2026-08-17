// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert to Montgomery form z := (2^256 * x) mod p_sm2
// Input x[4]; output z[4]
//
//    extern void bignum_tomont_sm2(uint64_t z[static 4], const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_sm2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_sm2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_sm2)
        .text

#define z %rdi
#define x %rsi

#define m0 %r8
#define m1 %r9
#define m2 %r10
#define m3 %r11

#define q %rax
#define n1 %rcx
#define n3 %rdx
#define qshort %eax

// ----------------------------------------------------------------------------
// Core "x |-> (2^64 * x) mod p_sm2" macro, with x assumed to be < p_sm2.
// Input is [d3;d2;d1;d0] and output is [d2;d1;d0;q] where q is a fixed
// quotient estimate register (%rax), so the registers get shuffled.
// ----------------------------------------------------------------------------

#define modstep_sm2(d3,d2,d1,d0)                                        \
/* Writing the input, with lower zero digit appended, as        */      \
/* z = 2^256 * d3 + 2^192 * d2 + t, quotient approximation is   */      \
/* MIN ((d3 * (1 + 2^32 + 2^64) + d2 + 2^64) >> 64) (2^64 - 1)  */      \
        movq    d2, n1 ;                                                 \
        movl    $1, qshort ;                                              \
        addq    d3, n1 ;                                                 \
        adcq    d3, q ;                                                  \
        shrq    $32, n1 ;                                                 \
        addq    d3, n1 ;                                                 \
        shrq    $32, n1 ;                                                 \
        addq    n1, q ;                                                  \
        sbbq    $0, q ;                                                   \
/* Compute the pre-reduced [d3;d2;d1;d0;q] = m - p_sm2 * q      */      \
/* = z - (2^256 - 2^224 - 2^96 + 2^64 - 1) * q                  */      \
        movq    q, n1 ;                                                  \
        movq    q, n3 ;                                                  \
        shlq    $32, n1 ;                                                 \
        shrq    $32, n3 ;                                                 \
        addq    n1, d2 ;                                                 \
        adcq    n3, d3 ;                                                 \
        subq    q, n1 ;                                                  \
        sbbq    $0, n3 ;                                                  \
        subq    q, d3 ;                                                  \
        addq    n1, d0 ;                                                 \
        adcq    n3, d1 ;                                                 \
        adcq    $0, d2 ;                                                  \
        adcq    $0, d3 ;                                                  \
/* Corrective addition with top word d3 as a bitmask            */      \
        movq    $0xffffffff00000000, n1 ;                                 \
        andq    d3, n1 ;                                                 \
        movq    $0xfffffffeffffffff, n3 ;                                 \
        andq    d3, n3 ;                                                 \
        addq    d3, q ;                                                  \
        adcq    n1, d0 ;                                                 \
        adcq    d3, d1 ;                                                 \
        adcq    n3, d2

S2N_BN_SYMBOL(bignum_tomont_sm2):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the inputs

        movq    (x), m0
        movq    8(x), m1
        movq    16(x), m2
        movq    24(x), m3

// Load non-trivial digits [n3; -1; n1; -1] = p_sm2 and do a conditional
// subtraction to reduce the four starting digits [m3;m2;m1;m0] modulo p_sm2

        subq    $-1, m0
        movq    $0xffffffff00000000, n1
        sbbq    n1, m1
        movq    $0xfffffffeffffffff, n3
        sbbq    $-1, m2
        sbbq    n3, m3
        sbbq    q, q
        andq    q, n1
        andq    q, n3
        addq    q, m0
        adcq    n1, m1
        adcq    q, m2
        adcq    n3, m3

// Now do 4 iterations of 5->4 word modular reduction

        modstep_sm2(m3,m2,m1,m0)

        movq    q, m3

        modstep_sm2(m2,m1,m0,m3)

        movq    q, m2

        modstep_sm2(m1,m0,m3,m2)

        movq    q, m1

        modstep_sm2(m0,m3,m2,m1)

// Write back result and return

        movq    q, (z)
        movq    m1, 8(z)
        movq    m2, 16(z)
        movq    m3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_tomont_sm2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
