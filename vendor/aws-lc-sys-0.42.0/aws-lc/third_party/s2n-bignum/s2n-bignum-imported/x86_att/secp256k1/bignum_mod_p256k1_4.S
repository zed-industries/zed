// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Reduce modulo field characteristic, z := x mod p_256k1
// Input x[4]; output z[4]
//
//    extern void bignum_mod_p256k1_4(uint64_t z[static 4],
//                                    const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mod_p256k1_4)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mod_p256k1_4)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mod_p256k1_4)
        .text

#define z %rdi
#define x %rsi

#define d0 %rdx
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define c %r10

#define d %rax


S2N_BN_SYMBOL(bignum_mod_p256k1_4):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the inputs as [d3;d2;d1;d0] and let d be an AND of [d3;d2;d1] to
// condense the comparison below.

        movq    (x), d0
        movq    8(x), d1
        movq    d1, d
        movq    16(x), d2
        andq    d2, d
        movq    24(x), d3
        andq    d3, d

// Compare x >= p_256k1 = 2^256 - 4294968273 using condensed carry:
// we get a carry from the lowest digit and all other digits are 1.
// We end up with c and d as adjusted digits for x - p_256k1 if so.

        movq    $4294968273, c
        addq    d0, c
        adcq    $0, d

// If indeed x >= p_256k1 then x := x - p_256k1, using c and d
// Either way, write back to z

        cmovcq  c, d0
        movq    d0, (z)
        cmovcq  d, d1
        movq    d1, 8(z)
        cmovcq  d, d2
        movq    d2, 16(z)
        cmovcq  d, d3
        movq    d3, 24(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mod_p256k1_4)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
