// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// The x25519 function for curve25519
// Inputs scalar[4], point[4]; output res[4]
//
// extern void curve25519_x25519_alt
//   (uint64_t res[static 4],const uint64_t scalar[static 4],
//    const uint64_t point[static 4]);
//
// The function has a second prototype considering the arguments as arrays
// of bytes rather than 64-bit words. The underlying code is the same, since
// the x86 platform is little-endian.
//
// extern void curve25519_x25519_byte_alt
//   (uint8_t res[static 32],const uint8_t scalar[static 32],
//    const uint8_t point[static 32]);
//
// Given a scalar n and the X coordinate of an input point P = (X,Y) on
// curve25519 (Y can live in any extension field of characteristic 2^255-19),
// this returns the X coordinate of n * P = (X, Y), or 0 when n * P is the
// point at infinity. Both n and X inputs are first slightly modified/mangled
// as specified in the relevant RFC (https://www.rfc-editor.org/rfc/rfc7748);
// in particular the lower three bits of n are set to zero. Does not implement
// the zero-check specified in Section 6.1.
//
// Standard x86-64 ABI: RDI = res, RSI = scalar, RDX = point
// Microsoft x64 ABI:   RCX = res, RDX = scalar, R8 = point
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(curve25519_x25519_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(curve25519_x25519_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(curve25519_x25519_alt)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(curve25519_x25519_byte_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(curve25519_x25519_byte_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(curve25519_x25519_byte_alt)
        .text

// Size of individual field elements

#define NUMSIZE 32

// Stable homes for the input result argument during the whole body
// and other variables that are only needed prior to the modular inverse.

#define res  12*NUMSIZE(%rsp)
#define i  12*NUMSIZE+8(%rsp)
#define swap  12*NUMSIZE+16(%rsp)

// Pointers to result x coord to be written, assuming the base "res"
// has been loaded into %rbp

#define resx 0(%rbp)

// Pointer-offset pairs for temporaries on stack with some aliasing.
// Both dmsn and dnsm need space for >= 5 digits, and we allocate 8

#define scalar (0*NUMSIZE)(%rsp)

#define pointx (1*NUMSIZE)(%rsp)

#define dm (2*NUMSIZE)(%rsp)

#define zm (3*NUMSIZE)(%rsp)
#define sm (3*NUMSIZE)(%rsp)
#define dpro (3*NUMSIZE)(%rsp)

#define sn (4*NUMSIZE)(%rsp)

#define dn (5*NUMSIZE)(%rsp)
#define e (5*NUMSIZE)(%rsp)

#define dmsn (6*NUMSIZE)(%rsp)
#define p (6*NUMSIZE)(%rsp)
#define zn (7*NUMSIZE)(%rsp)

#define xm (8*NUMSIZE)(%rsp)
#define dnsm (8*NUMSIZE)(%rsp)
#define spro (8*NUMSIZE)(%rsp)

#define xn (10*NUMSIZE)(%rsp)
#define s (10*NUMSIZE)(%rsp)

#define d (11*NUMSIZE)(%rsp)

// Total size to reserve on the stack
// This includes space for the 3 other variables above
// and rounds up to a multiple of 32

#define NSPACE 13*NUMSIZE

// Macro wrapping up the basic field operation bignum_mul_p25519_alt, only
// trivially different from a pure function call to that subroutine.

#define mul_p25519(P0,P1,P2)                    \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        xorq    %r10, %r10 ;                        \
        xorq    %r11, %r11 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     P2;                 \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    $0x0, %r11 ;                        \
        xorq    %r12, %r12 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    %r12, %r12 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x8+P2;             \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        xorq    %r13, %r13 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    %r13, %r13 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        xorq    %r14, %r14 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    %r14, %r14 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        xorq    %r15, %r15 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    %r15, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    $0x0, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                        \
        adcq    %rdx, %r15 ;                        \
        movl    $0x26, %esi ;                       \
        movq    %r12, %rax ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        sbbq    %rcx, %rcx ;                        \
        movq    %r13, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r14, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r15, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        xorq    %rcx, %rcx ;                        \
        addq    %rax, %r11 ;                        \
        movq    %rdx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        shldq   $0x1, %r11, %r12 ;                    \
        leaq    0x1(%r12), %rax ;                  \
        movl    $0x13, %esi ;                       \
        bts     $63, %r11 ;                         \
        imulq   %rsi, %rax ;                        \
        addq    %rax, %r8 ;                         \
        adcq    %rcx, %r9 ;                         \
        adcq    %rcx, %r10 ;                        \
        adcq    %rcx, %r11 ;                        \
        sbbq    %rax, %rax ;                        \
        notq    %rax;                            \
        andq    %rsi, %rax ;                        \
        subq    %rax, %r8 ;                         \
        sbbq    %rcx, %r9 ;                         \
        sbbq    %rcx, %r10 ;                        \
        sbbq    %rcx, %r11 ;                        \
        btr     $63, %r11 ;                         \
        movq    %r8, P0 ;                        \
        movq    %r9, 0x8+P0 ;                    \
        movq    %r10, 0x10+P0 ;                  \
        movq    %r11, 0x18+P0

// A version of multiplication that only guarantees output < 2 * p_25519.
// This basically skips the +1 and final correction in quotient estimation.

#define mul_4(P0,P1,P2)                         \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        xorq    %r10, %r10 ;                        \
        xorq    %r11, %r11 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     P2;                 \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    $0x0, %r11 ;                        \
        xorq    %r12, %r12 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    %r12, %r12 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x8+P2;             \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        xorq    %r13, %r13 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    %r13, %r13 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        xorq    %r14, %r14 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    %r14, %r14 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        xorq    %r15, %r15 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    %r15, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    $0x0, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                        \
        adcq    %rdx, %r15 ;                        \
        movl    $0x26, %esi ;                       \
        movq    %r12, %rax ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        sbbq    %rcx, %rcx ;                        \
        movq    %r13, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r14, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r15, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        xorq    %rcx, %rcx ;                        \
        addq    %rax, %r11 ;                        \
        movq    %rdx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        shldq   $0x1, %r11, %r12 ;                    \
        btr     $0x3f, %r11 ;                      \
        movl    $0x13, %edx ;                      \
        imulq   %r12, %rdx ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rcx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Multiplication just giving a 5-digit result (actually < 39 * p_25519)
// by not doing anything beyond the first stage of reduction

#define mul_5(P0,P1,P2)                         \
        movq    P1, %rax ;                      \
        mulq     P2;                 \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        xorq    %r10, %r10 ;                        \
        xorq    %r11, %r11 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x8+P2;             \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     P2;                 \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    $0x0, %r11 ;                        \
        xorq    %r12, %r12 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x10+P2;            \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    %r12, %r12 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x8+P2;             \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        xorq    %r13, %r13 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x18+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    %r13, %r13 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P2;            \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     P2;                 \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        xorq    %r14, %r14 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    %r14, %r14 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x8+P2;             \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        xorq    %r15, %r15 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    %r15, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x10+P2;            \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    $0x0, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq     0x18+P2;            \
        addq    %rax, %r14 ;                        \
        adcq    %rdx, %r15 ;                        \
        movl    $0x26, %esi ;                       \
        movq    %r12, %rax ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        sbbq    %rcx, %rcx ;                        \
        movq    %r13, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r14, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r15, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        xorq    %rcx, %rcx ;                        \
        addq    %rax, %r11 ;                        \
        movq    %rdx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0 ;                 \
        movq    %r12, 0x20+P0

// Squaring just giving a result < 2 * p_25519, which is done by
// basically skipping the +1 in the quotient estimate and the final
// optional correction.

#define sqr_4(P0,P1)                            \
        movq    P1, %rax ;                      \
        mulq    %rax;                            \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        xorq    %r10, %r10 ;                        \
        xorq    %r11, %r11 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x8+P1;             \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r11 ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        adcq    $0x0, %r11 ;                        \
        xorq    %r12, %r12 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq    %rax;                            \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r12 ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        adcq    $0x0, %r12 ;                        \
        xorq    %r13, %r13 ;                        \
        movq    P1, %rax ;                      \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r13 ;                        \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x10+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r13 ;                        \
        addq    %rax, %r11 ;                        \
        adcq    %rdx, %r12 ;                        \
        adcq    $0x0, %r13 ;                        \
        xorq    %r14, %r14 ;                        \
        movq    0x8+P1, %rax ;                  \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r14 ;                        \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r12 ;                        \
        adcq    %rdx, %r13 ;                        \
        adcq    $0x0, %r14 ;                        \
        xorq    %r15, %r15 ;                        \
        movq    0x10+P1, %rax ;                 \
        mulq     0x18+P1;            \
        addq    %rax, %rax ;                        \
        adcq    %rdx, %rdx ;                        \
        adcq    $0x0, %r15 ;                        \
        addq    %rax, %r13 ;                        \
        adcq    %rdx, %r14 ;                        \
        adcq    $0x0, %r15 ;                        \
        movq    0x18+P1, %rax ;                 \
        mulq    %rax;                            \
        addq    %rax, %r14 ;                        \
        adcq    %rdx, %r15 ;                        \
        movl    $0x26, %esi ;                       \
        movq    %r12, %rax ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        sbbq    %rcx, %rcx ;                        \
        movq    %r13, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r14, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r15, %rax ;                        \
        mulq    %rsi;                            \
        subq    %rcx, %rdx ;                        \
        xorq    %rcx, %rcx ;                        \
        addq    %rax, %r11 ;                        \
        movq    %rdx, %r12 ;                        \
        adcq    %rcx, %r12 ;                        \
        shldq   $0x1, %r11, %r12 ;                  \
        btr     $0x3f, %r11 ;                      \
        movl    $0x13, %edx ;                      \
        imulq   %r12, %rdx ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rcx, %r9 ;                        \
        adcq    %rcx, %r10 ;                       \
        adcq    %rcx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Add 5-digit inputs and normalize to 4 digits

#define add5_4(P0,P1,P2)                        \
        movq    P1, %r8 ;                       \
        addq    P2, %r8 ;                       \
        movq    8+P1, %r9 ;                     \
        adcq    8+P2, %r9 ;                     \
        movq    16+P1, %r10 ;                   \
        adcq    16+P2, %r10 ;                   \
        movq    24+P1, %r11 ;                   \
        adcq    24+P2, %r11 ;                   \
        movq    32+P1, %r12 ;                   \
        adcq    32+P2, %r12 ;                   \
        xorl    %ebx, %ebx ;                       \
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

// Modular addition with double modulus 2 * p_25519 = 2^256 - 38.
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

// 5-digit subtraction with upward bias to make it positive, adding
// 1000 * (2^255 - 19) = 2^256 * 500 - 19000, then normalizing to 4 digits

#define sub5_4(P0,P1,P2)                        \
        movq    P1, %r8 ;                       \
        subq    P2, %r8 ;                       \
        movq    8+P1, %r9 ;                     \
        sbbq    8+P2, %r9 ;                     \
        movq    16+P1, %r10 ;                   \
        sbbq    16+P2, %r10 ;                   \
        movq    24+P1, %r11 ;                   \
        sbbq    24+P2, %r11 ;                   \
        movq    32+P1, %r12 ;                   \
        sbbq    32+P2, %r12 ;                   \
        xorl    %ebx, %ebx ;                       \
        subq    $19000, %r8 ;                      \
        sbbq    %rbx, %r9 ;                        \
        sbbq    %rbx, %r10 ;                       \
        sbbq    %rbx, %r11 ;                       \
        sbbq    %rbx, %r12 ;                       \
        addq    $500, %r12 ;                       \
        shldq   $0x1, %r11, %r12 ;                  \
        btr     $0x3f, %r11 ;                      \
        movl    $0x13, %edx ;                      \
        imulq   %r12, %rdx ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rbx, %r9 ;                        \
        adcq    %rbx, %r10 ;                       \
        adcq    %rbx, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Combined z = c * x + y with reduction only < 2 * p_25519
// It is assumed that 19 * (c * x + y) < 2^60 * 2^256 so we
// don't need a high mul in the final part.

#define cmadd_4(P0,C1,P2,P3)                    \
        movq    $C1, %rsi ;                         \
        movq    P2, %rax ;                       \
        mulq    %rsi;                            \
        movq    %rax, %r8 ;                         \
        movq    %rdx, %r9 ;                         \
        movq    0x8+P2, %rax ;                   \
        xorq    %r10, %r10 ;                        \
        mulq    %rsi;                            \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        movq    0x10+P2, %rax ;                  \
        mulq    %rsi;                            \
        addq    %rax, %r10 ;                        \
        adcq    $0x0, %rdx ;                        \
        movq    0x18+P2, %rax ;                  \
        movq    %rdx, %r11 ;                        \
        mulq    %rsi;                            \
        xorl    %esi, %esi ;                        \
        addq    %rax, %r11 ;                        \
        adcq    %rsi, %rdx ;                        \
        addq    P3, %r8 ;                        \
        adcq    0x8+P3, %r9 ;                    \
        adcq    0x10+P3, %r10 ;                  \
        adcq    0x18+P3, %r11 ;                  \
        adcq    %rsi, %rdx ;                        \
        shldq   $0x1, %r11, %rdx ;                  \
        btr     $63, %r11 ;                        \
        movl    $0x13, %ebx ;                      \
        imulq   %rbx, %rdx ;                       \
        addq    %rdx, %r8 ;                        \
        adcq    %rsi, %r9 ;                        \
        adcq    %rsi, %r10 ;                       \
        adcq    %rsi, %r11 ;                       \
        movq    %r8, P0 ;                       \
        movq    %r9, 0x8+P0 ;                   \
        movq    %r10, 0x10+P0 ;                 \
        movq    %r11, 0x18+P0

// Multiplex: z := if NZ then x else y

#define mux_4(P0,P1,P2)                         \
        movq    P1, %rax ;                      \
        movq    P2, %rcx ;                      \
        cmovzq  %rcx, %rax ;                       \
        movq    %rax, P0 ;                      \
        movq    8+P1, %rax ;                    \
        movq    8+P2, %rcx ;                    \
        cmovzq  %rcx, %rax ;                       \
        movq    %rax, 8+P0 ;                    \
        movq    16+P1, %rax ;                   \
        movq    16+P2, %rcx ;                   \
        cmovzq  %rcx, %rax ;                       \
        movq    %rax, 16+P0 ;                   \
        movq    24+P1, %rax ;                   \
        movq    24+P2, %rcx ;                   \
        cmovzq  %rcx, %rax ;                       \
        movq    %rax, 24+P0

S2N_BN_SYMBOL(curve25519_x25519_alt):
S2N_BN_SYMBOL(curve25519_x25519_byte_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
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

// Copy the inputs to the local variables with minimal mangling:
//
//  - The scalar is in principle turned into 01xxx...xxx000 but
//    in the structure below the special handling of these bits is
//    explicit in the main computation; the scalar is just copied.
//
//  - The point x coord is reduced mod 2^255 by masking off the
//    top bit. In the main loop we only need reduction < 2 * p_25519.

        movq    (%rsi), %rax
        movq    %rax, (%rsp)
        movq    8(%rsi), %rax
        movq    %rax, 8(%rsp)
        movq    16(%rsi), %rax
        movq    %rax, 16(%rsp)
        movq    24(%rsi), %rax
        movq    %rax, 24(%rsp)

        movq    (%rdx), %r8
        movq    8(%rdx), %r9
        movq    16(%rdx), %r10
        movq    24(%rdx), %r11
        btr     $63, %r11
        movq    %r8, 32(%rsp)
        movq    %r9, 40(%rsp)
        movq    %r10, 48(%rsp)
        movq    %r11, 56(%rsp)

// Initialize with explicit doubling in order to handle set bit 254.
// Set swap = 1 and (xm,zm) = (x,1) then double as (xn,zn) = 2 * (x,1).
// We use the fact that the point x coordinate is still in registers.
// Since zm = 1 we could do the doubling with an operation count of
// 2 * S + M instead of 2 * S + 2 * M, but it doesn't seem worth
// the slight complication arising from a different linear combination.

        movl    $1, %eax
        movq    %rax, swap
        movq    %r8, 256(%rsp)
        movq    %rax, 96(%rsp)
        xorl    %eax, %eax
        movq    %r9, 264(%rsp)
        movq    %rax, 104(%rsp)
        movq    %r10, 272(%rsp)
        movq    %rax, 112(%rsp)
        movq    %r11, 280(%rsp)
        movq    %rax, 120(%rsp)

        sub_twice4(d,xm,zm)
        add_twice4(s,xm,zm)
        sqr_4(d,d)
        sqr_4(s,s)
        sub_twice4(p,s,d)
        cmadd_4(e,0x1db42,p,d)
        mul_4(xn,s,d)
        mul_4(zn,p,e)

// The main loop over unmodified bits from i = 253, ..., i = 3 (inclusive).
// This is a classic Montgomery ladder, with the main coordinates only
// reduced mod 2 * p_25519, some intermediate results even more loosely.

        movl    $253, %eax
        movq    %rax, i

Lcurve25519_x25519_alt_scalarloop:

// sm = xm + zm; sn = xn + zn; dm = xm - zm; dn = xn - zn

        sub_twice4(dm,xm,zm)
        add_twice4(sn,xn,zn)
        sub_twice4(dn,xn,zn)
        add_twice4(sm,xm,zm)

// DOUBLING: mux d = xt - zt and s = xt + zt for appropriate choice of (xt,zt)

        movq    i, %rdx
        movq    %rdx, %rcx
        shrq    $6, %rdx
        movq    (%rsp,%rdx,8), %rdx
        shrq    %cl, %rdx
        andq    $1, %rdx
        cmpq    swap, %rdx
        movq    %rdx, swap
        mux_4(d,dm,dn)
        mux_4(s,sm,sn)

// ADDING: dmsn = dm * sn; dnsm = sm * dn

        mul_5(dnsm,sm,dn)
        mul_5(dmsn,sn,dm)

// DOUBLING: d = (xt - zt)^2

        sqr_4(d,d)

// ADDING: dpro = (dmsn - dnsm)^2, spro = (dmsn + dnsm)^2
// DOUBLING: s = (xt + zt)^2

        sub5_4(dpro,dmsn,dnsm)
        add5_4(spro,dmsn,dnsm)
        sqr_4(s,s)
        sqr_4(dpro,dpro)

// DOUBLING: p = 4 * xt * zt = s - d

        sub_twice4(p,s,d)

// ADDING: xm' = (dmsn + dnsm)^2

        sqr_4(xm,spro)

// DOUBLING: e = 121666 * p + d

        cmadd_4(e,0x1db42,p,d)

// DOUBLING: xn' = (xt + zt)^2 * (xt - zt)^2 = s * d

        mul_4(xn,s,d)

// DOUBLING: zn' = (4 * xt * zt) * ((xt - zt)^2 + 121666 * (4 * xt * zt))
//               = p * (d + 121666 * p)

        mul_4(zn,p,e)

// ADDING: zm' = x * (dmsn - dnsm)^2

        mul_4(zm,dpro,pointx)

// Loop down as far as 3 (inclusive)

        movq    i, %rax
        subq    $1, %rax
        movq    %rax, i
        cmpq    $3, %rax
        jnc     Lcurve25519_x25519_alt_scalarloop

// Multiplex directly into (xn,zn) then do three pure doubling steps;
// this accounts for the implicit zeroing of the three lowest bits
// of the scalar.

        movq    swap, %rdx
        testq   %rdx, %rdx
        mux_4(xn,xm,xn)
        mux_4(zn,zm,zn)

        sub_twice4(d,xn,zn)
        add_twice4(s,xn,zn)
        sqr_4(d,d)
        sqr_4(s,s)
        sub_twice4(p,s,d)
        cmadd_4(e,0x1db42,p,d)
        mul_4(xn,s,d)
        mul_4(zn,p,e)

        sub_twice4(d,xn,zn)
        add_twice4(s,xn,zn)
        sqr_4(d,d)
        sqr_4(s,s)
        sub_twice4(p,s,d)
        cmadd_4(e,0x1db42,p,d)
        mul_4(xn,s,d)
        mul_4(zn,p,e)

        sub_twice4(d,xn,zn)
        add_twice4(s,xn,zn)
        sqr_4(d,d)
        sqr_4(s,s)
        sub_twice4(p,s,d)
        cmadd_4(e,0x1db42,p,d)
        mul_4(xn,s,d)
        mul_4(zn,p,e)

// The projective result of the scalar multiplication is now (xn,zn).
// Prepare to call the modular inverse function to get zn' = 1/zn

        leaq    224(%rsp), %rdi
        leaq    224(%rsp), %rsi

// Inline copy of bignum_inv_p25519, identical except for stripping out
// the prologue and epilogue saving and restoring registers and making
// and reclaiming room on the stack. For more details and explanations see
// "x86/curve25519/bignum_inv_p25519.S". Note that the stack it uses for
// its own temporaries is 208 bytes, so it has no effect on variables
// that are needed in the rest of our computation here: res, xn and zn.

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
        jmp     Lcurve25519_x25519_alt_midloop
Lcurve25519_x25519_alt_inverseloop:
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
Lcurve25519_x25519_alt_midloop:
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
        jne     Lcurve25519_x25519_alt_inverseloop
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

// Now the result is xn * (1/zn), fully reduced modulo p.
// Note that in the degenerate case zn = 0 (mod p_25519), the
// modular inverse code above will produce 1/zn = 0, giving
// the correct overall X25519 result of zero for the point at
// infinity.

        movq    res, %rbp
        mul_p25519(resx,xn,zn)

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

S2N_BN_SIZE_DIRECTIVE(curve25519_x25519_alt)
S2N_BN_SIZE_DIRECTIVE(curve25519_x25519_byte_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
