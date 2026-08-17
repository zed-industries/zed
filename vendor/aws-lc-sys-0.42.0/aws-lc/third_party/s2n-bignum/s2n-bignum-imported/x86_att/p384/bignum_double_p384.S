// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double modulo p_384, z := (2 * x) mod p_384, assuming x reduced
// Input x[6]; output z[6]
//
//    extern void bignum_double_p384(uint64_t z[static 6],
//                                   const uint64_t x[static 6]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_double_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_double_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_double_p384)
        .text

#define z %rdi
#define x %rsi

#define d0 %rdx
#define d1 %rcx
#define d2 %r8
#define d3 %r9
#define d4 %r10
#define d5 %r11
#define c %rax

// Re-use the input pointer as a temporary once we're done

#define a %rsi

#define ashort %esi



S2N_BN_SYMBOL(bignum_double_p384):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Load the input and double it so that 2^384 * c + [d5;d4;d3;d2;d1;d0] = 2 * x
// Could also consider using shld to decouple carries *or* combining this
// and the next block into a double carry chain with ADCX and ADOX.

        xorq    c, c
        movq    (x), d0
        addq    d0, d0
        movq    8(x), d1
        adcq    d1, d1
        movq    16(x), d2
        adcq    d2, d2
        movq    24(x), d3
        adcq    d3, d3
        movq    32(x), d4
        adcq    d4, d4
        movq    40(x), d5
        adcq    d5, d5
        adcq    c, c

// Now subtract p_384 from 2^384 * c + [d5;d4;d3;d2;d1;d0] to get 2 * x - p_384
// This is actually done by *adding* the 7-word negation r_384 = 2^448 - p_384
// where r_384 = [-1; 0; 0; 0; 1; 0x00000000ffffffff; 0xffffffff00000001]

        movq    $0xffffffff00000001, a
        addq    a, d0
        movl    $0x00000000ffffffff, ashort
        adcq    a, d1
        adcq    $1, d2
        adcq    $0, d3
        adcq    $0, d4
        adcq    $0, d5
        adcq    $-1, c

// Since by hypothesis x < p_384 we know 2 * x - p_384 < 2^384, so the top
// carry c actually gives us a bitmask for 2 * x - p_384 < 0, which we
// now use to make r' = mask * (2^384 - p_384) for a compensating subtraction.
// We don't quite have enough ABI-modifiable registers to create all three
// nonzero digits of r while maintaining d0..d5, but make the first two now.

        andq    a, c // c = masked 0x00000000ffffffff
        xorq    a, a
        subq    c, a // a = masked 0xffffffff00000001

// Do the first two digits of addition and writeback

        subq    a, d0
        movq    d0, (z)
        sbbq    c, d1
        movq    d1, 8(z)

// Preserve the carry chain while creating the extra masked digit since
// the logical operation will clear CF

        sbbq    d0, d0
        andq    a, c // c = masked 0x0000000000000001
        negq    d0

// Do the rest of the addition and writeback

        sbbq    c, d2
        movq    d2, 16(z)
        sbbq    $0, d3
        movq    d3, 24(z)
        sbbq    $0, d4
        movq    d4, 32(z)
        sbbq    $0, d5
        movq    d5, 40(z)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_double_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
