// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// 256-bit nonzeroness test, returning 1 if x is nonzero, 0 if x is zero
// Input x[4]; output function return
//
//    extern uint64_t bignum_nonzero_4(const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = x, returns RAX
// Microsoft x64 ABI:   RCX = x, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_nonzero_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_nonzero_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_nonzero_4)
        .text

#define x %rdi
#define a %rax
#define d %rdx
#define dshort %edx



S2N_BN_SYMBOL(bignum_nonzero_4):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
#endif

// Generate a = an OR of all the words in the bignum

        movq    (x), a
        movq    8(x), d
        orq     16(x), a
        orq     24(x), d
        orq     d, a

// Set a standard C condition based on whether a is nonzero

        movl    $1, dshort
        cmovnzq d, a

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_nonzero_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
