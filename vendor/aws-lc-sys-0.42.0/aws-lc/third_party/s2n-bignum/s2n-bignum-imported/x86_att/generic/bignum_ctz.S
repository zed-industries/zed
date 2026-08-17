// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Count trailing zero bits
// Input x[k]; output function return
//
//    extern uint64_t bignum_ctz(uint64_t k, const uint64_t *x);
//
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


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_ctz)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_ctz)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_ctz)
        .text

#define k %rdi
#define x %rsi
#define i %rdx
#define w %rcx
#define a %rax

#define wshort %ecx



S2N_BN_SYMBOL(bignum_ctz):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// If the bignum is zero-length, just return 0

        xorq    %rax, %rax
        testq   k, k
        jz      Lbignum_ctz_end

// Use w = a[i-1] to store nonzero words in a top-down sweep
// Set the initial default to be as if we had a 1 word directly above

        movq    k, i
        incq    i
        movl    $1, wshort

Lbignum_ctz_loop:
        movq    -8(x,k,8), a
        testq   a, a
        cmovneq k, i
        cmovneq a, w
        decq    k
        jnz     Lbignum_ctz_loop

// Now w = a[i-1] is the lowest nonzero word, or in the zero case the
// default of the "extra" 1 = a[k]. We now want 64*(i-1) + ctz(w).
// Note that this code does not rely on the behavior of the BSF instruction
// for zero inputs, which is undefined according to the manual.

        decq    i
        shlq    $6, i
        bsfq    w, %rax
        addq    i, %rax

Lbignum_ctz_end:
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_ctz)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
