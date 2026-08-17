// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Modular inverse modulo p_384 = 2^384 - 2^128 - 2^96 + 2^32 - 1
// Input x[6]; output z[6]
//
// extern void bignum_inv_p384(uint64_t z[static 6],const uint64_t x[static 6]);
//
// If the 6-digit input x is coprime to p_384, i.e. is not divisible
// by it, returns z < p_384 such that x * z == 1 (mod p_384). Note that
// x does not need to be reduced modulo p_384, but the output always is.
// If the input is divisible (i.e. is 0 or p_384), then there can be no
// modular inverse and z = 0 is returned.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_inv_p384)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_inv_p384)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_inv_p384)
        .text

// Size in bytes of a 64-bit word

#define N 8

// Pointer-offset pairs for temporaries on stack
// The u and v variables are 6 words each as expected, but the f and g
// variables are 8 words each -- they need to have at least one extra
// word for a sign word, and to preserve alignment we "round up" to 8.
// In fact, we currently keep an extra word in u and v as well.

#define f 0(%rsp)
#define g (8*N)(%rsp)
#define u (16*N)(%rsp)
#define v (24*N)(%rsp)
#define tmp  (32*N)(%rsp)
#define tmp2  (33*N)(%rsp)
#define i  (34*N)(%rsp)
#define d  (35*N)(%rsp)

#define mat (36*N)(%rsp)

// Backup for the input pointer

#define res  (40*N)(%rsp)

// Total size to reserve on the stack

#define NSPACE 42*N

// Syntactic variants to make x86_att version simpler to generate

#define F 0
#define G (8*N)
#define U (16*N)
#define V (24*N)
#define MAT (36*N)

#define ff  (%rsp)
#define gg  (8*N)(%rsp)

// ---------------------------------------------------------------------------
// Core signed almost-Montgomery reduction macro from P[6..0] to P[5..0].
// ---------------------------------------------------------------------------

#define amontred(P)                                                     \
/* We only know the input is -2^444 < x < 2^444. To do traditional  */  \
/* unsigned Montgomery reduction, start by adding 2^61 * p_384.     */  \
        movq    $0xe000000000000000, %r8 ;                             \
        xorl    %eax, %eax ;                                           \
        addq    P, %r8 ;                                            \
        movq    $0x000000001fffffff, %r9 ;                             \
        leaq    -1(%rax), %rax ;                                       \
        adcq    N+P, %r9 ;                                          \
        movq    $0xdfffffffe0000000, %r10 ;                            \
        adcq    2*N+P, %r10 ;                                       \
        movq    3*N+P, %r11 ;                                       \
        adcq    %rax, %r11 ;                                           \
        movq    4*N+P, %r12 ;                                       \
        adcq    %rax, %r12 ;                                           \
        movq    5*N+P, %r13 ;                                       \
        adcq    %rax, %r13 ;                                           \
        movq    $0x1fffffffffffffff, %r14 ;                            \
        adcq    6*N+P, %r14 ;                                       \
/* Correction multiplier is %rbx = w = [d0 + (d0<<32)] mod 2^64 */   \
        movq    %r8, %rbx ;                                            \
        shlq    $32, %rbx ;                                            \
        addq    %r8, %rbx ;                                            \
/* Construct [%rbp;%rdx;%rax;-] = (2^384 - p_384) * w */               \
/* We know lowest word will cancel so can re-use %r8 as a temp */    \
        xorl    %ebp, %ebp ;                                           \
        movq    $0xffffffff00000001, %rax ;                            \
        mulq    %rbx;                                                \
        movq    %rdx, %r8 ;                                            \
        movq    $0x00000000ffffffff, %rax ;                            \
        mulq    %rbx;                                                \
        addq    %r8, %rax ;                                            \
        adcq    %rbx, %rdx ;                                           \
        adcl    %ebp, %ebp ;                                           \
/*  Now subtract that and add 2^384 * w, catching carry in %rax  */  \
        subq    %rax, %r9 ;                                            \
        sbbq    %rdx, %r10 ;                                           \
        sbbq    %rbp, %r11 ;                                           \
        sbbq    $0, %r12 ;                                             \
        sbbq    $0, %r13 ;                                             \
        sbbq    $0, %r14 ;                                             \
        sbbq    %rax, %rax ;                                           \
        addq    %rbx, %r14 ;                                           \
        adcq    $0, %rax ;                                             \
/* Now if top is nonzero we subtract p_384 (almost-Montgomery) */   \
        negq    %rax;                                                \
        movq    $0x00000000ffffffff, %rbx ;                            \
        andq    %rax, %rbx ;                                           \
        movq    $0xffffffff00000000, %rcx ;                            \
        andq    %rax, %rcx ;                                           \
        movq    $0xfffffffffffffffe, %rdx ;                            \
        andq    %rax, %rdx ;                                           \
        subq    %rbx, %r9 ;                                            \
        movq    %r9, P ;                                            \
        sbbq    %rcx, %r10 ;                                           \
        movq    %r10, N+P ;                                         \
        sbbq    %rdx, %r11 ;                                           \
        movq    %r11, 2*N+P ;                                       \
        sbbq    %rax, %r12 ;                                           \
        movq    %r12, 3*N+P ;                                       \
        sbbq    %rax, %r13 ;                                           \
        movq    %r13, 4*N+P ;                                       \
        sbbq    %rax, %r14 ;                                           \
        movq    %r14, 5*N+P

// Very similar to a subroutine call to the s2n-bignum word_divstep59.
// But different in register usage and returning the final matrix as
//
// [ %r8   %r10]
// [ %r12  %r14]
//
// and also returning the matrix still negated (which doesn't matter)

#define divstep59(din,fin,gin)                                          \
        movq    din, %rsi ;                                               \
        movq    fin, %rdx ;                                               \
        movq    gin, %rcx ;                                               \
        movq    %rdx, %rbx ;                                               \
        andq    $0xfffff, %rbx ;                                           \
        movabsq $0xfffffe0000000000, %rax ;                                \
        orq     %rax, %rbx ;                                               \
        andq    $0xfffff, %rcx ;                                           \
        movabsq $0xc000000000000000, %rax ;                                \
        orq     %rax, %rcx ;                                               \
        movq    $0xfffffffffffffffe, %rax ;                                \
        xorl    %ebp, %ebp ;                                               \
        movl    $0x2, %edx ;                                               \
        movq    %rbx, %rdi ;                                               \
        movq    %rax, %r8 ;                                                \
        testq   %rsi, %rsi ;                                               \
        cmovs   %rbp, %r8 ;                                                \
        testq   $0x1, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        sarq    $1, %rcx ;                                                 \
        movl    $0x100000, %eax ;                                          \
        leaq    (%rbx,%rax), %rdx ;                                         \
        leaq    (%rcx,%rax), %rdi ;                                         \
        shlq    $0x16, %rdx ;                                              \
        shlq    $0x16, %rdi ;                                              \
        sarq    $0x2b, %rdx ;                                              \
        sarq    $0x2b, %rdi ;                                              \
        movabsq $0x20000100000, %rax ;                                     \
        leaq    (%rbx,%rax), %rbx ;                                         \
        leaq    (%rcx,%rax), %rcx ;                                         \
        sarq    $0x2a, %rbx ;                                              \
        sarq    $0x2a, %rcx ;                                              \
        movq    %rdx, MAT(%rsp) ;                                         \
        movq    %rbx, MAT+0x8(%rsp) ;                                     \
        movq    %rdi, MAT+0x10(%rsp) ;                                    \
        movq    %rcx, MAT+0x18(%rsp) ;                                    \
        movq    fin, %r12 ;                                               \
        imulq   %r12, %rdi ;                                               \
        imulq   %rdx, %r12 ;                                               \
        movq    gin, %r13 ;                                               \
        imulq   %r13, %rbx ;                                               \
        imulq   %rcx, %r13 ;                                               \
        addq    %rbx, %r12 ;                                               \
        addq    %rdi, %r13 ;                                               \
        sarq    $0x14, %r12 ;                                              \
        sarq    $0x14, %r13 ;                                              \
        movq    %r12, %rbx ;                                               \
        andq    $0xfffff, %rbx ;                                           \
        movabsq $0xfffffe0000000000, %rax ;                                \
        orq     %rax, %rbx ;                                               \
        movq    %r13, %rcx ;                                               \
        andq    $0xfffff, %rcx ;                                           \
        movabsq $0xc000000000000000, %rax ;                                \
        orq     %rax, %rcx ;                                               \
        movq    $0xfffffffffffffffe, %rax ;                                \
        movl    $0x2, %edx ;                                               \
        movq    %rbx, %rdi ;                                               \
        movq    %rax, %r8 ;                                                \
        testq   %rsi, %rsi ;                                               \
        cmovs   %rbp, %r8 ;                                                \
        testq   $0x1, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        sarq    $1, %rcx ;                                                 \
        movl    $0x100000, %eax ;                                          \
        leaq    (%rbx,%rax), %r8 ;                                          \
        leaq    (%rcx,%rax), %r10 ;                                         \
        shlq    $0x16, %r8 ;                                               \
        shlq    $0x16, %r10 ;                                              \
        sarq    $0x2b, %r8 ;                                               \
        sarq    $0x2b, %r10 ;                                              \
        movabsq $0x20000100000, %rax ;                                     \
        leaq    (%rbx,%rax), %r15 ;                                         \
        leaq    (%rcx,%rax), %r11 ;                                         \
        sarq    $0x2a, %r15 ;                                              \
        sarq    $0x2a, %r11 ;                                              \
        movq    %r13, %rbx ;                                               \
        movq    %r12, %rcx ;                                               \
        imulq   %r8, %r12 ;                                                \
        imulq   %r15, %rbx ;                                               \
        addq    %rbx, %r12 ;                                               \
        imulq   %r11, %r13 ;                                               \
        imulq   %r10, %rcx ;                                               \
        addq    %rcx, %r13 ;                                               \
        sarq    $0x14, %r12 ;                                              \
        sarq    $0x14, %r13 ;                                              \
        movq    %r12, %rbx ;                                               \
        andq    $0xfffff, %rbx ;                                           \
        movabsq $0xfffffe0000000000, %rax ;                                \
        orq     %rax, %rbx ;                                               \
        movq    %r13, %rcx ;                                               \
        andq    $0xfffff, %rcx ;                                           \
        movabsq $0xc000000000000000, %rax ;                                \
        orq     %rax, %rcx ;                                               \
        movq    MAT(%rsp), %rax ;                                         \
        imulq   %r8, %rax ;                                                \
        movq    MAT+0x10(%rsp), %rdx ;                                    \
        imulq   %r15, %rdx ;                                               \
        imulq   MAT+0x8(%rsp), %r8 ;                                      \
        imulq   MAT+0x18(%rsp), %r15 ;                                    \
        addq    %r8, %r15 ;                                                \
        leaq    (%rax,%rdx), %r9 ;                                          \
        movq    MAT(%rsp), %rax ;                                         \
        imulq   %r10, %rax ;                                               \
        movq    MAT+0x10(%rsp), %rdx ;                                    \
        imulq   %r11, %rdx ;                                               \
        imulq   MAT+0x8(%rsp), %r10 ;                                     \
        imulq   MAT+0x18(%rsp), %r11 ;                                    \
        addq    %r10, %r11 ;                                               \
        leaq    (%rax,%rdx), %r13 ;                                         \
        movq    $0xfffffffffffffffe, %rax ;                                \
        movl    $0x2, %edx ;                                               \
        movq    %rbx, %rdi ;                                               \
        movq    %rax, %r8 ;                                                \
        testq   %rsi, %rsi ;                                               \
        cmovs   %rbp, %r8 ;                                                \
        testq   $0x1, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        cmovs   %rbp, %r8 ;                                                \
        movq    %rbx, %rdi ;                                               \
        testq   %rdx, %rcx ;                                               \
        cmoveq  %rbp, %r8 ;                                                \
        cmoveq  %rbp, %rdi ;                                               \
        sarq    $1, %rcx ;                                                 \
        xorq    %r8, %rdi ;                                                \
        xorq    %r8, %rsi ;                                                \
        btq     $0x3f, %r8 ;                                               \
        cmovbq  %rcx, %rbx ;                                               \
        movq    %rax, %r8 ;                                                \
        subq    %rax, %rsi ;                                               \
        leaq    (%rcx,%rdi), %rcx ;                                         \
        sarq    $1, %rcx ;                                                 \
        movl    $0x100000, %eax ;                                          \
        leaq    (%rbx,%rax), %r8 ;                                          \
        leaq    (%rcx,%rax), %r12 ;                                         \
        shlq    $0x15, %r8 ;                                               \
        shlq    $0x15, %r12 ;                                              \
        sarq    $0x2b, %r8 ;                                               \
        sarq    $0x2b, %r12 ;                                              \
        movabsq $0x20000100000, %rax ;                                     \
        leaq    (%rbx,%rax), %r10 ;                                         \
        leaq    (%rcx,%rax), %r14 ;                                         \
        sarq    $0x2b, %r10 ;                                              \
        sarq    $0x2b, %r14 ;                                              \
        movq    %r9, %rax ;                                                \
        imulq   %r8, %rax ;                                                \
        movq    %r13, %rdx ;                                               \
        imulq   %r10, %rdx ;                                               \
        imulq   %r15, %r8 ;                                                \
        imulq   %r11, %r10 ;                                               \
        addq    %r8, %r10 ;                                                \
        leaq    (%rax,%rdx), %r8 ;                                          \
        movq    %r9, %rax ;                                                \
        imulq   %r12, %rax ;                                               \
        movq    %r13, %rdx ;                                               \
        imulq   %r14, %rdx ;                                               \
        imulq   %r15, %r12 ;                                               \
        imulq   %r11, %r14 ;                                               \
        addq    %r12, %r14 ;                                               \
        leaq    (%rax,%rdx), %r12

S2N_BN_SYMBOL(bignum_inv_p384):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Save registers and make room for temporaries

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

// Save the return pointer for the end so we can overwrite %rdi later

        movq    %rdi, res

// Copy the constant p_384 into f including the 7th zero digit

        movl    $0xffffffff, %eax
        movq    %rax, F(%rsp)
        movq    %rax, %rbx
        notq    %rbx
        movq    %rbx, F+N(%rsp)
        xorl    %ebp, %ebp
        leaq    -2(%rbp), %rcx
        movq    %rcx, F+2*N(%rsp)
        leaq    -1(%rbp), %rdx
        movq    %rdx, F+3*N(%rsp)
        movq    %rdx, F+4*N(%rsp)
        movq    %rdx, F+5*N(%rsp)
        movq    %rbp, F+6*N(%rsp)

// Copy input but to g, reduced mod p_384 so that g <= f as assumed
// in the divstep bound proof.

        movq    (%rsi), %r8
        subq    %rax, %r8
        movq    N(%rsi), %r9
        sbbq    %rbx, %r9
        movq    2*N(%rsi), %r10
        sbbq    %rcx, %r10
        movq    3*N(%rsi), %r11
        sbbq    %rdx, %r11
        movq    4*N(%rsi), %r12
        sbbq    %rdx, %r12
        movq    5*N(%rsi), %r13
        sbbq    %rdx, %r13

        cmovcq  (%rsi), %r8
        cmovcq  N(%rsi), %r9
        cmovcq  2*N(%rsi), %r10
        cmovcq  3*N(%rsi), %r11
        cmovcq  4*N(%rsi), %r12
        cmovcq  5*N(%rsi), %r13

        movq    %r8, G(%rsp)
        movq    %r9, G+N(%rsp)
        movq    %r10, G+2*N(%rsp)
        movq    %r11, G+3*N(%rsp)
        movq    %r12, G+4*N(%rsp)
        movq    %r13, G+5*N(%rsp)
        movq    %rbp, G+6*N(%rsp)

// Also maintain reduced < 2^384 vector [u,v] such that
// [f,g] == x * 2^{5*i-75} * [u,v] (mod p_384)
// starting with [p_384,x] == x * 2^{5*0-75} * [0,2^75] (mod p_384)
// The weird-looking 5*i modifications come in because we are doing
// 64-bit word-sized Montgomery reductions at each stage, which is
// 5 bits more than the 59-bit requirement to keep things stable.

        xorl    %eax, %eax
        movq    %rax, U(%rsp)
        movq    %rax, U+N(%rsp)
        movq    %rax, U+2*N(%rsp)
        movq    %rax, U+3*N(%rsp)
        movq    %rax, U+4*N(%rsp)
        movq    %rax, U+5*N(%rsp)

        movl    $2048, %ecx
        movq    %rax, V(%rsp)
        movq    %rcx, V+N(%rsp)
        movq    %rax, V+2*N(%rsp)
        movq    %rax, V+3*N(%rsp)
        movq    %rax, V+4*N(%rsp)
        movq    %rax, V+5*N(%rsp)

// Start of main loop. We jump into the middle so that the divstep
// portion is common to the special fifteenth iteration after a uniform
// first 14.

        movq    $15, i
        movq    $1, d
        jmp     Lbignum_inv_p384_midloop

Lbignum_inv_p384_loop:

// Separate out the matrix into sign-magnitude pairs

        movq    %r8, %r9
        sarq    $63, %r9
        xorq    %r9, %r8
        subq    %r9, %r8

        movq    %r10, %r11
        sarq    $63, %r11
        xorq    %r11, %r10
        subq    %r11, %r10

        movq    %r12, %r13
        sarq    $63, %r13
        xorq    %r13, %r12
        subq    %r13, %r12

        movq    %r14, %r15
        sarq    $63, %r15
        xorq    %r15, %r14
        subq    %r15, %r14

// Adjust the initial values to allow for complement instead of negation
// This initial offset is the same for [f,g] and [u,v] compositions.
// Save it in temporary storage for the [u,v] part and do [f,g] first.

        movq    %r8, %rax
        andq    %r9, %rax
        movq    %r10, %rdi
        andq    %r11, %rdi
        addq    %rax, %rdi
        movq    %rdi, tmp

        movq    %r12, %rax
        andq    %r13, %rax
        movq    %r14, %rsi
        andq    %r15, %rsi
        addq    %rax, %rsi
        movq    %rsi, tmp2

// Now the computation of the updated f and g values. This maintains a
// 2-word carry between stages so we can conveniently insert the shift
// right by 59 before storing back, and not overwrite digits we need
// again of the old f and g values.
//
// Digit 0 of [f,g]

        xorl    %ebx, %ebx
        movq    F(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        movq    G(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rdi
        adcq    %rdx, %rbx

        xorl    %ebp, %ebp
        movq    F(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    G(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rbp

// Digit 1 of [f,g]

        xorl    %ecx, %ecx
        movq    F+N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    G+N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        shrdq   $59, %rbx, %rdi
        movq    %rdi, F(%rsp)

        xorl    %edi, %edi
        movq    F+N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rbp
        adcq    %rdx, %rdi
        movq    G+N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rdi
        shrdq   $59, %rbp, %rsi
        movq    %rsi, G(%rsp)

// Digit 2 of [f,g]

        xorl    %esi, %esi
        movq    F+2*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rsi
        movq    G+2*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rdx, %rsi
        shrdq   $59, %rcx, %rbx
        movq    %rbx, F+N(%rsp)

        xorl    %ebx, %ebx
        movq    F+2*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        movq    G+2*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        shrdq   $59, %rdi, %rbp
        movq    %rbp, G+N(%rsp)

// Digit 3 of [f,g]

        xorl    %ebp, %ebp
        movq    F+3*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    G+3*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        shrdq   $59, %rsi, %rcx
        movq    %rcx, F+2*N(%rsp)

        xorl    %ecx, %ecx
        movq    F+3*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    G+3*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        shrdq   $59, %rbx, %rdi
        movq    %rdi, G+2*N(%rsp)

// Digit 4 of [f,g]

        xorl    %edi, %edi
        movq    F+4*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbp
        adcq    %rdx, %rdi
        movq    G+4*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbp
        adcq    %rdx, %rdi
        shrdq   $59, %rbp, %rsi
        movq    %rsi, F+3*N(%rsp)

        xorl    %esi, %esi
        movq    F+4*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rcx
        adcq    %rdx, %rsi
        movq    G+4*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rcx
        adcq    %rdx, %rsi
        shrdq   $59, %rcx, %rbx
        movq    %rbx, G+3*N(%rsp)

// Digits 5 and 6 of [f,g]

        movq    F+5*N(%rsp), %rax
        xorq    %r9, %rax
        movq    F+6*N(%rsp), %rbx
        xorq    %r9, %rbx
        andq    %r8, %rbx
        negq    %rbx
        mulq    %r8
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        movq    G+5*N(%rsp), %rax
        xorq    %r11, %rax
        movq    G+6*N(%rsp), %rdx
        xorq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %rbx
        mulq    %r10
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        shrdq   $59, %rdi, %rbp
        movq    %rbp, F+4*N(%rsp)
        shrdq   $59, %rbx, %rdi
        sarq    $59, %rbx

        movq    F+5*N(%rsp), %rax
        movq    %rdi, F+5*N(%rsp)

        movq    F+6*N(%rsp), %rdi
        movq    %rbx, F+6*N(%rsp)

        xorq    %r13, %rax
        xorq    %r13, %rdi
        andq    %r12, %rdi
        negq    %rdi
        mulq    %r12
        addq    %rax, %rsi
        adcq    %rdx, %rdi
        movq    G+5*N(%rsp), %rax
        xorq    %r15, %rax
        movq    G+6*N(%rsp), %rdx
        xorq    %r15, %rdx
        andq    %r14, %rdx
        subq    %rdx, %rdi
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rdi
        shrdq   $59, %rsi, %rcx
        movq    %rcx, G+4*N(%rsp)
        shrdq   $59, %rdi, %rsi
        movq    %rsi, G+5*N(%rsp)
        sarq    $59, %rdi
        movq    %rdi, G+6*N(%rsp)

// Get the initial carries back from storage and do the [u,v] accumulation

        movq    tmp, %rbx
        movq    tmp2, %rbp

// Digit 0 of [u,v]

        xorl    %ecx, %ecx
        movq    U(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    V(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx

        xorl    %esi, %esi
        movq    U(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rbx, U(%rsp)
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    V(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    %rbp, V(%rsp)

// Digit 1 of [u,v]

        xorl    %ebx, %ebx
        movq    U+N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    V+N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rdx, %rbx

        xorl    %ebp, %ebp
        movq    U+N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rcx, U+N(%rsp)
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    V+N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    %rsi, V+N(%rsp)

// Digit 2 of [u,v]

        xorl    %ecx, %ecx
        movq    U+2*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    V+2*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx

        xorl    %esi, %esi
        movq    U+2*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rbx, U+2*N(%rsp)
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    V+2*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    %rbp, V+2*N(%rsp)

// Digit 3 of [u,v]

        xorl    %ebx, %ebx
        movq    U+3*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    V+3*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rdx, %rbx

        xorl    %ebp, %ebp
        movq    U+3*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rcx, U+3*N(%rsp)
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    V+3*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    %rsi, V+3*N(%rsp)

// Digit 4 of [u,v]

        xorl    %ecx, %ecx
        movq    U+4*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    V+4*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx

        xorl    %esi, %esi
        movq    U+4*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rbx, U+4*N(%rsp)
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    V+4*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    %rbp, V+4*N(%rsp)

// Digits 5 and 6 of u (top is unsigned)

        movq    U+5*N(%rsp), %rax
        xorq    %r9, %rax
        movq    %r9, %rbx
        andq    %r8, %rbx
        negq    %rbx
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    V+5*N(%rsp), %rax
        xorq    %r11, %rax
        movq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %rbx
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rbx, %rdx

// Preload for last use of old u digit 3

        movq    U+5*N(%rsp), %rax
        movq    %rcx, U+5*N(%rsp)
        movq    %rdx, U+6*N(%rsp)

// Digits 5 and 6 of v (top is unsigned)

        xorq    %r13, %rax
        movq    %r13, %rcx
        andq    %r12, %rcx
        negq    %rcx
        mulq    %r12
        addq    %rax, %rsi
        adcq    %rdx, %rcx
        movq    V+5*N(%rsp), %rax
        xorq    %r15, %rax
        movq    %r15, %rdx
        andq    %r14, %rdx
        subq    %rdx, %rcx
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rcx, %rdx
        movq    %rsi, V+5*N(%rsp)
        movq    %rdx, V+6*N(%rsp)

// Montgomery reduction of u

        amontred(u)

// Montgomery reduction of v

        amontred(v)

Lbignum_inv_p384_midloop:

        divstep59(d,ff,gg)
        movq    %rsi, d

// Next iteration

        decq    i
        jnz     Lbignum_inv_p384_loop

// The 15th and last iteration does not need anything except the
// u value and the sign of f; the latter can be obtained from the
// lowest word of f. So it's done differently from the main loop.
// Find the sign of the new f. For this we just need one digit
// since we know (for in-scope cases) that f is either +1 or -1.
// We don't explicitly shift right by 59 either, but looking at
// bit 63 (or any bit >= 60) of the unshifted result is enough
// to distinguish -1 from +1; this is then made into a mask.

        movq    F(%rsp), %rax
        movq    G(%rsp), %rcx
        imulq   %r8, %rax
        imulq   %r10, %rcx
        addq    %rcx, %rax
        sarq    $63, %rax

// Now separate out the matrix into sign-magnitude pairs
// and adjust each one based on the sign of f.
//
// Note that at this point we expect |f|=1 and we got its
// sign above, so then since [f,0] == x * [u,v] (mod p_384)
// we want to flip the sign of u according to that of f.

        movq    %r8, %r9
        sarq    $63, %r9
        xorq    %r9, %r8
        subq    %r9, %r8
        xorq    %rax, %r9

        movq    %r10, %r11
        sarq    $63, %r11
        xorq    %r11, %r10
        subq    %r11, %r10
        xorq    %rax, %r11

        movq    %r12, %r13
        sarq    $63, %r13
        xorq    %r13, %r12
        subq    %r13, %r12
        xorq    %rax, %r13

        movq    %r14, %r15
        sarq    $63, %r15
        xorq    %r15, %r14
        subq    %r15, %r14
        xorq    %rax, %r15

// Adjust the initial value to allow for complement instead of negation

        movq    %r8, %rax
        andq    %r9, %rax
        movq    %r10, %r12
        andq    %r11, %r12
        addq    %rax, %r12

// Digit 0 of [u]

        xorl    %r13d, %r13d
        movq    U(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    V(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r12
        movq    %r12, U(%rsp)
        adcq    %rdx, %r13

// Digit 1 of [u]

        xorl    %r14d, %r14d
        movq    U+N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r13
        adcq    %rdx, %r14
        movq    V+N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r13
        movq    %r13, U+N(%rsp)
        adcq    %rdx, %r14

// Digit 2 of [u]

        xorl    %r15d, %r15d
        movq    U+2*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r14
        adcq    %rdx, %r15
        movq    V+2*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r14
        movq    %r14, U+2*N(%rsp)
        adcq    %rdx, %r15

// Digit 3 of [u]

        xorl    %r14d, %r14d
        movq    U+3*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r15
        adcq    %rdx, %r14
        movq    V+3*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r15
        movq    %r15, U+3*N(%rsp)
        adcq    %rdx, %r14

// Digit 4 of [u]

        xorl    %r15d, %r15d
        movq    U+4*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r14
        adcq    %rdx, %r15
        movq    V+4*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r14
        movq    %r14, U+4*N(%rsp)
        adcq    %rdx, %r15

// Digits 5 and 6 of u (top is unsigned)

        movq    U+5*N(%rsp), %rax
        xorq    %r9, %rax
        andq    %r8, %r9
        negq    %r9
        mulq    %r8
        addq    %rax, %r15
        adcq    %rdx, %r9
        movq    V+5*N(%rsp), %rax
        xorq    %r11, %rax
        movq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %r9
        mulq    %r10
        addq    %rax, %r15
        movq    %r15, U+5*N(%rsp)
        adcq    %rdx, %r9
        movq    %r9, U+6*N(%rsp)

// Montgomery reduce u

        amontred(u)

// Perform final strict reduction mod p_384 and copy to output

        movl    $0xffffffff, %eax
        movq    %rax, %rbx
        notq    %rbx
        xorl    %ebp, %ebp
        leaq    -2(%rbp), %rcx
        leaq    -1(%rbp), %rdx

        movq    U(%rsp), %r8
        subq    %rax, %r8
        movq    U+N(%rsp), %r9
        sbbq    %rbx, %r9
        movq    U+2*N(%rsp), %r10
        sbbq    %rcx, %r10
        movq    U+3*N(%rsp), %r11
        sbbq    %rdx, %r11
        movq    U+4*N(%rsp), %r12
        sbbq    %rdx, %r12
        movq    U+5*N(%rsp), %r13
        sbbq    %rdx, %r13

        cmovcq  U(%rsp), %r8
        cmovcq  U+N(%rsp), %r9
        cmovcq  U+2*N(%rsp), %r10
        cmovcq  U+3*N(%rsp), %r11
        cmovcq  U+4*N(%rsp), %r12
        cmovcq  U+5*N(%rsp), %r13

        movq    res, %rdi
        movq    %r8, (%rdi)
        movq    %r9, N(%rdi)
        movq    %r10, 2*N(%rdi)
        movq    %r11, 3*N(%rdi)
        movq    %r12, 4*N(%rdi)
        movq    %r13, 5*N(%rdi)

// Restore stack and registers

        CFI_INC_RSP(NSPACE)

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

S2N_BN_SIZE_DIRECTIVE(bignum_inv_p384)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
