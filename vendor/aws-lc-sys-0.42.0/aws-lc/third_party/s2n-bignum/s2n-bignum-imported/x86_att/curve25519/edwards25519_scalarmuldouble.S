// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Double scalar multiplication for edwards25519, fresh and base point
// Input scalar[4], point[8], bscalar[4]; output res[8]
//
// extern void edwards25519_scalarmuldouble
//   (uint64_t res[static 8],const uint64_t scalar[static 4],
//    const uint64_t point[static 8],const uint64_t bscalar[static 4]);
//
// Given scalar = n, point = P and bscalar = m, returns in res
// the point (X,Y) = n * P + m * B where B = (...,4/5) is
// the standard basepoint for the edwards25519 (Ed25519) curve.
//
// Both 256-bit coordinates of the input point P are implicitly
// reduced modulo 2^255-19 if they are not already in reduced form,
// but the conventional usage is that they *are* already reduced.
// The scalars can be arbitrary 256-bit numbers but may also be
// considered as implicitly reduced modulo the group order.
//
// Standard x86-64 ABI: RDI = res, RSI = scalar, RDX = point, RCX = bscalar
// Microsoft x64 ABI:   RCX = res, RDX = scalar, R8 = point, R9 = bscalar
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_scalarmuldouble)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_scalarmuldouble)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_scalarmuldouble)


        .text

// Size of individual field elements

#define NUMSIZE 32

// Pointer-offset pairs for result and temporaries on stack with some aliasing.
// Both "resx" and "resy" assume the "res" pointer has been preloaded into %rbp.

#define resx (0*NUMSIZE)(%rbp)
#define resy (1*NUMSIZE)(%rbp)

#define scalar (0*NUMSIZE)(%rsp)
#define bscalar (1*NUMSIZE)(%rsp)

#define tabent (2*NUMSIZE)(%rsp)
#define btabent (6*NUMSIZE)(%rsp)

#define acc (9*NUMSIZE)(%rsp)

#define tab (13*NUMSIZE)(%rsp)

// Additional variables kept on the stack

#define bf  45*NUMSIZE(%rsp)
#define cf  45*NUMSIZE+8(%rsp)
#define i  45*NUMSIZE+16(%rsp)
#define res  45*NUMSIZE+24(%rsp)

// Total size to reserve on the stack (excluding local subroutines)

#define NSPACE 46*NUMSIZE

// Syntactic variants to make x86_att forms easier to generate

#define SCALAR (0*NUMSIZE)
#define BSCALAR (1*NUMSIZE)
#define TABENT (2*NUMSIZE)
#define BTABENT (6*NUMSIZE)
#define ACC (9*NUMSIZE)
#define TAB (13*NUMSIZE)

// Sub-references used in local subroutines with local stack

#define x_0 0(%rdi)
#define y_0 NUMSIZE(%rdi)
#define z_0 (2*NUMSIZE)(%rdi)
#define w_0 (3*NUMSIZE)(%rdi)

#define x_1 0(%rsi)
#define y_1 NUMSIZE(%rsi)
#define z_1 (2*NUMSIZE)(%rsi)
#define w_1 (3*NUMSIZE)(%rsi)

#define x_2 0(%rbp)
#define y_2 NUMSIZE(%rbp)
#define z_2 (2*NUMSIZE)(%rbp)
#define w_2 (3*NUMSIZE)(%rbp)

#define t0 (0*NUMSIZE)(%rsp)
#define t1 (1*NUMSIZE)(%rsp)
#define t2 (2*NUMSIZE)(%rsp)
#define t3 (3*NUMSIZE)(%rsp)
#define t4 (4*NUMSIZE)(%rsp)
#define t5 (5*NUMSIZE)(%rsp)

// Macro wrapping up the basic field multiplication, only trivially
// different from a pure function call to bignum_mul_p25519.

#define mul_p25519(P0,P1,P2)                    \
        xorl   %ecx, %ecx ;                        \
        movq   P2, %rdx ;                       \
        mulxq  P1, %r8, %r9 ;                    \
        mulxq  0x8+P1, %rax, %r10 ;              \
        addq   %rax, %r9 ;                         \
        mulxq  0x10+P1, %rax, %r11 ;             \
        adcq   %rax, %r10 ;                        \
        mulxq  0x18+P1, %rax, %r12 ;             \
        adcq   %rax, %r11 ;                        \
        adcq   %rcx, %r12 ;                        \
        xorl   %ecx, %ecx ;                        \
        movq   0x8+P2, %rdx ;                   \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r9 ;                         \
        adoxq  %rbx, %r10 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x18+P1, %rax, %r13 ;             \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rcx, %r13 ;                        \
        adcxq  %rcx, %r13 ;                        \
        xorl   %ecx, %ecx ;                        \
        movq   0x10+P2, %rdx ;                  \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rbx, %r13 ;                        \
        mulxq  0x18+P1, %rax, %r14 ;             \
        adcxq  %rax, %r13 ;                        \
        adoxq  %rcx, %r14 ;                        \
        adcxq  %rcx, %r14 ;                        \
        xorl   %ecx, %ecx ;                        \
        movq   0x18+P2, %rdx ;                  \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rbx, %r13 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r13 ;                        \
        adoxq  %rbx, %r14 ;                        \
        mulxq  0x18+P1, %rax, %r15 ;             \
        adcxq  %rax, %r14 ;                        \
        adoxq  %rcx, %r15 ;                        \
        adcxq  %rcx, %r15 ;                        \
        movl   $0x26, %edx ;                       \
        xorl   %ecx, %ecx ;                        \
        mulxq  %r12, %rax, %rbx ;                   \
        adcxq  %rax, %r8 ;                         \
        adoxq  %rbx, %r9 ;                         \
        mulxq  %r13, %rax, %rbx ;                   \
        adcxq  %rax, %r9 ;                         \
        adoxq  %rbx, %r10 ;                        \
        mulxq  %r14, %rax, %rbx ;                   \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  %r15, %rax, %r12 ;                   \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rcx, %r12 ;                        \
        adcxq  %rcx, %r12 ;                        \
        shldq  $0x1, %r11, %r12 ;                   \
        movl   $0x13, %edx ;                       \
        incq   %r12;                             \
        bts    $63, %r11 ;                         \
        mulxq  %r12, %rax, %rbx ;                   \
        addq   %rax, %r8 ;                         \
        adcq   %rbx, %r9 ;                         \
        adcq   %rcx, %r10 ;                        \
        adcq   %rcx, %r11 ;                        \
        sbbq   %rax, %rax ;                        \
        notq   %rax;                             \
        andq   %rdx, %rax ;                        \
        subq   %rax, %r8 ;                         \
        sbbq   %rcx, %r9 ;                         \
        sbbq   %rcx, %r10 ;                        \
        sbbq   %rcx, %r11 ;                        \
        btr    $63, %r11 ;                         \
        movq   %r8, P0 ;                        \
        movq   %r9, 0x8+P0 ;                    \
        movq   %r10, 0x10+P0 ;                  \
        movq   %r11, 0x18+P0

// A version of multiplication that only guarantees output < 2 * p_25519.
// This basically skips the +1 and final correction in quotient estimation.

#define mul_4(P0,P1,P2)                         \
        xorl   %ecx, %ecx ;                        \
        movq   P2, %rdx ;                       \
        mulxq  P1, %r8, %r9 ;                    \
        mulxq  0x8+P1, %rax, %r10 ;              \
        addq   %rax, %r9 ;                         \
        mulxq  0x10+P1, %rax, %r11 ;             \
        adcq   %rax, %r10 ;                        \
        mulxq  0x18+P1, %rax, %r12 ;             \
        adcq   %rax, %r11 ;                        \
        adcq   %rcx, %r12 ;                        \
        xorl   %ecx, %ecx ;                        \
        movq   0x8+P2, %rdx ;                   \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r9 ;                         \
        adoxq  %rbx, %r10 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x18+P1, %rax, %r13 ;             \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rcx, %r13 ;                        \
        adcxq  %rcx, %r13 ;                        \
        xorl   %ecx, %ecx ;                        \
        movq   0x10+P2, %rdx ;                  \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rbx, %r13 ;                        \
        mulxq  0x18+P1, %rax, %r14 ;             \
        adcxq  %rax, %r13 ;                        \
        adoxq  %rcx, %r14 ;                        \
        adcxq  %rcx, %r14 ;                        \
        xorl   %ecx, %ecx ;                        \
        movq   0x18+P2, %rdx ;                  \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rbx, %r13 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r13 ;                        \
        adoxq  %rbx, %r14 ;                        \
        mulxq  0x18+P1, %rax, %r15 ;             \
        adcxq  %rax, %r14 ;                        \
        adoxq  %rcx, %r15 ;                        \
        adcxq  %rcx, %r15 ;                        \
        movl   $0x26, %edx ;                       \
        xorl   %ecx, %ecx ;                        \
        mulxq  %r12, %rax, %rbx ;                   \
        adcxq  %rax, %r8 ;                         \
        adoxq  %rbx, %r9 ;                         \
        mulxq  %r13, %rax, %rbx ;                   \
        adcxq  %rax, %r9 ;                         \
        adoxq  %rbx, %r10 ;                        \
        mulxq  %r14, %rax, %rbx ;                   \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  %r15, %rax, %r12 ;                   \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rcx, %r12 ;                        \
        adcxq  %rcx, %r12 ;                        \
        shldq  $0x1, %r11, %r12 ;                   \
        btr    $0x3f, %r11 ;                       \
        movl   $0x13, %edx ;                       \
        imulq  %r12, %rdx ;                        \
        addq   %rdx, %r8 ;                         \
        adcq   %rcx, %r9 ;                         \
        adcq   %rcx, %r10 ;                        \
        adcq   %rcx, %r11 ;                        \
        movq   %r8, P0 ;                        \
        movq   %r9, 0x8+P0 ;                    \
        movq   %r10, 0x10+P0 ;                  \
        movq   %r11, 0x18+P0

// Squaring just giving a result < 2 * p_25519, which is done by
// basically skipping the +1 in the quotient estimate and the final
// optional correction.

#define sqr_4(P0,P1)                            \
        movq   P1, %rdx ;                       \
        mulxq  %rdx, %r8, %r15 ;                    \
        mulxq  0x8+P1, %r9, %r10 ;               \
        mulxq  0x18+P1, %r11, %r12 ;             \
        movq   0x10+P1, %rdx ;                  \
        mulxq  0x18+P1, %r13, %r14 ;             \
        xorl   %ebx, %ebx ;                        \
        mulxq  P1, %rax, %rcx ;                  \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rcx, %r11 ;                        \
        mulxq  0x8+P1, %rax, %rcx ;              \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rcx, %r12 ;                        \
        movq   0x18+P1, %rdx ;                  \
        mulxq  0x8+P1, %rax, %rcx ;              \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rcx, %r13 ;                        \
        adcxq  %rbx, %r13 ;                        \
        adoxq  %rbx, %r14 ;                        \
        adcq   %rbx, %r14 ;                        \
        xorl   %ebx, %ebx ;                        \
        adcxq  %r9, %r9 ;                          \
        adoxq  %r15, %r9 ;                         \
        movq   0x8+P1, %rdx ;                   \
        mulxq  %rdx, %rax, %rdx ;                   \
        adcxq  %r10, %r10 ;                        \
        adoxq  %rax, %r10 ;                        \
        adcxq  %r11, %r11 ;                        \
        adoxq  %rdx, %r11 ;                        \
        movq   0x10+P1, %rdx ;                  \
        mulxq  %rdx, %rax, %rdx ;                   \
        adcxq  %r12, %r12 ;                        \
        adoxq  %rax, %r12 ;                        \
        adcxq  %r13, %r13 ;                        \
        adoxq  %rdx, %r13 ;                        \
        movq   0x18+P1, %rdx ;                  \
        mulxq  %rdx, %rax, %r15 ;                   \
        adcxq  %r14, %r14 ;                        \
        adoxq  %rax, %r14 ;                        \
        adcxq  %rbx, %r15 ;                        \
        adoxq  %rbx, %r15 ;                        \
        movl   $0x26, %edx ;                       \
        xorl   %ebx, %ebx ;                        \
        mulxq  %r12, %rax, %rcx ;                   \
        adcxq  %rax, %r8 ;                         \
        adoxq  %rcx, %r9 ;                         \
        mulxq  %r13, %rax, %rcx ;                   \
        adcxq  %rax, %r9 ;                         \
        adoxq  %rcx, %r10 ;                        \
        mulxq  %r14, %rax, %rcx ;                   \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rcx, %r11 ;                        \
        mulxq  %r15, %rax, %r12 ;                   \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        adcxq  %rbx, %r12 ;                        \
        shldq  $0x1, %r11, %r12 ;                   \
        btr    $0x3f, %r11 ;                       \
        movl   $0x13, %edx ;                       \
        imulq  %r12, %rdx ;                        \
        addq   %rdx, %r8 ;                         \
        adcq   %rbx, %r9 ;                         \
        adcq   %rbx, %r10 ;                        \
        adcq   %rbx, %r11 ;                        \
        movq   %r8, P0 ;                        \
        movq   %r9, 0x8+P0 ;                    \
        movq   %r10, 0x10+P0 ;                  \
        movq   %r11, 0x18+P0

// Modular subtraction with double modulus 2 * p_25519 = 2^256 - 38

#define sub_twice4(P0,P1,P2)                    \
        movq    P1, %r8 ;                       \
        xorl    %ebx, %ebx ;                       \
        subq    P2, %r8 ;                       \
        movq    8+P1, %r9 ;                     \
        sbbq    8+P2, %r9 ;                     \
        movl    $38, %ecx ;                        \
        movq    16+P1, %r10 ;                   \
        sbbq    16+P2, %r10 ;                   \
        movq    24+P1, %rax ;                   \
        sbbq    24+P2, %rax ;                   \
        cmovncq %rbx, %rcx ;                       \
        subq    %rcx, %r8 ;                        \
        sbbq    %rbx, %r9 ;                        \
        sbbq    %rbx, %r10 ;                       \
        sbbq    %rbx, %rax ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 8+P0 ;                     \
        movq    %r10, 16+P0 ;                   \
        movq    %rax, 24+P0

// Modular addition and doubling with double modulus 2 * p_25519 = 2^256 - 38.
// This only ensures that the result fits in 4 digits, not that it is reduced
// even w.r.t. double modulus. The result is always correct modulo provided
// the sum of the inputs is < 2^256 + 2^256 - 38, so in particular provided
// at least one of them is reduced double modulo.

#define add_twice4(P0,P1,P2)                    \
        movq    P1, %r8 ;                       \
        xorl    %ecx, %ecx ;                       \
        addq    P2, %r8 ;                       \
        movq    0x8+P1, %r9 ;                   \
        adcq    0x8+P2, %r9 ;                   \
        movq    0x10+P1, %r10 ;                 \
        adcq    0x10+P2, %r10 ;                 \
        movq    0x18+P1, %r11 ;                 \
        adcq    0x18+P2, %r11 ;                 \
        movl    $38, %eax ;                        \
        cmovncq %rcx, %rax ;                       \
        addq    %rax, %r8 ;                        \
        adcq    %rcx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

#define double_twice4(P0,P1)                    \
        movq    P1, %r8 ;                       \
        xorl    %ecx, %ecx ;                       \
        addq    %r8, %r8 ;                         \
        movq    0x8+P1, %r9 ;                   \
        adcq    %r9, %r9 ;                         \
        movq    0x10+P1, %r10 ;                 \
        adcq    %r10, %r10 ;                       \
        movq    0x18+P1, %r11 ;                 \
        adcq    %r11, %r11 ;                       \
        movl    $38, %eax ;                        \
        cmovncq %rcx, %rax ;                       \
        addq    %rax, %r8 ;                        \
        adcq    %rcx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Load the constant k_25519 = 2 * d_25519 using immediate operations

#define load_k25519(P0)                         \
        movq    $0xebd69b9426b2f159, %rax ;        \
        movq    %rax, P0 ;                      \
        movq    $0x00e0149a8283b156, %rax ;        \
        movq    %rax, 8+P0 ;                    \
        movq    $0x198e80f2eef3d130, %rax ;        \
        movq    %rax, 16+P0 ;                   \
        movq    $0x2406d9dc56dffce7, %rax ;        \
        movq    %rax, 24+P0

S2N_BN_SYMBOL(edwards25519_scalarmuldouble):
        CFI_START
        _CET_ENDBR

// In this case the Windows form literally makes a subroutine call.
// This avoids hassle arising from keeping code and data together.

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
        CFI_CALL(Ledwards25519_scalarmuldouble_standard)
        CFI_POP(%rsi)
        CFI_POP(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(edwards25519_scalarmuldouble)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_scalarmuldouble_standard)

Ledwards25519_scalarmuldouble_standard:
        CFI_START
#endif

// Save registers, make room for temps, preserve input arguments.

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(NSPACE)

// Move the output pointer to a stable place

        movq    %rdi, res

// Copy scalars while recoding all 4-bit nybbles except the top
// one (bits 252..255) into signed 4-bit digits. This is essentially
// done just by adding the recoding constant 0x0888..888, after
// which all digits except the first have an implicit bias of -8,
// so 0 -> -8, 1 -> -7, ... 7 -> -1, 8 -> 0, 9 -> 1, ... 15 -> 7.
// (We could literally create 2s complement signed nybbles by
// XORing with the same constant 0x0888..888 afterwards, but it
// doesn't seem to make the end usage any simpler.)
//
// In order to ensure that the unrecoded top nybble (bits 252..255)
// does not become > 8 as a result of carries lower down from the
// recoding, we first (conceptually) subtract the group order iff
// the top digit of the scalar is > 2^63. In the implementation the
// reduction and recoding are combined by optionally using the
// modified recoding constant 0x0888...888 + (2^256 - group_order).

        movq    (%rcx), %r8
        movq    8(%rcx), %r9
        movq    16(%rcx), %r10
        movq    24(%rcx), %r11
        movq    $0xc7f56fb5a0d9e920, %r12
        movq    $0xe190b99370cba1d5, %r13
        movq    $0x8888888888888887, %r14
        movq    $0x8888888888888888, %r15
        movq    $0x8000000000000000, %rax
        movq    $0x0888888888888888, %rbx
        cmpq    %r11, %rax
        cmovncq %r15, %r12
        cmovncq %r15, %r13
        cmovncq %r15, %r14
        cmovncq %rbx, %r15
        addq    %r12, %r8
        adcq    %r13, %r9
        adcq    %r14, %r10
        adcq    %r15, %r11
        movq    %r8, BSCALAR(%rsp)
        movq    %r9, BSCALAR+8(%rsp)
        movq    %r10, BSCALAR+16(%rsp)
        movq    %r11, BSCALAR+24(%rsp)

        movq    (%rsi), %r8
        movq    8(%rsi), %r9
        movq    16(%rsi), %r10
        movq    24(%rsi), %r11
        movq    $0xc7f56fb5a0d9e920, %r12
        movq    $0xe190b99370cba1d5, %r13
        movq    $0x8888888888888887, %r14
        movq    $0x8888888888888888, %r15
        movq    $0x8000000000000000, %rax
        movq    $0x0888888888888888, %rbx
        cmpq    %r11, %rax
        cmovncq %r15, %r12
        cmovncq %r15, %r13
        cmovncq %r15, %r14
        cmovncq %rbx, %r15
        addq    %r12, %r8
        adcq    %r13, %r9
        adcq    %r14, %r10
        adcq    %r15, %r11
        movq    %r8, SCALAR(%rsp)
        movq    %r9, SCALAR+8(%rsp)
        movq    %r10, SCALAR+16(%rsp)
        movq    %r11, SCALAR+24(%rsp)

// Create table of multiples 1..8 of the general input point at "tab".
// Reduce the input coordinates x and y modulo 2^256 - 38 first, for the
// sake of definiteness; this is the reduction that will be maintained.
// We could slightly optimize the additions because we know the input
// point is affine (so Z = 1), but it doesn't seem worth the complication.

        movl    $38, %eax
        movq    (%rdx), %r8
        xorl    %ebx, %ebx
        movq    8(%rdx), %r9
        xorl    %ecx, %ecx
        movq    16(%rdx), %r10
        xorl    %esi, %esi
        movq    24(%rdx), %r11
        addq    %r8, %rax
        adcq    %r9, %rbx
        adcq    %r10, %rcx
        adcq    %r11, %rsi
        cmovncq %r8, %rax
        movq    %rax, TAB(%rsp)
        cmovncq %r9, %rbx
        movq    %rbx, TAB+8(%rsp)
        cmovncq %r10, %rcx
        movq    %rcx, TAB+16(%rsp)
        cmovncq %r11, %rsi
        movq    %rsi, TAB+24(%rsp)

        movl    $38, %eax
        movq    32(%rdx), %r8
        xorl    %ebx, %ebx
        movq    40(%rdx), %r9
        xorl    %ecx, %ecx
        movq    48(%rdx), %r10
        xorl    %esi, %esi
        movq    56(%rdx), %r11
        addq    %r8, %rax
        adcq    %r9, %rbx
        adcq    %r10, %rcx
        adcq    %r11, %rsi
        cmovncq %r8, %rax
        movq    %rax, TAB+32(%rsp)
        cmovncq %r9, %rbx
        movq    %rbx, TAB+40(%rsp)
        cmovncq %r10, %rcx
        movq    %rcx, TAB+48(%rsp)
        cmovncq %r11, %rsi
        movq    %rsi, TAB+56(%rsp)

        movl    $1, %eax
        movq    %rax, TAB+64(%rsp)
        xorl    %eax, %eax
        movq    %rax, TAB+72(%rsp)
        movq    %rax, TAB+80(%rsp)
        movq    %rax, TAB+88(%rsp)

        leaq    TAB+96(%rsp), %rdi
        leaq    TAB(%rsp), %rsi
        leaq    TAB+32(%rsp), %rbp
        mul_4(x_0,x_1,x_2)

// Multiple 2

        leaq    TAB+1*128(%rsp), %rdi
        leaq    TAB(%rsp), %rsi
        CFI_CALL(Ledwards25519_scalarmuldouble_epdouble)

// Multiple 3

        leaq    TAB+2*128(%rsp), %rdi
        leaq    TAB(%rsp), %rsi
        leaq    TAB+1*128(%rsp), %rbp
        CFI_CALL(Ledwards25519_scalarmuldouble_epadd)

// Multiple 4

        leaq    TAB+3*128(%rsp), %rdi
        leaq    TAB+1*128(%rsp), %rsi
        CFI_CALL(Ledwards25519_scalarmuldouble_epdouble)

// Multiple 5

        leaq    TAB+4*128(%rsp), %rdi
        leaq    TAB(%rsp), %rsi
        leaq    TAB+3*128(%rsp), %rbp
        CFI_CALL(Ledwards25519_scalarmuldouble_epadd)

// Multiple 6

        leaq    TAB+5*128(%rsp), %rdi
        leaq    TAB+2*128(%rsp), %rsi
        CFI_CALL(Ledwards25519_scalarmuldouble_epdouble)

// Multiple 7

        leaq    TAB+6*128(%rsp), %rdi
        leaq    TAB(%rsp), %rsi
        leaq    TAB+5*128(%rsp), %rbp
        CFI_CALL(Ledwards25519_scalarmuldouble_epadd)

// Multiple 8

        leaq    TAB+7*128(%rsp), %rdi
        leaq    TAB+3*128(%rsp), %rsi
        CFI_CALL(Ledwards25519_scalarmuldouble_epdouble)

// Handle the initialization, starting the loop counter at i = 252
// and initializing acc to the sum of the table entries for the
// top nybbles of the scalars (the ones with no implicit -8 bias).

        movq    $252, %rax
        movq    %rax, i

// Index for btable entry...

        movq    BSCALAR+24(%rsp), %rax
        shrq    $60, %rax
        movq    %rax, bf

// ...and constant-time indexing based on that index

        movl    $1, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        movl    $1, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d
        xorl    %r12d, %r12d
        xorl    %r13d, %r13d
        xorl    %r14d, %r14d
        xorl    %r15d, %r15d

        leaq    Ledwards25519_scalarmuldouble_table(%rip), %rbp

        cmpq    $1, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $2, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $3, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $4, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $5, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $6, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $7, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $8, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15

        movq    %rax, BTABENT(%rsp)
        movq    %rbx, BTABENT+8(%rsp)
        movq    %rcx, BTABENT+16(%rsp)
        movq    %rdx, BTABENT+24(%rsp)
        movq    %r8, BTABENT+32(%rsp)
        movq    %r9, BTABENT+40(%rsp)
        movq    %r10, BTABENT+48(%rsp)
        movq    %r11, BTABENT+56(%rsp)
        movq    %r12, BTABENT+64(%rsp)
        movq    %r13, BTABENT+72(%rsp)
        movq    %r14, BTABENT+80(%rsp)
        movq    %r15, BTABENT+88(%rsp)

// Index for table entry...

        movq    SCALAR+24(%rsp), %rax
        shrq    $60, %rax
        movq    %rax, bf

// ...and constant-time indexing based on that index.
// Do the Y and Z fields first, to save on registers...

        movl    $1, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        movl    $1, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d

        leaq    TAB+32(%rsp), %rbp

        cmpq    $1, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $2, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $3, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $4, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $5, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $6, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $7, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $8, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11

        movq    %rax, TABENT+32(%rsp)
        movq    %rbx, TABENT+40(%rsp)
        movq    %rcx, TABENT+48(%rsp)
        movq    %rdx, TABENT+56(%rsp)
        movq    %r8, TABENT+64(%rsp)
        movq    %r9, TABENT+72(%rsp)
        movq    %r10, TABENT+80(%rsp)
        movq    %r11, TABENT+88(%rsp)

// ...followed by the X and W fields

        leaq    TAB(%rsp), %rbp

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        xorl    %r8d, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d

        cmpq    $1, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $2, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $3, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $4, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $5, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $6, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $7, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $8, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11

        movq    %rax, TABENT(%rsp)
        movq    %rbx, TABENT+8(%rsp)
        movq    %rcx, TABENT+16(%rsp)
        movq    %rdx, TABENT+24(%rsp)
        movq    %r8, TABENT+96(%rsp)
        movq    %r9, TABENT+104(%rsp)
        movq    %r10, TABENT+112(%rsp)
        movq    %r11, TABENT+120(%rsp)

// Add those elements to initialize the accumulator for bit position 252

        leaq    ACC(%rsp), %rdi
        leaq    TABENT(%rsp), %rsi
        leaq    BTABENT(%rsp), %rbp
        CFI_CALL(Ledwards25519_scalarmuldouble_pepadd)

// Main loop with acc = [scalar/2^i] * point + [bscalar/2^i] * basepoint
// Start with i = 252 for bits 248..251 and go down four at a time to 3..0

Ledwards25519_scalarmuldouble_loop:

        movq    i, %rax
        subq    $4, %rax
        movq    %rax, i

// Double to acc' = 2 * acc

        leaq    ACC(%rsp), %rdi
        leaq    ACC(%rsp), %rsi
        CFI_CALL(Ledwards25519_scalarmuldouble_pdouble)

// Get btable entry, first getting the adjusted bitfield...

        movq    i, %rax
        movq    %rax, %rcx
        shrq    $6, %rax
        movq    32(%rsp,%rax,8), %rax
        shrq    %cl, %rax
        andq    $15, %rax

        subq    $8, %rax
        sbbq    %rcx, %rcx
        xorq    %rcx, %rax
        subq    %rcx, %rax
        movq    %rcx, cf
        movq    %rax, bf

// ... then doing constant-time lookup with the appropriate index...

        movl    $1, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        movl    $1, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d
        xorl    %r12d, %r12d
        xorl    %r13d, %r13d
        xorl    %r14d, %r14d
        xorl    %r15d, %r15d

        leaq    Ledwards25519_scalarmuldouble_table(%rip), %rbp

        cmpq    $1, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $2, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $3, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $4, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $5, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $6, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $7, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $8, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15

// ... then optionally negating before storing. The table entry
// is in precomputed form and we currently have
//
//      [%rdx;%rcx;%rbx;%rax] = y - x
//      [%r11;%r10;%r9;%r8] = x + y
//      [%r15;%r14;%r13;%r12] = 2 * d * x * y
//
// Negation for Edwards curves is -(x,y) = (-x,y), which in this modified
// form amounts to swapping the first two fields and negating the third.
// The negation does not always fully reduce even mod 2^256-38 in the zero
// case, instead giving -0 = 2^256-38. But that is fine since the result is
// always fed to a multiplication inside the "pepadd" function below that
// handles any 256-bit input.

        movq    cf, %rdi
        testq   %rdi, %rdi

        movq    %rax, %rsi
        cmovnzq %r8, %rsi
        cmovnzq %rax, %r8
        movq    %rsi, BTABENT(%rsp)
        movq    %r8, BTABENT+32(%rsp)

        movq    %rbx, %rsi
        cmovnzq %r9, %rsi
        cmovnzq %rbx, %r9
        movq    %rsi, BTABENT+8(%rsp)
        movq    %r9, BTABENT+40(%rsp)

        movq    %rcx, %rsi
        cmovnzq %r10, %rsi
        cmovnzq %rcx, %r10
        movq    %rsi, BTABENT+16(%rsp)
        movq    %r10, BTABENT+48(%rsp)

        movq    %rdx, %rsi
        cmovnzq %r11, %rsi
        cmovnzq %rdx, %r11
        movq    %rsi, BTABENT+24(%rsp)
        movq    %r11, BTABENT+56(%rsp)

        xorq    %rdi, %r12
        xorq    %rdi, %r13
        xorq    %rdi, %r14
        xorq    %rdi, %r15
        andq    $37, %rdi
        subq    %rdi, %r12
        sbbq    $0, %r13
        sbbq    $0, %r14
        sbbq    $0, %r15
        movq    %r12, BTABENT+64(%rsp)
        movq    %r13, BTABENT+72(%rsp)
        movq    %r14, BTABENT+80(%rsp)
        movq    %r15, BTABENT+88(%rsp)

// Get table entry, first getting the adjusted bitfield...

        movq    i, %rax
        movq    %rax, %rcx
        shrq    $6, %rax
        movq    (%rsp,%rax,8), %rax
        shrq    %cl, %rax
        andq    $15, %rax

        subq    $8, %rax
        sbbq    %rcx, %rcx
        xorq    %rcx, %rax
        subq    %rcx, %rax
        movq    %rcx, cf
        movq    %rax, bf

// ...and constant-time indexing based on that index
// Do the Y and Z fields first, to save on registers
// and store them back (they don't need any modification)

        movl    $1, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        movl    $1, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d

        leaq    TAB+32(%rsp), %rbp

        cmpq    $1, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $2, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $3, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $4, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $5, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $6, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $7, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $8, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11

        movq    %rax, TABENT+32(%rsp)
        movq    %rbx, TABENT+40(%rsp)
        movq    %rcx, TABENT+48(%rsp)
        movq    %rdx, TABENT+56(%rsp)
        movq    %r8, TABENT+64(%rsp)
        movq    %r9, TABENT+72(%rsp)
        movq    %r10, TABENT+80(%rsp)
        movq    %r11, TABENT+88(%rsp)

// Now do the X and W fields...

        leaq    TAB(%rsp), %rbp

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        xorl    %r8d, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d

        cmpq    $1, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $2, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $3, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $4, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $5, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $6, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $7, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11
        addq    $128, %rbp

        cmpq    $8, bf
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    96(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    104(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    112(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    120(%rbp), %rsi
        cmovzq  %rsi, %r11

// ... then optionally negate before storing the X and W fields. This
// time the table entry is extended-projective, and is here:
//
//      [%rdx;%rcx;%rbx;%rax] = X
//      [tabent+32] = Y
//      [tabent+64] = Z
//      [%r11;%r10;%r9;%r8] = W
//
// This time we just need to negate the X and the W fields.
// The crude way negation is done can result in values of X or W
// (when initially zero before negation) being exactly equal to
// 2^256-38, but the "pepadd" function handles that correctly.

        movq    cf, %rdi

        xorq    %rdi, %rax
        xorq    %rdi, %rbx
        xorq    %rdi, %rcx
        xorq    %rdi, %rdx

        xorq    %rdi, %r8
        xorq    %rdi, %r9
        xorq    %rdi, %r10
        xorq    %rdi, %r11

        andq    $37, %rdi

        subq    %rdi, %rax
        sbbq    $0, %rbx
        sbbq    $0, %rcx
        sbbq    $0, %rdx

        movq    %rax, TABENT(%rsp)
        movq    %rbx, TABENT+8(%rsp)
        movq    %rcx, TABENT+16(%rsp)
        movq    %rdx, TABENT+24(%rsp)

        subq    %rdi, %r8
        sbbq    $0, %r9
        sbbq    $0, %r10
        sbbq    $0, %r11

        movq    %r8, TABENT+96(%rsp)
        movq    %r9, TABENT+104(%rsp)
        movq    %r10, TABENT+112(%rsp)
        movq    %r11, TABENT+120(%rsp)

// Double to acc' = 4 * acc

        leaq    ACC(%rsp), %rdi
        leaq    ACC(%rsp), %rsi
        CFI_CALL(Ledwards25519_scalarmuldouble_pdouble)

// Add tabent := tabent + btabent

        leaq    TABENT(%rsp), %rdi
        leaq    TABENT(%rsp), %rsi
        leaq    BTABENT(%rsp), %rbp
        CFI_CALL(Ledwards25519_scalarmuldouble_pepadd)

// Double to acc' = 8 * acc

        leaq    ACC(%rsp), %rdi
        leaq    ACC(%rsp), %rsi
        CFI_CALL(Ledwards25519_scalarmuldouble_pdouble)

// Double to acc' = 16 * acc

        leaq    ACC(%rsp), %rdi
        leaq    ACC(%rsp), %rsi
        CFI_CALL(Ledwards25519_scalarmuldouble_epdouble)

// Add table entry, acc := acc + tabent

        leaq    ACC(%rsp), %rdi
        leaq    ACC(%rsp), %rsi
        leaq    TABENT(%rsp), %rbp
        CFI_CALL(Ledwards25519_scalarmuldouble_epadd)

// Loop down

        movq    i, %rax
        testq   %rax, %rax
        jnz     Ledwards25519_scalarmuldouble_loop

// Prepare to call the modular inverse function to get tab = 1/z

        leaq    TAB(%rsp), %rdi
        leaq    ACC+64(%rsp), %rsi

// Inline copy of bignum_inv_p25519, identical except for stripping out
// the prologue and epilogue saving and restoring registers and making
// and reclaiming room on the stack. For more details and explanations see
// "x86/curve25519/bignum_inv_p25519.S". Note that the stack it uses for
// its own temporaries is 208 bytes, so it has no effect on variables
// that are needed in the rest of our computation here: res, tab and acc.

        movq    %rdi, 0xc0(%rsp)
        xorl    %eax, %eax
        leaq    -0x13(%rax), %rcx
        notq    %rax
        movq    %rcx, (%rsp)
        movq    %rax, 0x8(%rsp)
        movq    %rax, 0x10(%rsp)
        btr     $0x3f, %rax
        movq    %rax, 0x18(%rsp)
        movq    (%rsi), %rdx
        movq    0x8(%rsi), %rcx
        movq    0x10(%rsi), %r8
        movq    0x18(%rsi), %r9
        movl    $0x1, %eax
        xorl    %r10d, %r10d
        bts     $0x3f, %r9
        adcq    %r10, %rax
        imulq   $0x13, %rax, %rax
        addq    %rax, %rdx
        adcq    %r10, %rcx
        adcq    %r10, %r8
        adcq    %r10, %r9
        movl    $0x13, %eax
        cmovbq  %r10, %rax
        subq    %rax, %rdx
        sbbq    %r10, %rcx
        sbbq    %r10, %r8
        sbbq    %r10, %r9
        btr     $0x3f, %r9
        movq    %rdx, 0x20(%rsp)
        movq    %rcx, 0x28(%rsp)
        movq    %r8, 0x30(%rsp)
        movq    %r9, 0x38(%rsp)
        xorl    %eax, %eax
        movq    %rax, 0x40(%rsp)
        movq    %rax, 0x48(%rsp)
        movq    %rax, 0x50(%rsp)
        movq    %rax, 0x58(%rsp)
        movabsq $0xa0f99e2375022099, %rax
        movq    %rax, 0x60(%rsp)
        movabsq $0xa8c68f3f1d132595, %rax
        movq    %rax, 0x68(%rsp)
        movabsq $0x6c6c893805ac5242, %rax
        movq    %rax, 0x70(%rsp)
        movabsq $0x276508b241770615, %rax
        movq    %rax, 0x78(%rsp)
        movq    $0xa,  0x90(%rsp)
        movq    $0x1,  0x98(%rsp)
        jmp     Ledwards25519_scalarmuldouble_midloop
Ledwards25519_scalarmuldouble_inverseloop:
        movq    %r8, %r9
        sarq    $0x3f, %r9
        xorq    %r9, %r8
        subq    %r9, %r8
        movq    %r10, %r11
        sarq    $0x3f, %r11
        xorq    %r11, %r10
        subq    %r11, %r10
        movq    %r12, %r13
        sarq    $0x3f, %r13
        xorq    %r13, %r12
        subq    %r13, %r12
        movq    %r14, %r15
        sarq    $0x3f, %r15
        xorq    %r15, %r14
        subq    %r15, %r14
        movq    %r8, %rax
        andq    %r9, %rax
        movq    %r10, %rdi
        andq    %r11, %rdi
        addq    %rax, %rdi
        movq    %rdi, 0x80(%rsp)
        movq    %r12, %rax
        andq    %r13, %rax
        movq    %r14, %rsi
        andq    %r15, %rsi
        addq    %rax, %rsi
        movq    %rsi, 0x88(%rsp)
        xorl    %ebx, %ebx
        movq    (%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        movq    0x20(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        xorl    %ebp, %ebp
        movq    (%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    0x20(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        xorl    %ecx, %ecx
        movq    0x8(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    0x28(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        shrdq   $0x3b, %rbx, %rdi
        movq    %rdi, (%rsp)
        xorl    %edi, %edi
        movq    0x8(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rbp
        adcq    %rdx, %rdi
        movq    0x28(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rdi
        shrdq   $0x3b, %rbp, %rsi
        movq    %rsi, 0x20(%rsp)
        xorl    %esi, %esi
        movq    0x10(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rsi
        movq    0x30(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rdx, %rsi
        shrdq   $0x3b, %rcx, %rbx
        movq    %rbx, 0x8(%rsp)
        xorl    %ebx, %ebx
        movq    0x10(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        movq    0x30(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        shrdq   $0x3b, %rdi, %rbp
        movq    %rbp, 0x28(%rsp)
        movq    0x18(%rsp), %rax
        xorq    %r9, %rax
        movq    %rax, %rbp
        sarq    $0x3f, %rbp
        andq    %r8, %rbp
        negq    %rbp
        mulq    %r8
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    0x38(%rsp), %rax
        xorq    %r11, %rax
        movq    %rax, %rdx
        sarq    $0x3f, %rdx
        andq    %r10, %rdx
        subq    %rdx, %rbp
        mulq    %r10
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        shrdq   $0x3b, %rsi, %rcx
        movq    %rcx, 0x10(%rsp)
        shrdq   $0x3b, %rbp, %rsi
        movq    0x18(%rsp), %rax
        movq    %rsi, 0x18(%rsp)
        xorq    %r13, %rax
        movq    %rax, %rsi
        sarq    $0x3f, %rsi
        andq    %r12, %rsi
        negq    %rsi
        mulq    %r12
        addq    %rax, %rbx
        adcq    %rdx, %rsi
        movq    0x38(%rsp), %rax
        xorq    %r15, %rax
        movq    %rax, %rdx
        sarq    $0x3f, %rdx
        andq    %r14, %rdx
        subq    %rdx, %rsi
        mulq    %r14
        addq    %rax, %rbx
        adcq    %rdx, %rsi
        shrdq   $0x3b, %rbx, %rdi
        movq    %rdi, 0x30(%rsp)
        shrdq   $0x3b, %rsi, %rbx
        movq    %rbx, 0x38(%rsp)
        movq    0x80(%rsp), %rbx
        movq    0x88(%rsp), %rbp
        xorl    %ecx, %ecx
        movq    0x40(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    0x60(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        xorl    %esi, %esi
        movq    0x40(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rbx, 0x40(%rsp)
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    0x60(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    %rbp, 0x60(%rsp)
        xorl    %ebx, %ebx
        movq    0x48(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    0x68(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        xorl    %ebp, %ebp
        movq    0x48(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rcx, 0x48(%rsp)
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    0x68(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    %rsi, 0x68(%rsp)
        xorl    %ecx, %ecx
        movq    0x50(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    0x70(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        xorl    %esi, %esi
        movq    0x50(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rbx, 0x50(%rsp)
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    0x70(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    %rbp, 0x70(%rsp)
        movq    0x58(%rsp), %rax
        xorq    %r9, %rax
        movq    %r9, %rbx
        andq    %r8, %rbx
        negq    %rbx
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    0x78(%rsp), %rax
        xorq    %r11, %rax
        movq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %rbx
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rbx, %rdx
        movq    %rdx, %rbx
        shldq   $0x1, %rcx, %rdx
        sarq    $0x3f, %rbx
        addq    %rbx, %rdx
        movl    $0x13, %eax
        imulq   %rdx
        movq    0x40(%rsp), %r8
        addq    %rax, %r8
        movq    %r8, 0x40(%rsp)
        movq    0x48(%rsp), %r8
        adcq    %rdx, %r8
        movq    %r8, 0x48(%rsp)
        movq    0x50(%rsp), %r8
        adcq    %rbx, %r8
        movq    %r8, 0x50(%rsp)
        adcq    %rbx, %rcx
        shlq    $0x3f, %rax
        addq    %rax, %rcx
        movq    0x58(%rsp), %rax
        movq    %rcx, 0x58(%rsp)
        xorq    %r13, %rax
        movq    %r13, %rcx
        andq    %r12, %rcx
        negq    %rcx
        mulq    %r12
        addq    %rax, %rsi
        adcq    %rdx, %rcx
        movq    0x78(%rsp), %rax
        xorq    %r15, %rax
        movq    %r15, %rdx
        andq    %r14, %rdx
        subq    %rdx, %rcx
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rcx, %rdx
        movq    %rdx, %rcx
        shldq   $0x1, %rsi, %rdx
        sarq    $0x3f, %rcx
        movl    $0x13, %eax
        addq    %rcx, %rdx
        imulq   %rdx
        movq    0x60(%rsp), %r8
        addq    %rax, %r8
        movq    %r8, 0x60(%rsp)
        movq    0x68(%rsp), %r8
        adcq    %rdx, %r8
        movq    %r8, 0x68(%rsp)
        movq    0x70(%rsp), %r8
        adcq    %rcx, %r8
        movq    %r8, 0x70(%rsp)
        adcq    %rcx, %rsi
        shlq    $0x3f, %rax
        addq    %rax, %rsi
        movq    %rsi, 0x78(%rsp)
Ledwards25519_scalarmuldouble_midloop:
        movq    0x98(%rsp), %rsi
        movq    (%rsp), %rdx
        movq    0x20(%rsp), %rcx
        movq    %rdx, %rbx
        andq    $0xfffff, %rbx
        movabsq $0xfffffe0000000000, %rax
        orq     %rax, %rbx
        andq    $0xfffff, %rcx
        movabsq $0xc000000000000000, %rax
        orq     %rax, %rcx
        movq    $0xfffffffffffffffe, %rax
        xorl    %ebp, %ebp
        movl    $0x2, %edx
        movq    %rbx, %rdi
        movq    %rax, %r8
        testq   %rsi, %rsi
        cmovs   %rbp, %r8
        testq   $0x1, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        sarq    $1, %rcx
        movl    $0x100000, %eax
        leaq    (%rbx,%rax), %rdx
        leaq    (%rcx,%rax), %rdi
        shlq    $0x16, %rdx
        shlq    $0x16, %rdi
        sarq    $0x2b, %rdx
        sarq    $0x2b, %rdi
        movabsq $0x20000100000, %rax
        leaq    (%rbx,%rax), %rbx
        leaq    (%rcx,%rax), %rcx
        sarq    $0x2a, %rbx
        sarq    $0x2a, %rcx
        movq    %rdx, 0xa0(%rsp)
        movq    %rbx, 0xa8(%rsp)
        movq    %rdi, 0xb0(%rsp)
        movq    %rcx, 0xb8(%rsp)
        movq    (%rsp), %r12
        imulq   %r12, %rdi
        imulq   %rdx, %r12
        movq    0x20(%rsp), %r13
        imulq   %r13, %rbx
        imulq   %rcx, %r13
        addq    %rbx, %r12
        addq    %rdi, %r13
        sarq    $0x14, %r12
        sarq    $0x14, %r13
        movq    %r12, %rbx
        andq    $0xfffff, %rbx
        movabsq $0xfffffe0000000000, %rax
        orq     %rax, %rbx
        movq    %r13, %rcx
        andq    $0xfffff, %rcx
        movabsq $0xc000000000000000, %rax
        orq     %rax, %rcx
        movq    $0xfffffffffffffffe, %rax
        movl    $0x2, %edx
        movq    %rbx, %rdi
        movq    %rax, %r8
        testq   %rsi, %rsi
        cmovs   %rbp, %r8
        testq   $0x1, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        sarq    $1, %rcx
        movl    $0x100000, %eax
        leaq    (%rbx,%rax), %r8
        leaq    (%rcx,%rax), %r10
        shlq    $0x16, %r8
        shlq    $0x16, %r10
        sarq    $0x2b, %r8
        sarq    $0x2b, %r10
        movabsq $0x20000100000, %rax
        leaq    (%rbx,%rax), %r15
        leaq    (%rcx,%rax), %r11
        sarq    $0x2a, %r15
        sarq    $0x2a, %r11
        movq    %r13, %rbx
        movq    %r12, %rcx
        imulq   %r8, %r12
        imulq   %r15, %rbx
        addq    %rbx, %r12
        imulq   %r11, %r13
        imulq   %r10, %rcx
        addq    %rcx, %r13
        sarq    $0x14, %r12
        sarq    $0x14, %r13
        movq    %r12, %rbx
        andq    $0xfffff, %rbx
        movabsq $0xfffffe0000000000, %rax
        orq     %rax, %rbx
        movq    %r13, %rcx
        andq    $0xfffff, %rcx
        movabsq $0xc000000000000000, %rax
        orq     %rax, %rcx
        movq    0xa0(%rsp), %rax
        imulq   %r8, %rax
        movq    0xb0(%rsp), %rdx
        imulq   %r15, %rdx
        imulq   0xa8(%rsp), %r8
        imulq   0xb8(%rsp), %r15
        addq    %r8, %r15
        leaq    (%rax,%rdx), %r9
        movq    0xa0(%rsp), %rax
        imulq   %r10, %rax
        movq    0xb0(%rsp), %rdx
        imulq   %r11, %rdx
        imulq   0xa8(%rsp), %r10
        imulq   0xb8(%rsp), %r11
        addq    %r10, %r11
        leaq    (%rax,%rdx), %r13
        movq    $0xfffffffffffffffe, %rax
        movl    $0x2, %edx
        movq    %rbx, %rdi
        movq    %rax, %r8
        testq   %rsi, %rsi
        cmovs   %rbp, %r8
        testq   $0x1, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        cmovs   %rbp, %r8
        movq    %rbx, %rdi
        testq   %rdx, %rcx
        cmoveq  %rbp, %r8
        cmoveq  %rbp, %rdi
        sarq    $1, %rcx
        xorq    %r8, %rdi
        xorq    %r8, %rsi
        btq     $0x3f, %r8
        cmovbq  %rcx, %rbx
        movq    %rax, %r8
        subq    %rax, %rsi
        leaq    (%rcx,%rdi), %rcx
        sarq    $1, %rcx
        movl    $0x100000, %eax
        leaq    (%rbx,%rax), %r8
        leaq    (%rcx,%rax), %r12
        shlq    $0x15, %r8
        shlq    $0x15, %r12
        sarq    $0x2b, %r8
        sarq    $0x2b, %r12
        movabsq $0x20000100000, %rax
        leaq    (%rbx,%rax), %r10
        leaq    (%rcx,%rax), %r14
        sarq    $0x2b, %r10
        sarq    $0x2b, %r14
        movq    %r9, %rax
        imulq   %r8, %rax
        movq    %r13, %rdx
        imulq   %r10, %rdx
        imulq   %r15, %r8
        imulq   %r11, %r10
        addq    %r8, %r10
        leaq    (%rax,%rdx), %r8
        movq    %r9, %rax
        imulq   %r12, %rax
        movq    %r13, %rdx
        imulq   %r14, %rdx
        imulq   %r15, %r12
        imulq   %r11, %r14
        addq    %r12, %r14
        leaq    (%rax,%rdx), %r12
        movq    %rsi, 0x98(%rsp)
        decq     0x90(%rsp)
        jne     Ledwards25519_scalarmuldouble_inverseloop
        movq    (%rsp), %rax
        movq    0x20(%rsp), %rcx
        imulq   %r8, %rax
        imulq   %r10, %rcx
        addq    %rcx, %rax
        sarq    $0x3f, %rax
        movq    %r8, %r9
        sarq    $0x3f, %r9
        xorq    %r9, %r8
        subq    %r9, %r8
        xorq    %rax, %r9
        movq    %r10, %r11
        sarq    $0x3f, %r11
        xorq    %r11, %r10
        subq    %r11, %r10
        xorq    %rax, %r11
        movq    %r12, %r13
        sarq    $0x3f, %r13
        xorq    %r13, %r12
        subq    %r13, %r12
        xorq    %rax, %r13
        movq    %r14, %r15
        sarq    $0x3f, %r15
        xorq    %r15, %r14
        subq    %r15, %r14
        xorq    %rax, %r15
        movq    %r8, %rax
        andq    %r9, %rax
        movq    %r10, %r12
        andq    %r11, %r12
        addq    %rax, %r12
        xorl    %r13d, %r13d
        movq    0x40(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x60(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movq    0x48(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r13
        adcq    %rdx, %r14
        movq    0x68(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    0x50(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r14
        adcq    %rdx, %r15
        movq    0x70(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r14
        adcq    %rdx, %r15
        movq    0x58(%rsp), %rax
        xorq    %r9, %rax
        andq    %r8, %r9
        negq    %r9
        mulq    %r8
        addq    %rax, %r15
        adcq    %rdx, %r9
        movq    0x78(%rsp), %rax
        xorq    %r11, %rax
        movq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %r9
        mulq    %r10
        addq    %rax, %r15
        adcq    %rdx, %r9
        movq    %r9, %rax
        shldq   $0x1, %r15, %rax
        sarq    $0x3f, %r9
        movl    $0x13, %ebx
        leaq    0x1(%rax,%r9,1), %rax
        imulq   %rbx
        xorl    %ebp, %ebp
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r9, %r14
        adcq    %r9, %r15
        shlq    $0x3f, %rax
        addq    %rax, %r15
        cmovns  %rbp, %rbx
        subq    %rbx, %r12
        sbbq    %rbp, %r13
        sbbq    %rbp, %r14
        sbbq    %rbp, %r15
        btr     $0x3f, %r15
        movq    0xc0(%rsp), %rdi
        movq    %r12, (%rdi)
        movq    %r13, 0x8(%rdi)
        movq    %r14, 0x10(%rdi)
        movq    %r15, 0x18(%rdi)

// Store result

        movq    res, %rdi
        leaq    ACC(%rsp), %rsi
        leaq    TAB(%rsp), %rbp
        mul_p25519(x_0,x_1,x_2)

        movq    res, %rdi
        addq    $32, %rdi
        leaq    ACC+32(%rsp), %rsi
        leaq    TAB(%rsp), %rbp
        mul_p25519(x_0,x_1,x_2)

// Restore stack and registers

        CFI_INC_RSP(NSPACE)

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

#if WINDOWS_ABI
S2N_BN_SIZE_DIRECTIVE(Ledwards25519_scalarmuldouble_standard)
#else
S2N_BN_SIZE_DIRECTIVE(edwards25519_scalarmuldouble)
#endif

// ****************************************************************************
// Localized versions of subroutines.
// These are close to the standalone functions "edwards25519_epdouble" etc.,
// but are only maintaining reduction modulo 2^256 - 38, not 2^255 - 19.
// ****************************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_scalarmuldouble_epdouble)

Ledwards25519_scalarmuldouble_epdouble:
        CFI_START
        CFI_DEC_RSP(5*NUMSIZE)
        add_twice4(t0,x_1,y_1)
        sqr_4(t1,z_1)
        sqr_4(t2,x_1)
        sqr_4(t3,y_1)
        double_twice4(t1,t1)
        sqr_4(t0,t0)
        add_twice4(t4,t2,t3)
        sub_twice4(t2,t2,t3)
        add_twice4(t3,t1,t2)
        sub_twice4(t1,t4,t0)
        mul_4(y_0,t2,t4)
        mul_4(z_0,t3,t2)
        mul_4(w_0,t1,t4)
        mul_4(x_0,t1,t3)
        CFI_INC_RSP(5*NUMSIZE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_scalarmuldouble_epdouble)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_scalarmuldouble_pdouble)

Ledwards25519_scalarmuldouble_pdouble:
        CFI_START
        CFI_DEC_RSP(5*NUMSIZE)
        add_twice4(t0,x_1,y_1)
        sqr_4(t1,z_1)
        sqr_4(t2,x_1)
        sqr_4(t3,y_1)
        double_twice4(t1,t1)
        sqr_4(t0,t0)
        add_twice4(t4,t2,t3)
        sub_twice4(t2,t2,t3)
        add_twice4(t3,t1,t2)
        sub_twice4(t1,t4,t0)
        mul_4(y_0,t2,t4)
        mul_4(z_0,t3,t2)
        mul_4(x_0,t1,t3)
        CFI_INC_RSP(5*NUMSIZE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_scalarmuldouble_pdouble)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_scalarmuldouble_epadd)

Ledwards25519_scalarmuldouble_epadd:
        CFI_START
        CFI_DEC_RSP(6*NUMSIZE)
        mul_4(t0,w_1,w_2)
        sub_twice4(t1,y_1,x_1)
        sub_twice4(t2,y_2,x_2)
        add_twice4(t3,y_1,x_1)
        add_twice4(t4,y_2,x_2)
        double_twice4(t5,z_2)
        mul_4(t1,t1,t2)
        mul_4(t3,t3,t4)
        load_k25519(t2)
        mul_4(t2,t2,t0)
        mul_4(t4,z_1,t5)
        sub_twice4(t0,t3,t1)
        add_twice4(t5,t3,t1)
        sub_twice4(t1,t4,t2)
        add_twice4(t3,t4,t2)
        mul_4(w_0,t0,t5)
        mul_4(x_0,t0,t1)
        mul_4(y_0,t3,t5)
        mul_4(z_0,t1,t3)
        CFI_INC_RSP(6*NUMSIZE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_scalarmuldouble_epadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_scalarmuldouble_pepadd)

Ledwards25519_scalarmuldouble_pepadd:
        CFI_START
        CFI_DEC_RSP(6*NUMSIZE)
        double_twice4(t0,z_1);
        sub_twice4(t1,y_1,x_1);
        add_twice4(t2,y_1,x_1);
        mul_4(t3,w_1,z_2);
        mul_4(t1,t1,x_2);
        mul_4(t2,t2,y_2);
        sub_twice4(t4,t0,t3);
        add_twice4(t0,t0,t3);
        sub_twice4(t5,t2,t1);
        add_twice4(t1,t2,t1);
        mul_4(z_0,t4,t0);
        mul_4(x_0,t5,t4);
        mul_4(y_0,t0,t1);
        mul_4(w_0,t5,t1);
        CFI_INC_RSP(6*NUMSIZE)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_scalarmuldouble_pepadd)

S2N_BN_SIZE_DIRECTIVE(edwards25519_scalarmuldouble)

// ****************************************************************************
// The precomputed data (all read-only). This is currently part of the same
// text section, which gives position-independent code with simple PC-relative
// addressing. However it could be put in a separate section via something like
//
// .section .rodata
// ****************************************************************************

// Precomputed table of multiples of generator for edwards25519
// all in precomputed extended-projective (y-x,x+y,2*d*x*y) triples.

Ledwards25519_scalarmuldouble_table:

        // 1 * G

        .quad   0x9d103905d740913e
        .quad   0xfd399f05d140beb3
        .quad   0xa5c18434688f8a09
        .quad   0x44fd2f9298f81267
        .quad   0x2fbc93c6f58c3b85
        .quad   0xcf932dc6fb8c0e19
        .quad   0x270b4898643d42c2
        .quad   0x07cf9d3a33d4ba65
        .quad   0xabc91205877aaa68
        .quad   0x26d9e823ccaac49e
        .quad   0x5a1b7dcbdd43598c
        .quad   0x6f117b689f0c65a8

        // 2 * G

        .quad   0x8a99a56042b4d5a8
        .quad   0x8f2b810c4e60acf6
        .quad   0xe09e236bb16e37aa
        .quad   0x6bb595a669c92555
        .quad   0x9224e7fc933c71d7
        .quad   0x9f469d967a0ff5b5
        .quad   0x5aa69a65e1d60702
        .quad   0x590c063fa87d2e2e
        .quad   0x43faa8b3a59b7a5f
        .quad   0x36c16bdd5d9acf78
        .quad   0x500fa0840b3d6a31
        .quad   0x701af5b13ea50b73

        // 3 * G

        .quad   0x56611fe8a4fcd265
        .quad   0x3bd353fde5c1ba7d
        .quad   0x8131f31a214bd6bd
        .quad   0x2ab91587555bda62
        .quad   0xaf25b0a84cee9730
        .quad   0x025a8430e8864b8a
        .quad   0xc11b50029f016732
        .quad   0x7a164e1b9a80f8f4
        .quad   0x14ae933f0dd0d889
        .quad   0x589423221c35da62
        .quad   0xd170e5458cf2db4c
        .quad   0x5a2826af12b9b4c6

        // 4 * G

        .quad   0x95fe050a056818bf
        .quad   0x327e89715660faa9
        .quad   0xc3e8e3cd06a05073
        .quad   0x27933f4c7445a49a
        .quad   0x287351b98efc099f
        .quad   0x6765c6f47dfd2538
        .quad   0xca348d3dfb0a9265
        .quad   0x680e910321e58727
        .quad   0x5a13fbe9c476ff09
        .quad   0x6e9e39457b5cc172
        .quad   0x5ddbdcf9102b4494
        .quad   0x7f9d0cbf63553e2b

        // 5 * G

        .quad   0x7f9182c3a447d6ba
        .quad   0xd50014d14b2729b7
        .quad   0xe33cf11cb864a087
        .quad   0x154a7e73eb1b55f3
        .quad   0xa212bc4408a5bb33
        .quad   0x8d5048c3c75eed02
        .quad   0xdd1beb0c5abfec44
        .quad   0x2945ccf146e206eb
        .quad   0xbcbbdbf1812a8285
        .quad   0x270e0807d0bdd1fc
        .quad   0xb41b670b1bbda72d
        .quad   0x43aabe696b3bb69a

        // 6 * G

        .quad   0x499806b67b7d8ca4
        .quad   0x575be28427d22739
        .quad   0xbb085ce7204553b9
        .quad   0x38b64c41ae417884
        .quad   0x3a0ceeeb77157131
        .quad   0x9b27158900c8af88
        .quad   0x8065b668da59a736
        .quad   0x51e57bb6a2cc38bd
        .quad   0x85ac326702ea4b71
        .quad   0xbe70e00341a1bb01
        .quad   0x53e4a24b083bc144
        .quad   0x10b8e91a9f0d61e3

        // 7 * G

        .quad   0xba6f2c9aaa3221b1
        .quad   0x6ca021533bba23a7
        .quad   0x9dea764f92192c3a
        .quad   0x1d6edd5d2e5317e0
        .quad   0x6b1a5cd0944ea3bf
        .quad   0x7470353ab39dc0d2
        .quad   0x71b2528228542e49
        .quad   0x461bea69283c927e
        .quad   0xf1836dc801b8b3a2
        .quad   0xb3035f47053ea49a
        .quad   0x529c41ba5877adf3
        .quad   0x7a9fbb1c6a0f90a7

        // 8 * G

        .quad   0xe2a75dedf39234d9
        .quad   0x963d7680e1b558f9
        .quad   0x2c2741ac6e3c23fb
        .quad   0x3a9024a1320e01c3
        .quad   0x59b7596604dd3e8f
        .quad   0x6cb30377e288702c
        .quad   0xb1339c665ed9c323
        .quad   0x0915e76061bce52f
        .quad   0xe7c1f5d9c9a2911a
        .quad   0xb8a371788bcca7d7
        .quad   0x636412190eb62a32
        .quad   0x26907c5c2ecc4e95

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
