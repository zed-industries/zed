// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Extended projective addition for edwards25519
// Inputs p1[16], p2[16]; output p3[16]
//
// extern void edwards25519_epadd_alt
//   (uint64_t p3[static 16],const uint64_t p1[static 16],
//    const uint64_t p2[static 16]);
//
// The output p3 and both inputs p1 and p2 are points (x,y) on
// edwards25519 represented in extended projective quadruples (X,Y,Z,T)
// where x = X / Z, y = Y / Z and x * y = T / Z.
//
// Standard x86-64 ABI: RDI = p3, RSI = p1, RDX = p2
// Microsoft x64 ABI:   RCX = p3, RDX = p1, R8 = p2
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_epadd_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_epadd_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_epadd_alt)
        .text

// Size of individual field elements

#define NUMSIZE 32

// Registers used for inputs and outputs within basic operations.
// Here p1 and p3 are where the parameters come in anyway;
// the p2 = %rbp assignment is set up at the beginning.

#define p3 %rdi
#define p1 %rsi
#define p2 %rbp

// Pointers to input and output coordinates

#define x_1 0(p1)
#define y_1 NUMSIZE(p1)
#define z_1 (2*NUMSIZE)(p1)
#define w_1 (3*NUMSIZE)(p1)

#define x_2 0(p2)
#define y_2 NUMSIZE(p2)
#define z_2 (2*NUMSIZE)(p2)
#define w_2 (3*NUMSIZE)(p2)

#define x_3 0(p3)
#define y_3 NUMSIZE(p3)
#define z_3 (2*NUMSIZE)(p3)
#define w_3 (3*NUMSIZE)(p3)

// Pointer-offset pairs for temporaries on stack

#define t0 (0*NUMSIZE)(%rsp)
#define t1 (1*NUMSIZE)(%rsp)
#define t2 (2*NUMSIZE)(%rsp)
#define t3 (3*NUMSIZE)(%rsp)
#define t4 (4*NUMSIZE)(%rsp)
#define t5 (5*NUMSIZE)(%rsp)

// Total size to reserve on the stack

#define NSPACE 6*NUMSIZE

// Macro wrapping up the basic field multiplication, only trivially
// different from a pure function call to bignum_mul_p25519_alt.

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
        movl    $0x26, %ebx ;                       \
        movq    %r12, %rax ;                        \
        mulq    %rbx;                            \
        addq    %rax, %r8 ;                         \
        adcq    %rdx, %r9 ;                         \
        sbbq    %rcx, %rcx ;                        \
        movq    %r13, %rax ;                        \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r9 ;                         \
        adcq    %rdx, %r10 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r14, %rax ;                        \
        mulq    %rbx;                            \
        subq    %rcx, %rdx ;                        \
        addq    %rax, %r10 ;                        \
        adcq    %rdx, %r11 ;                        \
        sbbq    %rcx, %rcx ;                        \
        movq    %r15, %rax ;                        \
        mulq    %rbx;                            \
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

// Plain 4-digit add and doubling without any normalization
// With inputs < p_25519 (indeed < 2^255) it still gives a 4-digit result,
// indeed one < 2 * p_25519 for normalized inputs.

#define add_4(P0,P1,P2)                         \
        movq    P1, %rax ;                      \
        addq    P2, %rax ;                      \
        movq    %rax, P0 ;                      \
        movq    8+P1, %rax ;                    \
        adcq    8+P2, %rax ;                    \
        movq    %rax, 8+P0 ;                    \
        movq    16+P1, %rax ;                   \
        adcq    16+P2, %rax ;                   \
        movq    %rax, 16+P0 ;                   \
        movq    24+P1, %rax ;                   \
        adcq    24+P2, %rax ;                   \
        movq    %rax, 24+P0

#define double_4(P0,P1)                         \
        movq    P1, %rax ;                      \
        addq    %rax, %rax ;                       \
        movq    %rax, P0 ;                      \
        movq    8+P1, %rax ;                    \
        adcq    %rax, %rax ;                       \
        movq    %rax, 8+P0 ;                    \
        movq    16+P1, %rax ;                   \
        adcq    %rax, %rax ;                       \
        movq    %rax, 16+P0 ;                   \
        movq    24+P1, %rax ;                   \
        adcq    %rax, %rax ;                       \
        movq    %rax, 24+P0

// Subtraction of a pair of numbers < p_25519 just sufficient
// to give a 4-digit result. It actually always does (x - z) + (2^255-19)
// which in turn is done by (x - z) - (2^255+19) discarding the 2^256
// implicitly

#define sub_4(P0,P1,P2)                         \
        movq    P1, %r8 ;                       \
        subq    P2, %r8 ;                       \
        movq    8+P1, %r9 ;                     \
        sbbq    8+P2, %r9 ;                     \
        movq    16+P1, %r10 ;                   \
        sbbq    16+P2, %r10 ;                   \
        movq    24+P1, %rax ;                   \
        sbbq    24+P2, %rax ;                   \
        subq    $19, %r8 ;                         \
        movq    %r8, P0 ;                       \
        sbbq    $0, %r9 ;                          \
        movq    %r9, 8+P0 ;                     \
        sbbq    $0, %r10 ;                         \
        movq    %r10, 16+P0 ;                   \
        sbbq    $0, %rax ;                         \
        btc     $63, %rax ;                        \
        movq    %rax, 24+P0

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

// Modular addition with inputs double modulus 2 * p_25519 = 2^256 - 38
// and in general only guaranteeing a 4-digit result, not even < 2 * p_25519.

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

S2N_BN_SYMBOL(edwards25519_epadd_alt):
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

// Main sequence of operations. after setting up p2 in its register

        movq    %rdx, p2

        mul_4(t0,w_1,w_2)

        sub_4(t1,y_1,x_1)
        sub_4(t2,y_2,x_2)
        add_4(t3,y_1,x_1)
        add_4(t4,y_2,x_2)
        double_4(t5,z_2)

        mul_4(t1,t1,t2)
        mul_4(t3,t3,t4)

        load_k25519(t2)
        mul_4(t2,t2,t0)

        mul_4(t4,z_1,t5)

        sub_twice4(t0,t3,t1)
        add_twice4(t5,t3,t1)
        sub_twice4(t1,t4,t2)
        add_twice4(t3,t4,t2)

        mul_p25519(w_3,t0,t5)
        mul_p25519(x_3,t0,t1)
        mul_p25519(y_3,t3,t5)
        mul_p25519(z_3,t1,t3)

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

S2N_BN_SIZE_DIRECTIVE(edwards25519_epadd_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
