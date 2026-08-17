// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery-Jacobian form scalar multiplication for P-384
// Input scalar[6], point[18]; output res[18]
//
// extern void p384_montjscalarmul_alt
//   (uint64_t res[static 18],
//    const uint64_t scalar[static 6],
//    const uint64_t point[static 18]);
//
// This function is a variant of its affine point version p384_scalarmul.
// Here, input and output points are assumed to be in Jacobian form with
// their coordinates in the Montgomery domain. Thus, if priming indicates
// Montgomery form, x' = (2^384 * x) mod p_384 etc., each point argument
// is a triple (x',y',z') representing the affine point (x/z^2,y/z^3) when
// z' is nonzero or the point at infinity (group identity) if z' = 0.
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve P-384, returns a representation of n * P. If the result is the
// point at infinity (either because the input point was or because the
// scalar was a multiple of p_384) then the output is guaranteed to
// represent the point at infinity, i.e. to have its z coordinate zero.
//
// Standard x86-64 ABI: RDI = res, RSI = scalar, RDX = point
// Microsoft x64 ABI:   RCX = res, RDX = scalar, R8 = point
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p384_montjscalarmul_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p384_montjscalarmul_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p384_montjscalarmul_alt)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 48
#define JACSIZE (3*NUMSIZE)

// Intermediate variables on the stack.
// The table is 16 entries, each of size JACSIZE = 3 * NUMSIZE
// Uppercase syntactic variants make x86_att version simpler to generate.

#define SCALARB (0*NUMSIZE)
#define scalarb (0*NUMSIZE)(%rsp)
#define ACC (1*NUMSIZE)
#define acc (1*NUMSIZE)(%rsp)
#define TABENT (4*NUMSIZE)
#define tabent (4*NUMSIZE)(%rsp)

#define TAB (7*NUMSIZE)
#define tab (7*NUMSIZE)(%rsp)

#define res (55*NUMSIZE)(%rsp)

#define NSPACE 56*NUMSIZE

// Avoid using .rep for the sake of the BoringSSL/AWS-LC delocator,
// which doesn't accept repetitions, assembler macros etc.

#define selectblock_xz(I)                         \
        cmpq    $I, %rdi ;                           \
        cmovzq  TAB+JACSIZE*(I-1)(%rsp), %rax ;     \
        cmovzq  TAB+JACSIZE*(I-1)+8(%rsp), %rbx ;   \
        cmovzq  TAB+JACSIZE*(I-1)+16(%rsp), %rcx ;  \
        cmovzq  TAB+JACSIZE*(I-1)+24(%rsp), %rdx ;  \
        cmovzq  TAB+JACSIZE*(I-1)+32(%rsp), %r8 ;   \
        cmovzq  TAB+JACSIZE*(I-1)+40(%rsp), %r9 ;   \
        cmovzq  TAB+JACSIZE*(I-1)+96(%rsp), %r10 ;  \
        cmovzq  TAB+JACSIZE*(I-1)+104(%rsp), %r11 ; \
        cmovzq  TAB+JACSIZE*(I-1)+112(%rsp), %r12 ; \
        cmovzq  TAB+JACSIZE*(I-1)+120(%rsp), %r13 ; \
        cmovzq  TAB+JACSIZE*(I-1)+128(%rsp), %r14 ; \
        cmovzq  TAB+JACSIZE*(I-1)+136(%rsp), %r15

#define selectblock_y(I)                          \
        cmpq    $I, %rdi ;                           \
        cmovzq  TAB+JACSIZE*(I-1)+48(%rsp), %rax ;  \
        cmovzq  TAB+JACSIZE*(I-1)+56(%rsp), %rbx ;  \
        cmovzq  TAB+JACSIZE*(I-1)+64(%rsp), %rcx ;  \
        cmovzq  TAB+JACSIZE*(I-1)+72(%rsp), %rdx ;  \
        cmovzq  TAB+JACSIZE*(I-1)+80(%rsp), %r8 ;   \
        cmovzq  TAB+JACSIZE*(I-1)+88(%rsp), %r9

S2N_BN_SYMBOL(p384_montjscalarmul_alt):
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
        CFI_CALL(Lp384_montjscalarmul_alt_standard)
        CFI_POP(%rsi)
        CFI_POP(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p384_montjscalarmul_alt)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp384_montjscalarmul_alt_standard)

Lp384_montjscalarmul_alt_standard:
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

// Preserve the "res" input argument; others get processed early.

        movq    %rdi, res

// Reduce the input scalar mod n_384, i.e. conditionally subtract n_384.
// Store it to "scalarb".

        movq    (%rsi), %r8
        movq    $0xecec196accc52973, %rax
        subq    %rax, %r8
        movq    8(%rsi), %r9
        movq    $0x581a0db248b0a77a, %rax
        sbbq    %rax, %r9
        movq    16(%rsi), %r10
        movq    $0xc7634d81f4372ddf, %rax
        sbbq    %rax, %r10
        movq    24(%rsi), %r11
        movq    $0xffffffffffffffff, %rax
        sbbq    %rax, %r11
        movq    32(%rsi), %r12
        sbbq    %rax, %r12
        movq    40(%rsi), %r13
        sbbq    %rax, %r13

        cmovcq  (%rsi), %r8
        cmovcq  8(%rsi), %r9
        cmovcq  16(%rsi), %r10
        cmovcq  24(%rsi), %r11
        cmovcq  32(%rsi), %r12
        cmovcq  40(%rsi), %r13

        movq    %r8, SCALARB(%rsp)
        movq    %r9, SCALARB+8(%rsp)
        movq    %r10, SCALARB+16(%rsp)
        movq    %r11, SCALARB+24(%rsp)
        movq    %r12, SCALARB+32(%rsp)
        movq    %r13, SCALARB+40(%rsp)

// Set the tab[0] table entry to the input point = 1 * P

        movq    (%rdx), %rax
        movq    %rax, TAB(%rsp)
        movq    8(%rdx), %rax
        movq    %rax, TAB+8(%rsp)
        movq    16(%rdx), %rax
        movq    %rax, TAB+16(%rsp)
        movq    24(%rdx), %rax
        movq    %rax, TAB+24(%rsp)
        movq    32(%rdx), %rax
        movq    %rax, TAB+32(%rsp)
        movq    40(%rdx), %rax
        movq    %rax, TAB+40(%rsp)

        movq    48(%rdx), %rax
        movq    %rax, TAB+48(%rsp)
        movq    56(%rdx), %rax
        movq    %rax, TAB+56(%rsp)
        movq    64(%rdx), %rax
        movq    %rax, TAB+64(%rsp)
        movq    72(%rdx), %rax
        movq    %rax, TAB+72(%rsp)
        movq    80(%rdx), %rax
        movq    %rax, TAB+80(%rsp)
        movq    88(%rdx), %rax
        movq    %rax, TAB+88(%rsp)

        movq    96(%rdx), %rax
        movq    %rax, TAB+96(%rsp)
        movq    104(%rdx), %rax
        movq    %rax, TAB+104(%rsp)
        movq    112(%rdx), %rax
        movq    %rax, TAB+112(%rsp)
        movq    120(%rdx), %rax
        movq    %rax, TAB+120(%rsp)
        movq    128(%rdx), %rax
        movq    %rax, TAB+128(%rsp)
        movq    136(%rdx), %rax
        movq    %rax, TAB+136(%rsp)

// Compute and record tab[1] = 2 * p, ..., tab[15] = 16 * P

        leaq    TAB+JACSIZE*1(%rsp), %rdi
        leaq    TAB(%rsp), %rsi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    TAB+JACSIZE*2(%rsp), %rdi
        leaq    TAB+JACSIZE*1(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjadd)

        leaq    TAB+JACSIZE*3(%rsp), %rdi
        leaq    TAB+JACSIZE*1(%rsp), %rsi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    TAB+JACSIZE*4(%rsp), %rdi
        leaq    TAB+JACSIZE*3(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjadd)

        leaq    TAB+JACSIZE*5(%rsp), %rdi
        leaq    TAB+JACSIZE*2(%rsp), %rsi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    TAB+JACSIZE*6(%rsp), %rdi
        leaq    TAB+JACSIZE*5(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjadd)

        leaq    TAB+JACSIZE*7(%rsp), %rdi
        leaq    TAB+JACSIZE*3(%rsp), %rsi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    TAB+JACSIZE*8(%rsp), %rdi
        leaq    TAB+JACSIZE*7(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjadd)

        leaq    TAB+JACSIZE*9(%rsp), %rdi
        leaq    TAB+JACSIZE*4(%rsp), %rsi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    TAB+JACSIZE*10(%rsp), %rdi
        leaq    TAB+JACSIZE*9(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjadd)

        leaq    TAB+JACSIZE*11(%rsp), %rdi
        leaq    TAB+JACSIZE*5(%rsp), %rsi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    TAB+JACSIZE*12(%rsp), %rdi
        leaq    TAB+JACSIZE*11(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjadd)

        leaq    TAB+JACSIZE*13(%rsp), %rdi
        leaq    TAB+JACSIZE*6(%rsp), %rsi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    TAB+JACSIZE*14(%rsp), %rdi
        leaq    TAB+JACSIZE*13(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjadd)

        leaq    TAB+JACSIZE*15(%rsp), %rdi
        leaq    TAB+JACSIZE*7(%rsp), %rsi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

// Add the recoding constant sum_i(16 * 32^i) to the scalar to allow signed
// digits. The digits of the constant, in lowest-to-highest order, are as
// follows; they are generated dynamically to use fewer large constant loads.
//
// 0x0842108421084210
// 0x1084210842108421
// 0x2108421084210842
// 0x4210842108421084
// 0x8421084210842108
// 0x0842108421084210

        movq    $0x1084210842108421, %rax
        movq    %rax, %rcx
        shrq    $1, %rax
        movq    SCALARB(%rsp), %r8
        addq    %rax, %r8
        movq    SCALARB+8(%rsp), %r9
        adcq    %rcx, %r9
        leaq    (%rcx,%rcx), %rcx
        movq    SCALARB+16(%rsp), %r10
        adcq    %rcx, %r10
        leaq    (%rcx,%rcx), %rcx
        movq    SCALARB+24(%rsp), %r11
        adcq    %rcx, %r11
        leaq    (%rcx,%rcx), %rcx
        movq    SCALARB+32(%rsp), %r12
        adcq    %rcx, %r12
        movq    SCALARB+40(%rsp), %r13
        adcq    %rax, %r13
        sbbq    %rdi, %rdi
        negq    %rdi

// Record the top bitfield in %rdi then shift the whole scalar left 4 bits
// to align the top of the next bitfield with the MSB (bits 379..383).

        shldq   $4, %r13, %rdi
        shldq   $4, %r12, %r13
        shldq   $4, %r11, %r12
        shldq   $4, %r10, %r11
        shldq   $4, %r9, %r10
        shldq   $4, %r8, %r9
        shlq    $4, %r8

        movq    %r8, SCALARB(%rsp)
        movq    %r9, SCALARB+8(%rsp)
        movq    %r10, SCALARB+16(%rsp)
        movq    %r11, SCALARB+24(%rsp)
        movq    %r12, SCALARB+32(%rsp)
        movq    %r13, SCALARB+40(%rsp)

// Initialize the accumulator to the corresponding entry using constant-time
// lookup in the table. This top digit, uniquely, is not recoded so there is
// no sign adjustment to make. On the x86 integer side we don't have enough
// registers to hold all the fields; this could be better done with SIMD
// registers anyway. So we do x and z coordinates in one sweep, y in another
// (this is a rehearsal for below where we might need to negate the y).

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

        selectblock_xz(1)
        selectblock_xz(2)
        selectblock_xz(3)
        selectblock_xz(4)
        selectblock_xz(5)
        selectblock_xz(6)
        selectblock_xz(7)
        selectblock_xz(8)
        selectblock_xz(9)
        selectblock_xz(10)
        selectblock_xz(11)
        selectblock_xz(12)
        selectblock_xz(13)
        selectblock_xz(14)
        selectblock_xz(15)
        selectblock_xz(16)

        movq     %rax, ACC(%rsp)
        movq     %rbx, ACC+8(%rsp)
        movq     %rcx, ACC+16(%rsp)
        movq     %rdx, ACC+24(%rsp)
        movq     %r8, ACC+32(%rsp)
        movq     %r9, ACC+40(%rsp)
        movq     %r10, ACC+96(%rsp)
        movq     %r11, ACC+104(%rsp)
        movq     %r12, ACC+112(%rsp)
        movq     %r13, ACC+120(%rsp)
        movq     %r14, ACC+128(%rsp)
        movq     %r15, ACC+136(%rsp)

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        xorl    %r8d, %r8d
        xorl    %r9d, %r9d

        selectblock_y(1)
        selectblock_y(2)
        selectblock_y(3)
        selectblock_y(4)
        selectblock_y(5)
        selectblock_y(6)
        selectblock_y(7)
        selectblock_y(8)
        selectblock_y(9)
        selectblock_y(10)
        selectblock_y(11)
        selectblock_y(12)
        selectblock_y(13)
        selectblock_y(14)
        selectblock_y(15)
        selectblock_y(16)

        movq     %rax, ACC+48(%rsp)
        movq     %rbx, ACC+56(%rsp)
        movq     %rcx, ACC+64(%rsp)
        movq     %rdx, ACC+72(%rsp)
        movq     %r8, ACC+80(%rsp)
        movq     %r9, ACC+88(%rsp)

// Main loop over size-5 bitfields: double 5 times then add signed digit
// At each stage we shift the scalar left by 5 bits so we can simply pick
// the top 5 bits as the bitfield, saving some fiddle over indexing.

        movl    $380, %ebp

Lp384_montjscalarmul_alt_mainloop:
        subq    $5, %rbp

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjdouble)

// Choose the bitfield and adjust it to sign and magnitude

        movq    SCALARB(%rsp), %r8
        movq    SCALARB+8(%rsp), %r9
        movq    SCALARB+16(%rsp), %r10
        movq    SCALARB+24(%rsp), %r11
        movq    SCALARB+32(%rsp), %r12
        movq    SCALARB+40(%rsp), %r13

        movq    %r13, %rdi
        shrq    $59, %rdi
        shldq   $5, %r12, %r13
        shldq   $5, %r11, %r12
        shldq   $5, %r10, %r11
        shldq   $5, %r9, %r10
        shldq   $5, %r8, %r9
        shlq    $5, %r8

        movq    %r8, SCALARB(%rsp)
        movq    %r9, SCALARB+8(%rsp)
        movq    %r10, SCALARB+16(%rsp)
        movq    %r11, SCALARB+24(%rsp)
        movq    %r12, SCALARB+32(%rsp)
        movq    %r13, SCALARB+40(%rsp)

        subq    $16, %rdi
        sbbq    %rsi, %rsi // %rsi = sign of digit (-1 = negative)
        xorq    %rsi, %rdi
        subq    %rsi, %rdi // %rdi = absolute value of digit

// Conditionally select the table entry tab[i-1] = i * P in constant time
// Again, this is done in two sweeps, first doing x and z then y.

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

selectblock_xz(1)
        selectblock_xz(2)
        selectblock_xz(3)
        selectblock_xz(4)
        selectblock_xz(5)
        selectblock_xz(6)
        selectblock_xz(7)
        selectblock_xz(8)
        selectblock_xz(9)
        selectblock_xz(10)
        selectblock_xz(11)
        selectblock_xz(12)
        selectblock_xz(13)
        selectblock_xz(14)
        selectblock_xz(15)
        selectblock_xz(16)

        movq     %rax, TABENT(%rsp)
        movq     %rbx, TABENT+8(%rsp)
        movq     %rcx, TABENT+16(%rsp)
        movq     %rdx, TABENT+24(%rsp)
        movq     %r8, TABENT+32(%rsp)
        movq     %r9, TABENT+40(%rsp)
        movq     %r10, TABENT+96(%rsp)
        movq     %r11, TABENT+104(%rsp)
        movq     %r12, TABENT+112(%rsp)
        movq     %r13, TABENT+120(%rsp)
        movq     %r14, TABENT+128(%rsp)
        movq     %r15, TABENT+136(%rsp)

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        xorl    %r8d, %r8d
        xorl    %r9d, %r9d

        selectblock_y(1)
        selectblock_y(2)
        selectblock_y(3)
        selectblock_y(4)
        selectblock_y(5)
        selectblock_y(6)
        selectblock_y(7)
        selectblock_y(8)
        selectblock_y(9)
        selectblock_y(10)
        selectblock_y(11)
        selectblock_y(12)
        selectblock_y(13)
        selectblock_y(14)
        selectblock_y(15)
        selectblock_y(16)

// Store it to "tabent" with the y coordinate optionally negated.
// This is done carefully to give coordinates < p_384 even in
// the degenerate case y = 0 (when z = 0 for points on the curve).
// The digits of the prime p_384 are generated dynamically from
// the zeroth via not/lea to reduce the number of constant loads.

        movq    %rax, %r10
        orq     %rbx, %r10
        movq    %rcx, %r11
        orq     %rdx, %r11
        movq    %r8, %r12
        orq     %r9, %r12
        orq     %r11, %r10
        orq     %r12, %r10
        cmovzq  %r10, %rsi

        movl    $0xffffffff, %r10d
        movq    %r10, %r11
        notq    %r11
        leaq    (%r10,%r11), %r13
        subq    %rax, %r10
        leaq    -1(%r13), %r12
        sbbq    %rbx, %r11
        movq    %r13, %r14
        sbbq    %rcx, %r12
        sbbq    %rdx, %r13
        movq    %r14, %r15
        sbbq    %r8, %r14
        sbbq    %r9, %r15

        testq   %rsi, %rsi
        cmovnzq  %r10, %rax
        cmovnzq  %r11, %rbx
        cmovnzq  %r12, %rcx
        cmovnzq  %r13, %rdx
        cmovnzq  %r14, %r8
        cmovnzq  %r15, %r9

        movq    %rax, TABENT+48(%rsp)
        movq    %rbx, TABENT+56(%rsp)
        movq    %rcx, TABENT+64(%rsp)
        movq    %rdx, TABENT+72(%rsp)
        movq    %r8, TABENT+80(%rsp)
        movq    %r9, TABENT+88(%rsp)

// Add to the accumulator

        leaq    TABENT(%rsp), %rdx
        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp384_montjscalarmul_alt_p384_montjadd)

        testq   %rbp, %rbp
        jne     Lp384_montjscalarmul_alt_mainloop

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
        movq    ACC+96(%rsp), %rax
        movq    %rax, 96(%rdi)
        movq    ACC+104(%rsp), %rax
        movq    %rax, 104(%rdi)
        movq    ACC+112(%rsp), %rax
        movq    %rax, 112(%rdi)
        movq    ACC+120(%rsp), %rax
        movq    %rax, 120(%rdi)
        movq    ACC+128(%rsp), %rax
        movq    %rax, 128(%rdi)
        movq    ACC+136(%rsp), %rax
        movq    %rax, 136(%rdi)

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
S2N_BN_SIZE_DIRECTIVE(Lp384_montjscalarmul_alt_standard)
#else
S2N_BN_SIZE_DIRECTIVE(p384_montjscalarmul_alt)
#endif

// Local copies of subroutines, complete clones at the moment

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp384_montjscalarmul_alt_p384_montjadd)

Lp384_montjscalarmul_alt_p384_montjadd:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(352)
        movq    %rsi, 0x150(%rsp)
        movq    %rdx, 0x158(%rsp)
        movq    0x60(%rsi), %rbx
        movq    0x68(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x78(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x88(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x78(%rsi), %rax
        mulq     0x80(%rsi)
        movq    %rax, %r15
        movq    %rdx, %rcx
        movq    0x70(%rsi), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x68(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbp, %rbp
        movq    0x68(%rsi), %rbx
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    $0x0, %rcx
        movq    0x80(%rsi), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x70(%rsi), %rbx
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r15
        adcq    %rdx, %rcx
        sbbq    %rbp, %rbp
        xorl    %ebx, %ebx
        movq    0x78(%rsi), %rax
        mulq     0x88(%rsi)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    0x80(%rsi), %rax
        mulq     0x88(%rsi)
        addq    %rax, %rbx
        adcq    %rdx, %rbp
        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d
        movq    0x60(%rsi), %rax
        mulq    %rax
        movq    %r8, (%rsp)
        movq    %rax, %r8
        movq    0x68(%rsi), %rax
        movq    %rbp, 0x8(%rsp)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x70(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x78(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x80(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp
        movq    0x88(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    0x8(%rsp), %rax
        adcq    (%rsp), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rsi
        movq    %rbx, (%rsp)
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r8
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r8, %r9
        sbbq    %rdx, %r10
        sbbq    %rax, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        movq    %rbx, %r8
        sbbq    $0x0, %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r9
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r9, %r10
        sbbq    %rdx, %r11
        sbbq    %rax, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r8
        movq    %rbx, %r9
        sbbq    $0x0, %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r10
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r10, %r11
        sbbq    %rdx, %r12
        sbbq    %rax, %r13
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        movq    %rbx, %r10
        sbbq    $0x0, %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r11
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r11, %r12
        sbbq    %rdx, %r13
        sbbq    %rax, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        movq    %rbx, %r11
        sbbq    $0x0, %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r12
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r12, %r13
        sbbq    %rdx, %r8
        sbbq    %rax, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %r11
        movq    %rbx, %r12
        sbbq    $0x0, %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r13
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r13, %r8
        sbbq    %rdx, %r9
        sbbq    %rax, %r10
        sbbq    $0x0, %r11
        sbbq    $0x0, %r12
        movq    %rbx, %r13
        sbbq    $0x0, %r13
        movq    (%rsp), %rbx
        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rsi
        movl    $0x0, %r8d
        adcq    %r8, %r8
        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x1, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rsi, %r13
        adcq    $0x0, %r8
        cmovneq %rax, %r14
        cmovneq %r9, %r15
        cmovneq %r10, %rcx
        cmovneq %r11, %rbx
        cmovneq %r12, %rbp
        cmovneq %r13, %rsi
        movq    %r14, (%rsp)
        movq    %r15, 0x8(%rsp)
        movq    %rcx, 0x10(%rsp)
        movq    %rbx, 0x18(%rsp)
        movq    %rbp, 0x20(%rsp)
        movq    %rsi, 0x28(%rsp)
        movq    0x158(%rsp), %rsi
        movq    0x60(%rsi), %rbx
        movq    0x68(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x78(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x88(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x78(%rsi), %rax
        mulq     0x80(%rsi)
        movq    %rax, %r15
        movq    %rdx, %rcx
        movq    0x70(%rsi), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x68(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbp, %rbp
        movq    0x68(%rsi), %rbx
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    $0x0, %rcx
        movq    0x80(%rsi), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x70(%rsi), %rbx
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r15
        adcq    %rdx, %rcx
        sbbq    %rbp, %rbp
        xorl    %ebx, %ebx
        movq    0x78(%rsi), %rax
        mulq     0x88(%rsi)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    0x80(%rsi), %rax
        mulq     0x88(%rsi)
        addq    %rax, %rbx
        adcq    %rdx, %rbp
        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d
        movq    0x60(%rsi), %rax
        mulq    %rax
        movq    %r8, 0xf0(%rsp)
        movq    %rax, %r8
        movq    0x68(%rsi), %rax
        movq    %rbp, 0xf8(%rsp)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x70(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x78(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x80(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp
        movq    0x88(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    0xf8(%rsp), %rax
        adcq    0xf0(%rsp), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rsi
        movq    %rbx, 0xf0(%rsp)
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r8
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r8, %r9
        sbbq    %rdx, %r10
        sbbq    %rax, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        movq    %rbx, %r8
        sbbq    $0x0, %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r9
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r9, %r10
        sbbq    %rdx, %r11
        sbbq    %rax, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r8
        movq    %rbx, %r9
        sbbq    $0x0, %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r10
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r10, %r11
        sbbq    %rdx, %r12
        sbbq    %rax, %r13
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        movq    %rbx, %r10
        sbbq    $0x0, %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r11
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r11, %r12
        sbbq    %rdx, %r13
        sbbq    %rax, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        movq    %rbx, %r11
        sbbq    $0x0, %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r12
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r12, %r13
        sbbq    %rdx, %r8
        sbbq    %rax, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %r11
        movq    %rbx, %r12
        sbbq    $0x0, %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r13
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r13, %r8
        sbbq    %rdx, %r9
        sbbq    %rax, %r10
        sbbq    $0x0, %r11
        sbbq    $0x0, %r12
        movq    %rbx, %r13
        sbbq    $0x0, %r13
        movq    0xf0(%rsp), %rbx
        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rsi
        movl    $0x0, %r8d
        adcq    %r8, %r8
        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x1, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rsi, %r13
        adcq    $0x0, %r8
        cmovneq %rax, %r14
        cmovneq %r9, %r15
        cmovneq %r10, %rcx
        cmovneq %r11, %rbx
        cmovneq %r12, %rbp
        cmovneq %r13, %rsi
        movq    %r14, 0xf0(%rsp)
        movq    %r15, 0xf8(%rsp)
        movq    %rcx, 0x100(%rsp)
        movq    %rbx, 0x108(%rsp)
        movq    %rbp, 0x110(%rsp)
        movq    %rsi, 0x118(%rsp)
        movq    0x150(%rsp), %rsi
        movq    0x158(%rsp), %rcx
        movq    0x30(%rsi), %rbx
        movq    0x60(%rcx), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x68(%rcx), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x70(%rcx), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x78(%rcx), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x80(%rcx), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x88(%rcx), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x38(%rsi), %rbx
        movq    0x60(%rcx), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x68(%rcx), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x70(%rcx), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x78(%rcx), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x80(%rcx), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x88(%rcx), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x40(%rsi), %rbx
        movq    0x60(%rcx), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x68(%rcx), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x70(%rcx), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x78(%rcx), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x80(%rcx), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x88(%rcx), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x48(%rsi), %rbx
        movq    0x60(%rcx), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x68(%rcx), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x70(%rcx), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x78(%rcx), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x80(%rcx), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x88(%rcx), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x50(%rsi), %rbx
        movq    0x60(%rcx), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x68(%rcx), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x70(%rcx), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x78(%rcx), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x80(%rcx), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x88(%rcx), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x58(%rsi), %rbx
        movq    0x60(%rcx), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x68(%rcx), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x70(%rcx), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x78(%rcx), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x80(%rcx), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x88(%rcx), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0x120(%rsp)
        movq    %r15, 0x128(%rsp)
        movq    %r8, 0x130(%rsp)
        movq    %r9, 0x138(%rsp)
        movq    %r10, 0x140(%rsp)
        movq    %r11, 0x148(%rsp)
        movq    0x150(%rsp), %rsi
        movq    0x158(%rsp), %rcx
        movq    0x30(%rcx), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x68(%rsi), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x70(%rsi), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x78(%rsi), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x80(%rsi), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x88(%rsi), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x38(%rcx), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x68(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x70(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x40(%rcx), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x68(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x70(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x48(%rcx), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x68(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x70(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x50(%rcx), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x68(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x70(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x58(%rcx), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x68(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x70(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        movq    %r8, 0x40(%rsp)
        movq    %r9, 0x48(%rsp)
        movq    %r10, 0x50(%rsp)
        movq    %r11, 0x58(%rsp)
        movq    0x158(%rsp), %rcx
        movq    (%rcx), %rbx
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
        movq    0x20(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x28(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x8(%rcx), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x10(%rcx), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x18(%rcx), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x20(%rcx), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x28(%rcx), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0x60(%rsp)
        movq    %r15, 0x68(%rsp)
        movq    %r8, 0x70(%rsp)
        movq    %r9, 0x78(%rsp)
        movq    %r10, 0x80(%rsp)
        movq    %r11, 0x88(%rsp)
        movq    0x150(%rsp), %rsi
        movq    (%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x118(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x8(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x10(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x18(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x20(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x28(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0xc0(%rsp)
        movq    %r15, 0xc8(%rsp)
        movq    %r8, 0xd0(%rsp)
        movq    %r9, 0xd8(%rsp)
        movq    %r10, 0xe0(%rsp)
        movq    %r11, 0xe8(%rsp)
        movq    0x30(%rsp), %rbx
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
        movq    0x20(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x28(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x38(%rsp), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x40(%rsp), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x48(%rsp), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x50(%rsp), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x58(%rsp), %rbx
        movq    (%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x10(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x18(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x20(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x28(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        movq    %r8, 0x40(%rsp)
        movq    %r9, 0x48(%rsp)
        movq    %r10, 0x50(%rsp)
        movq    %r11, 0x58(%rsp)
        movq    0x120(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x118(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x128(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x130(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x138(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x140(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x148(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0x120(%rsp)
        movq    %r15, 0x128(%rsp)
        movq    %r8, 0x130(%rsp)
        movq    %r9, 0x138(%rsp)
        movq    %r10, 0x140(%rsp)
        movq    %r11, 0x148(%rsp)
        movq    0x60(%rsp), %rax
        subq    0xc0(%rsp), %rax
        movq    0x68(%rsp), %rdx
        sbbq    0xc8(%rsp), %rdx
        movq    0x70(%rsp), %r8
        sbbq    0xd0(%rsp), %r8
        movq    0x78(%rsp), %r9
        sbbq    0xd8(%rsp), %r9
        movq    0x80(%rsp), %r10
        sbbq    0xe0(%rsp), %r10
        movq    0x88(%rsp), %r11
        sbbq    0xe8(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %esi
        andq    %rsi, %rcx
        xorq    %rsi, %rsi
        subq    %rcx, %rsi
        subq    %rsi, %rax
        movq    %rax, 0xf0(%rsp)
        sbbq    %rcx, %rdx
        movq    %rdx, 0xf8(%rsp)
        sbbq    %rax, %rax
        andq    %rsi, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0x100(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x108(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x110(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x118(%rsp)
        movq    0x30(%rsp), %rax
        subq    0x120(%rsp), %rax
        movq    0x38(%rsp), %rdx
        sbbq    0x128(%rsp), %rdx
        movq    0x40(%rsp), %r8
        sbbq    0x130(%rsp), %r8
        movq    0x48(%rsp), %r9
        sbbq    0x138(%rsp), %r9
        movq    0x50(%rsp), %r10
        sbbq    0x140(%rsp), %r10
        movq    0x58(%rsp), %r11
        sbbq    0x148(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %esi
        andq    %rsi, %rcx
        xorq    %rsi, %rsi
        subq    %rcx, %rsi
        subq    %rsi, %rax
        movq    %rax, 0x30(%rsp)
        sbbq    %rcx, %rdx
        movq    %rdx, 0x38(%rsp)
        sbbq    %rax, %rax
        andq    %rsi, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0x40(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x48(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x50(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x58(%rsp)
        movq    0xf0(%rsp), %rbx
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x108(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x118(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x108(%rsp), %rax
        mulq     0x110(%rsp)
        movq    %rax, %r15
        movq    %rdx, %rcx
        movq    0x100(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbp, %rbp
        movq    0xf8(%rsp), %rbx
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    $0x0, %rcx
        movq    0x110(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x100(%rsp), %rbx
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r15
        adcq    %rdx, %rcx
        sbbq    %rbp, %rbp
        xorl    %ebx, %ebx
        movq    0x108(%rsp), %rax
        mulq     0x118(%rsp)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    0x110(%rsp), %rax
        mulq     0x118(%rsp)
        addq    %rax, %rbx
        adcq    %rdx, %rbp
        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d
        movq    0xf0(%rsp), %rax
        mulq    %rax
        movq    %r8, 0x90(%rsp)
        movq    %rax, %r8
        movq    0xf8(%rsp), %rax
        movq    %rbp, 0x98(%rsp)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x100(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x108(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x110(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp
        movq    0x118(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    0x98(%rsp), %rax
        adcq    0x90(%rsp), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rsi
        movq    %rbx, 0x90(%rsp)
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r8
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r8, %r9
        sbbq    %rdx, %r10
        sbbq    %rax, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        movq    %rbx, %r8
        sbbq    $0x0, %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r9
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r9, %r10
        sbbq    %rdx, %r11
        sbbq    %rax, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r8
        movq    %rbx, %r9
        sbbq    $0x0, %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r10
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r10, %r11
        sbbq    %rdx, %r12
        sbbq    %rax, %r13
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        movq    %rbx, %r10
        sbbq    $0x0, %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r11
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r11, %r12
        sbbq    %rdx, %r13
        sbbq    %rax, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        movq    %rbx, %r11
        sbbq    $0x0, %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r12
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r12, %r13
        sbbq    %rdx, %r8
        sbbq    %rax, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %r11
        movq    %rbx, %r12
        sbbq    $0x0, %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r13
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r13, %r8
        sbbq    %rdx, %r9
        sbbq    %rax, %r10
        sbbq    $0x0, %r11
        sbbq    $0x0, %r12
        movq    %rbx, %r13
        sbbq    $0x0, %r13
        movq    0x90(%rsp), %rbx
        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rsi
        movl    $0x0, %r8d
        adcq    %r8, %r8
        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x1, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rsi, %r13
        adcq    $0x0, %r8
        cmovneq %rax, %r14
        cmovneq %r9, %r15
        cmovneq %r10, %rcx
        cmovneq %r11, %rbx
        cmovneq %r12, %rbp
        cmovneq %r13, %rsi
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    %rcx, 0xa0(%rsp)
        movq    %rbx, 0xa8(%rsp)
        movq    %rbp, 0xb0(%rsp)
        movq    %rsi, 0xb8(%rsp)
        movq    0x30(%rsp), %rbx
        movq    0x38(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x48(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x58(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x48(%rsp), %rax
        mulq     0x50(%rsp)
        movq    %rax, %r15
        movq    %rdx, %rcx
        movq    0x40(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbp, %rbp
        movq    0x38(%rsp), %rbx
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    $0x0, %rcx
        movq    0x50(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x40(%rsp), %rbx
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r15
        adcq    %rdx, %rcx
        sbbq    %rbp, %rbp
        xorl    %ebx, %ebx
        movq    0x48(%rsp), %rax
        mulq     0x58(%rsp)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    0x50(%rsp), %rax
        mulq     0x58(%rsp)
        addq    %rax, %rbx
        adcq    %rdx, %rbp
        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d
        movq    0x30(%rsp), %rax
        mulq    %rax
        movq    %r8, (%rsp)
        movq    %rax, %r8
        movq    0x38(%rsp), %rax
        movq    %rbp, 0x8(%rsp)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x40(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x48(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x50(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp
        movq    0x58(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    0x8(%rsp), %rax
        adcq    (%rsp), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rsi
        movq    %rbx, (%rsp)
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r8
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r8, %r9
        sbbq    %rdx, %r10
        sbbq    %rax, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        movq    %rbx, %r8
        sbbq    $0x0, %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r9
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r9, %r10
        sbbq    %rdx, %r11
        sbbq    %rax, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r8
        movq    %rbx, %r9
        sbbq    $0x0, %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r10
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r10, %r11
        sbbq    %rdx, %r12
        sbbq    %rax, %r13
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        movq    %rbx, %r10
        sbbq    $0x0, %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r11
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r11, %r12
        sbbq    %rdx, %r13
        sbbq    %rax, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        movq    %rbx, %r11
        sbbq    $0x0, %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r12
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r12, %r13
        sbbq    %rdx, %r8
        sbbq    %rax, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %r11
        movq    %rbx, %r12
        sbbq    $0x0, %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r13
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r13, %r8
        sbbq    %rdx, %r9
        sbbq    %rax, %r10
        sbbq    $0x0, %r11
        sbbq    $0x0, %r12
        movq    %rbx, %r13
        sbbq    $0x0, %r13
        movq    (%rsp), %rbx
        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rsi
        movl    $0x0, %r8d
        adcq    %r8, %r8
        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x1, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rsi, %r13
        adcq    $0x0, %r8
        cmovneq %rax, %r14
        cmovneq %r9, %r15
        cmovneq %r10, %rcx
        cmovneq %r11, %rbx
        cmovneq %r12, %rbp
        cmovneq %r13, %rsi
        movq    %r14, (%rsp)
        movq    %r15, 0x8(%rsp)
        movq    %rcx, 0x10(%rsp)
        movq    %rbx, 0x18(%rsp)
        movq    %rbp, 0x20(%rsp)
        movq    %rsi, 0x28(%rsp)
        movq    0xc0(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x98(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0xc8(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0xd0(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0xd8(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0xe0(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0xe8(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0xc0(%rsp)
        movq    %r15, 0xc8(%rsp)
        movq    %r8, 0xd0(%rsp)
        movq    %r9, 0xd8(%rsp)
        movq    %r10, 0xe0(%rsp)
        movq    %r11, 0xe8(%rsp)
        movq    0x60(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x98(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x68(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x70(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x78(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x80(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x88(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0x60(%rsp)
        movq    %r15, 0x68(%rsp)
        movq    %r8, 0x70(%rsp)
        movq    %r9, 0x78(%rsp)
        movq    %r10, 0x80(%rsp)
        movq    %r11, 0x88(%rsp)
        movq    (%rsp), %rax
        subq    0xc0(%rsp), %rax
        movq    0x8(%rsp), %rdx
        sbbq    0xc8(%rsp), %rdx
        movq    0x10(%rsp), %r8
        sbbq    0xd0(%rsp), %r8
        movq    0x18(%rsp), %r9
        sbbq    0xd8(%rsp), %r9
        movq    0x20(%rsp), %r10
        sbbq    0xe0(%rsp), %r10
        movq    0x28(%rsp), %r11
        sbbq    0xe8(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %esi
        andq    %rsi, %rcx
        xorq    %rsi, %rsi
        subq    %rcx, %rsi
        subq    %rsi, %rax
        movq    %rax, (%rsp)
        sbbq    %rcx, %rdx
        movq    %rdx, 0x8(%rsp)
        sbbq    %rax, %rax
        andq    %rsi, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0x10(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x18(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x20(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x28(%rsp)
        movq    0x60(%rsp), %rax
        subq    0xc0(%rsp), %rax
        movq    0x68(%rsp), %rdx
        sbbq    0xc8(%rsp), %rdx
        movq    0x70(%rsp), %r8
        sbbq    0xd0(%rsp), %r8
        movq    0x78(%rsp), %r9
        sbbq    0xd8(%rsp), %r9
        movq    0x80(%rsp), %r10
        sbbq    0xe0(%rsp), %r10
        movq    0x88(%rsp), %r11
        sbbq    0xe8(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %esi
        andq    %rsi, %rcx
        xorq    %rsi, %rsi
        subq    %rcx, %rsi
        subq    %rsi, %rax
        movq    %rax, 0x90(%rsp)
        sbbq    %rcx, %rdx
        movq    %rdx, 0x98(%rsp)
        sbbq    %rax, %rax
        andq    %rsi, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0xa0(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0xa8(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0xb0(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0xb8(%rsp)
        movq    0x150(%rsp), %rsi
        movq    0x60(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x118(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x68(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x70(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x78(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x80(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x88(%rsi), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0xf0(%rsp)
        movq    %r15, 0xf8(%rsp)
        movq    %r8, 0x100(%rsp)
        movq    %r9, 0x108(%rsp)
        movq    %r10, 0x110(%rsp)
        movq    %r11, 0x118(%rsp)
        movq    (%rsp), %rax
        subq    0x60(%rsp), %rax
        movq    0x8(%rsp), %rdx
        sbbq    0x68(%rsp), %rdx
        movq    0x10(%rsp), %r8
        sbbq    0x70(%rsp), %r8
        movq    0x18(%rsp), %r9
        sbbq    0x78(%rsp), %r9
        movq    0x20(%rsp), %r10
        sbbq    0x80(%rsp), %r10
        movq    0x28(%rsp), %r11
        sbbq    0x88(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %esi
        andq    %rsi, %rcx
        xorq    %rsi, %rsi
        subq    %rcx, %rsi
        subq    %rsi, %rax
        movq    %rax, (%rsp)
        sbbq    %rcx, %rdx
        movq    %rdx, 0x8(%rsp)
        sbbq    %rax, %rax
        andq    %rsi, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0x10(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x18(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x20(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x28(%rsp)
        movq    0xc0(%rsp), %rax
        subq    (%rsp), %rax
        movq    0xc8(%rsp), %rdx
        sbbq    0x8(%rsp), %rdx
        movq    0xd0(%rsp), %r8
        sbbq    0x10(%rsp), %r8
        movq    0xd8(%rsp), %r9
        sbbq    0x18(%rsp), %r9
        movq    0xe0(%rsp), %r10
        sbbq    0x20(%rsp), %r10
        movq    0xe8(%rsp), %r11
        sbbq    0x28(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %esi
        andq    %rsi, %rcx
        xorq    %rsi, %rsi
        subq    %rcx, %rsi
        subq    %rsi, %rax
        movq    %rax, 0xc0(%rsp)
        sbbq    %rcx, %rdx
        movq    %rdx, 0xc8(%rsp)
        sbbq    %rax, %rax
        andq    %rsi, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0xd0(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0xd8(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0xe0(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0xe8(%rsp)
        movq    0x120(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x98(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x128(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x130(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x138(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x140(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x148(%rsp), %rbx
        movq    0x90(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x98(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0xa0(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0xa8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0xb0(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0xb8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    %r8, 0xa0(%rsp)
        movq    %r9, 0xa8(%rsp)
        movq    %r10, 0xb0(%rsp)
        movq    %r11, 0xb8(%rsp)
        movq    0x158(%rsp), %rcx
        movq    0x60(%rcx), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x118(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x68(%rcx), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x70(%rcx), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x78(%rcx), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x80(%rcx), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x88(%rcx), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0xf0(%rsp)
        movq    %r15, 0xf8(%rsp)
        movq    %r8, 0x100(%rsp)
        movq    %r9, 0x108(%rsp)
        movq    %r10, 0x110(%rsp)
        movq    %r11, 0x118(%rsp)
        movq    0xc0(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x38(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x40(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x48(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x50(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x58(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0xc8(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x40(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0xd0(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x40(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0xd8(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x40(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0xe0(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x40(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0xe8(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x40(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0xc0(%rsp)
        movq    %r15, 0xc8(%rsp)
        movq    %r8, 0xd0(%rsp)
        movq    %r9, 0xd8(%rsp)
        movq    %r10, 0xe0(%rsp)
        movq    %r11, 0xe8(%rsp)
        movq    0xc0(%rsp), %rax
        subq    0x90(%rsp), %rax
        movq    0xc8(%rsp), %rdx
        sbbq    0x98(%rsp), %rdx
        movq    0xd0(%rsp), %r8
        sbbq    0xa0(%rsp), %r8
        movq    0xd8(%rsp), %r9
        sbbq    0xa8(%rsp), %r9
        movq    0xe0(%rsp), %r10
        sbbq    0xb0(%rsp), %r10
        movq    0xe8(%rsp), %r11
        sbbq    0xb8(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %esi
        andq    %rsi, %rcx
        xorq    %rsi, %rsi
        subq    %rcx, %rsi
        subq    %rsi, %rax
        movq    %rax, 0xc0(%rsp)
        sbbq    %rcx, %rdx
        movq    %rdx, 0xc8(%rsp)
        sbbq    %rax, %rax
        andq    %rsi, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0xd0(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0xd8(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0xe0(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0xe8(%rsp)
        movq    0x158(%rsp), %rcx
        movq    0x60(%rcx), %r8
        movq    0x68(%rcx), %r9
        movq    0x70(%rcx), %r10
        movq    0x78(%rcx), %r11
        movq    0x80(%rcx), %rbx
        movq    0x88(%rcx), %rbp
        movq    %r8, %rax
        movq    %r9, %rdx
        orq     %r10, %rax
        orq     %r11, %rdx
        orq     %rbx, %rax
        orq     %rbp, %rdx
        orq     %rdx, %rax
        negq    %rax
        sbbq    %rax, %rax
        movq    0x150(%rsp), %rsi
        movq    0x60(%rsi), %r12
        movq    0x68(%rsi), %r13
        movq    0x70(%rsi), %r14
        movq    0x78(%rsi), %r15
        movq    0x80(%rsi), %rdx
        movq    0x88(%rsi), %rcx
        cmoveq  %r12, %r8
        cmoveq  %r13, %r9
        cmoveq  %r14, %r10
        cmoveq  %r15, %r11
        cmoveq  %rdx, %rbx
        cmoveq  %rcx, %rbp
        orq     %r13, %r12
        orq     %r15, %r14
        orq     %rcx, %rdx
        orq     %r14, %r12
        orq     %r12, %rdx
        negq    %rdx
        sbbq    %rdx, %rdx
        cmpq    %rdx, %rax
        cmoveq  0xf0(%rsp), %r8
        cmoveq  0xf8(%rsp), %r9
        cmoveq  0x100(%rsp), %r10
        cmoveq  0x108(%rsp), %r11
        cmoveq  0x110(%rsp), %rbx
        cmoveq  0x118(%rsp), %rbp
        movq    %r8, 0xf0(%rsp)
        movq    %r9, 0xf8(%rsp)
        movq    %r10, 0x100(%rsp)
        movq    %r11, 0x108(%rsp)
        movq    %rbx, 0x110(%rsp)
        movq    %rbp, 0x118(%rsp)
        movq    0x158(%rsp), %rcx
        movq    0x150(%rsp), %rsi
        movq    (%rsp), %r8
        cmovbq  (%rsi), %r8
        cmova   (%rcx), %r8
        movq    0x8(%rsp), %r9
        cmovbq  0x8(%rsi), %r9
        cmova   0x8(%rcx), %r9
        movq    0x10(%rsp), %r10
        cmovbq  0x10(%rsi), %r10
        cmova   0x10(%rcx), %r10
        movq    0x18(%rsp), %r11
        cmovbq  0x18(%rsi), %r11
        cmova   0x18(%rcx), %r11
        movq    0x20(%rsp), %rbx
        cmovbq  0x20(%rsi), %rbx
        cmova   0x20(%rcx), %rbx
        movq    0x28(%rsp), %rbp
        cmovbq  0x28(%rsi), %rbp
        cmova   0x28(%rcx), %rbp
        movq    0xc0(%rsp), %r12
        cmovbq  0x30(%rsi), %r12
        cmova   0x30(%rcx), %r12
        movq    0xc8(%rsp), %r13
        cmovbq  0x38(%rsi), %r13
        cmova   0x38(%rcx), %r13
        movq    0xd0(%rsp), %r14
        cmovbq  0x40(%rsi), %r14
        cmova   0x40(%rcx), %r14
        movq    0xd8(%rsp), %r15
        cmovbq  0x48(%rsi), %r15
        cmova   0x48(%rcx), %r15
        movq    0xe0(%rsp), %rdx
        cmovbq  0x50(%rsi), %rdx
        cmova   0x50(%rcx), %rdx
        movq    0xe8(%rsp), %rax
        cmovbq  0x58(%rsi), %rax
        cmova   0x58(%rcx), %rax
        movq    %r8, (%rdi)
        movq    %r9, 0x8(%rdi)
        movq    %r10, 0x10(%rdi)
        movq    %r11, 0x18(%rdi)
        movq    %rbx, 0x20(%rdi)
        movq    %rbp, 0x28(%rdi)
        movq    0xf0(%rsp), %r8
        movq    0xf8(%rsp), %r9
        movq    0x100(%rsp), %r10
        movq    0x108(%rsp), %r11
        movq    0x110(%rsp), %rbx
        movq    0x118(%rsp), %rbp
        movq    %r12, 0x30(%rdi)
        movq    %r13, 0x38(%rdi)
        movq    %r14, 0x40(%rdi)
        movq    %r15, 0x48(%rdi)
        movq    %rdx, 0x50(%rdi)
        movq    %rax, 0x58(%rdi)
        movq    %r8, 0x60(%rdi)
        movq    %r9, 0x68(%rdi)
        movq    %r10, 0x70(%rdi)
        movq    %r11, 0x78(%rdi)
        movq    %rbx, 0x80(%rdi)
        movq    %rbp, 0x88(%rdi)
        CFI_INC_RSP(352)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp384_montjscalarmul_alt_p384_montjadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp384_montjscalarmul_alt_p384_montjdouble)

Lp384_montjscalarmul_alt_p384_montjdouble:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(344)
        movq    %rdi, 0x150(%rsp)
        movq    0x60(%rsi), %rbx
        movq    0x68(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x78(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x88(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x78(%rsi), %rax
        mulq     0x80(%rsi)
        movq    %rax, %r15
        movq    %rdx, %rcx
        movq    0x70(%rsi), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x68(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbp, %rbp
        movq    0x68(%rsi), %rbx
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    $0x0, %rcx
        movq    0x80(%rsi), %rbx
        movq    0x60(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x70(%rsi), %rbx
        movq    0x78(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x80(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x88(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r15
        adcq    %rdx, %rcx
        sbbq    %rbp, %rbp
        xorl    %ebx, %ebx
        movq    0x78(%rsi), %rax
        mulq     0x88(%rsi)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    0x80(%rsi), %rax
        mulq     0x88(%rsi)
        addq    %rax, %rbx
        adcq    %rdx, %rbp
        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d
        movq    0x60(%rsi), %rax
        mulq    %rax
        movq    %r8, (%rsp)
        movq    %rax, %r8
        movq    0x68(%rsi), %rax
        movq    %rbp, 0x8(%rsp)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x70(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x78(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x80(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp
        movq    0x88(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    0x8(%rsp), %rax
        adcq    (%rsp), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rdi
        movq    %rbx, (%rsp)
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r8
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r8, %r9
        sbbq    %rdx, %r10
        sbbq    %rax, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        movq    %rbx, %r8
        sbbq    $0x0, %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r9
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r9, %r10
        sbbq    %rdx, %r11
        sbbq    %rax, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r8
        movq    %rbx, %r9
        sbbq    $0x0, %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r10
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r10, %r11
        sbbq    %rdx, %r12
        sbbq    %rax, %r13
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        movq    %rbx, %r10
        sbbq    $0x0, %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r11
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r11, %r12
        sbbq    %rdx, %r13
        sbbq    %rax, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        movq    %rbx, %r11
        sbbq    $0x0, %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r12
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r12, %r13
        sbbq    %rdx, %r8
        sbbq    %rax, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %r11
        movq    %rbx, %r12
        sbbq    $0x0, %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r13
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r13, %r8
        sbbq    %rdx, %r9
        sbbq    %rax, %r10
        sbbq    $0x0, %r11
        sbbq    $0x0, %r12
        movq    %rbx, %r13
        sbbq    $0x0, %r13
        movq    (%rsp), %rbx
        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rdi
        movl    $0x0, %r8d
        adcq    %r8, %r8
        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x1, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rdi, %r13
        adcq    $0x0, %r8
        cmovneq %rax, %r14
        cmovneq %r9, %r15
        cmovneq %r10, %rcx
        cmovneq %r11, %rbx
        cmovneq %r12, %rbp
        cmovneq %r13, %rdi
        movq    %r14, (%rsp)
        movq    %r15, 0x8(%rsp)
        movq    %rcx, 0x10(%rsp)
        movq    %rbx, 0x18(%rsp)
        movq    %rbp, 0x20(%rsp)
        movq    %rdi, 0x28(%rsp)
        movq    0x30(%rsi), %rbx
        movq    0x38(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x48(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x58(%rsi), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x48(%rsi), %rax
        mulq     0x50(%rsi)
        movq    %rax, %r15
        movq    %rdx, %rcx
        movq    0x40(%rsi), %rbx
        movq    0x30(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x38(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbp, %rbp
        movq    0x38(%rsi), %rbx
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x50(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x58(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    $0x0, %rcx
        movq    0x50(%rsi), %rbx
        movq    0x30(%rsi), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x40(%rsi), %rbx
        movq    0x48(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x50(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x58(%rsi), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r15
        adcq    %rdx, %rcx
        sbbq    %rbp, %rbp
        xorl    %ebx, %ebx
        movq    0x48(%rsi), %rax
        mulq     0x58(%rsi)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    0x50(%rsi), %rax
        mulq     0x58(%rsi)
        addq    %rax, %rbx
        adcq    %rdx, %rbp
        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d
        movq    0x30(%rsi), %rax
        mulq    %rax
        movq    %r8, 0x30(%rsp)
        movq    %rax, %r8
        movq    0x38(%rsi), %rax
        movq    %rbp, 0x38(%rsp)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x40(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x48(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x50(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp
        movq    0x58(%rsi), %rax
        mulq    %rax
        negq    %rbp
        adcq    0x38(%rsp), %rax
        adcq    0x30(%rsp), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rdi
        movq    %rbx, 0x30(%rsp)
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r8
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r8, %r9
        sbbq    %rdx, %r10
        sbbq    %rax, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        movq    %rbx, %r8
        sbbq    $0x0, %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r9
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r9, %r10
        sbbq    %rdx, %r11
        sbbq    %rax, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r8
        movq    %rbx, %r9
        sbbq    $0x0, %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r10
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r10, %r11
        sbbq    %rdx, %r12
        sbbq    %rax, %r13
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        movq    %rbx, %r10
        sbbq    $0x0, %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r11
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r11, %r12
        sbbq    %rdx, %r13
        sbbq    %rax, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        movq    %rbx, %r11
        sbbq    $0x0, %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r12
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r12, %r13
        sbbq    %rdx, %r8
        sbbq    %rax, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %r11
        movq    %rbx, %r12
        sbbq    $0x0, %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r13
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r13, %r8
        sbbq    %rdx, %r9
        sbbq    %rax, %r10
        sbbq    $0x0, %r11
        sbbq    $0x0, %r12
        movq    %rbx, %r13
        sbbq    $0x0, %r13
        movq    0x30(%rsp), %rbx
        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rdi
        movl    $0x0, %r8d
        adcq    %r8, %r8
        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x1, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rdi, %r13
        adcq    $0x0, %r8
        cmovneq %rax, %r14
        cmovneq %r9, %r15
        cmovneq %r10, %rcx
        cmovneq %r11, %rbx
        cmovneq %r12, %rbp
        cmovneq %r13, %rdi
        movq    %r14, 0x30(%rsp)
        movq    %r15, 0x38(%rsp)
        movq    %rcx, 0x40(%rsp)
        movq    %rbx, 0x48(%rsp)
        movq    %rbp, 0x50(%rsp)
        movq    %rdi, 0x58(%rsp)
        movq    (%rsi), %rax
        addq    (%rsp), %rax
        movq    0x8(%rsi), %rcx
        adcq    0x8(%rsp), %rcx
        movq    0x10(%rsi), %r8
        adcq    0x10(%rsp), %r8
        movq    0x18(%rsi), %r9
        adcq    0x18(%rsp), %r9
        movq    0x20(%rsi), %r10
        adcq    0x20(%rsp), %r10
        movq    0x28(%rsi), %r11
        adcq    0x28(%rsp), %r11
        sbbq    %rdx, %rdx
        movl    $0x1, %ebx
        andq    %rdx, %rbx
        movl    $0xffffffff, %ebp
        andq    %rbp, %rdx
        xorq    %rbp, %rbp
        subq    %rdx, %rbp
        addq    %rbp, %rax
        movq    %rax, 0xf0(%rsp)
        adcq    %rdx, %rcx
        movq    %rcx, 0xf8(%rsp)
        adcq    %rbx, %r8
        movq    %r8, 0x100(%rsp)
        adcq    $0x0, %r9
        movq    %r9, 0x108(%rsp)
        adcq    $0x0, %r10
        movq    %r10, 0x110(%rsp)
        adcq    $0x0, %r11
        movq    %r11, 0x118(%rsp)
        movq    (%rsi), %rax
        subq    (%rsp), %rax
        movq    0x8(%rsi), %rdx
        sbbq    0x8(%rsp), %rdx
        movq    0x10(%rsi), %r8
        sbbq    0x10(%rsp), %r8
        movq    0x18(%rsi), %r9
        sbbq    0x18(%rsp), %r9
        movq    0x20(%rsi), %r10
        sbbq    0x20(%rsp), %r10
        movq    0x28(%rsi), %r11
        sbbq    0x28(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %ebx
        andq    %rbx, %rcx
        xorq    %rbx, %rbx
        subq    %rcx, %rbx
        subq    %rbx, %rax
        movq    %rax, 0xc0(%rsp)
        sbbq    %rcx, %rdx
        movq    %rdx, 0xc8(%rsp)
        sbbq    %rax, %rax
        andq    %rbx, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0xd0(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0xd8(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0xe0(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0xe8(%rsp)
        movq    0xc0(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x118(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0xc8(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0xd0(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0xd8(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0xe0(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0xe8(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x100(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0x60(%rsp)
        movq    %r15, 0x68(%rsp)
        movq    %r8, 0x70(%rsp)
        movq    %r9, 0x78(%rsp)
        movq    %r10, 0x80(%rsp)
        movq    %r11, 0x88(%rsp)
        movq    0x30(%rsi), %rax
        addq    0x60(%rsi), %rax
        movq    0x38(%rsi), %rcx
        adcq    0x68(%rsi), %rcx
        movq    0x40(%rsi), %r8
        adcq    0x70(%rsi), %r8
        movq    0x48(%rsi), %r9
        adcq    0x78(%rsi), %r9
        movq    0x50(%rsi), %r10
        adcq    0x80(%rsi), %r10
        movq    0x58(%rsi), %r11
        adcq    0x88(%rsi), %r11
        movl    $0x0, %edx
        adcq    %rdx, %rdx
        movabsq $0xffffffff00000001, %rbp
        addq    %rbp, %rax
        movl    $0xffffffff, %ebp
        adcq    %rbp, %rcx
        adcq    $0x1, %r8
        adcq    $0x0, %r9
        adcq    $0x0, %r10
        adcq    $0x0, %r11
        adcq    $0xffffffffffffffff, %rdx
        movl    $0x1, %ebx
        andq    %rdx, %rbx
        andq    %rbp, %rdx
        xorq    %rbp, %rbp
        subq    %rdx, %rbp
        subq    %rbp, %rax
        movq    %rax, 0xf0(%rsp)
        sbbq    %rdx, %rcx
        movq    %rcx, 0xf8(%rsp)
        sbbq    %rbx, %r8
        movq    %r8, 0x100(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x108(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x110(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x118(%rsp)
        movq    0x60(%rsp), %rbx
        movq    0x68(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x78(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x88(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x78(%rsp), %rax
        mulq     0x80(%rsp)
        movq    %rax, %r15
        movq    %rdx, %rcx
        movq    0x70(%rsp), %rbx
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x68(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbp, %rbp
        movq    0x68(%rsp), %rbx
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x80(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x88(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    $0x0, %rcx
        movq    0x80(%rsp), %rbx
        movq    0x60(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x70(%rsp), %rbx
        movq    0x78(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x80(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x88(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r15
        adcq    %rdx, %rcx
        sbbq    %rbp, %rbp
        xorl    %ebx, %ebx
        movq    0x78(%rsp), %rax
        mulq     0x88(%rsp)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    0x80(%rsp), %rax
        mulq     0x88(%rsp)
        addq    %rax, %rbx
        adcq    %rdx, %rbp
        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d
        movq    0x60(%rsp), %rax
        mulq    %rax
        movq    %r8, 0x120(%rsp)
        movq    %rax, %r8
        movq    0x68(%rsp), %rax
        movq    %rbp, 0x128(%rsp)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x70(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x78(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x80(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp
        movq    0x88(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    0x128(%rsp), %rax
        adcq    0x120(%rsp), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rdi
        movq    %rbx, 0x120(%rsp)
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r8
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r8, %r9
        sbbq    %rdx, %r10
        sbbq    %rax, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        movq    %rbx, %r8
        sbbq    $0x0, %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r9
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r9, %r10
        sbbq    %rdx, %r11
        sbbq    %rax, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r8
        movq    %rbx, %r9
        sbbq    $0x0, %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r10
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r10, %r11
        sbbq    %rdx, %r12
        sbbq    %rax, %r13
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        movq    %rbx, %r10
        sbbq    $0x0, %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r11
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r11, %r12
        sbbq    %rdx, %r13
        sbbq    %rax, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        movq    %rbx, %r11
        sbbq    $0x0, %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r12
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r12, %r13
        sbbq    %rdx, %r8
        sbbq    %rax, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %r11
        movq    %rbx, %r12
        sbbq    $0x0, %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r13
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r13, %r8
        sbbq    %rdx, %r9
        sbbq    %rax, %r10
        sbbq    $0x0, %r11
        sbbq    $0x0, %r12
        movq    %rbx, %r13
        sbbq    $0x0, %r13
        movq    0x120(%rsp), %rbx
        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rdi
        movl    $0x0, %r8d
        adcq    %r8, %r8
        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x1, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rdi, %r13
        adcq    $0x0, %r8
        cmovneq %rax, %r14
        cmovneq %r9, %r15
        cmovneq %r10, %rcx
        cmovneq %r11, %rbx
        cmovneq %r12, %rbp
        cmovneq %r13, %rdi
        movq    %r14, 0x120(%rsp)
        movq    %r15, 0x128(%rsp)
        movq    %rcx, 0x130(%rsp)
        movq    %rbx, 0x138(%rsp)
        movq    %rbp, 0x140(%rsp)
        movq    %rdi, 0x148(%rsp)
        movq    0x30(%rsp), %rbx
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
        movq    0x20(%rsi), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x28(%rsi), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x38(%rsp), %rbx
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x20(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x28(%rsi), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x40(%rsp), %rbx
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x20(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x28(%rsi), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x48(%rsp), %rbx
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x20(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x28(%rsi), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x50(%rsp), %rbx
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x20(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x28(%rsi), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x58(%rsp), %rbx
        movq    (%rsi), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x8(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x10(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x18(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x20(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x28(%rsi), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0x90(%rsp)
        movq    %r15, 0x98(%rsp)
        movq    %r8, 0xa0(%rsp)
        movq    %r9, 0xa8(%rsp)
        movq    %r10, 0xb0(%rsp)
        movq    %r11, 0xb8(%rsp)
        movq    0xf0(%rsp), %rbx
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x108(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x118(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x108(%rsp), %rax
        mulq     0x110(%rsp)
        movq    %rax, %r15
        movq    %rdx, %rcx
        movq    0x100(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0xf8(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbp, %rbp
        movq    0xf8(%rsp), %rbx
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    $0x0, %rcx
        movq    0x110(%rsp), %rbx
        movq    0xf0(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x100(%rsp), %rbx
        movq    0x108(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x110(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x118(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r15
        adcq    %rdx, %rcx
        sbbq    %rbp, %rbp
        xorl    %ebx, %ebx
        movq    0x108(%rsp), %rax
        mulq     0x118(%rsp)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    0x110(%rsp), %rax
        mulq     0x118(%rsp)
        addq    %rax, %rbx
        adcq    %rdx, %rbp
        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d
        movq    0xf0(%rsp), %rax
        mulq    %rax
        movq    %r8, 0xc0(%rsp)
        movq    %rax, %r8
        movq    0xf8(%rsp), %rax
        movq    %rbp, 0xc8(%rsp)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x100(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x108(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x110(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp
        movq    0x118(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    0xc8(%rsp), %rax
        adcq    0xc0(%rsp), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rdi
        movq    %rbx, 0xc0(%rsp)
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r8
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r8, %r9
        sbbq    %rdx, %r10
        sbbq    %rax, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        movq    %rbx, %r8
        sbbq    $0x0, %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r9
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r9, %r10
        sbbq    %rdx, %r11
        sbbq    %rax, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r8
        movq    %rbx, %r9
        sbbq    $0x0, %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r10
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r10, %r11
        sbbq    %rdx, %r12
        sbbq    %rax, %r13
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        movq    %rbx, %r10
        sbbq    $0x0, %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r11
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r11, %r12
        sbbq    %rdx, %r13
        sbbq    %rax, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        movq    %rbx, %r11
        sbbq    $0x0, %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r12
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r12, %r13
        sbbq    %rdx, %r8
        sbbq    %rax, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %r11
        movq    %rbx, %r12
        sbbq    $0x0, %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r13
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r13, %r8
        sbbq    %rdx, %r9
        sbbq    %rax, %r10
        sbbq    $0x0, %r11
        sbbq    $0x0, %r12
        movq    %rbx, %r13
        sbbq    $0x0, %r13
        movq    0xc0(%rsp), %rbx
        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rdi
        movl    $0x0, %r8d
        adcq    %r8, %r8
        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x1, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rdi, %r13
        adcq    $0x0, %r8
        cmovneq %rax, %r14
        cmovneq %r9, %r15
        cmovneq %r10, %rcx
        cmovneq %r11, %rbx
        cmovneq %r12, %rbp
        cmovneq %r13, %rdi
        movq    %r14, 0xc0(%rsp)
        movq    %r15, 0xc8(%rsp)
        movq    %rcx, 0xd0(%rsp)
        movq    %rbx, 0xd8(%rsp)
        movq    %rbp, 0xe0(%rsp)
        movq    %rdi, 0xe8(%rsp)
        movabsq $0xffffffff, %r9
        subq    0x120(%rsp), %r9
        movabsq $0xffffffff00000000, %r10
        sbbq    0x128(%rsp), %r10
        movq    $0xfffffffffffffffe, %r11
        sbbq    0x130(%rsp), %r11
        movq    $0xffffffffffffffff, %r12
        sbbq    0x138(%rsp), %r12
        movq    $0xffffffffffffffff, %r13
        sbbq    0x140(%rsp), %r13
        movq    $0xffffffffffffffff, %r14
        sbbq    0x148(%rsp), %r14
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
        movq    %r13, %rax
        xorl    %r13d, %r13d
        mulq    %rcx
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    %r14, %rax
        movl    $0x1, %r14d
        mulq    %rcx
        addq    %rax, %r13
        adcq    %rdx, %r14
        movl    $0xc, %ecx
        movq    0x90(%rsp), %rax
        mulq    %rcx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %rbx, %rbx
        movq    0x98(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rbx, %rbx
        movq    0xa0(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbx, %rbx
        movq    0xa8(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbx, %rbx
        movq    0xb0(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbx, %rbx
        movq    0xb8(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        movabsq $0xffffffff00000001, %rax
        mulq    %r14
        addq    %rax, %r8
        adcq    %rdx, %r9
        adcq    %r14, %r10
        movq    %r14, %rax
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %edx
        negq    %rcx
        mulq    %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        adcq    %rcx, %r11
        adcq    $0x0, %r12
        adcq    $0x0, %r13
        sbbq    %rcx, %rcx
        notq    %rcx
        movl    $0xffffffff, %edx
        xorq    %rax, %rax
        andq    %rcx, %rdx
        subq    %rdx, %rax
        andq    $0x1, %rcx
        subq    %rax, %r8
        movq    %r8, 0x120(%rsp)
        sbbq    %rdx, %r9
        movq    %r9, 0x128(%rsp)
        sbbq    %rcx, %r10
        movq    %r10, 0x130(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x138(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x140(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x148(%rsp)
        movq    0xc0(%rsp), %rax
        subq    (%rsp), %rax
        movq    0xc8(%rsp), %rdx
        sbbq    0x8(%rsp), %rdx
        movq    0xd0(%rsp), %r8
        sbbq    0x10(%rsp), %r8
        movq    0xd8(%rsp), %r9
        sbbq    0x18(%rsp), %r9
        movq    0xe0(%rsp), %r10
        sbbq    0x20(%rsp), %r10
        movq    0xe8(%rsp), %r11
        sbbq    0x28(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %ebx
        andq    %rbx, %rcx
        xorq    %rbx, %rbx
        subq    %rcx, %rbx
        subq    %rbx, %rax
        movq    %rax, 0xf0(%rsp)
        sbbq    %rcx, %rdx
        movq    %rdx, 0xf8(%rsp)
        sbbq    %rax, %rax
        andq    %rbx, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0x100(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x108(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x110(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x118(%rsp)
        movq    0x30(%rsp), %rbx
        movq    0x38(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r9
        movq    %rdx, %r10
        movq    0x48(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r11
        movq    %rdx, %r12
        movq    0x58(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r13
        movq    %rdx, %r14
        movq    0x48(%rsp), %rax
        mulq     0x50(%rsp)
        movq    %rax, %r15
        movq    %rdx, %rcx
        movq    0x40(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x38(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbp, %rbp
        movq    0x38(%rsp), %rbx
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        adcq    $0x0, %rcx
        movq    0x50(%rsp), %rbx
        movq    0x30(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x40(%rsp), %rbx
        movq    0x48(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %rbp, %rbp
        movq    0x50(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x58(%rsp), %rax
        mulq    %rbx
        subq    %rbp, %rdx
        addq    %rax, %r15
        adcq    %rdx, %rcx
        sbbq    %rbp, %rbp
        xorl    %ebx, %ebx
        movq    0x48(%rsp), %rax
        mulq     0x58(%rsp)
        subq    %rbp, %rdx
        xorl    %ebp, %ebp
        addq    %rax, %rcx
        adcq    %rdx, %rbx
        adcl    %ebp, %ebp
        movq    0x50(%rsp), %rax
        mulq     0x58(%rsp)
        addq    %rax, %rbx
        adcq    %rdx, %rbp
        xorl    %r8d, %r8d
        addq    %r9, %r9
        adcq    %r10, %r10
        adcq    %r11, %r11
        adcq    %r12, %r12
        adcq    %r13, %r13
        adcq    %r14, %r14
        adcq    %r15, %r15
        adcq    %rcx, %rcx
        adcq    %rbx, %rbx
        adcq    %rbp, %rbp
        adcl    %r8d, %r8d
        movq    0x30(%rsp), %rax
        mulq    %rax
        movq    %r8, 0xc0(%rsp)
        movq    %rax, %r8
        movq    0x38(%rsp), %rax
        movq    %rbp, 0xc8(%rsp)
        addq    %rdx, %r9
        sbbq    %rbp, %rbp
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbp, %rbp
        movq    0x40(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbp, %rbp
        movq    0x48(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %rbp, %rbp
        movq    0x50(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    %rax, %rcx
        adcq    %rdx, %rbx
        sbbq    %rbp, %rbp
        movq    0x58(%rsp), %rax
        mulq    %rax
        negq    %rbp
        adcq    0xc8(%rsp), %rax
        adcq    0xc0(%rsp), %rdx
        movq    %rax, %rbp
        movq    %rdx, %rdi
        movq    %rbx, 0xc0(%rsp)
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r8
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r8, %r9
        sbbq    %rdx, %r10
        sbbq    %rax, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        movq    %rbx, %r8
        sbbq    $0x0, %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r9
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r9, %r10
        sbbq    %rdx, %r11
        sbbq    %rax, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r8
        movq    %rbx, %r9
        sbbq    $0x0, %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r10
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r10, %r11
        sbbq    %rdx, %r12
        sbbq    %rax, %r13
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        movq    %rbx, %r10
        sbbq    $0x0, %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r11
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r11, %r12
        sbbq    %rdx, %r13
        sbbq    %rax, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        movq    %rbx, %r11
        sbbq    $0x0, %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r12
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r12, %r13
        sbbq    %rdx, %r8
        sbbq    %rax, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %r11
        movq    %rbx, %r12
        sbbq    $0x0, %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %rax, %r13
        movl    $0x0, %eax
        adcq    %rbx, %rdx
        adcl    %eax, %eax
        subq    %r13, %r8
        sbbq    %rdx, %r9
        sbbq    %rax, %r10
        sbbq    $0x0, %r11
        sbbq    $0x0, %r12
        movq    %rbx, %r13
        sbbq    $0x0, %r13
        movq    0xc0(%rsp), %rbx
        addq    %r8, %r14
        adcq    %r9, %r15
        adcq    %r10, %rcx
        adcq    %r11, %rbx
        adcq    %r12, %rbp
        adcq    %r13, %rdi
        movl    $0x0, %r8d
        adcq    %r8, %r8
        xorq    %r11, %r11
        xorq    %r12, %r12
        xorq    %r13, %r13
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %r9d
        adcq    %r15, %r9
        movl    $0x1, %r10d
        adcq    %rcx, %r10
        adcq    %rbx, %r11
        adcq    %rbp, %r12
        adcq    %rdi, %r13
        adcq    $0x0, %r8
        cmovneq %rax, %r14
        cmovneq %r9, %r15
        cmovneq %r10, %rcx
        cmovneq %r11, %rbx
        cmovneq %r12, %rbp
        cmovneq %r13, %rdi
        movq    %r14, 0xc0(%rsp)
        movq    %r15, 0xc8(%rsp)
        movq    %rcx, 0xd0(%rsp)
        movq    %rbx, 0xd8(%rsp)
        movq    %rbp, 0xe0(%rsp)
        movq    %rdi, 0xe8(%rsp)
        movq    0x150(%rsp), %rdi
        movq    0xf0(%rsp), %rax
        subq    0x30(%rsp), %rax
        movq    0xf8(%rsp), %rdx
        sbbq    0x38(%rsp), %rdx
        movq    0x100(%rsp), %r8
        sbbq    0x40(%rsp), %r8
        movq    0x108(%rsp), %r9
        sbbq    0x48(%rsp), %r9
        movq    0x110(%rsp), %r10
        sbbq    0x50(%rsp), %r10
        movq    0x118(%rsp), %r11
        sbbq    0x58(%rsp), %r11
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %ebx
        andq    %rbx, %rcx
        xorq    %rbx, %rbx
        subq    %rcx, %rbx
        subq    %rbx, %rax
        movq    %rax, 0x60(%rdi)
        sbbq    %rcx, %rdx
        movq    %rdx, 0x68(%rdi)
        sbbq    %rax, %rax
        andq    %rbx, %rcx
        negq    %rax
        sbbq    %rcx, %r8
        movq    %r8, 0x70(%rdi)
        sbbq    $0x0, %r9
        movq    %r9, 0x78(%rdi)
        sbbq    $0x0, %r10
        movq    %r10, 0x80(%rdi)
        sbbq    $0x0, %r11
        movq    %r11, 0x88(%rdi)
        movq    0x60(%rsp), %rbx
        movq    0x120(%rsp), %rax
        mulq    %rbx
        movq    %rax, %r8
        movq    %rdx, %r9
        movq    0x128(%rsp), %rax
        mulq    %rbx
        xorl    %r10d, %r10d
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x130(%rsp), %rax
        mulq    %rbx
        xorl    %r11d, %r11d
        addq    %rax, %r10
        adcq    %rdx, %r11
        movq    0x138(%rsp), %rax
        mulq    %rbx
        xorl    %r12d, %r12d
        addq    %rax, %r11
        adcq    %rdx, %r12
        movq    0x140(%rsp), %rax
        mulq    %rbx
        xorl    %r13d, %r13d
        addq    %rax, %r12
        adcq    %rdx, %r13
        movq    0x148(%rsp), %rax
        mulq    %rbx
        xorl    %r14d, %r14d
        addq    %rax, %r13
        adcq    %rdx, %r14
        xorl    %r15d, %r15d
        movq    %r8, %rbx
        shlq    $0x20, %rbx
        addq    %r8, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r8
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r8, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r9
        sbbq    %rdx, %r10
        sbbq    %rbp, %r11
        sbbq    $0x0, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %rbx
        addq    %rbx, %r14
        adcq    $0x0, %r15
        movq    0x68(%rsp), %rbx
        movq    0x120(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r8, %r8
        movq    0x128(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r8, %r8
        movq    0x130(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r8, %r8
        movq    0x138(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r8, %r8
        movq    0x140(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r8, %r8
        movq    0x148(%rsp), %rax
        mulq    %rbx
        subq    %r8, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r8, %r8
        negq    %r8
        movq    %r9, %rbx
        shlq    $0x20, %rbx
        addq    %r9, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r9
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r9, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r10
        sbbq    %rdx, %r11
        sbbq    %rbp, %r12
        sbbq    $0x0, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %rbx
        addq    %rbx, %r15
        adcq    $0x0, %r8
        movq    0x70(%rsp), %rbx
        movq    0x120(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r9, %r9
        movq    0x128(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r9, %r9
        movq    0x130(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r9, %r9
        movq    0x138(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r9, %r9
        movq    0x140(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r9, %r9
        movq    0x148(%rsp), %rax
        mulq    %rbx
        subq    %r9, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r9, %r9
        negq    %r9
        movq    %r10, %rbx
        shlq    $0x20, %rbx
        addq    %r10, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r10
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r10, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r11
        sbbq    %rdx, %r12
        sbbq    %rbp, %r13
        sbbq    $0x0, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %rbx
        addq    %rbx, %r8
        adcq    $0x0, %r9
        movq    0x78(%rsp), %rbx
        movq    0x120(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %r10, %r10
        movq    0x128(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r10, %r10
        movq    0x130(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r10, %r10
        movq    0x138(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r10, %r10
        movq    0x140(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r10, %r10
        movq    0x148(%rsp), %rax
        mulq    %rbx
        subq    %r10, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r10, %r10
        negq    %r10
        movq    %r11, %rbx
        shlq    $0x20, %rbx
        addq    %r11, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r11
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r11, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r12
        sbbq    %rdx, %r13
        sbbq    %rbp, %r14
        sbbq    $0x0, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %rbx
        addq    %rbx, %r9
        adcq    $0x0, %r10
        movq    0x80(%rsp), %rbx
        movq    0x120(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %r11, %r11
        movq    0x128(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r11, %r11
        movq    0x130(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r11, %r11
        movq    0x138(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r11, %r11
        movq    0x140(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r11, %r11
        movq    0x148(%rsp), %rax
        mulq    %rbx
        subq    %r11, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r11, %r11
        negq    %r11
        movq    %r12, %rbx
        shlq    $0x20, %rbx
        addq    %r12, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r12
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r12, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r13
        sbbq    %rdx, %r14
        sbbq    %rbp, %r15
        sbbq    $0x0, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %rbx
        addq    %rbx, %r10
        adcq    $0x0, %r11
        movq    0x88(%rsp), %rbx
        movq    0x120(%rsp), %rax
        mulq    %rbx
        addq    %rax, %r13
        adcq    %rdx, %r14
        sbbq    %r12, %r12
        movq    0x128(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r14
        adcq    %rdx, %r15
        sbbq    %r12, %r12
        movq    0x130(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8
        sbbq    %r12, %r12
        movq    0x138(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %r12, %r12
        movq    0x140(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %r12, %r12
        movq    0x148(%rsp), %rax
        mulq    %rbx
        subq    %r12, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %r12, %r12
        negq    %r12
        movq    %r13, %rbx
        shlq    $0x20, %rbx
        addq    %r13, %rbx
        xorl    %ebp, %ebp
        movabsq $0xffffffff00000001, %rax
        mulq    %rbx
        movq    %rdx, %r13
        movabsq $0xffffffff, %rax
        mulq    %rbx
        addq    %r13, %rax
        adcq    %rbx, %rdx
        adcl    %ebp, %ebp
        subq    %rax, %r14
        sbbq    %rdx, %r15
        sbbq    %rbp, %r8
        sbbq    $0x0, %r9
        sbbq    $0x0, %r10
        sbbq    $0x0, %rbx
        addq    %rbx, %r11
        adcq    $0x0, %r12
        xorl    %edx, %edx
        xorl    %ebp, %ebp
        xorl    %r13d, %r13d
        movabsq $0xffffffff00000001, %rax
        addq    %r14, %rax
        movl    $0xffffffff, %ebx
        adcq    %r15, %rbx
        movl    $0x1, %ecx
        adcq    %r8, %rcx
        adcq    %r9, %rdx
        adcq    %r10, %rbp
        adcq    %r11, %r13
        adcq    $0x0, %r12
        cmovneq %rax, %r14
        cmovneq %rbx, %r15
        cmovneq %rcx, %r8
        cmovneq %rdx, %r9
        cmovneq %rbp, %r10
        cmovneq %r13, %r11
        movq    %r14, 0xf0(%rsp)
        movq    %r15, 0xf8(%rsp)
        movq    %r8, 0x100(%rsp)
        movq    %r9, 0x108(%rsp)
        movq    %r10, 0x110(%rsp)
        movq    %r11, 0x118(%rsp)
        movq    0xb8(%rsp), %rcx
        movq    %rcx, %r13
        shrq    $0x3e, %rcx
        movq    0xb0(%rsp), %r12
        shldq   $0x2, %r12, %r13
        movq    0xa8(%rsp), %r11
        shldq   $0x2, %r11, %r12
        movq    0xa0(%rsp), %r10
        shldq   $0x2, %r10, %r11
        movq    0x98(%rsp), %r9
        shldq   $0x2, %r9, %r10
        movq    0x90(%rsp), %r8
        shldq   $0x2, %r8, %r9
        shlq    $0x2, %r8
        addq    $0x1, %rcx
        subq    0x120(%rsp), %r8
        sbbq    0x128(%rsp), %r9
        sbbq    0x130(%rsp), %r10
        sbbq    0x138(%rsp), %r11
        sbbq    0x140(%rsp), %r12
        sbbq    0x148(%rsp), %r13
        sbbq    $0x0, %rcx
        movabsq $0xffffffff00000001, %rax
        mulq    %rcx
        addq    %rax, %r8
        adcq    %rdx, %r9
        adcq    %rcx, %r10
        movq    %rcx, %rax
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %edx
        negq    %rcx
        mulq    %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        adcq    %rcx, %r11
        adcq    $0x0, %r12
        adcq    $0x0, %r13
        sbbq    %rcx, %rcx
        notq    %rcx
        movl    $0xffffffff, %edx
        xorq    %rax, %rax
        andq    %rcx, %rdx
        subq    %rdx, %rax
        andq    $0x1, %rcx
        subq    %rax, %r8
        movq    %r8, (%rdi)
        sbbq    %rdx, %r9
        movq    %r9, 0x8(%rdi)
        sbbq    %rcx, %r10
        movq    %r10, 0x10(%rdi)
        sbbq    $0x0, %r11
        movq    %r11, 0x18(%rdi)
        sbbq    $0x0, %r12
        movq    %r12, 0x20(%rdi)
        sbbq    $0x0, %r13
        movq    %r13, 0x28(%rdi)
        movabsq $0xffffffff, %r8
        subq    0xc0(%rsp), %r8
        movabsq $0xffffffff00000000, %r9
        sbbq    0xc8(%rsp), %r9
        movq    $0xfffffffffffffffe, %r10
        sbbq    0xd0(%rsp), %r10
        movq    $0xffffffffffffffff, %r11
        sbbq    0xd8(%rsp), %r11
        movq    $0xffffffffffffffff, %r12
        sbbq    0xe0(%rsp), %r12
        movq    $0xffffffffffffffff, %r13
        sbbq    0xe8(%rsp), %r13
        movq    %r13, %r14
        shrq    $0x3d, %r14
        shldq   $0x3, %r12, %r13
        shldq   $0x3, %r11, %r12
        shldq   $0x3, %r10, %r11
        shldq   $0x3, %r9, %r10
        shldq   $0x3, %r8, %r9
        shlq    $0x3, %r8
        addq    $0x1, %r14
        movl    $0x3, %ecx
        movq    0xf0(%rsp), %rax
        mulq    %rcx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %rbx, %rbx
        movq    0xf8(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rbx, %rbx
        movq    0x100(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rbx, %rbx
        movq    0x108(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r11
        adcq    %rdx, %r12
        sbbq    %rbx, %rbx
        movq    0x110(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r12
        adcq    %rdx, %r13
        sbbq    %rbx, %rbx
        movq    0x118(%rsp), %rax
        mulq    %rcx
        subq    %rbx, %rdx
        addq    %rax, %r13
        adcq    %rdx, %r14
        movabsq $0xffffffff00000001, %rax
        mulq    %r14
        addq    %rax, %r8
        adcq    %rdx, %r9
        adcq    %r14, %r10
        movq    %r14, %rax
        sbbq    %rcx, %rcx
        movl    $0xffffffff, %edx
        negq    %rcx
        mulq    %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        adcq    %rcx, %r11
        adcq    $0x0, %r12
        adcq    $0x0, %r13
        sbbq    %rcx, %rcx
        notq    %rcx
        movl    $0xffffffff, %edx
        xorq    %rax, %rax
        andq    %rcx, %rdx
        subq    %rdx, %rax
        andq    $0x1, %rcx
        subq    %rax, %r8
        movq    %r8, 0x30(%rdi)
        sbbq    %rdx, %r9
        movq    %r9, 0x38(%rdi)
        sbbq    %rcx, %r10
        movq    %r10, 0x40(%rdi)
        sbbq    $0x0, %r11
        movq    %r11, 0x48(%rdi)
        sbbq    $0x0, %r12
        movq    %r12, 0x50(%rdi)
        sbbq    $0x0, %r13
        movq    %r13, 0x58(%rdi)
        CFI_INC_RSP(344)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp384_montjscalarmul_alt_p384_montjdouble)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
