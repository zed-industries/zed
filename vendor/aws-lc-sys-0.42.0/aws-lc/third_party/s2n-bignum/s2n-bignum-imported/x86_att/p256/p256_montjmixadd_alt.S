// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Point mixed addition on NIST curve P-256 in Montgomery-Jacobian coordinates
//
//    extern void p256_montjmixadd_alt(uint64_t p3[static 12],
//                                     const uint64_t p1[static 12],
//                                     const uint64_t p2[static 8]);
//
// Does p3 := p1 + p2 where all points are regarded as Jacobian triples with
// each coordinate in the Montgomery domain, i.e. x' = (2^256 * x) mod p_256.
// A Jacobian triple (x',y',z') represents affine point (x/z^2,y/z^3).
// The "mixed" part means that p2 only has x and y coordinates, with the
// implicit z coordinate assumed to be the identity.
//
// Standard x86-64 ABI: RDI = p3, RSI = p1, RDX = p2
// Microsoft x64 ABI:   RCX = p3, RDX = p1, R8 = p2
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(p256_montjmixadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(p256_montjmixadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(p256_montjmixadd_alt)
        .text

// Size of individual field elements

#define NUMSIZE 32

// Pointer-offset pairs for inputs and outputs.
// These assume %rdi = p3, %rsi = p1 and %rbp = p2,
// which needs to be set up explicitly before use.
// The first two hold initially, and the second is
// set up by copying the initial %rdx input to %rbp.
// Thereafter, no code macro modifies any of them.

#define x_1 0(%rsi)
#define y_1 NUMSIZE(%rsi)
#define z_1 (2*NUMSIZE)(%rsi)

#define x_2 0(%rbp)
#define y_2 NUMSIZE(%rbp)

#define x_3 0(%rdi)
#define y_3 NUMSIZE(%rdi)
#define z_3 (2*NUMSIZE)(%rdi)

// Pointer-offset pairs for temporaries, with some aliasing.
// NSPACE is the total stack needed for all temporaries.

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

// Corresponds exactly to bignum_montmul_p256_alt

#define montmul_p256(P0,P1,P2)                  \
        movq    P2, %rbx ;                      \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r9 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        xorl    %r10d, %r10d ;                     \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        xorl    %r11d, %r11d ;                     \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        xorl    %r12d, %r12d ;                     \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        movq    0x8+P2, %rbx ;                  \
        xorl    %r13d, %r13d ;                     \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %r14, %r14 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %r14, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %r14, %r14 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r14, %rdx ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %r14, %r14 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r14, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        xorl    %r14d, %r14d ;                     \
        movq    $0x100000000, %rbx ;               \
        movq    %r8, %rax ;                        \
        mulq    %rbx;                            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %r15, %r15 ;                       \
        movq    %r9, %rax ;                        \
        mulq    %rbx;                            \
        subq    %r15, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %r15, %r15 ;                       \
        notq    %rbx;                            \
        leaq    0x2(%rbx), %rbx ;                 \
        movq    %r8, %rax ;                        \
        mulq    %rbx;                            \
        subq    %r15, %rdx ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %r15, %r15 ;                       \
        movq    %r9, %rax ;                        \
        mulq    %rbx;                            \
        subq    %r15, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        movq    0x10+P2, %rbx ;                 \
        xorl    %r15d, %r15d ;                     \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %r8, %r8 ;                         \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %r8, %rdx ;                        \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %r8, %r8 ;                         \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r8, %rdx ;                        \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %r8, %r8 ;                         \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r8, %rdx ;                        \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        adcq    %r15, %r15 ;                       \
        movq    0x18+P2, %rbx ;                 \
        xorl    %r8d, %r8d ;                       \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %r9, %r9 ;                         \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %r9, %rdx ;                        \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %r9, %r9 ;                         \
        movq    0x10+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r9, %rdx ;                        \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %r9, %r9 ;                         \
        movq    0x18+P1, %rax ;                 \
        mulq    %rbx;                            \
        subq    %r9, %rdx ;                        \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    %r8, %r8 ;                         \
        xorl    %r9d, %r9d ;                       \
        movq    $0x100000000, %rbx ;               \
        movq    %r10, %rax ;                       \
        mulq    %rbx;                            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r11, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %rcx, %rcx ;                       \
        notq    %rbx;                            \
        leaq    0x2(%rbx), %rbx ;                 \
        movq    %r10, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r11, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    %r9, %r8 ;                         \
        movl    $0x1, %ecx ;                       \
        addq    %r12, %rcx ;                       \
        decq    %rbx;                            \
        adcq    %r13, %rbx ;                       \
        decq    %r9;                             \
        movq    %r9, %rax ;                        \
        adcq    %r14, %r9 ;                        \
        movl    $0xfffffffe, %r11d ;               \
        adcq    %r15, %r11 ;                       \
        adcq    %r8, %rax ;                        \
        cmovbq  %rcx, %r12 ;                       \
        cmovbq  %rbx, %r13 ;                       \
        cmovbq  %r9, %r14 ;                        \
        cmovbq  %r11, %r15 ;                       \
        movq    %r12, P0 ;                      \
        movq    %r13, 0x8+P0 ;                  \
        movq    %r14, 0x10+P0 ;                 \
        movq    %r15, 0x18+P0

// Corresponds exactly to bignum_montsqr_p256_alt

#define montsqr_p256(P0,P1)                     \
        movq    P1, %rax ;                      \
        movq    %rax, %rbx ;                       \
        mulq    %rax;                            \
        movq    %rax, %r8 ;                        \
        movq    %rdx, %r15 ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        movq    %rax, %r9 ;                        \
        movq    %rdx, %r10 ;                       \
        movq    0x18+P1, %rax ;                 \
        movq    %rax, %r13 ;                       \
        mulq    %rbx;                            \
        movq    %rax, %r11 ;                       \
        movq    %rdx, %r12 ;                       \
        movq    0x10+P1, %rax ;                 \
        movq    %rax, %rbx ;                       \
        mulq    %r13;                            \
        movq    %rax, %r13 ;                       \
        movq    %rdx, %r14 ;                       \
        movq    P1, %rax ;                      \
        mulq    %rbx;                            \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    0x18+P1, %rbx ;                 \
        movq    0x8+P1, %rax ;                  \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    $0x0, %r14 ;                       \
        xorl    %ecx, %ecx ;                       \
        addq    %r9, %r9 ;                         \
        adcq    %r10, %r10 ;                       \
        adcq    %r11, %r11 ;                       \
        adcq    %r12, %r12 ;                       \
        adcq    %r13, %r13 ;                       \
        adcq    %r14, %r14 ;                       \
        adcq    %rcx, %rcx ;                       \
        movq    0x8+P1, %rax ;                  \
        mulq    %rax;                            \
        addq    %r15, %r9 ;                        \
        adcq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %r15, %r15 ;                       \
        movq    0x10+P1, %rax ;                 \
        mulq    %rax;                            \
        negq    %r15;                            \
        adcq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %r15, %r15 ;                       \
        movq    0x18+P1, %rax ;                 \
        mulq    %rax;                            \
        negq    %r15;                            \
        adcq    %rax, %r14 ;                       \
        adcq    %rcx, %rdx ;                       \
        movq    %rdx, %r15 ;                       \
        movq    $0x100000000, %rbx ;               \
        movq    %r8, %rax ;                        \
        mulq    %rbx;                            \
        addq    %rax, %r9 ;                        \
        adcq    %rdx, %r10 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r9, %rax ;                        \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r10 ;                       \
        adcq    %rdx, %r11 ;                       \
        sbbq    %rcx, %rcx ;                       \
        notq    %rbx;                            \
        leaq    0x2(%rbx), %rbx ;                 \
        movq    %r8, %rax ;                        \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %rcx, %rcx ;                       \
        xorl    %r8d, %r8d ;                       \
        movq    %r9, %rax ;                        \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        adcq    %r8, %r14 ;                        \
        adcq    %r8, %r15 ;                        \
        adcq    %r8, %r8 ;                         \
        movq    $0x100000000, %rbx ;               \
        movq    %r10, %rax ;                       \
        mulq    %rbx;                            \
        addq    %rax, %r11 ;                       \
        adcq    %rdx, %r12 ;                       \
        sbbq    %rcx, %rcx ;                       \
        movq    %r11, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r12 ;                       \
        adcq    %rdx, %r13 ;                       \
        sbbq    %rcx, %rcx ;                       \
        notq    %rbx;                            \
        leaq    0x2(%rbx), %rbx ;                 \
        movq    %r10, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r13 ;                       \
        adcq    %rdx, %r14 ;                       \
        sbbq    %rcx, %rcx ;                       \
        xorl    %r9d, %r9d ;                       \
        movq    %r11, %rax ;                       \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                       \
        addq    %rax, %r14 ;                       \
        adcq    %rdx, %r15 ;                       \
        adcq    %r9, %r8 ;                         \
        movl    $0x1, %ecx ;                       \
        addq    %r12, %rcx ;                       \
        leaq    -0x1(%rbx), %rbx ;                 \
        adcq    %r13, %rbx ;                       \
        leaq    -0x1(%r9), %r9 ;                   \
        movq    %r9, %rax ;                        \
        adcq    %r14, %r9 ;                        \
        movl    $0xfffffffe, %r11d ;               \
        adcq    %r15, %r11 ;                       \
        adcq    %r8, %rax ;                        \
        cmovbq  %rcx, %r12 ;                       \
        cmovbq  %rbx, %r13 ;                       \
        cmovbq  %r9, %r14 ;                        \
        cmovbq  %r11, %r15 ;                       \
        movq    %r12, P0 ;                      \
        movq    %r13, 0x8+P0 ;                  \
        movq    %r14, 0x10+P0 ;                 \
        movq    %r15, 0x18+P0

// Corresponds exactly to bignum_sub_p256

#define sub_p256(P0,P1,P2)                      \
        movq    P1, %rax ;                       \
        subq    P2, %rax ;                       \
        movq    0x8+P1, %rcx ;                   \
        sbbq    0x8+P2, %rcx ;                   \
        movq    0x10+P1, %r8 ;                   \
        sbbq    0x10+P2, %r8 ;                   \
        movq    0x18+P1, %r9 ;                   \
        sbbq    0x18+P2, %r9 ;                   \
        movl    $0xffffffff, %r10d ;                \
        sbbq    %r11, %r11 ;                        \
        xorq    %rdx, %rdx ;                        \
        andq    %r11, %r10 ;                        \
        subq    %r10, %rdx ;                        \
        addq    %r11, %rax ;                        \
        movq    %rax, P0 ;                       \
        adcq    %r10, %rcx ;                        \
        movq    %rcx, 0x8+P0 ;                   \
        adcq    $0x0, %r8 ;                         \
        movq    %r8, 0x10+P0 ;                   \
        adcq    %rdx, %r9 ;                         \
        movq    %r9, 0x18+P0

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

S2N_BN_SYMBOL(p256_montjmixadd_alt):
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
// Put the input y in %rbp where it lasts as long as it's needed.

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

        CFI_DEC_RSP(NSPACE)

        movq    %rdx, %rbp

// Main code, just a sequence of basic field operations
// 8 * multiply + 3 * square + 7 * subtract

        montsqr_p256(zp2,z_1)

        montmul_p256(y2a,z_1,y_2)
        montmul_p256(x2a,zp2,x_2)
        montmul_p256(y2a,zp2,y2a)

        sub_p256(xd,x2a,x_1)

        sub_p256(yd,y2a,y_1)

        montsqr_p256(zz,xd)
        montsqr_p256(ww,yd)

        montmul_p256(zzx1,zz,x_1)
        montmul_p256(zzx2,zz,x2a)

        sub_p256(resx,ww,zzx1)
        sub_p256(t1,zzx2,zzx1)

        montmul_p256(resz,xd,z_1)

        sub_p256(resx,resx,zzx2)

        sub_p256(t2,zzx1,resx)

        montmul_p256(t1,t1,y_1)
        montmul_p256(t2,yd,t2)

        sub_p256(resy,t2,t1)

// Test if z_1 = 0 to decide if p1 = 0 (up to projective equivalence)

        testzero4(z_1)

// Multiplex: if p1 <> 0 just copy the computed result from the staging area.
// If p1 = 0 then return the point p2 augmented with a z = 1 coordinate (in
// Montgomery form so not the simple constant 1 but rather 2^256 - p_256),
// hence giving 0 + p2 = p2 for the final result.

        mux4(%r8,%r9,%r10,%r11,resx,x_2)
        mux4(%r12,%r13,%r14,%r15,resy,y_2)

        store4(x_3,%r8,%r9,%r10,%r11)
        store4(y_3,%r12,%r13,%r14,%r15)

        load4(%r8,%r9,%r10,%r11,resz)
        movl    $1, %eax
        cmovzq  %rax, %r8
        movq    $0xffffffff00000000, %rax
        cmovzq  %rax, %r9
        movq    $0xffffffffffffffff, %rax
        cmovzq  %rax, %r10
        movl    $0x00000000fffffffe, %eax
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

S2N_BN_SIZE_DIRECTIVE(p256_montjmixadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
