// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Jacobian form scalar multiplication for P-521
// Input scalar[9], point[27]; output res[27]
//
// extern void p521_jscalarmul
//   (uint64_t res[static 27],
//    const uint64_t scalar[static 9],
//    const uint64_t point[static 27]);
//
// This function is a variant of its affine point version p521_scalarmul.
// Here, input and output points are assumed to be in Jacobian form with
// a triple (x,y,z) representing the affine point (x/z^2,y/z^3) when
// z is nonzero or the point at infinity (group identity) if z = 0.
//
// Given scalar = n and point = P, assumed to be on the NIST elliptic
// curve P-521, returns a representation of n * P. If the result is the
// point at infinity (either because the input point was or because the
// scalar was a multiple of p_521) then the output is guaranteed to
// represent the point at infinity, i.e. to have its z coordinate zero.
//
// Standard x86-64 ABI: RDI = res, RSI = scalar, RDX = point
// Microsoft x64 ABI:   RCX = res, RDX = scalar, R8 = point
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p521_jscalarmul)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p521_jscalarmul)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p521_jscalarmul)


        .text
        .balign 4

// Size of individual field elements

#define NUMSIZE 72
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

#define selectblock(I,C)                                        \
        cmpq    $I, %rdi ;                                         \
        cmovzq  TAB+JACSIZE*(I-1)+C*NUMSIZE(%rsp), %rax ;         \
        cmovzq  TAB+JACSIZE*(I-1)+8+C*NUMSIZE(%rsp), %rbx ;       \
        cmovzq  TAB+JACSIZE*(I-1)+16+C*NUMSIZE(%rsp), %rcx ;      \
        cmovzq  TAB+JACSIZE*(I-1)+24+C*NUMSIZE(%rsp), %rdx ;      \
        cmovzq  TAB+JACSIZE*(I-1)+32+C*NUMSIZE(%rsp), %r8 ;       \
        cmovzq  TAB+JACSIZE*(I-1)+40+C*NUMSIZE(%rsp), %r9 ;       \
        cmovzq  TAB+JACSIZE*(I-1)+48+C*NUMSIZE(%rsp), %r10 ;      \
        cmovzq  TAB+JACSIZE*(I-1)+56+C*NUMSIZE(%rsp), %r11 ;      \
        cmovzq  TAB+JACSIZE*(I-1)+64+C*NUMSIZE(%rsp), %r12

S2N_BN_SYMBOL(p521_jscalarmul):
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
        CFI_CALL(Lp521_jscalarmul_standard)
        CFI_POP(%rsi)
        CFI_POP(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(p521_jscalarmul)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_standard)

Lp521_jscalarmul_standard:
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

// Reduce the input scalar mod n_521 and store it to "scalarb".

        movq    %rdx, %rbx
        leaq    SCALARB(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_bignum_mod_n521_9)

// Set the tab[0] table entry to the input point = 1 * P, but also
// reduce all coordinates modulo p. In principle we assume reduction
// as a precondition, but this reduces the scope for surprise, e.g.
// making sure that any input with z = 0 is treated as zero, even
// if the other coordinates are not in fact reduced.

        leaq    TAB(%rsp), %rdi
        movq    %rbx, %rsi
        CFI_CALL(Lp521_jscalarmul_bignum_mod_p521_9)

        leaq    TAB+NUMSIZE(%rsp), %rdi
        leaq    NUMSIZE(%rbx), %rsi
        CFI_CALL(Lp521_jscalarmul_bignum_mod_p521_9)

        leaq    TAB+2*NUMSIZE(%rsp), %rdi
        leaq    2*NUMSIZE(%rbx), %rsi
        CFI_CALL(Lp521_jscalarmul_bignum_mod_p521_9)

// If bit 520 of the scalar is set, then negate the scalar mod n_521,
// i.e. do scalar |-> n_521 - scalar, and also the point to compensate
// by negating its y coordinate. This further step is not needed by
// the indexing scheme (the top window is only a couple of bits either
// way), but is convenient to exclude a problem with the specific value
// scalar = n_521 - 18, where the last Jacobian addition is of the form
// (n_521 - 9) * P + -(9 * P) and hence is a degenerate doubling case.

        xorl    %eax, %eax
        notq    %rax
        movq    $0xbb6fb71e91386409, %r8
        subq    SCALARB(%rsp), %r8
        movq    $0x3bb5c9b8899c47ae, %r9
        sbbq    SCALARB+8(%rsp), %r9
        movq    $0x7fcc0148f709a5d0, %r10
        sbbq    SCALARB+16(%rsp), %r10
        movq    $0x51868783bf2f966b, %r11
        sbbq    SCALARB+24(%rsp), %r11
        leaq    -5(%rax), %r12
        sbbq    SCALARB+32(%rsp), %r12
        movq    %rax, %r13
        sbbq    SCALARB+40(%rsp), %r13
        movq    %rax, %r14
        sbbq    SCALARB+48(%rsp), %r14
        movq    %rax, %r15
        sbbq    SCALARB+56(%rsp), %r15
        movq    $0x1ff, %rax
        movq    SCALARB+64(%rsp), %rcx
        sbbq    %rcx, %rax

        btq     $8, %rcx
        sbbq    %rcx, %rcx

        cmovncq SCALARB(%rsp), %r8
        cmovncq SCALARB+8(%rsp), %r9
        cmovncq SCALARB+16(%rsp), %r10
        cmovncq SCALARB+24(%rsp), %r11
        cmovncq SCALARB+32(%rsp), %r12
        cmovncq SCALARB+40(%rsp), %r13
        cmovncq SCALARB+48(%rsp), %r14
        cmovncq SCALARB+56(%rsp), %r15
        cmovncq SCALARB+64(%rsp), %rax

        movq    %r8, SCALARB(%rsp)
        movq    %r9, SCALARB+8(%rsp)
        movq    %r10, SCALARB+16(%rsp)
        movq    %r11, SCALARB+24(%rsp)
        movq    %r12, SCALARB+32(%rsp)
        movq    %r13, SCALARB+40(%rsp)
        movq    %r14, SCALARB+48(%rsp)
        movq    %r15, SCALARB+56(%rsp)
        movq    %rax, SCALARB+64(%rsp)

        movq    TAB+NUMSIZE(%rsp), %r8
        movq    TAB+NUMSIZE+8(%rsp), %r9
        movq    TAB+NUMSIZE+16(%rsp), %r10
        movq    TAB+NUMSIZE+24(%rsp), %r11
        movq    TAB+NUMSIZE+32(%rsp), %r12
        movq    TAB+NUMSIZE+40(%rsp), %r13
        movq    TAB+NUMSIZE+48(%rsp), %r14
        movq    TAB+NUMSIZE+56(%rsp), %r15
        movq    TAB+NUMSIZE+64(%rsp), %rax

        movq    %r8, %rbx
        movq    %r12, %rbp
        orq     %r9, %rbx
        orq     %r13, %rbp
        orq     %r10, %rbx
        orq     %r14, %rbp
        orq     %r11, %rbx
        orq     %r15, %rbp
        orq     %rbp, %rbx
        orq     %rax, %rbx
        cmovzq  %rbx, %rcx

        xorq    %rcx, %r8
        xorq    %rcx, %r9
        xorq    %rcx, %r10
        xorq    %rcx, %r11
        xorq    %rcx, %r12
        xorq    %rcx, %r13
        xorq    %rcx, %r14
        xorq    %rcx, %r15
        andq    $0x1FF, %rcx
        xorq    %rcx, %rax

        movq    %r8, TAB+NUMSIZE(%rsp)
        movq    %r9, TAB+NUMSIZE+8(%rsp)
        movq    %r10, TAB+NUMSIZE+16(%rsp)
        movq    %r11, TAB+NUMSIZE+24(%rsp)
        movq    %r12, TAB+NUMSIZE+32(%rsp)
        movq    %r13, TAB+NUMSIZE+40(%rsp)
        movq    %r14, TAB+NUMSIZE+48(%rsp)
        movq    %r15, TAB+NUMSIZE+56(%rsp)
        movq    %rax, TAB+NUMSIZE+64(%rsp)

// Compute and record tab[1] = 2 * p, ..., tab[15] = 16 * P

        leaq    TAB+JACSIZE*1(%rsp), %rdi
        leaq    TAB(%rsp), %rsi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    TAB+JACSIZE*2(%rsp), %rdi
        leaq    TAB+JACSIZE*1(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp521_jscalarmul_jadd)

        leaq    TAB+JACSIZE*3(%rsp), %rdi
        leaq    TAB+JACSIZE*1(%rsp), %rsi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    TAB+JACSIZE*4(%rsp), %rdi
        leaq    TAB+JACSIZE*3(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp521_jscalarmul_jadd)

        leaq    TAB+JACSIZE*5(%rsp), %rdi
        leaq    TAB+JACSIZE*2(%rsp), %rsi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    TAB+JACSIZE*6(%rsp), %rdi
        leaq    TAB+JACSIZE*5(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp521_jscalarmul_jadd)

        leaq    TAB+JACSIZE*7(%rsp), %rdi
        leaq    TAB+JACSIZE*3(%rsp), %rsi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    TAB+JACSIZE*8(%rsp), %rdi
        leaq    TAB+JACSIZE*7(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp521_jscalarmul_jadd)

        leaq    TAB+JACSIZE*9(%rsp), %rdi
        leaq    TAB+JACSIZE*4(%rsp), %rsi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    TAB+JACSIZE*10(%rsp), %rdi
        leaq    TAB+JACSIZE*9(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp521_jscalarmul_jadd)

        leaq    TAB+JACSIZE*11(%rsp), %rdi
        leaq    TAB+JACSIZE*5(%rsp), %rsi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    TAB+JACSIZE*12(%rsp), %rdi
        leaq    TAB+JACSIZE*11(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp521_jscalarmul_jadd)

        leaq    TAB+JACSIZE*13(%rsp), %rdi
        leaq    TAB+JACSIZE*6(%rsp), %rsi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    TAB+JACSIZE*14(%rsp), %rdi
        leaq    TAB+JACSIZE*13(%rsp), %rsi
        leaq    TAB(%rsp), %rdx
        CFI_CALL(Lp521_jscalarmul_jadd)

        leaq    TAB+JACSIZE*15(%rsp), %rdi
        leaq    TAB+JACSIZE*7(%rsp), %rsi
        CFI_CALL(Lp521_jscalarmul_jdouble)

// Add the recoding constant sum_i(16 * 32^i) to the scalar to allow signed
// digits. The digits of the constant, in lowest-to-highest order, are as
// follows; they are generated dynamically to use fewer large constant loads.
//
// 0x0842108421084210 %rax
// 0x1084210842108421 %rbx
// 0x2108421084210842 %rbx<<1
// 0x4210842108421084 %rbx<<2
// 0x8421084210842108 %rbx<<3
// 0x0842108421084210 %rax
// 0x1084210842108421 %rbx
// 0x2108421084210842 %rbx<<1
// 0x0000000000000084

        movq    $0x1084210842108421, %rax
        movq    %rax, %rbx
        shrq    $1, %rax
        movq    SCALARB(%rsp), %r8
        addq    %rax, %r8
        movq    SCALARB+8(%rsp), %r9
        adcq    %rbx, %r9
        leaq    (%rbx,%rbx), %rcx
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
        movq    SCALARB+48(%rsp), %r14
        adcq    %rbx, %r14
        movq    SCALARB+56(%rsp), %r15
        leaq    (%rbx,%rbx), %rcx
        adcq    %rcx, %r15
        movq    SCALARB+64(%rsp), %rax
        adcq    $0x84, %rax

// Because of the initial reduction the top bitfield (>= bits 520) is <= 1,
// i.e. just a single bit. Record that in %rdi, then shift the whole
// scalar left 56 bits to align the top of the next bitfield with the MSB
// (bits 571..575).

        movq    %rax, %rdi
        shrq    $8, %rdi
        shldq   $56, %r15, %rax
        shldq   $56, %r14, %r15
        shldq   $56, %r13, %r14
        shldq   $56, %r12, %r13
        shldq   $56, %r11, %r12
        shldq   $56, %r10, %r11
        shldq   $56, %r9, %r10
        shldq   $56, %r8, %r9
        shlq    $56, %r8

        movq    %r8, SCALARB(%rsp)
        movq    %r9, SCALARB+8(%rsp)
        movq    %r10, SCALARB+16(%rsp)
        movq    %r11, SCALARB+24(%rsp)
        movq    %r12, SCALARB+32(%rsp)
        movq    %r13, SCALARB+40(%rsp)
        movq    %r14, SCALARB+48(%rsp)
        movq    %r15, SCALARB+56(%rsp)
        movq    %rax, SCALARB+64(%rsp)

// According to the top bit, initialize the accumulator to P or 0. This top
// digit, uniquely, is not recoded so there is no sign adjustment to make.
// We only really need to adjust the z coordinate to zero, but do all three.

        xorl    %ecx, %ecx
        testq   %rdi, %rdi

        movq    TAB(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC(%rsp)
        movq    TAB+8(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+8(%rsp)
        movq    TAB+16(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+16(%rsp)
        movq    TAB+24(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+24(%rsp)
        movq    TAB+32(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+32(%rsp)
        movq    TAB+40(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+40(%rsp)
        movq    TAB+48(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+48(%rsp)
        movq    TAB+56(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+56(%rsp)
        movq    TAB+64(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+64(%rsp)
        movq    TAB+72(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+72(%rsp)
        movq    TAB+80(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+80(%rsp)
        movq    TAB+88(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+88(%rsp)
        movq    TAB+96(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+96(%rsp)
        movq    TAB+104(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+104(%rsp)
        movq    TAB+112(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+112(%rsp)
        movq    TAB+120(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+120(%rsp)
        movq    TAB+128(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+128(%rsp)
        movq    TAB+136(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+136(%rsp)
        movq    TAB+144(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+144(%rsp)
        movq    TAB+152(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+152(%rsp)
        movq    TAB+160(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+160(%rsp)
        movq    TAB+168(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+168(%rsp)
        movq    TAB+176(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+176(%rsp)
        movq    TAB+184(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+184(%rsp)
        movq    TAB+192(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+192(%rsp)
        movq    TAB+200(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+200(%rsp)
        movq    TAB+208(%rsp), %rax
        cmovzq  %rcx, %rax
        movq    %rax, ACC+208(%rsp)

// Main loop over size-5 bitfields: double 5 times then add signed digit
// At each stage we shift the scalar left by 5 bits so we can simply pick
// the top 5 bits as the bitfield, saving some fiddle over indexing.

        movl    $520, %ebp

Lp521_jscalarmul_mainloop:
        subq    $5, %rbp

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_jdouble)

        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_jdouble)

// Choose the bitfield and adjust it to sign and magnitude

        movq    SCALARB(%rsp), %r8
        movq    SCALARB+8(%rsp), %r9
        movq    SCALARB+16(%rsp), %r10
        movq    SCALARB+24(%rsp), %r11
        movq    SCALARB+32(%rsp), %r12
        movq    SCALARB+40(%rsp), %r13
        movq    SCALARB+48(%rsp), %r14
        movq    SCALARB+56(%rsp), %r15
        movq    SCALARB+64(%rsp), %rax


        movq    %rax, %rdi
        shrq    $59, %rdi

        shldq   $5, %r15, %rax
        shldq   $5, %r14, %r15
        shldq   $5, %r13, %r14
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
        movq    %r14, SCALARB+48(%rsp)
        movq    %r15, SCALARB+56(%rsp)
        movq    %rax, SCALARB+64(%rsp)

        subq    $16, %rdi
        sbbq    %rsi, %rsi // %rsi = sign of digit (-1 = negative)
        xorq    %rsi, %rdi
        subq    %rsi, %rdi // %rdi = absolute value of digit

// Conditionally select the table entry tab[i-1] = i * P in constant time
// Again, this is done in separate sweeps per coordinate, doing y last.

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        xorl    %r8d, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d
        xorl    %r12d, %r12d
        selectblock(1,0)
        selectblock(2,0)
        selectblock(3,0)
        selectblock(4,0)
        selectblock(5,0)
        selectblock(6,0)
        selectblock(7,0)
        selectblock(8,0)
        selectblock(9,0)
        selectblock(10,0)
        selectblock(11,0)
        selectblock(12,0)
        selectblock(13,0)
        selectblock(14,0)
        selectblock(15,0)
        selectblock(16,0)
        movq    %rax, TABENT(%rsp)
        movq    %rbx, TABENT+8(%rsp)
        movq    %rcx, TABENT+16(%rsp)
        movq    %rdx, TABENT+24(%rsp)
        movq    %r8, TABENT+32(%rsp)
        movq    %r9, TABENT+40(%rsp)
        movq    %r10, TABENT+48(%rsp)
        movq    %r11, TABENT+56(%rsp)
        movq    %r12, TABENT+64(%rsp)

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        xorl    %r8d, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d
        xorl    %r12d, %r12d
        selectblock(1,2)
        selectblock(2,2)
        selectblock(3,2)
        selectblock(4,2)
        selectblock(5,2)
        selectblock(6,2)
        selectblock(7,2)
        selectblock(8,2)
        selectblock(9,2)
        selectblock(10,2)
        selectblock(11,2)
        selectblock(12,2)
        selectblock(13,2)
        selectblock(14,2)
        selectblock(15,2)
        selectblock(16,2)
        movq    %rax, TABENT+2*NUMSIZE(%rsp)
        movq    %rbx, TABENT+2*NUMSIZE+8(%rsp)
        movq    %rcx, TABENT+2*NUMSIZE+16(%rsp)
        movq    %rdx, TABENT+2*NUMSIZE+24(%rsp)
        movq    %r8, TABENT+2*NUMSIZE+32(%rsp)
        movq    %r9, TABENT+2*NUMSIZE+40(%rsp)
        movq    %r10, TABENT+2*NUMSIZE+48(%rsp)
        movq    %r11, TABENT+2*NUMSIZE+56(%rsp)
        movq    %r12, TABENT+2*NUMSIZE+64(%rsp)

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        xorl    %r8d, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d
        xorl    %r12d, %r12d
        selectblock(1,1)
        selectblock(2,1)
        selectblock(3,1)
        selectblock(4,1)
        selectblock(5,1)
        selectblock(6,1)
        selectblock(7,1)
        selectblock(8,1)
        selectblock(9,1)
        selectblock(10,1)
        selectblock(11,1)
        selectblock(12,1)
        selectblock(13,1)
        selectblock(14,1)
        selectblock(15,1)
        selectblock(16,1)

// Store it to "tabent" with the y coordinate optionally negated.
// This is done carefully to give coordinates < p_521 even in
// the degenerate case y = 0 (when z = 0 for points on the curve).

        movq    %rax, %r13
        orq     %rbx, %r13
        movq    %rcx, %r14
        orq     %rdx, %r14
        movq    %r8, %r15
        orq     %r9, %r15
        movq    %r10, %rdi
        orq     %r11, %rdi
        orq     %r14, %r13
        orq     %rdi, %r15
        orq     %r12, %r15
        orq     %r15, %r13
        cmovzq  %r13, %rsi

        xorq    %rsi, %rax
        xorq    %rsi, %rbx
        xorq    %rsi, %rcx
        xorq    %rsi, %rdx
        xorq    %rsi, %r8
        xorq    %rsi, %r9
        xorq    %rsi, %r10
        xorq    %rsi, %r11
        andq    $0x1FF, %rsi
        xorq    %rsi, %r12

        movq    %rax, TABENT+NUMSIZE(%rsp)
        movq    %rbx, TABENT+NUMSIZE+8(%rsp)
        movq    %rcx, TABENT+NUMSIZE+16(%rsp)
        movq    %rdx, TABENT+NUMSIZE+24(%rsp)
        movq    %r8, TABENT+NUMSIZE+32(%rsp)
        movq    %r9, TABENT+NUMSIZE+40(%rsp)
        movq    %r10, TABENT+NUMSIZE+48(%rsp)
        movq    %r11, TABENT+NUMSIZE+56(%rsp)
        movq    %r12, TABENT+NUMSIZE+64(%rsp)

// Add to the accumulator

        leaq    TABENT(%rsp), %rdx
        leaq    ACC(%rsp), %rsi
        leaq    ACC(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_jadd)

        testq   %rbp, %rbp
        jne     Lp521_jscalarmul_mainloop

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
        movq    ACC+144(%rsp), %rax
        movq    %rax, 144(%rdi)
        movq    ACC+152(%rsp), %rax
        movq    %rax, 152(%rdi)
        movq    ACC+160(%rsp), %rax
        movq    %rax, 160(%rdi)
        movq    ACC+168(%rsp), %rax
        movq    %rax, 168(%rdi)
        movq    ACC+176(%rsp), %rax
        movq    %rax, 176(%rdi)
        movq    ACC+184(%rsp), %rax
        movq    %rax, 184(%rdi)
        movq    ACC+192(%rsp), %rax
        movq    %rax, 192(%rdi)
        movq    ACC+200(%rsp), %rax
        movq    %rax, 200(%rdi)
        movq    ACC+208(%rsp), %rax
        movq    %rax, 208(%rdi)

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
S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_standard)
#else
S2N_BN_SIZE_DIRECTIVE(p521_jscalarmul)
#endif

// Local copies of subroutines, complete clones at the moment

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_bignum_mod_p521_9)

Lp521_jscalarmul_bignum_mod_p521_9:
        CFI_START
        CFI_PUSH(%rbx)
        movq    0x40(%rsi), %rax
        movl    $0x1ff, %edx
        andq    %rax, %rdx
        shrq    $0x9, %rax
        stc
        adcq    (%rsi), %rax
        movq    0x8(%rsi), %rcx
        adcq    $0x0, %rcx
        movq    0x10(%rsi), %r8
        adcq    $0x0, %r8
        movq    0x18(%rsi), %r9
        adcq    $0x0, %r9
        movq    0x20(%rsi), %r10
        adcq    $0x0, %r10
        movq    0x28(%rsi), %r11
        adcq    $0x0, %r11
        movq    0x30(%rsi), %rbx
        adcq    $0x0, %rbx
        movq    0x38(%rsi), %rsi
        adcq    $0x0, %rsi
        adcq    $0x0, %rdx
        cmpq    $0x200, %rdx
        sbbq    $0x0, %rax
        movq    %rax, (%rdi)
        sbbq    $0x0, %rcx
        movq    %rcx, 0x8(%rdi)
        sbbq    $0x0, %r8
        movq    %r8, 0x10(%rdi)
        sbbq    $0x0, %r9
        movq    %r9, 0x18(%rdi)
        sbbq    $0x0, %r10
        movq    %r10, 0x20(%rdi)
        sbbq    $0x0, %r11
        movq    %r11, 0x28(%rdi)
        sbbq    $0x0, %rbx
        movq    %rbx, 0x30(%rdi)
        sbbq    $0x0, %rsi
        movq    %rsi, 0x38(%rdi)
        sbbq    $0x0, %rdx
        andq    $0x1ff, %rdx
        movq    %rdx, 0x40(%rdi)
        CFI_POP(%rbx)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_bignum_mod_p521_9)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_bignum_mod_n521_9)

Lp521_jscalarmul_bignum_mod_n521_9:
        CFI_START
        movq    0x40(%rsi), %rdx
        movq    $0xfffffffffffffe00, %rax
        orq     %rdx, %rax
        movq    %rax, 0x40(%rdi)
        shrq    $0x9, %rdx
        addq    $0x1, %rdx
        movq    $0x449048e16ec79bf7, %r9
        mulxq   %r9, %rax, %rcx
        adcxq   (%rsi), %rax
        movq    %rax, (%rdi)
        movq    $0xc44a36477663b851, %r10
        mulxq   %r10, %rax, %r8
        adcxq   0x8(%rsi), %rax
        adoxq   %rcx, %rax
        movq    %rax, 0x8(%rdi)
        movq    $0x8033feb708f65a2f, %r11
        mulxq   %r11, %rax, %rcx
        adcxq   0x10(%rsi), %rax
        adoxq   %r8, %rax
        movq    %rax, 0x10(%rdi)
        movq    $0xae79787c40d06994, %rax
        mulxq   %rax, %rax, %r8
        adcxq   0x18(%rsi), %rax
        adoxq   %rcx, %rax
        movq    %rax, 0x18(%rdi)
        movl    $0x5, %eax
        mulxq   %rax, %rax, %rcx
        adcxq   0x20(%rsi), %rax
        adoxq   %r8, %rax
        movq    %rax, 0x20(%rdi)
        movq    %rcx, %rax
        adoxq   %rcx, %rcx
        adcq    0x28(%rsi), %rcx
        movq    %rcx, 0x28(%rdi)
        movq    0x30(%rsi), %rcx
        adcq    %rax, %rcx
        movq    %rcx, 0x30(%rdi)
        movq    0x38(%rsi), %rcx
        adcq    %rax, %rcx
        movq    %rcx, 0x38(%rdi)
        movq    0x40(%rdi), %rcx
        adcq    %rax, %rcx
        cmc
        sbbq    %rdx, %rdx
        andq    %rdx, %r9
        andq    %rdx, %r10
        andq    %rdx, %r11
        movq    $0xae79787c40d06994, %r8
        andq    %rdx, %r8
        andl    $0x5, %edx
        subq    %r9, (%rdi)
        sbbq    %r10, 0x8(%rdi)
        sbbq    %r11, 0x10(%rdi)
        sbbq    %r8, 0x18(%rdi)
        sbbq    %rdx, 0x20(%rdi)
        sbbq    %rax, 0x28(%rdi)
        sbbq    %rax, 0x30(%rdi)
        sbbq    %rax, 0x38(%rdi)
        sbbl    %eax, %ecx
        andl    $0x1ff, %ecx
        movq    %rcx, 0x40(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_bignum_mod_n521_9)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_jadd)

Lp521_jscalarmul_jadd:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(528)
        movq    %rdi, 0x1f8(%rsp)
        movq    %rsi, 0x200(%rsp)
        movq    %rdx, 0x208(%rsp)
        movq    0x200(%rsp), %rsi
        leaq    0x90(%rsi), %rsi
        leaq    (%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_sqr_p521)
        movq    0x208(%rsp), %rdi
        leaq    0x90(%rdi), %rsi
        leaq    0x168(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_sqr_p521)
        movq    0x200(%rsp), %rsi
        movq    0x208(%rsp), %rdi
        leaq    0x48(%rsi), %rdx
        leaq    0x90(%rdi), %rsi
        leaq    0x1b0(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    0x200(%rsp), %rsi
        movq    0x208(%rsp), %rdi
        leaq    0x48(%rdi), %rdx
        leaq    0x90(%rsi), %rsi
        leaq    0x48(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    0x208(%rsp), %rdi
        leaq    (%rdi), %rdx
        leaq    (%rsp), %rsi
        leaq    0x90(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    0x200(%rsp), %rsi
        leaq    (%rsi), %rdx
        leaq    0x168(%rsp), %rsi
        leaq    0x120(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        leaq    0x48(%rsp), %rdx
        leaq    (%rsp), %rsi
        leaq    0x48(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        leaq    0x1b0(%rsp), %rdx
        leaq    0x168(%rsp), %rsi
        leaq    0x1b0(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    0x90(%rsp), %rax
        subq    0x120(%rsp), %rax
        movq    0x98(%rsp), %rdx
        sbbq    0x128(%rsp), %rdx
        movq    0xa0(%rsp), %r8
        sbbq    0x130(%rsp), %r8
        movq    0xa8(%rsp), %r9
        sbbq    0x138(%rsp), %r9
        movq    0xb0(%rsp), %r10
        sbbq    0x140(%rsp), %r10
        movq    0xb8(%rsp), %r11
        sbbq    0x148(%rsp), %r11
        movq    0xc0(%rsp), %r12
        sbbq    0x150(%rsp), %r12
        movq    0xc8(%rsp), %r13
        sbbq    0x158(%rsp), %r13
        movq    0xd0(%rsp), %r14
        sbbq    0x160(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, 0x168(%rsp)
        sbbq    $0x0, %rdx
        movq    %rdx, 0x170(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x178(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x180(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x188(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x190(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x198(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x1a0(%rsp)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0x1a8(%rsp)
        movq    0x48(%rsp), %rax
        subq    0x1b0(%rsp), %rax
        movq    0x50(%rsp), %rdx
        sbbq    0x1b8(%rsp), %rdx
        movq    0x58(%rsp), %r8
        sbbq    0x1c0(%rsp), %r8
        movq    0x60(%rsp), %r9
        sbbq    0x1c8(%rsp), %r9
        movq    0x68(%rsp), %r10
        sbbq    0x1d0(%rsp), %r10
        movq    0x70(%rsp), %r11
        sbbq    0x1d8(%rsp), %r11
        movq    0x78(%rsp), %r12
        sbbq    0x1e0(%rsp), %r12
        movq    0x80(%rsp), %r13
        sbbq    0x1e8(%rsp), %r13
        movq    0x88(%rsp), %r14
        sbbq    0x1f0(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, 0x48(%rsp)
        sbbq    $0x0, %rdx
        movq    %rdx, 0x50(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x58(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x60(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x68(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x70(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x78(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x80(%rsp)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0x88(%rsp)
        leaq    0x168(%rsp), %rsi
        leaq    0xd8(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_sqr_p521)
        leaq    0x48(%rsp), %rsi
        leaq    (%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_sqr_p521)
        leaq    0x120(%rsp), %rdx
        leaq    0xd8(%rsp), %rsi
        leaq    0x120(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        leaq    0x90(%rsp), %rdx
        leaq    0xd8(%rsp), %rsi
        leaq    0x90(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    (%rsp), %rax
        subq    0x120(%rsp), %rax
        movq    0x8(%rsp), %rdx
        sbbq    0x128(%rsp), %rdx
        movq    0x10(%rsp), %r8
        sbbq    0x130(%rsp), %r8
        movq    0x18(%rsp), %r9
        sbbq    0x138(%rsp), %r9
        movq    0x20(%rsp), %r10
        sbbq    0x140(%rsp), %r10
        movq    0x28(%rsp), %r11
        sbbq    0x148(%rsp), %r11
        movq    0x30(%rsp), %r12
        sbbq    0x150(%rsp), %r12
        movq    0x38(%rsp), %r13
        sbbq    0x158(%rsp), %r13
        movq    0x40(%rsp), %r14
        sbbq    0x160(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, (%rsp)
        sbbq    $0x0, %rdx
        movq    %rdx, 0x8(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x10(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x18(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x20(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x28(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x30(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x38(%rsp)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0x40(%rsp)
        movq    0x90(%rsp), %rax
        subq    0x120(%rsp), %rax
        movq    0x98(%rsp), %rdx
        sbbq    0x128(%rsp), %rdx
        movq    0xa0(%rsp), %r8
        sbbq    0x130(%rsp), %r8
        movq    0xa8(%rsp), %r9
        sbbq    0x138(%rsp), %r9
        movq    0xb0(%rsp), %r10
        sbbq    0x140(%rsp), %r10
        movq    0xb8(%rsp), %r11
        sbbq    0x148(%rsp), %r11
        movq    0xc0(%rsp), %r12
        sbbq    0x150(%rsp), %r12
        movq    0xc8(%rsp), %r13
        sbbq    0x158(%rsp), %r13
        movq    0xd0(%rsp), %r14
        sbbq    0x160(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, 0xd8(%rsp)
        sbbq    $0x0, %rdx
        movq    %rdx, 0xe0(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0xe8(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0xf0(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0xf8(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x100(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x108(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x110(%rsp)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0x118(%rsp)
        movq    0x200(%rsp), %rsi
        leaq    0x90(%rsi), %rdx
        leaq    0x168(%rsp), %rsi
        leaq    0x168(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    (%rsp), %rax
        subq    0x90(%rsp), %rax
        movq    0x8(%rsp), %rdx
        sbbq    0x98(%rsp), %rdx
        movq    0x10(%rsp), %r8
        sbbq    0xa0(%rsp), %r8
        movq    0x18(%rsp), %r9
        sbbq    0xa8(%rsp), %r9
        movq    0x20(%rsp), %r10
        sbbq    0xb0(%rsp), %r10
        movq    0x28(%rsp), %r11
        sbbq    0xb8(%rsp), %r11
        movq    0x30(%rsp), %r12
        sbbq    0xc0(%rsp), %r12
        movq    0x38(%rsp), %r13
        sbbq    0xc8(%rsp), %r13
        movq    0x40(%rsp), %r14
        sbbq    0xd0(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, (%rsp)
        sbbq    $0x0, %rdx
        movq    %rdx, 0x8(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x10(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x18(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x20(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x28(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x30(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x38(%rsp)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0x40(%rsp)
        movq    0x120(%rsp), %rax
        subq    (%rsp), %rax
        movq    0x128(%rsp), %rdx
        sbbq    0x8(%rsp), %rdx
        movq    0x130(%rsp), %r8
        sbbq    0x10(%rsp), %r8
        movq    0x138(%rsp), %r9
        sbbq    0x18(%rsp), %r9
        movq    0x140(%rsp), %r10
        sbbq    0x20(%rsp), %r10
        movq    0x148(%rsp), %r11
        sbbq    0x28(%rsp), %r11
        movq    0x150(%rsp), %r12
        sbbq    0x30(%rsp), %r12
        movq    0x158(%rsp), %r13
        sbbq    0x38(%rsp), %r13
        movq    0x160(%rsp), %r14
        sbbq    0x40(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, 0x120(%rsp)
        sbbq    $0x0, %rdx
        movq    %rdx, 0x128(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x130(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x138(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x140(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x148(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x150(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x158(%rsp)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0x160(%rsp)
        leaq    0x1b0(%rsp), %rdx
        leaq    0xd8(%rsp), %rsi
        leaq    0xd8(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    0x208(%rsp), %rdi
        leaq    0x90(%rdi), %rdx
        leaq    0x168(%rsp), %rsi
        leaq    0x168(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        leaq    0x120(%rsp), %rdx
        leaq    0x48(%rsp), %rsi
        leaq    0x120(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    0x120(%rsp), %rax
        subq    0xd8(%rsp), %rax
        movq    0x128(%rsp), %rdx
        sbbq    0xe0(%rsp), %rdx
        movq    0x130(%rsp), %r8
        sbbq    0xe8(%rsp), %r8
        movq    0x138(%rsp), %r9
        sbbq    0xf0(%rsp), %r9
        movq    0x140(%rsp), %r10
        sbbq    0xf8(%rsp), %r10
        movq    0x148(%rsp), %r11
        sbbq    0x100(%rsp), %r11
        movq    0x150(%rsp), %r12
        sbbq    0x108(%rsp), %r12
        movq    0x158(%rsp), %r13
        sbbq    0x110(%rsp), %r13
        movq    0x160(%rsp), %r14
        sbbq    0x118(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, 0x120(%rsp)
        sbbq    $0x0, %rdx
        movq    %rdx, 0x128(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x130(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x138(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x140(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x148(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x150(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x158(%rsp)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0x160(%rsp)
        movq    0x200(%rsp), %rsi
        movq    0x90(%rsi), %r8
        movq    0x98(%rsi), %r9
        movq    0xa0(%rsi), %r10
        movq    0xa8(%rsi), %r11
        movq    0xb0(%rsi), %r12
        movq    0xb8(%rsi), %r13
        movq    0xc0(%rsi), %r14
        movq    0xc8(%rsi), %r15
        movq    0xd0(%rsi), %rbp
        orq     %r9, %r8
        orq     %r11, %r10
        orq     %r13, %r12
        orq     %r15, %r14
        orq     %r10, %r8
        orq     %r14, %r12
        orq     %rbp, %r8
        orq     %r12, %r8
        negq    %r8
        sbbq    %rax, %rax
        movq    0x208(%rsp), %rdi
        movq    0x90(%rdi), %r8
        movq    0x98(%rdi), %r9
        movq    0xa0(%rdi), %r10
        movq    0xa8(%rdi), %r11
        movq    0xb0(%rdi), %r12
        movq    0xb8(%rdi), %r13
        movq    0xc0(%rdi), %r14
        movq    0xc8(%rdi), %r15
        movq    0xd0(%rdi), %rbp
        orq     %r9, %r8
        orq     %r11, %r10
        orq     %r13, %r12
        orq     %r15, %r14
        orq     %r10, %r8
        orq     %r14, %r12
        orq     %rbp, %r8
        orq     %r12, %r8
        negq    %r8
        sbbq    %rdx, %rdx
        cmpq    %rax, %rdx
        movq    0x120(%rsp), %r8
        cmovbq  0x48(%rsi), %r8
        cmova   0x48(%rdi), %r8
        movq    0x128(%rsp), %r9
        cmovbq  0x50(%rsi), %r9
        cmova   0x50(%rdi), %r9
        movq    0x130(%rsp), %r10
        cmovbq  0x58(%rsi), %r10
        cmova   0x58(%rdi), %r10
        movq    0x138(%rsp), %r11
        cmovbq  0x60(%rsi), %r11
        cmova   0x60(%rdi), %r11
        movq    0x140(%rsp), %r12
        cmovbq  0x68(%rsi), %r12
        cmova   0x68(%rdi), %r12
        movq    0x148(%rsp), %r13
        cmovbq  0x70(%rsi), %r13
        cmova   0x70(%rdi), %r13
        movq    0x150(%rsp), %r14
        cmovbq  0x78(%rsi), %r14
        cmova   0x78(%rdi), %r14
        movq    0x158(%rsp), %r15
        cmovbq  0x80(%rsi), %r15
        cmova   0x80(%rdi), %r15
        movq    0x160(%rsp), %rbp
        cmovbq  0x88(%rsi), %rbp
        cmova   0x88(%rdi), %rbp
        movq    %r8, 0x120(%rsp)
        movq    %r9, 0x128(%rsp)
        movq    %r10, 0x130(%rsp)
        movq    %r11, 0x138(%rsp)
        movq    %r12, 0x140(%rsp)
        movq    %r13, 0x148(%rsp)
        movq    %r14, 0x150(%rsp)
        movq    %r15, 0x158(%rsp)
        movq    %rbp, 0x160(%rsp)
        movq    0x168(%rsp), %r8
        cmovbq  0x90(%rsi), %r8
        cmova   0x90(%rdi), %r8
        movq    0x170(%rsp), %r9
        cmovbq  0x98(%rsi), %r9
        cmova   0x98(%rdi), %r9
        movq    0x178(%rsp), %r10
        cmovbq  0xa0(%rsi), %r10
        cmova   0xa0(%rdi), %r10
        movq    0x180(%rsp), %r11
        cmovbq  0xa8(%rsi), %r11
        cmova   0xa8(%rdi), %r11
        movq    0x188(%rsp), %r12
        cmovbq  0xb0(%rsi), %r12
        cmova   0xb0(%rdi), %r12
        movq    0x190(%rsp), %r13
        cmovbq  0xb8(%rsi), %r13
        cmova   0xb8(%rdi), %r13
        movq    0x198(%rsp), %r14
        cmovbq  0xc0(%rsi), %r14
        cmova   0xc0(%rdi), %r14
        movq    0x1a0(%rsp), %r15
        cmovbq  0xc8(%rsi), %r15
        cmova   0xc8(%rdi), %r15
        movq    0x1a8(%rsp), %rbp
        cmovbq  0xd0(%rsi), %rbp
        cmova   0xd0(%rdi), %rbp
        movq    %r8, 0x168(%rsp)
        movq    %r9, 0x170(%rsp)
        movq    %r10, 0x178(%rsp)
        movq    %r11, 0x180(%rsp)
        movq    %r12, 0x188(%rsp)
        movq    %r13, 0x190(%rsp)
        movq    %r14, 0x198(%rsp)
        movq    %r15, 0x1a0(%rsp)
        movq    %rbp, 0x1a8(%rsp)
        movq    (%rsp), %r8
        cmovbq  (%rsi), %r8
        cmova   (%rdi), %r8
        movq    0x8(%rsp), %r9
        cmovbq  0x8(%rsi), %r9
        cmova   0x8(%rdi), %r9
        movq    0x10(%rsp), %r10
        cmovbq  0x10(%rsi), %r10
        cmova   0x10(%rdi), %r10
        movq    0x18(%rsp), %r11
        cmovbq  0x18(%rsi), %r11
        cmova   0x18(%rdi), %r11
        movq    0x20(%rsp), %r12
        cmovbq  0x20(%rsi), %r12
        cmova   0x20(%rdi), %r12
        movq    0x28(%rsp), %r13
        cmovbq  0x28(%rsi), %r13
        cmova   0x28(%rdi), %r13
        movq    0x30(%rsp), %r14
        cmovbq  0x30(%rsi), %r14
        cmova   0x30(%rdi), %r14
        movq    0x38(%rsp), %r15
        cmovbq  0x38(%rsi), %r15
        cmova   0x38(%rdi), %r15
        movq    0x40(%rsp), %rbp
        cmovbq  0x40(%rsi), %rbp
        cmova   0x40(%rdi), %rbp
        movq    0x1f8(%rsp), %rdi
        movq    %r8, (%rdi)
        movq    %r9, 0x8(%rdi)
        movq    %r10, 0x10(%rdi)
        movq    %r11, 0x18(%rdi)
        movq    %r12, 0x20(%rdi)
        movq    %r13, 0x28(%rdi)
        movq    %r14, 0x30(%rdi)
        movq    %r15, 0x38(%rdi)
        movq    %rbp, 0x40(%rdi)
        movq    0x120(%rsp), %rax
        movq    %rax, 0x48(%rdi)
        movq    0x128(%rsp), %rax
        movq    %rax, 0x50(%rdi)
        movq    0x130(%rsp), %rax
        movq    %rax, 0x58(%rdi)
        movq    0x138(%rsp), %rax
        movq    %rax, 0x60(%rdi)
        movq    0x140(%rsp), %rax
        movq    %rax, 0x68(%rdi)
        movq    0x148(%rsp), %rax
        movq    %rax, 0x70(%rdi)
        movq    0x150(%rsp), %rax
        movq    %rax, 0x78(%rdi)
        movq    0x158(%rsp), %rax
        movq    %rax, 0x80(%rdi)
        movq    0x160(%rsp), %rax
        movq    %rax, 0x88(%rdi)
        movq    0x168(%rsp), %rax
        movq    %rax, 0x90(%rdi)
        movq    0x170(%rsp), %rax
        movq    %rax, 0x98(%rdi)
        movq    0x178(%rsp), %rax
        movq    %rax, 0xa0(%rdi)
        movq    0x180(%rsp), %rax
        movq    %rax, 0xa8(%rdi)
        movq    0x188(%rsp), %rax
        movq    %rax, 0xb0(%rdi)
        movq    0x190(%rsp), %rax
        movq    %rax, 0xb8(%rdi)
        movq    0x198(%rsp), %rax
        movq    %rax, 0xc0(%rdi)
        movq    0x1a0(%rsp), %rax
        movq    %rax, 0xc8(%rdi)
        movq    0x1a8(%rsp), %rax
        movq    %rax, 0xd0(%rdi)
        CFI_INC_RSP(528)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_jadd)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_jdouble)

Lp521_jscalarmul_jdouble:
        CFI_START
        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(520)
        movq    %rdi, 0x1f8(%rsp)
        movq    %rsi, 0x200(%rsp)
        movq    0x200(%rsp), %rdi
        leaq    0x90(%rdi), %rsi
        leaq    (%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_sqr_p521)
        movq    0x200(%rsp), %rdi
        leaq    0x48(%rdi), %rsi
        leaq    0x48(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_sqr_p521)
        movq    0x200(%rsp), %rdi
        stc
        movq    (%rdi), %rax
        adcq    (%rsp), %rax
        movq    0x8(%rdi), %rbx
        adcq    0x8(%rsp), %rbx
        movq    0x10(%rdi), %r8
        adcq    0x10(%rsp), %r8
        movq    0x18(%rdi), %r9
        adcq    0x18(%rsp), %r9
        movq    0x20(%rdi), %r10
        adcq    0x20(%rsp), %r10
        movq    0x28(%rdi), %r11
        adcq    0x28(%rsp), %r11
        movq    0x30(%rdi), %r12
        adcq    0x30(%rsp), %r12
        movq    0x38(%rdi), %r13
        adcq    0x38(%rsp), %r13
        movq    0x40(%rdi), %r14
        adcq    0x40(%rsp), %r14
        movq    $0x200, %rdx
        andq    %r14, %rdx
        cmpq    $0x200, %rdx
        sbbq    $0x0, %rax
        movq    %rax, 0x168(%rsp)
        sbbq    $0x0, %rbx
        movq    %rbx, 0x170(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x178(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x180(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x188(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x190(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x198(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x1a0(%rsp)
        sbbq    %rdx, %r14
        movq    %r14, 0x1a8(%rsp)
        movq    0x200(%rsp), %rdi
        movq    (%rdi), %rax
        subq    (%rsp), %rax
        movq    0x8(%rdi), %rdx
        sbbq    0x8(%rsp), %rdx
        movq    0x10(%rdi), %r8
        sbbq    0x10(%rsp), %r8
        movq    0x18(%rdi), %r9
        sbbq    0x18(%rsp), %r9
        movq    0x20(%rdi), %r10
        sbbq    0x20(%rsp), %r10
        movq    0x28(%rdi), %r11
        sbbq    0x28(%rsp), %r11
        movq    0x30(%rdi), %r12
        sbbq    0x30(%rsp), %r12
        movq    0x38(%rdi), %r13
        sbbq    0x38(%rsp), %r13
        movq    0x40(%rdi), %r14
        sbbq    0x40(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, 0x120(%rsp)
        sbbq    $0x0, %rdx
        movq    %rdx, 0x128(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x130(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x138(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x140(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x148(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x150(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x158(%rsp)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0x160(%rsp)
        leaq    0x120(%rsp), %rdx
        leaq    0x168(%rsp), %rsi
        leaq    0x90(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    0x200(%rsp), %rdi
        stc
        movq    0x48(%rdi), %rax
        adcq    0x90(%rdi), %rax
        movq    0x50(%rdi), %rbx
        adcq    0x98(%rdi), %rbx
        movq    0x58(%rdi), %r8
        adcq    0xa0(%rdi), %r8
        movq    0x60(%rdi), %r9
        adcq    0xa8(%rdi), %r9
        movq    0x68(%rdi), %r10
        adcq    0xb0(%rdi), %r10
        movq    0x70(%rdi), %r11
        adcq    0xb8(%rdi), %r11
        movq    0x78(%rdi), %r12
        adcq    0xc0(%rdi), %r12
        movq    0x80(%rdi), %r13
        adcq    0xc8(%rdi), %r13
        movq    0x88(%rdi), %r14
        adcq    0xd0(%rdi), %r14
        movq    $0x200, %rdx
        andq    %r14, %rdx
        cmpq    $0x200, %rdx
        sbbq    $0x0, %rax
        movq    %rax, 0x168(%rsp)
        sbbq    $0x0, %rbx
        movq    %rbx, 0x170(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x178(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x180(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x188(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x190(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x198(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x1a0(%rsp)
        sbbq    %rdx, %r14
        movq    %r14, 0x1a8(%rsp)
        leaq    0x90(%rsp), %rsi
        leaq    0x1b0(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_sqr_p521)
        movq    0x200(%rsp), %rdi
        leaq    0x48(%rsp), %rdx
        leaq    (%rdi), %rsi
        leaq    0xd8(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        leaq    0x168(%rsp), %rsi
        leaq    0x120(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_sqr_p521)
        movq    $0x9, %rdx
        movq    0x1f0(%rsp), %rbx
        xorq    $0x1ff, %rbx
        movq    0x1b0(%rsp), %rax
        notq    %rax
        mulxq   %rax, %r8, %r9
        movq    0x1b8(%rsp), %rax
        notq    %rax
        mulxq   %rax, %rax, %r10
        addq    %rax, %r9
        movq    0x1c0(%rsp), %rax
        notq    %rax
        mulxq   %rax, %rax, %r11
        adcq    %rax, %r10
        movq    0x1c8(%rsp), %rax
        notq    %rax
        mulxq   %rax, %rax, %r12
        adcq    %rax, %r11
        movq    0x1d0(%rsp), %rax
        notq    %rax
        mulxq   %rax, %rax, %r13
        adcq    %rax, %r12
        movq    0x1d8(%rsp), %rax
        notq    %rax
        mulxq   %rax, %rax, %r14
        adcq    %rax, %r13
        movq    0x1e0(%rsp), %rax
        notq    %rax
        mulxq   %rax, %rax, %r15
        adcq    %rax, %r14
        movq    0x1e8(%rsp), %rax
        notq    %rax
        mulxq   %rax, %rax, %rcx
        adcq    %rax, %r15
        mulxq   %rbx, %rbx, %rax
        adcq    %rcx, %rbx
        xorl    %eax, %eax
        movq    $0xc, %rdx
        mulxq   0xd8(%rsp), %rax, %rcx
        adcxq   %rax, %r8
        adoxq   %rcx, %r9
        mulxq   0xe0(%rsp), %rax, %rcx
        adcxq   %rax, %r9
        adoxq   %rcx, %r10
        mulxq   0xe8(%rsp), %rax, %rcx
        adcxq   %rax, %r10
        adoxq   %rcx, %r11
        mulxq   0xf0(%rsp), %rax, %rcx
        adcxq   %rax, %r11
        adoxq   %rcx, %r12
        mulxq   0xf8(%rsp), %rax, %rcx
        adcxq   %rax, %r12
        adoxq   %rcx, %r13
        mulxq   0x100(%rsp), %rax, %rcx
        adcxq   %rax, %r13
        adoxq   %rcx, %r14
        mulxq   0x108(%rsp), %rax, %rcx
        adcxq   %rax, %r14
        adoxq   %rcx, %r15
        mulxq   0x110(%rsp), %rax, %rcx
        adcxq   %rax, %r15
        adoxq   %rcx, %rbx
        mulxq   0x118(%rsp), %rax, %rcx
        adcxq   %rax, %rbx
        movq    %r9, %rax
        andq    %r10, %rax
        andq    %r11, %rax
        andq    %r12, %rax
        andq    %r13, %rax
        andq    %r14, %rax
        andq    %r15, %rax
        movq    %rbx, %rdx
        shrq    $0x9, %rdx
        orq     $0xfffffffffffffe00, %rbx
        leaq    0x1(%rdx), %rcx
        addq    %r8, %rcx
        movl    $0x0, %ecx
        adcq    %rcx, %rax
        movq    %rbx, %rax
        adcq    %rcx, %rax
        adcq    %rdx, %r8
        movq    %r8, 0x1b0(%rsp)
        adcq    %rcx, %r9
        movq    %r9, 0x1b8(%rsp)
        adcq    %rcx, %r10
        movq    %r10, 0x1c0(%rsp)
        adcq    %rcx, %r11
        movq    %r11, 0x1c8(%rsp)
        adcq    %rcx, %r12
        movq    %r12, 0x1d0(%rsp)
        adcq    %rcx, %r13
        movq    %r13, 0x1d8(%rsp)
        adcq    %rcx, %r14
        movq    %r14, 0x1e0(%rsp)
        adcq    %rcx, %r15
        movq    %r15, 0x1e8(%rsp)
        adcq    %rcx, %rbx
        andq    $0x1ff, %rbx
        movq    %rbx, 0x1f0(%rsp)
        movq    0x120(%rsp), %rax
        subq    (%rsp), %rax
        movq    0x128(%rsp), %rdx
        sbbq    0x8(%rsp), %rdx
        movq    0x130(%rsp), %r8
        sbbq    0x10(%rsp), %r8
        movq    0x138(%rsp), %r9
        sbbq    0x18(%rsp), %r9
        movq    0x140(%rsp), %r10
        sbbq    0x20(%rsp), %r10
        movq    0x148(%rsp), %r11
        sbbq    0x28(%rsp), %r11
        movq    0x150(%rsp), %r12
        sbbq    0x30(%rsp), %r12
        movq    0x158(%rsp), %r13
        sbbq    0x38(%rsp), %r13
        movq    0x160(%rsp), %r14
        sbbq    0x40(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, 0x168(%rsp)
        sbbq    $0x0, %rdx
        movq    %rdx, 0x170(%rsp)
        sbbq    $0x0, %r8
        movq    %r8, 0x178(%rsp)
        sbbq    $0x0, %r9
        movq    %r9, 0x180(%rsp)
        sbbq    $0x0, %r10
        movq    %r10, 0x188(%rsp)
        sbbq    $0x0, %r11
        movq    %r11, 0x190(%rsp)
        sbbq    $0x0, %r12
        movq    %r12, 0x198(%rsp)
        sbbq    $0x0, %r13
        movq    %r13, 0x1a0(%rsp)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0x1a8(%rsp)
        leaq    0x48(%rsp), %rsi
        leaq    0x120(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_sqr_p521)
        movq    0x1f8(%rsp), %rdi
        movq    0x168(%rsp), %rax
        subq    0x48(%rsp), %rax
        movq    0x170(%rsp), %rdx
        sbbq    0x50(%rsp), %rdx
        movq    0x178(%rsp), %r8
        sbbq    0x58(%rsp), %r8
        movq    0x180(%rsp), %r9
        sbbq    0x60(%rsp), %r9
        movq    0x188(%rsp), %r10
        sbbq    0x68(%rsp), %r10
        movq    0x190(%rsp), %r11
        sbbq    0x70(%rsp), %r11
        movq    0x198(%rsp), %r12
        sbbq    0x78(%rsp), %r12
        movq    0x1a0(%rsp), %r13
        sbbq    0x80(%rsp), %r13
        movq    0x1a8(%rsp), %r14
        sbbq    0x88(%rsp), %r14
        sbbq    $0x0, %rax
        movq    %rax, 0x90(%rdi)
        sbbq    $0x0, %rdx
        movq    %rdx, 0x98(%rdi)
        sbbq    $0x0, %r8
        movq    %r8, 0xa0(%rdi)
        sbbq    $0x0, %r9
        movq    %r9, 0xa8(%rdi)
        sbbq    $0x0, %r10
        movq    %r10, 0xb0(%rdi)
        sbbq    $0x0, %r11
        movq    %r11, 0xb8(%rdi)
        sbbq    $0x0, %r12
        movq    %r12, 0xc0(%rdi)
        sbbq    $0x0, %r13
        movq    %r13, 0xc8(%rdi)
        sbbq    $0x0, %r14
        andq    $0x1ff, %r14
        movq    %r14, 0xd0(%rdi)
        leaq    0x90(%rsp), %rdx
        leaq    0x1b0(%rsp), %rsi
        leaq    0x168(%rsp), %rdi
        CFI_CALL(Lp521_jscalarmul_mul_p521)
        movq    0x1f8(%rsp), %rdi
        movq    0x118(%rsp), %rbx
        movq    0x110(%rsp), %r15
        shldq   $0x2, %r15, %rbx
        movq    0x108(%rsp), %r14
        shldq   $0x2, %r14, %r15
        movq    0x100(%rsp), %r13
        shldq   $0x2, %r13, %r14
        movq    0xf8(%rsp), %r12
        shldq   $0x2, %r12, %r13
        movq    0xf0(%rsp), %r11
        shldq   $0x2, %r11, %r12
        movq    0xe8(%rsp), %r10
        shldq   $0x2, %r10, %r11
        movq    0xe0(%rsp), %r9
        shldq   $0x2, %r9, %r10
        movq    0xd8(%rsp), %r8
        shldq   $0x2, %r8, %r9
        shlq    $0x2, %r8
        movq    0x1f0(%rsp), %rcx
        xorq    $0x1ff, %rcx
        movq    0x1b0(%rsp), %rax
        notq    %rax
        addq    %rax, %r8
        movq    0x1b8(%rsp), %rax
        notq    %rax
        adcq    %rax, %r9
        movq    0x1c0(%rsp), %rax
        notq    %rax
        adcq    %rax, %r10
        movq    0x1c8(%rsp), %rax
        notq    %rax
        adcq    %rax, %r11
        movq    0x1d0(%rsp), %rax
        notq    %rax
        adcq    %rax, %r12
        movq    0x1d8(%rsp), %rax
        notq    %rax
        adcq    %rax, %r13
        movq    0x1e0(%rsp), %rax
        notq    %rax
        adcq    %rax, %r14
        movq    0x1e8(%rsp), %rax
        notq    %rax
        adcq    %rax, %r15
        adcq    %rcx, %rbx
        movq    %r9, %rax
        andq    %r10, %rax
        andq    %r11, %rax
        andq    %r12, %rax
        andq    %r13, %rax
        andq    %r14, %rax
        andq    %r15, %rax
        movq    %rbx, %rdx
        shrq    $0x9, %rdx
        orq     $0xfffffffffffffe00, %rbx
        leaq    0x1(%rdx), %rcx
        addq    %r8, %rcx
        movl    $0x0, %ecx
        adcq    %rcx, %rax
        movq    %rbx, %rax
        adcq    %rcx, %rax
        adcq    %rdx, %r8
        movq    %r8, (%rdi)
        adcq    %rcx, %r9
        movq    %r9, 0x8(%rdi)
        adcq    %rcx, %r10
        movq    %r10, 0x10(%rdi)
        adcq    %rcx, %r11
        movq    %r11, 0x18(%rdi)
        adcq    %rcx, %r12
        movq    %r12, 0x20(%rdi)
        adcq    %rcx, %r13
        movq    %r13, 0x28(%rdi)
        adcq    %rcx, %r14
        movq    %r14, 0x30(%rdi)
        adcq    %rcx, %r15
        movq    %r15, 0x38(%rdi)
        adcq    %rcx, %rbx
        andq    $0x1ff, %rbx
        movq    %rbx, 0x40(%rdi)
        movq    0x1f8(%rsp), %rdi
        movq    0x160(%rsp), %rbx
        xorq    $0x1ff, %rbx
        movq    0x158(%rsp), %r15
        notq    %r15
        shldq   $0x3, %r15, %rbx
        movq    0x150(%rsp), %r14
        notq    %r14
        shldq   $0x3, %r14, %r15
        movq    0x148(%rsp), %r13
        notq    %r13
        shldq   $0x3, %r13, %r14
        movq    0x140(%rsp), %r12
        notq    %r12
        shldq   $0x3, %r12, %r13
        movq    0x138(%rsp), %r11
        notq    %r11
        shldq   $0x3, %r11, %r12
        movq    0x130(%rsp), %r10
        notq    %r10
        shldq   $0x3, %r10, %r11
        movq    0x128(%rsp), %r9
        notq    %r9
        shldq   $0x3, %r9, %r10
        movq    0x120(%rsp), %r8
        notq    %r8
        shldq   $0x3, %r8, %r9
        shlq    $0x3, %r8
        movq    $0x3, %rdx
        xorl    %eax, %eax
        mulxq   0x168(%rsp), %rax, %rcx
        adcxq   %rax, %r8
        adoxq   %rcx, %r9
        mulxq   0x170(%rsp), %rax, %rcx
        adcxq   %rax, %r9
        adoxq   %rcx, %r10
        mulxq   0x178(%rsp), %rax, %rcx
        adcxq   %rax, %r10
        adoxq   %rcx, %r11
        mulxq   0x180(%rsp), %rax, %rcx
        adcxq   %rax, %r11
        adoxq   %rcx, %r12
        mulxq   0x188(%rsp), %rax, %rcx
        adcxq   %rax, %r12
        adoxq   %rcx, %r13
        mulxq   0x190(%rsp), %rax, %rcx
        adcxq   %rax, %r13
        adoxq   %rcx, %r14
        mulxq   0x198(%rsp), %rax, %rcx
        adcxq   %rax, %r14
        adoxq   %rcx, %r15
        mulxq   0x1a0(%rsp), %rax, %rcx
        adcxq   %rax, %r15
        adoxq   %rcx, %rbx
        mulxq   0x1a8(%rsp), %rax, %rcx
        adcxq   %rax, %rbx
        movq    %r9, %rax
        andq    %r10, %rax
        andq    %r11, %rax
        andq    %r12, %rax
        andq    %r13, %rax
        andq    %r14, %rax
        andq    %r15, %rax
        movq    %rbx, %rdx
        shrq    $0x9, %rdx
        orq     $0xfffffffffffffe00, %rbx
        leaq    0x1(%rdx), %rcx
        addq    %r8, %rcx
        movl    $0x0, %ecx
        adcq    %rcx, %rax
        movq    %rbx, %rax
        adcq    %rcx, %rax
        adcq    %rdx, %r8
        movq    %r8, 0x48(%rdi)
        adcq    %rcx, %r9
        movq    %r9, 0x50(%rdi)
        adcq    %rcx, %r10
        movq    %r10, 0x58(%rdi)
        adcq    %rcx, %r11
        movq    %r11, 0x60(%rdi)
        adcq    %rcx, %r12
        movq    %r12, 0x68(%rdi)
        adcq    %rcx, %r13
        movq    %r13, 0x70(%rdi)
        adcq    %rcx, %r14
        movq    %r14, 0x78(%rdi)
        adcq    %rcx, %r15
        movq    %r15, 0x80(%rdi)
        adcq    %rcx, %rbx
        andq    $0x1ff, %rbx
        movq    %rbx, 0x88(%rdi)
        CFI_INC_RSP(520)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_jdouble)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_mul_p521)

Lp521_jscalarmul_mul_p521:
        CFI_START
        CFI_DEC_RSP(64)
        movq    %rdx, %rcx
        xorl    %ebp, %ebp
        movq    (%rcx), %rdx
        mulxq   (%rsi), %r8, %r9
        movq    %r8, (%rsp)
        mulxq   0x8(%rsi), %rbx, %r10
        adcq    %rbx, %r9
        mulxq   0x10(%rsi), %rbx, %r11
        adcq    %rbx, %r10
        mulxq   0x18(%rsi), %rbx, %r12
        adcq    %rbx, %r11
        mulxq   0x20(%rsi), %rbx, %r13
        adcq    %rbx, %r12
        mulxq   0x28(%rsi), %rbx, %r14
        adcq    %rbx, %r13
        mulxq   0x30(%rsi), %rbx, %r15
        adcq    %rbx, %r14
        mulxq   0x38(%rsi), %rbx, %r8
        adcq    %rbx, %r15
        adcq    %rbp, %r8
        movq    0x8(%rcx), %rdx
        xorl    %ebp, %ebp
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        movq    %r9, 0x8(%rsp)
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x18(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x20(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        mulxq   0x30(%rsi), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
        mulxq   0x38(%rsi), %rax, %r9
        adcxq   %rax, %r8
        adoxq   %rbp, %r9
        adcq    %rbp, %r9
        movq    0x10(%rcx), %rdx
        xorl    %ebp, %ebp
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        movq    %r10, 0x10(%rsp)
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x18(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   0x20(%rsi), %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
        mulxq   0x30(%rsi), %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   0x38(%rsi), %rax, %r10
        adcxq   %rax, %r9
        adoxq   %rbp, %r10
        adcq    %rbp, %r10
        movq    0x18(%rcx), %rdx
        xorl    %ebp, %ebp
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        movq    %r11, 0x18(%rsp)
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   0x18(%rsi), %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        mulxq   0x20(%rsi), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   0x30(%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x38(%rsi), %rax, %r11
        adcxq   %rax, %r10
        adoxq   %rbp, %r11
        adcq    %rbp, %r11
        movq    0x20(%rcx), %rdx
        xorl    %ebp, %ebp
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        movq    %r12, 0x20(%rsp)
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        mulxq   0x18(%rsi), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
        mulxq   0x20(%rsi), %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x30(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x38(%rsi), %rax, %r12
        adcxq   %rax, %r11
        adoxq   %rbp, %r12
        adcq    %rbp, %r12
        movq    0x28(%rcx), %rdx
        xorl    %ebp, %ebp
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        movq    %r13, 0x28(%rsp)
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
        mulxq   0x18(%rsi), %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   0x20(%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x30(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x38(%rsi), %rax, %r13
        adcxq   %rax, %r12
        adoxq   %rbp, %r13
        adcq    %rbp, %r13
        movq    0x30(%rcx), %rdx
        xorl    %ebp, %ebp
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        movq    %r14, 0x30(%rsp)
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   0x18(%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x20(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x30(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x38(%rsi), %rax, %r14
        adcxq   %rax, %r13
        adoxq   %rbp, %r14
        adcq    %rbp, %r14
        movq    0x38(%rcx), %rdx
        xorl    %ebp, %ebp
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
        movq    %r15, 0x38(%rsp)
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x18(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x20(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x30(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   0x38(%rsi), %rax, %r15
        adcxq   %rax, %r14
        adoxq   %rbp, %r15
        adcq    %rbp, %r15
        movq    0x40(%rsi), %rdx
        xorl    %ebp, %ebp
        mulxq   (%rcx), %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   0x8(%rcx), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x10(%rcx), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x18(%rcx), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x20(%rcx), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x28(%rcx), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   0x30(%rcx), %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        mulxq   0x38(%rcx), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbp, %rbx
        adcq    %rbx, %rbp
        movq    0x40(%rcx), %rdx
        xorl    %eax, %eax
        mulxq   (%rsi), %rax, %rbx
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
        mulxq   0x8(%rsi), %rax, %rbx
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
        mulxq   0x10(%rsi), %rax, %rbx
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
        mulxq   0x18(%rsi), %rax, %rbx
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
        mulxq   0x20(%rsi), %rax, %rbx
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
        mulxq   0x28(%rsi), %rax, %rbx
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
        mulxq   0x30(%rsi), %rax, %rbx
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
        mulxq   0x38(%rsi), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbx, %rbp
        mulxq   0x40(%rsi), %rax, %rbx
        adcq    %rax, %rbp
        movq    %r8, %rax
        andq    $0x1ff, %rax
        shrdq   $0x9, %r9, %r8
        shrdq   $0x9, %r10, %r9
        shrdq   $0x9, %r11, %r10
        shrdq   $0x9, %r12, %r11
        shrdq   $0x9, %r13, %r12
        shrdq   $0x9, %r14, %r13
        shrdq   $0x9, %r15, %r14
        shrdq   $0x9, %rbp, %r15
        shrq    $0x9, %rbp
        addq    %rax, %rbp
        stc
        adcq    (%rsp), %r8
        adcq    0x8(%rsp), %r9
        adcq    0x10(%rsp), %r10
        adcq    0x18(%rsp), %r11
        adcq    0x20(%rsp), %r12
        adcq    0x28(%rsp), %r13
        adcq    0x30(%rsp), %r14
        adcq    0x38(%rsp), %r15
        adcq    $0xfffffffffffffe00, %rbp
        cmc
        sbbq    $0x0, %r8
        movq    %r8, (%rdi)
        sbbq    $0x0, %r9
        movq    %r9, 0x8(%rdi)
        sbbq    $0x0, %r10
        movq    %r10, 0x10(%rdi)
        sbbq    $0x0, %r11
        movq    %r11, 0x18(%rdi)
        sbbq    $0x0, %r12
        movq    %r12, 0x20(%rdi)
        sbbq    $0x0, %r13
        movq    %r13, 0x28(%rdi)
        sbbq    $0x0, %r14
        movq    %r14, 0x30(%rdi)
        sbbq    $0x0, %r15
        movq    %r15, 0x38(%rdi)
        sbbq    $0x0, %rbp
        andq    $0x1ff, %rbp
        movq    %rbp, 0x40(%rdi)
        CFI_INC_RSP(64)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_mul_p521)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lp521_jscalarmul_sqr_p521)

Lp521_jscalarmul_sqr_p521:
        CFI_START
        CFI_DEC_RSP(64)
        xorl    %ebp, %ebp
        movq    (%rsi), %rdx
        mulxq   0x8(%rsi), %r9, %rax
        movq    %r9, 0x8(%rsp)
        mulxq   0x10(%rsi), %r10, %rcx
        adcxq   %rax, %r10
        movq    %r10, 0x10(%rsp)
        mulxq   0x18(%rsi), %r11, %rax
        adcxq   %rcx, %r11
        mulxq   0x20(%rsi), %r12, %rcx
        adcxq   %rax, %r12
        mulxq   0x28(%rsi), %r13, %rax
        adcxq   %rcx, %r13
        mulxq   0x30(%rsi), %r14, %rcx
        adcxq   %rax, %r14
        mulxq   0x38(%rsi), %r15, %r8
        adcxq   %rcx, %r15
        adcxq   %rbp, %r8
        xorl    %ebp, %ebp
        movq    0x8(%rsi), %rdx
        mulxq   0x10(%rsi), %rax, %rcx
        adcxq   %rax, %r11
        adoxq   %rcx, %r12
        movq    %r11, 0x18(%rsp)
        mulxq   0x18(%rsi), %rax, %rcx
        adcxq   %rax, %r12
        adoxq   %rcx, %r13
        movq    %r12, 0x20(%rsp)
        mulxq   0x20(%rsi), %rax, %rcx
        adcxq   %rax, %r13
        adoxq   %rcx, %r14
        mulxq   0x28(%rsi), %rax, %rcx
        adcxq   %rax, %r14
        adoxq   %rcx, %r15
        mulxq   0x30(%rsi), %rax, %rcx
        adcxq   %rax, %r15
        adoxq   %rcx, %r8
        mulxq   0x38(%rsi), %rax, %r9
        adcxq   %rax, %r8
        adoxq   %rbp, %r9
        movq    0x20(%rsi), %rdx
        mulxq   0x28(%rsi), %rax, %r10
        adcxq   %rax, %r9
        adoxq   %rbp, %r10
        adcxq   %rbp, %r10
        xorl    %ebp, %ebp
        movq    0x10(%rsi), %rdx
        mulxq   0x18(%rsi), %rax, %rcx
        adcxq   %rax, %r13
        adoxq   %rcx, %r14
        movq    %r13, 0x28(%rsp)
        mulxq   0x20(%rsi), %rax, %rcx
        adcxq   %rax, %r14
        adoxq   %rcx, %r15
        movq    %r14, 0x30(%rsp)
        mulxq   0x28(%rsi), %rax, %rcx
        adcxq   %rax, %r15
        adoxq   %rcx, %r8
        mulxq   0x30(%rsi), %rax, %rcx
        adcxq   %rax, %r8
        adoxq   %rcx, %r9
        mulxq   0x38(%rsi), %rax, %rcx
        adcxq   %rax, %r9
        adoxq   %rcx, %r10
        movq    0x30(%rsi), %rdx
        mulxq   0x20(%rsi), %rax, %r11
        adcxq   %rax, %r10
        adoxq   %rbp, %r11
        mulxq   0x28(%rsi), %rax, %r12
        adcxq   %rax, %r11
        adoxq   %rbp, %r12
        adcxq   %rbp, %r12
        xorl    %ebp, %ebp
        movq    0x18(%rsi), %rdx
        mulxq   0x20(%rsi), %rax, %rcx
        adcxq   %rax, %r15
        adoxq   %rcx, %r8
        movq    %r15, 0x38(%rsp)
        mulxq   0x28(%rsi), %rax, %rcx
        adcxq   %rax, %r8
        adoxq   %rcx, %r9
        mulxq   0x30(%rsi), %rax, %rcx
        adcxq   %rax, %r9
        adoxq   %rcx, %r10
        mulxq   0x38(%rsi), %rax, %rcx
        adcxq   %rax, %r10
        adoxq   %rcx, %r11
        movq    0x38(%rsi), %rdx
        mulxq   0x20(%rsi), %rax, %rcx
        adcxq   %rax, %r11
        adoxq   %rcx, %r12
        mulxq   0x28(%rsi), %rax, %r13
        adcxq   %rax, %r12
        adoxq   %rbp, %r13
        mulxq   0x30(%rsi), %rax, %r14
        adcxq   %rax, %r13
        adoxq   %rbp, %r14
        adcxq   %rbp, %r14
        xorl    %ebp, %ebp
        movq    (%rsi), %rdx
        mulxq   %rdx, %rax, %rcx
        movq    %rax, (%rsp)
        movq    0x8(%rsp), %rax
        adcxq   %rax, %rax
        adoxq   %rcx, %rax
        movq    %rax, 0x8(%rsp)
        movq    0x10(%rsp), %rax
        movq    0x8(%rsi), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %rax, %rax
        adoxq   %rdx, %rax
        movq    %rax, 0x10(%rsp)
        movq    0x18(%rsp), %rax
        adcxq   %rax, %rax
        adoxq   %rcx, %rax
        movq    %rax, 0x18(%rsp)
        movq    0x20(%rsp), %rax
        movq    0x10(%rsi), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %rax, %rax
        adoxq   %rdx, %rax
        movq    %rax, 0x20(%rsp)
        movq    0x28(%rsp), %rax
        adcxq   %rax, %rax
        adoxq   %rcx, %rax
        movq    %rax, 0x28(%rsp)
        movq    0x30(%rsp), %rax
        movq    0x18(%rsi), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %rax, %rax
        adoxq   %rdx, %rax
        movq    %rax, 0x30(%rsp)
        movq    0x38(%rsp), %rax
        adcxq   %rax, %rax
        adoxq   %rcx, %rax
        movq    %rax, 0x38(%rsp)
        movq    0x20(%rsi), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %r8, %r8
        adoxq   %rdx, %r8
        adcxq   %r9, %r9
        adoxq   %rcx, %r9
        movq    0x28(%rsi), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %r10, %r10
        adoxq   %rdx, %r10
        adcxq   %r11, %r11
        adoxq   %rcx, %r11
        movq    0x30(%rsi), %rdx
        mulxq   %rdx, %rdx, %rcx
        adcxq   %r12, %r12
        adoxq   %rdx, %r12
        adcxq   %r13, %r13
        adoxq   %rcx, %r13
        movq    0x38(%rsi), %rdx
        mulxq   %rdx, %rdx, %r15
        adcxq   %r14, %r14
        adoxq   %rdx, %r14
        adcxq   %rbp, %r15
        adoxq   %rbp, %r15
        movq    0x40(%rsi), %rdx
        movq    %rdx, %rbp
        imulq   %rbp, %rbp
        addq    %rdx, %rdx
        mulxq   (%rsi), %rax, %rcx
        adcxq   %rax, %r8
        adoxq   %rcx, %r9
        mulxq   0x8(%rsi), %rax, %rcx
        adcxq   %rax, %r9
        adoxq   %rcx, %r10
        mulxq   0x10(%rsi), %rax, %rcx
        adcxq   %rax, %r10
        adoxq   %rcx, %r11
        mulxq   0x18(%rsi), %rax, %rcx
        adcxq   %rax, %r11
        adoxq   %rcx, %r12
        mulxq   0x20(%rsi), %rax, %rcx
        adcxq   %rax, %r12
        adoxq   %rcx, %r13
        mulxq   0x28(%rsi), %rax, %rcx
        adcxq   %rax, %r13
        adoxq   %rcx, %r14
        mulxq   0x30(%rsi), %rax, %rcx
        adcxq   %rax, %r14
        adoxq   %rcx, %r15
        mulxq   0x38(%rsi), %rax, %rcx
        adcxq   %rax, %r15
        adoxq   %rcx, %rbp
        adcq    $0x0, %rbp
        movq    %r8, %rax
        andq    $0x1ff, %rax
        shrdq   $0x9, %r9, %r8
        shrdq   $0x9, %r10, %r9
        shrdq   $0x9, %r11, %r10
        shrdq   $0x9, %r12, %r11
        shrdq   $0x9, %r13, %r12
        shrdq   $0x9, %r14, %r13
        shrdq   $0x9, %r15, %r14
        shrdq   $0x9, %rbp, %r15
        shrq    $0x9, %rbp
        addq    %rax, %rbp
        stc
        adcq    (%rsp), %r8
        adcq    0x8(%rsp), %r9
        adcq    0x10(%rsp), %r10
        adcq    0x18(%rsp), %r11
        adcq    0x20(%rsp), %r12
        adcq    0x28(%rsp), %r13
        adcq    0x30(%rsp), %r14
        adcq    0x38(%rsp), %r15
        adcq    $0xfffffffffffffe00, %rbp
        cmc
        sbbq    $0x0, %r8
        movq    %r8, (%rdi)
        sbbq    $0x0, %r9
        movq    %r9, 0x8(%rdi)
        sbbq    $0x0, %r10
        movq    %r10, 0x10(%rdi)
        sbbq    $0x0, %r11
        movq    %r11, 0x18(%rdi)
        sbbq    $0x0, %r12
        movq    %r12, 0x20(%rdi)
        sbbq    $0x0, %r13
        movq    %r13, 0x28(%rdi)
        sbbq    $0x0, %r14
        movq    %r14, 0x30(%rdi)
        sbbq    $0x0, %r15
        movq    %r15, 0x38(%rdi)
        sbbq    $0x0, %rbp
        andq    $0x1ff, %rbp
        movq    %rbp, 0x40(%rdi)
        CFI_INC_RSP(64)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lp521_jscalarmul_sqr_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
