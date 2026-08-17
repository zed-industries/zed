// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_25519
// Input x[4]; output z[4]
//
//    extern void bignum_mod_p25519_4(uint64_t z[static 4],
//                                    const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p25519_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p25519_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p25519_4)
        .text

#define z %rdi
#define x %rsi

#define d0 %rdx
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define c %r10

#define q %rax

#define qshort %eax
#define cshort %r10d

S2N_BN_SYMBOL(bignum_mod_p25519_4):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the inputs as [d3;d2;d1;d0]

        movq    (x), d0
        movq    8(x), d1
        movq    16(x), d2
        movq    24(x), d3

// Letting x = 2^255 * h + l where h is the top bit, the provisional quotient
// is q = h + 1, which is either correct or 1 too high.

        movl    $1, qshort
        xorl    cshort, cshort
        bts     $63, d3
        adcq    c, q
        imulq   $19, q

// Writing the provisional remainder as r = x - (2^255 - 19) * q we
// compute r' = (2^255 + l) + 19 * q = r + 2^256

        addq    q, d0
        adcq    c, d1
        adcq    c, d2
        adcq    c, d3

// Now r < 0 <=> r' < 2^256 <=> ~CF and in this case we correct by adding
// 2^255 - 19, or in fact subtracting 19 and masking to 255 bits.

        movl    $19, qshort
        cmovcq  c, q

        subq    q, d0
        sbbq    c, d1
        sbbq    c, d2
        sbbq    c, d3
        btr     $63, d3

// Store the end result

        movq    d0, (z)
        movq    d1, 8(z)
        movq    d2, 16(z)
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p25519_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
