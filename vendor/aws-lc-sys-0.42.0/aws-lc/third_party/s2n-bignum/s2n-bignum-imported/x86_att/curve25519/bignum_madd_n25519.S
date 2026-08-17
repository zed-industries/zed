// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Multiply-add modulo the order of the curve25519/edwards25519 basepoint
// Inputs x[4], y[4], c[4]; output z[4]
//
//    extern void bignum_madd_n25519(uint64_t z[static 4], const uint64_t x[static 4],
//                                   const uint64_t y[static 4],
//                                   const uint64_t c[static 4]);
//
// Performs z := (x * y + c) mod n_25519, where the modulus is
// n_25519 = 2^252 + 27742317777372353535851937790883648493, the
// order of the curve25519/edwards25519 basepoint. The result z
// and the inputs x, y and c are all 4 digits (256 bits).
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y, RCX = c
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y, R9 = c
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_madd_n25519)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_madd_n25519)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_madd_n25519)
        .text

// Single round of modular reduction mod_n25519, mapping
// [m4;m3;m2;m1;m0] = m to [m3;m2;m1;m0] = m mod n_25519,
// *assuming* the input m < 2^64 * n_25519. This is very
// close to the loop body of the bignum_mod_n25519 function.

#define reduce(m4,m3,m2,m1,m0)                                  \
        movq    m4, %rbx ;                                        \
        shldq   $0x4, m3, %rbx ;                                   \
        shrq    $0x3c, m4 ;                                       \
        subq    m4, %rbx ;                                        \
        shlq    $0x4, m3 ;                                        \
        shrdq   $0x4, m4, m3 ;                                    \
        movabsq $0x5812631a5cf5d3ed, %rax ;                        \
        mulq    %rbx;                                            \
        movq    %rax, %rbp ;                                       \
        movq    %rdx, %rcx ;                                       \
        movabsq $0x14def9dea2f79cd6, %rax ;                        \
        mulq    %rbx;                                            \
        addq    %rax, %rcx ;                                       \
        adcq    $0x0, %rdx ;                                       \
        subq    %rbp, m0 ;                                        \
        sbbq    %rcx, m1 ;                                        \
        sbbq    %rdx, m2 ;                                        \
        sbbq    $0x0, m3 ;                                        \
        sbbq    %rbx, %rbx ;                                       \
        movabsq $0x5812631a5cf5d3ed, %rax ;                        \
        andq    %rbx, %rax ;                                       \
        movabsq $0x14def9dea2f79cd6, %rdx ;                        \
        andq    %rbx, %rdx ;                                       \
        movabsq $0x1000000000000000, %rbx ;                        \
        andq    %rax, %rbx ;                                       \
        addq    %rax, m0 ;                                        \
        adcq    %rdx, m1 ;                                        \
        adcq    $0x0, m2 ;                                        \
        adcq    %rbx, m3

// Special case of "reduce" with m4 = 0. As well as not using m4,
// the quotient selection is slightly simpler, just floor(m/2^252)
// versus min (floor(m/2^252)) (2^63-1).

#define reduce0(m3,m2,m1,m0)                                    \
        movq    m3, %rbx ;                                        \
        shrq    $60, %rbx ;                                        \
        shlq    $4, m3 ;                                          \
        shrq    $4, m3 ;                                          \
        movabsq $0x5812631a5cf5d3ed, %rax ;                        \
        mulq    %rbx;                                            \
        movq    %rax, %rbp ;                                       \
        movq    %rdx, %rcx ;                                       \
        movabsq $0x14def9dea2f79cd6, %rax ;                        \
        mulq    %rbx;                                            \
        addq    %rax, %rcx ;                                       \
        adcq    $0x0, %rdx ;                                       \
        subq    %rbp, m0 ;                                        \
        sbbq    %rcx, m1 ;                                        \
        sbbq    %rdx, m2 ;                                        \
        sbbq    $0x0, m3 ;                                        \
        sbbq    %rbx, %rbx ;                                       \
        movabsq $0x5812631a5cf5d3ed, %rax ;                        \
        andq    %rbx, %rax ;                                       \
        movabsq $0x14def9dea2f79cd6, %rdx ;                        \
        andq    %rbx, %rdx ;                                       \
        movabsq $0x1000000000000000, %rbx ;                        \
        andq    %rax, %rbx ;                                       \
        addq    %rax, m0 ;                                        \
        adcq    %rdx, m1 ;                                        \
        adcq    $0x0, m2 ;                                        \
        adcq    %rbx, m3

S2N_BN_SYMBOL(bignum_madd_n25519):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
#endif

// Save some additional registers for use

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// First compute [%r15;%r14;%r13;%r12;%r11;%r10;%r9;%r8] = x * y + c. This is
// a multiply-add variant of an ADCX/ADOX-based schoolbook multiplier,
// starting the accumulation with the c term and doing the zeroth row
// in the same uniform fashion, otherwise similar to the start of
// bignum_mul_p256k1.

        movq    (%rcx), %r8
        movq    8(%rcx), %r9
        movq    16(%rcx), %r10
        movq    24(%rcx), %r11
        movq    %rdx, %rcx
        xorl    %ebp, %ebp
        movq    (%rcx), %rdx
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x18(%rsi), %rax, %r12
        adcxq   %rax, %r11
        adoxq   %rbp, %r12
        adcxq   %rbp, %r12
        xorl    %ebp, %ebp
        movq    0x8(%rcx), %rdx
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x18(%rsi), %rax, %r13
        adcxq   %rax, %r12
        adoxq   %rbp, %r13
        adcxq   %rbp, %r13
        xorl    %ebp, %ebp
        movq    0x10(%rcx), %rdx
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x18(%rsi), %rax, %r14
        adcxq   %rax, %r13
        adoxq   %rbp, %r14
        adcxq   %rbp, %r14
        xorl    %ebp, %ebp
        movq    0x18(%rcx), %rdx
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   0x18(%rsi), %rax, %r15
        adcxq   %rax, %r14
        adoxq   %rbp, %r15
        adcxq   %rbp, %r15

// Now do the modular reduction and write back

        reduce0(%r15,%r14,%r13,%r12)
        reduce(%r15,%r14,%r13,%r12,%r11)
        reduce(%r14,%r13,%r12,%r11,%r10)
        reduce(%r13,%r12,%r11,%r10,%r9)
        reduce(%r12,%r11,%r10,%r9,%r8)

        movq    %r8, (%rdi)
        movq    %r9, 8(%rdi)
        movq    %r10, 16(%rdi)
        movq    %r11, 24(%rdi)

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

S2N_BN_SIZE_DIRECTIVE(bignum_madd_n25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
