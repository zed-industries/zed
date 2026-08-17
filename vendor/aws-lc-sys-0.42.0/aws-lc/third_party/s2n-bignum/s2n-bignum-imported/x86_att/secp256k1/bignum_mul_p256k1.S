// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply modulo p_256k1, z := (x * y) mod p_256k1
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_mul_p256k1(uint64_t z[static 4], const uint64_t x[static 4],
//                                  const uint64_t y[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_p256k1)
        .text

// These are actually right

#define z %rdi
#define x %rsi

// Copied in or set up

#define y %rcx

// A zero register

#define zero %rbp
#define zeroe %ebp

// mulpadd(high,low,m) adds %rdx * m to a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries.

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rbx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rbx, high

// mulpade(high,low,m) adds %rdx * m to a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax as a temporary, assuming high created from scratch
// and that zero has value zero.

#define mulpade(high,low,m)             \
        mulxq   m, %rax, high ;           \
        adcxq   %rax, low ;               \
        adoxq   zero, high

S2N_BN_SYMBOL(bignum_mul_p256k1):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save more registers to play with

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Copy y into a safe register to start with

        movq    %rdx, y

// Zero a register, which also makes sure we don't get a fake carry-in

        xorl    zeroe, zeroe

// Do the zeroth row, which is a bit different

        movq    (y), %rdx

        mulxq   (x), %r8, %r9
        mulxq   8(x), %rax, %r10
        addq    %rax, %r9
        mulxq   16(x), %rax, %r11
        adcq    %rax, %r10
        mulxq   24(x), %rax, %r12
        adcq    %rax, %r11
        adcq    zero, %r12

// Add row 1

        xorl    zeroe, zeroe
        movq    8(y), %rdx
        mulpadd(%r10,%r9,(x))
        mulpadd(%r11,%r10,8(x))
        mulpadd(%r12,%r11,16(x))
        mulpade(%r13,%r12,24(x))
        adcxq   zero, %r13

// Add row 2

        xorl    zeroe, zeroe
        movq    16(y), %rdx
        mulpadd(%r11,%r10,(x))
        mulpadd(%r12,%r11,8(x))
        mulpadd(%r13,%r12,16(x))
        mulpade(%r14,%r13,24(x));
        adcxq   zero, %r14

// Add row 3

        xorl    zeroe, zeroe
        movq    24(y), %rdx
        mulpadd(%r12,%r11,(x))
        mulpadd(%r13,%r12,8(x))
        mulpadd(%r14,%r13,16(x));
        mulpade(%r15,%r14,24(x));
        adcxq   zero, %r15

// Now we have the full 8-digit product 2^256 * h + l where
// h = [%r15,%r14,%r13,%r12] and l = [%r11,%r10,%r9,%r8]
// and this is == 4294968273 * h + l (mod p_256k1)

        movq    $4294968273, %rdx

        xorl    zeroe, zeroe

        mulpadd(%r9,%r8,%r12)
        mulpadd(%r10,%r9,%r13)
        mulpadd(%r11,%r10,%r14)
        mulpade(%r12,%r11,%r15)
        adcxq   zero, %r12

// Now we have reduced to 5 digits, 2^256 * h + l = [%r12,%r11,%r10,%r9,%r8]
// Use q = h + 1 as the initial quotient estimate, either right or 1 too big.

        leaq    1(%r12), %rax
        mulxq   %rax, %rax, %rbx
        addq    %rax, %r8
        adcq    %rbx, %r9
        adcq    zero, %r10
        adcq    zero, %r11

// Now the effective answer is 2^256 * (CF - 1) + [%r11,%r10,%r9,%r8]
// So we correct if CF = 0 by subtracting 4294968273, i.e. by
// adding p_256k1 to the "full" answer

        cmovcq  zero, %rdx
        subq    %rdx, %r8
        sbbq    zero, %r9
        sbbq    zero, %r10
        sbbq    zero, %r11

// Write everything back

        movq    %r8, (z)
        movq    %r9, 8(z)
        movq    %r10, 16(z)
        movq    %r11, 24(z)

// Restore registers and return

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_mul_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
