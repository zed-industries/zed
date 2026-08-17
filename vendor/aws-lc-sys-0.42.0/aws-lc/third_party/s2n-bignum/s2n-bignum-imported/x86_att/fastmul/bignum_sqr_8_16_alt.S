// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[8]; output z[16]
//
//    extern void bignum_sqr_8_16_alt(uint64_t z[static 16],
//                                    const uint64_t x[static 8]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_8_16_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_8_16_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_8_16_alt)
        .text

// Input arguments

#define z %rdi
#define x %rsi

// Other variables used as a rotating 3-word window to add terms to

#define t0 %r8
#define t1 %r9
#define t2 %r10

// Additional temporaries for local windows to share doublings

#define u0 %rcx
#define u1 %r11

// Macro for the key "multiply and add to (c,h,l)" step

#define combadd(c,h,l,numa,numb)                \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

// Set up initial window (c,h,l) = numa * numb

#define combaddz(c,h,l,numa,numb)               \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        xorq    c, c ;                           \
        movq    %rax, l ;                         \
        movq    %rdx, h

// Doubling step (c,h,l) = 2 * (c,hh,ll) + (0,h,l)

#define doubladd(c,h,l,hh,ll)                   \
        addq    ll, ll ;                         \
        adcq    hh, hh ;                         \
        adcq    c, c ;                           \
        addq    ll, l ;                          \
        adcq    hh, h ;                          \
        adcq    $0, c

// Square term incorporation (c,h,l) += numba^2

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

// A version doubling directly before adding, for single non-square terms

#define combadd2(c,h,l,numa,numb)               \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0, c ;                           \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

S2N_BN_SYMBOL(bignum_sqr_8_16_alt):
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

        combaddz(t1,u1,u0,(x),24(x))
        combadd(t1,u1,u0,8(x),16(x))
        doubladd(t1,t0,t2,u1,u0)
        movq    t2, 24(z)

// Result term 4

        combaddz(t2,u1,u0,(x),32(x))
        combadd(t2,u1,u0,8(x),24(x))
        doubladd(t2,t1,t0,u1,u0)
        combadd1(t2,t1,t0,16(x))
        movq    t0, 32(z)

// Result term 5

        combaddz(t0,u1,u0,(x),40(x))
        combadd(t0,u1,u0,8(x),32(x))
        combadd(t0,u1,u0,16(x),24(x))
        doubladd(t0,t2,t1,u1,u0)
        movq    t1, 40(z)

// Result term 6

        combaddz(t1,u1,u0,(x),48(x))
        combadd(t1,u1,u0,8(x),40(x))
        combadd(t1,u1,u0,16(x),32(x))
        doubladd(t1,t0,t2,u1,u0)
        combadd1(t1,t0,t2,24(x))
        movq    t2, 48(z)

// Result term 7

        combaddz(t2,u1,u0,(x),56(x))
        combadd(t2,u1,u0,8(x),48(x))
        combadd(t2,u1,u0,16(x),40(x))
        combadd(t2,u1,u0,24(x),32(x))
        doubladd(t2,t1,t0,u1,u0)
        movq    t0, 56(z)

// Result term 8

        combaddz(t0,u1,u0,8(x),56(x))
        combadd(t0,u1,u0,16(x),48(x))
        combadd(t0,u1,u0,24(x),40(x))
        doubladd(t0,t2,t1,u1,u0)
        combadd1(t0,t2,t1,32(x))
        movq    t1, 64(z)

// Result term 9

        combaddz(t1,u1,u0,16(x),56(x))
        combadd(t1,u1,u0,24(x),48(x))
        combadd(t1,u1,u0,32(x),40(x))
        doubladd(t1,t0,t2,u1,u0)
        movq    t2, 72(z)

// Result term 10

        combaddz(t2,u1,u0,24(x),56(x))
        combadd(t2,u1,u0,32(x),48(x))
        doubladd(t2,t1,t0,u1,u0)
        combadd1(t2,t1,t0,40(x))
        movq    t0, 80(z)

// Result term 11

        combaddz(t0,u1,u0,32(x),56(x))
        combadd(t0,u1,u0,40(x),48(x))
        doubladd(t0,t2,t1,u1,u0)
        movq    t1, 88(z)

// Result term 12

        xorq    t1, t1
        combadd2(t1,t0,t2,40(x),56(x))
        combadd1(t1,t0,t2,48(x))
        movq    t2, 96(z)

// Result term 13

        xorq    t2, t2
        combadd2(t2,t1,t0,48(x),56(x))
        movq    t0, 104(z)

// Result term 14

        combads(t2,t1,56(x))
        movq    t1, 112(z)

// Result term 15

        movq    t2, 120(z)

// Return

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_8_16_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
