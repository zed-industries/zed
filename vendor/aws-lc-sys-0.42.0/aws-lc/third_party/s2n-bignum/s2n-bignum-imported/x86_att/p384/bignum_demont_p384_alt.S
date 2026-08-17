// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert from Montgomery form z := (x / 2^384) mod p_384, assuming x reduced
// Input x[6]; output z[6]
//
//    extern void bignum_demont_p384_alt(uint64_t z[static 6],
//                                       const uint64_t x[static 6]);
//
// This assumes the input is < p_384 for correctness. If this is not the case,
// use the variant "bignum_deamont_p384" instead.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_demont_p384_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_demont_p384_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_demont_p384_alt)
        .text

#define z %rdi
#define x %rsi

// Core one-step "short" Montgomery reduction macro. Takes input in
// [d5;d4;d3;d2;d1;d0] and returns result in [d6;d5;d4;d3;d2;d1],
// adding to the existing [d5;d4;d3;d2;d1] and re-using d0 as a
// temporary internally, as well as %rax, %rcx and %rdx.
// It is OK for d6 and d0 to be the same register (they often are)
//
// We want to add (2^384 - 2^128 - 2^96 + 2^32 - 1) * w
// where w = [d0 + (d0<<32)] mod 2^64
//
//       montreds(d6,d5,d4,d3,d2,d1,d0)

#define montreds(d6,d5,d4,d3,d2,d1,d0)                                  \
/* Our correction multiplier is w = [d0 + (d0<<32)] mod 2^64 */         \
        movq    d0, %rcx ;                                        \
        shlq    $32, %rcx ;                                        \
        addq    d0, %rcx ;                                        \
/* Construct [%rax;%rdx;d0;-] = (2^384 - p_384) * w            */         \
/* We know the lowest word will cancel so we can re-use d0   */         \
/* and %rcx as temps.                                         */         \
        movq    $0xffffffff00000001, %rax ;                        \
        mulq    %rcx;                                            \
        movq    %rdx, d0 ;                                        \
        movq    $0x00000000ffffffff, %rax ;                        \
        mulq    %rcx;                                            \
        addq    %rax, d0 ;                                        \
        movl    $0, %eax ;                                         \
        adcq    %rcx, %rdx ;                                       \
        adcl    %eax, %eax ;                                       \
/* Now subtract that and add 2^384 * w                       */         \
        subq    d0, d1 ;                                         \
        sbbq    %rdx, d2 ;                                        \
        sbbq    %rax, d3 ;                                        \
        sbbq    $0, d4 ;                                          \
        sbbq    $0, d5 ;                                          \
        movq    %rcx, d6 ;                                        \
        sbbq    $0, d6

S2N_BN_SYMBOL(bignum_demont_p384_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save more registers to play with

        CFI_PUSH(%r12)
        CFI_PUSH(%r13)

// Set up an initial window [%r13,%r12,%r11,%r10,%r9,%r8] = x

        movq    (x), %r8
        movq    8(x), %r9
        movq    16(x), %r10
        movq    24(x), %r11
        movq    32(x), %r12
        movq    40(x), %r13

// Montgomery reduce window 0

        montreds(%r8,%r13,%r12,%r11,%r10,%r9,%r8)

// Montgomery reduce window 1

        montreds(%r9,%r8,%r13,%r12,%r11,%r10,%r9)

// Montgomery reduce window 2

        montreds(%r10,%r9,%r8,%r13,%r12,%r11,%r10)

// Montgomery reduce window 3

        montreds(%r11,%r10,%r9,%r8,%r13,%r12,%r11)

// Montgomery reduce window 4

        montreds(%r12,%r11,%r10,%r9,%r8,%r13,%r12)

// Montgomery reduce window 5

        montreds(%r13,%r12,%r11,%r10,%r9,%r8,%r13)

// Write back the result

        movq    %r8, (z)
        movq    %r9, 8(z)
        movq    %r10, 16(z)
        movq    %r11, 24(z)
        movq    %r12, 32(z)
        movq    %r13, 40(z)

// Restore registers and return

        CFI_POP(%r13)
        CFI_POP(%r12)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_demont_p384_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
