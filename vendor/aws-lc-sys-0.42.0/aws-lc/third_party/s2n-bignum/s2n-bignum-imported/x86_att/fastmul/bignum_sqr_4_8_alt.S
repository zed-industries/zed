// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[4]; output z[8]
//
//    extern void bignum_sqr_4_8_alt(uint64_t z[static 8],
//                                   const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_4_8_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_4_8_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_4_8_alt)
        .text

// Input arguments

#define z %rdi
#define x %rsi

// Other variables used as a rotating 3-word window to add terms to

#define t0 %rcx
#define t1 %r8
#define t2 %r9

// Macro for the key "multiply and add to (c,h,l)" step, for square term

#define combadd1(c,h,l,numa)                    \
        movq    numa, %rax ;                      \
        mulq    %rax;                            \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

// A short form where we don't expect a top carry

#define combads(h,l,numa)                       \
        movq    numa, %rax ;                      \
        mulq    %rax;                            \
        addq    %rax, l ;                         \
        adcq    %rdx, h

// A version doubling before adding, for non-square terms

#define combadd2(c,h,l,numa,numb)               \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0, c ;                           \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

S2N_BN_SYMBOL(bignum_sqr_4_8_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Result term 0

        movq    (x), %rax
        mulq    %rax

        movq    %rax, (z)
        movq    %rdx, t0
        xorq    t1, t1

// Result term 1

       xorq    t2, t2
       combadd2(t2,t1,t0,(x),8(x))
       movq    t0, 8(z)

// Result term 2

        xorq    t0, t0
        combadd1(t0,t2,t1,8(x))
        combadd2(t0,t2,t1,(x),16(x))
        movq    t1, 16(z)

// Result term 3

        xorq    t1, t1
        combadd2(t1,t0,t2,(x),24(x))
        combadd2(t1,t0,t2,8(x),16(x))
        movq    t2, 24(z)

// Result term 4

        xorq    t2, t2
        combadd2(t2,t1,t0,8(x),24(x))
        combadd1(t2,t1,t0,16(x))
        movq    t0, 32(z)

// Result term 5

        xorq    t0, t0
        combadd2(t0,t2,t1,16(x),24(x))
        movq    t1, 40(z)

// Result term 6

        xorq    t1, t1
        combads(t0,t2,24(x))
        movq    t2, 48(z)

// Result term 7

        movq    t0, 56(z)

// Return

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_4_8_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
