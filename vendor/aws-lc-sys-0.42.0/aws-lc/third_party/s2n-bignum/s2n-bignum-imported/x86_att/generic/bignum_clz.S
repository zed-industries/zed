// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Count leading zero bits
// Input x[k]; output function return
//
//    extern uint64_t bignum_clz(uint64_t k, const uint64_t *x);
//
// In the case of a zero bignum as input the result is 64 * k
//
// In principle this has a precondition k < 2^58, but obviously that
// is always true in practice because of address space limitations
//
// Standard x86-64 ABI: RDI = k, RSI = x, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = x, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_clz)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_clz)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_clz)
        .text

#define k %rdi
#define x %rsi
#define i %rax
#define w %rdx
#define a %rcx
#define j %r8



S2N_BN_SYMBOL(bignum_clz):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Initialize the index i and also prepare default return value of 0 (i = %rax)

        xorq    i, i

// If the bignum is zero-length, just return 0

        testq   k, k
        jz      Lbignum_clz_end

// Use w = a[i-1] to store nonzero words in a bottom-up sweep
// Set the initial default to be as if we had a 11...11 word directly below

        movq    $-1, w
        xorq    j, j
Lbignum_clz_loop:
        movq    (x,j,8), a
        incq    j
        testq   a, a
        cmovnzq j, i
        cmovnzq a, w
        cmpq    k, j
        jnz     Lbignum_clz_loop

// Now w = a[i-1] is the highest nonzero word, or in the zero case the
// default of the "extra" 11...11 = a[0-1]. We now want 64*(k - i) + clz(w) =
// 64*(k - i) + (63 - bsr(w)). Note that this code does not rely on the
// behavior of the bsr instruction for zero inputs, where it is undefined

        subq    i, k
        shlq    $6, k
        bsrq    w, %rax
        xorq    $63, %rax
        addq    k, %rax

Lbignum_clz_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_clz)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
