// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Convert to Montgomery form z := (2^256 * x) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_tomont_p256(uint64_t z[static 4],
//                                   const uint64_t x[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_tomont_p256)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_tomont_p256)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_tomont_p256)
        .text

#define z %rdi
#define x %rsi

// Some temp registers for the last correction stage

#define d %rax
#define u %rdx
#define v %rcx

#define dshort %eax
#define ushort %edx

// Add %rdx * m into a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rcx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rcx, high

S2N_BN_SYMBOL(bignum_tomont_p256):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// We are essentially just doing a Montgomery multiplication of x and the
// precomputed constant y = 2^512 mod p, so the code is almost the same
// modulo a few registers and the change from loading y[i] to using constants.
// Because there is no y pointer to keep, we use one register less.

        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Do row 0 computation, which is a bit different:
// set up initial window [%r12,%r11,%r10,%r9,%r8] = y[0] * x
// Unlike later, we only need a single carry chain

        xorq    %r13, %r13
        movl    $0x0000000000000003, %edx
        mulxq   (x), %r8, %r9
        mulxq   8(x), %rcx, %r10
        adcxq   %rcx, %r9
        mulxq   16(x), %rcx, %r11
        adcxq   %rcx, %r10
        mulxq   24(x), %rcx, %r12
        adcxq   %rcx, %r11
        adcxq   %r13, %r12

// Add row 1

        movq    $0xfffffffbffffffff, %rdx
        xorq    %r14, %r14
        mulpadd(%r10,%r9,(x))
        mulpadd(%r11,%r10,8(x))
        mulpadd(%r12,%r11,16(x))
        mulpadd(%r13,%r12,24(x))
        adcq   %r14, %r13

// Montgomery reduce windows 0 and 1 together

        xorq    %r15, %r15
        movq    $0x0000000100000000, %rdx
        mulpadd(%r10,%r9,%r8)
        mulpadd(%r11,%r10,%r9)
        movq    $0xffffffff00000001, %rdx
        mulpadd(%r12,%r11,%r8)
        mulpadd(%r13,%r12,%r9)
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcxq   %r15, %r14

// Add row 2

        movq    $0xfffffffffffffffe, %rdx
        xorq    %r8, %r8
        mulpadd(%r11,%r10,(x))
        mulpadd(%r12,%r11,8(x))
        mulpadd(%r13,%r12,16(x))
        mulpadd(%r14,%r13,24(x))
        adcxq   %r8, %r14
        adoxq   %r8, %r15
        adcxq   %r8, %r15

// Add row 3

        movq    $0x00000004fffffffd, %rdx
        xorq    %r9, %r9
        mulpadd(%r12,%r11,(x))
        mulpadd(%r13,%r12,8(x))
        mulpadd(%r14,%r13,16(x))
        mulpadd(%r15,%r14,24(x))
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcxq   %r9, %r8

// Montgomery reduce windows 2 and 3 together

        xorq    %r9, %r9
        movq    $0x0000000100000000, %rdx
        mulpadd(%r12,%r11,%r10)
        mulpadd(%r13,%r12,%r11)
        movq    $0xffffffff00000001, %rdx
        mulpadd(%r14,%r13,%r10)
        mulpadd(%r15,%r14,%r11)
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcxq   %r9, %r8

// We now have a pre-reduced 5-word form [%r8; %r15;%r14;%r13;%r12]
// Load non-trivial digits of p_256 = [v; 0; u; -1]

        movl    $0x00000000ffffffff, ushort
        movq    $0xffffffff00000001, v

// Now do the subtraction (0,p_256-1) - (%r8,%r15,%r14,%r13,%r12) to get the carry

        movq    $-2, d
        subq    %r12, d
        movq    u, d
        sbbq    %r13, d
        movl    $0, dshort
        sbbq    %r14, d
        movq    v, d
        sbbq    %r15, d

// This last last comparison in the chain will actually even set the mask
// for us, so we don't need to separately create it from the carry.
// This means p_256 - 1 < (c,d1,d0,d5,d4), i.e. we are so far >= p_256

        movl    $0, dshort
        sbbq    %r8, d
        andq    d, u
        andq    d, v

// Do a masked subtraction of p_256 and write back

        subq    d, %r12
        sbbq    u, %r13
        sbbq    $0, %r14
        sbbq    v, %r15

        movq    %r12, (z)
        movq    %r13, 8(z)
        movq    %r14, 16(z)
        movq    %r15, 24(z)

// Restore registers and return

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_tomont_p256)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
