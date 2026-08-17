// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply z := x * y
// Inputs x[8], y[8]; output z[16]
//
//    extern void bignum_mul_8_16_alt(uint64_t z[static 16],
//                                    const uint64_t x[static 8],
//                                    const uint64_t y[static 8]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_8_16_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_8_16_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_8_16_alt)
        .text

// These are actually right

#define z %rdi
#define x %rsi

// This is moved from %rdx to free it for muls

#define y %rcx

// Other variables used as a rotating 3-word window to add terms to

#define t0 %r8
#define t1 %r9
#define t2 %r10

// Macro for the key "multiply and add to (c,h,l)" step

#define combadd(c,h,l,numa,numb)                \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

// A minutely shorter form for when c = 0 initially

#define combadz(c,h,l,numa,numb)                \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    c, c

// A short form where we don't expect a top carry

#define combads(h,l,numa,numb)                  \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h

S2N_BN_SYMBOL(bignum_mul_8_16_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Copy y into a safe register to start with

        movq    %rdx, y

// Result term 0

        movq    (x), %rax
        mulq     (y)

        movq    %rax, (z)
        movq    %rdx, t0
        xorq    t1, t1

// Result term 1

        xorq    t2, t2
        combads(t1,t0,(x),8(y))
        combadz(t2,t1,t0,8(x),(y))
        movq    t0, 8(z)

// Result term 2

        xorq    t0, t0
        combadz(t0,t2,t1,(x),16(y))
        combadd(t0,t2,t1,8(x),8(y))
        combadd(t0,t2,t1,16(x),(y))
        movq    t1, 16(z)

// Result term 3

        xorq    t1, t1
        combadz(t1,t0,t2,(x),24(y))
        combadd(t1,t0,t2,8(x),16(y))
        combadd(t1,t0,t2,16(x),8(y))
        combadd(t1,t0,t2,24(x),(y))
        movq    t2, 24(z)

// Result term 4

        xorq    t2, t2
        combadz(t2,t1,t0,(x),32(y))
        combadd(t2,t1,t0,8(x),24(y))
        combadd(t2,t1,t0,16(x),16(y))
        combadd(t2,t1,t0,24(x),8(y))
        combadd(t2,t1,t0,32(x),(y))
        movq    t0, 32(z)

// Result term 5

        xorq    t0, t0
        combadz(t0,t2,t1,(x),40(y))
        combadd(t0,t2,t1,8(x),32(y))
        combadd(t0,t2,t1,16(x),24(y))
        combadd(t0,t2,t1,24(x),16(y))
        combadd(t0,t2,t1,32(x),8(y))
        combadd(t0,t2,t1,40(x),(y))
        movq    t1, 40(z)

// Result term 6

        xorq    t1, t1
        combadz(t1,t0,t2,(x),48(y))
        combadd(t1,t0,t2,8(x),40(y))
        combadd(t1,t0,t2,16(x),32(y))
        combadd(t1,t0,t2,24(x),24(y))
        combadd(t1,t0,t2,32(x),16(y))
        combadd(t1,t0,t2,40(x),8(y))
        combadd(t1,t0,t2,48(x),(y))
        movq    t2, 48(z)

// Result term 7

        xorq    t2, t2
        combadz(t2,t1,t0,(x),56(y))
        combadd(t2,t1,t0,8(x),48(y))
        combadd(t2,t1,t0,16(x),40(y))
        combadd(t2,t1,t0,24(x),32(y))
        combadd(t2,t1,t0,32(x),24(y))
        combadd(t2,t1,t0,40(x),16(y))
        combadd(t2,t1,t0,48(x),8(y))
        combadd(t2,t1,t0,56(x),(y))
        movq    t0, 56(z)

// Result term 8

        xorq    t0, t0
        combadz(t0,t2,t1,8(x),56(y))
        combadd(t0,t2,t1,16(x),48(y))
        combadd(t0,t2,t1,24(x),40(y))
        combadd(t0,t2,t1,32(x),32(y))
        combadd(t0,t2,t1,40(x),24(y))
        combadd(t0,t2,t1,48(x),16(y))
        combadd(t0,t2,t1,56(x),8(y))
        movq    t1, 64(z)

// Result term 9

        xorq    t1, t1
        combadz(t1,t0,t2,16(x),56(y))
        combadd(t1,t0,t2,24(x),48(y))
        combadd(t1,t0,t2,32(x),40(y))
        combadd(t1,t0,t2,40(x),32(y))
        combadd(t1,t0,t2,48(x),24(y))
        combadd(t1,t0,t2,56(x),16(y))
        movq    t2, 72(z)

// Result term 10

        xorq    t2, t2
        combadz(t2,t1,t0,24(x),56(y))
        combadd(t2,t1,t0,32(x),48(y))
        combadd(t2,t1,t0,40(x),40(y))
        combadd(t2,t1,t0,48(x),32(y))
        combadd(t2,t1,t0,56(x),24(y))
        movq    t0, 80(z)

// Result term 11

        xorq    t0, t0
        combadz(t0,t2,t1,32(x),56(y))
        combadd(t0,t2,t1,40(x),48(y))
        combadd(t0,t2,t1,48(x),40(y))
        combadd(t0,t2,t1,56(x),32(y))
        movq    t1, 88(z)

// Result term 12

        xorq    t1, t1
        combadz(t1,t0,t2,40(x),56(y))
        combadd(t1,t0,t2,48(x),48(y))
        combadd(t1,t0,t2,56(x),40(y))
        movq    t2, 96(z)

// Result term 13

        xorq    t2, t2
        combadz(t2,t1,t0,48(x),56(y))
        combadd(t2,t1,t0,56(x),48(y))
        movq    t0, 104(z)

// Result term 14

        combads(t2,t1,56(x),56(y))
        movq    t1, 112(z)

// Result term 11

        movq    t2, 120(z)

// Return

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_8_16_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
