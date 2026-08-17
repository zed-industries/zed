// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Optionally subtract or add, z := x + sgn(p) * y interpreting p as signed
// Inputs x[k], p, y[k]; outputs function return (carry-out) and z[k]
//
//    extern uint64_t bignum_optsubadd(uint64_t k, uint64_t *z, const uint64_t *x,
//                                     uint64_t p, const uint64_t *y);
//
// If p has top bit set (i.e. is negative as a signed int) return z := x - y
// Else if p is nonzero (i.e. is positive as a signed int) return z := x + y
// Otherwise (i.e. p is zero) return z := x
//
// Return in RDI = the top carry, which will be 0 or 1, and appropriate for
// addition or subtraction respectively (and always zero for p = 0)
//
// 2^{64*k} * -carryout + z = x - y [for subtraction]
// 2^{64*k} * carryout + z = x + y [for addition]
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = x, RCX = p, R8 = y, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = x, R9 = p, [RSP+40] = y, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_optsubadd)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_optsubadd)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_optsubadd)
        .text

#define k %rdi
#define z %rsi
#define x %rdx
#define p %rcx
#define y %r8

#define c %rax
#define i %r9
#define m %rcx
#define q %r10
#define a %r11


S2N_BN_SYMBOL(bignum_optsubadd):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
        movq    56(%rsp), %r8
#endif

// Initialize top carry to zero in all cases (also return value)

        xorq    c, c

// If k = 0 do nothing

        testq   k, k
        jz      Lbignum_optsubadd_end

// Turn the input p into two bitmasks, m indicating to use the y input at
// all (same register as p) and q indicating a sign-flip

        movq    p, q
        sarq    $63, q
        negq    p
        sbbq    m, m

// Generate an initial carry-in for the negating case only to add 1; this
// is because we are actually going to do complements of the words of y

        movq    q, c

// Now go round the loop for i=0...k-1, saving the carry in c each iteration

        xorq    i, i
Lbignum_optsubadd_loop:
        movq    (y,i,8), a
        xorq    q, a
        andq    m, a
        negq    c
        adcq    (x,i,8), a
        sbbq    c, c
        movq    a, (z,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_optsubadd_loop

// Return carry flag, fixing up inversion for negative case

        xorq    q, %rax
        negq    %rax

Lbignum_optsubadd_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_optsubadd)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
