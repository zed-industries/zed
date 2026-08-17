// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply modulo p_25519, z := (x * y) mod p_25519
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_mul_p25519(uint64_t z[static 4], const uint64_t x[static 4],
//                                  const uint64_t y[static 4]);
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_mul_p25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_mul_p25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_mul_p25519)
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

S2N_BN_SYMBOL(bignum_mul_p25519):
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
        adcq    zero, %r13

// Add row 2

        xorl    zeroe, zeroe
        movq    16(y), %rdx
        mulpadd(%r11,%r10,(x))
        mulpadd(%r12,%r11,8(x))
        mulpadd(%r13,%r12,16(x))
        mulpade(%r14,%r13,24(x));
        adcq    zero, %r14

// Add row 3; also use an early 38*r15+r11 to get a quotient estimate q
// and then squeeze in a 19 * q computation to inject into the next
// double-carry chain. At the end %rcx = q and %rax = 19 * q.

        xorl    zeroe, zeroe
        movq    24(y), %rdx

        mulpadd(%r12,%r11,(x))

        mulxq   24(x), %rcx, %r15

        mulpadd(%r13,%r12,8(x))
        mulpadd(%r14,%r13,16(x))

        movl    $38, %edx
        mulxq   %r15, %rax, %rbx

        adcxq   %rcx, %r14
        adoxq   zero, %r15
        adcq    zero, %r15

        addq    %r11, %rax
        adcq    zero, %rbx
        btq     $63, %rax
        adcq    %rbx, %rbx
        leaq    1(%rbx), %rcx
        imulq   $19, %rcx

// Now we have the full 8-digit product 2^256 * h + l where
// h = [%r15,%r14,%r13,%r12] and l = [%r11,%r10,%r9,%r8]
// and this is == 38 * h + l (mod p_25519)
// We add in the precalculated 19 * q as well.
// This is kept in 4 words since we have enough information there.

        xorl    zeroe, zeroe
        adoxq   %rcx, %r8
        mulpadd(%r9,%r8,%r12)
        mulpadd(%r10,%r9,%r13)
        mulpadd(%r11,%r10,%r14)
        mulxq   %r15, %rax, %rbx
        adcq    %rax, %r11

// We still haven't made the -2^255 * q contribution yet. Since we
// are now safely in 4 words we just need a single bit of q, and we
// can actually use the LSB of %rcx = 19 * q since 19 is odd. And we
// don't literally need to subtract, just to see whether we would
// have a top 1 bit if we did, meaning we need to correct in the
// last step by adding 2^255 - 19.

        shlq    $63, %rcx
        cmpq    %rcx, %r11
        movl    $19, %eax
        cmovns  zero, %rax

// Now make that possible correction and finally mask to 255 bits

        subq    %rax, %r8
        sbbq    zero, %r9
        sbbq    zero, %r10
        sbbq    zero, %r11
        btr     $63, %r11

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

S2N_BN_SIZE_DIRECTIVE(bignum_mul_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
