// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Modular inverse modulo p_521 =  2^521 - 1
// Input x[9]; output z[9]
//
// extern void bignum_inv_p521(uint64_t z[static 9],const uint64_t x[static 9]);
//
// Assuming the 9-digit input x is coprime to p_521, i.e. is not divisible
// by it, returns z < p_521 such that x * z == 1 (mod p_521). Note that
// x does not need to be reduced modulo p_521, but the output always is.
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_inv_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_inv_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_inv_p521)
        .text

// Size in bytes of a 64-bit word

#define N 8

// Pointer-offset pairs for temporaries on stack

#define f 0(%rsp)
#define g (9*N)(%rsp)
#define u (18*N)(%rsp)
#define v (27*N)(%rsp)
#define tmp  (36*N)(%rsp)
#define tmp2  (37*N)(%rsp)
#define i  (38*N)(%rsp)
#define d  (39*N)(%rsp)

#define mat (40*N)(%rsp)

// Backup for the input pointer

#define res  (44*N)(%rsp)

// Total size to reserve on the stack

#define NSPACE 45*N

// Syntactic variants to make x86_att version simpler to generate

#define F 0
#define G (9*N)
#define U (18*N)
#define V (27*N)
#define MAT (40*N)

#define ff  (%rsp)
#define gg  (9*N)(%rsp)

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

S2N_BN_SYMBOL(bignum_inv_p521):
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

// Copy the prime p_521 = 2^521 - 1 into the f variable

        xorl    %eax, %eax
        notq    %rax
        movq    %rax, F(%rsp)
        movq    %rax, F+8(%rsp)
        movq    %rax, F+16(%rsp)
        movq    %rax, F+24(%rsp)
        movq    %rax, F+32(%rsp)
        movq    %rax, F+40(%rsp)
        movq    %rax, F+48(%rsp)
        movq    %rax, F+56(%rsp)
        movl    $0x1FF, %eax
        movq    %rax, F+64(%rsp)

// Copy the input into the g variable, but reduce it strictly mod p_521
// so that g <= f as assumed in the bound proof. This code fragment is
// very similar to bignum_mod_p521_9.

        movq    64(%rsi), %r8
        movl    $0x1FF, %ebx
        andq    %r8, %rbx
        shrq    $9, %r8

        stc
        adcq    (%rsi), %r8
        movq    8(%rsi), %r9
        adcq    $0, %r9
        movq    16(%rsi), %r10
        adcq    $0, %r10
        movq    24(%rsi), %r11
        adcq    $0, %r11
        movq    32(%rsi), %r12
        adcq    $0, %r12
        movq    40(%rsi), %r13
        adcq    $0, %r13
        movq    48(%rsi), %r14
        adcq    $0, %r14
        movq    56(%rsi), %r15
        adcq    $0, %r15
        adcq    $0, %rbx

        cmpq    $512, %rbx

        sbbq    $0, %r8
        movq    %r8, G(%rsp)
        sbbq    $0, %r9
        movq    %r9, G+8(%rsp)
        sbbq    $0, %r10
        movq    %r10, G+16(%rsp)
        sbbq    $0, %r11
        movq    %r11, G+24(%rsp)
        sbbq    $0, %r12
        movq    %r12, G+32(%rsp)
        sbbq    $0, %r13
        movq    %r13, G+40(%rsp)
        sbbq    $0, %r14
        movq    %r14, G+48(%rsp)
        sbbq    $0, %r15
        movq    %r15, G+56(%rsp)
        sbbq    $0, %rbx
        andq    $0x1FF, %rbx
        movq    %rbx, G+64(%rsp)

// Also maintain weakly reduced < 2*p_521 vector [u,v] such that
// [f,g] == x * 2^{1239-59*i} * [u,v] (mod p_521)
// starting with [p_521,x] == x * 2^{1239-59*0} * [0,2^-1239] (mod p_521)
// Note that because (2^{a+521} == 2^a) (mod p_521) we simply have
// (2^-1239 == 2^324) (mod p_521) so the constant initializer is simple.
//
// Based on the standard divstep bound, for inputs <= 2^b we need at least
// n >= (9437 * b + 1) / 4096. Since b is 521, that means 1201 iterations.
// Since we package divstep in multiples of 59 bits, we do 21 blocks of 59
// making *1239* total. (With a bit more effort we could avoid the full 59
// divsteps and use a shorter tail computation, but we keep it simple.)
// Hence, after the 21st iteration we have [f,g] == x * [u,v] and since
// |f| = 1 we get the modular inverse from u by flipping its sign with f.

        xorl    %eax, %eax
        movq    %rax, U(%rsp)
        movq    %rax, U+8(%rsp)
        movq    %rax, U+16(%rsp)
        movq    %rax, U+24(%rsp)
        movq    %rax, U+32(%rsp)
        movq    %rax, U+40(%rsp)
        movq    %rax, U+48(%rsp)
        movq    %rax, U+56(%rsp)
        movq    %rax, U+64(%rsp)

        movl    $16, %ebx
        movq    %rax, V(%rsp)
        movq    %rax, V+8(%rsp)
        movq    %rax, V+16(%rsp)
        movq    %rax, V+24(%rsp)
        movq    %rax, V+32(%rsp)
        movq    %rbx, V+40(%rsp)
        movq    %rax, V+48(%rsp)
        movq    %rax, V+56(%rsp)
        movq    %rax, V+64(%rsp)

// Start of main loop. We jump into the middle so that the divstep
// portion is common to the special 21st iteration after a uniform
// first 20.

        movq    $21, i
        movq    $1, d
        jmp     Lbignum_inv_p521_midloop

Lbignum_inv_p521_loop:

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

// Digit 5 of [f,g]

        xorl    %ebx, %ebx
        movq    F+5*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        movq    G+5*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        shrdq   $59, %rdi, %rbp
        movq    %rbp, F+4*N(%rsp)

        xorl    %ebp, %ebp
        movq    F+5*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    G+5*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        shrdq   $59, %rsi, %rcx
        movq    %rcx, G+4*N(%rsp)

// Digit 6 of [f,g]

        xorl    %ecx, %ecx
        movq    F+6*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    G+6*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        shrdq   $59, %rbx, %rdi
        movq    %rdi, F+5*N(%rsp)

        xorl    %edi, %edi
        movq    F+6*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rbp
        adcq    %rdx, %rdi
        movq    G+6*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rdi
        shrdq   $59, %rbp, %rsi
        movq    %rsi, G+5*N(%rsp)

// Digit 7 of [f,g]

        xorl    %esi, %esi
        movq    F+7*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rsi
        movq    G+7*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rdx, %rsi
        shrdq   $59, %rcx, %rbx
        movq    %rbx, F+6*N(%rsp)

        xorl    %ebx, %ebx
        movq    F+7*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        movq    G+7*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        shrdq   $59, %rdi, %rbp
        movq    %rbp, G+6*N(%rsp)

// Digits 8 and 9 of [f,g]

        movq    F+8*N(%rsp), %rax
        xorq    %r9, %rax
        movq    %rax, %rbp
        sarq    $63, %rbp
        andq    %r8, %rbp
        negq    %rbp
        mulq    %r8
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    G+8*N(%rsp), %rax
        xorq    %r11, %rax
        movq    %rax, %rdx
        sarq    $63, %rdx
        andq    %r10, %rdx
        subq    %rdx, %rbp
        mulq    %r10
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        shrdq   $59, %rsi, %rcx
        movq    %rcx, F+7*N(%rsp)
        shrdq   $59, %rbp, %rsi

        movq    F+8*N(%rsp), %rax
        movq    %rsi, F+8*N(%rsp)

        xorq    %r13, %rax
        movq    %rax, %rsi
        sarq    $63, %rsi
        andq    %r12, %rsi
        negq    %rsi
        mulq    %r12
        addq    %rax, %rbx
        adcq    %rdx, %rsi
        movq    G+8*N(%rsp), %rax
        xorq    %r15, %rax
        movq    %rax, %rdx
        sarq    $63, %rdx
        andq    %r14, %rdx
        subq    %rdx, %rsi
        mulq    %r14
        addq    %rax, %rbx
        adcq    %rdx, %rsi
        shrdq   $59, %rbx, %rdi
        movq    %rdi, G+7*N(%rsp)
        shrdq   $59, %rsi, %rbx
        movq    %rbx, G+8*N(%rsp)

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

// Digit 5 of [u,v]

        xorl    %ebx, %ebx
        movq    U+5*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    V+5*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rdx, %rbx

        xorl    %ebp, %ebp
        movq    U+5*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rcx, U+5*N(%rsp)
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    V+5*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    %rsi, V+5*N(%rsp)

// Digit 6 of [u,v]

        xorl    %ecx, %ecx
        movq    U+6*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    V+6*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx

        xorl    %esi, %esi
        movq    U+6*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rbx, U+6*N(%rsp)
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    V+6*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    %rbp, V+6*N(%rsp)

// Digit 7 of [u,v]

        xorl    %ebx, %ebx
        movq    U+7*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    V+7*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rdx, %rbx

        xorl    %ebp, %ebp
        movq    U+7*N(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rcx, U+7*N(%rsp)
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    V+7*N(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    %rsi, V+7*N(%rsp)

// Digits 8 and 9 of u (top is unsigned)

        movq    U+8*N(%rsp), %rax
        xorq    %r9, %rax
        movq    %r9, %rcx
        andq    %r8, %rcx
        negq    %rcx
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    V+8*N(%rsp), %rax
        xorq    %r11, %rax
        movq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %rcx
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rcx, %rdx

// Modular reduction of u

        movq    %rdx, %rax
        shldq   $55, %rbx, %rdx
        sarq    $63, %rax
        addq    %rax, %rdx
        movq    %rdx, %rax
        shlq    $9, %rdx
        subq    %rdx, %rbx
        movq    %rax, %rdx
        sarq    $63, %rax
        movq    U(%rsp), %rcx
        addq    %rdx, %rcx
        movq    %rcx, U(%rsp)
        movq    U+N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+N(%rsp)
        movq    U+2*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+2*N(%rsp)
        movq    U+3*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+3*N(%rsp)
        movq    U+4*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+4*N(%rsp)
        movq    U+5*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+5*N(%rsp)
        movq    U+6*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+6*N(%rsp)
        movq    U+7*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+7*N(%rsp)
        adcq    %rax, %rbx

// Preload for last use of old u digit 8

        movq    U+8*N(%rsp), %rax
        movq    %rbx, U+8*N(%rsp)

// Digits 8 and 9 of v (top is unsigned)

        xorq    %r13, %rax
        movq    %r13, %rbx
        andq    %r12, %rbx
        negq    %rbx
        mulq    %r12
        addq    %rax, %rbp
        adcq    %rdx, %rbx
        movq    V+8*N(%rsp), %rax
        xorq    %r15, %rax
        movq    %r15, %rdx
        andq    %r14, %rdx
        subq    %rdx, %rbx
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rbx, %rdx

// Modular reduction of v

        movq    %rdx, %rax
        shldq   $55, %rbp, %rdx
        sarq    $63, %rax
        addq    %rax, %rdx
        movq    %rdx, %rax
        shlq    $9, %rdx
        subq    %rdx, %rbp
        movq    %rax, %rdx
        sarq    $63, %rax
        movq    V(%rsp), %rcx
        addq    %rdx, %rcx
        movq    %rcx, V(%rsp)
        movq    V+N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, V+N(%rsp)
        movq    V+2*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, V+2*N(%rsp)
        movq    V+3*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, V+3*N(%rsp)
        movq    V+4*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, V+4*N(%rsp)
        movq    V+5*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, V+5*N(%rsp)
        movq    V+6*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, V+6*N(%rsp)
        movq    V+7*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, V+7*N(%rsp)
        adcq    %rax, %rbp
        movq    %rbp, V+8*N(%rsp)

Lbignum_inv_p521_midloop:

        divstep59(d,ff,gg)
        movq    %rsi, d

// Next iteration

        decq    i
        jnz     Lbignum_inv_p521_loop

// The 21st and last iteration does not need anything except the
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
// sign above, so then since [f,0] == x * [u,v] (mod p_521)
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
        movq    %r10, %rbx
        andq    %r11, %rbx
        addq    %rax, %rbx

// Digit 0 of u

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
        movq    %rbx, U(%rsp)
        adcq    %rdx, %rcx

// Digit 1 of u

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
        movq    %rcx, U+N(%rsp)
        adcq    %rdx, %rbx

// Digit 2 of u

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
        movq    %rbx, U+2*N(%rsp)
        adcq    %rdx, %rcx

// Digit 3 of u

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
        movq    %rcx, U+3*N(%rsp)
        adcq    %rdx, %rbx

// Digit 4 of u

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
        movq    %rbx, U+4*N(%rsp)
        adcq    %rdx, %rcx

// Digit 5 of u

        xorl    %ebx, %ebx
        movq    U+5*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    V+5*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        movq    %rcx, U+5*N(%rsp)
        adcq    %rdx, %rbx

// Digit 6 of u

        xorl    %ecx, %ecx
        movq    U+6*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    V+6*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        movq    %rbx, U+6*N(%rsp)
        adcq    %rdx, %rcx

// Digit 7 of u

        xorl    %ebx, %ebx
        movq    U+7*N(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    V+7*N(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        movq    %rcx, U+7*N(%rsp)
        adcq    %rdx, %rbx

// Digits 8 and 9 of u (top is unsigned)

        movq    U+8*N(%rsp), %rax
        xorq    %r9, %rax
        movq    %r9, %rcx
        andq    %r8, %rcx
        negq    %rcx
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    V+8*N(%rsp), %rax
        xorq    %r11, %rax
        movq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %rcx
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rcx, %rdx

// Modular reduction of u

        movq    %rdx, %rax
        shldq   $55, %rbx, %rdx
        sarq    $63, %rax
        addq    %rax, %rdx
        movq    %rdx, %rax
        shlq    $9, %rdx
        subq    %rdx, %rbx
        movq    %rax, %rdx
        sarq    $63, %rax
        movq    U(%rsp), %rcx
        addq    %rdx, %rcx
        movq    %rcx, U(%rsp)
        movq    U+N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+N(%rsp)
        movq    U+2*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+2*N(%rsp)
        movq    U+3*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+3*N(%rsp)
        movq    U+4*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+4*N(%rsp)
        movq    U+5*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+5*N(%rsp)
        movq    U+6*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+6*N(%rsp)
        movq    U+7*N(%rsp), %rcx
        adcq    %rax, %rcx
        movq    %rcx, U+7*N(%rsp)
        adcq    %rax, %rbx
        movq    %rbx, U+8*N(%rsp)

// Further strict reduction ready for the output, which just means
// a conditional subtraction of p_521

        xorl    %eax, %eax
        notq    %rax
        movq    U(%rsp), %r8
        subq    %rax, %r8
        movq    U+N(%rsp), %r9
        sbbq    %rax, %r9
        movq    U+2*N(%rsp), %r10
        sbbq    %rax, %r10
        movq    U+3*N(%rsp), %r11
        sbbq    %rax, %r11
        movq    U+4*N(%rsp), %r12
        sbbq    %rax, %r12
        movq    U+5*N(%rsp), %r13
        sbbq    %rax, %r13
        movq    U+6*N(%rsp), %r14
        sbbq    %rax, %r14
        movq    U+7*N(%rsp), %r15
        sbbq    %rax, %r15
        movl    $0x1FF, %eax
        movq    U+8*N(%rsp), %rbp
        sbbq    %rax, %rbp

        cmovcq  U(%rsp), %r8
        cmovcq  U+N(%rsp), %r9
        cmovcq  U+2*N(%rsp), %r10
        cmovcq  U+3*N(%rsp), %r11
        cmovcq  U+4*N(%rsp), %r12
        cmovcq  U+5*N(%rsp), %r13
        cmovcq  U+6*N(%rsp), %r14
        cmovcq  U+7*N(%rsp), %r15
        cmovcq  U+8*N(%rsp), %rbp

// Store it back to the final output

        movq    res, %rdi
        movq    %r8, (%rdi)
        movq    %r9, N(%rdi)
        movq    %r10, 2*N(%rdi)
        movq    %r11, 3*N(%rdi)
        movq    %r12, 4*N(%rdi)
        movq    %r13, 5*N(%rdi)
        movq    %r14, 6*N(%rdi)
        movq    %r15, 7*N(%rdi)
        movq    %rbp, 8*N(%rdi)

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

S2N_BN_SIZE_DIRECTIVE(bignum_inv_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
