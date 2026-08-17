// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Test bignum for even-ness
// Input x[k]; output function return
//
//    extern uint64_t bignum_even(uint64_t k, const uint64_t *x);
//
// Standard x86-64 ABI: RDI = k, RSI = x, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = x, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_even)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_even)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_even)
        .text

S2N_BN_SYMBOL(bignum_even):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Set default return value of 1 and finish if k = 0 (trivially even)

        movl    $1, %eax
        testq   %rdi, %rdi
        jz      Lbignum_even_end

// Otherwise XOR that initial 1 with the lowest bit of the input

        xorq    (%rsi), %rax
        andq    $1, %rax

Lbignum_even_end:

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_even)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
