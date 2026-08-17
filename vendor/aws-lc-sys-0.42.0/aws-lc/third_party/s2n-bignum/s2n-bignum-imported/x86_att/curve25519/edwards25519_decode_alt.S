// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Decode compressed 256-bit form of edwards25519 point
// Input c[32] (bytes); output function return and z[8]
//
// extern uint64_t edwards25519_decode_alt(uint64_t z[static 8], const uint8_t c[static 32]);
//
// This interprets the input byte string as a little-endian number
// representing a point (x,y) on the edwards25519 curve, encoded as
// 2^255 * x_0 + y where x_0 is the least significant bit of x. It
// returns the full pair of coordinates x (at z) and y (at z+4). The
// return code is 0 for success and 1 for failure, which means that
// the input does not correspond to the encoding of any edwards25519
// point. This can happen for three reasons, where y = the lowest
// 255 bits of the input:
//
//  * y >= p_25519
//    Input y coordinate is not reduced
//  * (y^2 - 1) * (1 + d_25519 * y^2) has no modular square root
//    There is no x such that (x,y) is on the curve
//  * y^2 = 1 and top bit of input is set
//    Cannot be the canonical encoding of (0,1) or (0,-1)
//
// Standard x86-64 ABI: RDI = z, RSI = c
// Microsoft x64 ABI:   RCX = z, RDX = c
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(edwards25519_decode_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(edwards25519_decode_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(edwards25519_decode_alt)

        .text

// Size in bytes of a 64-bit word

#define N 8

// Pointer-offset pairs for temporaries on stack

#define y 0(%rsp)
#define s (4*N)(%rsp)
#define t (8*N)(%rsp)
#define u (12*N)(%rsp)
#define v (16*N)(%rsp)
#define w (20*N)(%rsp)
#define q (24*N)(%rsp)
#define res  (28*N)(%rsp)
#define sgnbit  (29*N)(%rsp)
#define badun  (30*N)(%rsp)

// Total size to reserve on the stack

#define NSPACE 32*N

// Corrupted versions when stack is down 8 more

#define q8 (25*N)(%rsp)

// Syntactic variants to make x86_att version simpler to generate

#define Y 0
#define S (4*N)
#define T (8*N)
#define U (12*N)
#define V (16*N)
#define W (20*N)
#define Q8 (25*N)

S2N_BN_SYMBOL(edwards25519_decode_alt):
        CFI_START
        _CET_ENDBR

// In this case the Windows form literally makes a subroutine call.
// This avoids hassle arising from subroutine offsets

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        CFI_CALL(Ledwards25519_decode_alt_standard)
        CFI_POP(%rsi)
        CFI_POP(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(edwards25519_decode_alt)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_decode_alt_standard)

Ledwards25519_decode_alt_standard:
        CFI_START
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

// Load the inputs, which can be done word-wise since x86 is little-endian.
// Let y be the lowest 255 bits of the input and sgnbit the desired parity.
// If y >= p_25519 then already flag the input as invalid (badun = 1).

        movq    (%rsi), %rax
        movq    %rax, Y(%rsp)
        movq    8(%rsi), %rbx
        movq    %rbx, Y+8(%rsp)
        xorl    %ebp, %ebp
        movq    16(%rsi), %rcx
        movq    %rcx, Y+16(%rsp)
        movq    24(%rsi), %rdx
        btr     $63, %rdx
        movq    %rdx, Y+24(%rsp)
        adcq    %rbp, %rbp
        movq    %rbp, sgnbit

        addq    $19, %rax
        adcq    $0, %rbx
        adcq    $0, %rcx
        adcq    $0, %rdx
        shrq    $63, %rdx
        movq    %rdx, badun

// u = y^2 - 1 (actually y + 2^255-20, not reduced modulo)
// v = 1 + d * y^2 (not reduced modulo from the +1)
// w = u * v

        leaq    V(%rsp), %rdi
        movq    $1, %rsi
        leaq    Y(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)
        movq    V(%rsp), %rax
        subq    $20, %rax
        movq    V+8(%rsp), %rbx
        sbbq    $0, %rbx
        movq    V+16(%rsp), %rcx
        sbbq    $0, %rcx
        movq    V+24(%rsp), %rdx
        sbbq    $0, %rdx
        btc     $63, %rdx
        movq    %rax, U(%rsp)
        movq    %rbx, U+8(%rsp)
        movq    %rcx, U+16(%rsp)
        movq    %rdx, U+24(%rsp)

        movq    $0x75eb4dca135978a3, %rax
        movq    %rax, W(%rsp)
        movq    $0x00700a4d4141d8ab, %rax
        movq    %rax, W+8(%rsp)
        movq    $0x8cc740797779e898, %rax
        movq    %rax, W+16(%rsp)
        movq    $0x52036cee2b6ffe73, %rax
        movq    %rax, W+24(%rsp)
        leaq    V(%rsp), %rdi
        leaq    W(%rsp), %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)
        movq    V(%rsp), %rax
        addq    $1, %rax
        movq    V+8(%rsp), %rbx
        adcq    $0, %rbx
        movq    V+16(%rsp), %rcx
        adcq    $0, %rcx
        movq    V+24(%rsp), %rdx
        adcq    $0, %rdx
        movq    %rax, V(%rsp)
        movq    %rbx, V+8(%rsp)
        movq    %rcx, V+16(%rsp)
        movq    %rdx, V+24(%rsp)

        leaq    W(%rsp), %rdi
        leaq    U(%rsp), %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

// Get s = w^{252-3} as a candidate inverse square root 1/sqrt(w).
// This power tower computation is the same as bignum_invsqrt_p25519

        leaq    T(%rsp), %rdi
        movq    $1, %rsi
        leaq    W(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    T(%rsp), %rsi
        leaq    W(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $2, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $1, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    V(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    W(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $5, %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $10, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $5, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    V(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $25, %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $50, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $25, %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    V(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $125, %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    V(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

        leaq    S(%rsp), %rdi
        movq    $2, %rsi
        leaq    V(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    S(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    W(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

// Compute v' = s^2 * w to discriminate whether the square root sqrt(u/v)
// exists, in which case we should get 0, 1 or -1.

        leaq    V(%rsp), %rdi
        movq    $1, %rsi
        leaq    S(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_nsqr_p25519)

        leaq    V(%rsp), %rdi
        leaq    V(%rsp), %rsi
        leaq    W(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

// Get the two candidates for sqrt(u / v), one being s = u * w^{252-3}
// and the other being t = s * j_25519 where j_25519 = sqrt(-1).

        leaq    S(%rsp), %rdi
        leaq    U(%rsp), %rsi
        leaq    S(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)
        movq    $0xc4ee1b274a0ea0b0, %rax
        movq    %rax, T(%rsp)
        movq    $0x2f431806ad2fe478, %rax
        movq    %rax, T+8(%rsp)
        movq    $0x2b4d00993dfbd7a7, %rax
        movq    %rax, T+16(%rsp)
        movq    $0x2b8324804fc1df0b, %rax
        movq    %rax, T+24(%rsp)
        leaq    T(%rsp), %rdi
        leaq    S(%rsp), %rsi
        leaq    T(%rsp), %rdx
        CFI_CALL(Ledwards25519_decode_alt_mul_p25519)

// %rax = 0 <=> s^2 * w = 0 or 1

        movq    V(%rsp), %r8
        movq    V+8(%rsp), %r9
        movq    V+16(%rsp), %r10
        movq    V+24(%rsp), %r11
        movl    $1, %eax
        notq    %rax
        andq    %r8, %rax
        orq     %r9, %rax
        orq     %r10, %rax
        orq     %r11, %rax

// %r8 = 0 <=> s^2 * w = -1 (mod p_25519, i.e. s^2 * w = 2^255 - 20)

        addq    $20, %r8
        notq    %r9
        notq    %r10
        bts     $63, %r11
        addq    $1, %r11
        orq     %r9, %r8
        orq     %r11, %r10
        orq     %r10, %r8

// If s^2 * w is not 0 or 1 then replace s by t

        testq   %rax, %rax

        movq    S(%rsp), %r12
        movq    T(%rsp), %rbx
        cmovnzq %rbx, %r12
        movq    S+8(%rsp), %r13
        movq    T+8(%rsp), %rbx
        cmovnzq %rbx, %r13
        movq    S+16(%rsp), %r14
        movq    T+16(%rsp), %rbx
        cmovnzq %rbx, %r14
        movq    S+24(%rsp), %r15
        movq    T+24(%rsp), %rbx
        cmovnzq %rbx, %r15
        movq    %r12, S(%rsp)
        movq    %r13, S+8(%rsp)
        movq    %r14, S+16(%rsp)
        movq    %r15, S+24(%rsp)

// Check invalidity, occurring if s^2 * w is not in {0,1,-1}

        cmovzq  %rax, %r8
        negq    %r8
        sbbq    %r8, %r8
        negq    %r8
        orq     %r8, badun

// Let [%r11;%r10;%r9;%r8] = s and [%r15;%r14;%r13;%r12] = p_25519 - s

        movq    S(%rsp), %r8
        movq    $-19, %r12
        subq    %r8, %r12
        movq    S+8(%rsp), %r9
        movq    $-1, %r13
        sbbq    %r9, %r13
        movq    S+16(%rsp), %r10
        movq    $-1, %r14
        sbbq    %r10, %r14
        movq    S+24(%rsp), %r11
        movq    $0x7FFFFFFFFFFFFFFF, %r15
        sbbq    %r11, %r15

// Decide whether a flip is apparently indicated, s_0 <=> sgnbit
// Decide also if s = 0 by OR-ing its digits. Now if a flip is indicated:
//  - if s = 0 then mark as invalid
//  - if s <> 0 then indeed flip

        movl    $1, %ecx
        andq    %r8, %rcx
        xorq    sgnbit, %rcx
        movq    badun, %rdx
        movq    %rdx, %rsi
        orq     %rcx, %rdx
        xorl    %ebp, %ebp
        movq    %r8, %rax
        movq    %r9, %rbx
        orq     %r10, %rax
        orq     %r11, %rbx
        orq     %rbx, %rax
        cmovzq  %rbp, %rcx
        cmovnzq %rsi, %rdx

// Actual selection of x as s or -s, copying of y and return of validity

        testq   %rcx, %rcx

        cmovnzq %r12, %r8
        cmovnzq %r13, %r9
        cmovnzq %r14, %r10
        cmovnzq %r15, %r11

        movq    res, %rdi
        movq    %r8, (%rdi)
        movq    %r9, 8(%rdi)
        movq    %r10, 16(%rdi)
        movq    %r11, 24(%rdi)
        movq    Y(%rsp), %rcx
        movq    %rcx, 32(%rdi)
        movq    Y+8(%rsp), %rcx
        movq    %rcx, 40(%rdi)
        movq    Y+16(%rsp), %rcx
        movq    %rcx, 48(%rdi)
        movq    Y+24(%rsp), %rcx
        movq    %rcx, 56(%rdi)

        movq    %rdx, %rax

// Restore stack and registers

        CFI_INC_RSP(NSPACE)

        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)
        CFI_RET

#if WINDOWS_ABI
S2N_BN_SIZE_DIRECTIVE(Ledwards25519_decode_alt_standard)
#else
S2N_BN_SIZE_DIRECTIVE(edwards25519_decode_alt)
#endif

// *************************************************************
// Local z = x * y
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_decode_alt_mul_p25519)

Ledwards25519_decode_alt_mul_p25519:
        CFI_START
        movq    %rdx, %rcx
        movq    (%rsi), %rax
        mulq     (%rcx)
        movq    %rax, %r8
        movq    %rdx, %r9
        xorq    %r10, %r10
        xorq    %r11, %r11
        movq    (%rsi), %rax
        mulq     0x8(%rcx)
        addq    %rax, %r9
        adcq    %rdx, %r10
        movq    0x8(%rsi), %rax
        mulq     (%rcx)
        addq    %rax, %r9
        adcq    %rdx, %r10
        adcq    $0x0, %r11
        xorq    %r12, %r12
        movq    (%rsi), %rax
        mulq     0x10(%rcx)
        addq    %rax, %r10
        adcq    %rdx, %r11
        adcq    %r12, %r12
        movq    0x8(%rsi), %rax
        mulq     0x8(%rcx)
        addq    %rax, %r10
        adcq    %rdx, %r11
        adcq    $0x0, %r12
        movq    0x10(%rsi), %rax
        mulq     (%rcx)
        addq    %rax, %r10
        adcq    %rdx, %r11
        adcq    $0x0, %r12
        xorq    %r13, %r13
        movq    (%rsi), %rax
        mulq     0x18(%rcx)
        addq    %rax, %r11
        adcq    %rdx, %r12
        adcq    %r13, %r13
        movq    0x8(%rsi), %rax
        mulq     0x10(%rcx)
        addq    %rax, %r11
        adcq    %rdx, %r12
        adcq    $0x0, %r13
        movq    0x10(%rsi), %rax
        mulq     0x8(%rcx)
        addq    %rax, %r11
        adcq    %rdx, %r12
        adcq    $0x0, %r13
        movq    0x18(%rsi), %rax
        mulq     (%rcx)
        addq    %rax, %r11
        adcq    %rdx, %r12
        adcq    $0x0, %r13
        xorq    %r14, %r14
        movq    0x8(%rsi), %rax
        mulq     0x18(%rcx)
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    %r14, %r14
        movq    0x10(%rsi), %rax
        mulq     0x10(%rcx)
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        movq    0x18(%rsi), %rax
        mulq     0x8(%rcx)
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorq    %r15, %r15
        movq    0x10(%rsi), %rax
        mulq     0x18(%rcx)
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    %r15, %r15
        movq    0x18(%rsi), %rax
        mulq     0x10(%rcx)
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    $0x0, %r15
        movq    0x18(%rsi), %rax
        mulq     0x18(%rcx)
        addq    %rax, %r14
        adcq    %rdx, %r15
        movl    $0x26, %esi
        movq    %r12, %rax
        mulq    %rsi
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %rcx, %rcx
        movq    %r13, %rax
        mulq    %rsi
        subq    %rcx, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r14, %rax
        mulq    %rsi
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    %r15, %rax
        mulq    %rsi
        subq    %rcx, %rdx
        xorq    %rcx, %rcx
        addq    %rax, %r11
        movq    %rdx, %r12
        adcq    %rcx, %r12
        shldq   $0x1, %r11, %r12
        leaq    0x1(%r12), %rax
        movl    $0x13, %esi
        bts     $0x3f, %r11
        imulq   %rsi, %rax
        addq    %rax, %r8
        adcq    %rcx, %r9
        adcq    %rcx, %r10
        adcq    %rcx, %r11
        sbbq    %rax, %rax
        notq    %rax
        andq    %rsi, %rax
        subq    %rax, %r8
        sbbq    %rcx, %r9
        sbbq    %rcx, %r10
        sbbq    %rcx, %r11
        btr     $0x3f, %r11
        movq    %r8, (%rdi)
        movq    %r9, 0x8(%rdi)
        movq    %r10, 0x10(%rdi)
        movq    %r11, 0x18(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_decode_alt_mul_p25519)

// *************************************************************
// Local z = 2^n * x
// *************************************************************

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Ledwards25519_decode_alt_nsqr_p25519)

Ledwards25519_decode_alt_nsqr_p25519:
        CFI_START

// Copy input argument into q

        movq    (%rdx), %rax
        movq    8(%rdx), %rbx
        movq    16(%rdx), %rcx
        movq    24(%rdx), %rdx
        movq    %rax, Q8(%rsp)
        movq    %rbx, Q8+8(%rsp)
        movq    %rcx, Q8+16(%rsp)
        movq    %rdx, Q8+24(%rsp)

// Main squaring loop, accumulating in u consistently  and
// only ensuring the intermediates are < 2 * p_25519 = 2^256 - 38

Ledwards25519_decode_alt_loop:
        movq    Q8(%rsp), %rax
        mulq    %rax
        movq    %rax, %r8
        movq    %rdx, %r9
        xorq    %r10, %r10
        xorq    %r11, %r11
        movq    Q8(%rsp), %rax
        mulq     Q8+0x8(%rsp)
        addq    %rax, %rax
        adcq    %rdx, %rdx
        adcq    $0x0, %r11
        addq    %rax, %r9
        adcq    %rdx, %r10
        adcq    $0x0, %r11
        xorq    %r12, %r12
        movq    Q8+0x8(%rsp), %rax
        mulq    %rax
        addq    %rax, %r10
        adcq    %rdx, %r11
        adcq    $0x0, %r12
        movq    Q8(%rsp), %rax
        mulq     Q8+0x10(%rsp)
        addq    %rax, %rax
        adcq    %rdx, %rdx
        adcq    $0x0, %r12
        addq    %rax, %r10
        adcq    %rdx, %r11
        adcq    $0x0, %r12
        xorq    %r13, %r13
        movq    Q8(%rsp), %rax
        mulq     Q8+0x18(%rsp)
        addq    %rax, %rax
        adcq    %rdx, %rdx
        adcq    $0x0, %r13
        addq    %rax, %r11
        adcq    %rdx, %r12
        adcq    $0x0, %r13
        movq    Q8+0x8(%rsp), %rax
        mulq     Q8+0x10(%rsp)
        addq    %rax, %rax
        adcq    %rdx, %rdx
        adcq    $0x0, %r13
        addq    %rax, %r11
        adcq    %rdx, %r12
        adcq    $0x0, %r13
        xorq    %r14, %r14
        movq    Q8+0x8(%rsp), %rax
        mulq     Q8+0x18(%rsp)
        addq    %rax, %rax
        adcq    %rdx, %rdx
        adcq    $0x0, %r14
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        movq    Q8+0x10(%rsp), %rax
        mulq    %rax
        addq    %rax, %r12
        adcq    %rdx, %r13
        adcq    $0x0, %r14
        xorq    %r15, %r15
        movq    Q8+0x10(%rsp), %rax
        mulq     Q8+0x18(%rsp)
        addq    %rax, %rax
        adcq    %rdx, %rdx
        adcq    $0x0, %r15
        addq    %rax, %r13
        adcq    %rdx, %r14
        adcq    $0x0, %r15
        movq    Q8+0x18(%rsp), %rax
        mulq    %rax
        addq    %rax, %r14
        adcq    %rdx, %r15
        movl    $0x26, %ebx
        movq    %r12, %rax
        mulq    %rbx
        addq    %rax, %r8
        adcq    %rdx, %r9
        sbbq    %rcx, %rcx
        movq    %r13, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r9
        adcq    %rdx, %r10
        sbbq    %rcx, %rcx
        movq    %r14, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        addq    %rax, %r10
        adcq    %rdx, %r11
        sbbq    %rcx, %rcx
        movq    %r15, %rax
        mulq    %rbx
        subq    %rcx, %rdx
        xorq    %rcx, %rcx
        addq    %rax, %r11
        movq    %rdx, %r12
        adcq    %rcx, %r12
        shldq   $0x1, %r11, %r12
        btr     $0x3f, %r11
        movl    $0x13, %edx
        imulq   %r12, %rdx
        addq    %rdx, %r8
        adcq    %rcx, %r9
        adcq    %rcx, %r10
        adcq    %rcx, %r11
        movq    %r8, Q8(%rsp)
        movq    %r9, Q8+0x8(%rsp)
        movq    %r10, Q8+0x10(%rsp)
        movq    %r11, Q8+0x18(%rsp)

// Loop as applicable

        decq    %rsi
        jnz     Ledwards25519_decode_alt_loop

// We know the intermediate result x < 2^256 - 38, and now we do strict
// modular reduction mod 2^255 - 19. Note x < 2^255 - 19 <=> x + 19 < 2^255
// which is equivalent to a "ns" condition. We just use the results where
// they were in registers [%r11;%r10;%r9;%r8] instead of re-loading them.

        movl    $19, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        addq    %r8, %rax
        adcq    %r9, %rbx
        adcq    %r10, %rcx
        adcq    %r11, %rdx

        cmovns  %r8, %rax
        cmovns  %r9, %rbx
        cmovns  %r10, %rcx
        cmovns  %r11, %rdx
        btr     $63, %rdx
        movq    %rax, (%rdi)
        movq    %rbx, 8(%rdi)
        movq    %rcx, 16(%rdi)
        movq    %rdx, 24(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(Ledwards25519_decode_alt_nsqr_p25519)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
