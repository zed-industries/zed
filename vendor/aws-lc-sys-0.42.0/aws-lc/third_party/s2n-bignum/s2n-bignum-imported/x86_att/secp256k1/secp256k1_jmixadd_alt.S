// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point mixed addition on SECG curve secp256k1 in Jacobian coordinates
//
//    extern void secp256k1_jmixadd_alt(uint64_t p3[static 12],
//                                      const uint64_t p1[static 12],
//                                      const uint64_t p2[static 8]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples.
// A Jacobian triple (x,y,z) represents affine point (x/z^2,y/z^3).
// The "mixed" part means that p2 only has x and y coordinates, with the
// implicit z coordinate assumed to be the identity. It is assumed that
// all the coordinates of the input points p1 and p2 are fully reduced
// mod p_256k1, that the z coordinate of p1 is nonzero and that neither
// p1 =~= p2 or p1 =~= -p2, where "=~=" means "represents the same affine
// point as".
//
// Standard x86-64 ABI: RDI = p3, RSI = p1, RDX = p2
// Microsoft x64 ABI:   RCX = p3, RDX = p1, R8 = p2
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(secp256k1_jmixadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(secp256k1_jmixadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(secp256k1_jmixadd_alt)
        .text

// Size of individual field elements

#define NUMSIZE 32

// Pointer-offset pairs for inputs and outputs
// These assume %rdi = p3, %rsi = p1 and %rbp = p2,
// all of which are maintained throughout the code.

#define x_1 0(%rsi)
#define y_1 NUMSIZE(%rsi)
#define z_1 (2*NUMSIZE)(%rsi)

#define x_2 0(%rbp)
#define y_2 NUMSIZE(%rbp)
#define z_2 (2*NUMSIZE)(%rbp)

#define x_3 0(%rdi)
#define y_3 NUMSIZE(%rdi)
#define z_3 (2*NUMSIZE)(%rdi)

// Pointer-offset pairs for temporaries, with some aliasing
// NSPACE is the total stack needed for these temporaries

#define zp2 (NUMSIZE*0)(%rsp)
#define ww (NUMSIZE*0)(%rsp)
#define resx (NUMSIZE*0)(%rsp)

#define yd (NUMSIZE*1)(%rsp)
#define y2a (NUMSIZE*1)(%rsp)

#define x2a (NUMSIZE*2)(%rsp)
#define zzx2 (NUMSIZE*2)(%rsp)

#define zz (NUMSIZE*3)(%rsp)
#define t1 (NUMSIZE*3)(%rsp)

#define t2 (NUMSIZE*4)(%rsp)
#define zzx1 (NUMSIZE*4)(%rsp)
#define resy (NUMSIZE*4)(%rsp)

#define xd (NUMSIZE*5)(%rsp)
#define resz (NUMSIZE*5)(%rsp)

#define NSPACE NUMSIZE*6

// Corresponds to bignum_mul_p256k1_alt except %rsi -> %rbx

#define mul_p256k1(P0,P1,P2)                    \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        xorq    %r10, %r10 ;                       \
        xorq    %r11, %r11 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     P2;                 \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        xorq    %r12, %r12 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    %r12, %r12 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x8+P2;             \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        xorq    %r13, %r13 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    %r13, %r13 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P2;            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        xorq    %r14, %r14 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        xorq    %r15, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    %r15, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        movq    $0x1000003d1, %rbx ;               \
        movq    %r12, %rax ;                       \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rcx, %rcx ;                       \
        movq    %r13, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r14, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r15, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        xorq    %rcx, %rcx ;                       \
        addq    %rax, %r11 ;                       \
        movq    %rdx, %r12 ;                       \
        adcq    %rcx, %r12 ;                       \
        leaq    0x1(%r12), %rax ;                 \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        sbbq    %rax, %rax ;                       \
        notq    %rax;                            \
        andq    %rbx, %rax ;                       \
        subq    %rax, %r8 ;                        \
        sbbq    %rcx, %r9 ;                        \
        sbbq    %rcx, %r10 ;                       \
        sbbq    %rcx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Corresponds to bignum_sqr_p256k1_alt except for %rsi -> %rbx

#define sqr_p256k1(P0,P1)                       \
        movq    P1, %rax ;                      \
        mulq    %rax;                            \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        xorq    %r10, %r10 ;                       \
        xorq    %r11, %r11 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x8+P1;             \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r11 ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        adcq    $0x0, %r11 ;                       \
        xorq    %r12, %r12 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rax;                            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r12 ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        adcq    $0x0, %r12 ;                       \
        xorq    %r13, %r13 ;                       \
        movq    P1, %rax ;                      \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r13 ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        adcq    $0x0, %r13 ;                       \
        xorq    %r14, %r14 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r14 ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        xorq    %r15, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0x0, %r15 ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    $0x0, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        movq    $0x1000003d1, %rbx ;               \
        movq    %r12, %rax ;                       \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        sbbq    %rcx, %rcx ;                       \
        movq    %r13, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r14, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r15, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        xorq    %rcx, %rcx ;                       \
        addq    %rax, %r11 ;                       \
        movq    %rdx, %r12 ;                       \
        adcq    %rcx, %r12 ;                       \
        leaq    0x1(%r12), %rax ;                 \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                        \
        adcq    %rdx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        sbbq    %rax, %rax ;                       \
        notq    %rax;                            \
        andq    %rbx, %rax ;                       \
        subq    %rax, %r8 ;                        \
        sbbq    %rcx, %r9 ;                        \
        sbbq    %rcx, %r10 ;                       \
        sbbq    %rcx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Corresponds exactly to bignum_sub_p256k1

#define sub_p256k1(P0,P1,P2)                    \
        xorl   %eax, %eax ;                         \
        movq   P1, %r8 ;                         \
        subq   P2, %r8 ;                         \
        movq   0x8+P1, %r9 ;                     \
        sbbq   0x8+P2, %r9 ;                     \
        movq   0x10+P1, %r10 ;                   \
        sbbq   0x10+P2, %r10 ;                   \
        movq   0x18+P1, %r11 ;                   \
        sbbq   0x18+P2, %r11 ;                   \
        movabs $0x1000003d1, %rcx ;                 \
        cmovae %rax, %rcx ;                         \
        subq   %rcx, %r8 ;                          \
        movq   %r8, P0 ;                         \
        sbbq   %rax, %r9 ;                          \
        movq   %r9, 0x8+P0 ;                     \
        sbbq   %rax, %r10 ;                         \
        movq   %r10, 0x10+P0 ;                   \
        sbbq   %rax, %r11 ;                         \
        movq   %r11, 0x18+P0

// Additional macros to help with final multiplexing

#define testzero4(P)                            \
        movq    P, %rax ;                       \
        movq    8+P, %rdx ;                     \
        orq     16+P, %rax ;                    \
        orq     24+P, %rdx ;                    \
        orq     %rdx, %rax

#define mux4(r0,r1,r2,r3,PNE,PEQ)               \
        movq    PNE, r0 ;                      \
        movq    PEQ, %rax ;                     \
        cmovzq  %rax, r0 ;                        \
        movq    8+PNE, r1 ;                    \
        movq    8+PEQ, %rax ;                   \
        cmovzq  %rax, r1 ;                        \
        movq    16+PNE, r2 ;                   \
        movq    16+PEQ, %rax ;                  \
        cmovzq  %rax, r2 ;                        \
        movq    24+PNE, r3 ;                   \
        movq    24+PEQ, %rax ;                  \
        cmovzq  %rax, r3

#define load4(r0,r1,r2,r3,P)                    \
        movq    P, r0 ;                        \
        movq    8+P, r1 ;                      \
        movq    16+P, r2 ;                     \
        movq    24+P, r3

#define store4(P,r0,r1,r2,r3)                   \
        movq    r0, P ;                        \
        movq    r1, 8+P ;                      \
        movq    r2, 16+P ;                     \
        movq    r3, 24+P

S2N_BN_SYMBOL(secp256k1_jmixadd_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save registers and make room on stack for temporary variables
// Put the input y in %rbp where it stays

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

        movq    %rdx, %rbp

// Main code, just a sequence of basic field operations

        sqr_p256k1(zp2,z_1)

        mul_p256k1(y2a,z_1,y_2)
        mul_p256k1(x2a,zp2,x_2)
        mul_p256k1(y2a,zp2,y2a)

        sub_p256k1(xd,x2a,x_1)

        sub_p256k1(yd,y2a,y_1)

        sqr_p256k1(zz,xd)
        sqr_p256k1(ww,yd)

        mul_p256k1(zzx1,zz,x_1)
        mul_p256k1(zzx2,zz,x2a)

        sub_p256k1(resx,ww,zzx1)
        sub_p256k1(t1,zzx2,zzx1)

        mul_p256k1(resz,xd,z_1)

        sub_p256k1(resx,resx,zzx2)

        sub_p256k1(t2,zzx1,resx)

        mul_p256k1(t1,t1,y_1)
        mul_p256k1(t2,yd,t2)

        sub_p256k1(resy,t2,t1)

// Test if z_1 = 0 to decide if p1 = 0 (up to projective equivalence)

        testzero4(z_1)

// Multiplex: if p1 <> 0 just copy the computed result from the staging area.
// If p1 = 0 then return the point p2 augmented with an extra z = 1
// coordinate, hence giving 0 + p2 = p2 for the final result.

        mux4(%r8,%r9,%r10,%r11,resx,x_2)
        mux4(%r12,%r13,%r14,%r15,resy,y_2)

        store4(x_3,%r8,%r9,%r10,%r11)
        store4(y_3,%r12,%r13,%r14,%r15)

        load4(%r8,%r9,%r10,%r11,resz)
        movl    $1, %eax
        cmovzq  %rax, %r8
        movl    $0, %eax
        cmovzq  %rax, %r9
        cmovzq  %rax, %r10
        cmovzq  %rax, %r11

        store4(z_3,%r8,%r9,%r10,%r11)

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

S2N_BN_SIZE_DIRECTIVE(secp256k1_jmixadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
