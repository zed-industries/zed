// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery-Jacobian form scalar multiplication for P-256
// Input scalar[4], point[12]; output res[12]
//
// extern void p256_montjscalarmul
//   (uint64_t res[static 12],
//    const uint64_t scalar[static 4],
//    const uint64_t point[static 12]);
//
// This function is a variant of its affine point version p256_scalarmul.
// Here, input and output points are assumed to be in Jacobian form with
// their coordinates in the Montgomery domain. Thus, if priming indicates
// Montgomery form, x' = (2^256 * x) mod p_256 etc., each point argument
// is a triple (x',y',z') representing the affine point (x/z^2,y/z^3) when
// z' is nonzero or the point at infinity (group identity) if z' = 0.
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve P-256, returns a representation of n * P. If the result is the
// point at infinity (either because the input point was or because the
// scalar was a multiple of p_256) then the output is guaranteed to
// represent the point at infinity, i.e. to have its z coordinate zero.
//
// Standard x86-64 ABI: RDI = res, RSI = scalar, RDX = point
// Microsoft x64 ABI:   RCX = res, RDX = scalar, R8 = point
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjscalarmul)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_montjscalarmul)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjscalarmul)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 32

// Intermediate variables on the stack. Uppercase syntactic variants
// make x86_att version simpler to generate.

#define SCALARB (0*NUMSIZE)
#define scalarb (0*NUMSIZE)(%rsp)
#define ACC (1*NUMSIZE)
#define acc (1*NUMSIZE)(%rsp)
#define TABENT (4*NUMSIZE)
#define tabent (4*NUMSIZE)(%rsp)

#define TAB (7*NUMSIZE)
#define tab (7*NUMSIZE)(%rsp)

#define res (31*NUMSIZE)(%rsp)

#define NSPACE 32*NUMSIZE

// Avoid using .rep for the sake of the BoringSSL/AWS-LC delocator,
// which doesn't accept repetitions, assembler macros etc.

#define selectblock(I)                          \
        cmpq    $I, %rdi ;                         \
        cmovzq  TAB+96*(I-1)(%rsp), %rax ;        \
        cmovzq  TAB+96*(I-1)+8(%rsp), %rbx ;      \
        cmovzq  TAB+96*(I-1)+16(%rsp), %rcx ;     \
        cmovzq  TAB+96*(I-1)+24(%rsp), %rdx ;     \
        cmovzq  TAB+96*(I-1)+32(%rsp), %r8 ;      \
        cmovzq  TAB+96*(I-1)+40(%rsp), %r9 ;      \
        cmovzq  TAB+96*(I-1)+48(%rsp), %r10 ;     \
        cmovzq  TAB+96*(I-1)+56(%rsp), %r11 ;     \
        cmovzq  TAB+96*(I-1)+64(%rsp), %r12 ;     \
        cmovzq  TAB+96*(I-1)+72(%rsp), %r13 ;     \
        cmovzq  TAB+96*(I-1)+80(%rsp), %r14 ;     \
        cmovzq  TAB+96*(I-1)+88(%rsp), %r15

S2N_BN_SYMBOL(p256_montjscalarmul):
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
        CFI_CALL(Lp256_montjscalarmul_standard)
        CFI_POP(%rsi)
        CFI_POP(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p256_montjscalarmul)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_montjscalarmul_standard)

Lp256_montjscalarmul_standard:
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

// Set the tab[0] table entry to the input point = 1 * P, except
// that we negate it if the top bit of the scalar was set. This
// negation takes care over the y = 0 case to maintain all the
// coordinates < p_256 throughout, even though triples (x,y,z)
// with y = 0 can only represent a point on the curve when z = 0
// and it represents the point at infinity regardless of x and y.

        movq    (%rbx), %rax
        movq    %rax, TAB(%rsp)
        movq    8(%rbx), %rax
        movq    %rax, TAB+8(%rsp)
        movq    16(%rbx), %rax
        movq    %rax, TAB+16(%rsp)
        movq    24(%rbx), %rax
        movq    %rax, TAB+24(%rsp)

        movq    32(%rbx), %r12
        movq    %r12, %rax
        movq    40(%rbx), %r13
        orq     %r13, %rax
        movq    48(%rbx), %r14
        movq    %r14, %rcx
        movq    56(%rbx), %r15
        orq     %r15, %rcx
        orq     %rcx, %rax
        cmovzq  %rax, %rbp

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

        movq    64(%rbx), %rax
        movq    %rax, TAB+64(%rsp)
        movq    72(%rbx), %rax
        movq    %rax, TAB+72(%rsp)
        movq    80(%rbx), %rax
        movq    %rax, TAB+80(%rsp)
        movq    88(%rbx), %rax
        movq    %rax, TAB+88(%rsp)

// Compute and record tab[1] = 2 * p, ..., tab[7] = 8 * P

        leaq    TAB+96*1(%rsp), %rdi
        leaq    TAB(%rsp), %rsi
        CFI_CALL(Lp256_montjscalarmul_p256_montjdouble)

        leaq    TAB+96*2(%rsp), %rdi
        leaq    TAB+96*1(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp256_montjscalarmul_p256_montjadd)

        leaq    TAB+96*3(%rsp), %rdi
        leaq    TAB+96*1(%rsp), %rsi
        CFI_CALL(Lp256_montjscalarmul_p256_montjdouble)

        leaq    TAB+96*4(%rsp), %rdi
        leaq    TAB+96*3(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp256_montjscalarmul_p256_montjadd)

        leaq    TAB+96*5(%rsp), %rdi
        leaq    TAB+96*2(%rsp), %rsi
        CFI_CALL(Lp256_montjscalarmul_p256_montjdouble)

        leaq    TAB+96*6(%rsp), %rdi
        leaq    TAB+96*5(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp256_montjscalarmul_p256_montjadd)

        leaq    TAB+96*7(%rsp), %rdi
        leaq    TAB+96*3(%rsp), %rsi
        CFI_CALL(Lp256_montjscalarmul_p256_montjdouble)

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

        selectblock(1)
        selectblock(2)
        selectblock(3)
        selectblock(4)
        selectblock(5)
        selectblock(6)
        selectblock(7)
        selectblock(8)

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

Lp256_montjscalarmul_mainloop:
        subq    $4, %rbp

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_montjscalarmul_p256_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_montjscalarmul_p256_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_montjscalarmul_p256_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_montjscalarmul_p256_montjdouble)

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

        selectblock(1)
        selectblock(2)
        selectblock(3)
        selectblock(4)
        selectblock(5)
        selectblock(6)
        selectblock(7)
        selectblock(8)

// Store it to "tabent" with the y coordinate optionally negated
// Again, do it carefully to give coordinates < p_256 even in
// the degenerate case y = 0 (when z = 0 for points on the curve).

        movq     %rax, TABENT(%rsp)
        movq     %rbx, TABENT+8(%rsp)
        movq     %rcx, TABENT+16(%rsp)
        movq     %rdx, TABENT+24(%rsp)

        movq     %r12, TABENT+64(%rsp)
        movq     %r13, TABENT+72(%rsp)
        movq     %r14, TABENT+80(%rsp)
        movq     %r15, TABENT+88(%rsp)

        movq    %r8, %rax
        xorl    %r14d, %r14d
        orq     %r9, %rax
        leaq    -1(%r14), %r12
        movq    %r10, %rcx
        movq    $0x00000000ffffffff, %r15
        orq     %r11, %rcx
        movq    %r15, %r13
        negq    %r15
        orq     %rcx, %rax
        cmovzq  %rax, %rsi

        subq    %r8, %r12
        sbbq    %r9, %r13
        sbbq    %r10, %r14
        sbbq    %r11, %r15

        testq   %rsi, %rsi
        cmovnzq  %r12, %r8
        cmovnzq  %r13, %r9
        cmovnzq  %r14, %r10
        cmovnzq  %r15, %r11

        movq     %r8, TABENT+32(%rsp)
        movq     %r9, TABENT+40(%rsp)
        movq     %r10, TABENT+48(%rsp)
        movq     %r11, TABENT+56(%rsp)

        leaq    TABENT(%rsp), %rdx
        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp256_montjscalarmul_p256_montjadd)

        testq   %rbp, %rbp
        jne     Lp256_montjscalarmul_mainloop

// That's the end of the main loop, and we just need to copy the
// result in "acc" to the output.

        movq    res, %rdi
        movq    ACC(%rsp), %rax
        movq    %rax, (%rdi)
        movq    ACC+8(%rsp), %rax
        movq    %rax, 8(%rdi)
        movq    ACC+16(%rsp), %rax
        movq    %rax, 16(%rdi)
        movq    ACC+24(%rsp), %rax
        movq    %rax, 24(%rdi)

        movq    ACC+32(%rsp), %rax
        movq    %rax, 32(%rdi)
        movq    ACC+40(%rsp), %rax
        movq    %rax, 40(%rdi)
        movq    ACC+48(%rsp), %rax
        movq    %rax, 48(%rdi)
        movq    ACC+56(%rsp), %rax
        movq    %rax, 56(%rdi)

        movq    ACC+64(%rsp), %rax
        movq    %rax, 64(%rdi)
        movq    ACC+72(%rsp), %rax
        movq    %rax, 72(%rdi)
        movq    ACC+80(%rsp), %rax
        movq    %rax, 80(%rdi)
        movq    ACC+88(%rsp), %rax
        movq    %rax, 88(%rdi)

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
S2N_BN_SIZE_DIRECTIVE(Lp256_montjscalarmul_standard)
#else
S2N_BN_SIZE_DIRECTIVE(p256_montjscalarmul)
#endif

// Local copies of subroutines, complete clones at the moment

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_montjscalarmul_p256_montjadd)

Lp256_montjscalarmul_p256_montjadd:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(224)
        movq    %rdx, %rbp
        movq    0x40(%rsi), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   0x48(%rsi), %r9, %r10
        mulxq   0x58(%rsi), %r11, %r12
        movq    0x50(%rsi), %rdx
        mulxq   0x58(%rsi), %r13, %r14
        xorl    %ecx, %ecx
        mulxq   0x40(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x48(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    0x58(%rsi), %rdx
        mulxq   0x48(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rcx, %r13
        adoxq   %rcx, %r14
        adcq    %rcx, %r14
        xorl    %ecx, %ecx
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    0x48(%rsi), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    0x50(%rsi), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    0x58(%rsi), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %rcx, %r15
        adoxq   %rcx, %r15
        xorl    %ecx, %ecx
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    $0xffffffff00000001, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rcx, %r13
        movl    %ecx, %r9d
        adoxq   %rcx, %r9
        adcxq   %rcx, %r9
        addq    %r9, %r14
        adcq    %rcx, %r15
        movl    %ecx, %r8d
        adcq    %rcx, %r8
        xorl    %ecx, %ecx
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    $0xffffffff00000001, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %rcx, %r15
        adoxq   %rcx, %r8
        adcq    %rcx, %r8
        movl    $0x1, %r8d
        leaq    -0x1(%rdx), %rdx
        leaq    -0x1(%rcx), %rax
        movl    $0xfffffffe, %r11d
        cmoveq  %rcx, %r8
        cmoveq  %rcx, %rdx
        cmoveq  %rcx, %rax
        cmoveq  %rcx, %r11
        addq    %r8, %r12
        adcq    %rdx, %r13
        adcq    %rax, %r14
        adcq    %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        movq    0x40(%rbp), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   0x48(%rbp), %r9, %r10
        mulxq   0x58(%rbp), %r11, %r12
        movq    0x50(%rbp), %rdx
        mulxq   0x58(%rbp), %r13, %r14
        xorl    %ecx, %ecx
        mulxq   0x40(%rbp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x48(%rbp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    0x58(%rbp), %rdx
        mulxq   0x48(%rbp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rcx, %r13
        adoxq   %rcx, %r14
        adcq    %rcx, %r14
        xorl    %ecx, %ecx
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    0x48(%rbp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    0x50(%rbp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    0x58(%rbp), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %rcx, %r15
        adoxq   %rcx, %r15
        xorl    %ecx, %ecx
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    $0xffffffff00000001, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rcx, %r13
        movl    %ecx, %r9d
        adoxq   %rcx, %r9
        adcxq   %rcx, %r9
        addq    %r9, %r14
        adcq    %rcx, %r15
        movl    %ecx, %r8d
        adcq    %rcx, %r8
        xorl    %ecx, %ecx
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    $0xffffffff00000001, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %rcx, %r15
        adoxq   %rcx, %r8
        adcq    %rcx, %r8
        movl    $0x1, %r8d
        leaq    -0x1(%rdx), %rdx
        leaq    -0x1(%rcx), %rax
        movl    $0xfffffffe, %r11d
        cmoveq  %rcx, %r8
        cmoveq  %rcx, %rdx
        cmoveq  %rcx, %rax
        cmoveq  %rcx, %r11
        addq    %r8, %r12
        adcq    %rdx, %r13
        adcq    %rax, %r14
        adcq    %r11, %r15
        movq    %r12, 0xa0(%rsp)
        movq    %r13, 0xa8(%rsp)
        movq    %r14, 0xb0(%rsp)
        movq    %r15, 0xb8(%rsp)
        xorl    %r13d, %r13d
        movq    0x20(%rsi), %rdx
        mulxq   0x40(%rbp), %r8, %r9
        mulxq   0x48(%rbp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x50(%rbp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x58(%rbp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x28(%rsi), %rdx
        xorl    %r14d, %r14d
        mulxq   0x40(%rbp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x48(%rbp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x50(%rbp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x58(%rbp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x30(%rsi), %rdx
        xorl    %r8d, %r8d
        mulxq   0x40(%rbp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x48(%rbp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x50(%rbp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x58(%rbp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x38(%rsi), %rdx
        xorl    %r9d, %r9d
        mulxq   0x40(%rbp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x48(%rbp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x50(%rbp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x58(%rbp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xc0(%rsp)
        movq    %r13, 0xc8(%rsp)
        movq    %r14, 0xd0(%rsp)
        movq    %r15, 0xd8(%rsp)
        xorl    %r13d, %r13d
        movq    0x20(%rbp), %rdx
        mulxq   0x40(%rsi), %r8, %r9
        mulxq   0x48(%rsi), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x50(%rsi), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x58(%rsi), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x28(%rbp), %rdx
        xorl    %r14d, %r14d
        mulxq   0x40(%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x48(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x50(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x58(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x30(%rbp), %rdx
        xorl    %r8d, %r8d
        mulxq   0x40(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x48(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x50(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x58(%rsi), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x38(%rbp), %rdx
        xorl    %r9d, %r9d
        mulxq   0x40(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x48(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x50(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x58(%rsi), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x20(%rsp)
        movq    %r13, 0x28(%rsp)
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        xorl    %r13d, %r13d
        movq    0x0(%rbp), %rdx
        mulxq   (%rsp), %r8, %r9
        mulxq   0x8(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x10(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x18(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x8(%rbp), %rdx
        xorl    %r14d, %r14d
        mulxq   (%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x8(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x10(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x18(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x10(%rbp), %rdx
        xorl    %r8d, %r8d
        mulxq   (%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x8(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x10(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x18(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x18(%rbp), %rdx
        xorl    %r9d, %r9d
        mulxq   (%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x10(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x18(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x40(%rsp)
        movq    %r13, 0x48(%rsp)
        movq    %r14, 0x50(%rsp)
        movq    %r15, 0x58(%rsp)
        xorl    %r13d, %r13d
        movq    (%rsi), %rdx
        mulxq   0xa0(%rsp), %r8, %r9
        mulxq   0xa8(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0xb0(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0xb8(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x8(%rsi), %rdx
        xorl    %r14d, %r14d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x10(%rsi), %rdx
        xorl    %r8d, %r8d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x18(%rsi), %rdx
        xorl    %r9d, %r9d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x80(%rsp)
        movq    %r13, 0x88(%rsp)
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        xorl    %r13d, %r13d
        movq    0x20(%rsp), %rdx
        mulxq   (%rsp), %r8, %r9
        mulxq   0x8(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x10(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x18(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x28(%rsp), %rdx
        xorl    %r14d, %r14d
        mulxq   (%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x8(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x10(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x18(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x30(%rsp), %rdx
        xorl    %r8d, %r8d
        mulxq   (%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x8(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x10(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x18(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x38(%rsp), %rdx
        xorl    %r9d, %r9d
        mulxq   (%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x10(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x18(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x20(%rsp)
        movq    %r13, 0x28(%rsp)
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        xorl    %r13d, %r13d
        movq    0xc0(%rsp), %rdx
        mulxq   0xa0(%rsp), %r8, %r9
        mulxq   0xa8(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0xb0(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0xb8(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0xc8(%rsp), %rdx
        xorl    %r14d, %r14d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0xd0(%rsp), %rdx
        xorl    %r8d, %r8d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0xd8(%rsp), %rdx
        xorl    %r9d, %r9d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
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
        movq    0xa0(%rsp), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   0xa8(%rsp), %r9, %r10
        mulxq   0xb8(%rsp), %r11, %r12
        movq    0xb0(%rsp), %rdx
        mulxq   0xb8(%rsp), %r13, %r14
        xorl    %ecx, %ecx
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    0xb8(%rsp), %rdx
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rcx, %r13
        adoxq   %rcx, %r14
        adcq    %rcx, %r14
        xorl    %ecx, %ecx
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    0xa8(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    0xb0(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    0xb8(%rsp), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %rcx, %r15
        adoxq   %rcx, %r15
        xorl    %ecx, %ecx
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    $0xffffffff00000001, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rcx, %r13
        movl    %ecx, %r9d
        adoxq   %rcx, %r9
        adcxq   %rcx, %r9
        addq    %r9, %r14
        adcq    %rcx, %r15
        movl    %ecx, %r8d
        adcq    %rcx, %r8
        xorl    %ecx, %ecx
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    $0xffffffff00000001, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %rcx, %r15
        adoxq   %rcx, %r8
        adcq    %rcx, %r8
        movl    $0x1, %r8d
        leaq    -0x1(%rdx), %rdx
        leaq    -0x1(%rcx), %rax
        movl    $0xfffffffe, %r11d
        cmoveq  %rcx, %r8
        cmoveq  %rcx, %rdx
        cmoveq  %rcx, %rax
        cmoveq  %rcx, %r11
        addq    %r8, %r12
        adcq    %rdx, %r13
        adcq    %rax, %r14
        adcq    %r11, %r15
        movq    %r12, 0x60(%rsp)
        movq    %r13, 0x68(%rsp)
        movq    %r14, 0x70(%rsp)
        movq    %r15, 0x78(%rsp)
        movq    0x20(%rsp), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   0x28(%rsp), %r9, %r10
        mulxq   0x38(%rsp), %r11, %r12
        movq    0x30(%rsp), %rdx
        mulxq   0x38(%rsp), %r13, %r14
        xorl    %ecx, %ecx
        mulxq   0x20(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x28(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    0x38(%rsp), %rdx
        mulxq   0x28(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rcx, %r13
        adoxq   %rcx, %r14
        adcq    %rcx, %r14
        xorl    %ecx, %ecx
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    0x28(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    0x30(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    0x38(%rsp), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %rcx, %r15
        adoxq   %rcx, %r15
        xorl    %ecx, %ecx
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    $0xffffffff00000001, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rcx, %r13
        movl    %ecx, %r9d
        adoxq   %rcx, %r9
        adcxq   %rcx, %r9
        addq    %r9, %r14
        adcq    %rcx, %r15
        movl    %ecx, %r8d
        adcq    %rcx, %r8
        xorl    %ecx, %ecx
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    $0xffffffff00000001, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %rcx, %r15
        adoxq   %rcx, %r8
        adcq    %rcx, %r8
        movl    $0x1, %ebx
        addq    %r12, %rbx
        leaq    -0x1(%rdx), %rdx
        adcq    %r13, %rdx
        leaq    -0x1(%rcx), %rcx
        movq    %rcx, %rax
        adcq    %r14, %rcx
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rbx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %rcx, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        xorl    %r13d, %r13d
        movq    0x80(%rsp), %rdx
        mulxq   0x60(%rsp), %r8, %r9
        mulxq   0x68(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x70(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x78(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x88(%rsp), %rdx
        xorl    %r14d, %r14d
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x70(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x78(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x90(%rsp), %rdx
        xorl    %r8d, %r8d
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x70(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x78(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x98(%rsp), %rdx
        xorl    %r9d, %r9d
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x70(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x78(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x80(%rsp)
        movq    %r13, 0x88(%rsp)
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        xorl    %r13d, %r13d
        movq    0x40(%rsp), %rdx
        mulxq   0x60(%rsp), %r8, %r9
        mulxq   0x68(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x70(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x78(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x48(%rsp), %rdx
        xorl    %r14d, %r14d
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x70(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x78(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x50(%rsp), %rdx
        xorl    %r8d, %r8d
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x70(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x78(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x58(%rsp), %rdx
        xorl    %r9d, %r9d
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x70(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x78(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
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
        xorl    %r13d, %r13d
        movq    0x40(%rsi), %rdx
        mulxq   0xa0(%rsp), %r8, %r9
        mulxq   0xa8(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0xb0(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0xb8(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x48(%rsi), %rdx
        xorl    %r14d, %r14d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x50(%rsi), %rdx
        xorl    %r8d, %r8d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x58(%rsi), %rdx
        xorl    %r9d, %r9d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
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
        xorl    %r13d, %r13d
        movq    0xc0(%rsp), %rdx
        mulxq   0x60(%rsp), %r8, %r9
        mulxq   0x68(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x70(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x78(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0xc8(%rsp), %rdx
        xorl    %r14d, %r14d
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x70(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x78(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0xd0(%rsp), %rdx
        xorl    %r8d, %r8d
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x70(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x78(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0xd8(%rsp), %rdx
        xorl    %r9d, %r9d
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x70(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x78(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x60(%rsp)
        movq    %r13, 0x68(%rsp)
        movq    %r14, 0x70(%rsp)
        movq    %r15, 0x78(%rsp)
        xorl    %r13d, %r13d
        movq    0x40(%rbp), %rdx
        mulxq   0xa0(%rsp), %r8, %r9
        mulxq   0xa8(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0xb0(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0xb8(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x48(%rbp), %rdx
        xorl    %r14d, %r14d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x50(%rbp), %rdx
        xorl    %r8d, %r8d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x58(%rbp), %rdx
        xorl    %r9d, %r9d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xa0(%rsp)
        movq    %r13, 0xa8(%rsp)
        movq    %r14, 0xb0(%rsp)
        movq    %r15, 0xb8(%rsp)
        xorl    %r13d, %r13d
        movq    0x80(%rsp), %rdx
        mulxq   0x20(%rsp), %r8, %r9
        mulxq   0x28(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x30(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x38(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x88(%rsp), %rdx
        xorl    %r14d, %r14d
        mulxq   0x20(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x28(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x30(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x38(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x90(%rsp), %rdx
        xorl    %r8d, %r8d
        mulxq   0x20(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x28(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x30(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x38(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x98(%rsp), %rdx
        xorl    %r9d, %r9d
        mulxq   0x20(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x28(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x30(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x38(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
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

S2N_BN_SIZE_DIRECTIVE(Lp256_montjscalarmul_p256_montjadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp256_montjscalarmul_p256_montjdouble)

Lp256_montjscalarmul_p256_montjdouble:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(192)
        movq    0x40(%rsi), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   0x48(%rsi), %r9, %r10
        mulxq   0x58(%rsi), %r11, %r12
        movq    0x50(%rsi), %rdx
        mulxq   0x58(%rsi), %r13, %r14
        xorl    %ebp, %ebp
        mulxq   0x40(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x48(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    0x58(%rsi), %rdx
        mulxq   0x48(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        adoxq   %rbp, %r14
        adcq    %rbp, %r14
        xorl    %ebp, %ebp
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    0x48(%rsi), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    0x50(%rsi), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    0x58(%rsi), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %rbp, %r15
        adoxq   %rbp, %r15
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    $0xffffffff00000001, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        movl    %ebp, %r9d
        adoxq   %rbp, %r9
        adcxq   %rbp, %r9
        addq    %r9, %r14
        adcq    %rbp, %r15
        movl    %ebp, %r8d
        adcq    %rbp, %r8
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    $0xffffffff00000001, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %rbp, %r15
        adoxq   %rbp, %r8
        adcq    %rbp, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rdx), %rdx
        adcq    %r13, %rdx
        leaq    -0x1(%rbp), %rbp
        movq    %rbp, %rax
        adcq    %r14, %rbp
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %rbp, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        movq    0x20(%rsi), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   0x28(%rsi), %r9, %r10
        mulxq   0x38(%rsi), %r11, %r12
        movq    0x30(%rsi), %rdx
        mulxq   0x38(%rsi), %r13, %r14
        xorl    %ebp, %ebp
        mulxq   0x20(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    0x38(%rsi), %rdx
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        adoxq   %rbp, %r14
        adcq    %rbp, %r14
        xorl    %ebp, %ebp
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    0x28(%rsi), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    0x30(%rsi), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    0x38(%rsi), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %rbp, %r15
        adoxq   %rbp, %r15
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    $0xffffffff00000001, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        movl    %ebp, %r9d
        adoxq   %rbp, %r9
        adcxq   %rbp, %r9
        addq    %r9, %r14
        adcq    %rbp, %r15
        movl    %ebp, %r8d
        adcq    %rbp, %r8
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    $0xffffffff00000001, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %rbp, %r15
        adoxq   %rbp, %r8
        adcq    %rbp, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rdx), %rdx
        adcq    %r13, %rdx
        leaq    -0x1(%rbp), %rbp
        movq    %rbp, %rax
        adcq    %r14, %rbp
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %rbp, %r14
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
        xorl    %r13d, %r13d
        movq    0x60(%rsp), %rdx
        mulxq   0x40(%rsp), %r8, %r9
        mulxq   0x48(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x50(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x58(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x68(%rsp), %rdx
        xorl    %r14d, %r14d
        mulxq   0x40(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x48(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x50(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x58(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x70(%rsp), %rdx
        xorl    %r8d, %r8d
        mulxq   0x40(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x48(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x50(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x58(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x78(%rsp), %rdx
        xorl    %r9d, %r9d
        mulxq   0x40(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x48(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x50(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x58(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
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
        movq    $0xffffffff00000001, %rdx
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
        xorl    %r13d, %r13d
        movq    0x20(%rsp), %rdx
        mulxq   (%rsi), %r8, %r9
        mulxq   0x8(%rsi), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x10(%rsi), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x18(%rsi), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x28(%rsp), %rdx
        xorl    %r14d, %r14d
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x18(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x30(%rsp), %rdx
        xorl    %r8d, %r8d
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0x18(%rsi), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x38(%rsp), %rdx
        xorl    %r9d, %r9d
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0x18(%rsi), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %r9, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x80(%rsp)
        movq    %r13, 0x88(%rsp)
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    0x60(%rsp), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   0x68(%rsp), %r9, %r10
        mulxq   0x78(%rsp), %r11, %r12
        movq    0x70(%rsp), %rdx
        mulxq   0x78(%rsp), %r13, %r14
        xorl    %ebp, %ebp
        mulxq   0x60(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    0x78(%rsp), %rdx
        mulxq   0x68(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        adoxq   %rbp, %r14
        adcq    %rbp, %r14
        xorl    %ebp, %ebp
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    0x68(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    0x70(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    0x78(%rsp), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %rbp, %r15
        adoxq   %rbp, %r15
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    $0xffffffff00000001, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        movl    %ebp, %r9d
        adoxq   %rbp, %r9
        adcxq   %rbp, %r9
        addq    %r9, %r14
        adcq    %rbp, %r15
        movl    %ebp, %r8d
        adcq    %rbp, %r8
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    $0xffffffff00000001, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %rbp, %r15
        adoxq   %rbp, %r8
        adcq    %rbp, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rdx), %rdx
        adcq    %r13, %rdx
        leaq    -0x1(%rbp), %rbp
        movq    %rbp, %rax
        adcq    %r14, %rbp
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %rbp, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0xa0(%rsp)
        movq    %r13, 0xa8(%rsp)
        movq    %r14, 0xb0(%rsp)
        movq    %r15, 0xb8(%rsp)
        movq    0x40(%rsp), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   0x48(%rsp), %r9, %r10
        mulxq   0x58(%rsp), %r11, %r12
        movq    0x50(%rsp), %rdx
        mulxq   0x58(%rsp), %r13, %r14
        xorl    %ebp, %ebp
        mulxq   0x40(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x48(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    0x58(%rsp), %rdx
        mulxq   0x48(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        adoxq   %rbp, %r14
        adcq    %rbp, %r14
        xorl    %ebp, %ebp
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    0x48(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    0x50(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    0x58(%rsp), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %rbp, %r15
        adoxq   %rbp, %r15
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    $0xffffffff00000001, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        movl    %ebp, %r9d
        adoxq   %rbp, %r9
        adcxq   %rbp, %r9
        addq    %r9, %r14
        adcq    %rbp, %r15
        movl    %ebp, %r8d
        adcq    %rbp, %r8
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    $0xffffffff00000001, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %rbp, %r15
        adoxq   %rbp, %r8
        adcq    %rbp, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rdx), %rdx
        adcq    %r13, %rdx
        leaq    -0x1(%rbp), %rbp
        movq    %rbp, %rax
        adcq    %r14, %rbp
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %rbp, %r14
        cmovbq  %r11, %r15
        movq    %r12, 0x40(%rsp)
        movq    %r13, 0x48(%rsp)
        movq    %r14, 0x50(%rsp)
        movq    %r15, 0x58(%rsp)
        movq    $0xffffffffffffffff, %r8
        xorl    %r10d, %r10d
        subq    0xa0(%rsp), %r8
        movq    $0xffffffff, %r9
        sbbq    0xa8(%rsp), %r9
        sbbq    0xb0(%rsp), %r10
        movq    $0xffffffff00000001, %r11
        sbbq    0xb8(%rsp), %r11
        xorl    %r12d, %r12d
        movq    $0x9, %rdx
        mulxq   %r8, %r8, %rax
        mulxq   %r9, %r9, %rcx
        addq    %rax, %r9
        mulxq   %r10, %r10, %rax
        adcq    %rcx, %r10
        mulxq   %r11, %r11, %rcx
        adcq    %rax, %r11
        adcq    %rcx, %r12
        movq    $0xc, %rdx
        xorl    %eax, %eax
        mulxq   0x80(%rsp), %rax, %rcx
        adcxq   %rax, %r8
        adoxq   %rcx, %r9
        mulxq   0x88(%rsp), %rax, %rcx
        adcxq   %rax, %r9
        adoxq   %rcx, %r10
        mulxq   0x90(%rsp), %rax, %rcx
        adcxq   %rax, %r10
        adoxq   %rcx, %r11
        mulxq   0x98(%rsp), %rax, %rdx
        adcxq   %rax, %r11
        adoxq   %r12, %rdx
        adcq    $0x1, %rdx
        addq    %rdx, %r8
        movq    $0x100000000, %rax
        mulxq   %rax, %rax, %rcx
        sbbq    $0x0, %rax
        sbbq    $0x0, %rcx
        subq    %rax, %r9
        sbbq    %rcx, %r10
        movq    $0xffffffff00000001, %rax
        mulxq   %rax, %rax, %rcx
        sbbq    %rax, %r11
        sbbq    %rcx, %rdx
        decq    %rdx
        movl    $0xffffffff, %eax
        andq    %rdx, %rax
        xorl    %ecx, %ecx
        subq    %rax, %rcx
        addq    %rdx, %r8
        movq    %r8, 0xa0(%rsp)
        adcq    %rax, %r9
        movq    %r9, 0xa8(%rsp)
        adcq    $0x0, %r10
        movq    %r10, 0xb0(%rsp)
        adcq    %rcx, %r11
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
        movq    0x20(%rsp), %rdx
        mulxq   %rdx, %r8, %r15
        mulxq   0x28(%rsp), %r9, %r10
        mulxq   0x38(%rsp), %r11, %r12
        movq    0x30(%rsp), %rdx
        mulxq   0x38(%rsp), %r13, %r14
        xorl    %ebp, %ebp
        mulxq   0x20(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x28(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    0x38(%rsp), %rdx
        mulxq   0x28(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        adoxq   %rbp, %r14
        adcq    %rbp, %r14
        xorl    %ebp, %ebp
        adcxq   %r9, %r9
        adoxq   %r15, %r9
        movq    0x28(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r10, %r10
        adoxq   %rax, %r10
        adcxq   %r11, %r11
        adoxq   %rdx, %r11
        movq    0x30(%rsp), %rdx
        mulxq   %rdx, %rax, %rdx
        adcxq   %r12, %r12
        adoxq   %rax, %r12
        adcxq   %r13, %r13
        adoxq   %rdx, %r13
        movq    0x38(%rsp), %rdx
        mulxq   %rdx, %rax, %r15
        adcxq   %r14, %r14
        adoxq   %rax, %r14
        adcxq   %rbp, %r15
        adoxq   %rbp, %r15
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    $0xffffffff00000001, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %rbp, %r13
        movl    %ebp, %r9d
        adoxq   %rbp, %r9
        adcxq   %rbp, %r9
        addq    %r9, %r14
        adcq    %rbp, %r15
        movl    %ebp, %r8d
        adcq    %rbp, %r8
        xorl    %ebp, %ebp
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    $0xffffffff00000001, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %rbp, %r15
        adoxq   %rbp, %r8
        adcq    %rbp, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        leaq    -0x1(%rdx), %rdx
        adcq    %r13, %rdx
        leaq    -0x1(%rbp), %rbp
        movq    %rbp, %rax
        adcq    %r14, %rbp
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
        cmovbq  %rbp, %r14
        cmovbq  %r11, %r15
        movq    %r12, (%rsp)
        movq    %r13, 0x8(%rsp)
        movq    %r14, 0x10(%rsp)
        movq    %r15, 0x18(%rsp)
        xorl    %r13d, %r13d
        movq    0x60(%rsp), %rdx
        mulxq   0xa0(%rsp), %r8, %r9
        mulxq   0xa8(%rsp), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0xb0(%rsp), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0xb8(%rsp), %rbx, %r12
        adcq    %rbx, %r11
        adcq    %r13, %r12
        movq    0x68(%rsp), %rdx
        xorl    %r14d, %r14d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcq    %r14, %r13
        xorl    %r15d, %r15d
        movq    $0x100000000, %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r8, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r9, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adcxq   %r15, %r13
        adoxq   %r15, %r14
        adcq    %r15, %r14
        movq    0x70(%rsp), %rdx
        xorl    %r8d, %r8d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        adoxq   %r8, %r14
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r13
        adcq    %rbx, %r14
        adcq    %r8, %r15
        movq    0x78(%rsp), %rdx
        xorl    %r9d, %r9d
        mulxq   0xa0(%rsp), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0xa8(%rsp), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0xb0(%rsp), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        adoxq   %r9, %r15
        mulxq   0xb8(%rsp), %rax, %rbx
        adcq    %rax, %r14
        adcq    %rbx, %r15
        adcq    %r9, %r8
        xorl    %r9d, %r9d
        movq    $0x100000000, %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        notq    %rdx
        leaq    0x2(%rdx), %rdx
        mulxq   %r10, %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   %r11, %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        adcxq   %r9, %r15
        adoxq   %r9, %r8
        adcq    %r9, %r8
        movl    $0x1, %ecx
        addq    %r12, %rcx
        decq    %rdx
        adcq    %r13, %rdx
        decq    %r9
        movq    %r9, %rax
        adcq    %r14, %r9
        movl    $0xfffffffe, %r11d
        adcq    %r15, %r11
        adcq    %r8, %rax
        cmovbq  %rcx, %r12
        cmovbq  %rdx, %r13
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
        movq    %r11, %rdx
        movq    0x90(%rsp), %r10
        shldq   $0x2, %r10, %r11
        movq    0x88(%rsp), %r9
        shldq   $0x2, %r9, %r10
        movq    0x80(%rsp), %r8
        shldq   $0x2, %r8, %r9
        shlq    $0x2, %r8
        shrq    $0x3e, %rdx
        addq    $0x1, %rdx
        subq    0xa0(%rsp), %r8
        sbbq    0xa8(%rsp), %r9
        sbbq    0xb0(%rsp), %r10
        sbbq    0xb8(%rsp), %r11
        sbbq    $0x0, %rdx
        addq    %rdx, %r8
        movq    $0x100000000, %rax
        mulxq   %rax, %rax, %rcx
        sbbq    $0x0, %rax
        sbbq    $0x0, %rcx
        subq    %rax, %r9
        sbbq    %rcx, %r10
        movq    $0xffffffff00000001, %rax
        mulxq   %rax, %rax, %rcx
        sbbq    %rax, %r11
        sbbq    %rcx, %rdx
        decq    %rdx
        movl    $0xffffffff, %eax
        andq    %rdx, %rax
        xorl    %ecx, %ecx
        subq    %rax, %rcx
        addq    %rdx, %r8
        movq    %r8, (%rdi)
        adcq    %rax, %r9
        movq    %r9, 0x8(%rdi)
        adcq    $0x0, %r10
        movq    %r10, 0x10(%rdi)
        adcq    %rcx, %r11
        movq    %r11, 0x18(%rdi)
        movq    $0xffffffffffffffff, %r8
        xorl    %r10d, %r10d
        subq    (%rsp), %r8
        movq    $0xffffffff, %r9
        sbbq    0x8(%rsp), %r9
        sbbq    0x10(%rsp), %r10
        movq    $0xffffffff00000001, %r11
        sbbq    0x18(%rsp), %r11
        movq    %r11, %r12
        shldq   $0x3, %r10, %r11
        shldq   $0x3, %r9, %r10
        shldq   $0x3, %r8, %r9
        shlq    $0x3, %r8
        shrq    $0x3d, %r12
        movq    $0x3, %rdx
        xorl    %eax, %eax
        mulxq   0x60(%rsp), %rax, %rcx
        adcxq   %rax, %r8
        adoxq   %rcx, %r9
        mulxq   0x68(%rsp), %rax, %rcx
        adcxq   %rax, %r9
        adoxq   %rcx, %r10
        mulxq   0x70(%rsp), %rax, %rcx
        adcxq   %rax, %r10
        adoxq   %rcx, %r11
        mulxq   0x78(%rsp), %rax, %rdx
        adcxq   %rax, %r11
        adoxq   %r12, %rdx
        adcq    $0x1, %rdx
        addq    %rdx, %r8
        movq    $0x100000000, %rax
        mulxq   %rax, %rax, %rcx
        sbbq    $0x0, %rax
        sbbq    $0x0, %rcx
        subq    %rax, %r9
        sbbq    %rcx, %r10
        movq    $0xffffffff00000001, %rax
        mulxq   %rax, %rax, %rcx
        sbbq    %rax, %r11
        sbbq    %rcx, %rdx
        decq    %rdx
        movl    $0xffffffff, %eax
        andq    %rdx, %rax
        xorl    %ecx, %ecx
        subq    %rax, %rcx
        addq    %rdx, %r8
        movq    %r8, 0x20(%rdi)
        adcq    %rax, %r9
        movq    %r9, 0x28(%rdi)
        adcq    $0x0, %r10
        movq    %r10, 0x30(%rdi)
        adcq    %rcx, %r11
        movq    %r11, 0x38(%rdi)
        CFI_INC_RSP(192)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp256_montjscalarmul_p256_montjdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
