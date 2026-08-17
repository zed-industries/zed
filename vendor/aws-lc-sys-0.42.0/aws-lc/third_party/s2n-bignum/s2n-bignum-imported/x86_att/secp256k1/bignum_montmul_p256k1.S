// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^256) mod p_256k1
// Inputs x[4], y[4]; output z[4]
//
//    extern void bignum_montmul_p256k1(uint64_t z[static 4],
//                                      const uint64_t x[static 4],
//                                      const uint64_t y[static 4]);
//
// Does z := (2^{-256} * x * y) mod p_256k1, assuming that the inputs x and y
// satisfy x * y <= 2^256 * p_256k2 (in particular this is true if we are in
// the "usual" case x < p_256k1 and y < p_256k1).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p256k1)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p256k1)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p256k1)
        .text

// These are actually right

#define z %rdi
#define x %rsi

// Copied in or set up

#define y %rcx

// A zero register

#define zero %rbp
#define zeroe %ebp

// Also used for multiplicative inverse in second part

#define w %rbp

// mulpadd(high,low,m) adds %rdx * m to a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries.

#define mulpadd(high,low,m)             \
        mulxq   m, %rax, %rbx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rbx, high

// mulpade(high,low,i) adds %rdx * x[i] to a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax as a temporary, assuming high created from scratch
// and that zero has value zero.

#define mulpade(high,low,m)             \
        mulxq   m, %rax, high ;           \
        adcxq   %rax, low ;               \
        adoxq   zero, high

S2N_BN_SYMBOL(bignum_montmul_p256k1):
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
// Do Montgomery reductions, now using %rcx as a carry-saver.
// A direct carry chain is possible using mulx exclusively, but it
// requires more moves and overall seems to have lower performance.

        movq    $0xd838091dd2253531, w
        movq    $4294968273, %rbx

// Montgomery reduce row 0

        movq    %rbx, %rax
        imulq   w, %r8
        mulq    %r8
        subq    %rdx, %r9
        sbbq    %rcx, %rcx

// Montgomery reduce row 1

        movq    %rbx, %rax
        imulq   w, %r9
        mulq    %r9
        negq    %rcx
        sbbq    %rdx, %r10
        sbbq    %rcx, %rcx

// Montgomery reduce row 2

        movq    %rbx, %rax
        imulq   w, %r10
        mulq    %r10
        negq    %rcx
        sbbq    %rdx, %r11
        sbbq    %rcx, %rcx

// Montgomery reduce row 3

        movq    %rbx, %rax
        imulq   w, %r11
        mulq    %r11
        negq    %rcx

// Now [%r15,%r14,%r13,%r12] := [%r15,%r14,%r13,%r12] + [%r11,%r10,%r9,%r8] - (%rdx + CF)

        sbbq    %rdx, %r8
        sbbq    $0, %r9
        sbbq    $0, %r10
        sbbq    $0, %r11

        addq    %r8, %r12
        adcq    %r9, %r13
        adcq    %r10, %r14
        adcq    %r11, %r15
        sbbq    w, w

// Let b be the top carry captured just above as w = (2^64-1) * b
// Now if [b,%r15,%r14,%r13,%r12] >= p_256k1, subtract p_256k1, i.e. add 4294968273
// and either way throw away the top word. [b,%r15,%r14,%r13,%r12] - p_256k1 =
// [(b - 1),%r15,%r14,%r13,%r12] + 4294968273. If [%r15,%r14,%r13,%r12] + 4294968273
// gives carry flag CF then >= comparison is top = 0 <=> b - 1 + CF = 0 which
// is equivalent to b \/ CF, and so to (2^64-1) * b + (2^64 - 1) + CF >= 2^64

        movq    %r12, %r8
        addq    %rbx, %r8
        movq    %r13, %r9
        adcq    $0, %r9
        movq    %r14, %r10
        adcq    $0, %r10
        movq    %r15, %r11
        adcq    $0, %r11

        adcq    $-1, w

// Write everything back

        cmovcq  %r8, %r12
        movq    %r12, (z)
        cmovcq  %r9, %r13
        movq    %r13, 8(z)
        cmovcq  %r10, %r14
        movq    %r14, 16(z)
        cmovcq  %r11, %r15
        movq    %r15, 24(z)

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

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p256k1)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
