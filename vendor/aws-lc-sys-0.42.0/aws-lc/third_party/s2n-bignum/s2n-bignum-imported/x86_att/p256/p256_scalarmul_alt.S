// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Scalar multiplication for P-256
// Input scalar[4], point[8]; output res[8]
//
// extern void p256_scalarmul_alt
//   (uint64_t res[static 8],const uint64_t scalar[static 4],
//     const uint64_t point[static 8]);
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve P-256, returns the point (X,Y) = n * P. The input and output
// are affine points, and in the case of the point at infinity as
// the result, (0,0) is returned.
//
// Standard x86-64 ABI: RDI = res, RSI = scalar, RDX = point
// Microsoft x64 ABI:   RCX = res, RDX = scalar, R8 = point
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_scalarmul_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_scalarmul_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_scalarmul_alt)

        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Intermediate variables on the stack. The last z2, z3 values can
// safely be overlaid on the table, which is no longer needed at the end.
// Uppercase syntactic variants make x86_att version simpler to generate

#define SCALARB (0*NUMSIZE)
#define scalarb (0*NUMSIZE)(%rsp)
#define ACC (1*NUMSIZE)
#define acc (1*NUMSIZE)(%rsp)
#define TABENT (4*NUMSIZE)
#define tabent (4*NUMSIZE)(%rsp)

#define TAB (7*NUMSIZE)
#define tab (7*NUMSIZE)(%rsp)

#define Z2 (7*NUMSIZE)
#define z2 (7*NUMSIZE)(%rsp)
#define Z3 (8*NUMSIZE)
#define z3 (8*NUMSIZE)(%rsp)

#define res (31*NUMSIZE)(%rsp)

#define NSPACE 32*NUMSIZE

S2N_BN_SYMBOL(p256_scalarmul_alt):
        CFI_START
        _CET_ENDBR

// The Windows version literally calls the standard ABI version.
// This simplifies the proofs since subroutine offsets are fixed.

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        CFI_CALL(Lp256_scalarmul_alt_standard)
        CFI_POP(%rsi)
        CFI_POP(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p256_scalarmul_alt)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_scalarmul_alt_standard)

Lp256_scalarmul_alt_standard:
        CFI_START
#endif

// Real start of the standard ABI code.

        CFI_PUSH(%r15)
        CFI_PUSH(%r14)
        CFI_PUSH(%r13)
        CFI_PUSH(%r12)
        CFI_PUSH(%rbp)
        CFI_PUSH(%rbx)

        CFI_DEC_RSP(NSPACE)

// Preserve the "res" and "point" input arguments. We load and process the
// scalar immediately so we don't bother preserving that input argument.
// Also, "point" is only needed early on and so its register gets re-used.

        movq    %rdx, %rbx
        movq    %rdi, res

// Load the digits of group order n_256 = [%r15;%r14;%r13;%r12]

        movq    $0xf3b9cac2fc632551, %r12
        movq    $0xbce6faada7179e84, %r13
        movq    $0xffffffffffffffff, %r14
        movq    $0xffffffff00000000, %r15

// First, reduce the input scalar mod n_256, i.e. conditionally subtract n_256

        movq    (%rsi), %r8
        subq    %r12, %r8
        movq    8(%rsi), %r9
        sbbq    %r13, %r9
        movq    16(%rsi), %r10
        sbbq    %r14, %r10
        movq    24(%rsi), %r11
        sbbq    %r15, %r11

        cmovcq  (%rsi), %r8
        cmovcq  8(%rsi), %r9
        cmovcq  16(%rsi), %r10
        cmovcq  24(%rsi), %r11

// Now if the top bit of the reduced scalar is set, negate it mod n_256,
// i.e. do n |-> n_256 - n. Remember the sign in %rbp so we can
// correspondingly negate the point below.

        subq    %r8, %r12
        sbbq    %r9, %r13
        sbbq    %r10, %r14
        sbbq    %r11, %r15

        movq    %r11, %rbp
        shrq    $63, %rbp
        cmovnzq %r12, %r8
        cmovnzq %r13, %r9
        cmovnzq %r14, %r10
        cmovnzq %r15, %r11

// In either case then add the recoding constant 0x08888...888 to allow
// signed digits.

        movq    $0x8888888888888888, %rax
        addq    %rax, %r8
        adcq    %rax, %r9
        adcq    %rax, %r10
        adcq    %rax, %r11
        btc     $63, %r11

        movq    %r8, SCALARB(%rsp)
        movq    %r9, SCALARB+8(%rsp)
        movq    %r10, SCALARB+16(%rsp)
        movq    %r11, SCALARB+24(%rsp)

// Set the tab[0] table entry to Montgomery-Jacobian point = 1 * P
// The z coordinate is just the Montgomery form of the constant 1.

        leaq    TAB(%rsp), %rdi
        movq    %rbx, %rsi
        CFI_CALL(Lp256_scalarmul_alt_local_tomont_p256)

        leaq    32(%rbx), %rsi
        leaq    TAB+32(%rsp), %rdi
        CFI_CALL(Lp256_scalarmul_alt_local_tomont_p256)

        movl    $1, %eax
        movq    %rax, TAB+64(%rsp)
        movq    $0xffffffff00000000, %rdx
        movq    %rdx, TAB+72(%rsp)
        subq    $2, %rax
        movq    %rax, TAB+80(%rsp)
        movq    $0x00000000fffffffe, %rax
        movq    %rax, TAB+88(%rsp)

// If the top bit of the scalar was set, negate (y coordinate of) the point

        movq    TAB+32(%rsp), %r12
        movq    TAB+40(%rsp), %r13
        movq    TAB+48(%rsp), %r14
        movq    TAB+56(%rsp), %r15

        xorl    %r10d, %r10d
        leaq    -1(%r10), %r8
        movq    $0x00000000ffffffff, %r11
        movq    %r11, %r9
        negq    %r11

        subq    %r12, %r8
        sbbq    %r13, %r9
        sbbq    %r14, %r10
        sbbq    %r15, %r11

        testq   %rbp, %rbp
        cmovzq  %r12, %r8
        cmovzq  %r13, %r9
        cmovzq  %r14, %r10
        cmovzq  %r15, %r11

        movq    %r8, TAB+32(%rsp)
        movq    %r9, TAB+40(%rsp)
        movq    %r10, TAB+48(%rsp)
        movq    %r11, TAB+56(%rsp)

// Compute and record tab[1] = 2 * p, ..., tab[7] = 8 * P

        leaq    TAB+96*1(%rsp), %rdi
        leaq    TAB(%rsp), %rsi
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjdouble)

        leaq    TAB+96*2(%rsp), %rdi
        leaq    TAB+96*1(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjmixadd)

        leaq    TAB+96*3(%rsp), %rdi
        leaq    TAB+96*1(%rsp), %rsi
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjdouble)

        leaq    TAB+96*4(%rsp), %rdi
        leaq    TAB+96*3(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjmixadd)

        leaq    TAB+96*5(%rsp), %rdi
        leaq    TAB+96*2(%rsp), %rsi
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjdouble)

        leaq    TAB+96*6(%rsp), %rdi
        leaq    TAB+96*5(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjmixadd)

        leaq    TAB+96*7(%rsp), %rdi
        leaq    TAB+96*3(%rsp), %rsi
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjdouble)

// Set up accumulator as table entry for top 4 bits (constant-time indexing)

        movq    SCALARB+24(%rsp), %rdi
        shrq    $60, %rdi

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        xorl    %r8d, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d
        xorl    %r12d, %r12d
        xorl    %r13d, %r13d
        xorl    %r14d, %r14d
        xorl    %r15d, %r15d

        .set I, 1
.rep 8
        cmpq    $I, %rdi

        cmovzq  TAB+96*(I-1)(%rsp), %rax
        cmovzq  TAB+96*(I-1)+8(%rsp), %rbx
        cmovzq  TAB+96*(I-1)+16(%rsp), %rcx
        cmovzq  TAB+96*(I-1)+24(%rsp), %rdx
        cmovzq  TAB+96*(I-1)+32(%rsp), %r8
        cmovzq  TAB+96*(I-1)+40(%rsp), %r9
        cmovzq  TAB+96*(I-1)+48(%rsp), %r10
        cmovzq  TAB+96*(I-1)+56(%rsp), %r11
        cmovzq  TAB+96*(I-1)+64(%rsp), %r12
        cmovzq  TAB+96*(I-1)+72(%rsp), %r13
        cmovzq  TAB+96*(I-1)+80(%rsp), %r14
        cmovzq  TAB+96*(I-1)+88(%rsp), %r15
        .set    I, (I+1)
.endr
        movq     %rax, ACC(%rsp)
        movq     %rbx, ACC+8(%rsp)
        movq     %rcx, ACC+16(%rsp)
        movq     %rdx, ACC+24(%rsp)
        movq     %r8, ACC+32(%rsp)
        movq     %r9, ACC+40(%rsp)
        movq     %r10, ACC+48(%rsp)
        movq     %r11, ACC+56(%rsp)
        movq     %r12, ACC+64(%rsp)
        movq     %r13, ACC+72(%rsp)
        movq     %r14, ACC+80(%rsp)
        movq     %r15, ACC+88(%rsp)

// Main loop over size-4 bitfield

        movl    $252, %ebp

Lp256_scalarmul_alt_loop:
        subq    $4, %rbp

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjdouble)

        movq    %rbp, %rax
        shrq    $6, %rax
        movq    (%rsp,%rax,8), %rdi
        movq    %rbp, %rcx
        shrq    %cl, %rdi
        andq    $15, %rdi

        subq    $8, %rdi
        sbbq    %rsi, %rsi // %rsi = sign of digit (-1 = negative)
        xorq    %rsi, %rdi
        subq    %rsi, %rdi // %rdi = absolute value of digit

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        xorl    %r8d, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d
        xorl    %r12d, %r12d
        xorl    %r13d, %r13d
        xorl    %r14d, %r14d
        xorl    %r15d, %r15d

        .set I, 1
.rep 8
        cmpq    $I, %rdi

        cmovzq  TAB+96*(I-1)(%rsp), %rax
        cmovzq  TAB+96*(I-1)+8(%rsp), %rbx
        cmovzq  TAB+96*(I-1)+16(%rsp), %rcx
        cmovzq  TAB+96*(I-1)+24(%rsp), %rdx
        cmovzq  TAB+96*(I-1)+32(%rsp), %r8
        cmovzq  TAB+96*(I-1)+40(%rsp), %r9
        cmovzq  TAB+96*(I-1)+48(%rsp), %r10
        cmovzq  TAB+96*(I-1)+56(%rsp), %r11
        cmovzq  TAB+96*(I-1)+64(%rsp), %r12
        cmovzq  TAB+96*(I-1)+72(%rsp), %r13
        cmovzq  TAB+96*(I-1)+80(%rsp), %r14
        cmovzq  TAB+96*(I-1)+88(%rsp), %r15
        .set    I, (I+1)
.endr

        movq     %r12, TABENT+64(%rsp)
        movq     %r13, TABENT+72(%rsp)
        movq     %r14, TABENT+80(%rsp)
        movq     %r15, TABENT+88(%rsp)

        xorl    %r14d, %r14d
        leaq    -1(%r14), %r12
        movq    $0x00000000ffffffff, %r15
        movq    %r15, %r13
        negq    %r15

        subq    %r8, %r12
        sbbq    %r9, %r13
        sbbq    %r10, %r14
        sbbq    %r11, %r15

        testq    %rsi, %rsi
        cmovnzq  %r12, %r8
        cmovnzq  %r13, %r9
        cmovnzq  %r14, %r10
        cmovnzq  %r15, %r11

        movq     %rax, TABENT(%rsp)
        movq     %rbx, TABENT+8(%rsp)
        movq     %rcx, TABENT+16(%rsp)
        movq     %rdx, TABENT+24(%rsp)

        movq     %r8, TABENT+32(%rsp)
        movq     %r9, TABENT+40(%rsp)
        movq     %r10, TABENT+48(%rsp)
        movq     %r11, TABENT+56(%rsp)

        leaq    TABENT(%rsp), %rdx
        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_scalarmul_alt_local_p256_montjadd)

        testq   %rbp, %rbp
        jne     Lp256_scalarmul_alt_loop

// Let z2 = 1/z^2 and z3 = 1/z^3, both without Montgomery form

        leaq    Z2(%rsp), %rdi
        leaq    ACC+64(%rsp), %rsi
        CFI_CALL(Lp256_scalarmul_alt_local_montsqr_p256)

        leaq    Z3(%rsp), %rdi
        leaq    ACC+64(%rsp), %rsi
        leaq    Z2(%rsp), %rdx
        CFI_CALL(Lp256_scalarmul_alt_local_montmul_p256)

        leaq    Z2(%rsp), %rdi
        leaq    Z3(%rsp), %rsi
        CFI_CALL(Lp256_scalarmul_alt_local_demont_p256)

        leaq    Z3(%rsp), %rdi
        leaq    Z2(%rsp), %rsi
        CFI_CALL(Lp256_scalarmul_alt_local_inv_p256)

        leaq    Z2(%rsp), %rdi
        leaq    ACC+64(%rsp), %rsi
        leaq    Z3(%rsp), %rdx
        CFI_CALL(Lp256_scalarmul_alt_local_montmul_p256)

// Convert back from Jacobian (X, Y, Z) |-> (X/Z^2, Y/Z^3)

        movq    res, %rdi
        leaq    ACC(%rsp), %rsi
        leaq    Z2(%rsp), %rdx
        movq    %rdi, %rbx
        CFI_CALL(Lp256_scalarmul_alt_local_montmul_p256)

        leaq    32(%rbx), %rdi
        leaq    ACC+32(%rsp), %rsi
        leaq    Z3(%rsp), %rdx
        CFI_CALL(Lp256_scalarmul_alt_local_montmul_p256)

// Restore stack and registers and return

        CFI_INC_RSP(NSPACE)
        CFI_POP(%rbx)
        CFI_POP(%rbp)
        CFI_POP(%r12)
        CFI_POP(%r13)
        CFI_POP(%r14)
        CFI_POP(%r15)
        CFI_RET

#if WINDOWS_ABI
S2N_BN_SIZE_DIRECTIVE(Lp256_scalarmul_alt_standard)
#else
S2N_BN_SIZE_DIRECTIVE(p256_scalarmul_alt)
#endif

// Local copies of subroutines, complete clones at the moment

Lp256_scalarmul_alt_local_demont_p256:
        CFI_START
        movq    (%rsi), %r8
        movq    0x8(%rsi), %r9
        movq    0x10(%rsi), %r10
        movq    0x18(%rsi), %r11
        movabsq $0x100000000, %rcx
        movq    %r8, %rax
        mulq    %rcx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rsi, %rsi
        movq    %r9, %rax
        mulq    %rcx
        subq    %rsi, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rsi, %rsi
        negq    %rcx
        negq    %rsi
        incq    %rcx
        movq    %r8, %rax
        mulq    %rcx
        addq    %rax, %r11
        adcq    %rdx, %rsi
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rax
        mulq    %rcx
        addq    %rax, %rsi
        adcq    %rdx, %r8
        negq    %rcx
        incq    %rcx
        movq    %r10, %rax
        mulq    %rcx
        addq    %rax, %r11
        adcq    %rdx, %rsi
        sbbq    %r9, %r9
        movq    %r11, %rax
        mulq    %rcx
        subq    %r9, %rdx
        addq    %rax, %rsi
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %rcx
        negq    %r9
        incq    %rcx
        movq    %r10, %rax
        mulq    %rcx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rax
        mulq    %rcx
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    %rsi, (%rdi)
        movq    %r8, 0x8(%rdi)
        movq    %r9, 0x10(%rdi)
        movq    %r10, 0x18(%rdi)
        CFI_RET

Lp256_scalarmul_alt_local_inv_p256:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(240)
        movq    %rdi, 0xe0(%rsp)
        xorl    %ecx, %ecx
        movl    $0xffffffff, %edx
        movq    %rdx, %rbx
        leaq    -0x1(%rcx), %rax
        negq    %rdx
        movq    %rax, (%rsp)
        movq    %rbx, 0x8(%rsp)
        movq    %rcx, 0x10(%rsp)
        movq    %rdx, 0x18(%rsp)
        movq    %rcx, 0x20(%rsp)
        movq    (%rsi), %r8
        movq    0x8(%rsi), %r9
        movq    0x10(%rsi), %r10
        movq    0x18(%rsi), %r11
        leaq    0x1(%rcx), %rax
        addq    %r8, %rax
        leaq    -0x1(%rdx), %rbx
        adcq    %r9, %rbx
        notq    %rcx
        adcq    %r10, %rcx
        notq    %rdx
        adcq    %r11, %rdx
        cmovaeq %r8, %rax
        cmovaeq %r9, %rbx
        cmovaeq %r10, %rcx
        cmovaeq %r11, %rdx
        movq    %rax, 0x28(%rsp)
        movq    %rbx, 0x30(%rsp)
        movq    %rcx, 0x38(%rsp)
        movq    %rdx, 0x40(%rsp)
        xorl    %eax, %eax
        movq    %rax, 0x48(%rsp)
        xorl    %eax, %eax
        movq    %rax, 0x50(%rsp)
        movq    %rax, 0x58(%rsp)
        movq    %rax, 0x60(%rsp)
        movq    %rax, 0x68(%rsp)
        movabsq $0x4000000000000, %rcx
        movq    %rcx, 0x78(%rsp)
        movq    %rax, 0x80(%rsp)
        movq    %rax, 0x88(%rsp)
        movq    %rax, 0x90(%rsp)
        movq    $0xa,  0xb0(%rsp)
        movq    $0x1,  0xb8(%rsp)
        jmp     Lp256_scalarmul_alt_inv_midloop
Lp256_scalarmul_alt_inv_loop:
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
        movq    %rdi, 0xa0(%rsp)
        movq    %r12, %rax
        andq    %r13, %rax
        movq    %r14, %rsi
        andq    %r15, %rsi
        addq    %rax, %rsi
        movq    %rsi, 0xa8(%rsp)
        xorl    %ebx, %ebx
        movq    (%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        movq    0x28(%rsp), %rax
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
        movq    0x28(%rsp), %rax
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
        movq    0x30(%rsp), %rax
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
        movq    0x30(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rdi
        shrdq   $0x3b, %rbp, %rsi
        movq    %rsi, 0x28(%rsp)
        xorl    %esi, %esi
        movq    0x10(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rsi
        movq    0x38(%rsp), %rax
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
        movq    0x38(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rdi
        adcq    %rdx, %rbx
        shrdq   $0x3b, %rdi, %rbp
        movq    %rbp, 0x30(%rsp)
        movq    0x18(%rsp), %rax
        xorq    %r9, %rax
        movq    0x20(%rsp), %rbp
        xorq    %r9, %rbp
        andq    %r8, %rbp
        negq    %rbp
        mulq    %r8
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    0x40(%rsp), %rax
        xorq    %r11, %rax
        movq    0x48(%rsp), %rdx
        xorq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %rbp
        mulq    %r10
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        shrdq   $0x3b, %rsi, %rcx
        movq    %rcx, 0x10(%rsp)
        shrdq   $0x3b, %rbp, %rsi
        sarq    $0x3b, %rbp
        movq    0x18(%rsp), %rax
        movq    %rsi, 0x18(%rsp)
        movq    0x20(%rsp), %rsi
        movq    %rbp, 0x20(%rsp)
        xorq    %r13, %rax
        xorq    %r13, %rsi
        andq    %r12, %rsi
        negq    %rsi
        mulq    %r12
        addq    %rax, %rbx
        adcq    %rdx, %rsi
        movq    0x40(%rsp), %rax
        xorq    %r15, %rax
        movq    0x48(%rsp), %rdx
        xorq    %r15, %rdx
        andq    %r14, %rdx
        subq    %rdx, %rsi
        mulq    %r14
        addq    %rax, %rbx
        adcq    %rdx, %rsi
        shrdq   $0x3b, %rbx, %rdi
        movq    %rdi, 0x38(%rsp)
        shrdq   $0x3b, %rsi, %rbx
        movq    %rbx, 0x40(%rsp)
        sarq    $0x3b, %rsi
        movq    %rsi, 0x48(%rsp)
        movq    0xa0(%rsp), %rbx
        movq    0xa8(%rsp), %rbp
        xorl    %ecx, %ecx
        movq    0x50(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    0x78(%rsp), %rax
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
        movq    0x78(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    %rbp, 0x78(%rsp)
        xorl    %ebx, %ebx
        movq    0x58(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    0x80(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        xorl    %ebp, %ebp
        movq    0x58(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rcx, 0x58(%rsp)
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    0x80(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rdx, %rbp
        movq    %rsi, 0x80(%rsp)
        xorl    %ecx, %ecx
        movq    0x60(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        movq    0x88(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %rbx
        adcq    %rdx, %rcx
        xorl    %esi, %esi
        movq    0x60(%rsp), %rax
        xorq    %r13, %rax
        mulq    %r12
        movq    %rbx, 0x60(%rsp)
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    0x88(%rsp), %rax
        xorq    %r15, %rax
        mulq    %r14
        addq    %rax, %rbp
        adcq    %rdx, %rsi
        movq    %rbp, 0x88(%rsp)
        movq    0x68(%rsp), %rax
        xorq    %r9, %rax
        movq    %r9, %rbx
        andq    %r8, %rbx
        negq    %rbx
        mulq    %r8
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        movq    0x90(%rsp), %rax
        xorq    %r11, %rax
        movq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %rbx
        mulq    %r10
        addq    %rax, %rcx
        adcq    %rbx, %rdx
        movq    0x68(%rsp), %rax
        movq    %rcx, 0x68(%rsp)
        movq    %rdx, 0x70(%rsp)
        xorq    %r13, %rax
        movq    %r13, %rcx
        andq    %r12, %rcx
        negq    %rcx
        mulq    %r12
        addq    %rax, %rsi
        adcq    %rdx, %rcx
        movq    0x90(%rsp), %rax
        xorq    %r15, %rax
        movq    %r15, %rdx
        andq    %r14, %rdx
        subq    %rdx, %rcx
        mulq    %r14
        addq    %rax, %rsi
        adcq    %rcx, %rdx
        movq    %rsi, 0x90(%rsp)
        movq    %rdx, 0x98(%rsp)
        movabsq $0xe000000000000000, %r8
        addq    0x50(%rsp), %r8
        movq    $0xffffffffffffffff, %r9
        adcq    0x58(%rsp), %r9
        movq    $0x1fffffff, %r10
        adcq    0x60(%rsp), %r10
        movabsq $0x2000000000000000, %r11
        adcq    0x68(%rsp), %r11
        movabsq $0x1fffffffe0000000, %r12
        adcq    0x70(%rsp), %r12
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %r8
        shrq    $0x20, %r8
        addq    %rbx, %r9
        adcq    %r8, %r10
        adcq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rax, %rax
        movl    $0xffffffff, %ebx
        andq    %rax, %rbx
        movabsq $0xffffffff00000001, %rdx
        andq    %rax, %rdx
        subq    %rax, %r9
        movq    %r9, 0x50(%rsp)
        sbbq    %rbx, %r10
        movq    %r10, 0x58(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x60(%rsp)
        sbbq    %rdx, %r12
        movq    %r12, 0x68(%rsp)
        movabsq $0xe000000000000000, %r8
        addq    0x78(%rsp), %r8
        movq    $0xffffffffffffffff, %r9
        adcq    0x80(%rsp), %r9
        movq    $0x1fffffff, %r10
        adcq    0x88(%rsp), %r10
        movabsq $0x2000000000000000, %r11
        adcq    0x90(%rsp), %r11
        movabsq $0x1fffffffe0000000, %r12
        adcq    0x98(%rsp), %r12
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %r8
        shrq    $0x20, %r8
        addq    %rbx, %r9
        adcq    %r8, %r10
        adcq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rax, %rax
        movl    $0xffffffff, %ebx
        andq    %rax, %rbx
        movabsq $0xffffffff00000001, %rdx
        andq    %rax, %rdx
        subq    %rax, %r9
        movq    %r9, 0x78(%rsp)
        sbbq    %rbx, %r10
        movq    %r10, 0x80(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x88(%rsp)
        sbbq    %rdx, %r12
        movq    %r12, 0x90(%rsp)
Lp256_scalarmul_alt_inv_midloop:
        movq    0xb8(%rsp), %rsi
        movq    (%rsp), %rdx
        movq    0x28(%rsp), %rcx
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
        movq    %rdx, 0xc0(%rsp)
        movq    %rbx, 0xc8(%rsp)
        movq    %rdi, 0xd0(%rsp)
        movq    %rcx, 0xd8(%rsp)
        movq    (%rsp), %r12
        imulq   %r12, %rdi
        imulq   %rdx, %r12
        movq    0x28(%rsp), %r13
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
        movq    0xc0(%rsp), %rax
        imulq   %r8, %rax
        movq    0xd0(%rsp), %rdx
        imulq   %r15, %rdx
        imulq   0xc8(%rsp), %r8
        imulq   0xd8(%rsp), %r15
        addq    %r8, %r15
        leaq    (%rax,%rdx), %r9
        movq    0xc0(%rsp), %rax
        imulq   %r10, %rax
        movq    0xd0(%rsp), %rdx
        imulq   %r11, %rdx
        imulq   0xc8(%rsp), %r10
        imulq   0xd8(%rsp), %r11
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
        movq    %rsi, 0xb8(%rsp)
        decq     0xb0(%rsp)
        jne     Lp256_scalarmul_alt_inv_loop
        movq    (%rsp), %rax
        movq    0x28(%rsp), %rcx
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
        movq    0x50(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x78(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movq    0x58(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r13
        adcq    %rdx, %r14
        movq    0x80(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    0x60(%rsp), %rax
        xorq    %r9, %rax
        mulq    %r8
        addq    %rax, %r14
        adcq    %rdx, %r15
        movq    0x88(%rsp), %rax
        xorq    %r11, %rax
        mulq    %r10
        addq    %rax, %r14
        adcq    %rdx, %r15
        movq    0x68(%rsp), %rax
        xorq    %r9, %rax
        andq    %r8, %r9
        negq    %r9
        mulq    %r8
        addq    %rax, %r15
        adcq    %rdx, %r9
        movq    0x90(%rsp), %rax
        xorq    %r11, %rax
        movq    %r11, %rdx
        andq    %r10, %rdx
        subq    %rdx, %r9
        mulq    %r10
        addq    %rax, %r15
        adcq    %rdx, %r9
        movq    %r12, 0x50(%rsp)
        movq    %r13, 0x58(%rsp)
        movq    %r14, 0x60(%rsp)
        movq    %r15, 0x68(%rsp)
        movq    %r9, 0x70(%rsp)
        movabsq $0xe000000000000000, %r8
        addq    0x50(%rsp), %r8
        movq    $0xffffffffffffffff, %r9
        adcq    0x58(%rsp), %r9
        movq    $0x1fffffff, %r10
        adcq    0x60(%rsp), %r10
        movabsq $0x2000000000000000, %r11
        adcq    0x68(%rsp), %r11
        movabsq $0x1fffffffe0000000, %r12
        adcq    0x70(%rsp), %r12
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %r8
        shrq    $0x20, %r8
        addq    %rbx, %r9
        adcq    %r8, %r10
        adcq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rax, %rax
        movl    $0xffffffff, %ebx
        andq    %rax, %rbx
        movabsq $0xffffffff00000001, %rdx
        andq    %rax, %rdx
        subq    %rax, %r9
        movq    %r9, 0x50(%rsp)
        sbbq    %rbx, %r10
        movq    %r10, 0x58(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x60(%rsp)
        sbbq    %rdx, %r12
        movq    %r12, 0x68(%rsp)
        movq    0x50(%rsp), %r8
        movq    0x58(%rsp), %r9
        movq    0x60(%rsp), %r10
        movq    0x68(%rsp), %r11
        movl    $0x1, %eax
        movl    $0xffffffff, %ebx
        leaq    -0x2(%rax), %rcx
        leaq    -0x1(%rbx), %rdx
        notq    %rbx
        addq    %r8, %rax
        adcq    %r9, %rbx
        adcq    %r10, %rcx
        adcq    %r11, %rdx
        cmovaeq %r8, %rax
        cmovaeq %r9, %rbx
        cmovaeq %r10, %rcx
        cmovaeq %r11, %rdx
        movq    0xe0(%rsp), %rdi
        movq    %rax, (%rdi)
        movq    %rbx, 0x8(%rdi)
        movq    %rcx, 0x10(%rdi)
        movq    %rdx, 0x18(%rdi)
        CFI_INC_RSP(240)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

Lp256_scalarmul_alt_local_montmul_p256:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        movq    %rdx, %rcx
        movq    (%rcx), %rbx
        movq    (%rsi), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x8(%rsi), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x10(%rsi), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x18(%rsi), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x8(%rcx), %rbx
        xorl    %r13d, %r13d
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x10(%rcx), %rbx
        xorl    %r15d, %r15d
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x18(%rcx), %rbx
        xorl    %r8d, %r8d
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rdi)
        movq    %r13, 0x8(%rdi)
        movq    %r14, 0x10(%rdi)
        movq    %r15, 0x18(%rdi)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
        CFI_RET

Lp256_scalarmul_alt_local_montsqr_p256:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        movq    (%rsi), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x8(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x18(%rsi), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x10(%rsi), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x18(%rsi), %rbx
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x8(%rsi), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x10(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x18(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rdi)
        movq    %r13, 0x8(%rdi)
        movq    %r14, 0x10(%rdi)
        movq    %r15, 0x18(%rdi)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
        CFI_RET


Lp256_scalarmul_alt_local_tomont_p256:
        CFI_START
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        movl    $0x3, %ecx
        movq    (%rsi), %rax
        mulq    %rcx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x8(%rsi), %rax
        mulq    %rcx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x10(%rsi), %rax
        mulq    %rcx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x18(%rsi), %rax
        mulq    %rcx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movabsq $0xfffffffbffffffff, %rcx
        xorl    %r13d, %r13d
        movq    (%rsi), %rax
        mulq    %rcx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x8(%rsi), %rax
        mulq    %rcx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x10(%rsi), %rax
        mulq    %rcx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x18(%rsi), %rax
        mulq    %rcx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rcx
        movq    %r8, %rax
        mulq    %rcx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rcx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rcx
        leaq    0x2(%rcx), %rcx
        movq    %r8, %rax
        mulq    %rcx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rcx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    $0xfffffffffffffffe, %rcx
        xorl    %r15d, %r15d
        movq    (%rsi), %rax
        mulq    %rcx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x8(%rsi), %rax
        mulq    %rcx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x10(%rsi), %rax
        mulq    %rcx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x18(%rsi), %rax
        mulq    %rcx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movabsq $0x4fffffffd, %rcx
        xorl    %r8d, %r8d
        movq    (%rsi), %rax
        mulq    %rcx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x8(%rsi), %rax
        mulq    %rcx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x10(%rsi), %rax
        mulq    %rcx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x18(%rsi), %rax
        mulq    %rcx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rcx
        movq    %r10, %rax
        mulq    %rcx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    %r11, %rax
        mulq    %rcx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        notq    %rcx
        leaq    0x2(%rcx), %rcx
        movq    %r10, %rax
        mulq    %rcx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    %r11, %rax
        mulq    %rcx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        subq    %r9, %r8
        xorl    %edx, %edx
        leaq    -0x1(%rdx), %r9
        incq    %rdx
        addq    %r12, %rdx
        decq    %rcx
        adcq    %r13, %rcx
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rdx, %r12
        cmovbq  %rcx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rdi)
        movq    %r13, 0x8(%rdi)
        movq    %r14, 0x10(%rdi)
        movq    %r15, 0x18(%rdi)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_RET

Lp256_scalarmul_alt_local_p256_montjadd:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(224)
        movq    %rdx, %rbp
        movq    0x40(%rsi), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x48(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x58(%rsi), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x50(%rsi), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x40(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x58(%rsi), %rbx
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x48(%rsi), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x50(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x58(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        movq    0x40(%rbp), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x48(%rbp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x58(%rbp), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x50(%rbp), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x40(%rbp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x48(%rbp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x58(%rbp), %rbx
        movq    0x48(%rbp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x48(%rbp), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x50(%rbp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x58(%rbp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xa0(%rsp)
        movq    %r13, 0xa8(%rsp)
        movq    %r14, 0xb0(%rsp)
        movq    %r15, 0xb8(%rsp)
        movq    0x20(%rsi), %rbx
        movq    0x40(%rbp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x48(%rbp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x50(%rbp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x58(%rbp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x28(%rsi), %rbx
        xorl    %r13d, %r13d
        movq    0x40(%rbp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x48(%rbp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x50(%rbp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x58(%rbp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x30(%rsi), %rbx
        xorl    %r15d, %r15d
        movq    0x40(%rbp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x48(%rbp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x50(%rbp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x58(%rbp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x38(%rsi), %rbx
        xorl    %r8d, %r8d
        movq    0x40(%rbp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x48(%rbp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x50(%rbp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x58(%rbp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xc0(%rsp)
        movq    %r13, 0xc8(%rsp)
        movq    %r14, 0xd0(%rsp)
        movq    %r15, 0xd8(%rsp)
        movq    0x20(%rbp), %rbx
        movq    0x40(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x48(%rsi), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x50(%rsi), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x58(%rsi), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x28(%rbp), %rbx
        xorl    %r13d, %r13d
        movq    0x40(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x50(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x58(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x30(%rbp), %rbx
        xorl    %r15d, %r15d
        movq    0x40(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x50(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x58(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x38(%rbp), %rbx
        xorl    %r8d, %r8d
        movq    0x40(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x50(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x58(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x20(%rsp)
        movq    %r13, 0x28(%rsp)
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        movq    0x0(%rbp), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x10(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x18(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x8(%rbp), %rbx
        xorl    %r13d, %r13d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x10(%rbp), %rbx
        xorl    %r15d, %r15d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x18(%rbp), %rbx
        xorl    %r8d, %r8d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x40(%rsp)
        movq    %r13, 0x48(%rsp)
        movq    %r14, 0x50(%rsp)
        movq    %r15, 0x58(%rsp)
        movq    (%rsi), %rbx
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x8(%rsi), %rbx
        xorl    %r13d, %r13d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x10(%rsi), %rbx
        xorl    %r15d, %r15d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x18(%rsi), %rbx
        xorl    %r8d, %r8d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x80(%rsp)
        movq    %r13, 0x88(%rsp)
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    0x20(%rsp), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x10(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x18(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x28(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x30(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x38(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x20(%rsp)
        movq    %r13, 0x28(%rsp)
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        movq    0xc0(%rsp), %rbx
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0xc8(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0xd0(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0xd8(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xc0(%rsp)
        movq    %r13, 0xc8(%rsp)
        movq    %r14, 0xd0(%rsp)
        movq    %r15, 0xd8(%rsp)
        movq    0x40(%rsp), %rax
        subq    0x80(%rsp), %rax
        movq    0x48(%rsp), %rcx
        sbbq    0x88(%rsp), %rcx
        movq    0x50(%rsp), %r8
        sbbq    0x90(%rsp), %r8
        movq    0x58(%rsp), %r9
        sbbq    0x98(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0xa0(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0xa8(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0xb0(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0xb8(%rsp)
        movq    0x20(%rsp), %rax
        subq    0xc0(%rsp), %rax
        movq    0x28(%rsp), %rcx
        sbbq    0xc8(%rsp), %rcx
        movq    0x30(%rsp), %r8
        sbbq    0xd0(%rsp), %r8
        movq    0x38(%rsp), %r9
        sbbq    0xd8(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x20(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x28(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x30(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x38(%rsp)
        movq    0xa0(%rsp), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0xb8(%rsp), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0xb0(%rsp), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0xb8(%rsp), %rbx
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0xa8(%rsp), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0xb0(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0xb8(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x60(%rsp)
        movq    %r13, 0x68(%rsp)
        movq    %r14, 0x70(%rsp)
        movq    %r15, 0x78(%rsp)
        movq    0x20(%rsp), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x28(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x38(%rsp), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x30(%rsp), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x20(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x38(%rsp), %rbx
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x28(%rsp), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x30(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x38(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        movq    0x80(%rsp), %rbx
        movq    0x60(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x70(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x78(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x88(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x90(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x98(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x80(%rsp)
        movq    %r13, 0x88(%rsp)
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    0x40(%rsp), %rbx
        movq    0x60(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x70(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x78(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x48(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x50(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x58(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x40(%rsp)
        movq    %r13, 0x48(%rsp)
        movq    %r14, 0x50(%rsp)
        movq    %r15, 0x58(%rsp)
        movq    (%rsp), %rax
        subq    0x80(%rsp), %rax
        movq    0x8(%rsp), %rcx
        sbbq    0x88(%rsp), %rcx
        movq    0x10(%rsp), %r8
        sbbq    0x90(%rsp), %r8
        movq    0x18(%rsp), %r9
        sbbq    0x98(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, (%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x8(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x10(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x18(%rsp)
        movq    0x40(%rsp), %rax
        subq    0x80(%rsp), %rax
        movq    0x48(%rsp), %rcx
        sbbq    0x88(%rsp), %rcx
        movq    0x50(%rsp), %r8
        sbbq    0x90(%rsp), %r8
        movq    0x58(%rsp), %r9
        sbbq    0x98(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x60(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x68(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x70(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x78(%rsp)
        movq    0x40(%rsi), %rbx
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x48(%rsi), %rbx
        xorl    %r13d, %r13d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x50(%rsi), %rbx
        xorl    %r15d, %r15d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x58(%rsi), %rbx
        xorl    %r8d, %r8d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xa0(%rsp)
        movq    %r13, 0xa8(%rsp)
        movq    %r14, 0xb0(%rsp)
        movq    %r15, 0xb8(%rsp)
        movq    (%rsp), %rax
        subq    0x40(%rsp), %rax
        movq    0x8(%rsp), %rcx
        sbbq    0x48(%rsp), %rcx
        movq    0x10(%rsp), %r8
        sbbq    0x50(%rsp), %r8
        movq    0x18(%rsp), %r9
        sbbq    0x58(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, (%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x8(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x10(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x18(%rsp)
        movq    0x80(%rsp), %rax
        subq    (%rsp), %rax
        movq    0x88(%rsp), %rcx
        sbbq    0x8(%rsp), %rcx
        movq    0x90(%rsp), %r8
        sbbq    0x10(%rsp), %r8
        movq    0x98(%rsp), %r9
        sbbq    0x18(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x80(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x88(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x90(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x98(%rsp)
        movq    0xc0(%rsp), %rbx
        movq    0x60(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x70(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x78(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0xc8(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0xd0(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0xd8(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x60(%rsp)
        movq    %r13, 0x68(%rsp)
        movq    %r14, 0x70(%rsp)
        movq    %r15, 0x78(%rsp)
        movq    0x40(%rbp), %rbx
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x48(%rbp), %rbx
        xorl    %r13d, %r13d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x50(%rbp), %rbx
        xorl    %r15d, %r15d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x58(%rbp), %rbx
        xorl    %r8d, %r8d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xa0(%rsp)
        movq    %r13, 0xa8(%rsp)
        movq    %r14, 0xb0(%rsp)
        movq    %r15, 0xb8(%rsp)
        movq    0x80(%rsp), %rbx
        movq    0x20(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x28(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x30(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x38(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x88(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    0x20(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x30(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x90(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    0x20(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x30(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x98(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    0x20(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x30(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x80(%rsp)
        movq    %r13, 0x88(%rsp)
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    0x80(%rsp), %rax
        subq    0x60(%rsp), %rax
        movq    0x88(%rsp), %rcx
        sbbq    0x68(%rsp), %rcx
        movq    0x90(%rsp), %r8
        sbbq    0x70(%rsp), %r8
        movq    0x98(%rsp), %r9
        sbbq    0x78(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x80(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x88(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x90(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x98(%rsp)
        movq    0x40(%rsi), %r8
        movq    0x48(%rsi), %r9
        movq    0x50(%rsi), %r10
        movq    0x58(%rsi), %r11
        movq    %r8, %rax
        movq    %r9, %rdx
        orq     %r10, %rax
        orq     %r11, %rdx
        orq     %rdx, %rax
        negq    %rax
        sbbq    %rax, %rax
        movq    0x40(%rbp), %r12
        movq    0x48(%rbp), %r13
        movq    0x50(%rbp), %r14
        movq    0x58(%rbp), %r15
        movq    %r12, %rbx
        movq    %r13, %rdx
        orq     %r14, %rbx
        orq     %r15, %rdx
        orq     %rdx, %rbx
        negq    %rbx
        sbbq    %rbx, %rbx
        cmpq    %rax, %rbx
        cmovbq  %r8, %r12
        cmovbq  %r9, %r13
        cmovbq  %r10, %r14
        cmovbq  %r11, %r15
        cmoveq  0xa0(%rsp), %r12
        cmoveq  0xa8(%rsp), %r13
        cmoveq  0xb0(%rsp), %r14
        cmoveq  0xb8(%rsp), %r15
        movq    (%rsp), %rax
        cmovbq  (%rsi), %rax
        cmova   0x0(%rbp), %rax
        movq    0x8(%rsp), %rbx
        cmovbq  0x8(%rsi), %rbx
        cmova   0x8(%rbp), %rbx
        movq    0x10(%rsp), %rcx
        cmovbq  0x10(%rsi), %rcx
        cmova   0x10(%rbp), %rcx
        movq    0x18(%rsp), %rdx
        cmovbq  0x18(%rsi), %rdx
        cmova   0x18(%rbp), %rdx
        movq    0x80(%rsp), %r8
        cmovbq  0x20(%rsi), %r8
        cmova   0x20(%rbp), %r8
        movq    0x88(%rsp), %r9
        cmovbq  0x28(%rsi), %r9
        cmova   0x28(%rbp), %r9
        movq    0x90(%rsp), %r10
        cmovbq  0x30(%rsi), %r10
        cmova   0x30(%rbp), %r10
        movq    0x98(%rsp), %r11
        cmovbq  0x38(%rsi), %r11
        cmova   0x38(%rbp), %r11
        movq    %rax, (%rdi)
        movq    %rbx, 0x8(%rdi)
        movq    %rcx, 0x10(%rdi)
        movq    %rdx, 0x18(%rdi)
        movq    %r8, 0x20(%rdi)
        movq    %r9, 0x28(%rdi)
        movq    %r10, 0x30(%rdi)
        movq    %r11, 0x38(%rdi)
        movq    %r12, 0x40(%rdi)
        movq    %r13, 0x48(%rdi)
        movq    %r14, 0x50(%rdi)
        movq    %r15, 0x58(%rdi)
        CFI_INC_RSP(224)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

Lp256_scalarmul_alt_local_p256_montjdouble:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(192)
        movq    0x40(%rsi), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x48(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x58(%rsi), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x50(%rsi), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x40(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x58(%rsi), %rbx
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x48(%rsi), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x50(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x58(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        movq    0x20(%rsi), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x28(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x38(%rsi), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x30(%rsi), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x20(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x28(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x38(%rsi), %rbx
        movq    0x28(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x28(%rsi), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x30(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x38(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x20(%rsp)
        movq    %r13, 0x28(%rsp)
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        movq    (%rsi), %rax
        subq    (%rsp), %rax
        movq    0x8(%rsi), %rcx
        sbbq    0x8(%rsp), %rcx
        movq    0x10(%rsi), %r8
        sbbq    0x10(%rsp), %r8
        movq    0x18(%rsi), %r9
        sbbq    0x18(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x60(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x68(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x70(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x78(%rsp)
        movq    (%rsi), %rax
        addq    (%rsp), %rax
        movq    0x8(%rsi), %rcx
        adcq    0x8(%rsp), %rcx
        movq    0x10(%rsi), %r8
        adcq    0x10(%rsp), %r8
        movq    0x18(%rsi), %r9
        adcq    0x18(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        subq    %r11, %rax
        movq    %rax, 0x40(%rsp)
        sbbq    %r10, %rcx
        movq    %rcx, 0x48(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x50(%rsp)
        sbbq    %rdx, %r9
        movq    %r9, 0x58(%rsp)
        movq    0x60(%rsp), %rbx
        movq    0x40(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x48(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x50(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x58(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x68(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    0x40(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x70(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    0x40(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x78(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    0x40(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x60(%rsp)
        movq    %r13, 0x68(%rsp)
        movq    %r14, 0x70(%rsp)
        movq    %r15, 0x78(%rsp)
        xorq    %r11, %r11
        movq    0x20(%rsi), %rax
        addq    0x40(%rsi), %rax
        movq    0x28(%rsi), %rcx
        adcq    0x48(%rsi), %rcx
        movq    0x30(%rsi), %r8
        adcq    0x50(%rsi), %r8
        movq    0x38(%rsi), %r9
        adcq    0x58(%rsi), %r9
        adcq    %r11, %r11
        subq    $0xffffffffffffffff, %rax
        movl    $0xffffffff, %r10d
        sbbq    %r10, %rcx
        sbbq    $0x0, %r8
        movabsq $0xffffffff00000001, %rdx
        sbbq    %rdx, %r9
        sbbq    $0x0, %r11
        andq    %r11, %r10
        andq    %r11, %rdx
        addq    %r11, %rax
        movq    %rax, 0x40(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x48(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x50(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x58(%rsp)
        movq    0x20(%rsp), %rbx
        movq    (%rsi), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x8(%rsi), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x10(%rsi), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x18(%rsi), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x28(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x30(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x38(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x80(%rsp)
        movq    %r13, 0x88(%rsp)
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    0x60(%rsp), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x68(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x78(%rsp), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x70(%rsp), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x78(%rsp), %rbx
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x68(%rsp), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x70(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x78(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xa0(%rsp)
        movq    %r13, 0xa8(%rsp)
        movq    %r14, 0xb0(%rsp)
        movq    %r15, 0xb8(%rsp)
        movq    0x40(%rsp), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x48(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x58(%rsp), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x50(%rsp), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x40(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x58(%rsp), %rbx
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x48(%rsp), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x50(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x58(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x40(%rsp)
        movq    %r13, 0x48(%rsp)
        movq    %r14, 0x50(%rsp)
        movq    %r15, 0x58(%rsp)
        movq    $0xffffffffffffffff, %r9
        xorl    %r11d, %r11d
        subq    0xa0(%rsp), %r9
        movabsq $0xffffffff, %r10
        sbbq    0xa8(%rsp), %r10
        sbbq    0xb0(%rsp), %r11
        movabsq $0xffffffff00000001, %r12
        sbbq    0xb8(%rsp), %r12
        movq    $0x9, %rcx
        movq    %r9, %rax
        mulq    %rcx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    %r10, %rax
        xorl    %r10d, %r10d
        mulq    %rcx
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    %r11, %rax
        xorl    %r11d, %r11d
        mulq    %rcx
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    %r12, %rax
        xorl    %r12d, %r12d
        mulq    %rcx
        addq    %rax, %r11
        adcq    %rdx, %r12
        movl    $0xc, %ecx
        movq    0x80(%rsp), %rax
        mulq    %rcx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %rbx, %rbx
        movq    0x88(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rbx, %rbx
        movq    0x90(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbx, %rbx
        movq    0x98(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        leaq    0x1(%r12), %rcx
        movabsq $0xffffffff00000001, %rax
        mulq    %rcx
        movq    %rcx, %rbx
        shlq    $0x20, %rbx
        addq    %rcx, %r8
        sbbq    $0x0, %rbx
        subq    %rbx, %r9
        sbbq    $0x0, %r10
        sbbq    %rax, %r11
        sbbq    %rdx, %rcx
        decq    %rcx
        movl    $0xffffffff, %eax
        andq    %rcx, %rax
        xorl    %edx, %edx
        subq    %rax, %rdx
        addq    %rcx, %r8
        movq    %r8, 0xa0(%rsp)
        adcq    %rax, %r9
        movq    %r9, 0xa8(%rsp)
        adcq    $0x0, %r10
        movq    %r10, 0xb0(%rsp)
        adcq    %rdx, %r11
        movq    %r11, 0xb8(%rsp)
        movq    0x40(%rsp), %rax
        subq    (%rsp), %rax
        movq    0x48(%rsp), %rcx
        sbbq    0x8(%rsp), %rcx
        movq    0x50(%rsp), %r8
        sbbq    0x10(%rsp), %r8
        movq    0x58(%rsp), %r9
        sbbq    0x18(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x40(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x48(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x50(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x58(%rsp)
        movq    0x20(%rsp), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x28(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x38(%rsp), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x30(%rsp), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x20(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x38(%rsp), %rbx
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x28(%rsp), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x30(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x38(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        movq    0x60(%rsp), %rbx
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x68(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x70(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x78(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x60(%rsp)
        movq    %r13, 0x68(%rsp)
        movq    %r14, 0x70(%rsp)
        movq    %r15, 0x78(%rsp)
        movq    0x40(%rsp), %rax
        subq    0x20(%rsp), %rax
        movq    0x48(%rsp), %rcx
        sbbq    0x28(%rsp), %rcx
        movq    0x50(%rsp), %r8
        sbbq    0x30(%rsp), %r8
        movq    0x58(%rsp), %r9
        sbbq    0x38(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x40(%rdi)
        adcq    %r10, %rcx
        movq    %rcx, 0x48(%rdi)
        adcq    $0x0, %r8
        movq    %r8, 0x50(%rdi)
        adcq    %rdx, %r9
        movq    %r9, 0x58(%rdi)
        movq    0x98(%rsp), %r11
        movq    %r11, %rcx
        movq    0x90(%rsp), %r10
        shldq   $0x2, %r10, %r11
        movq    0x88(%rsp), %r9
        shldq   $0x2, %r9, %r10
        movq    0x80(%rsp), %r8
        shldq   $0x2, %r8, %r9
        shlq    $0x2, %r8
        shrq    $0x3e, %rcx
        addq    $0x1, %rcx
        subq    0xa0(%rsp), %r8
        sbbq    0xa8(%rsp), %r9
        sbbq    0xb0(%rsp), %r10
        sbbq    0xb8(%rsp), %r11
        sbbq    $0x0, %rcx
        movabsq $0xffffffff00000001, %rax
        mulq    %rcx
        movq    %rcx, %rbx
        shlq    $0x20, %rbx
        addq    %rcx, %r8
        sbbq    $0x0, %rbx
        subq    %rbx, %r9
        sbbq    $0x0, %r10
        sbbq    %rax, %r11
        sbbq    %rdx, %rcx
        decq    %rcx
        movl    $0xffffffff, %eax
        andq    %rcx, %rax
        xorl    %edx, %edx
        subq    %rax, %rdx
        addq    %rcx, %r8
        movq    %r8, (%rdi)
        adcq    %rax, %r9
        movq    %r9, 0x8(%rdi)
        adcq    $0x0, %r10
        movq    %r10, 0x10(%rdi)
        adcq    %rdx, %r11
        movq    %r11, 0x18(%rdi)
        movq    $0xffffffffffffffff, %r8
        xorl    %r10d, %r10d
        subq    (%rsp), %r8
        movabsq $0xffffffff, %r9
        sbbq    0x8(%rsp), %r9
        sbbq    0x10(%rsp), %r10
        movabsq $0xffffffff00000001, %r11
        sbbq    0x18(%rsp), %r11
        movq    %r11, %r12
        shldq   $0x3, %r10, %r11
        shldq   $0x3, %r9, %r10
        shldq   $0x3, %r8, %r9
        shlq    $0x3, %r8
        shrq    $0x3d, %r12
        movl    $0x3, %ecx
        movq    0x60(%rsp), %rax
        mulq    %rcx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %rbx, %rbx
        movq    0x68(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rbx, %rbx
        movq    0x70(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbx, %rbx
        movq    0x78(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        leaq    0x1(%r12), %rcx
        movabsq $0xffffffff00000001, %rax
        mulq    %rcx
        movq    %rcx, %rbx
        shlq    $0x20, %rbx
        addq    %rcx, %r8
        sbbq    $0x0, %rbx
        subq    %rbx, %r9
        sbbq    $0x0, %r10
        sbbq    %rax, %r11
        sbbq    %rdx, %rcx
        decq    %rcx
        movl    $0xffffffff, %eax
        andq    %rcx, %rax
        xorl    %edx, %edx
        subq    %rax, %rdx
        addq    %rcx, %r8
        movq    %r8, 0x20(%rdi)
        adcq    %rax, %r9
        movq    %r9, 0x28(%rdi)
        adcq    $0x0, %r10
        movq    %r10, 0x30(%rdi)
        adcq    %rdx, %r11
        movq    %r11, 0x38(%rdi)
        CFI_INC_RSP(192)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
        CFI_RET

Lp256_scalarmul_alt_local_p256_montjmixadd:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(192)
        movq    %rdx, %rbp
        movq    0x40(%rsi), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x48(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x58(%rsi), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x50(%rsi), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x40(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x58(%rsi), %rbx
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x48(%rsi), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x50(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x58(%rsi), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        movq    0x20(%rbp), %rbx
        movq    0x40(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x48(%rsi), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x50(%rsi), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x58(%rsi), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x28(%rbp), %rbx
        xorl    %r13d, %r13d
        movq    0x40(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x50(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x58(%rsi), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x30(%rbp), %rbx
        xorl    %r15d, %r15d
        movq    0x40(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x50(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x58(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x38(%rbp), %rbx
        xorl    %r8d, %r8d
        movq    0x40(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x50(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x58(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x20(%rsp)
        movq    %r13, 0x28(%rsp)
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        movq    0x0(%rbp), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x10(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x18(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x8(%rbp), %rbx
        xorl    %r13d, %r13d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x10(%rbp), %rbx
        xorl    %r15d, %r15d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x18(%rbp), %rbx
        xorl    %r8d, %r8d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x40(%rsp)
        movq    %r13, 0x48(%rsp)
        movq    %r14, 0x50(%rsp)
        movq    %r15, 0x58(%rsp)
        movq    0x20(%rsp), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x10(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x18(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x28(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x30(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x38(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x20(%rsp)
        movq    %r13, 0x28(%rsp)
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        movq    0x40(%rsp), %rax
        subq    (%rsi), %rax
        movq    0x48(%rsp), %rcx
        sbbq    0x8(%rsi), %rcx
        movq    0x50(%rsp), %r8
        sbbq    0x10(%rsi), %r8
        movq    0x58(%rsp), %r9
        sbbq    0x18(%rsi), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0xa0(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0xa8(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0xb0(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0xb8(%rsp)
        movq    0x20(%rsp), %rax
        subq    0x20(%rsi), %rax
        movq    0x28(%rsp), %rcx
        sbbq    0x28(%rsi), %rcx
        movq    0x30(%rsp), %r8
        sbbq    0x30(%rsi), %r8
        movq    0x38(%rsp), %r9
        sbbq    0x38(%rsi), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x20(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x28(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x30(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x38(%rsp)
        movq    0xa0(%rsp), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0xb8(%rsp), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0xb0(%rsp), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0xb8(%rsp), %rbx
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0xa8(%rsp), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0xb0(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0xb8(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x60(%rsp)
        movq    %r13, 0x68(%rsp)
        movq    %r14, 0x70(%rsp)
        movq    %r15, 0x78(%rsp)
        movq    0x20(%rsp), %rax
        movq    %rax, %rbx
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r15
        movq    0x28(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x38(%rsp), %rax
        movq    %rax, %r13
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x30(%rsp), %rax
        movq    %rax, %rbx
        mulq    %r13
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x20(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    0x38(%rsp), %rbx
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorl    %ecx, %ecx
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %rcx, %rcx
        movq    0x28(%rsp), %rax
        mulq    %rax
        addq    %r15, %r9
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        movq    0x30(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r15, %r15
        movq    0x38(%rsp), %rax
        mulq    %rax
        negq    %r15
        adcq    %rax, %r14
        adcq    %rcx, %rdx
        movq    %rdx, %r15
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        xorl    %r8d, %r8d
        movq    %r9, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r8, %r14
        adcq    %r8, %r15
        adcq    %r8, %r8
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        xorl    %r9d, %r9d
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rbx), %rbx
        adcq    %r13, %rbx
        leaq    -0x1(%r9), %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        movq    (%rsi), %rbx
        movq    0x60(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x70(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x78(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x8(%rsi), %rbx
        xorl    %r13d, %r13d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x10(%rsi), %rbx
        xorl    %r15d, %r15d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x18(%rsi), %rbx
        xorl    %r8d, %r8d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x80(%rsp)
        movq    %r13, 0x88(%rsp)
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    0x40(%rsp), %rbx
        movq    0x60(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x70(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x78(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x48(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x50(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x58(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x40(%rsp)
        movq    %r13, 0x48(%rsp)
        movq    %r14, 0x50(%rsp)
        movq    %r15, 0x58(%rsp)
        movq    (%rsp), %rax
        subq    0x80(%rsp), %rax
        movq    0x8(%rsp), %rcx
        sbbq    0x88(%rsp), %rcx
        movq    0x10(%rsp), %r8
        sbbq    0x90(%rsp), %r8
        movq    0x18(%rsp), %r9
        sbbq    0x98(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, (%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x8(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x10(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x18(%rsp)
        movq    0x40(%rsp), %rax
        subq    0x80(%rsp), %rax
        movq    0x48(%rsp), %rcx
        sbbq    0x88(%rsp), %rcx
        movq    0x50(%rsp), %r8
        sbbq    0x90(%rsp), %r8
        movq    0x58(%rsp), %r9
        sbbq    0x98(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x60(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x68(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x70(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x78(%rsp)
        movq    0x40(%rsi), %rbx
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x48(%rsi), %rbx
        xorl    %r13d, %r13d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x50(%rsi), %rbx
        xorl    %r15d, %r15d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x58(%rsi), %rbx
        xorl    %r8d, %r8d
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xa0(%rsp)
        movq    %r13, 0xa8(%rsp)
        movq    %r14, 0xb0(%rsp)
        movq    %r15, 0xb8(%rsp)
        movq    (%rsp), %rax
        subq    0x40(%rsp), %rax
        movq    0x8(%rsp), %rcx
        sbbq    0x48(%rsp), %rcx
        movq    0x10(%rsp), %r8
        sbbq    0x50(%rsp), %r8
        movq    0x18(%rsp), %r9
        sbbq    0x58(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, (%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x8(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x10(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x18(%rsp)
        movq    0x80(%rsp), %rax
        subq    (%rsp), %rax
        movq    0x88(%rsp), %rcx
        sbbq    0x8(%rsp), %rcx
        movq    0x90(%rsp), %r8
        sbbq    0x10(%rsp), %r8
        movq    0x98(%rsp), %r9
        sbbq    0x18(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x80(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x88(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x90(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x98(%rsp)
        movq    0x20(%rsi), %rbx
        movq    0x60(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x70(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x78(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x28(%rsi), %rbx
        xorl    %r13d, %r13d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x30(%rsi), %rbx
        xorl    %r15d, %r15d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x38(%rsi), %rbx
        xorl    %r8d, %r8d
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x70(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x60(%rsp)
        movq    %r13, 0x68(%rsp)
        movq    %r14, 0x70(%rsp)
        movq    %r15, 0x78(%rsp)
        movq    0x80(%rsp), %rbx
        movq    0x20(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x28(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x30(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x38(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x88(%rsp), %rbx
        xorl    %r13d, %r13d
        movq    0x20(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r14, %r14
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r14, %r14
        movq    0x30(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r14, %r14
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r14, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        xorl    %r14d, %r14d
        movabsq $0x100000000, %rbx
        movq    %r8, %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r15, %r15
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r8, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r15, %r15
        movq    %r9, %rax
        mulq    %rbx
        subq    %r15, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x90(%rsp), %rbx
        xorl    %r15d, %r15d
        movq    0x20(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x30(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x98(%rsp), %rbx
        xorl    %r8d, %r8d
        movq    0x20(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x30(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r8, %r8
        xorl    %r9d, %r9d
        movabsq $0x100000000, %rbx
        movq    %r10, %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rcx, %rcx
        notq    %rbx
        leaq    0x2(%rbx), %rbx
        movq    %r10, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rcx, %rcx
        movq    %r11, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rbx
        adcq    %r13, %rbx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rbx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x80(%rsp)
        movq    %r13, 0x88(%rsp)
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    0x80(%rsp), %rax
        subq    0x60(%rsp), %rax
        movq    0x88(%rsp), %rcx
        sbbq    0x68(%rsp), %rcx
        movq    0x90(%rsp), %r8
        sbbq    0x70(%rsp), %r8
        movq    0x98(%rsp), %r9
        sbbq    0x78(%rsp), %r9
        movl    $0xffffffff, %r10d
        sbbq    %r11, %r11
        xorq    %rdx, %rdx
        andq    %r11, %r10
        subq    %r10, %rdx
        addq    %r11, %rax
        movq    %rax, 0x80(%rsp)
        adcq    %r10, %rcx
        movq    %rcx, 0x88(%rsp)
        adcq    $0x0, %r8
        movq    %r8, 0x90(%rsp)
        adcq    %rdx, %r9
        movq    %r9, 0x98(%rsp)
        movq    0x40(%rsi), %rax
        movq    0x48(%rsi), %rdx
        orq     0x50(%rsi), %rax
        orq     0x58(%rsi), %rdx
        orq     %rdx, %rax
        movq    (%rsp), %r8
        movq    0x0(%rbp), %rax
        cmoveq  %rax, %r8
        movq    0x8(%rsp), %r9
        movq    0x8(%rbp), %rax
        cmoveq  %rax, %r9
        movq    0x10(%rsp), %r10
        movq    0x10(%rbp), %rax
        cmoveq  %rax, %r10
        movq    0x18(%rsp), %r11
        movq    0x18(%rbp), %rax
        cmoveq  %rax, %r11
        movq    0x80(%rsp), %r12
        movq    0x20(%rbp), %rax
        cmoveq  %rax, %r12
        movq    0x88(%rsp), %r13
        movq    0x28(%rbp), %rax
        cmoveq  %rax, %r13
        movq    0x90(%rsp), %r14
        movq    0x30(%rbp), %rax
        cmoveq  %rax, %r14
        movq    0x98(%rsp), %r15
        movq    0x38(%rbp), %rax
        cmoveq  %rax, %r15
        movq    %r8, (%rdi)
        movq    %r9, 0x8(%rdi)
        movq    %r10, 0x10(%rdi)
        movq    %r11, 0x18(%rdi)
        movq    %r12, 0x20(%rdi)
        movq    %r13, 0x28(%rdi)
        movq    %r14, 0x30(%rdi)
        movq    %r15, 0x38(%rdi)
        movq    0xa0(%rsp), %r8
        movq    0xa8(%rsp), %r9
        movq    0xb0(%rsp), %r10
        movq    0xb8(%rsp), %r11
        movl    $0x1, %eax
        cmoveq  %rax, %r8
        movabsq $0xffffffff00000000, %rax
        cmoveq  %rax, %r9
        movq    $0xffffffffffffffff, %rax
        cmoveq  %rax, %r10
        movl    $0xfffffffe, %eax
        cmoveq  %rax, %r11
        movq    %r8, 0x40(%rdi)
        movq    %r9, 0x48(%rdi)
        movq    %r10, 0x50(%rdi)
        movq    %r11, 0x58(%rdi)
        CFI_INC_RSP(192)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
