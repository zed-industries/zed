// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// The x25519 function for curve25519 on base element 9
// Input scalar[4]; output res[4]
//
// extern void curve25519_x25519base
//   (uint64_t res[static 4], const uint64_t scalar[static 4]);
//
// The function has a second prototype considering the arguments as arrays
// of bytes rather than 64-bit words. The underlying code is the same, since
// the x86 platform is little-endian.
//
// extern void curve25519_x25519base_byte
//   (uint8_t res[static 32],const uint8_t scalar[static 32]);
//
// Given a scalar n, returns the X coordinate of n * G where G = (9,...) is
// the standard generator. The scalar is first slightly modified/mangled
// as specified in the relevant RFC (https://www.rfc-editor.org/rfc/rfc7748).
//
// Standard x86-64 ABI: RDI = res, RSI = scalar
// Microsoft x64 ABI:   RCX = res, RDX = scalar
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(curve25519_x25519base)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(curve25519_x25519base)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(curve25519_x25519base)
        S2N_BN_SYM_VISIBILITY_DIRECTIVE(curve25519_x25519base_byte)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(curve25519_x25519base_byte)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(curve25519_x25519base_byte)

        .text

// Size of individual field elements

#define NUMSIZE 32

// Pointer-offset pairs for result and temporaries on stack with some aliasing.
// The result "resx" assumes the "res" pointer has been preloaded into %rbp.

#define resx (0*NUMSIZE)(%rbp)

#define scalar (0*NUMSIZE)(%rsp)

#define tabent (1*NUMSIZE)(%rsp)
#define ymx_2 (1*NUMSIZE)(%rsp)
#define xpy_2 (2*NUMSIZE)(%rsp)
#define kxy_2 (3*NUMSIZE)(%rsp)

#define acc (4*NUMSIZE)(%rsp)
#define x_1 (4*NUMSIZE)(%rsp)
#define y_1 (5*NUMSIZE)(%rsp)
#define z_1 (6*NUMSIZE)(%rsp)
#define w_1 (7*NUMSIZE)(%rsp)
#define x_3 (4*NUMSIZE)(%rsp)
#define y_3 (5*NUMSIZE)(%rsp)
#define z_3 (6*NUMSIZE)(%rsp)
#define w_3 (7*NUMSIZE)(%rsp)

#define tmpspace (8*NUMSIZE)(%rsp)
#define t0 (8*NUMSIZE)(%rsp)
#define t1 (9*NUMSIZE)(%rsp)
#define t2 (10*NUMSIZE)(%rsp)
#define t3 (11*NUMSIZE)(%rsp)
#define t4 (12*NUMSIZE)(%rsp)
#define t5 (13*NUMSIZE)(%rsp)

// Stable homes for the input result pointer, and other variables

#define res  14*NUMSIZE(%rsp)

#define i  14*NUMSIZE+8(%rsp)

#define bias  14*NUMSIZE+16(%rsp)

#define bf  14*NUMSIZE+24(%rsp)
#define ix  14*NUMSIZE+24(%rsp)

#define tab  15*NUMSIZE(%rsp)

// Total size to reserve on the stack

#define NSPACE 488

// Macro wrapping up the basic field multiplication, only trivially
// different from a pure function call to bignum_mul_p25519.

#define mul_p25519(P0,P1,P2)                    \
        xorl   %esi, %esi ;                        \
        movq   P2, %rdx ;                       \
        mulxq  P1, %r8, %r9 ;                    \
        mulxq  0x8+P1, %rax, %r10 ;              \
        addq   %rax, %r9 ;                         \
        mulxq  0x10+P1, %rax, %r11 ;             \
        adcq   %rax, %r10 ;                        \
        mulxq  0x18+P1, %rax, %r12 ;             \
        adcq   %rax, %r11 ;                        \
        adcq   %rsi, %r12 ;                        \
        xorl   %esi, %esi ;                        \
        movq   0x8+P2, %rdx ;                   \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r9 ;                         \
        adoxq  %rbx, %r10 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x18+P1, %rax, %r13 ;             \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rsi, %r13 ;                        \
        adcxq  %rsi, %r13 ;                        \
        xorl   %esi, %esi ;                        \
        movq   0x10+P2, %rdx ;                  \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rbx, %r13 ;                        \
        mulxq  0x18+P1, %rax, %r14 ;             \
        adcxq  %rax, %r13 ;                        \
        adoxq  %rsi, %r14 ;                        \
        adcxq  %rsi, %r14 ;                        \
        xorl   %esi, %esi ;                        \
        movq   0x18+P2, %rdx ;                  \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rbx, %r13 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r13 ;                        \
        adoxq  %rbx, %r14 ;                        \
        mulxq  0x18+P1, %rax, %r15 ;             \
        adcxq  %rax, %r14 ;                        \
        adoxq  %rsi, %r15 ;                        \
        adcxq  %rsi, %r15 ;                        \
        movl   $0x26, %edx ;                       \
        xorl   %esi, %esi ;                        \
        mulxq  %r12, %rax, %rbx ;                   \
        adcxq  %rax, %r8 ;                         \
        adoxq  %rbx, %r9 ;                         \
        mulxq  %r13, %rax, %rbx ;                   \
        adcxq  %rax, %r9 ;                         \
        adoxq  %rbx, %r10 ;                        \
        mulxq  %r14, %rax, %rbx ;                   \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  %r15, %rax, %r12 ;                   \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rsi, %r12 ;                        \
        adcxq  %rsi, %r12 ;                        \
        shldq  $0x1, %r11, %r12 ;                   \
        movl   $0x13, %edx ;                       \
        incq   %r12;                             \
        bts    $63, %r11 ;                         \
        mulxq  %r12, %rax, %rbx ;                   \
        addq   %rax, %r8 ;                         \
        adcq   %rbx, %r9 ;                         \
        adcq   %rsi, %r10 ;                        \
        adcq   %rsi, %r11 ;                        \
        sbbq   %rax, %rax ;                        \
        notq   %rax;                             \
        andq   %rdx, %rax ;                        \
        subq   %rax, %r8 ;                         \
        sbbq   %rsi, %r9 ;                         \
        sbbq   %rsi, %r10 ;                        \
        sbbq   %rsi, %r11 ;                        \
        btr    $63, %r11 ;                         \
        movq   %r8, P0 ;                        \
        movq   %r9, 0x8+P0 ;                    \
        movq   %r10, 0x10+P0 ;                  \
        movq   %r11, 0x18+P0

// A version of multiplication that only guarantees output < 2 * p_25519.
// This basically skips the +1 and final correction in quotient estimation.

#define mul_4(P0,P1,P2)                         \
        xorl   %ecx, %ecx ;                        \
        movq   P2, %rdx ;                       \
        mulxq  P1, %r8, %r9 ;                    \
        mulxq  0x8+P1, %rax, %r10 ;              \
        addq   %rax, %r9 ;                         \
        mulxq  0x10+P1, %rax, %r11 ;             \
        adcq   %rax, %r10 ;                        \
        mulxq  0x18+P1, %rax, %r12 ;             \
        adcq   %rax, %r11 ;                        \
        adcq   %rcx, %r12 ;                        \
        xorl   %ecx, %ecx ;                        \
        movq   0x8+P2, %rdx ;                   \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r9 ;                         \
        adoxq  %rbx, %r10 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x18+P1, %rax, %r13 ;             \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rcx, %r13 ;                        \
        adcxq  %rcx, %r13 ;                        \
        xorl   %ecx, %ecx ;                        \
        movq   0x10+P2, %rdx ;                  \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rbx, %r13 ;                        \
        mulxq  0x18+P1, %rax, %r14 ;             \
        adcxq  %rax, %r13 ;                        \
        adoxq  %rcx, %r14 ;                        \
        adcxq  %rcx, %r14 ;                        \
        xorl   %ecx, %ecx ;                        \
        movq   0x18+P2, %rdx ;                  \
        mulxq  P1, %rax, %rbx ;                  \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rbx, %r12 ;                        \
        mulxq  0x8+P1, %rax, %rbx ;              \
        adcxq  %rax, %r12 ;                        \
        adoxq  %rbx, %r13 ;                        \
        mulxq  0x10+P1, %rax, %rbx ;             \
        adcxq  %rax, %r13 ;                        \
        adoxq  %rbx, %r14 ;                        \
        mulxq  0x18+P1, %rax, %r15 ;             \
        adcxq  %rax, %r14 ;                        \
        adoxq  %rcx, %r15 ;                        \
        adcxq  %rcx, %r15 ;                        \
        movl   $0x26, %edx ;                       \
        xorl   %ecx, %ecx ;                        \
        mulxq  %r12, %rax, %rbx ;                   \
        adcxq  %rax, %r8 ;                         \
        adoxq  %rbx, %r9 ;                         \
        mulxq  %r13, %rax, %rbx ;                   \
        adcxq  %rax, %r9 ;                         \
        adoxq  %rbx, %r10 ;                        \
        mulxq  %r14, %rax, %rbx ;                   \
        adcxq  %rax, %r10 ;                        \
        adoxq  %rbx, %r11 ;                        \
        mulxq  %r15, %rax, %r12 ;                   \
        adcxq  %rax, %r11 ;                        \
        adoxq  %rcx, %r12 ;                        \
        adcxq  %rcx, %r12 ;                        \
        shldq  $0x1, %r11, %r12 ;                   \
        btr    $0x3f, %r11 ;                       \
        movl   $0x13, %edx ;                       \
        imulq  %r12, %rdx ;                        \
        addq   %rdx, %r8 ;                         \
        adcq   %rcx, %r9 ;                         \
        adcq   %rcx, %r10 ;                        \
        adcq   %rcx, %r11 ;                        \
        movq   %r8, P0 ;                        \
        movq   %r9, 0x8+P0 ;                    \
        movq   %r10, 0x10+P0 ;                  \
        movq   %r11, 0x18+P0

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

// Modular addition and doubling with double modulus 2 * p_25519 = 2^256 - 38.
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

#define double_twice4(P0,P1)                    \
        movq    P1, %r8 ;                       \
        xorl    %ecx, %ecx ;                       \
        addq    %r8, %r8 ;                         \
        movq    0x8+P1, %r9 ;                   \
        adcq    %r9, %r9 ;                         \
        movq    0x10+P1, %r10 ;                 \
        adcq    %r10, %r10 ;                       \
        movq    0x18+P1, %r11 ;                 \
        adcq    %r11, %r11 ;                       \
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

S2N_BN_SYMBOL(curve25519_x25519base):
S2N_BN_SYMBOL(curve25519_x25519base_byte):
        CFI_START
        _CET_ENDBR

// In this case the Windows form literally makes a subroutine call.
// This avoids hassle arising from keeping code and data together.

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        CFI_CALL(Lcurve25519_x25519base_standard)
        CFI_POP(%rsi)
        CFI_POP(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(curve25519_x25519base)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lcurve25519_x25519base_standard)

Lcurve25519_x25519base_standard:
        CFI_START
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

// Copy the input scalar to its local variable while mangling it.
// In principle the mangling is into 01xxx...xxx000, but actually
// we only clear the top two bits so 00xxx...xxxxxx. The additional
// 2^254 * G is taken care of by the starting value for the addition
// chain below, while we never look at the three low bits at all.

        movq    (%rsi), %rax
        movq    %rax, (%rsp)
        movq    8(%rsi), %rax
        movq    %rax, 8(%rsp)
        movq    16(%rsi), %rax
        movq    %rax, 16(%rsp)
        movq    $0x3fffffffffffffff, %rax
        andq    24(%rsi), %rax
        movq    %rax, 24(%rsp)

// The main part of the computation is on the edwards25519 curve in
// extended-projective coordinates (X,Y,Z,T), representing a point
// (x,y) via x = X/Z, y = Y/Z and x * y = T/Z (so X * Y = T * Z).
// Only at the very end do we translate back to curve25519. So G
// below means the generator within edwards25519 corresponding to
// (9,...) for curve25519, via the standard isomorphism.
//
// Initialize accumulator "acc" to either (2^254 + 8) * G or just 2^254 * G
// depending on bit 3 of the scalar, the only nonzero bit of the bottom 4.
// Thus, we have effectively dealt with bits 0, 1, 2, 3, 254 and 255.

        movq    (%rsp), %rax
        andq    $8, %rax

        leaq    Lcurve25519_x25519base_edwards25519_0g(%rip), %r10
        leaq    Lcurve25519_x25519base_edwards25519_8g(%rip), %r11

        movq    (%r10), %rax
        movq    (%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*16(%rsp)

        movq    8*1(%r10), %rax
        movq    8*1(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*17(%rsp)

        movq    8*2(%r10), %rax
        movq    8*2(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*18(%rsp)

        movq    8*3(%r10), %rax
        movq    8*3(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*19(%rsp)

        movq    8*4(%r10), %rax
        movq    8*4(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*20(%rsp)

        movq    8*5(%r10), %rax
        movq    8*5(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*21(%rsp)

        movq    8*6(%r10), %rax
        movq    8*6(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*22(%rsp)

        movq    8*7(%r10), %rax
        movq    8*7(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*23(%rsp)

        movl    $1, %eax
        movq    %rax, 8*24(%rsp)
        movl    $0, %eax
        movq    %rax, 8*25(%rsp)
        movq    %rax, 8*26(%rsp)
        movq    %rax, 8*27(%rsp)

        movq    8*8(%r10), %rax
        movq    8*8(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*28(%rsp)

        movq    8*9(%r10), %rax
        movq    8*9(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*29(%rsp)

        movq    8*10(%r10), %rax
        movq    8*10(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*30(%rsp)

        movq    8*11(%r10), %rax
        movq    8*11(%r11), %rcx
        cmovnzq %rcx, %rax
        movq    %rax, 8*31(%rsp)

// The counter "i" tracks the bit position for which the scalar has
// already been absorbed, starting at 4 and going up in chunks of 4.
//
// The pointer "tab" points at the current block of the table for
// multiples (2^i * j) * G at the current bit position i; 1 <= j <= 8.
//
// The bias is always either 0 and 1 and needs to be added to the
// partially processed scalar implicitly. This is used to absorb 4 bits
// of scalar per iteration from 3-bit table indexing by exploiting
// negation: (16 * h + l) * G = (16 * (h + 1) - (16 - l)) * G is used
// when l >= 9. Note that we can't have any bias left over at the
// end because of the clearing of bit 255 of the scalar, meaning the
// l >= 9 case cannot arise on the last iteration.

        movq    $4, i
        leaq    Lcurve25519_x25519base_edwards25519_gtable(%rip), %rax
        movq    %rax, tab
        movq    $0, bias

// Start of the main loop, repeated 63 times for i = 4, 8, ..., 252

Lcurve25519_x25519base_scalarloop:

// Look at the next 4-bit field "bf", adding the previous bias as well.
// Choose the table index "ix" as bf when bf <= 8 and 16 - bf for bf >= 9,
// setting the bias to 1 for the next iteration in the latter case.

        movq    i, %rax
        movq    %rax, %rcx
        shrq    $6, %rax
        movq    (%rsp,%rax,8), %rax // Exploiting scalar = sp exactly
        shrq    %cl, %rax
        andq    $15, %rax
        addq    bias, %rax
        movq    %rax, bf

        cmpq    $9, bf
        sbbq    %rax, %rax
        incq    %rax
        movq    %rax, bias

        movq    $16, %rdi
        subq    bf, %rdi
        cmpq    $0, bias
        cmovzq  bf, %rdi
        movq    %rdi, ix

// Perform constant-time lookup in the table to get element number "ix".
// The table entry for the affine point (x,y) is actually a triple
// (y - x,x + y,2 * d * x * y) to precompute parts of the addition.
// Note that "ix" can be 0, so we set up the appropriate identity first.

        movl    $1, %eax
        xorl    %ebx, %ebx
        xorl    %ecx, %ecx
        xorl    %edx, %edx
        movl    $1, %r8d
        xorl    %r9d, %r9d
        xorl    %r10d, %r10d
        xorl    %r11d, %r11d
        xorl    %r12d, %r12d
        xorl    %r13d, %r13d
        xorl    %r14d, %r14d
        xorl    %r15d, %r15d

        movq    tab, %rbp

        cmpq    $1, ix
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $2, ix
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $3, ix
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $4, ix
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $5, ix
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $6, ix
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $7, ix
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15
        addq    $96, %rbp

        cmpq    $8, ix
        movq    (%rbp), %rsi
        cmovzq  %rsi, %rax
        movq    8(%rbp), %rsi
        cmovzq  %rsi, %rbx
        movq    16(%rbp), %rsi
        cmovzq  %rsi, %rcx
        movq    24(%rbp), %rsi
        cmovzq  %rsi, %rdx
        movq    32(%rbp), %rsi
        cmovzq  %rsi, %r8
        movq    40(%rbp), %rsi
        cmovzq  %rsi, %r9
        movq    48(%rbp), %rsi
        cmovzq  %rsi, %r10
        movq    56(%rbp), %rsi
        cmovzq  %rsi, %r11
        movq    64(%rbp), %rsi
        cmovzq  %rsi, %r12
        movq    72(%rbp), %rsi
        cmovzq  %rsi, %r13
        movq    80(%rbp), %rsi
        cmovzq  %rsi, %r14
        movq    88(%rbp), %rsi
        cmovzq  %rsi, %r15

        addq    $96, %rbp
        movq    %rbp, tab

// We now have the triple from the table in registers as follows
//
//      [%rdx;%rcx;%rbx;%rax] = y - x
//      [%r11;%r10;%r9;%r8] = x + y
//      [%r15;%r14;%r13;%r12] = 2 * d * x * y
//
// In case bias = 1 we need to negate this. For Edwards curves
// -(x,y) = (-x,y), i.e. we need to negate the x coordinate.
// In this processed encoding, that amounts to swapping the
// first two fields and negating the third.
//
// The optional negation here also pretends bias = 0 whenever
// ix = 0 so that it doesn't need to handle the case of zero
// inputs, since no non-trivial table entries are zero. Note
// that in the zero case the whole negation is trivial, and
// so indeed is the swapping.

        cmpq    $0, bias

        movq    %rax, %rsi
        cmovnzq %r8, %rsi
        cmovnzq %rax, %r8
        movq    %rsi, 32(%rsp)
        movq    %r8, 64(%rsp)

        movq    %rbx, %rsi
        cmovnzq %r9, %rsi
        cmovnzq %rbx, %r9
        movq    %rsi, 40(%rsp)
        movq    %r9, 72(%rsp)

        movq    %rcx, %rsi
        cmovnzq %r10, %rsi
        cmovnzq %rcx, %r10
        movq    %rsi, 48(%rsp)
        movq    %r10, 80(%rsp)

        movq    %rdx, %rsi
        cmovnzq %r11, %rsi
        cmovnzq %rdx, %r11
        movq    %rsi, 56(%rsp)
        movq    %r11, 88(%rsp)

        movq    $-19, %rax
        movq    $-1, %rbx
        movq    $-1, %rcx
        movq    $0x7fffffffffffffff, %rdx
        subq    %r12, %rax
        sbbq    %r13, %rbx
        sbbq    %r14, %rcx
        sbbq    %r15, %rdx

        movq    ix, %r8
        movq    bias, %r9
        testq   %r8, %r8
        cmovzq  %r8, %r9
        testq   %r9, %r9

        cmovzq  %r12, %rax
        cmovzq  %r13, %rbx
        cmovzq  %r14, %rcx
        cmovzq  %r15, %rdx
        movq    %rax, 96(%rsp)
        movq    %rbx, 104(%rsp)
        movq    %rcx, 112(%rsp)
        movq    %rdx, 120(%rsp)

// Extended-projective and precomputed mixed addition.
// This is effectively the same as calling the standalone
// function edwards25519_pepadd(acc,acc,tabent), but we
// only retain slightly weaker normalization < 2 * p_25519
// throughout the inner loop, so the computation is
// slightly different, and faster overall.

        double_twice4(t0,z_1)
        sub_twice4(t1,y_1,x_1)
        add_twice4(t2,y_1,x_1)
        mul_4(t3,w_1,kxy_2)
        mul_4(t1,t1,ymx_2)
        mul_4(t2,t2,xpy_2)
        sub_twice4(t4,t0,t3)
        add_twice4(t0,t0,t3)
        sub_twice4(t5,t2,t1)
        add_twice4(t1,t2,t1)
        mul_4(z_3,t4,t0)
        mul_4(x_3,t5,t4)
        mul_4(y_3,t0,t1)
        mul_4(w_3,t5,t1)

// End of the main loop; move on by 4 bits.

        addq    $4, i
        cmpq    $256, i
        jc      Lcurve25519_x25519base_scalarloop

// Now we need to translate from Edwards curve edwards25519 back
// to the Montgomery form curve25519. The mapping in the affine
// representations is
//
// (x,y) |-> ((1 + y) / (1 - y), c * (1 + y) / ((1 - y) * x))
//
// For x25519, we only need the x coordinate, and we compute this as
//
// (1 + y) / (1 - y) = (x + x * y) / (x - x * y)
//                   = (X/Z + T/Z) / (X/Z - T/Z)
//                   = (X + T) / (X - T)
//                   = (X + T) * inverse(X - T)
//
// We could equally well use (Z + Y) / (Z - Y), but the above has the
// same cost, and it more explicitly forces zero output whenever X = 0,
// regardless of how the modular inverse behaves on zero inputs. In
// the present setting (base point 9, mangled scalar) that doesn't
// really matter anyway since X = 0 never arises, but it seems a
// little bit tidier. Note that both Edwards point (0,1) which maps to
// the Montgomery point at infinity, and Edwards (0,-1) which maps to
// Montgomery (0,0) [this is the 2-torsion point] are both by definition
// mapped to 0 by the X coordinate mapping used to define curve25519.
//
// First the addition and subtraction:

        add_twice4(t1,x_3,w_3)
        sub_twice4(t2,x_3,w_3)

// Prepare to call the modular inverse function to get t0 = 1/t2
// Note that this works for the weakly normalized z_3 equally well.
// The non-coprime case z_3 == 0 (mod p_25519) cannot arise anyway.

        leaq    256(%rsp), %rdi
        leaq    320(%rsp), %rsi

// Inline copy of bignum_inv_p25519, identical except for stripping out
// the prologue and epilogue saving and restoring registers and making
// and reclaiming room on the stack. For more details and explanations see
// "x86/curve25519/bignum_inv_p25519.S". Note that the stack it uses for
// its own temporaries is 208 bytes, so it has no effect on variables
// that are needed in the rest of our computation here: res, t0, t1, t2.

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
        jmp     Lcurve25519_x25519base_midloop
Lcurve25519_x25519base_inverseloop:
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
Lcurve25519_x25519base_midloop:
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
        jne     Lcurve25519_x25519base_inverseloop
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

// The final result is (X + T) / (X - T)
// This is the only operation in the whole computation that
// fully reduces modulo p_25519 since now we want the canonical
// answer as output.

        movq    res, %rbp
        mul_p25519(resx,t1,t0)

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
S2N_BN_SIZE_DIRECTIVE(Lcurve25519_x25519base_standard)
#else
S2N_BN_SIZE_DIRECTIVE(curve25519_x25519base)
#endif

// ****************************************************************************
// The precomputed data (all read-only). This is currently part of the same
// text section, which gives position-independent code with simple PC-relative
// addressing. However it could be put in a separate section via something like
//
// .section .rodata
// ****************************************************************************

// 2^254 * G and (2^254 + 8) * G in extended-projective coordinates
// but with z = 1 assumed and hence left out, so they are (X,Y,T) only.

Lcurve25519_x25519base_edwards25519_0g:

        .quad   0x251037f7cf4e861d
        .quad   0x10ede0fb19fb128f
        .quad   0x96c033b175f5e2c8
        .quad   0x055f070d6c15fb0d

        .quad   0x7c52af2c97473e69
        .quad   0x022f82391bad8378
        .quad   0x9991e1b02adb476f
        .quad   0x511144a03a99b855

        .quad   0x5fafc3b88ff2e4ae
        .quad   0x855e4ff0de1230ff
        .quad   0x72e302a348492870
        .quad   0x1253c19e53dbe1bc

Lcurve25519_x25519base_edwards25519_8g:

        .quad   0x331d086e0d9abcaa
        .quad   0x1e23c96d311a10c9
        .quad   0x96d0f95e58c13478
        .quad   0x2f72f7384fcfcc59

        .quad   0x39a6cd1cfd7d87c9
        .quad   0x9867a0abd8ae153a
        .quad   0xa49d2a5f35986745
        .quad   0x57012940cdfe82e1

        .quad   0x5046a6532ec5544a
        .quad   0x6d674004739ff6c9
        .quad   0x9bbaa44b234a70e3
        .quad   0x5e6d8901138cf386

// Precomputed table of multiples of generator for edwards25519
// all in precomputed extended-projective (y-x,x+y,2*d*x*y) triples.

Lcurve25519_x25519base_edwards25519_gtable:

        // 2^4 * 1 * G

        .quad   0x7ec851ca553e2df3
        .quad   0xa71284cba64878b3
        .quad   0xe6b5e4193288d1e7
        .quad   0x4cf210ec5a9a8883
        .quad   0x322d04a52d9021f6
        .quad   0xb9c19f3375c6bf9c
        .quad   0x587a3a4342d20b09
        .quad   0x143b1cf8aa64fe61
        .quad   0x9f867c7d968acaab
        .quad   0x5f54258e27092729
        .quad   0xd0a7d34bea180975
        .quad   0x21b546a3374126e1

        // 2^4 * 2 * G

        .quad   0xa94ff858a2888343
        .quad   0xce0ed4565313ed3c
        .quad   0xf55c3dcfb5bf34fa
        .quad   0x0a653ca5c9eab371
        .quad   0x490a7a45d185218f
        .quad   0x9a15377846049335
        .quad   0x0060ea09cc31e1f6
        .quad   0x7e041577f86ee965
        .quad   0x66b2a496ce5b67f3
        .quad   0xff5492d8bd569796
        .quad   0x503cec294a592cd0
        .quad   0x566943650813acb2

        // 2^4 * 3 * G

        .quad   0xb818db0c26620798
        .quad   0x5d5c31d9606e354a
        .quad   0x0982fa4f00a8cdc7
        .quad   0x17e12bcd4653e2d4
        .quad   0x5672f9eb1dabb69d
        .quad   0xba70b535afe853fc
        .quad   0x47ac0f752796d66d
        .quad   0x32a5351794117275
        .quad   0xd3a644a6df648437
        .quad   0x703b6559880fbfdd
        .quad   0xcb852540ad3a1aa5
        .quad   0x0900b3f78e4c6468

        // 2^4 * 4 * G

        .quad   0x0a851b9f679d651b
        .quad   0xe108cb61033342f2
        .quad   0xd601f57fe88b30a3
        .quad   0x371f3acaed2dd714
        .quad   0xed280fbec816ad31
        .quad   0x52d9595bd8e6efe3
        .quad   0x0fe71772f6c623f5
        .quad   0x4314030b051e293c
        .quad   0xd560005efbf0bcad
        .quad   0x8eb70f2ed1870c5e
        .quad   0x201f9033d084e6a0
        .quad   0x4c3a5ae1ce7b6670

        // 2^4 * 5 * G

        .quad   0x4138a434dcb8fa95
        .quad   0x870cf67d6c96840b
        .quad   0xde388574297be82c
        .quad   0x7c814db27262a55a
        .quad   0xbaf875e4c93da0dd
        .quad   0xb93282a771b9294d
        .quad   0x80d63fb7f4c6c460
        .quad   0x6de9c73dea66c181
        .quad   0x478904d5a04df8f2
        .quad   0xfafbae4ab10142d3
        .quad   0xf6c8ac63555d0998
        .quad   0x5aac4a412f90b104

        // 2^4 * 6 * G

        .quad   0xc64f326b3ac92908
        .quad   0x5551b282e663e1e0
        .quad   0x476b35f54a1a4b83
        .quad   0x1b9da3fe189f68c2
        .quad   0x603a0d0abd7f5134
        .quad   0x8089c932e1d3ae46
        .quad   0xdf2591398798bd63
        .quad   0x1c145cd274ba0235
        .quad   0x32e8386475f3d743
        .quad   0x365b8baf6ae5d9ef
        .quad   0x825238b6385b681e
        .quad   0x234929c1167d65e1

        // 2^4 * 7 * G

        .quad   0x984decaba077ade8
        .quad   0x383f77ad19eb389d
        .quad   0xc7ec6b7e2954d794
        .quad   0x59c77b3aeb7c3a7a
        .quad   0x48145cc21d099fcf
        .quad   0x4535c192cc28d7e5
        .quad   0x80e7c1e548247e01
        .quad   0x4a5f28743b2973ee
        .quad   0xd3add725225ccf62
        .quad   0x911a3381b2152c5d
        .quad   0xd8b39fad5b08f87d
        .quad   0x6f05606b4799fe3b

        // 2^4 * 8 * G

        .quad   0x9ffe9e92177ba962
        .quad   0x98aee71d0de5cae1
        .quad   0x3ff4ae942d831044
        .quad   0x714de12e58533ac8
        .quad   0x5b433149f91b6483
        .quad   0xadb5dc655a2cbf62
        .quad   0x87fa8412632827b3
        .quad   0x60895e91ab49f8d8
        .quad   0xe9ecf2ed0cf86c18
        .quad   0xb46d06120735dfd4
        .quad   0xbc9da09804b96be7
        .quad   0x73e2e62fd96dc26b

        // 2^8 * 1 * G

        .quad   0xed5b635449aa515e
        .quad   0xa865c49f0bc6823a
        .quad   0x850c1fe95b42d1c4
        .quad   0x30d76d6f03d315b9
        .quad   0x2eccdd0e632f9c1d
        .quad   0x51d0b69676893115
        .quad   0x52dfb76ba8637a58
        .quad   0x6dd37d49a00eef39
        .quad   0x6c4444172106e4c7
        .quad   0xfb53d680928d7f69
        .quad   0xb4739ea4694d3f26
        .quad   0x10c697112e864bb0

        // 2^8 * 2 * G

        .quad   0x6493c4277dbe5fde
        .quad   0x265d4fad19ad7ea2
        .quad   0x0e00dfc846304590
        .quad   0x25e61cabed66fe09
        .quad   0x0ca62aa08358c805
        .quad   0x6a3d4ae37a204247
        .quad   0x7464d3a63b11eddc
        .quad   0x03bf9baf550806ef
        .quad   0x3f13e128cc586604
        .quad   0x6f5873ecb459747e
        .quad   0xa0b63dedcc1268f5
        .quad   0x566d78634586e22c

        // 2^8 * 3 * G

        .quad   0x1637a49f9cc10834
        .quad   0xbc8e56d5a89bc451
        .quad   0x1cb5ec0f7f7fd2db
        .quad   0x33975bca5ecc35d9
        .quad   0xa1054285c65a2fd0
        .quad   0x6c64112af31667c3
        .quad   0x680ae240731aee58
        .quad   0x14fba5f34793b22a
        .quad   0x3cd746166985f7d4
        .quad   0x593e5e84c9c80057
        .quad   0x2fc3f2b67b61131e
        .quad   0x14829cea83fc526c

        // 2^8 * 4 * G

        .quad   0xff437b8497dd95c2
        .quad   0x6c744e30aa4eb5a7
        .quad   0x9e0c5d613c85e88b
        .quad   0x2fd9c71e5f758173
        .quad   0x21e70b2f4e71ecb8
        .quad   0xe656ddb940a477e3
        .quad   0xbf6556cece1d4f80
        .quad   0x05fc3bc4535d7b7e
        .quad   0x24b8b3ae52afdedd
        .quad   0x3495638ced3b30cf
        .quad   0x33a4bc83a9be8195
        .quad   0x373767475c651f04

        // 2^8 * 5 * G

        .quad   0x2fba99fd40d1add9
        .quad   0xb307166f96f4d027
        .quad   0x4363f05215f03bae
        .quad   0x1fbea56c3b18f999
        .quad   0x634095cb14246590
        .quad   0xef12144016c15535
        .quad   0x9e38140c8910bc60
        .quad   0x6bf5905730907c8c
        .quad   0x0fa778f1e1415b8a
        .quad   0x06409ff7bac3a77e
        .quad   0x6f52d7b89aa29a50
        .quad   0x02521cf67a635a56

        // 2^8 * 6 * G

        .quad   0x513fee0b0a9d5294
        .quad   0x8f98e75c0fdf5a66
        .quad   0xd4618688bfe107ce
        .quad   0x3fa00a7e71382ced
        .quad   0xb1146720772f5ee4
        .quad   0xe8f894b196079ace
        .quad   0x4af8224d00ac824a
        .quad   0x001753d9f7cd6cc4
        .quad   0x3c69232d963ddb34
        .quad   0x1dde87dab4973858
        .quad   0xaad7d1f9a091f285
        .quad   0x12b5fe2fa048edb6

        // 2^8 * 7 * G

        .quad   0x71f0fbc496fce34d
        .quad   0x73b9826badf35bed
        .quad   0xd2047261ff28c561
        .quad   0x749b76f96fb1206f
        .quad   0xdf2b7c26ad6f1e92
        .quad   0x4b66d323504b8913
        .quad   0x8c409dc0751c8bc3
        .quad   0x6f7e93c20796c7b8
        .quad   0x1f5af604aea6ae05
        .quad   0xc12351f1bee49c99
        .quad   0x61a808b5eeff6b66
        .quad   0x0fcec10f01e02151

        // 2^8 * 8 * G

        .quad   0x644d58a649fe1e44
        .quad   0x21fcaea231ad777e
        .quad   0x02441c5a887fd0d2
        .quad   0x4901aa7183c511f3
        .quad   0x3df2d29dc4244e45
        .quad   0x2b020e7493d8de0a
        .quad   0x6cc8067e820c214d
        .quad   0x413779166feab90a
        .quad   0x08b1b7548c1af8f0
        .quad   0xce0f7a7c246299b4
        .quad   0xf760b0f91e06d939
        .quad   0x41bb887b726d1213

        // 2^12 * 1 * G

        .quad   0x9267806c567c49d8
        .quad   0x066d04ccca791e6a
        .quad   0xa69f5645e3cc394b
        .quad   0x5c95b686a0788cd2
        .quad   0x97d980e0aa39f7d2
        .quad   0x35d0384252c6b51c
        .quad   0x7d43f49307cd55aa
        .quad   0x56bd36cfb78ac362
        .quad   0x2ac519c10d14a954
        .quad   0xeaf474b494b5fa90
        .quad   0xe6af8382a9f87a5a
        .quad   0x0dea6db1879be094

        // 2^12 * 2 * G

        .quad   0xaa66bf547344e5ab
        .quad   0xda1258888f1b4309
        .quad   0x5e87d2b3fd564b2f
        .quad   0x5b2c78885483b1dd
        .quad   0x15baeb74d6a8797a
        .quad   0x7ef55cf1fac41732
        .quad   0x29001f5a3c8b05c5
        .quad   0x0ad7cc8752eaccfb
        .quad   0x52151362793408cf
        .quad   0xeb0f170319963d94
        .quad   0xa833b2fa883d9466
        .quad   0x093a7fa775003c78

        // 2^12 * 3 * G

        .quad   0xe5107de63a16d7be
        .quad   0xa377ffdc9af332cf
        .quad   0x70d5bf18440b677f
        .quad   0x6a252b19a4a31403
        .quad   0xb8e9604460a91286
        .quad   0x7f3fd8047778d3de
        .quad   0x67d01e31bf8a5e2d
        .quad   0x7b038a06c27b653e
        .quad   0x9ed919d5d36990f3
        .quad   0x5213aebbdb4eb9f2
        .quad   0xc708ea054cb99135
        .quad   0x58ded57f72260e56

        // 2^12 * 4 * G

        .quad   0x78e79dade9413d77
        .quad   0xf257f9d59729e67d
        .quad   0x59db910ee37aa7e6
        .quad   0x6aa11b5bbb9e039c
        .quad   0xda6d53265b0fd48b
        .quad   0x8960823193bfa988
        .quad   0xd78ac93261d57e28
        .quad   0x79f2942d3a5c8143
        .quad   0x97da2f25b6c88de9
        .quad   0x251ba7eaacf20169
        .quad   0x09b44f87ef4eb4e4
        .quad   0x7d90ab1bbc6a7da5

        // 2^12 * 5 * G

        .quad   0x9acca683a7016bfe
        .quad   0x90505f4df2c50b6d
        .quad   0x6b610d5fcce435aa
        .quad   0x19a10d446198ff96
        .quad   0x1a07a3f496b3c397
        .quad   0x11ceaa188f4e2532
        .quad   0x7d9498d5a7751bf0
        .quad   0x19ed161f508dd8a0
        .quad   0x560a2cd687dce6ca
        .quad   0x7f3568c48664cf4d
        .quad   0x8741e95222803a38
        .quad   0x483bdab1595653fc

        // 2^12 * 6 * G

        .quad   0xfa780f148734fa49
        .quad   0x106f0b70360534e0
        .quad   0x2210776fe3e307bd
        .quad   0x3286c109dde6a0fe
        .quad   0xd6cf4d0ab4da80f6
        .quad   0x82483e45f8307fe0
        .quad   0x05005269ae6f9da4
        .quad   0x1c7052909cf7877a
        .quad   0x32ee7de2874e98d4
        .quad   0x14c362e9b97e0c60
        .quad   0x5781dcde6a60a38a
        .quad   0x217dd5eaaa7aa840

        // 2^12 * 7 * G

        .quad   0x9db7c4d0248e1eb0
        .quad   0xe07697e14d74bf52
        .quad   0x1e6a9b173c562354
        .quad   0x7fa7c21f795a4965
        .quad   0x8bdf1fb9be8c0ec8
        .quad   0x00bae7f8e30a0282
        .quad   0x4963991dad6c4f6c
        .quad   0x07058a6e5df6f60a
        .quad   0xe9eb02c4db31f67f
        .quad   0xed25fd8910bcfb2b
        .quad   0x46c8131f5c5cddb4
        .quad   0x33b21c13a0cb9bce

        // 2^12 * 8 * G

        .quad   0x360692f8087d8e31
        .quad   0xf4dcc637d27163f7
        .quad   0x25a4e62065ea5963
        .quad   0x659bf72e5ac160d9
        .quad   0x9aafb9b05ee38c5b
        .quad   0xbf9d2d4e071a13c7
        .quad   0x8eee6e6de933290a
        .quad   0x1c3bab17ae109717
        .quad   0x1c9ab216c7cab7b0
        .quad   0x7d65d37407bbc3cc
        .quad   0x52744750504a58d5
        .quad   0x09f2606b131a2990

        // 2^16 * 1 * G

        .quad   0x40e87d44744346be
        .quad   0x1d48dad415b52b25
        .quad   0x7c3a8a18a13b603e
        .quad   0x4eb728c12fcdbdf7
        .quad   0x7e234c597c6691ae
        .quad   0x64889d3d0a85b4c8
        .quad   0xdae2c90c354afae7
        .quad   0x0a871e070c6a9e1d
        .quad   0x3301b5994bbc8989
        .quad   0x736bae3a5bdd4260
        .quad   0x0d61ade219d59e3c
        .quad   0x3ee7300f2685d464

        // 2^16 * 2 * G

        .quad   0xf5d255e49e7dd6b7
        .quad   0x8016115c610b1eac
        .quad   0x3c99975d92e187ca
        .quad   0x13815762979125c2
        .quad   0x43fa7947841e7518
        .quad   0xe5c6fa59639c46d7
        .quad   0xa1065e1de3052b74
        .quad   0x7d47c6a2cfb89030
        .quad   0x3fdad0148ef0d6e0
        .quad   0x9d3e749a91546f3c
        .quad   0x71ec621026bb8157
        .quad   0x148cf58d34c9ec80

        // 2^16 * 3 * G

        .quad   0x46a492f67934f027
        .quad   0x469984bef6840aa9
        .quad   0x5ca1bc2a89611854
        .quad   0x3ff2fa1ebd5dbbd4
        .quad   0xe2572f7d9ae4756d
        .quad   0x56c345bb88f3487f
        .quad   0x9fd10b6d6960a88d
        .quad   0x278febad4eaea1b9
        .quad   0xb1aa681f8c933966
        .quad   0x8c21949c20290c98
        .quad   0x39115291219d3c52
        .quad   0x4104dd02fe9c677b

        // 2^16 * 4 * G

        .quad   0x72b2bf5e1124422a
        .quad   0xa1fa0c3398a33ab5
        .quad   0x94cb6101fa52b666
        .quad   0x2c863b00afaf53d5
        .quad   0x81214e06db096ab8
        .quad   0x21a8b6c90ce44f35
        .quad   0x6524c12a409e2af5
        .quad   0x0165b5a48efca481
        .quad   0xf190a474a0846a76
        .quad   0x12eff984cd2f7cc0
        .quad   0x695e290658aa2b8f
        .quad   0x591b67d9bffec8b8

        // 2^16 * 5 * G

        .quad   0x312f0d1c80b49bfa
        .quad   0x5979515eabf3ec8a
        .quad   0x727033c09ef01c88
        .quad   0x3de02ec7ca8f7bcb
        .quad   0x99b9b3719f18b55d
        .quad   0xe465e5faa18c641e
        .quad   0x61081136c29f05ed
        .quad   0x489b4f867030128b
        .quad   0xd232102d3aeb92ef
        .quad   0xe16253b46116a861
        .quad   0x3d7eabe7190baa24
        .quad   0x49f5fbba496cbebf

        // 2^16 * 6 * G

        .quad   0x30949a108a5bcfd4
        .quad   0xdc40dd70bc6473eb
        .quad   0x92c294c1307c0d1c
        .quad   0x5604a86dcbfa6e74
        .quad   0x155d628c1e9c572e
        .quad   0x8a4d86acc5884741
        .quad   0x91a352f6515763eb
        .quad   0x06a1a6c28867515b
        .quad   0x7288d1d47c1764b6
        .quad   0x72541140e0418b51
        .quad   0x9f031a6018acf6d1
        .quad   0x20989e89fe2742c6

        // 2^16 * 7 * G

        .quad   0x499777fd3a2dcc7f
        .quad   0x32857c2ca54fd892
        .quad   0xa279d864d207e3a0
        .quad   0x0403ed1d0ca67e29
        .quad   0x1674278b85eaec2e
        .quad   0x5621dc077acb2bdf
        .quad   0x640a4c1661cbf45a
        .quad   0x730b9950f70595d3
        .quad   0xc94b2d35874ec552
        .quad   0xc5e6c8cf98246f8d
        .quad   0xf7cb46fa16c035ce
        .quad   0x5bd7454308303dcc

        // 2^16 * 8 * G

        .quad   0x7f9ad19528b24cc2
        .quad   0x7f6b54656335c181
        .quad   0x66b8b66e4fc07236
        .quad   0x133a78007380ad83
        .quad   0x85c4932115e7792a
        .quad   0xc64c89a2bdcdddc9
        .quad   0x9d1e3da8ada3d762
        .quad   0x5bb7db123067f82c
        .quad   0x0961f467c6ca62be
        .quad   0x04ec21d6211952ee
        .quad   0x182360779bd54770
        .quad   0x740dca6d58f0e0d2

        // 2^20 * 1 * G

        .quad   0x50b70bf5d3f0af0b
        .quad   0x4feaf48ae32e71f7
        .quad   0x60e84ed3a55bbd34
        .quad   0x00ed489b3f50d1ed
        .quad   0x3906c72aed261ae5
        .quad   0x9ab68fd988e100f7
        .quad   0xf5e9059af3360197
        .quad   0x0e53dc78bf2b6d47
        .quad   0xb90829bf7971877a
        .quad   0x5e4444636d17e631
        .quad   0x4d05c52e18276893
        .quad   0x27632d9a5a4a4af5

        // 2^20 * 2 * G

        .quad   0xd11ff05154b260ce
        .quad   0xd86dc38e72f95270
        .quad   0x601fcd0d267cc138
        .quad   0x2b67916429e90ccd
        .quad   0xa98285d187eaffdb
        .quad   0xa5b4fbbbd8d0a864
        .quad   0xb658f27f022663f7
        .quad   0x3bbc2b22d99ce282
        .quad   0xb917c952583c0a58
        .quad   0x653ff9b80fe4c6f3
        .quad   0x9b0da7d7bcdf3c0c
        .quad   0x43a0eeb6ab54d60e

        // 2^20 * 3 * G

        .quad   0x396966a46d4a5487
        .quad   0xf811a18aac2bb3ba
        .quad   0x66e4685b5628b26b
        .quad   0x70a477029d929b92
        .quad   0x3ac6322357875fe8
        .quad   0xd9d4f4ecf5fbcb8f
        .quad   0x8dee8493382bb620
        .quad   0x50c5eaa14c799fdc
        .quad   0xdd0edc8bd6f2fb3c
        .quad   0x54c63aa79cc7b7a0
        .quad   0xae0b032b2c8d9f1a
        .quad   0x6f9ce107602967fb

        // 2^20 * 4 * G

        .quad   0xad1054b1cde1c22a
        .quad   0xc4a8e90248eb32df
        .quad   0x5f3e7b33accdc0ea
        .quad   0x72364713fc79963e
        .quad   0x139693063520e0b5
        .quad   0x437fcf7c88ea03fe
        .quad   0xf7d4c40bd3c959bc
        .quad   0x699154d1f893ded9
        .quad   0x315d5c75b4b27526
        .quad   0xcccb842d0236daa5
        .quad   0x22f0c8a3345fee8e
        .quad   0x73975a617d39dbed

        // 2^20 * 5 * G

        .quad   0xe4024df96375da10
        .quad   0x78d3251a1830c870
        .quad   0x902b1948658cd91c
        .quad   0x7e18b10b29b7438a
        .quad   0x6f37f392f4433e46
        .quad   0x0e19b9a11f566b18
        .quad   0x220fb78a1fd1d662
        .quad   0x362a4258a381c94d
        .quad   0x9071d9132b6beb2f
        .quad   0x0f26e9ad28418247
        .quad   0xeab91ec9bdec925d
        .quad   0x4be65bc8f48af2de

        // 2^20 * 6 * G

        .quad   0x78487feba36e7028
        .quad   0x5f3f13001dd8ce34
        .quad   0x934fb12d4b30c489
        .quad   0x056c244d397f0a2b
        .quad   0x1d50fba257c26234
        .quad   0x7bd4823adeb0678b
        .quad   0xc2b0dc6ea6538af5
        .quad   0x5665eec6351da73e
        .quad   0xdb3ee00943bfb210
        .quad   0x4972018720800ac2
        .quad   0x26ab5d6173bd8667
        .quad   0x20b209c2ab204938

        // 2^20 * 7 * G

        .quad   0x549e342ac07fb34b
        .quad   0x02d8220821373d93
        .quad   0xbc262d70acd1f567
        .quad   0x7a92c9fdfbcac784
        .quad   0x1fcca94516bd3289
        .quad   0x448d65aa41420428
        .quad   0x59c3b7b216a55d62
        .quad   0x49992cc64e612cd8
        .quad   0x65bd1bea70f801de
        .quad   0x1befb7c0fe49e28a
        .quad   0xa86306cdb1b2ae4a
        .quad   0x3b7ac0cd265c2a09

        // 2^20 * 8 * G

        .quad   0x822bee438c01bcec
        .quad   0x530cb525c0fbc73b
        .quad   0x48519034c1953fe9
        .quad   0x265cc261e09a0f5b
        .quad   0xf0d54e4f22ed39a7
        .quad   0xa2aae91e5608150a
        .quad   0xf421b2e9eddae875
        .quad   0x31bc531d6b7de992
        .quad   0xdf3d134da980f971
        .quad   0x7a4fb8d1221a22a7
        .quad   0x3df7d42035aad6d8
        .quad   0x2a14edcc6a1a125e

        // 2^24 * 1 * G

        .quad   0xdf48ee0752cfce4e
        .quad   0xc3fffaf306ec08b7
        .quad   0x05710b2ab95459c4
        .quad   0x161d25fa963ea38d
        .quad   0x231a8c570478433c
        .quad   0xb7b5270ec281439d
        .quad   0xdbaa99eae3d9079f
        .quad   0x2c03f5256c2b03d9
        .quad   0x790f18757b53a47d
        .quad   0x307b0130cf0c5879
        .quad   0x31903d77257ef7f9
        .quad   0x699468bdbd96bbaf

        // 2^24 * 2 * G

        .quad   0xbd1f2f46f4dafecf
        .quad   0x7cef0114a47fd6f7
        .quad   0xd31ffdda4a47b37f
        .quad   0x525219a473905785
        .quad   0xd8dd3de66aa91948
        .quad   0x485064c22fc0d2cc
        .quad   0x9b48246634fdea2f
        .quad   0x293e1c4e6c4a2e3a
        .quad   0x376e134b925112e1
        .quad   0x703778b5dca15da0
        .quad   0xb04589af461c3111
        .quad   0x5b605c447f032823

        // 2^24 * 3 * G

        .quad   0xb965805920c47c89
        .quad   0xe7f0100c923b8fcc
        .quad   0x0001256502e2ef77
        .quad   0x24a76dcea8aeb3ee
        .quad   0x3be9fec6f0e7f04c
        .quad   0x866a579e75e34962
        .quad   0x5542ef161e1de61a
        .quad   0x2f12fef4cc5abdd5
        .quad   0x0a4522b2dfc0c740
        .quad   0x10d06e7f40c9a407
        .quad   0xc6cf144178cff668
        .quad   0x5e607b2518a43790

        // 2^24 * 4 * G

        .quad   0x58b31d8f6cdf1818
        .quad   0x35cfa74fc36258a2
        .quad   0xe1b3ff4f66e61d6e
        .quad   0x5067acab6ccdd5f7
        .quad   0xa02c431ca596cf14
        .quad   0xe3c42d40aed3e400
        .quad   0xd24526802e0f26db
        .quad   0x201f33139e457068
        .quad   0xfd527f6b08039d51
        .quad   0x18b14964017c0006
        .quad   0xd5220eb02e25a4a8
        .quad   0x397cba8862460375

        // 2^24 * 5 * G

        .quad   0x30c13093f05959b2
        .quad   0xe23aa18de9a97976
        .quad   0x222fd491721d5e26
        .quad   0x2339d320766e6c3a
        .quad   0x7815c3fbc81379e7
        .quad   0xa6619420dde12af1
        .quad   0xffa9c0f885a8fdd5
        .quad   0x771b4022c1e1c252
        .quad   0xd87dd986513a2fa7
        .quad   0xf5ac9b71f9d4cf08
        .quad   0xd06bc31b1ea283b3
        .quad   0x331a189219971a76

        // 2^24 * 6 * G

        .quad   0xf5166f45fb4f80c6
        .quad   0x9c36c7de61c775cf
        .quad   0xe3d4e81b9041d91c
        .quad   0x31167c6b83bdfe21
        .quad   0x26512f3a9d7572af
        .quad   0x5bcbe28868074a9e
        .quad   0x84edc1c11180f7c4
        .quad   0x1ac9619ff649a67b
        .quad   0xf22b3842524b1068
        .quad   0x5068343bee9ce987
        .quad   0xfc9d71844a6250c8
        .quad   0x612436341f08b111

        // 2^24 * 7 * G

        .quad   0xd99d41db874e898d
        .quad   0x09fea5f16c07dc20
        .quad   0x793d2c67d00f9bbc
        .quad   0x46ebe2309e5eff40
        .quad   0x8b6349e31a2d2638
        .quad   0x9ddfb7009bd3fd35
        .quad   0x7f8bf1b8a3a06ba4
        .quad   0x1522aa3178d90445
        .quad   0x2c382f5369614938
        .quad   0xdafe409ab72d6d10
        .quad   0xe8c83391b646f227
        .quad   0x45fe70f50524306c

        // 2^24 * 8 * G

        .quad   0xda4875a6960c0b8c
        .quad   0x5b68d076ef0e2f20
        .quad   0x07fb51cf3d0b8fd4
        .quad   0x428d1623a0e392d4
        .quad   0x62f24920c8951491
        .quad   0x05f007c83f630ca2
        .quad   0x6fbb45d2f5c9d4b8
        .quad   0x16619f6db57a2245
        .quad   0x084f4a4401a308fd
        .quad   0xa82219c376a5caac
        .quad   0xdeb8de4643d1bc7d
        .quad   0x1d81592d60bd38c6

        // 2^28 * 1 * G

        .quad   0xd833d7beec2a4c38
        .quad   0x2c9162830acc20ed
        .quad   0xe93a47aa92df7581
        .quad   0x702d67a3333c4a81
        .quad   0x3a4a369a2f89c8a1
        .quad   0x63137a1d7c8de80d
        .quad   0xbcac008a78eda015
        .quad   0x2cb8b3a5b483b03f
        .quad   0x36e417cbcb1b90a1
        .quad   0x33b3ddaa7f11794e
        .quad   0x3f510808885bc607
        .quad   0x24141dc0e6a8020d

        // 2^28 * 2 * G

        .quad   0x59f73c773fefee9d
        .quad   0xb3f1ef89c1cf989d
        .quad   0xe35dfb42e02e545f
        .quad   0x5766120b47a1b47c
        .quad   0x91925dccbd83157d
        .quad   0x3ca1205322cc8094
        .quad   0x28e57f183f90d6e4
        .quad   0x1a4714cede2e767b
        .quad   0xdb20ba0fb8b6b7ff
        .quad   0xb732c3b677511fa1
        .quad   0xa92b51c099f02d89
        .quad   0x4f3875ad489ca5f1

        // 2^28 * 3 * G

        .quad   0xc7fc762f4932ab22
        .quad   0x7ac0edf72f4c3c1b
        .quad   0x5f6b55aa9aa895e8
        .quad   0x3680274dad0a0081
        .quad   0x79ed13f6ee73eec0
        .quad   0xa5c6526d69110bb1
        .quad   0xe48928c38603860c
        .quad   0x722a1446fd7059f5
        .quad   0xd0959fe9a8cf8819
        .quad   0xd0a995508475a99c
        .quad   0x6eac173320b09cc5
        .quad   0x628ecf04331b1095

        // 2^28 * 4 * G

        .quad   0x98bcb118a9d0ddbc
        .quad   0xee449e3408b4802b
        .quad   0x87089226b8a6b104
        .quad   0x685f349a45c7915d
        .quad   0x9b41acf85c74ccf1
        .quad   0xb673318108265251
        .quad   0x99c92aed11adb147
        .quad   0x7a47d70d34ecb40f
        .quad   0x60a0c4cbcc43a4f5
        .quad   0x775c66ca3677bea9
        .quad   0xa17aa1752ff8f5ed
        .quad   0x11ded9020e01fdc0

        // 2^28 * 5 * G

        .quad   0x890e7809caefe704
        .quad   0x8728296de30e8c6c
        .quad   0x4c5cd2a392aeb1c9
        .quad   0x194263d15771531f
        .quad   0x471f95b03bea93b7
        .quad   0x0552d7d43313abd3
        .quad   0xbd9370e2e17e3f7b
        .quad   0x7b120f1db20e5bec
        .quad   0x17d2fb3d86502d7a
        .quad   0xb564d84450a69352
        .quad   0x7da962c8a60ed75d
        .quad   0x00d0f85b318736aa

        // 2^28 * 6 * G

        .quad   0x978b142e777c84fd
        .quad   0xf402644705a8c062
        .quad   0xa67ad51be7e612c7
        .quad   0x2f7b459698dd6a33
        .quad   0xa6753c1efd7621c1
        .quad   0x69c0b4a7445671f5
        .quad   0x971f527405b23c11
        .quad   0x387bc74851a8c7cd
        .quad   0x81894b4d4a52a9a8
        .quad   0xadd93e12f6b8832f
        .quad   0x184d8548b61bd638
        .quad   0x3f1c62dbd6c9f6cd

        // 2^28 * 7 * G

        .quad   0x2e8f1f0091910c1f
        .quad   0xa4df4fe0bff2e12c
        .quad   0x60c6560aee927438
        .quad   0x6338283facefc8fa
        .quad   0x3fad3e40148f693d
        .quad   0x052656e194eb9a72
        .quad   0x2f4dcbfd184f4e2f
        .quad   0x406f8db1c482e18b
        .quad   0x9e630d2c7f191ee4
        .quad   0x4fbf8301bc3ff670
        .quad   0x787d8e4e7afb73c4
        .quad   0x50d83d5be8f58fa5

        // 2^28 * 8 * G

        .quad   0x85683916c11a1897
        .quad   0x2d69a4efe506d008
        .quad   0x39af1378f664bd01
        .quad   0x65942131361517c6
        .quad   0xc0accf90b4d3b66d
        .quad   0xa7059de561732e60
        .quad   0x033d1f7870c6b0ba
        .quad   0x584161cd26d946e4
        .quad   0xbbf2b1a072d27ca2
        .quad   0xbf393c59fbdec704
        .quad   0xe98dbbcee262b81e
        .quad   0x02eebd0b3029b589

        // 2^32 * 1 * G

        .quad   0x61368756a60dac5f
        .quad   0x17e02f6aebabdc57
        .quad   0x7f193f2d4cce0f7d
        .quad   0x20234a7789ecdcf0
        .quad   0x8765b69f7b85c5e8
        .quad   0x6ff0678bd168bab2
        .quad   0x3a70e77c1d330f9b
        .quad   0x3a5f6d51b0af8e7c
        .quad   0x76d20db67178b252
        .quad   0x071c34f9d51ed160
        .quad   0xf62a4a20b3e41170
        .quad   0x7cd682353cffe366

        // 2^32 * 2 * G

        .quad   0x0be1a45bd887fab6
        .quad   0x2a846a32ba403b6e
        .quad   0xd9921012e96e6000
        .quad   0x2838c8863bdc0943
        .quad   0xa665cd6068acf4f3
        .quad   0x42d92d183cd7e3d3
        .quad   0x5759389d336025d9
        .quad   0x3ef0253b2b2cd8ff
        .quad   0xd16bb0cf4a465030
        .quad   0xfa496b4115c577ab
        .quad   0x82cfae8af4ab419d
        .quad   0x21dcb8a606a82812

        // 2^32 * 3 * G

        .quad   0x5c6004468c9d9fc8
        .quad   0x2540096ed42aa3cb
        .quad   0x125b4d4c12ee2f9c
        .quad   0x0bc3d08194a31dab
        .quad   0x9a8d00fabe7731ba
        .quad   0x8203607e629e1889
        .quad   0xb2cc023743f3d97f
        .quad   0x5d840dbf6c6f678b
        .quad   0x706e380d309fe18b
        .quad   0x6eb02da6b9e165c7
        .quad   0x57bbba997dae20ab
        .quad   0x3a4276232ac196dd

        // 2^32 * 4 * G

        .quad   0x4b42432c8a7084fa
        .quad   0x898a19e3dfb9e545
        .quad   0xbe9f00219c58e45d
        .quad   0x1ff177cea16debd1
        .quad   0x3bf8c172db447ecb
        .quad   0x5fcfc41fc6282dbd
        .quad   0x80acffc075aa15fe
        .quad   0x0770c9e824e1a9f9
        .quad   0xcf61d99a45b5b5fd
        .quad   0x860984e91b3a7924
        .quad   0xe7300919303e3e89
        .quad   0x39f264fd41500b1e

        // 2^32 * 5 * G

        .quad   0xa7ad3417dbe7e29c
        .quad   0xbd94376a2b9c139c
        .quad   0xa0e91b8e93597ba9
        .quad   0x1712d73468889840
        .quad   0xd19b4aabfe097be1
        .quad   0xa46dfce1dfe01929
        .quad   0xc3c908942ca6f1ff
        .quad   0x65c621272c35f14e
        .quad   0xe72b89f8ce3193dd
        .quad   0x4d103356a125c0bb
        .quad   0x0419a93d2e1cfe83
        .quad   0x22f9800ab19ce272

        // 2^32 * 6 * G

        .quad   0x605a368a3e9ef8cb
        .quad   0xe3e9c022a5504715
        .quad   0x553d48b05f24248f
        .quad   0x13f416cd647626e5
        .quad   0x42029fdd9a6efdac
        .quad   0xb912cebe34a54941
        .quad   0x640f64b987bdf37b
        .quad   0x4171a4d38598cab4
        .quad   0xfa2758aa99c94c8c
        .quad   0x23006f6fb000b807
        .quad   0xfbd291ddadda5392
        .quad   0x508214fa574bd1ab

        // 2^32 * 7 * G

        .quad   0xc20269153ed6fe4b
        .quad   0xa65a6739511d77c4
        .quad   0xcbde26462c14af94
        .quad   0x22f960ec6faba74b
        .quad   0x461a15bb53d003d6
        .quad   0xb2102888bcf3c965
        .quad   0x27c576756c683a5a
        .quad   0x3a7758a4c86cb447
        .quad   0x548111f693ae5076
        .quad   0x1dae21df1dfd54a6
        .quad   0x12248c90f3115e65
        .quad   0x5d9fd15f8de7f494

        // 2^32 * 8 * G

        .quad   0x031408d36d63727f
        .quad   0x6a379aefd7c7b533
        .quad   0xa9e18fc5ccaee24b
        .quad   0x332f35914f8fbed3
        .quad   0x3f244d2aeed7521e
        .quad   0x8e3a9028432e9615
        .quad   0xe164ba772e9c16d4
        .quad   0x3bc187fa47eb98d8
        .quad   0x6d470115ea86c20c
        .quad   0x998ab7cb6c46d125
        .quad   0xd77832b53a660188
        .quad   0x450d81ce906fba03

        // 2^36 * 1 * G

        .quad   0xf8ae4d2ad8453902
        .quad   0x7018058ee8db2d1d
        .quad   0xaab3995fc7d2c11e
        .quad   0x53b16d2324ccca79
        .quad   0x23264d66b2cae0b5
        .quad   0x7dbaed33ebca6576
        .quad   0x030ebed6f0d24ac8
        .quad   0x2a887f78f7635510
        .quad   0x2a23b9e75c012d4f
        .quad   0x0c974651cae1f2ea
        .quad   0x2fb63273675d70ca
        .quad   0x0ba7250b864403f5

        // 2^36 * 2 * G

        .quad   0xbb0d18fd029c6421
        .quad   0xbc2d142189298f02
        .quad   0x8347f8e68b250e96
        .quad   0x7b9f2fe8032d71c9
        .quad   0xdd63589386f86d9c
        .quad   0x61699176e13a85a4
        .quad   0x2e5111954eaa7d57
        .quad   0x32c21b57fb60bdfb
        .quad   0xd87823cd319e0780
        .quad   0xefc4cfc1897775c5
        .quad   0x4854fb129a0ab3f7
        .quad   0x12c49d417238c371

        // 2^36 * 3 * G

        .quad   0x0950b533ffe83769
        .quad   0x21861c1d8e1d6bd1
        .quad   0xf022d8381302e510
        .quad   0x2509200c6391cab4
        .quad   0x09b3a01783799542
        .quad   0x626dd08faad5ee3f
        .quad   0xba00bceeeb70149f
        .quad   0x1421b246a0a444c9
        .quad   0x4aa43a8e8c24a7c7
        .quad   0x04c1f540d8f05ef5
        .quad   0xadba5e0c0b3eb9dc
        .quad   0x2ab5504448a49ce3

        // 2^36 * 4 * G

        .quad   0x2ed227266f0f5dec
        .quad   0x9824ee415ed50824
        .quad   0x807bec7c9468d415
        .quad   0x7093bae1b521e23f
        .quad   0xdc07ac631c5d3afa
        .quad   0x58615171f9df8c6c
        .quad   0x72a079d89d73e2b0
        .quad   0x7301f4ceb4eae15d
        .quad   0x6409e759d6722c41
        .quad   0xa674e1cf72bf729b
        .quad   0xbc0a24eb3c21e569
        .quad   0x390167d24ebacb23

        // 2^36 * 5 * G

        .quad   0x27f58e3bba353f1c
        .quad   0x4c47764dbf6a4361
        .quad   0xafbbc4e56e562650
        .quad   0x07db2ee6aae1a45d
        .quad   0xd7bb054ba2f2120b
        .quad   0xe2b9ceaeb10589b7
        .quad   0x3fe8bac8f3c0edbe
        .quad   0x4cbd40767112cb69
        .quad   0x0b603cc029c58176
        .quad   0x5988e3825cb15d61
        .quad   0x2bb61413dcf0ad8d
        .quad   0x7b8eec6c74183287

        // 2^36 * 6 * G

        .quad   0xe4ca40782cd27cb0
        .quad   0xdaf9c323fbe967bd
        .quad   0xb29bd34a8ad41e9e
        .quad   0x72810497626ede4d
        .quad   0x32fee570fc386b73
        .quad   0xda8b0141da3a8cc7
        .quad   0x975ffd0ac8968359
        .quad   0x6ee809a1b132a855
        .quad   0x9444bb31fcfd863a
        .quad   0x2fe3690a3e4e48c5
        .quad   0xdc29c867d088fa25
        .quad   0x13bd1e38d173292e

        // 2^36 * 7 * G

        .quad   0xd32b4cd8696149b5
        .quad   0xe55937d781d8aab7
        .quad   0x0bcb2127ae122b94
        .quad   0x41e86fcfb14099b0
        .quad   0x223fb5cf1dfac521
        .quad   0x325c25316f554450
        .quad   0x030b98d7659177ac
        .quad   0x1ed018b64f88a4bd
        .quad   0x3630dfa1b802a6b0
        .quad   0x880f874742ad3bd5
        .quad   0x0af90d6ceec5a4d4
        .quad   0x746a247a37cdc5d9

        // 2^36 * 8 * G

        .quad   0xd531b8bd2b7b9af6
        .quad   0x5005093537fc5b51
        .quad   0x232fcf25c593546d
        .quad   0x20a365142bb40f49
        .quad   0x6eccd85278d941ed
        .quad   0x2254ae83d22f7843
        .quad   0xc522d02e7bbfcdb7
        .quad   0x681e3351bff0e4e2
        .quad   0x8b64b59d83034f45
        .quad   0x2f8b71f21fa20efb
        .quad   0x69249495ba6550e4
        .quad   0x539ef98e45d5472b

        // 2^40 * 1 * G

        .quad   0x6e7bb6a1a6205275
        .quad   0xaa4f21d7413c8e83
        .quad   0x6f56d155e88f5cb2
        .quad   0x2de25d4ba6345be1
        .quad   0xd074d8961cae743f
        .quad   0xf86d18f5ee1c63ed
        .quad   0x97bdc55be7f4ed29
        .quad   0x4cbad279663ab108
        .quad   0x80d19024a0d71fcd
        .quad   0xc525c20afb288af8
        .quad   0xb1a3974b5f3a6419
        .quad   0x7d7fbcefe2007233

        // 2^40 * 2 * G

        .quad   0xfaef1e6a266b2801
        .quad   0x866c68c4d5739f16
        .quad   0xf68a2fbc1b03762c
        .quad   0x5975435e87b75a8d
        .quad   0xcd7c5dc5f3c29094
        .quad   0xc781a29a2a9105ab
        .quad   0x80c61d36421c3058
        .quad   0x4f9cd196dcd8d4d7
        .quad   0x199297d86a7b3768
        .quad   0xd0d058241ad17a63
        .quad   0xba029cad5c1c0c17
        .quad   0x7ccdd084387a0307

        // 2^40 * 3 * G

        .quad   0xdca6422c6d260417
        .quad   0xae153d50948240bd
        .quad   0xa9c0c1b4fb68c677
        .quad   0x428bd0ed61d0cf53
        .quad   0x9b0c84186760cc93
        .quad   0xcdae007a1ab32a99
        .quad   0xa88dec86620bda18
        .quad   0x3593ca848190ca44
        .quad   0x9213189a5e849aa7
        .quad   0xd4d8c33565d8facd
        .quad   0x8c52545b53fdbbd1
        .quad   0x27398308da2d63e6

        // 2^40 * 4 * G

        .quad   0x42c38d28435ed413
        .quad   0xbd50f3603278ccc9
        .quad   0xbb07ab1a79da03ef
        .quad   0x269597aebe8c3355
        .quad   0xb9a10e4c0a702453
        .quad   0x0fa25866d57d1bde
        .quad   0xffb9d9b5cd27daf7
        .quad   0x572c2945492c33fd
        .quad   0xc77fc745d6cd30be
        .quad   0xe4dfe8d3e3baaefb
        .quad   0xa22c8830aa5dda0c
        .quad   0x7f985498c05bca80

        // 2^40 * 5 * G

        .quad   0x3849ce889f0be117
        .quad   0x8005ad1b7b54a288
        .quad   0x3da3c39f23fc921c
        .quad   0x76c2ec470a31f304
        .quad   0xd35615520fbf6363
        .quad   0x08045a45cf4dfba6
        .quad   0xeec24fbc873fa0c2
        .quad   0x30f2653cd69b12e7
        .quad   0x8a08c938aac10c85
        .quad   0x46179b60db276bcb
        .quad   0xa920c01e0e6fac70
        .quad   0x2f1273f1596473da

        // 2^40 * 6 * G

        .quad   0x4739fc7c8ae01e11
        .quad   0xfd5274904a6aab9f
        .quad   0x41d98a8287728f2e
        .quad   0x5d9e572ad85b69f2
        .quad   0x30488bd755a70bc0
        .quad   0x06d6b5a4f1d442e7
        .quad   0xead1a69ebc596162
        .quad   0x38ac1997edc5f784
        .quad   0x0666b517a751b13b
        .quad   0x747d06867e9b858c
        .quad   0xacacc011454dde49
        .quad   0x22dfcd9cbfe9e69c

        // 2^40 * 7 * G

        .quad   0x8ddbd2e0c30d0cd9
        .quad   0xad8e665facbb4333
        .quad   0x8f6b258c322a961f
        .quad   0x6b2916c05448c1c7
        .quad   0x56ec59b4103be0a1
        .quad   0x2ee3baecd259f969
        .quad   0x797cb29413f5cd32
        .quad   0x0fe9877824cde472
        .quad   0x7edb34d10aba913b
        .quad   0x4ea3cd822e6dac0e
        .quad   0x66083dff6578f815
        .quad   0x4c303f307ff00a17

        // 2^40 * 8 * G

        .quad   0xd30a3bd617b28c85
        .quad   0xc5d377b739773bea
        .quad   0xc6c6e78c1e6a5cbf
        .quad   0x0d61b8f78b2ab7c4
        .quad   0x29fc03580dd94500
        .quad   0xecd27aa46fbbec93
        .quad   0x130a155fc2e2a7f8
        .quad   0x416b151ab706a1d5
        .quad   0x56a8d7efe9c136b0
        .quad   0xbd07e5cd58e44b20
        .quad   0xafe62fda1b57e0ab
        .quad   0x191a2af74277e8d2

        // 2^44 * 1 * G

        .quad   0xd550095bab6f4985
        .quad   0x04f4cd5b4fbfaf1a
        .quad   0x9d8e2ed12a0c7540
        .quad   0x2bc24e04b2212286
        .quad   0x09d4b60b2fe09a14
        .quad   0xc384f0afdbb1747e
        .quad   0x58e2ea8978b5fd6e
        .quad   0x519ef577b5e09b0a
        .quad   0x1863d7d91124cca9
        .quad   0x7ac08145b88a708e
        .quad   0x2bcd7309857031f5
        .quad   0x62337a6e8ab8fae5

        // 2^44 * 2 * G

        .quad   0x4bcef17f06ffca16
        .quad   0xde06e1db692ae16a
        .quad   0x0753702d614f42b0
        .quad   0x5f6041b45b9212d0
        .quad   0xd1ab324e1b3a1273
        .quad   0x18947cf181055340
        .quad   0x3b5d9567a98c196e
        .quad   0x7fa00425802e1e68
        .quad   0x7d531574028c2705
        .quad   0x80317d69db0d75fe
        .quad   0x30fface8ef8c8ddd
        .quad   0x7e9de97bb6c3e998

        // 2^44 * 3 * G

        .quad   0x1558967b9e6585a3
        .quad   0x97c99ce098e98b92
        .quad   0x10af149b6eb3adad
        .quad   0x42181fe8f4d38cfa
        .quad   0xf004be62a24d40dd
        .quad   0xba0659910452d41f
        .quad   0x81c45ee162a44234
        .quad   0x4cb829d8a22266ef
        .quad   0x1dbcaa8407b86681
        .quad   0x081f001e8b26753b
        .quad   0x3cd7ce6a84048e81
        .quad   0x78af11633f25f22c

        // 2^44 * 4 * G

        .quad   0x8416ebd40b50babc
        .quad   0x1508722628208bee
        .quad   0xa3148fafb9c1c36d
        .quad   0x0d07daacd32d7d5d
        .quad   0x3241c00e7d65318c
        .quad   0xe6bee5dcd0e86de7
        .quad   0x118b2dc2fbc08c26
        .quad   0x680d04a7fc603dc3
        .quad   0xf9c2414a695aa3eb
        .quad   0xdaa42c4c05a68f21
        .quad   0x7c6c23987f93963e
        .quad   0x210e8cd30c3954e3

        // 2^44 * 5 * G

        .quad   0xac4201f210a71c06
        .quad   0x6a65e0aef3bfb021
        .quad   0xbc42c35c393632f7
        .quad   0x56ea8db1865f0742
        .quad   0x2b50f16137fe6c26
        .quad   0xe102bcd856e404d8
        .quad   0x12b0f1414c561f6b
        .quad   0x51b17bc8d028ec91
        .quad   0xfff5fb4bcf535119
        .quad   0xf4989d79df1108a0
        .quad   0xbdfcea659a3ba325
        .quad   0x18a11f1174d1a6f2

        // 2^44 * 6 * G

        .quad   0x407375ab3f6bba29
        .quad   0x9ec3b6d8991e482e
        .quad   0x99c80e82e55f92e9
        .quad   0x307c13b6fb0c0ae1
        .quad   0xfbd63cdad27a5f2c
        .quad   0xf00fc4bc8aa106d7
        .quad   0x53fb5c1a8e64a430
        .quad   0x04eaabe50c1a2e85
        .quad   0x24751021cb8ab5e7
        .quad   0xfc2344495c5010eb
        .quad   0x5f1e717b4e5610a1
        .quad   0x44da5f18c2710cd5

        // 2^44 * 7 * G

        .quad   0x033cc55ff1b82eb5
        .quad   0xb15ae36d411cae52
        .quad   0xba40b6198ffbacd3
        .quad   0x768edce1532e861f
        .quad   0x9156fe6b89d8eacc
        .quad   0xe6b79451e23126a1
        .quad   0xbd7463d93944eb4e
        .quad   0x726373f6767203ae
        .quad   0xe305ca72eb7ef68a
        .quad   0x662cf31f70eadb23
        .quad   0x18f026fdb4c45b68
        .quad   0x513b5384b5d2ecbd

        // 2^44 * 8 * G

        .quad   0x46d46280c729989e
        .quad   0x4b93fbd05368a5dd
        .quad   0x63df3f81d1765a89
        .quad   0x34cebd64b9a0a223
        .quad   0x5e2702878af34ceb
        .quad   0x900b0409b946d6ae
        .quad   0x6512ebf7dabd8512
        .quad   0x61d9b76988258f81
        .quad   0xa6c5a71349b7d94b
        .quad   0xa3f3d15823eb9446
        .quad   0x0416fbd277484834
        .quad   0x69d45e6f2c70812f

        // 2^48 * 1 * G

        .quad   0xce16f74bc53c1431
        .quad   0x2b9725ce2072edde
        .quad   0xb8b9c36fb5b23ee7
        .quad   0x7e2e0e450b5cc908
        .quad   0x9fe62b434f460efb
        .quad   0xded303d4a63607d6
        .quad   0xf052210eb7a0da24
        .quad   0x237e7dbe00545b93
        .quad   0x013575ed6701b430
        .quad   0x231094e69f0bfd10
        .quad   0x75320f1583e47f22
        .quad   0x71afa699b11155e3

        // 2^48 * 2 * G

        .quad   0x65ce6f9b3953b61d
        .quad   0xc65839eaafa141e6
        .quad   0x0f435ffda9f759fe
        .quad   0x021142e9c2b1c28e
        .quad   0xea423c1c473b50d6
        .quad   0x51e87a1f3b38ef10
        .quad   0x9b84bf5fb2c9be95
        .quad   0x00731fbc78f89a1c
        .quad   0xe430c71848f81880
        .quad   0xbf960c225ecec119
        .quad   0xb6dae0836bba15e3
        .quad   0x4c4d6f3347e15808

        // 2^48 * 3 * G

        .quad   0x18f7eccfc17d1fc9
        .quad   0x6c75f5a651403c14
        .quad   0xdbde712bf7ee0cdf
        .quad   0x193fddaaa7e47a22
        .quad   0x2f0cddfc988f1970
        .quad   0x6b916227b0b9f51b
        .quad   0x6ec7b6c4779176be
        .quad   0x38bf9500a88f9fa8
        .quad   0x1fd2c93c37e8876f
        .quad   0xa2f61e5a18d1462c
        .quad   0x5080f58239241276
        .quad   0x6a6fb99ebf0d4969

        // 2^48 * 4 * G

        .quad   0x6a46c1bb560855eb
        .quad   0x2416bb38f893f09d
        .quad   0xd71d11378f71acc1
        .quad   0x75f76914a31896ea
        .quad   0xeeb122b5b6e423c6
        .quad   0x939d7010f286ff8e
        .quad   0x90a92a831dcf5d8c
        .quad   0x136fda9f42c5eb10
        .quad   0xf94cdfb1a305bdd1
        .quad   0x0f364b9d9ff82c08
        .quad   0x2a87d8a5c3bb588a
        .quad   0x022183510be8dcba

        // 2^48 * 5 * G

        .quad   0x4af766385ead2d14
        .quad   0xa08ed880ca7c5830
        .quad   0x0d13a6e610211e3d
        .quad   0x6a071ce17b806c03
        .quad   0x9d5a710143307a7f
        .quad   0xb063de9ec47da45f
        .quad   0x22bbfe52be927ad3
        .quad   0x1387c441fd40426c
        .quad   0xb5d3c3d187978af8
        .quad   0x722b5a3d7f0e4413
        .quad   0x0d7b4848bb477ca0
        .quad   0x3171b26aaf1edc92

        // 2^48 * 6 * G

        .quad   0xa92f319097564ca8
        .quad   0xff7bb84c2275e119
        .quad   0x4f55fe37a4875150
        .quad   0x221fd4873cf0835a
        .quad   0xa60db7d8b28a47d1
        .quad   0xa6bf14d61770a4f1
        .quad   0xd4a1f89353ddbd58
        .quad   0x6c514a63344243e9
        .quad   0x2322204f3a156341
        .quad   0xfb73e0e9ba0a032d
        .quad   0xfce0dd4c410f030e
        .quad   0x48daa596fb924aaa

        // 2^48 * 7 * G

        .quad   0x6eca8e665ca59cc7
        .quad   0xa847254b2e38aca0
        .quad   0x31afc708d21e17ce
        .quad   0x676dd6fccad84af7
        .quad   0x14f61d5dc84c9793
        .quad   0x9941f9e3ef418206
        .quad   0xcdf5b88f346277ac
        .quad   0x58c837fa0e8a79a9
        .quad   0x0cf9688596fc9058
        .quad   0x1ddcbbf37b56a01b
        .quad   0xdcc2e77d4935d66a
        .quad   0x1c4f73f2c6a57f0a

        // 2^48 * 8 * G

        .quad   0x0e7a4fbd305fa0bb
        .quad   0x829d4ce054c663ad
        .quad   0xf421c3832fe33848
        .quad   0x795ac80d1bf64c42
        .quad   0xb36e706efc7c3484
        .quad   0x73dfc9b4c3c1cf61
        .quad   0xeb1d79c9781cc7e5
        .quad   0x70459adb7daf675c
        .quad   0x1b91db4991b42bb3
        .quad   0x572696234b02dcca
        .quad   0x9fdf9ee51f8c78dc
        .quad   0x5fe162848ce21fd3

        // 2^52 * 1 * G

        .quad   0xe2790aae4d077c41
        .quad   0x8b938270db7469a3
        .quad   0x6eb632dc8abd16a2
        .quad   0x720814ecaa064b72
        .quad   0x315c29c795115389
        .quad   0xd7e0e507862f74ce
        .quad   0x0c4a762185927432
        .quad   0x72de6c984a25a1e4
        .quad   0xae9ab553bf6aa310
        .quad   0x050a50a9806d6e1b
        .quad   0x92bb7403adff5139
        .quad   0x0394d27645be618b

        // 2^52 * 2 * G

        .quad   0x4d572251857eedf4
        .quad   0xe3724edde19e93c5
        .quad   0x8a71420e0b797035
        .quad   0x3b3c833687abe743
        .quad   0xf5396425b23545a4
        .quad   0x15a7a27e98fbb296
        .quad   0xab6c52bc636fdd86
        .quad   0x79d995a8419334ee
        .quad   0xcd8a8ea61195dd75
        .quad   0xa504d8a81dd9a82f
        .quad   0x540dca81a35879b6
        .quad   0x60dd16a379c86a8a

        // 2^52 * 3 * G

        .quad   0x35a2c8487381e559
        .quad   0x596ffea6d78082cb
        .quad   0xcb9771ebdba7b653
        .quad   0x5a08b5019b4da685
        .quad   0x3501d6f8153e47b8
        .quad   0xb7a9675414a2f60c
        .quad   0x112ee8b6455d9523
        .quad   0x4e62a3c18112ea8a
        .quad   0xc8d4ac04516ab786
        .quad   0x595af3215295b23d
        .quad   0xd6edd234db0230c1
        .quad   0x0929efe8825b41cc

        // 2^52 * 4 * G

        .quad   0x5f0601d1cbd0f2d3
        .quad   0x736e412f6132bb7f
        .quad   0x83604432238dde87
        .quad   0x1e3a5272f5c0753c
        .quad   0x8b3172b7ad56651d
        .quad   0x01581b7a3fabd717
        .quad   0x2dc94df6424df6e4
        .quad   0x30376e5d2c29284f
        .quad   0xd2918da78159a59c
        .quad   0x6bdc1cd93f0713f3
        .quad   0x565f7a934acd6590
        .quad   0x53daacec4cb4c128

        // 2^52 * 5 * G

        .quad   0x4ca73bd79cc8a7d6
        .quad   0x4d4a738f47e9a9b2
        .quad   0xf4cbf12942f5fe00
        .quad   0x01a13ff9bdbf0752
        .quad   0x99852bc3852cfdb0
        .quad   0x2cc12e9559d6ed0b
        .quad   0x70f9e2bf9b5ac27b
        .quad   0x4f3b8c117959ae99
        .quad   0x55b6c9c82ff26412
        .quad   0x1ac4a8c91fb667a8
        .quad   0xd527bfcfeb778bf2
        .quad   0x303337da7012a3be

        // 2^52 * 6 * G

        .quad   0x955422228c1c9d7c
        .quad   0x01fac1371a9b340f
        .quad   0x7e8d9177925b48d7
        .quad   0x53f8ad5661b3e31b
        .quad   0x976d3ccbfad2fdd1
        .quad   0xcb88839737a640a8
        .quad   0x2ff00c1d6734cb25
        .quad   0x269ff4dc789c2d2b
        .quad   0x0c003fbdc08d678d
        .quad   0x4d982fa37ead2b17
        .quad   0xc07e6bcdb2e582f1
        .quad   0x296c7291df412a44

        // 2^52 * 7 * G

        .quad   0x7903de2b33daf397
        .quad   0xd0ff0619c9a624b3
        .quad   0x8a1d252b555b3e18
        .quad   0x2b6d581c52e0b7c0
        .quad   0xdfb23205dab8b59e
        .quad   0x465aeaa0c8092250
        .quad   0xd133c1189a725d18
        .quad   0x2327370261f117d1
        .quad   0x3d0543d3623e7986
        .quad   0x679414c2c278a354
        .quad   0xae43f0cc726196f6
        .quad   0x7836c41f8245eaba

        // 2^52 * 8 * G

        .quad   0xe7a254db49e95a81
        .quad   0x5192d5d008b0ad73
        .quad   0x4d20e5b1d00afc07
        .quad   0x5d55f8012cf25f38
        .quad   0xca651e848011937c
        .quad   0xc6b0c46e6ef41a28
        .quad   0xb7021ba75f3f8d52
        .quad   0x119dff99ead7b9fd
        .quad   0x43eadfcbf4b31d4d
        .quad   0xc6503f7411148892
        .quad   0xfeee68c5060d3b17
        .quad   0x329293b3dd4a0ac8

        // 2^56 * 1 * G

        .quad   0x4e59214fe194961a
        .quad   0x49be7dc70d71cd4f
        .quad   0x9300cfd23b50f22d
        .quad   0x4789d446fc917232
        .quad   0x2879852d5d7cb208
        .quad   0xb8dedd70687df2e7
        .quad   0xdc0bffab21687891
        .quad   0x2b44c043677daa35
        .quad   0x1a1c87ab074eb78e
        .quad   0xfac6d18e99daf467
        .quad   0x3eacbbcd484f9067
        .quad   0x60c52eef2bb9a4e4

        // 2^56 * 2 * G

        .quad   0x0b5d89bc3bfd8bf1
        .quad   0xb06b9237c9f3551a
        .quad   0x0e4c16b0d53028f5
        .quad   0x10bc9c312ccfcaab
        .quad   0x702bc5c27cae6d11
        .quad   0x44c7699b54a48cab
        .quad   0xefbc4056ba492eb2
        .quad   0x70d77248d9b6676d
        .quad   0xaa8ae84b3ec2a05b
        .quad   0x98699ef4ed1781e0
        .quad   0x794513e4708e85d1
        .quad   0x63755bd3a976f413

        // 2^56 * 3 * G

        .quad   0xb55fa03e2ad10853
        .quad   0x356f75909ee63569
        .quad   0x9ff9f1fdbe69b890
        .quad   0x0d8cc1c48bc16f84
        .quad   0x3dc7101897f1acb7
        .quad   0x5dda7d5ec165bbd8
        .quad   0x508e5b9c0fa1020f
        .quad   0x2763751737c52a56
        .quad   0x029402d36eb419a9
        .quad   0xf0b44e7e77b460a5
        .quad   0xcfa86230d43c4956
        .quad   0x70c2dd8a7ad166e7

        // 2^56 * 4 * G

        .quad   0x656194509f6fec0e
        .quad   0xee2e7ea946c6518d
        .quad   0x9733c1f367e09b5c
        .quad   0x2e0fac6363948495
        .quad   0x91d4967db8ed7e13
        .quad   0x74252f0ad776817a
        .quad   0xe40982e00d852564
        .quad   0x32b8613816a53ce5
        .quad   0x79e7f7bee448cd64
        .quad   0x6ac83a67087886d0
        .quad   0xf89fd4d9a0e4db2e
        .quad   0x4179215c735a4f41

        // 2^56 * 5 * G

        .quad   0x8c7094e7d7dced2a
        .quad   0x97fb8ac347d39c70
        .quad   0xe13be033a906d902
        .quad   0x700344a30cd99d76
        .quad   0xe4ae33b9286bcd34
        .quad   0xb7ef7eb6559dd6dc
        .quad   0x278b141fb3d38e1f
        .quad   0x31fa85662241c286
        .quad   0xaf826c422e3622f4
        .quad   0xc12029879833502d
        .quad   0x9bc1b7e12b389123
        .quad   0x24bb2312a9952489

        // 2^56 * 6 * G

        .quad   0xb1a8ed1732de67c3
        .quad   0x3cb49418461b4948
        .quad   0x8ebd434376cfbcd2
        .quad   0x0fee3e871e188008
        .quad   0x41f80c2af5f85c6b
        .quad   0x687284c304fa6794
        .quad   0x8945df99a3ba1bad
        .quad   0x0d1d2af9ffeb5d16
        .quad   0xa9da8aa132621edf
        .quad   0x30b822a159226579
        .quad   0x4004197ba79ac193
        .quad   0x16acd79718531d76

        // 2^56 * 7 * G

        .quad   0x72df72af2d9b1d3d
        .quad   0x63462a36a432245a
        .quad   0x3ecea07916b39637
        .quad   0x123e0ef6b9302309
        .quad   0xc959c6c57887b6ad
        .quad   0x94e19ead5f90feba
        .quad   0x16e24e62a342f504
        .quad   0x164ed34b18161700
        .quad   0x487ed94c192fe69a
        .quad   0x61ae2cea3a911513
        .quad   0x877bf6d3b9a4de27
        .quad   0x78da0fc61073f3eb

        // 2^56 * 8 * G

        .quad   0x5bf15d28e52bc66a
        .quad   0x2c47e31870f01a8e
        .quad   0x2419afbc06c28bdd
        .quad   0x2d25deeb256b173a
        .quad   0xa29f80f1680c3a94
        .quad   0x71f77e151ae9e7e6
        .quad   0x1100f15848017973
        .quad   0x054aa4b316b38ddd
        .quad   0xdfc8468d19267cb8
        .quad   0x0b28789c66e54daf
        .quad   0x2aeb1d2a666eec17
        .quad   0x134610a6ab7da760

        // 2^60 * 1 * G

        .quad   0xcaf55ec27c59b23f
        .quad   0x99aeed3e154d04f2
        .quad   0x68441d72e14141f4
        .quad   0x140345133932a0a2
        .quad   0xd91430e0dc028c3c
        .quad   0x0eb955a85217c771
        .quad   0x4b09e1ed2c99a1fa
        .quad   0x42881af2bd6a743c
        .quad   0x7bfec69aab5cad3d
        .quad   0xc23e8cd34cb2cfad
        .quad   0x685dd14bfb37d6a2
        .quad   0x0ad6d64415677a18

        // 2^60 * 2 * G

        .quad   0x781a439e417becb5
        .quad   0x4ac5938cd10e0266
        .quad   0x5da385110692ac24
        .quad   0x11b065a2ade31233
        .quad   0x7914892847927e9f
        .quad   0x33dad6ef370aa877
        .quad   0x1f8f24fa11122703
        .quad   0x5265ac2f2adf9592
        .quad   0x405fdd309afcb346
        .quad   0xd9723d4428e63f54
        .quad   0x94c01df05f65aaae
        .quad   0x43e4dc3ae14c0809

        // 2^60 * 3 * G

        .quad   0xbc12c7f1a938a517
        .quad   0x473028ab3180b2e1
        .quad   0x3f78571efbcd254a
        .quad   0x74e534426ff6f90f
        .quad   0xea6f7ac3adc2c6a3
        .quad   0xd0e928f6e9717c94
        .quad   0xe2d379ead645eaf5
        .quad   0x46dd8785c51ffbbe
        .quad   0x709801be375c8898
        .quad   0x4b06dab5e3fd8348
        .quad   0x75880ced27230714
        .quad   0x2b09468fdd2f4c42

        // 2^60 * 4 * G

        .quad   0x97c749eeb701cb96
        .quad   0x83f438d4b6a369c3
        .quad   0x62962b8b9a402cd9
        .quad   0x6976c7509888df7b
        .quad   0x5b97946582ffa02a
        .quad   0xda096a51fea8f549
        .quad   0xa06351375f77af9b
        .quad   0x1bcfde61201d1e76
        .quad   0x4a4a5490246a59a2
        .quad   0xd63ebddee87fdd90
        .quad   0xd9437c670d2371fa
        .quad   0x69e87308d30f8ed6

        // 2^60 * 5 * G

        .quad   0x435a8bb15656beb0
        .quad   0xf8fac9ba4f4d5bca
        .quad   0xb9b278c41548c075
        .quad   0x3eb0ef76e892b622
        .quad   0x0f80bf028bc80303
        .quad   0x6aae16b37a18cefb
        .quad   0xdd47ea47d72cd6a3
        .quad   0x61943588f4ed39aa
        .quad   0xd26e5c3e91039f85
        .quad   0xc0e9e77df6f33aa9
        .quad   0xe8968c5570066a93
        .quad   0x3c34d1881faaaddd

        // 2^60 * 6 * G

        .quad   0x3f9d2b5ea09f9ec0
        .quad   0x1dab3b6fb623a890
        .quad   0xa09ba3ea72d926c4
        .quad   0x374193513fd8b36d
        .quad   0xbd5b0b8f2fffe0d9
        .quad   0x6aa254103ed24fb9
        .quad   0x2ac7d7bcb26821c4
        .quad   0x605b394b60dca36a
        .quad   0xb4e856e45a9d1ed2
        .quad   0xefe848766c97a9a2
        .quad   0xb104cf641e5eee7d
        .quad   0x2f50b81c88a71c8f

        // 2^60 * 7 * G

        .quad   0x31723c61fc6811bb
        .quad   0x9cb450486211800f
        .quad   0x768933d347995753
        .quad   0x3491a53502752fcd
        .quad   0x2b552ca0a7da522a
        .quad   0x3230b336449b0250
        .quad   0xf2c4c5bca4b99fb9
        .quad   0x7b2c674958074a22
        .quad   0xd55165883ed28cdf
        .quad   0x12d84fd2d362de39
        .quad   0x0a874ad3e3378e4f
        .quad   0x000d2b1f7c763e74

        // 2^60 * 8 * G

        .quad   0x3d420811d06d4a67
        .quad   0xbefc048590e0ffe3
        .quad   0xf870c6b7bd487bde
        .quad   0x6e2a7316319afa28
        .quad   0x9624778c3e94a8ab
        .quad   0x0ad6f3cee9a78bec
        .quad   0x948ac7810d743c4f
        .quad   0x76627935aaecfccc
        .quad   0x56a8ac24d6d59a9f
        .quad   0xc8db753e3096f006
        .quad   0x477f41e68f4c5299
        .quad   0x588d851cf6c86114

        // 2^64 * 1 * G

        .quad   0x51138ec78df6b0fe
        .quad   0x5397da89e575f51b
        .quad   0x09207a1d717af1b9
        .quad   0x2102fdba2b20d650
        .quad   0xcd2a65e777d1f515
        .quad   0x548991878faa60f1
        .quad   0xb1b73bbcdabc06e5
        .quad   0x654878cba97cc9fb
        .quad   0x969ee405055ce6a1
        .quad   0x36bca7681251ad29
        .quad   0x3a1af517aa7da415
        .quad   0x0ad725db29ecb2ba

        // 2^64 * 2 * G

        .quad   0xdc4267b1834e2457
        .quad   0xb67544b570ce1bc5
        .quad   0x1af07a0bf7d15ed7
        .quad   0x4aefcffb71a03650
        .quad   0xfec7bc0c9b056f85
        .quad   0x537d5268e7f5ffd7
        .quad   0x77afc6624312aefa
        .quad   0x4f675f5302399fd9
        .quad   0xc32d36360415171e
        .quad   0xcd2bef118998483b
        .quad   0x870a6eadd0945110
        .quad   0x0bccbb72a2a86561

        // 2^64 * 3 * G

        .quad   0x185e962feab1a9c8
        .quad   0x86e7e63565147dcd
        .quad   0xb092e031bb5b6df2
        .quad   0x4024f0ab59d6b73e
        .quad   0x186d5e4c50fe1296
        .quad   0xe0397b82fee89f7e
        .quad   0x3bc7f6c5507031b0
        .quad   0x6678fd69108f37c2
        .quad   0x1586fa31636863c2
        .quad   0x07f68c48572d33f2
        .quad   0x4f73cc9f789eaefc
        .quad   0x2d42e2108ead4701

        // 2^64 * 4 * G

        .quad   0x97f5131594dfd29b
        .quad   0x6155985d313f4c6a
        .quad   0xeba13f0708455010
        .quad   0x676b2608b8d2d322
        .quad   0x21717b0d0f537593
        .quad   0x914e690b131e064c
        .quad   0x1bb687ae752ae09f
        .quad   0x420bf3a79b423c6e
        .quad   0x8138ba651c5b2b47
        .quad   0x8671b6ec311b1b80
        .quad   0x7bff0cb1bc3135b0
        .quad   0x745d2ffa9c0cf1e0

        // 2^64 * 5 * G

        .quad   0xbf525a1e2bc9c8bd
        .quad   0xea5b260826479d81
        .quad   0xd511c70edf0155db
        .quad   0x1ae23ceb960cf5d0
        .quad   0x6036df5721d34e6a
        .quad   0xb1db8827997bb3d0
        .quad   0xd3c209c3c8756afa
        .quad   0x06e15be54c1dc839
        .quad   0x5b725d871932994a
        .quad   0x32351cb5ceb1dab0
        .quad   0x7dc41549dab7ca05
        .quad   0x58ded861278ec1f7

        // 2^64 * 6 * G

        .quad   0xd8173793f266c55c
        .quad   0xc8c976c5cc454e49
        .quad   0x5ce382f8bc26c3a8
        .quad   0x2ff39de85485f6f9
        .quad   0x2dfb5ba8b6c2c9a8
        .quad   0x48eeef8ef52c598c
        .quad   0x33809107f12d1573
        .quad   0x08ba696b531d5bd8
        .quad   0x77ed3eeec3efc57a
        .quad   0x04e05517d4ff4811
        .quad   0xea3d7a3ff1a671cb
        .quad   0x120633b4947cfe54

        // 2^64 * 7 * G

        .quad   0x0b94987891610042
        .quad   0x4ee7b13cecebfae8
        .quad   0x70be739594f0a4c0
        .quad   0x35d30a99b4d59185
        .quad   0x82bd31474912100a
        .quad   0xde237b6d7e6fbe06
        .quad   0xe11e761911ea79c6
        .quad   0x07433be3cb393bde
        .quad   0xff7944c05ce997f4
        .quad   0x575d3de4b05c51a3
        .quad   0x583381fd5a76847c
        .quad   0x2d873ede7af6da9f

        // 2^64 * 8 * G

        .quad   0x157a316443373409
        .quad   0xfab8b7eef4aa81d9
        .quad   0xb093fee6f5a64806
        .quad   0x2e773654707fa7b6
        .quad   0xaa6202e14e5df981
        .quad   0xa20d59175015e1f5
        .quad   0x18a275d3bae21d6c
        .quad   0x0543618a01600253
        .quad   0x0deabdf4974c23c1
        .quad   0xaa6f0a259dce4693
        .quad   0x04202cb8a29aba2c
        .quad   0x4b1443362d07960d

        // 2^68 * 1 * G

        .quad   0x47b837f753242cec
        .quad   0x256dc48cc04212f2
        .quad   0xe222fbfbe1d928c5
        .quad   0x48ea295bad8a2c07
        .quad   0x299b1c3f57c5715e
        .quad   0x96cb929e6b686d90
        .quad   0x3004806447235ab3
        .quad   0x2c435c24a44d9fe1
        .quad   0x0607c97c80f8833f
        .quad   0x0e851578ca25ec5b
        .quad   0x54f7450b161ebb6f
        .quad   0x7bcb4792a0def80e

        // 2^68 * 2 * G

        .quad   0x8487e3d02bc73659
        .quad   0x4baf8445059979df
        .quad   0xd17c975adcad6fbf
        .quad   0x57369f0bdefc96b6
        .quad   0x1cecd0a0045224c2
        .quad   0x757f1b1b69e53952
        .quad   0x775b7a925289f681
        .quad   0x1b6cc62016736148
        .quad   0xf1a9990175638698
        .quad   0x353dd1beeeaa60d3
        .quad   0x849471334c9ba488
        .quad   0x63fa6e6843ade311

        // 2^68 * 3 * G

        .quad   0xd15c20536597c168
        .quad   0x9f73740098d28789
        .quad   0x18aee7f13257ba1f
        .quad   0x3418bfda07346f14
        .quad   0x2195becdd24b5eb7
        .quad   0x5e41f18cc0cd44f9
        .quad   0xdf28074441ca9ede
        .quad   0x07073b98f35b7d67
        .quad   0xd03c676c4ce530d4
        .quad   0x0b64c0473b5df9f4
        .quad   0x065cef8b19b3a31e
        .quad   0x3084d661533102c9

        // 2^68 * 4 * G

        .quad   0xe1f6b79ebf8469ad
        .quad   0x15801004e2663135
        .quad   0x9a498330af74181b
        .quad   0x3ba2504f049b673c
        .quad   0x9a6ce876760321fd
        .quad   0x7fe2b5109eb63ad8
        .quad   0x00e7d4ae8ac80592
        .quad   0x73d86b7abb6f723a
        .quad   0x0b52b5606dba5ab6
        .quad   0xa9134f0fbbb1edab
        .quad   0x30a9520d9b04a635
        .quad   0x6813b8f37973e5db

        // 2^68 * 5 * G

        .quad   0x9854b054334127c1
        .quad   0x105d047882fbff25
        .quad   0xdb49f7f944186f4f
        .quad   0x1768e838bed0b900
        .quad   0xf194ca56f3157e29
        .quad   0x136d35705ef528a5
        .quad   0xdd4cef778b0599bc
        .quad   0x7d5472af24f833ed
        .quad   0xd0ef874daf33da47
        .quad   0x00d3be5db6e339f9
        .quad   0x3f2a8a2f9c9ceece
        .quad   0x5d1aeb792352435a

        // 2^68 * 6 * G

        .quad   0xf59e6bb319cd63ca
        .quad   0x670c159221d06839
        .quad   0xb06d565b2150cab6
        .quad   0x20fb199d104f12a3
        .quad   0x12c7bfaeb61ba775
        .quad   0xb84e621fe263bffd
        .quad   0x0b47a5c35c840dcf
        .quad   0x7e83be0bccaf8634
        .quad   0x61943dee6d99c120
        .quad   0x86101f2e460b9fe0
        .quad   0x6bb2f1518ee8598d
        .quad   0x76b76289fcc475cc

        // 2^68 * 7 * G

        .quad   0x791b4cc1756286fa
        .quad   0xdbced317d74a157c
        .quad   0x7e732421ea72bde6
        .quad   0x01fe18491131c8e9
        .quad   0x4245f1a1522ec0b3
        .quad   0x558785b22a75656d
        .quad   0x1d485a2548a1b3c0
        .quad   0x60959eccd58fe09f
        .quad   0x3ebfeb7ba8ed7a09
        .quad   0x49fdc2bbe502789c
        .quad   0x44ebce5d3c119428
        .quad   0x35e1eb55be947f4a

        // 2^68 * 8 * G

        .quad   0xdbdae701c5738dd3
        .quad   0xf9c6f635b26f1bee
        .quad   0x61e96a8042f15ef4
        .quad   0x3aa1d11faf60a4d8
        .quad   0x14fd6dfa726ccc74
        .quad   0x3b084cfe2f53b965
        .quad   0xf33ae4f552a2c8b4
        .quad   0x59aab07a0d40166a
        .quad   0x77bcec4c925eac25
        .quad   0x1848718460137738
        .quad   0x5b374337fea9f451
        .quad   0x1865e78ec8e6aa46

        // 2^72 * 1 * G

        .quad   0xccc4b7c7b66e1f7a
        .quad   0x44157e25f50c2f7e
        .quad   0x3ef06dfc713eaf1c
        .quad   0x582f446752da63f7
        .quad   0x967c54e91c529ccb
        .quad   0x30f6269264c635fb
        .quad   0x2747aff478121965
        .quad   0x17038418eaf66f5c
        .quad   0xc6317bd320324ce4
        .quad   0xa81042e8a4488bc4
        .quad   0xb21ef18b4e5a1364
        .quad   0x0c2a1c4bcda28dc9

        // 2^72 * 2 * G

        .quad   0xd24dc7d06f1f0447
        .quad   0xb2269e3edb87c059
        .quad   0xd15b0272fbb2d28f
        .quad   0x7c558bd1c6f64877
        .quad   0xedc4814869bd6945
        .quad   0x0d6d907dbe1c8d22
        .quad   0xc63bd212d55cc5ab
        .quad   0x5a6a9b30a314dc83
        .quad   0xd0ec1524d396463d
        .quad   0x12bb628ac35a24f0
        .quad   0xa50c3a791cbc5fa4
        .quad   0x0404a5ca0afbafc3

        // 2^72 * 3 * G

        .quad   0x8c1f40070aa743d6
        .quad   0xccbad0cb5b265ee8
        .quad   0x574b046b668fd2de
        .quad   0x46395bfdcadd9633
        .quad   0x62bc9e1b2a416fd1
        .quad   0xb5c6f728e350598b
        .quad   0x04343fd83d5d6967
        .quad   0x39527516e7f8ee98
        .quad   0x117fdb2d1a5d9a9c
        .quad   0x9c7745bcd1005c2a
        .quad   0xefd4bef154d56fea
        .quad   0x76579a29e822d016

        // 2^72 * 4 * G

        .quad   0x45b68e7e49c02a17
        .quad   0x23cd51a2bca9a37f
        .quad   0x3ed65f11ec224c1b
        .quad   0x43a384dc9e05bdb1
        .quad   0x333cb51352b434f2
        .quad   0xd832284993de80e1
        .quad   0xb5512887750d35ce
        .quad   0x02c514bb2a2777c1
        .quad   0x684bd5da8bf1b645
        .quad   0xfb8bd37ef6b54b53
        .quad   0x313916d7a9b0d253
        .quad   0x1160920961548059

        // 2^72 * 5 * G

        .quad   0xb44d166929dacfaa
        .quad   0xda529f4c8413598f
        .quad   0xe9ef63ca453d5559
        .quad   0x351e125bc5698e0b
        .quad   0x7a385616369b4dcd
        .quad   0x75c02ca7655c3563
        .quad   0x7dc21bf9d4f18021
        .quad   0x2f637d7491e6e042
        .quad   0xd4b49b461af67bbe
        .quad   0xd603037ac8ab8961
        .quad   0x71dee19ff9a699fb
        .quad   0x7f182d06e7ce2a9a

        // 2^72 * 6 * G

        .quad   0x7a7c8e64ab0168ec
        .quad   0xcb5a4a5515edc543
        .quad   0x095519d347cd0eda
        .quad   0x67d4ac8c343e93b0
        .quad   0x09454b728e217522
        .quad   0xaa58e8f4d484b8d8
        .quad   0xd358254d7f46903c
        .quad   0x44acc043241c5217
        .quad   0x1c7d6bbb4f7a5777
        .quad   0x8b35fed4918313e1
        .quad   0x4adca1c6c96b4684
        .quad   0x556d1c8312ad71bd

        // 2^72 * 7 * G

        .quad   0x17ef40e30c8d3982
        .quad   0x31f7073e15a3fa34
        .quad   0x4f21f3cb0773646e
        .quad   0x746c6c6d1d824eff
        .quad   0x81f06756b11be821
        .quad   0x0faff82310a3f3dd
        .quad   0xf8b2d0556a99465d
        .quad   0x097abe38cc8c7f05
        .quad   0x0c49c9877ea52da4
        .quad   0x4c4369559bdc1d43
        .quad   0x022c3809f7ccebd2
        .quad   0x577e14a34bee84bd

        // 2^72 * 8 * G

        .quad   0xf0e268ac61a73b0a
        .quad   0xf2fafa103791a5f5
        .quad   0xc1e13e826b6d00e9
        .quad   0x60fa7ee96fd78f42
        .quad   0x94fecebebd4dd72b
        .quad   0xf46a4fda060f2211
        .quad   0x124a5977c0c8d1ff
        .quad   0x705304b8fb009295
        .quad   0xb63d1d354d296ec6
        .quad   0xf3c3053e5fad31d8
        .quad   0x670b958cb4bd42ec
        .quad   0x21398e0ca16353fd

        // 2^76 * 1 * G

        .quad   0x216ab2ca8da7d2ef
        .quad   0x366ad9dd99f42827
        .quad   0xae64b9004fdd3c75
        .quad   0x403a395b53909e62
        .quad   0x86c5fc16861b7e9a
        .quad   0xf6a330476a27c451
        .quad   0x01667267a1e93597
        .quad   0x05ffb9cd6082dfeb
        .quad   0xa617fa9ff53f6139
        .quad   0x60f2b5e513e66cb6
        .quad   0xd7a8beefb3448aa4
        .quad   0x7a2932856f5ea192

        // 2^76 * 2 * G

        .quad   0x0b39d761b02de888
        .quad   0x5f550e7ed2414e1f
        .quad   0xa6bfa45822e1a940
        .quad   0x050a2f7dfd447b99
        .quad   0xb89c444879639302
        .quad   0x4ae4f19350c67f2c
        .quad   0xf0b35da8c81af9c6
        .quad   0x39d0003546871017
        .quad   0x437c3b33a650db77
        .quad   0x6bafe81dbac52bb2
        .quad   0xfe99402d2db7d318
        .quad   0x2b5b7eec372ba6ce

        // 2^76 * 3 * G

        .quad   0xb3bc4bbd83f50eef
        .quad   0x508f0c998c927866
        .quad   0x43e76587c8b7e66e
        .quad   0x0f7655a3a47f98d9
        .quad   0xa694404d613ac8f4
        .quad   0x500c3c2bfa97e72c
        .quad   0x874104d21fcec210
        .quad   0x1b205fb38604a8ee
        .quad   0x55ecad37d24b133c
        .quad   0x441e147d6038c90b
        .quad   0x656683a1d62c6fee
        .quad   0x0157d5dc87e0ecae

        // 2^76 * 4 * G

        .quad   0xf2a7af510354c13d
        .quad   0xd7a0b145aa372b60
        .quad   0x2869b96a05a3d470
        .quad   0x6528e42d82460173
        .quad   0x95265514d71eb524
        .quad   0xe603d8815df14593
        .quad   0x147cdf410d4de6b7
        .quad   0x5293b1730437c850
        .quad   0x23d0e0814bccf226
        .quad   0x92c745cd8196fb93
        .quad   0x8b61796c59541e5b
        .quad   0x40a44df0c021f978

        // 2^76 * 5 * G

        .quad   0xdaa869894f20ea6a
        .quad   0xea14a3d14c620618
        .quad   0x6001fccb090bf8be
        .quad   0x35f4e822947e9cf0
        .quad   0x86c96e514bc5d095
        .quad   0xf20d4098fca6804a
        .quad   0x27363d89c826ea5d
        .quad   0x39ca36565719cacf
        .quad   0x97506f2f6f87b75c
        .quad   0xc624aea0034ae070
        .quad   0x1ec856e3aad34dd6
        .quad   0x055b0be0e440e58f

        // 2^76 * 6 * G

        .quad   0x6469a17d89735d12
        .quad   0xdb6f27d5e662b9f1
        .quad   0x9fcba3286a395681
        .quad   0x363b8004d269af25
        .quad   0x4d12a04b6ea33da2
        .quad   0x57cf4c15e36126dd
        .quad   0x90ec9675ee44d967
        .quad   0x64ca348d2a985aac
        .quad   0x99588e19e4c4912d
        .quad   0xefcc3b4e1ca5ce6b
        .quad   0x4522ea60fa5b98d5
        .quad   0x7064bbab1de4a819

        // 2^76 * 7 * G

        .quad   0xb919e1515a770641
        .quad   0xa9a2e2c74e7f8039
        .quad   0x7527250b3df23109
        .quad   0x756a7330ac27b78b
        .quad   0xa290c06142542129
        .quad   0xf2e2c2aebe8d5b90
        .quad   0xcf2458db76abfe1b
        .quad   0x02157ade83d626bf
        .quad   0x3e46972a1b9a038b
        .quad   0x2e4ee66a7ee03fb4
        .quad   0x81a248776edbb4ca
        .quad   0x1a944ee88ecd0563

        // 2^76 * 8 * G

        .quad   0xd5a91d1151039372
        .quad   0x2ed377b799ca26de
        .quad   0xa17202acfd366b6b
        .quad   0x0730291bd6901995
        .quad   0xbb40a859182362d6
        .quad   0xb99f55778a4d1abb
        .quad   0x8d18b427758559f6
        .quad   0x26c20fe74d26235a
        .quad   0x648d1d9fe9cc22f5
        .quad   0x66bc561928dd577c
        .quad   0x47d3ed21652439d1
        .quad   0x49d271acedaf8b49

        // 2^80 * 1 * G

        .quad   0x89f5058a382b33f3
        .quad   0x5ae2ba0bad48c0b4
        .quad   0x8f93b503a53db36e
        .quad   0x5aa3ed9d95a232e6
        .quad   0x2798aaf9b4b75601
        .quad   0x5eac72135c8dad72
        .quad   0xd2ceaa6161b7a023
        .quad   0x1bbfb284e98f7d4e
        .quad   0x656777e9c7d96561
        .quad   0xcb2b125472c78036
        .quad   0x65053299d9506eee
        .quad   0x4a07e14e5e8957cc

        // 2^80 * 2 * G

        .quad   0x4ee412cb980df999
        .quad   0xa315d76f3c6ec771
        .quad   0xbba5edde925c77fd
        .quad   0x3f0bac391d313402
        .quad   0x240b58cdc477a49b
        .quad   0xfd38dade6447f017
        .quad   0x19928d32a7c86aad
        .quad   0x50af7aed84afa081
        .quad   0x6e4fde0115f65be5
        .quad   0x29982621216109b2
        .quad   0x780205810badd6d9
        .quad   0x1921a316baebd006

        // 2^80 * 3 * G

        .quad   0x89422f7edfb870fc
        .quad   0x2c296beb4f76b3bd
        .quad   0x0738f1d436c24df7
        .quad   0x6458df41e273aeb0
        .quad   0xd75aad9ad9f3c18b
        .quad   0x566a0eef60b1c19c
        .quad   0x3e9a0bac255c0ed9
        .quad   0x7b049deca062c7f5
        .quad   0xdccbe37a35444483
        .quad   0x758879330fedbe93
        .quad   0x786004c312c5dd87
        .quad   0x6093dccbc2950e64

        // 2^80 * 4 * G

        .quad   0x1ff39a8585e0706d
        .quad   0x36d0a5d8b3e73933
        .quad   0x43b9f2e1718f453b
        .quad   0x57d1ea084827a97c
        .quad   0x6bdeeebe6084034b
        .quad   0x3199c2b6780fb854
        .quad   0x973376abb62d0695
        .quad   0x6e3180c98b647d90
        .quad   0xee7ab6e7a128b071
        .quad   0xa4c1596d93a88baa
        .quad   0xf7b4de82b2216130
        .quad   0x363e999ddd97bd18

        // 2^80 * 5 * G

        .quad   0x96a843c135ee1fc4
        .quad   0x976eb35508e4c8cf
        .quad   0xb42f6801b58cd330
        .quad   0x48ee9b78693a052b
        .quad   0x2f1848dce24baec6
        .quad   0x769b7255babcaf60
        .quad   0x90cb3c6e3cefe931
        .quad   0x231f979bc6f9b355
        .quad   0x5c31de4bcc2af3c6
        .quad   0xb04bb030fe208d1f
        .quad   0xb78d7009c14fb466
        .quad   0x079bfa9b08792413

        // 2^80 * 6 * G

        .quad   0xe3903a51da300df4
        .quad   0x843964233da95ab0
        .quad   0xed3cf12d0b356480
        .quad   0x038c77f684817194
        .quad   0xf3c9ed80a2d54245
        .quad   0x0aa08b7877f63952
        .quad   0xd76dac63d1085475
        .quad   0x1ef4fb159470636b
        .quad   0x854e5ee65b167bec
        .quad   0x59590a4296d0cdc2
        .quad   0x72b2df3498102199
        .quad   0x575ee92a4a0bff56

        // 2^80 * 7 * G

        .quad   0xd4c080908a182fcf
        .quad   0x30e170c299489dbd
        .quad   0x05babd5752f733de
        .quad   0x43d4e7112cd3fd00
        .quad   0x5d46bc450aa4d801
        .quad   0xc3af1227a533b9d8
        .quad   0x389e3b262b8906c2
        .quad   0x200a1e7e382f581b
        .quad   0x518db967eaf93ac5
        .quad   0x71bc989b056652c0
        .quad   0xfe2b85d9567197f5
        .quad   0x050eca52651e4e38

        // 2^80 * 8 * G

        .quad   0xc3431ade453f0c9c
        .quad   0xe9f5045eff703b9b
        .quad   0xfcd97ac9ed847b3d
        .quad   0x4b0ee6c21c58f4c6
        .quad   0x97ac397660e668ea
        .quad   0x9b19bbfe153ab497
        .quad   0x4cb179b534eca79f
        .quad   0x6151c09fa131ae57
        .quad   0x3af55c0dfdf05d96
        .quad   0xdd262ee02ab4ee7a
        .quad   0x11b2bb8712171709
        .quad   0x1fef24fa800f030b

        // 2^84 * 1 * G

        .quad   0xb496123a6b6c6609
        .quad   0xa750fe8580ab5938
        .quad   0xf471bf39b7c27a5f
        .quad   0x507903ce77ac193c
        .quad   0xff91a66a90166220
        .quad   0xf22552ae5bf1e009
        .quad   0x7dff85d87f90df7c
        .quad   0x4f620ffe0c736fb9
        .quad   0x62f90d65dfde3e34
        .quad   0xcf28c592b9fa5fad
        .quad   0x99c86ef9c6164510
        .quad   0x25d448044a256c84

        // 2^84 * 2 * G

        .quad   0xbd68230ec7e9b16f
        .quad   0x0eb1b9c1c1c5795d
        .quad   0x7943c8c495b6b1ff
        .quad   0x2f9faf620bbacf5e
        .quad   0x2c7c4415c9022b55
        .quad   0x56a0d241812eb1fe
        .quad   0xf02ea1c9d7b65e0d
        .quad   0x4180512fd5323b26
        .quad   0xa4ff3e698a48a5db
        .quad   0xba6a3806bd95403b
        .quad   0x9f7ce1af47d5b65d
        .quad   0x15e087e55939d2fb

        // 2^84 * 3 * G

        .quad   0x12207543745c1496
        .quad   0xdaff3cfdda38610c
        .quad   0xe4e797272c71c34f
        .quad   0x39c07b1934bdede9
        .quad   0x8894186efb963f38
        .quad   0x48a00e80dc639bd5
        .quad   0xa4e8092be96c1c99
        .quad   0x5a097d54ca573661
        .quad   0x2d45892b17c9e755
        .quad   0xd033fd7289308df8
        .quad   0x6c2fe9d9525b8bd9
        .quad   0x2edbecf1c11cc079

        // 2^84 * 4 * G

        .quad   0x1616a4e3c715a0d2
        .quad   0x53623cb0f8341d4d
        .quad   0x96ef5329c7e899cb
        .quad   0x3d4e8dbba668baa6
        .quad   0xee0f0fddd087a25f
        .quad   0x9c7531555c3e34ee
        .quad   0x660c572e8fab3ab5
        .quad   0x0854fc44544cd3b2
        .quad   0x61eba0c555edad19
        .quad   0x24b533fef0a83de6
        .quad   0x3b77042883baa5f8
        .quad   0x678f82b898a47e8d

        // 2^84 * 5 * G

        .quad   0xb1491d0bd6900c54
        .quad   0x3539722c9d132636
        .quad   0x4db928920b362bc9
        .quad   0x4d7cd1fea68b69df
        .quad   0x1e09d94057775696
        .quad   0xeed1265c3cd951db
        .quad   0xfa9dac2b20bce16f
        .quad   0x0f7f76e0e8d089f4
        .quad   0x36d9ebc5d485b00c
        .quad   0xa2596492e4adb365
        .quad   0xc1659480c2119ccd
        .quad   0x45306349186e0d5f

        // 2^84 * 6 * G

        .quad   0x94ddd0c1a6cdff1d
        .quad   0x55f6f115e84213ae
        .quad   0x6c935f85992fcf6a
        .quad   0x067ee0f54a37f16f
        .quad   0x96a414ec2b072491
        .quad   0x1bb2218127a7b65b
        .quad   0x6d2849596e8a4af0
        .quad   0x65f3b08ccd27765f
        .quad   0xecb29fff199801f7
        .quad   0x9d361d1fa2a0f72f
        .quad   0x25f11d2375fd2f49
        .quad   0x124cefe80fe10fe2

        // 2^84 * 7 * G

        .quad   0x4c126cf9d18df255
        .quad   0xc1d471e9147a63b6
        .quad   0x2c6d3c73f3c93b5f
        .quad   0x6be3a6a2e3ff86a2
        .quad   0x1518e85b31b16489
        .quad   0x8faadcb7db710bfb
        .quad   0x39b0bdf4a14ae239
        .quad   0x05f4cbea503d20c1
        .quad   0xce040e9ec04145bc
        .quad   0xc71ff4e208f6834c
        .quad   0xbd546e8dab8847a3
        .quad   0x64666aa0a4d2aba5

        // 2^84 * 8 * G

        .quad   0x6841435a7c06d912
        .quad   0xca123c21bb3f830b
        .quad   0xd4b37b27b1cbe278
        .quad   0x1d753b84c76f5046
        .quad   0xb0c53bf73337e94c
        .quad   0x7cb5697e11e14f15
        .quad   0x4b84abac1930c750
        .quad   0x28dd4abfe0640468
        .quad   0x7dc0b64c44cb9f44
        .quad   0x18a3e1ace3925dbf
        .quad   0x7a3034862d0457c4
        .quad   0x4c498bf78a0c892e

        // 2^88 * 1 * G

        .quad   0x37d653fb1aa73196
        .quad   0x0f9495303fd76418
        .quad   0xad200b09fb3a17b2
        .quad   0x544d49292fc8613e
        .quad   0x22d2aff530976b86
        .quad   0x8d90b806c2d24604
        .quad   0xdca1896c4de5bae5
        .quad   0x28005fe6c8340c17
        .quad   0x6aefba9f34528688
        .quad   0x5c1bff9425107da1
        .quad   0xf75bbbcd66d94b36
        .quad   0x72e472930f316dfa

        // 2^88 * 2 * G

        .quad   0x2695208c9781084f
        .quad   0xb1502a0b23450ee1
        .quad   0xfd9daea603efde02
        .quad   0x5a9d2e8c2733a34c
        .quad   0x07f3f635d32a7627
        .quad   0x7aaa4d865f6566f0
        .quad   0x3c85e79728d04450
        .quad   0x1fee7f000fe06438
        .quad   0x765305da03dbf7e5
        .quad   0xa4daf2491434cdbd
        .quad   0x7b4ad5cdd24a88ec
        .quad   0x00f94051ee040543

        // 2^88 * 3 * G

        .quad   0x8d356b23c3d330b2
        .quad   0xf21c8b9bb0471b06
        .quad   0xb36c316c6e42b83c
        .quad   0x07d79c7e8beab10d
        .quad   0xd7ef93bb07af9753
        .quad   0x583ed0cf3db766a7
        .quad   0xce6998bf6e0b1ec5
        .quad   0x47b7ffd25dd40452
        .quad   0x87fbfb9cbc08dd12
        .quad   0x8a066b3ae1eec29b
        .quad   0x0d57242bdb1fc1bf
        .quad   0x1c3520a35ea64bb6

        // 2^88 * 4 * G

        .quad   0x80d253a6bccba34a
        .quad   0x3e61c3a13838219b
        .quad   0x90c3b6019882e396
        .quad   0x1c3d05775d0ee66f
        .quad   0xcda86f40216bc059
        .quad   0x1fbb231d12bcd87e
        .quad   0xb4956a9e17c70990
        .quad   0x38750c3b66d12e55
        .quad   0x692ef1409422e51a
        .quad   0xcbc0c73c2b5df671
        .quad   0x21014fe7744ce029
        .quad   0x0621e2c7d330487c

        // 2^88 * 5 * G

        .quad   0xaf9860cc8259838d
        .quad   0x90ea48c1c69f9adc
        .quad   0x6526483765581e30
        .quad   0x0007d6097bd3a5bc
        .quad   0xb7ae1796b0dbf0f3
        .quad   0x54dfafb9e17ce196
        .quad   0x25923071e9aaa3b4
        .quad   0x5d8e589ca1002e9d
        .quad   0xc0bf1d950842a94b
        .quad   0xb2d3c363588f2e3e
        .quad   0x0a961438bb51e2ef
        .quad   0x1583d7783c1cbf86

        // 2^88 * 6 * G

        .quad   0xeceea2ef5da27ae1
        .quad   0x597c3a1455670174
        .quad   0xc9a62a126609167a
        .quad   0x252a5f2e81ed8f70
        .quad   0x90034704cc9d28c7
        .quad   0x1d1b679ef72cc58f
        .quad   0x16e12b5fbe5b8726
        .quad   0x4958064e83c5580a
        .quad   0x0d2894265066e80d
        .quad   0xfcc3f785307c8c6b
        .quad   0x1b53da780c1112fd
        .quad   0x079c170bd843b388

        // 2^88 * 7 * G

        .quad   0x0506ece464fa6fff
        .quad   0xbee3431e6205e523
        .quad   0x3579422451b8ea42
        .quad   0x6dec05e34ac9fb00
        .quad   0xcdd6cd50c0d5d056
        .quad   0x9af7686dbb03573b
        .quad   0x3ca6723ff3c3ef48
        .quad   0x6768c0d7317b8acc
        .quad   0x94b625e5f155c1b3
        .quad   0x417bf3a7997b7b91
        .quad   0xc22cbddc6d6b2600
        .quad   0x51445e14ddcd52f4

        // 2^88 * 8 * G

        .quad   0x57502b4b3b144951
        .quad   0x8e67ff6b444bbcb3
        .quad   0xb8bd6927166385db
        .quad   0x13186f31e39295c8
        .quad   0x893147ab2bbea455
        .quad   0x8c53a24f92079129
        .quad   0x4b49f948be30f7a7
        .quad   0x12e990086e4fd43d
        .quad   0xf10c96b37fdfbb2e
        .quad   0x9f9a935e121ceaf9
        .quad   0xdf1136c43a5b983f
        .quad   0x77b2e3f05d3e99af

        // 2^92 * 1 * G

        .quad   0xfd0d75879cf12657
        .quad   0xe82fef94e53a0e29
        .quad   0xcc34a7f05bbb4be7
        .quad   0x0b251172a50c38a2
        .quad   0x9532f48fcc5cd29b
        .quad   0x2ba851bea3ce3671
        .quad   0x32dacaa051122941
        .quad   0x478d99d9350004f2
        .quad   0x1d5ad94890bb02c0
        .quad   0x50e208b10ec25115
        .quad   0xa26a22894ef21702
        .quad   0x4dc923343b524805

        // 2^92 * 2 * G

        .quad   0xe3828c400f8086b6
        .quad   0x3f77e6f7979f0dc8
        .quad   0x7ef6de304df42cb4
        .quad   0x5265797cb6abd784
        .quad   0x3ad3e3ebf36c4975
        .quad   0xd75d25a537862125
        .quad   0xe873943da025a516
        .quad   0x6bbc7cb4c411c847
        .quad   0x3c6f9cd1d4a50d56
        .quad   0xb6244077c6feab7e
        .quad   0x6ff9bf483580972e
        .quad   0x00375883b332acfb

        // 2^92 * 3 * G

        .quad   0x0001b2cd28cb0940
        .quad   0x63fb51a06f1c24c9
        .quad   0xb5ad8691dcd5ca31
        .quad   0x67238dbd8c450660
        .quad   0xc98bec856c75c99c
        .quad   0xe44184c000e33cf4
        .quad   0x0a676b9bba907634
        .quad   0x669e2cb571f379d7
        .quad   0xcb116b73a49bd308
        .quad   0x025aad6b2392729e
        .quad   0xb4793efa3f55d9b1
        .quad   0x72a1056140678bb9

        // 2^92 * 4 * G

        .quad   0xa2b6812b1cc9249d
        .quad   0x62866eee21211f58
        .quad   0x2cb5c5b85df10ece
        .quad   0x03a6b259e263ae00
        .quad   0x0d8d2909e2e505b6
        .quad   0x98ca78abc0291230
        .quad   0x77ef5569a9b12327
        .quad   0x7c77897b81439b47
        .quad   0xf1c1b5e2de331cb5
        .quad   0x5a9f5d8e15fca420
        .quad   0x9fa438f17bd932b1
        .quad   0x2a381bf01c6146e7

        // 2^92 * 5 * G

        .quad   0xac9b9879cfc811c1
        .quad   0x8b7d29813756e567
        .quad   0x50da4e607c70edfc
        .quad   0x5dbca62f884400b6
        .quad   0xf7c0be32b534166f
        .quad   0x27e6ca6419cf70d4
        .quad   0x934df7d7a957a759
        .quad   0x5701461dabdec2aa
        .quad   0x2c6747402c915c25
        .quad   0x1bdcd1a80b0d340a
        .quad   0x5e5601bd07b43f5f
        .quad   0x2555b4e05539a242

        // 2^92 * 6 * G

        .quad   0x6fc09f5266ddd216
        .quad   0xdce560a7c8e37048
        .quad   0xec65939da2df62fd
        .quad   0x7a869ae7e52ed192
        .quad   0x78409b1d87e463d4
        .quad   0xad4da95acdfb639d
        .quad   0xec28773755259b9c
        .quad   0x69c806e9c31230ab
        .quad   0x7b48f57414bb3f22
        .quad   0x68c7cee4aedccc88
        .quad   0xed2f936179ed80be
        .quad   0x25d70b885f77bc4b

        // 2^92 * 7 * G

        .quad   0x4151c3d9762bf4de
        .quad   0x083f435f2745d82b
        .quad   0x29775a2e0d23ddd5
        .quad   0x138e3a6269a5db24
        .quad   0x98459d29bb1ae4d4
        .quad   0x56b9c4c739f954ec
        .quad   0x832743f6c29b4b3e
        .quad   0x21ea8e2798b6878a
        .quad   0x87bef4b46a5a7b9c
        .quad   0xd2299d1b5fc1d062
        .quad   0x82409818dd321648
        .quad   0x5c5abeb1e5a2e03d

        // 2^92 * 8 * G

        .quad   0x14722af4b73c2ddb
        .quad   0xbc470c5f5a05060d
        .quad   0x00943eac2581b02e
        .quad   0x0e434b3b1f499c8f
        .quad   0x02cde6de1306a233
        .quad   0x7b5a52a2116f8ec7
        .quad   0xe1c681f4c1163b5b
        .quad   0x241d350660d32643
        .quad   0x6be4404d0ebc52c7
        .quad   0xae46233bb1a791f5
        .quad   0x2aec170ed25db42b
        .quad   0x1d8dfd966645d694

        // 2^96 * 1 * G

        .quad   0x296fa9c59c2ec4de
        .quad   0xbc8b61bf4f84f3cb
        .quad   0x1c7706d917a8f908
        .quad   0x63b795fc7ad3255d
        .quad   0xd598639c12ddb0a4
        .quad   0xa5d19f30c024866b
        .quad   0xd17c2f0358fce460
        .quad   0x07a195152e095e8a
        .quad   0xa8368f02389e5fc8
        .quad   0x90433b02cf8de43b
        .quad   0xafa1fd5dc5412643
        .quad   0x3e8fe83d032f0137

        // 2^96 * 2 * G

        .quad   0x2f8b15b90570a294
        .quad   0x94f2427067084549
        .quad   0xde1c5ae161bbfd84
        .quad   0x75ba3b797fac4007
        .quad   0x08704c8de8efd13c
        .quad   0xdfc51a8e33e03731
        .quad   0xa59d5da51260cde3
        .quad   0x22d60899a6258c86
        .quad   0x6239dbc070cdd196
        .quad   0x60fe8a8b6c7d8a9a
        .quad   0xb38847bceb401260
        .quad   0x0904d07b87779e5e

        // 2^96 * 3 * G

        .quad   0xb4ce1fd4ddba919c
        .quad   0xcf31db3ec74c8daa
        .quad   0x2c63cc63ad86cc51
        .quad   0x43e2143fbc1dde07
        .quad   0xf4322d6648f940b9
        .quad   0x06952f0cbd2d0c39
        .quad   0x167697ada081f931
        .quad   0x6240aacebaf72a6c
        .quad   0xf834749c5ba295a0
        .quad   0xd6947c5bca37d25a
        .quad   0x66f13ba7e7c9316a
        .quad   0x56bdaf238db40cac

        // 2^96 * 4 * G

        .quad   0x362ab9e3f53533eb
        .quad   0x338568d56eb93d40
        .quad   0x9e0e14521d5a5572
        .quad   0x1d24a86d83741318
        .quad   0x1310d36cc19d3bb2
        .quad   0x062a6bb7622386b9
        .quad   0x7c9b8591d7a14f5c
        .quad   0x03aa31507e1e5754
        .quad   0xf4ec7648ffd4ce1f
        .quad   0xe045eaf054ac8c1c
        .quad   0x88d225821d09357c
        .quad   0x43b261dc9aeb4859

        // 2^96 * 5 * G

        .quad   0xe55b1e1988bb79bb
        .quad   0xa09ed07dc17a359d
        .quad   0xb02c2ee2603dea33
        .quad   0x326055cf5b276bc2
        .quad   0x19513d8b6c951364
        .quad   0x94fe7126000bf47b
        .quad   0x028d10ddd54f9567
        .quad   0x02b4d5e242940964
        .quad   0xb4a155cb28d18df2
        .quad   0xeacc4646186ce508
        .quad   0xc49cf4936c824389
        .quad   0x27a6c809ae5d3410

        // 2^96 * 6 * G

        .quad   0x8ba6ebcd1f0db188
        .quad   0x37d3d73a675a5be8
        .quad   0xf22edfa315f5585a
        .quad   0x2cb67174ff60a17e
        .quad   0xcd2c270ac43d6954
        .quad   0xdd4a3e576a66cab2
        .quad   0x79fa592469d7036c
        .quad   0x221503603d8c2599
        .quad   0x59eecdf9390be1d0
        .quad   0xa9422044728ce3f1
        .quad   0x82891c667a94f0f4
        .quad   0x7b1df4b73890f436

        // 2^96 * 7 * G

        .quad   0xe492f2e0b3b2a224
        .quad   0x7c6c9e062b551160
        .quad   0x15eb8fe20d7f7b0e
        .quad   0x61fcef2658fc5992
        .quad   0x5f2e221807f8f58c
        .quad   0xe3555c9fd49409d4
        .quad   0xb2aaa88d1fb6a630
        .quad   0x68698245d352e03d
        .quad   0xdbb15d852a18187a
        .quad   0xf3e4aad386ddacd7
        .quad   0x44bae2810ff6c482
        .quad   0x46cf4c473daf01cf

        // 2^96 * 8 * G

        .quad   0x426525ed9ec4e5f9
        .quad   0x0e5eda0116903303
        .quad   0x72b1a7f2cbe5cadc
        .quad   0x29387bcd14eb5f40
        .quad   0x213c6ea7f1498140
        .quad   0x7c1e7ef8392b4854
        .quad   0x2488c38c5629ceba
        .quad   0x1065aae50d8cc5bb
        .quad   0x1c2c4525df200d57
        .quad   0x5c3b2dd6bfca674a
        .quad   0x0a07e7b1e1834030
        .quad   0x69a198e64f1ce716

        // 2^100 * 1 * G

        .quad   0x7afcd613efa9d697
        .quad   0x0cc45aa41c067959
        .quad   0xa56fe104c1fada96
        .quad   0x3a73b70472e40365
        .quad   0x7b26e56b9e2d4734
        .quad   0xc4c7132b81c61675
        .quad   0xef5c9525ec9cde7f
        .quad   0x39c80b16e71743ad
        .quad   0x0f196e0d1b826c68
        .quad   0xf71ff0e24960e3db
        .quad   0x6113167023b7436c
        .quad   0x0cf0ea5877da7282

        // 2^100 * 2 * G

        .quad   0x196c80a4ddd4ccbd
        .quad   0x22e6f55d95f2dd9d
        .quad   0xc75e33c740d6c71b
        .quad   0x7bb51279cb3c042f
        .quad   0xe332ced43ba6945a
        .quad   0xde0b1361e881c05d
        .quad   0x1ad40f095e67ed3b
        .quad   0x5da8acdab8c63d5d
        .quad   0xc4b6664a3a70159f
        .quad   0x76194f0f0a904e14
        .quad   0xa5614c39a4096c13
        .quad   0x6cd0ff50979feced

        // 2^100 * 3 * G

        .quad   0xc0e067e78f4428ac
        .quad   0x14835ab0a61135e3
        .quad   0xf21d14f338062935
        .quad   0x6390a4c8df04849c
        .quad   0x7fecfabdb04ba18e
        .quad   0xd0fc7bfc3bddbcf7
        .quad   0xa41d486e057a131c
        .quad   0x641a4391f2223a61
        .quad   0xc5c6b95aa606a8db
        .quad   0x914b7f9eb06825f1
        .quad   0x2a731f6b44fc9eff
        .quad   0x30ddf38562705cfc

        // 2^100 * 4 * G

        .quad   0x4e3dcbdad1bff7f9
        .quad   0xc9118e8220645717
        .quad   0xbacccebc0f189d56
        .quad   0x1b4822e9d4467668
        .quad   0x33bef2bd68bcd52c
        .quad   0xc649dbb069482ef2
        .quad   0xb5b6ee0c41cb1aee
        .quad   0x5c294d270212a7e5
        .quad   0xab360a7f25563781
        .quad   0x2512228a480f7958
        .quad   0xc75d05276114b4e3
        .quad   0x222d9625d976fe2a

        // 2^100 * 5 * G

        .quad   0x1c717f85b372ace1
        .quad   0x81930e694638bf18
        .quad   0x239cad056bc08b58
        .quad   0x0b34271c87f8fff4
        .quad   0x0f94be7e0a344f85
        .quad   0xeb2faa8c87f22c38
        .quad   0x9ce1e75e4ee16f0f
        .quad   0x43e64e5418a08dea
        .quad   0x8155e2521a35ce63
        .quad   0xbe100d4df912028e
        .quad   0xbff80bf8a57ddcec
        .quad   0x57342dc96d6bc6e4

        // 2^100 * 6 * G

        .quad   0xefeef065c8ce5998
        .quad   0xbf029510b5cbeaa2
        .quad   0x8c64a10620b7c458
        .quad   0x35134fb231c24855
        .quad   0xf3c3bcb71e707bf6
        .quad   0x351d9b8c7291a762
        .quad   0x00502e6edad69a33
        .quad   0x522f521f1ec8807f
        .quad   0x272c1f46f9a3902b
        .quad   0xc91ba3b799657bcc
        .quad   0xae614b304f8a1c0e
        .quad   0x7afcaad70b99017b

        // 2^100 * 7 * G

        .quad   0xc25ded54a4b8be41
        .quad   0x902d13e11bb0e2dd
        .quad   0x41f43233cde82ab2
        .quad   0x1085faa5c3aae7cb
        .quad   0xa88141ecef842b6b
        .quad   0x55e7b14797abe6c5
        .quad   0x8c748f9703784ffe
        .quad   0x5b50a1f7afcd00b7
        .quad   0x9b840f66f1361315
        .quad   0x18462242701003e9
        .quad   0x65ed45fae4a25080
        .quad   0x0a2862393fda7320

        // 2^100 * 8 * G

        .quad   0x46ab13c8347cbc9d
        .quad   0x3849e8d499c12383
        .quad   0x4cea314087d64ac9
        .quad   0x1f354134b1a29ee7
        .quad   0x960e737b6ecb9d17
        .quad   0xfaf24948d67ceae1
        .quad   0x37e7a9b4d55e1b89
        .quad   0x5cb7173cb46c59eb
        .quad   0x4a89e68b82b7abf0
        .quad   0xf41cd9279ba6b7b9
        .quad   0x16e6c210e18d876f
        .quad   0x7cacdb0f7f1b09c6

        // 2^104 * 1 * G

        .quad   0x9062b2e0d91a78bc
        .quad   0x47c9889cc8509667
        .quad   0x9df54a66405070b8
        .quad   0x7369e6a92493a1bf
        .quad   0xe1014434dcc5caed
        .quad   0x47ed5d963c84fb33
        .quad   0x70019576ed86a0e7
        .quad   0x25b2697bd267f9e4
        .quad   0x9d673ffb13986864
        .quad   0x3ca5fbd9415dc7b8
        .quad   0xe04ecc3bdf273b5e
        .quad   0x1420683db54e4cd2

        // 2^104 * 2 * G

        .quad   0xb478bd1e249dd197
        .quad   0x620c35005e58c102
        .quad   0xfb02d32fccbaac5c
        .quad   0x60b63bebf508a72d
        .quad   0x34eebb6fc1cc5ad0
        .quad   0x6a1b0ce99646ac8b
        .quad   0xd3b0da49a66bde53
        .quad   0x31e83b4161d081c1
        .quad   0x97e8c7129e062b4f
        .quad   0x49e48f4f29320ad8
        .quad   0x5bece14b6f18683f
        .quad   0x55cf1eb62d550317

        // 2^104 * 3 * G

        .quad   0x5879101065c23d58
        .quad   0x8b9d086d5094819c
        .quad   0xe2402fa912c55fa7
        .quad   0x669a6564570891d4
        .quad   0x3076b5e37df58c52
        .quad   0xd73ab9dde799cc36
        .quad   0xbd831ce34913ee20
        .quad   0x1a56fbaa62ba0133
        .quad   0x943e6b505c9dc9ec
        .quad   0x302557bba77c371a
        .quad   0x9873ae5641347651
        .quad   0x13c4836799c58a5c

        // 2^104 * 4 * G

        .quad   0x423a5d465ab3e1b9
        .quad   0xfc13c187c7f13f61
        .quad   0x19f83664ecb5b9b6
        .quad   0x66f80c93a637b607
        .quad   0xc4dcfb6a5d8bd080
        .quad   0xdeebc4ec571a4842
        .quad   0xd4b2e883b8e55365
        .quad   0x50bdc87dc8e5b827
        .quad   0x606d37836edfe111
        .quad   0x32353e15f011abd9
        .quad   0x64b03ac325b73b96
        .quad   0x1dd56444725fd5ae

        // 2^104 * 5 * G

        .quad   0x8fa47ff83362127d
        .quad   0xbc9f6ac471cd7c15
        .quad   0x6e71454349220c8b
        .quad   0x0e645912219f732e
        .quad   0xc297e60008bac89a
        .quad   0x7d4cea11eae1c3e0
        .quad   0xf3e38be19fe7977c
        .quad   0x3a3a450f63a305cd
        .quad   0x078f2f31d8394627
        .quad   0x389d3183de94a510
        .quad   0xd1e36c6d17996f80
        .quad   0x318c8d9393a9a87b

        // 2^104 * 6 * G

        .quad   0xf2745d032afffe19
        .quad   0x0c9f3c497f24db66
        .quad   0xbc98d3e3ba8598ef
        .quad   0x224c7c679a1d5314
        .quad   0x5d669e29ab1dd398
        .quad   0xfc921658342d9e3b
        .quad   0x55851dfdf35973cd
        .quad   0x509a41c325950af6
        .quad   0xbdc06edca6f925e9
        .quad   0x793ef3f4641b1f33
        .quad   0x82ec12809d833e89
        .quad   0x05bff02328a11389

        // 2^104 * 7 * G

        .quad   0x3632137023cae00b
        .quad   0x544acf0ad1accf59
        .quad   0x96741049d21a1c88
        .quad   0x780b8cc3fa2a44a7
        .quad   0x6881a0dd0dc512e4
        .quad   0x4fe70dc844a5fafe
        .quad   0x1f748e6b8f4a5240
        .quad   0x576277cdee01a3ea
        .quad   0x1ef38abc234f305f
        .quad   0x9a577fbd1405de08
        .quad   0x5e82a51434e62a0d
        .quad   0x5ff418726271b7a1

        // 2^104 * 8 * G

        .quad   0x398e080c1789db9d
        .quad   0xa7602025f3e778f5
        .quad   0xfa98894c06bd035d
        .quad   0x106a03dc25a966be
        .quad   0xe5db47e813b69540
        .quad   0xf35d2a3b432610e1
        .quad   0xac1f26e938781276
        .quad   0x29d4db8ca0a0cb69
        .quad   0xd9ad0aaf333353d0
        .quad   0x38669da5acd309e5
        .quad   0x3c57658ac888f7f0
        .quad   0x4ab38a51052cbefa

        // 2^108 * 1 * G

        .quad   0xdfdacbee4324c0e9
        .quad   0x054442883f955bb7
        .quad   0xdef7aaa8ea31609f
        .quad   0x68aee70642287cff
        .quad   0xf68fe2e8809de054
        .quad   0xe3bc096a9c82bad1
        .quad   0x076353d40aadbf45
        .quad   0x7b9b1fb5dea1959e
        .quad   0xf01cc8f17471cc0c
        .quad   0x95242e37579082bb
        .quad   0x27776093d3e46b5f
        .quad   0x2d13d55a28bd85fb

        // 2^108 * 2 * G

        .quad   0xfac5d2065b35b8da
        .quad   0xa8da8a9a85624bb7
        .quad   0xccd2ca913d21cd0f
        .quad   0x6b8341ee8bf90d58
        .quad   0xbf019cce7aee7a52
        .quad   0xa8ded2b6e454ead3
        .quad   0x3c619f0b87a8bb19
        .quad   0x3619b5d7560916d8
        .quad   0x3579f26b0282c4b2
        .quad   0x64d592f24fafefae
        .quad   0xb7cded7b28c8c7c0
        .quad   0x6a927b6b7173a8d7

        // 2^108 * 3 * G

        .quad   0x1f6db24f986e4656
        .quad   0x1021c02ed1e9105b
        .quad   0xf8ff3fff2cc0a375
        .quad   0x1d2a6bf8c6c82592
        .quad   0x8d7040863ece88eb
        .quad   0xf0e307a980eec08c
        .quad   0xac2250610d788fda
        .quad   0x056d92a43a0d478d
        .quad   0x1b05a196fc3da5a1
        .quad   0x77d7a8c243b59ed0
        .quad   0x06da3d6297d17918
        .quad   0x66fbb494f12353f7

        // 2^108 * 4 * G

        .quad   0x751a50b9d85c0fb8
        .quad   0xd1afdc258bcf097b
        .quad   0x2f16a6a38309a969
        .quad   0x14ddff9ee5b00659
        .quad   0xd6d70996f12309d6
        .quad   0xdbfb2385e9c3d539
        .quad   0x46d602b0f7552411
        .quad   0x270a0b0557843e0c
        .quad   0x61ff0640a7862bcc
        .quad   0x81cac09a5f11abfe
        .quad   0x9047830455d12abb
        .quad   0x19a4bde1945ae873

        // 2^108 * 5 * G

        .quad   0x9b9f26f520a6200a
        .quad   0x64804443cf13eaf8
        .quad   0x8a63673f8631edd3
        .quad   0x72bbbce11ed39dc1
        .quad   0x40c709dec076c49f
        .quad   0x657bfaf27f3e53f6
        .quad   0x40662331eca042c4
        .quad   0x14b375487eb4df04
        .quad   0xae853c94ab66dc47
        .quad   0xeb62343edf762d6e
        .quad   0xf08e0e186fb2f7d1
        .quad   0x4f0b1c02700ab37a

        // 2^108 * 6 * G

        .quad   0xe1706787d81951fa
        .quad   0xa10a2c8eb290c77b
        .quad   0xe7382fa03ed66773
        .quad   0x0a4d84710bcc4b54
        .quad   0x79fd21ccc1b2e23f
        .quad   0x4ae7c281453df52a
        .quad   0xc8172ec9d151486b
        .quad   0x68abe9443e0a7534
        .quad   0xda12c6c407831dcb
        .quad   0x0da230d74d5c510d
        .quad   0x4ab1531e6bd404e1
        .quad   0x4106b166bcf440ef

        // 2^108 * 7 * G

        .quad   0x02e57a421cd23668
        .quad   0x4ad9fb5d0eaef6fd
        .quad   0x954e6727b1244480
        .quad   0x7f792f9d2699f331
        .quad   0xa485ccd539e4ecf2
        .quad   0x5aa3f3ad0555bab5
        .quad   0x145e3439937df82d
        .quad   0x1238b51e1214283f
        .quad   0x0b886b925fd4d924
        .quad   0x60906f7a3626a80d
        .quad   0xecd367b4b98abd12
        .quad   0x2876beb1def344cf

        // 2^108 * 8 * G

        .quad   0xdc84e93563144691
        .quad   0x632fe8a0d61f23f4
        .quad   0x4caa800612a9a8d5
        .quad   0x48f9dbfa0e9918d3
        .quad   0xd594b3333a8a85f8
        .quad   0x4ea37689e78d7d58
        .quad   0x73bf9f455e8e351f
        .quad   0x5507d7d2bc41ebb4
        .quad   0x1ceb2903299572fc
        .quad   0x7c8ccaa29502d0ee
        .quad   0x91bfa43411cce67b
        .quad   0x5784481964a831e7

        // 2^112 * 1 * G

        .quad   0xda7c2b256768d593
        .quad   0x98c1c0574422ca13
        .quad   0xf1a80bd5ca0ace1d
        .quad   0x29cdd1adc088a690
        .quad   0xd6cfd1ef5fddc09c
        .quad   0xe82b3efdf7575dce
        .quad   0x25d56b5d201634c2
        .quad   0x3041c6bb04ed2b9b
        .quad   0x0ff2f2f9d956e148
        .quad   0xade797759f356b2e
        .quad   0x1a4698bb5f6c025c
        .quad   0x104bbd6814049a7b

        // 2^112 * 2 * G

        .quad   0x51f0fd3168f1ed67
        .quad   0x2c811dcdd86f3bc2
        .quad   0x44dc5c4304d2f2de
        .quad   0x5be8cc57092a7149
        .quad   0xa95d9a5fd67ff163
        .quad   0xe92be69d4cc75681
        .quad   0xb7f8024cde20f257
        .quad   0x204f2a20fb072df5
        .quad   0xc8143b3d30ebb079
        .quad   0x7589155abd652e30
        .quad   0x653c3c318f6d5c31
        .quad   0x2570fb17c279161f

        // 2^112 * 3 * G

        .quad   0x3efa367f2cb61575
        .quad   0xf5f96f761cd6026c
        .quad   0xe8c7142a65b52562
        .quad   0x3dcb65ea53030acd
        .quad   0x192ea9550bb8245a
        .quad   0xc8e6fba88f9050d1
        .quad   0x7986ea2d88a4c935
        .quad   0x241c5f91de018668
        .quad   0x28d8172940de6caa
        .quad   0x8fbf2cf022d9733a
        .quad   0x16d7fcdd235b01d1
        .quad   0x08420edd5fcdf0e5

        // 2^112 * 4 * G

        .quad   0xcdff20ab8362fa4a
        .quad   0x57e118d4e21a3e6e
        .quad   0xe3179617fc39e62b
        .quad   0x0d9a53efbc1769fd
        .quad   0x0358c34e04f410ce
        .quad   0xb6135b5a276e0685
        .quad   0x5d9670c7ebb91521
        .quad   0x04d654f321db889c
        .quad   0x5e7dc116ddbdb5d5
        .quad   0x2954deb68da5dd2d
        .quad   0x1cb608173334a292
        .quad   0x4a7a4f2618991ad7

        // 2^112 * 5 * G

        .quad   0xf4a718025fb15f95
        .quad   0x3df65f346b5c1b8f
        .quad   0xcdfcf08500e01112
        .quad   0x11b50c4cddd31848
        .quad   0x24c3b291af372a4b
        .quad   0x93da8270718147f2
        .quad   0xdd84856486899ef2
        .quad   0x4a96314223e0ee33
        .quad   0xa6e8274408a4ffd6
        .quad   0x738e177e9c1576d9
        .quad   0x773348b63d02b3f2
        .quad   0x4f4bce4dce6bcc51

        // 2^112 * 6 * G

        .quad   0xa71fce5ae2242584
        .quad   0x26ea725692f58a9e
        .quad   0xd21a09d71cea3cf4
        .quad   0x73fcdd14b71c01e6
        .quad   0x30e2616ec49d0b6f
        .quad   0xe456718fcaec2317
        .quad   0x48eb409bf26b4fa6
        .quad   0x3042cee561595f37
        .quad   0x427e7079449bac41
        .quad   0x855ae36dbce2310a
        .quad   0x4cae76215f841a7c
        .quad   0x389e740c9a9ce1d6

        // 2^112 * 7 * G

        .quad   0x64fcb3ae34dcb9ce
        .quad   0x97500323e348d0ad
        .quad   0x45b3f07d62c6381b
        .quad   0x61545379465a6788
        .quad   0xc9bd78f6570eac28
        .quad   0xe55b0b3227919ce1
        .quad   0x65fc3eaba19b91ed
        .quad   0x25c425e5d6263690
        .quad   0x3f3e06a6f1d7de6e
        .quad   0x3ef976278e062308
        .quad   0x8c14f6264e8a6c77
        .quad   0x6539a08915484759

        // 2^112 * 8 * G

        .quad   0xe9d21f74c3d2f773
        .quad   0xc150544125c46845
        .quad   0x624e5ce8f9b99e33
        .quad   0x11c5e4aac5cd186c
        .quad   0xddc4dbd414bb4a19
        .quad   0x19b2bc3c98424f8e
        .quad   0x48a89fd736ca7169
        .quad   0x0f65320ef019bd90
        .quad   0xd486d1b1cafde0c6
        .quad   0x4f3fe6e3163b5181
        .quad   0x59a8af0dfaf2939a
        .quad   0x4cabc7bdec33072a

        // 2^116 * 1 * G

        .quad   0x16faa8fb532f7428
        .quad   0xdbd42ea046a4e272
        .quad   0x5337653b8b9ea480
        .quad   0x4065947223973f03
        .quad   0xf7c0a19c1a54a044
        .quad   0x4a1c5e2477bd9fbb
        .quad   0xa6e3ca115af22972
        .quad   0x1819bb953f2e9e0d
        .quad   0x498fbb795e042e84
        .quad   0x7d0dd89a7698b714
        .quad   0x8bfb0ba427fe6295
        .quad   0x36ba82e721200524

        // 2^116 * 2 * G

        .quad   0xd60ecbb74245ec41
        .quad   0xfd9be89e34348716
        .quad   0xc9240afee42284de
        .quad   0x4472f648d0531db4
        .quad   0xc8d69d0a57274ed5
        .quad   0x45ba803260804b17
        .quad   0xdf3cda102255dfac
        .quad   0x77d221232709b339
        .quad   0x498a6d7064ad94d8
        .quad   0xa5b5c8fd9af62263
        .quad   0x8ca8ed0545c141f4
        .quad   0x2c63bec3662d358c

        // 2^116 * 3 * G

        .quad   0x7fe60d8bea787955
        .quad   0xb9dc117eb5f401b7
        .quad   0x91c7c09a19355cce
        .quad   0x22692ef59442bedf
        .quad   0x9a518b3a8586f8bf
        .quad   0x9ee71af6cbb196f0
        .quad   0xaa0625e6a2385cf2
        .quad   0x1deb2176ddd7c8d1
        .quad   0x8563d19a2066cf6c
        .quad   0x401bfd8c4dcc7cd7
        .quad   0xd976a6becd0d8f62
        .quad   0x67cfd773a278b05e

        // 2^116 * 4 * G

        .quad   0x8dec31faef3ee475
        .quad   0x99dbff8a9e22fd92
        .quad   0x512d11594e26cab1
        .quad   0x0cde561eec4310b9
        .quad   0x2d5fa9855a4e586a
        .quad   0x65f8f7a449beab7e
        .quad   0xaa074dddf21d33d3
        .quad   0x185cba721bcb9dee
        .quad   0x93869da3f4e3cb41
        .quad   0xbf0392f540f7977e
        .quad   0x026204fcd0463b83
        .quad   0x3ec91a769eec6eed

        // 2^116 * 5 * G

        .quad   0x1e9df75bf78166ad
        .quad   0x4dfda838eb0cd7af
        .quad   0xba002ed8c1eaf988
        .quad   0x13fedb3e11f33cfc
        .quad   0x0fad2fb7b0a3402f
        .quad   0x46615ecbfb69f4a8
        .quad   0xf745bcc8c5f8eaa6
        .quad   0x7a5fa8794a94e896
        .quad   0x52958faa13cd67a1
        .quad   0x965ee0818bdbb517
        .quad   0x16e58daa2e8845b3
        .quad   0x357d397d5499da8f

        // 2^116 * 6 * G

        .quad   0x1ebfa05fb0bace6c
        .quad   0xc934620c1caf9a1e
        .quad   0xcc771cc41d82b61a
        .quad   0x2d94a16aa5f74fec
        .quad   0x481dacb4194bfbf8
        .quad   0x4d77e3f1bae58299
        .quad   0x1ef4612e7d1372a0
        .quad   0x3a8d867e70ff69e1
        .quad   0x6f58cd5d55aff958
        .quad   0xba3eaa5c75567721
        .quad   0x75c123999165227d
        .quad   0x69be1343c2f2b35e

        // 2^116 * 7 * G

        .quad   0x0e091d5ee197c92a
        .quad   0x4f51019f2945119f
        .quad   0x143679b9f034e99c
        .quad   0x7d88112e4d24c696
        .quad   0x82bbbdac684b8de3
        .quad   0xa2f4c7d03fca0718
        .quad   0x337f92fbe096aaa8
        .quad   0x200d4d8c63587376
        .quad   0x208aed4b4893b32b
        .quad   0x3efbf23ebe59b964
        .quad   0xd762deb0dba5e507
        .quad   0x69607bd681bd9d94

        // 2^116 * 8 * G

        .quad   0xf6be021068de1ce1
        .quad   0xe8d518e70edcbc1f
        .quad   0xe3effdd01b5505a5
        .quad   0x35f63353d3ec3fd0
        .quad   0x3b7f3bd49323a902
        .quad   0x7c21b5566b2c6e53
        .quad   0xe5ba8ff53a7852a7
        .quad   0x28bc77a5838ece00
        .quad   0x63ba78a8e25d8036
        .quad   0x63651e0094333490
        .quad   0x48d82f20288ce532
        .quad   0x3a31abfa36b57524

        // 2^120 * 1 * G

        .quad   0x239e9624089c0a2e
        .quad   0xc748c4c03afe4738
        .quad   0x17dbed2a764fa12a
        .quad   0x639b93f0321c8582
        .quad   0xc08f788f3f78d289
        .quad   0xfe30a72ca1404d9f
        .quad   0xf2778bfccf65cc9d
        .quad   0x7ee498165acb2021
        .quad   0x7bd508e39111a1c3
        .quad   0x2b2b90d480907489
        .quad   0xe7d2aec2ae72fd19
        .quad   0x0edf493c85b602a6

        // 2^120 * 2 * G

        .quad   0xaecc8158599b5a68
        .quad   0xea574f0febade20e
        .quad   0x4fe41d7422b67f07
        .quad   0x403b92e3019d4fb4
        .quad   0x6767c4d284764113
        .quad   0xa090403ff7f5f835
        .quad   0x1c8fcffacae6bede
        .quad   0x04c00c54d1dfa369
        .quad   0x4dc22f818b465cf8
        .quad   0x71a0f35a1480eff8
        .quad   0xaee8bfad04c7d657
        .quad   0x355bb12ab26176f4

        // 2^120 * 3 * G

        .quad   0xa71e64cc7493bbf4
        .quad   0xe5bd84d9eca3b0c3
        .quad   0x0a6bc50cfa05e785
        .quad   0x0f9b8132182ec312
        .quad   0xa301dac75a8c7318
        .quad   0xed90039db3ceaa11
        .quad   0x6f077cbf3bae3f2d
        .quad   0x7518eaf8e052ad8e
        .quad   0xa48859c41b7f6c32
        .quad   0x0f2d60bcf4383298
        .quad   0x1815a929c9b1d1d9
        .quad   0x47c3871bbb1755c4

        // 2^120 * 4 * G

        .quad   0x5144539771ec4f48
        .quad   0xf805b17dc98c5d6e
        .quad   0xf762c11a47c3c66b
        .quad   0x00b89b85764699dc
        .quad   0xfbe65d50c85066b0
        .quad   0x62ecc4b0b3a299b0
        .quad   0xe53754ea441ae8e0
        .quad   0x08fea02ce8d48d5f
        .quad   0x824ddd7668deead0
        .quad   0xc86445204b685d23
        .quad   0xb514cfcd5d89d665
        .quad   0x473829a74f75d537

        // 2^120 * 5 * G

        .quad   0x82d2da754679c418
        .quad   0xe63bd7d8b2618df0
        .quad   0x355eef24ac47eb0a
        .quad   0x2078684c4833c6b4
        .quad   0x23d9533aad3902c9
        .quad   0x64c2ddceef03588f
        .quad   0x15257390cfe12fb4
        .quad   0x6c668b4d44e4d390
        .quad   0x3b48cf217a78820c
        .quad   0xf76a0ab281273e97
        .quad   0xa96c65a78c8eed7b
        .quad   0x7411a6054f8a433f

        // 2^120 * 6 * G

        .quad   0x4d659d32b99dc86d
        .quad   0x044cdc75603af115
        .quad   0xb34c712cdcc2e488
        .quad   0x7c136574fb8134ff
        .quad   0x579ae53d18b175b4
        .quad   0x68713159f392a102
        .quad   0x8455ecba1eef35f5
        .quad   0x1ec9a872458c398f
        .quad   0xb8e6a4d400a2509b
        .quad   0x9b81d7020bc882b4
        .quad   0x57e7cc9bf1957561
        .quad   0x3add88a5c7cd6460

        // 2^120 * 7 * G

        .quad   0xab895770b635dcf2
        .quad   0x02dfef6cf66c1fbc
        .quad   0x85530268beb6d187
        .quad   0x249929fccc879e74
        .quad   0x85c298d459393046
        .quad   0x8f7e35985ff659ec
        .quad   0x1d2ca22af2f66e3a
        .quad   0x61ba1131a406a720
        .quad   0xa3d0a0f116959029
        .quad   0x023b6b6cba7ebd89
        .quad   0x7bf15a3e26783307
        .quad   0x5620310cbbd8ece7

        // 2^120 * 8 * G

        .quad   0x528993434934d643
        .quad   0xb9dbf806a51222f5
        .quad   0x8f6d878fc3f41c22
        .quad   0x37676a2a4d9d9730
        .quad   0x6646b5f477e285d6
        .quad   0x40e8ff676c8f6193
        .quad   0xa6ec7311abb594dd
        .quad   0x7ec846f3658cec4d
        .quad   0x9b5e8f3f1da22ec7
        .quad   0x130f1d776c01cd13
        .quad   0x214c8fcfa2989fb8
        .quad   0x6daaf723399b9dd5

        // 2^124 * 1 * G

        .quad   0x591e4a5610628564
        .quad   0x2a4bb87ca8b4df34
        .quad   0xde2a2572e7a38e43
        .quad   0x3cbdabd9fee5046e
        .quad   0x81aebbdd2cd13070
        .quad   0x962e4325f85a0e9e
        .quad   0xde9391aacadffecb
        .quad   0x53177fda52c230e6
        .quad   0xa7bc970650b9de79
        .quad   0x3d12a7fbc301b59b
        .quad   0x02652e68d36ae38c
        .quad   0x79d739835a6199dc

        // 2^124 * 2 * G

        .quad   0xd9354df64131c1bd
        .quad   0x758094a186ec5822
        .quad   0x4464ee12e459f3c2
        .quad   0x6c11fce4cb133282
        .quad   0x21c9d9920d591737
        .quad   0x9bea41d2e9b46cd6
        .quad   0xe20e84200d89bfca
        .quad   0x79d99f946eae5ff8
        .quad   0xf17b483568673205
        .quad   0x387deae83caad96c
        .quad   0x61b471fd56ffe386
        .quad   0x31741195b745a599

        // 2^124 * 3 * G

        .quad   0xe8d10190b77a360b
        .quad   0x99b983209995e702
        .quad   0xbd4fdff8fa0247aa
        .quad   0x2772e344e0d36a87
        .quad   0x17f8ba683b02a047
        .quad   0x50212096feefb6c8
        .quad   0x70139be21556cbe2
        .quad   0x203e44a11d98915b
        .quad   0xd6863eba37b9e39f
        .quad   0x105bc169723b5a23
        .quad   0x104f6459a65c0762
        .quad   0x567951295b4d38d4

        // 2^124 * 4 * G

        .quad   0x535fd60613037524
        .quad   0xe210adf6b0fbc26a
        .quad   0xac8d0a9b23e990ae
        .quad   0x47204d08d72fdbf9
        .quad   0x07242eb30d4b497f
        .quad   0x1ef96306b9bccc87
        .quad   0x37950934d8116f45
        .quad   0x05468d6201405b04
        .quad   0x00f565a9f93267de
        .quad   0xcecfd78dc0d58e8a
        .quad   0xa215e2dcf318e28e
        .quad   0x4599ee919b633352

        // 2^124 * 5 * G

        .quad   0xd3c220ca70e0e76b
        .quad   0xb12bea58ea9f3094
        .quad   0x294ddec8c3271282
        .quad   0x0c3539e1a1d1d028
        .quad   0xac746d6b861ae579
        .quad   0x31ab0650f6aea9dc
        .quad   0x241d661140256d4c
        .quad   0x2f485e853d21a5de
        .quad   0x329744839c0833f3
        .quad   0x6fe6257fd2abc484
        .quad   0x5327d1814b358817
        .quad   0x65712585893fe9bc

        // 2^124 * 6 * G

        .quad   0x9c102fb732a61161
        .quad   0xe48e10dd34d520a8
        .quad   0x365c63546f9a9176
        .quad   0x32f6fe4c046f6006
        .quad   0x81c29f1bd708ee3f
        .quad   0xddcb5a05ae6407d0
        .quad   0x97aec1d7d2a3eba7
        .quad   0x1590521a91d50831
        .quad   0x40a3a11ec7910acc
        .quad   0x9013dff8f16d27ae
        .quad   0x1a9720d8abb195d4
        .quad   0x1bb9fe452ea98463

        // 2^124 * 7 * G

        .quad   0xe9d1d950b3d54f9e
        .quad   0x2d5f9cbee00d33c1
        .quad   0x51c2c656a04fc6ac
        .quad   0x65c091ee3c1cbcc9
        .quad   0xcf5e6c95cc36747c
        .quad   0x294201536b0bc30d
        .quad   0x453ac67cee797af0
        .quad   0x5eae6ab32a8bb3c9
        .quad   0x7083661114f118ea
        .quad   0x2b37b87b94349cad
        .quad   0x7273f51cb4e99f40
        .quad   0x78a2a95823d75698

        // 2^124 * 8 * G

        .quad   0xa2b072e95c8c2ace
        .quad   0x69cffc96651e9c4b
        .quad   0x44328ef842e7b42b
        .quad   0x5dd996c122aadeb3
        .quad   0xb4f23c425ef83207
        .quad   0xabf894d3c9a934b5
        .quad   0xd0708c1339fd87f7
        .quad   0x1876789117166130
        .quad   0x925b5ef0670c507c
        .quad   0x819bc842b93c33bf
        .quad   0x10792e9a70dd003f
        .quad   0x59ad4b7a6e28dc74

        // 2^128 * 1 * G

        .quad   0x5f3a7562eb3dbe47
        .quad   0xf7ea38548ebda0b8
        .quad   0x00c3e53145747299
        .quad   0x1304e9e71627d551
        .quad   0x583b04bfacad8ea2
        .quad   0x29b743e8148be884
        .quad   0x2b1e583b0810c5db
        .quad   0x2b5449e58eb3bbaa
        .quad   0x789814d26adc9cfe
        .quad   0x3c1bab3f8b48dd0b
        .quad   0xda0fe1fff979c60a
        .quad   0x4468de2d7c2dd693

        // 2^128 * 2 * G

        .quad   0x51bb355e9419469e
        .quad   0x33e6dc4c23ddc754
        .quad   0x93a5b6d6447f9962
        .quad   0x6cce7c6ffb44bd63
        .quad   0x4b9ad8c6f86307ce
        .quad   0x21113531435d0c28
        .quad   0xd4a866c5657a772c
        .quad   0x5da6427e63247352
        .quad   0x1a94c688deac22ca
        .quad   0xb9066ef7bbae1ff8
        .quad   0x88ad8c388d59580f
        .quad   0x58f29abfe79f2ca8

        // 2^128 * 3 * G

        .quad   0xe90ecfab8de73e68
        .quad   0x54036f9f377e76a5
        .quad   0xf0495b0bbe015982
        .quad   0x577629c4a7f41e36
        .quad   0x4b5a64bf710ecdf6
        .quad   0xb14ce538462c293c
        .quad   0x3643d056d50b3ab9
        .quad   0x6af93724185b4870
        .quad   0x3220024509c6a888
        .quad   0xd2e036134b558973
        .quad   0x83e236233c33289f
        .quad   0x701f25bb0caec18f

        // 2^128 * 4 * G

        .quad   0xc3a8b0f8e4616ced
        .quad   0xf700660e9e25a87d
        .quad   0x61e3061ff4bca59c
        .quad   0x2e0c92bfbdc40be9
        .quad   0x9d18f6d97cbec113
        .quad   0x844a06e674bfdbe4
        .quad   0x20f5b522ac4e60d6
        .quad   0x720a5bc050955e51
        .quad   0x0c3f09439b805a35
        .quad   0xe84e8b376242abfc
        .quad   0x691417f35c229346
        .quad   0x0e9b9cbb144ef0ec

        // 2^128 * 5 * G

        .quad   0xfbbad48ffb5720ad
        .quad   0xee81916bdbf90d0e
        .quad   0xd4813152635543bf
        .quad   0x221104eb3f337bd8
        .quad   0x8dee9bd55db1beee
        .quad   0xc9c3ab370a723fb9
        .quad   0x44a8f1bf1c68d791
        .quad   0x366d44191cfd3cde
        .quad   0x9e3c1743f2bc8c14
        .quad   0x2eda26fcb5856c3b
        .quad   0xccb82f0e68a7fb97
        .quad   0x4167a4e6bc593244

        // 2^128 * 6 * G

        .quad   0x643b9d2876f62700
        .quad   0x5d1d9d400e7668eb
        .quad   0x1b4b430321fc0684
        .quad   0x7938bb7e2255246a
        .quad   0xc2be2665f8ce8fee
        .quad   0xe967ff14e880d62c
        .quad   0xf12e6e7e2f364eee
        .quad   0x34b33370cb7ed2f6
        .quad   0xcdc591ee8681d6cc
        .quad   0xce02109ced85a753
        .quad   0xed7485c158808883
        .quad   0x1176fc6e2dfe65e4

        // 2^128 * 7 * G

        .quad   0xb4af6cd05b9c619b
        .quad   0x2ddfc9f4b2a58480
        .quad   0x3d4fa502ebe94dc4
        .quad   0x08fc3a4c677d5f34
        .quad   0xdb90e28949770eb8
        .quad   0x98fbcc2aacf440a3
        .quad   0x21354ffeded7879b
        .quad   0x1f6a3e54f26906b6
        .quad   0x60a4c199d30734ea
        .quad   0x40c085b631165cd6
        .quad   0xe2333e23f7598295
        .quad   0x4f2fad0116b900d1

        // 2^128 * 8 * G

        .quad   0x44beb24194ae4e54
        .quad   0x5f541c511857ef6c
        .quad   0xa61e6b2d368d0498
        .quad   0x445484a4972ef7ab
        .quad   0x962cd91db73bb638
        .quad   0xe60577aafc129c08
        .quad   0x6f619b39f3b61689
        .quad   0x3451995f2944ee81
        .quad   0x9152fcd09fea7d7c
        .quad   0x4a816c94b0935cf6
        .quad   0x258e9aaa47285c40
        .quad   0x10b89ca6042893b7

        // 2^132 * 1 * G

        .quad   0x9b2a426e3b646025
        .quad   0x32127190385ce4cf
        .quad   0xa25cffc2dd6dea45
        .quad   0x06409010bea8de75
        .quad   0xd67cded679d34aa0
        .quad   0xcc0b9ec0cc4db39f
        .quad   0xa535a456e35d190f
        .quad   0x2e05d9eaf61f6fef
        .quad   0xc447901ad61beb59
        .quad   0x661f19bce5dc880a
        .quad   0x24685482b7ca6827
        .quad   0x293c778cefe07f26

        // 2^132 * 2 * G

        .quad   0x86809e7007069096
        .quad   0xaad75b15e4e50189
        .quad   0x07f35715a21a0147
        .quad   0x0487f3f112815d5e
        .quad   0x16c795d6a11ff200
        .quad   0xcb70d0e2b15815c9
        .quad   0x89f293209b5395b5
        .quad   0x50b8c2d031e47b4f
        .quad   0x48350c08068a4962
        .quad   0x6ffdd05351092c9a
        .quad   0x17af4f4aaf6fc8dd
        .quad   0x4b0553b53cdba58b

        // 2^132 * 3 * G

        .quad   0x9c65fcbe1b32ff79
        .quad   0xeb75ea9f03b50f9b
        .quad   0xfced2a6c6c07e606
        .quad   0x35106cd551717908
        .quad   0xbf05211b27c152d4
        .quad   0x5ec26849bd1af639
        .quad   0x5e0b2caa8e6fab98
        .quad   0x054c8bdd50bd0840
        .quad   0x38a0b12f1dcf073d
        .quad   0x4b60a8a3b7f6a276
        .quad   0xfed5ac25d3404f9a
        .quad   0x72e82d5e5505c229

        // 2^132 * 4 * G

        .quad   0x6b0b697ff0d844c8
        .quad   0xbb12f85cd979cb49
        .quad   0xd2a541c6c1da0f1f
        .quad   0x7b7c242958ce7211
        .quad   0x00d9cdfd69771d02
        .quad   0x410276cd6cfbf17e
        .quad   0x4c45306c1cb12ec7
        .quad   0x2857bf1627500861
        .quad   0x9f21903f0101689e
        .quad   0xd779dfd3bf861005
        .quad   0xa122ee5f3deb0f1b
        .quad   0x510df84b485a00d4

        // 2^132 * 5 * G

        .quad   0xa54133bb9277a1fa
        .quad   0x74ec3b6263991237
        .quad   0x1a3c54dc35d2f15a
        .quad   0x2d347144e482ba3a
        .quad   0x24b3c887c70ac15e
        .quad   0xb0f3a557fb81b732
        .quad   0x9b2cde2fe578cc1b
        .quad   0x4cf7ed0703b54f8e
        .quad   0x6bd47c6598fbee0f
        .quad   0x9e4733e2ab55be2d
        .quad   0x1093f624127610c5
        .quad   0x4e05e26ad0a1eaa4

        // 2^132 * 6 * G

        .quad   0xda9b6b624b531f20
        .quad   0x429a760e77509abb
        .quad   0xdbe9f522e823cb80
        .quad   0x618f1856880c8f82
        .quad   0x1833c773e18fe6c0
        .quad   0xe3c4711ad3c87265
        .quad   0x3bfd3c4f0116b283
        .quad   0x1955875eb4cd4db8
        .quad   0x6da6de8f0e399799
        .quad   0x7ad61aa440fda178
        .quad   0xb32cd8105e3563dd
        .quad   0x15f6beae2ae340ae

        // 2^132 * 7 * G

        .quad   0x862bcb0c31ec3a62
        .quad   0x810e2b451138f3c2
        .quad   0x788ec4b839dac2a4
        .quad   0x28f76867ae2a9281
        .quad   0xba9a0f7b9245e215
        .quad   0xf368612dd98c0dbb
        .quad   0x2e84e4cbf220b020
        .quad   0x6ba92fe962d90eda
        .quad   0x3e4df9655884e2aa
        .quad   0xbd62fbdbdbd465a5
        .quad   0xd7596caa0de9e524
        .quad   0x6e8042ccb2b1b3d7

        // 2^132 * 8 * G

        .quad   0xf10d3c29ce28ca6e
        .quad   0xbad34540fcb6093d
        .quad   0xe7426ed7a2ea2d3f
        .quad   0x08af9d4e4ff298b9
        .quad   0x1530653616521f7e
        .quad   0x660d06b896203dba
        .quad   0x2d3989bc545f0879
        .quad   0x4b5303af78ebd7b0
        .quad   0x72f8a6c3bebcbde8
        .quad   0x4f0fca4adc3a8e89
        .quad   0x6fa9d4e8c7bfdf7a
        .quad   0x0dcf2d679b624eb7

        // 2^136 * 1 * G

        .quad   0x3d5947499718289c
        .quad   0x12ebf8c524533f26
        .quad   0x0262bfcb14c3ef15
        .quad   0x20b878d577b7518e
        .quad   0x753941be5a45f06e
        .quad   0xd07caeed6d9c5f65
        .quad   0x11776b9c72ff51b6
        .quad   0x17d2d1d9ef0d4da9
        .quad   0x27f2af18073f3e6a
        .quad   0xfd3fe519d7521069
        .quad   0x22e3b72c3ca60022
        .quad   0x72214f63cc65c6a7

        // 2^136 * 2 * G

        .quad   0xb4e37f405307a693
        .quad   0xaba714d72f336795
        .quad   0xd6fbd0a773761099
        .quad   0x5fdf48c58171cbc9
        .quad   0x1d9db7b9f43b29c9
        .quad   0xd605824a4f518f75
        .quad   0xf2c072bd312f9dc4
        .quad   0x1f24ac855a1545b0
        .quad   0x24d608328e9505aa
        .quad   0x4748c1d10c1420ee
        .quad   0xc7ffe45c06fb25a2
        .quad   0x00ba739e2ae395e6

        // 2^136 * 3 * G

        .quad   0x592e98de5c8790d6
        .quad   0xe5bfb7d345c2a2df
        .quad   0x115a3b60f9b49922
        .quad   0x03283a3e67ad78f3
        .quad   0xae4426f5ea88bb26
        .quad   0x360679d984973bfb
        .quad   0x5c9f030c26694e50
        .quad   0x72297de7d518d226
        .quad   0x48241dc7be0cb939
        .quad   0x32f19b4d8b633080
        .quad   0xd3dfc90d02289308
        .quad   0x05e1296846271945

        // 2^136 * 4 * G

        .quad   0xba82eeb32d9c495a
        .quad   0xceefc8fcf12bb97c
        .quad   0xb02dabae93b5d1e0
        .quad   0x39c00c9c13698d9b
        .quad   0xadbfbbc8242c4550
        .quad   0xbcc80cecd03081d9
        .quad   0x843566a6f5c8df92
        .quad   0x78cf25d38258ce4c
        .quad   0x15ae6b8e31489d68
        .quad   0xaa851cab9c2bf087
        .quad   0xc9a75a97f04efa05
        .quad   0x006b52076b3ff832

        // 2^136 * 5 * G

        .quad   0x29e0cfe19d95781c
        .quad   0xb681df18966310e2
        .quad   0x57df39d370516b39
        .quad   0x4d57e3443bc76122
        .quad   0xf5cb7e16b9ce082d
        .quad   0x3407f14c417abc29
        .quad   0xd4b36bce2bf4a7ab
        .quad   0x7de2e9561a9f75ce
        .quad   0xde70d4f4b6a55ecb
        .quad   0x4801527f5d85db99
        .quad   0xdbc9c440d3ee9a81
        .quad   0x6b2a90af1a6029ed

        // 2^136 * 6 * G

        .quad   0x6923f4fc9ae61e97
        .quad   0x5735281de03f5fd1
        .quad   0xa764ae43e6edd12d
        .quad   0x5fd8f4e9d12d3e4a
        .quad   0x77ebf3245bb2d80a
        .quad   0xd8301b472fb9079b
        .quad   0xc647e6f24cee7333
        .quad   0x465812c8276c2109
        .quad   0x4d43beb22a1062d9
        .quad   0x7065fb753831dc16
        .quad   0x180d4a7bde2968d7
        .quad   0x05b32c2b1cb16790

        // 2^136 * 7 * G

        .quad   0xc8c05eccd24da8fd
        .quad   0xa1cf1aac05dfef83
        .quad   0xdbbeeff27df9cd61
        .quad   0x3b5556a37b471e99
        .quad   0xf7fca42c7ad58195
        .quad   0x3214286e4333f3cc
        .quad   0xb6c29d0d340b979d
        .quad   0x31771a48567307e1
        .quad   0x32b0c524e14dd482
        .quad   0xedb351541a2ba4b6
        .quad   0xa3d16048282b5af3
        .quad   0x4fc079d27a7336eb

        // 2^136 * 8 * G

        .quad   0x51c938b089bf2f7f
        .quad   0x2497bd6502dfe9a7
        .quad   0xffffc09c7880e453
        .quad   0x124567cecaf98e92
        .quad   0xdc348b440c86c50d
        .quad   0x1337cbc9cc94e651
        .quad   0x6422f74d643e3cb9
        .quad   0x241170c2bae3cd08
        .quad   0x3ff9ab860ac473b4
        .quad   0xf0911dee0113e435
        .quad   0x4ae75060ebc6c4af
        .quad   0x3f8612966c87000d

        // 2^140 * 1 * G

        .quad   0x0c9c5303f7957be4
        .quad   0xa3c31a20e085c145
        .quad   0xb0721d71d0850050
        .quad   0x0aba390eab0bf2da
        .quad   0x529fdffe638c7bf3
        .quad   0xdf2b9e60388b4995
        .quad   0xe027b34f1bad0249
        .quad   0x7bc92fc9b9fa74ed
        .quad   0x9f97ef2e801ad9f9
        .quad   0x83697d5479afda3a
        .quad   0xe906b3ffbd596b50
        .quad   0x02672b37dd3fb8e0

        // 2^140 * 2 * G

        .quad   0x48b2ca8b260885e4
        .quad   0xa4286bec82b34c1c
        .quad   0x937e1a2617f58f74
        .quad   0x741d1fcbab2ca2a5
        .quad   0xee9ba729398ca7f5
        .quad   0xeb9ca6257a4849db
        .quad   0x29eb29ce7ec544e1
        .quad   0x232ca21ef736e2c8
        .quad   0xbf61423d253fcb17
        .quad   0x08803ceafa39eb14
        .quad   0xf18602df9851c7af
        .quad   0x0400f3a049e3414b

        // 2^140 * 3 * G

        .quad   0xabce0476ba61c55b
        .quad   0x36a3d6d7c4d39716
        .quad   0x6eb259d5e8d82d09
        .quad   0x0c9176e984d756fb
        .quad   0x2efba412a06e7b06
        .quad   0x146785452c8d2560
        .quad   0xdf9713ebd67a91c7
        .quad   0x32830ac7157eadf3
        .quad   0x0e782a7ab73769e8
        .quad   0x04a05d7875b18e2c
        .quad   0x29525226ebcceae1
        .quad   0x0d794f8383eba820

        // 2^140 * 4 * G

        .quad   0xff35f5cb9e1516f4
        .quad   0xee805bcf648aae45
        .quad   0xf0d73c2bb93a9ef3
        .quad   0x097b0bf22092a6c2
        .quad   0x7be44ce7a7a2e1ac
        .quad   0x411fd93efad1b8b7
        .quad   0x1734a1d70d5f7c9b
        .quad   0x0d6592233127db16
        .quad   0xc48bab1521a9d733
        .quad   0xa6c2eaead61abb25
        .quad   0x625c6c1cc6cb4305
        .quad   0x7fc90fea93eb3a67

        // 2^140 * 5 * G

        .quad   0x0408f1fe1f5c5926
        .quad   0x1a8f2f5e3b258bf4
        .quad   0x40a951a2fdc71669
        .quad   0x6598ee93c98b577e
        .quad   0xc527deb59c7cb23d
        .quad   0x955391695328404e
        .quad   0xd64392817ccf2c7a
        .quad   0x6ce97dabf7d8fa11
        .quad   0x25b5a8e50ef7c48f
        .quad   0xeb6034116f2ce532
        .quad   0xc5e75173e53de537
        .quad   0x73119fa08c12bb03

        // 2^140 * 6 * G

        .quad   0xed30129453f1a4cb
        .quad   0xbce621c9c8f53787
        .quad   0xfacb2b1338bee7b9
        .quad   0x3025798a9ea8428c
        .quad   0x7845b94d21f4774d
        .quad   0xbf62f16c7897b727
        .quad   0x671857c03c56522b
        .quad   0x3cd6a85295621212
        .quad   0x3fecde923aeca999
        .quad   0xbdaa5b0062e8c12f
        .quad   0x67b99dfc96988ade
        .quad   0x3f52c02852661036

        // 2^140 * 7 * G

        .quad   0xffeaa48e2a1351c6
        .quad   0x28624754fa7f53d7
        .quad   0x0b5ba9e57582ddf1
        .quad   0x60c0104ba696ac59
        .quad   0x9258bf99eec416c6
        .quad   0xac8a5017a9d2f671
        .quad   0x629549ab16dea4ab
        .quad   0x05d0e85c99091569
        .quad   0x051de020de9cbe97
        .quad   0xfa07fc56b50bcf74
        .quad   0x378cec9f0f11df65
        .quad   0x36853c69ab96de4d

        // 2^140 * 8 * G

        .quad   0x36d9b8de78f39b2d
        .quad   0x7f42ed71a847b9ec
        .quad   0x241cd1d679bd3fde
        .quad   0x6a704fec92fbce6b
        .quad   0x4433c0b0fac5e7be
        .quad   0x724bae854c08dcbe
        .quad   0xf1f24cc446978f9b
        .quad   0x4a0aff6d62825fc8
        .quad   0xe917fb9e61095301
        .quad   0xc102df9402a092f8
        .quad   0xbf09e2f5fa66190b
        .quad   0x681109bee0dcfe37

        // 2^144 * 1 * G

        .quad   0x559a0cc9782a0dde
        .quad   0x551dcdb2ea718385
        .quad   0x7f62865b31ef238c
        .quad   0x504aa7767973613d
        .quad   0x9c18fcfa36048d13
        .quad   0x29159db373899ddd
        .quad   0xdc9f350b9f92d0aa
        .quad   0x26f57eee878a19d4
        .quad   0x0cab2cd55687efb1
        .quad   0x5180d162247af17b
        .quad   0x85c15a344f5a2467
        .quad   0x4041943d9dba3069

        // 2^144 * 2 * G

        .quad   0xc3c0eeba43ebcc96
        .quad   0x8d749c9c26ea9caf
        .quad   0xd9fa95ee1c77ccc6
        .quad   0x1420a1d97684340f
        .quad   0x4b217743a26caadd
        .quad   0x47a6b424648ab7ce
        .quad   0xcb1d4f7a03fbc9e3
        .quad   0x12d931429800d019
        .quad   0x00c67799d337594f
        .quad   0x5e3c5140b23aa47b
        .quad   0x44182854e35ff395
        .quad   0x1b4f92314359a012

        // 2^144 * 3 * G

        .quad   0x3e5c109d89150951
        .quad   0x39cefa912de9696a
        .quad   0x20eae43f975f3020
        .quad   0x239b572a7f132dae
        .quad   0x33cf3030a49866b1
        .quad   0x251f73d2215f4859
        .quad   0xab82aa4051def4f6
        .quad   0x5ff191d56f9a23f6
        .quad   0x819ed433ac2d9068
        .quad   0x2883ab795fc98523
        .quad   0xef4572805593eb3d
        .quad   0x020c526a758f36cb

        // 2^144 * 4 * G

        .quad   0x779834f89ed8dbbc
        .quad   0xc8f2aaf9dc7ca46c
        .quad   0xa9524cdca3e1b074
        .quad   0x02aacc4615313877
        .quad   0xe931ef59f042cc89
        .quad   0x2c589c9d8e124bb6
        .quad   0xadc8e18aaec75997
        .quad   0x452cfe0a5602c50c
        .quad   0x86a0f7a0647877df
        .quad   0xbbc464270e607c9f
        .quad   0xab17ea25f1fb11c9
        .quad   0x4cfb7d7b304b877b

        // 2^144 * 5 * G

        .quad   0x72b43d6cb89b75fe
        .quad   0x54c694d99c6adc80
        .quad   0xb8c3aa373ee34c9f
        .quad   0x14b4622b39075364
        .quad   0xe28699c29789ef12
        .quad   0x2b6ecd71df57190d
        .quad   0xc343c857ecc970d0
        .quad   0x5b1d4cbc434d3ac5
        .quad   0xb6fb2615cc0a9f26
        .quad   0x3a4f0e2bb88dcce5
        .quad   0x1301498b3369a705
        .quad   0x2f98f71258592dd1

        // 2^144 * 6 * G

        .quad   0x0c94a74cb50f9e56
        .quad   0x5b1ff4a98e8e1320
        .quad   0x9a2acc2182300f67
        .quad   0x3a6ae249d806aaf9
        .quad   0x2e12ae444f54a701
        .quad   0xfcfe3ef0a9cbd7de
        .quad   0xcebf890d75835de0
        .quad   0x1d8062e9e7614554
        .quad   0x657ada85a9907c5a
        .quad   0x1a0ea8b591b90f62
        .quad   0x8d0e1dfbdf34b4e9
        .quad   0x298b8ce8aef25ff3

        // 2^144 * 7 * G

        .quad   0x2a927953eff70cb2
        .quad   0x4b89c92a79157076
        .quad   0x9418457a30a7cf6a
        .quad   0x34b8a8404d5ce485
        .quad   0x837a72ea0a2165de
        .quad   0x3fab07b40bcf79f6
        .quad   0x521636c77738ae70
        .quad   0x6ba6271803a7d7dc
        .quad   0xc26eecb583693335
        .quad   0xd5a813df63b5fefd
        .quad   0xa293aa9aa4b22573
        .quad   0x71d62bdd465e1c6a

        // 2^144 * 8 * G

        .quad   0x6533cc28d378df80
        .quad   0xf6db43790a0fa4b4
        .quad   0xe3645ff9f701da5a
        .quad   0x74d5f317f3172ba4
        .quad   0xcd2db5dab1f75ef5
        .quad   0xd77f95cf16b065f5
        .quad   0x14571fea3f49f085
        .quad   0x1c333621262b2b3d
        .quad   0xa86fe55467d9ca81
        .quad   0x398b7c752b298c37
        .quad   0xda6d0892e3ac623b
        .quad   0x4aebcc4547e9d98c

        // 2^148 * 1 * G

        .quad   0x53175a7205d21a77
        .quad   0xb0c04422d3b934d4
        .quad   0xadd9f24bdd5deadc
        .quad   0x074f46e69f10ff8c
        .quad   0x0de9b204a059a445
        .quad   0xe15cb4aa4b17ad0f
        .quad   0xe1bbec521f79c557
        .quad   0x2633f1b9d071081b
        .quad   0xc1fb4177018b9910
        .quad   0xa6ea20dc6c0fe140
        .quad   0xd661f3e74354c6ff
        .quad   0x5ecb72e6f1a3407a

        // 2^148 * 2 * G

        .quad   0xa515a31b2259fb4e
        .quad   0x0960f3972bcac52f
        .quad   0xedb52fec8d3454cb
        .quad   0x382e2720c476c019
        .quad   0xfeeae106e8e86997
        .quad   0x9863337f98d09383
        .quad   0x9470480eaa06ebef
        .quad   0x038b6898d4c5c2d0
        .quad   0xf391c51d8ace50a6
        .quad   0x3142d0b9ae2d2948
        .quad   0xdb4d5a1a7f24ca80
        .quad   0x21aeba8b59250ea8

        // 2^148 * 3 * G

        .quad   0x24f13b34cf405530
        .quad   0x3c44ea4a43088af7
        .quad   0x5dd5c5170006a482
        .quad   0x118eb8f8890b086d
        .quad   0x53853600f0087f23
        .quad   0x4c461879da7d5784
        .quad   0x6af303deb41f6860
        .quad   0x0a3c16c5c27c18ed
        .quad   0x17e49c17cc947f3d
        .quad   0xccc6eda6aac1d27b
        .quad   0xdf6092ceb0f08e56
        .quad   0x4909b3e22c67c36b

        // 2^148 * 4 * G

        .quad   0x9c9c85ea63fe2e89
        .quad   0xbe1baf910e9412ec
        .quad   0x8f7baa8a86fbfe7b
        .quad   0x0fb17f9fef968b6c
        .quad   0x59a16676706ff64e
        .quad   0x10b953dd0d86a53d
        .quad   0x5848e1e6ce5c0b96
        .quad   0x2d8b78e712780c68
        .quad   0x79d5c62eafc3902b
        .quad   0x773a215289e80728
        .quad   0xc38ae640e10120b9
        .quad   0x09ae23717b2b1a6d

        // 2^148 * 5 * G

        .quad   0xbb6a192a4e4d083c
        .quad   0x34ace0630029e192
        .quad   0x98245a59aafabaeb
        .quad   0x6d9c8a9ada97faac
        .quad   0x10ab8fa1ad32b1d0
        .quad   0xe9aced1be2778b24
        .quad   0xa8856bc0373de90f
        .quad   0x66f35ddddda53996
        .quad   0xd27d9afb24997323
        .quad   0x1bb7e07ef6f01d2e
        .quad   0x2ba7472df52ecc7f
        .quad   0x03019b4f646f9dc8

        // 2^148 * 6 * G

        .quad   0x04a186b5565345cd
        .quad   0xeee76610bcc4116a
        .quad   0x689c73b478fb2a45
        .quad   0x387dcbff65697512
        .quad   0xaf09b214e6b3dc6b
        .quad   0x3f7573b5ad7d2f65
        .quad   0xd019d988100a23b0
        .quad   0x392b63a58b5c35f7
        .quad   0x4093addc9c07c205
        .quad   0xc565be15f532c37e
        .quad   0x63dbecfd1583402a
        .quad   0x61722b4aef2e032e

        // 2^148 * 7 * G

        .quad   0x0012aafeecbd47af
        .quad   0x55a266fb1cd46309
        .quad   0xf203eb680967c72c
        .quad   0x39633944ca3c1429
        .quad   0xd6b07a5581cb0e3c
        .quad   0x290ff006d9444969
        .quad   0x08680b6a16dcda1f
        .quad   0x5568d2b75a06de59
        .quad   0x8d0cb88c1b37cfe1
        .quad   0x05b6a5a3053818f3
        .quad   0xf2e9bc04b787d959
        .quad   0x6beba1249add7f64

        // 2^148 * 8 * G

        .quad   0x1d06005ca5b1b143
        .quad   0x6d4c6bb87fd1cda2
        .quad   0x6ef5967653fcffe7
        .quad   0x097c29e8c1ce1ea5
        .quad   0x5c3cecb943f5a53b
        .quad   0x9cc9a61d06c08df2
        .quad   0xcfba639a85895447
        .quad   0x5a845ae80df09fd5
        .quad   0x4ce97dbe5deb94ca
        .quad   0x38d0a4388c709c48
        .quad   0xc43eced4a169d097
        .quad   0x0a1249fff7e587c3

        // 2^152 * 1 * G

        .quad   0x12f0071b276d01c9
        .quad   0xe7b8bac586c48c70
        .quad   0x5308129b71d6fba9
        .quad   0x5d88fbf95a3db792
        .quad   0x0b408d9e7354b610
        .quad   0x806b32535ba85b6e
        .quad   0xdbe63a034a58a207
        .quad   0x173bd9ddc9a1df2c
        .quad   0x2b500f1efe5872df
        .quad   0x58d6582ed43918c1
        .quad   0xe6ed278ec9673ae0
        .quad   0x06e1cd13b19ea319

        // 2^152 * 2 * G

        .quad   0x40d0ad516f166f23
        .quad   0x118e32931fab6abe
        .quad   0x3fe35e14a04d088e
        .quad   0x3080603526e16266
        .quad   0x472baf629e5b0353
        .quad   0x3baa0b90278d0447
        .quad   0x0c785f469643bf27
        .quad   0x7f3a6a1a8d837b13
        .quad   0xf7e644395d3d800b
        .quad   0x95a8d555c901edf6
        .quad   0x68cd7830592c6339
        .quad   0x30d0fded2e51307e

        // 2^152 * 3 * G

        .quad   0xe0594d1af21233b3
        .quad   0x1bdbe78ef0cc4d9c
        .quad   0x6965187f8f499a77
        .quad   0x0a9214202c099868
        .quad   0x9cb4971e68b84750
        .quad   0xa09572296664bbcf
        .quad   0x5c8de72672fa412b
        .quad   0x4615084351c589d9
        .quad   0xbc9019c0aeb9a02e
        .quad   0x55c7110d16034cae
        .quad   0x0e6df501659932ec
        .quad   0x3bca0d2895ca5dfe

        // 2^152 * 4 * G

        .quad   0x40f031bc3c5d62a4
        .quad   0x19fc8b3ecff07a60
        .quad   0x98183da2130fb545
        .quad   0x5631deddae8f13cd
        .quad   0x9c688eb69ecc01bf
        .quad   0xf0bc83ada644896f
        .quad   0xca2d955f5f7a9fe2
        .quad   0x4ea8b4038df28241
        .quad   0x2aed460af1cad202
        .quad   0x46305305a48cee83
        .quad   0x9121774549f11a5f
        .quad   0x24ce0930542ca463

        // 2^152 * 5 * G

        .quad   0x1fe890f5fd06c106
        .quad   0xb5c468355d8810f2
        .quad   0x827808fe6e8caf3e
        .quad   0x41d4e3c28a06d74b
        .quad   0x3fcfa155fdf30b85
        .quad   0xd2f7168e36372ea4
        .quad   0xb2e064de6492f844
        .quad   0x549928a7324f4280
        .quad   0xf26e32a763ee1a2e
        .quad   0xae91e4b7d25ffdea
        .quad   0xbc3bd33bd17f4d69
        .quad   0x491b66dec0dcff6a

        // 2^152 * 6 * G

        .quad   0x98f5b13dc7ea32a7
        .quad   0xe3d5f8cc7e16db98
        .quad   0xac0abf52cbf8d947
        .quad   0x08f338d0c85ee4ac
        .quad   0x75f04a8ed0da64a1
        .quad   0xed222caf67e2284b
        .quad   0x8234a3791f7b7ba4
        .quad   0x4cf6b8b0b7018b67
        .quad   0xc383a821991a73bd
        .quad   0xab27bc01df320c7a
        .quad   0xc13d331b84777063
        .quad   0x530d4a82eb078a99

        // 2^152 * 7 * G

        .quad   0x004c3630e1f94825
        .quad   0x7e2d78268cab535a
        .quad   0xc7482323cc84ff8b
        .quad   0x65ea753f101770b9
        .quad   0x6d6973456c9abf9e
        .quad   0x257fb2fc4900a880
        .quad   0x2bacf412c8cfb850
        .quad   0x0db3e7e00cbfbd5b
        .quad   0x3d66fc3ee2096363
        .quad   0x81d62c7f61b5cb6b
        .quad   0x0fbe044213443b1a
        .quad   0x02a4ec1921e1a1db

        // 2^152 * 8 * G

        .quad   0x5ce6259a3b24b8a2
        .quad   0xb8577acc45afa0b8
        .quad   0xcccbe6e88ba07037
        .quad   0x3d143c51127809bf
        .quad   0xf5c86162f1cf795f
        .quad   0x118c861926ee57f2
        .quad   0x172124851c063578
        .quad   0x36d12b5dec067fcf
        .quad   0x126d279179154557
        .quad   0xd5e48f5cfc783a0a
        .quad   0x36bdb6e8df179bac
        .quad   0x2ef517885ba82859

        // 2^156 * 1 * G

        .quad   0x88bd438cd11e0d4a
        .quad   0x30cb610d43ccf308
        .quad   0xe09a0e3791937bcc
        .quad   0x4559135b25b1720c
        .quad   0x1ea436837c6da1e9
        .quad   0xf9c189af1fb9bdbe
        .quad   0x303001fcce5dd155
        .quad   0x28a7c99ebc57be52
        .quad   0xb8fd9399e8d19e9d
        .quad   0x908191cb962423ff
        .quad   0xb2b948d747c742a3
        .quad   0x37f33226d7fb44c4

        // 2^156 * 2 * G

        .quad   0x0dae8767b55f6e08
        .quad   0x4a43b3b35b203a02
        .quad   0xe3725a6e80af8c79
        .quad   0x0f7a7fd1705fa7a3
        .quad   0x33912553c821b11d
        .quad   0x66ed42c241e301df
        .quad   0x066fcc11104222fd
        .quad   0x307a3b41c192168f
        .quad   0x8eeb5d076eb55ce0
        .quad   0x2fc536bfaa0d925a
        .quad   0xbe81830fdcb6c6e8
        .quad   0x556c7045827baf52

        // 2^156 * 3 * G

        .quad   0x8e2b517302e9d8b7
        .quad   0xe3e52269248714e8
        .quad   0xbd4fbd774ca960b5
        .quad   0x6f4b4199c5ecada9
        .quad   0xb94b90022bf44406
        .quad   0xabd4237eff90b534
        .quad   0x7600a960faf86d3a
        .quad   0x2f45abdac2322ee3
        .quad   0x61af4912c8ef8a6a
        .quad   0xe58fa4fe43fb6e5e
        .quad   0xb5afcc5d6fd427cf
        .quad   0x6a5393281e1e11eb

        // 2^156 * 4 * G

        .quad   0xf3da5139a5d1ee89
        .quad   0x8145457cff936988
        .quad   0x3f622fed00e188c4
        .quad   0x0f513815db8b5a3d
        .quad   0x0fff04fe149443cf
        .quad   0x53cac6d9865cddd7
        .quad   0x31385b03531ed1b7
        .quad   0x5846a27cacd1039d
        .quad   0x4ff5cdac1eb08717
        .quad   0x67e8b29590f2e9bc
        .quad   0x44093b5e237afa99
        .quad   0x0d414bed8708b8b2

        // 2^156 * 5 * G

        .quad   0xcfb68265fd0e75f6
        .quad   0xe45b3e28bb90e707
        .quad   0x7242a8de9ff92c7a
        .quad   0x685b3201933202dd
        .quad   0x81886a92294ac9e8
        .quad   0x23162b45d55547be
        .quad   0x94cfbc4403715983
        .quad   0x50eb8fdb134bc401
        .quad   0xc0b73ec6d6b330cd
        .quad   0x84e44807132faff1
        .quad   0x732b7352c4a5dee1
        .quad   0x5d7c7cf1aa7cd2d2

        // 2^156 * 6 * G

        .quad   0xaf3b46bf7a4aafa2
        .quad   0xb78705ec4d40d411
        .quad   0x114f0c6aca7c15e3
        .quad   0x3f364faaa9489d4d
        .quad   0x33d1013e9b73a562
        .quad   0x925cef5748ec26e1
        .quad   0xa7fce614dd468058
        .quad   0x78b0fad41e9aa438
        .quad   0xbf56a431ed05b488
        .quad   0xa533e66c9c495c7e
        .quad   0xe8652baf87f3651a
        .quad   0x0241800059d66c33

        // 2^156 * 7 * G

        .quad   0xceb077fea37a5be4
        .quad   0xdb642f02e5a5eeb7
        .quad   0xc2e6d0c5471270b8
        .quad   0x4771b65538e4529c
        .quad   0x28350c7dcf38ea01
        .quad   0x7c6cdbc0b2917ab6
        .quad   0xace7cfbe857082f7
        .quad   0x4d2845aba2d9a1e0
        .quad   0xbb537fe0447070de
        .quad   0xcba744436dd557df
        .quad   0xd3b5a3473600dbcb
        .quad   0x4aeabbe6f9ffd7f8

        // 2^156 * 8 * G

        .quad   0x4630119e40d8f78c
        .quad   0xa01a9bc53c710e11
        .quad   0x486d2b258910dd79
        .quad   0x1e6c47b3db0324e5
        .quad   0x6a2134bcc4a9c8f2
        .quad   0xfbf8fd1c8ace2e37
        .quad   0x000ae3049911a0ba
        .quad   0x046e3a616bc89b9e
        .quad   0x14e65442f03906be
        .quad   0x4a019d54e362be2a
        .quad   0x68ccdfec8dc230c7
        .quad   0x7cfb7e3faf6b861c

        // 2^160 * 1 * G

        .quad   0x4637974e8c58aedc
        .quad   0xb9ef22fbabf041a4
        .quad   0xe185d956e980718a
        .quad   0x2f1b78fab143a8a6
        .quad   0x96eebffb305b2f51
        .quad   0xd3f938ad889596b8
        .quad   0xf0f52dc746d5dd25
        .quad   0x57968290bb3a0095
        .quad   0xf71ab8430a20e101
        .quad   0xf393658d24f0ec47
        .quad   0xcf7509a86ee2eed1
        .quad   0x7dc43e35dc2aa3e1

        // 2^160 * 2 * G

        .quad   0x85966665887dd9c3
        .quad   0xc90f9b314bb05355
        .quad   0xc6e08df8ef2079b1
        .quad   0x7ef72016758cc12f
        .quad   0x5a782a5c273e9718
        .quad   0x3576c6995e4efd94
        .quad   0x0f2ed8051f237d3e
        .quad   0x044fb81d82d50a99
        .quad   0xc1df18c5a907e3d9
        .quad   0x57b3371dce4c6359
        .quad   0xca704534b201bb49
        .quad   0x7f79823f9c30dd2e

        // 2^160 * 3 * G

        .quad   0x8334d239a3b513e8
        .quad   0xc13670d4b91fa8d8
        .quad   0x12b54136f590bd33
        .quad   0x0a4e0373d784d9b4
        .quad   0x6a9c1ff068f587ba
        .quad   0x0827894e0050c8de
        .quad   0x3cbf99557ded5be7
        .quad   0x64a9b0431c06d6f0
        .quad   0x2eb3d6a15b7d2919
        .quad   0xb0b4f6a0d53a8235
        .quad   0x7156ce4389a45d47
        .quad   0x071a7d0ace18346c

        // 2^160 * 4 * G

        .quad   0xd3072daac887ba0b
        .quad   0x01262905bfa562ee
        .quad   0xcf543002c0ef768b
        .quad   0x2c3bcc7146ea7e9c
        .quad   0xcc0c355220e14431
        .quad   0x0d65950709b15141
        .quad   0x9af5621b209d5f36
        .quad   0x7c69bcf7617755d3
        .quad   0x07f0d7eb04e8295f
        .quad   0x10db18252f50f37d
        .quad   0xe951a9a3171798d7
        .quad   0x6f5a9a7322aca51d

        // 2^160 * 5 * G

        .quad   0x8ba1000c2f41c6c5
        .quad   0xc49f79c10cfefb9b
        .quad   0x4efa47703cc51c9f
        .quad   0x494e21a2e147afca
        .quad   0xe729d4eba3d944be
        .quad   0x8d9e09408078af9e
        .quad   0x4525567a47869c03
        .quad   0x02ab9680ee8d3b24
        .quad   0xefa48a85dde50d9a
        .quad   0x219a224e0fb9a249
        .quad   0xfa091f1dd91ef6d9
        .quad   0x6b5d76cbea46bb34

        // 2^160 * 6 * G

        .quad   0x8857556cec0cd994
        .quad   0x6472dc6f5cd01dba
        .quad   0xaf0169148f42b477
        .quad   0x0ae333f685277354
        .quad   0xe0f941171e782522
        .quad   0xf1e6ae74036936d3
        .quad   0x408b3ea2d0fcc746
        .quad   0x16fb869c03dd313e
        .quad   0x288e199733b60962
        .quad   0x24fc72b4d8abe133
        .quad   0x4811f7ed0991d03e
        .quad   0x3f81e38b8f70d075

        // 2^160 * 7 * G

        .quad   0x7f910fcc7ed9affe
        .quad   0x545cb8a12465874b
        .quad   0xa8397ed24b0c4704
        .quad   0x50510fc104f50993
        .quad   0x0adb7f355f17c824
        .quad   0x74b923c3d74299a4
        .quad   0xd57c3e8bcbf8eaf7
        .quad   0x0ad3e2d34cdedc3d
        .quad   0x6f0c0fc5336e249d
        .quad   0x745ede19c331cfd9
        .quad   0xf2d6fd0009eefe1c
        .quad   0x127c158bf0fa1ebe

        // 2^160 * 8 * G

        .quad   0xf6197c422e9879a2
        .quad   0xa44addd452ca3647
        .quad   0x9b413fc14b4eaccb
        .quad   0x354ef87d07ef4f68
        .quad   0xdea28fc4ae51b974
        .quad   0x1d9973d3744dfe96
        .quad   0x6240680b873848a8
        .quad   0x4ed82479d167df95
        .quad   0xfee3b52260c5d975
        .quad   0x50352efceb41b0b8
        .quad   0x8808ac30a9f6653c
        .quad   0x302d92d20539236d

        // 2^164 * 1 * G

        .quad   0x4c59023fcb3efb7c
        .quad   0x6c2fcb99c63c2a94
        .quad   0xba4190e2c3c7e084
        .quad   0x0e545daea51874d9
        .quad   0x957b8b8b0df53c30
        .quad   0x2a1c770a8e60f098
        .quad   0xbbc7a670345796de
        .quad   0x22a48f9a90c99bc9
        .quad   0x6b7dc0dc8d3fac58
        .quad   0x5497cd6ce6e42bfd
        .quad   0x542f7d1bf400d305
        .quad   0x4159f47f048d9136

        // 2^164 * 2 * G

        .quad   0x20ad660839e31e32
        .quad   0xf81e1bd58405be50
        .quad   0xf8064056f4dabc69
        .quad   0x14d23dd4ce71b975
        .quad   0x748515a8bbd24839
        .quad   0x77128347afb02b55
        .quad   0x50ba2ac649a2a17f
        .quad   0x060525513ad730f1
        .quad   0xf2398e098aa27f82
        .quad   0x6d7982bb89a1b024
        .quad   0xfa694084214dd24c
        .quad   0x71ab966fa32301c3

        // 2^164 * 3 * G

        .quad   0x2dcbd8e34ded02fc
        .quad   0x1151f3ec596f22aa
        .quad   0xbca255434e0328da
        .quad   0x35768fbe92411b22
        .quad   0xb1088a0702809955
        .quad   0x43b273ea0b43c391
        .quad   0xca9b67aefe0686ed
        .quad   0x605eecbf8335f4ed
        .quad   0x83200a656c340431
        .quad   0x9fcd71678ee59c2f
        .quad   0x75d4613f71300f8a
        .quad   0x7a912faf60f542f9

        // 2^164 * 4 * G

        .quad   0xb204585e5edc1a43
        .quad   0x9f0e16ee5897c73c
        .quad   0x5b82c0ae4e70483c
        .quad   0x624a170e2bddf9be
        .quad   0x253f4f8dfa2d5597
        .quad   0x25e49c405477130c
        .quad   0x00c052e5996b1102
        .quad   0x33cb966e33bb6c4a
        .quad   0x597028047f116909
        .quad   0x828ac41c1e564467
        .quad   0x70417dbde6217387
        .quad   0x721627aefbac4384

        // 2^164 * 5 * G

        .quad   0x97d03bc38736add5
        .quad   0x2f1422afc532b130
        .quad   0x3aa68a057101bbc4
        .quad   0x4c946cf7e74f9fa7
        .quad   0xfd3097bc410b2f22
        .quad   0xf1a05da7b5cfa844
        .quad   0x61289a1def57ca74
        .quad   0x245ea199bb821902
        .quad   0xaedca66978d477f8
        .quad   0x1898ba3c29117fe1
        .quad   0xcf73f983720cbd58
        .quad   0x67da12e6b8b56351

        // 2^164 * 6 * G

        .quad   0x7067e187b4bd6e07
        .quad   0x6e8f0203c7d1fe74
        .quad   0x93c6aa2f38c85a30
        .quad   0x76297d1f3d75a78a
        .quad   0x2b7ef3d38ec8308c
        .quad   0x828fd7ec71eb94ab
        .quad   0x807c3b36c5062abd
        .quad   0x0cb64cb831a94141
        .quad   0x3030fc33534c6378
        .quad   0xb9635c5ce541e861
        .quad   0x15d9a9bed9b2c728
        .quad   0x49233ea3f3775dcb

        // 2^164 * 7 * G

        .quad   0x629398fa8dbffc3a
        .quad   0xe12fe52dd54db455
        .quad   0xf3be11dfdaf25295
        .quad   0x628b140dce5e7b51
        .quad   0x7b3985fe1c9f249b
        .quad   0x4fd6b2d5a1233293
        .quad   0xceb345941adf4d62
        .quad   0x6987ff6f542de50c
        .quad   0x47e241428f83753c
        .quad   0x6317bebc866af997
        .quad   0xdabb5b433d1a9829
        .quad   0x074d8d245287fb2d

        // 2^164 * 8 * G

        .quad   0x8337d9cd440bfc31
        .quad   0x729d2ca1af318fd7
        .quad   0xa040a4a4772c2070
        .quad   0x46002ef03a7349be
        .quad   0x481875c6c0e31488
        .quad   0x219429b2e22034b4
        .quad   0x7223c98a31283b65
        .quad   0x3420d60b342277f9
        .quad   0xfaa23adeaffe65f7
        .quad   0x78261ed45be0764c
        .quad   0x441c0a1e2f164403
        .quad   0x5aea8e567a87d395

        // 2^168 * 1 * G

        .quad   0x7813c1a2bca4283d
        .quad   0xed62f091a1863dd9
        .quad   0xaec7bcb8c268fa86
        .quad   0x10e5d3b76f1cae4c
        .quad   0x2dbc6fb6e4e0f177
        .quad   0x04e1bf29a4bd6a93
        .quad   0x5e1966d4787af6e8
        .quad   0x0edc5f5eb426d060
        .quad   0x5453bfd653da8e67
        .quad   0xe9dc1eec24a9f641
        .quad   0xbf87263b03578a23
        .quad   0x45b46c51361cba72

        // 2^168 * 2 * G

        .quad   0xa9402abf314f7fa1
        .quad   0xe257f1dc8e8cf450
        .quad   0x1dbbd54b23a8be84
        .quad   0x2177bfa36dcb713b
        .quad   0xce9d4ddd8a7fe3e4
        .quad   0xab13645676620e30
        .quad   0x4b594f7bb30e9958
        .quad   0x5c1c0aef321229df
        .quad   0x37081bbcfa79db8f
        .quad   0x6048811ec25f59b3
        .quad   0x087a76659c832487
        .quad   0x4ae619387d8ab5bb

        // 2^168 * 3 * G

        .quad   0x8ddbf6aa5344a32e
        .quad   0x7d88eab4b41b4078
        .quad   0x5eb0eb974a130d60
        .quad   0x1a00d91b17bf3e03
        .quad   0x61117e44985bfb83
        .quad   0xfce0462a71963136
        .quad   0x83ac3448d425904b
        .quad   0x75685abe5ba43d64
        .quad   0x6e960933eb61f2b2
        .quad   0x543d0fa8c9ff4952
        .quad   0xdf7275107af66569
        .quad   0x135529b623b0e6aa

        // 2^168 * 4 * G

        .quad   0x18f0dbd7add1d518
        .quad   0x979f7888cfc11f11
        .quad   0x8732e1f07114759b
        .quad   0x79b5b81a65ca3a01
        .quad   0xf5c716bce22e83fe
        .quad   0xb42beb19e80985c1
        .quad   0xec9da63714254aae
        .quad   0x5972ea051590a613
        .quad   0x0fd4ac20dc8f7811
        .quad   0x9a9ad294ac4d4fa8
        .quad   0xc01b2d64b3360434
        .quad   0x4f7e9c95905f3bdb

        // 2^168 * 5 * G

        .quad   0x62674bbc5781302e
        .quad   0xd8520f3989addc0f
        .quad   0x8c2999ae53fbd9c6
        .quad   0x31993ad92e638e4c
        .quad   0x71c8443d355299fe
        .quad   0x8bcd3b1cdbebead7
        .quad   0x8092499ef1a49466
        .quad   0x1942eec4a144adc8
        .quad   0x7dac5319ae234992
        .quad   0x2c1b3d910cea3e92
        .quad   0x553ce494253c1122
        .quad   0x2a0a65314ef9ca75

        // 2^168 * 6 * G

        .quad   0x2db7937ff7f927c2
        .quad   0xdb741f0617d0a635
        .quad   0x5982f3a21155af76
        .quad   0x4cf6e218647c2ded
        .quad   0xcf361acd3c1c793a
        .quad   0x2f9ebcac5a35bc3b
        .quad   0x60e860e9a8cda6ab
        .quad   0x055dc39b6dea1a13
        .quad   0xb119227cc28d5bb6
        .quad   0x07e24ebc774dffab
        .quad   0xa83c78cee4a32c89
        .quad   0x121a307710aa24b6

        // 2^168 * 7 * G

        .quad   0xe4db5d5e9f034a97
        .quad   0xe153fc093034bc2d
        .quad   0x460546919551d3b1
        .quad   0x333fc76c7a40e52d
        .quad   0xd659713ec77483c9
        .quad   0x88bfe077b82b96af
        .quad   0x289e28231097bcd3
        .quad   0x527bb94a6ced3a9b
        .quad   0x563d992a995b482e
        .quad   0x3405d07c6e383801
        .quad   0x485035de2f64d8e5
        .quad   0x6b89069b20a7a9f7

        // 2^168 * 8 * G

        .quad   0x812aa0416270220d
        .quad   0x995a89faf9245b4e
        .quad   0xffadc4ce5072ef05
        .quad   0x23bc2103aa73eb73
        .quad   0x4082fa8cb5c7db77
        .quad   0x068686f8c734c155
        .quad   0x29e6c8d9f6e7a57e
        .quad   0x0473d308a7639bcf
        .quad   0xcaee792603589e05
        .quad   0x2b4b421246dcc492
        .quad   0x02a1ef74e601a94f
        .quad   0x102f73bfde04341a

        // 2^172 * 1 * G

        .quad   0xb5a2d50c7ec20d3e
        .quad   0xc64bdd6ea0c97263
        .quad   0x56e89052c1ff734d
        .quad   0x4929c6f72b2ffaba
        .quad   0x358ecba293a36247
        .quad   0xaf8f9862b268fd65
        .quad   0x412f7e9968a01c89
        .quad   0x5786f312cd754524
        .quad   0x337788ffca14032c
        .quad   0xf3921028447f1ee3
        .quad   0x8b14071f231bccad
        .quad   0x4c817b4bf2344783

        // 2^172 * 2 * G

        .quad   0x0ff853852871b96e
        .quad   0xe13e9fab60c3f1bb
        .quad   0xeefd595325344402
        .quad   0x0a37c37075b7744b
        .quad   0x413ba057a40b4484
        .quad   0xba4c2e1a4f5f6a43
        .quad   0x614ba0a5aee1d61c
        .quad   0x78a1531a8b05dc53
        .quad   0x6cbdf1703ad0562b
        .quad   0x8ecf4830c92521a3
        .quad   0xdaebd303fd8424e7
        .quad   0x72ad82a42e5ec56f

        // 2^172 * 3 * G

        .quad   0x3f9e8e35bafb65f6
        .quad   0x39d69ec8f27293a1
        .quad   0x6cb8cd958cf6a3d0
        .quad   0x1734778173adae6d
        .quad   0xc368939167024bc3
        .quad   0x8e69d16d49502fda
        .quad   0xfcf2ec3ce45f4b29
        .quad   0x065f669ea3b4cbc4
        .quad   0x8a00aec75532db4d
        .quad   0xb869a4e443e31bb1
        .quad   0x4a0f8552d3a7f515
        .quad   0x19adeb7c303d7c08

        // 2^172 * 4 * G

        .quad   0xc720cb6153ead9a3
        .quad   0x55b2c97f512b636e
        .quad   0xb1e35b5fd40290b1
        .quad   0x2fd9ccf13b530ee2
        .quad   0x9d05ba7d43c31794
        .quad   0x2470c8ff93322526
        .quad   0x8323dec816197438
        .quad   0x2852709881569b53
        .quad   0x07bd475b47f796b8
        .quad   0xd2c7b013542c8f54
        .quad   0x2dbd23f43b24f87e
        .quad   0x6551afd77b0901d6

        // 2^172 * 5 * G

        .quad   0x4546baaf54aac27f
        .quad   0xf6f66fecb2a45a28
        .quad   0x582d1b5b562bcfe8
        .quad   0x44b123f3920f785f
        .quad   0x68a24ce3a1d5c9ac
        .quad   0xbb77a33d10ff6461
        .quad   0x0f86ce4425d3166e
        .quad   0x56507c0950b9623b
        .quad   0x1206f0b7d1713e63
        .quad   0x353fe3d915bafc74
        .quad   0x194ceb970ad9d94d
        .quad   0x62fadd7cf9d03ad3

        // 2^172 * 6 * G

        .quad   0xc6b5967b5598a074
        .quad   0x5efe91ce8e493e25
        .quad   0xd4b72c4549280888
        .quad   0x20ef1149a26740c2
        .quad   0x3cd7bc61e7ce4594
        .quad   0xcd6b35a9b7dd267e
        .quad   0xa080abc84366ef27
        .quad   0x6ec7c46f59c79711
        .quad   0x2f07ad636f09a8a2
        .quad   0x8697e6ce24205e7d
        .quad   0xc0aefc05ee35a139
        .quad   0x15e80958b5f9d897

        // 2^172 * 7 * G

        .quad   0x25a5ef7d0c3e235b
        .quad   0x6c39c17fbe134ee7
        .quad   0xc774e1342dc5c327
        .quad   0x021354b892021f39
        .quad   0x4dd1ed355bb061c4
        .quad   0x42dc0cef941c0700
        .quad   0x61305dc1fd86340e
        .quad   0x56b2cc930e55a443
        .quad   0x1df79da6a6bfc5a2
        .quad   0x02f3a2749fde4369
        .quad   0xb323d9f2cda390a7
        .quad   0x7be0847b8774d363

        // 2^172 * 8 * G

        .quad   0x8c99cc5a8b3f55c3
        .quad   0x0611d7253fded2a0
        .quad   0xed2995ff36b70a36
        .quad   0x1f699a54d78a2619
        .quad   0x1466f5af5307fa11
        .quad   0x817fcc7ded6c0af2
        .quad   0x0a6de44ec3a4a3fb
        .quad   0x74071475bc927d0b
        .quad   0xe77292f373e7ea8a
        .quad   0x296537d2cb045a31
        .quad   0x1bd0653ed3274fde
        .quad   0x2f9a2c4476bd2966

        // 2^176 * 1 * G

        .quad   0xeb18b9ab7f5745c6
        .quad   0x023a8aee5787c690
        .quad   0xb72712da2df7afa9
        .quad   0x36597d25ea5c013d
        .quad   0xa2b4dae0b5511c9a
        .quad   0x7ac860292bffff06
        .quad   0x981f375df5504234
        .quad   0x3f6bd725da4ea12d
        .quad   0x734d8d7b106058ac
        .quad   0xd940579e6fc6905f
        .quad   0x6466f8f99202932d
        .quad   0x7b7ecc19da60d6d0

        // 2^176 * 2 * G

        .quad   0x78c2373c695c690d
        .quad   0xdd252e660642906e
        .quad   0x951d44444ae12bd2
        .quad   0x4235ad7601743956
        .quad   0x6dae4a51a77cfa9b
        .quad   0x82263654e7a38650
        .quad   0x09bbffcd8f2d82db
        .quad   0x03bedc661bf5caba
        .quad   0x6258cb0d078975f5
        .quad   0x492942549189f298
        .quad   0xa0cab423e2e36ee4
        .quad   0x0e7ce2b0cdf066a1

        // 2^176 * 3 * G

        .quad   0xc494643ac48c85a3
        .quad   0xfd361df43c6139ad
        .quad   0x09db17dd3ae94d48
        .quad   0x666e0a5d8fb4674a
        .quad   0xfea6fedfd94b70f9
        .quad   0xf130c051c1fcba2d
        .quad   0x4882d47e7f2fab89
        .quad   0x615256138aeceeb5
        .quad   0x2abbf64e4870cb0d
        .quad   0xcd65bcf0aa458b6b
        .quad   0x9abe4eba75e8985d
        .quad   0x7f0bc810d514dee4

        // 2^176 * 4 * G

        .quad   0xb9006ba426f4136f
        .quad   0x8d67369e57e03035
        .quad   0xcbc8dfd94f463c28
        .quad   0x0d1f8dbcf8eedbf5
        .quad   0x83ac9dad737213a0
        .quad   0x9ff6f8ba2ef72e98
        .quad   0x311e2edd43ec6957
        .quad   0x1d3a907ddec5ab75
        .quad   0xba1693313ed081dc
        .quad   0x29329fad851b3480
        .quad   0x0128013c030321cb
        .quad   0x00011b44a31bfde3

        // 2^176 * 5 * G

        .quad   0x3fdfa06c3fc66c0c
        .quad   0x5d40e38e4dd60dd2
        .quad   0x7ae38b38268e4d71
        .quad   0x3ac48d916e8357e1
        .quad   0x16561f696a0aa75c
        .quad   0xc1bf725c5852bd6a
        .quad   0x11a8dd7f9a7966ad
        .quad   0x63d988a2d2851026
        .quad   0x00120753afbd232e
        .quad   0xe92bceb8fdd8f683
        .quad   0xf81669b384e72b91
        .quad   0x33fad52b2368a066

        // 2^176 * 6 * G

        .quad   0x540649c6c5e41e16
        .quad   0x0af86430333f7735
        .quad   0xb2acfcd2f305e746
        .quad   0x16c0f429a256dca7
        .quad   0x8d2cc8d0c422cfe8
        .quad   0x072b4f7b05a13acb
        .quad   0xa3feb6e6ecf6a56f
        .quad   0x3cc355ccb90a71e2
        .quad   0xe9b69443903e9131
        .quad   0xb8a494cb7a5637ce
        .quad   0xc87cd1a4baba9244
        .quad   0x631eaf426bae7568

        // 2^176 * 7 * G

        .quad   0xb3e90410da66fe9f
        .quad   0x85dd4b526c16e5a6
        .quad   0xbc3d97611ef9bf83
        .quad   0x5599648b1ea919b5
        .quad   0x47d975b9a3700de8
        .quad   0x7280c5fbe2f80552
        .quad   0x53658f2732e45de1
        .quad   0x431f2c7f665f80b5
        .quad   0xd6026344858f7b19
        .quad   0x14ab352fa1ea514a
        .quad   0x8900441a2090a9d7
        .quad   0x7b04715f91253b26

        // 2^176 * 8 * G

        .quad   0x83edbd28acf6ae43
        .quad   0x86357c8b7d5c7ab4
        .quad   0xc0404769b7eb2c44
        .quad   0x59b37bf5c2f6583f
        .quad   0xb376c280c4e6bac6
        .quad   0x970ed3dd6d1d9b0b
        .quad   0xb09a9558450bf944
        .quad   0x48d0acfa57cde223
        .quad   0xb60f26e47dabe671
        .quad   0xf1d1a197622f3a37
        .quad   0x4208ce7ee9960394
        .quad   0x16234191336d3bdb

        // 2^180 * 1 * G

        .quad   0xf19aeac733a63aef
        .quad   0x2c7fba5d4442454e
        .quad   0x5da87aa04795e441
        .quad   0x413051e1a4e0b0f5
        .quad   0x852dd1fd3d578bbe
        .quad   0x2b65ce72c3286108
        .quad   0x658c07f4eace2273
        .quad   0x0933f804ec38ab40
        .quad   0xa7ab69798d496476
        .quad   0x8121aadefcb5abc8
        .quad   0xa5dc12ef7b539472
        .quad   0x07fd47065e45351a

        // 2^180 * 2 * G

        .quad   0xc8583c3d258d2bcd
        .quad   0x17029a4daf60b73f
        .quad   0xfa0fc9d6416a3781
        .quad   0x1c1e5fba38b3fb23
        .quad   0x304211559ae8e7c3
        .quad   0xf281b229944882a5
        .quad   0x8a13ac2e378250e4
        .quad   0x014afa0954ba48f4
        .quad   0xcb3197001bb3666c
        .quad   0x330060524bffecb9
        .quad   0x293711991a88233c
        .quad   0x291884363d4ed364

        // 2^180 * 3 * G

        .quad   0x033c6805dc4babfa
        .quad   0x2c15bf5e5596ecc1
        .quad   0x1bc70624b59b1d3b
        .quad   0x3ede9850a19f0ec5
        .quad   0xfb9d37c3bc1ab6eb
        .quad   0x02be14534d57a240
        .quad   0xf4d73415f8a5e1f6
        .quad   0x5964f4300ccc8188
        .quad   0xe44a23152d096800
        .quad   0x5c08c55970866996
        .quad   0xdf2db60a46affb6e
        .quad   0x579155c1f856fd89

        // 2^180 * 4 * G

        .quad   0x96324edd12e0c9ef
        .quad   0x468b878df2420297
        .quad   0x199a3776a4f573be
        .quad   0x1e7fbcf18e91e92a
        .quad   0xb5f16b630817e7a6
        .quad   0x808c69233c351026
        .quad   0x324a983b54cef201
        .quad   0x53c092084a485345
        .quad   0xd2d41481f1cbafbf
        .quad   0x231d2db6716174e5
        .quad   0x0b7d7656e2a55c98
        .quad   0x3e955cd82aa495f6

        // 2^180 * 5 * G

        .quad   0xe48f535e3ed15433
        .quad   0xd075692a0d7270a3
        .quad   0x40fbd21daade6387
        .quad   0x14264887cf4495f5
        .quad   0xab39f3ef61bb3a3f
        .quad   0x8eb400652eb9193e
        .quad   0xb5de6ecc38c11f74
        .quad   0x654d7e9626f3c49f
        .quad   0xe564cfdd5c7d2ceb
        .quad   0x82eeafded737ccb9
        .quad   0x6107db62d1f9b0ab
        .quad   0x0b6baac3b4358dbb

        // 2^180 * 6 * G

        .quad   0x7ae62bcb8622fe98
        .quad   0x47762256ceb891af
        .quad   0x1a5a92bcf2e406b4
        .quad   0x7d29401784e41501
        .quad   0x204abad63700a93b
        .quad   0xbe0023d3da779373
        .quad   0xd85f0346633ab709
        .quad   0x00496dc490820412
        .quad   0x1c74b88dc27e6360
        .quad   0x074854268d14850c
        .quad   0xa145fb7b3e0dcb30
        .quad   0x10843f1b43803b23

        // 2^180 * 7 * G

        .quad   0xc5f90455376276dd
        .quad   0xce59158dd7645cd9
        .quad   0x92f65d511d366b39
        .quad   0x11574b6e526996c4
        .quad   0xd56f672de324689b
        .quad   0xd1da8aedb394a981
        .quad   0xdd7b58fe9168cfed
        .quad   0x7ce246cd4d56c1e8
        .quad   0xb8f4308e7f80be53
        .quad   0x5f3cb8cb34a9d397
        .quad   0x18a961bd33cc2b2c
        .quad   0x710045fb3a9af671

        // 2^180 * 8 * G

        .quad   0x73f93d36101b95eb
        .quad   0xfaef33794f6f4486
        .quad   0x5651735f8f15e562
        .quad   0x7fa3f19058b40da1
        .quad   0xa03fc862059d699e
        .quad   0x2370cfa19a619e69
        .quad   0xc4fe3b122f823deb
        .quad   0x1d1b056fa7f0844e
        .quad   0x1bc64631e56bf61f
        .quad   0xd379ab106e5382a3
        .quad   0x4d58c57e0540168d
        .quad   0x566256628442d8e4

        // 2^184 * 1 * G

        .quad   0xb9e499def6267ff6
        .quad   0x7772ca7b742c0843
        .quad   0x23a0153fe9a4f2b1
        .quad   0x2cdfdfecd5d05006
        .quad   0xdd499cd61ff38640
        .quad   0x29cd9bc3063625a0
        .quad   0x51e2d8023dd73dc3
        .quad   0x4a25707a203b9231
        .quad   0x2ab7668a53f6ed6a
        .quad   0x304242581dd170a1
        .quad   0x4000144c3ae20161
        .quad   0x5721896d248e49fc

        // 2^184 * 2 * G

        .quad   0x0b6e5517fd181bae
        .quad   0x9022629f2bb963b4
        .quad   0x5509bce932064625
        .quad   0x578edd74f63c13da
        .quad   0x285d5091a1d0da4e
        .quad   0x4baa6fa7b5fe3e08
        .quad   0x63e5177ce19393b3
        .quad   0x03c935afc4b030fd
        .quad   0x997276c6492b0c3d
        .quad   0x47ccc2c4dfe205fc
        .quad   0xdcd29b84dd623a3c
        .quad   0x3ec2ab590288c7a2

        // 2^184 * 3 * G

        .quad   0xa1a0d27be4d87bb9
        .quad   0xa98b4deb61391aed
        .quad   0x99a0ddd073cb9b83
        .quad   0x2dd5c25a200fcace
        .quad   0xa7213a09ae32d1cb
        .quad   0x0f2b87df40f5c2d5
        .quad   0x0baea4c6e81eab29
        .quad   0x0e1bf66c6adbac5e
        .quad   0xe2abd5e9792c887e
        .quad   0x1a020018cb926d5d
        .quad   0xbfba69cdbaae5f1e
        .quad   0x730548b35ae88f5f

        // 2^184 * 4 * G

        .quad   0xc43551a3cba8b8ee
        .quad   0x65a26f1db2115f16
        .quad   0x760f4f52ab8c3850
        .quad   0x3043443b411db8ca
        .quad   0x805b094ba1d6e334
        .quad   0xbf3ef17709353f19
        .quad   0x423f06cb0622702b
        .quad   0x585a2277d87845dd
        .quad   0xa18a5f8233d48962
        .quad   0x6698c4b5ec78257f
        .quad   0xa78e6fa5373e41ff
        .quad   0x7656278950ef981f

        // 2^184 * 5 * G

        .quad   0x38c3cf59d51fc8c0
        .quad   0x9bedd2fd0506b6f2
        .quad   0x26bf109fab570e8f
        .quad   0x3f4160a8c1b846a6
        .quad   0xe17073a3ea86cf9d
        .quad   0x3a8cfbb707155fdc
        .quad   0x4853e7fc31838a8e
        .quad   0x28bbf484b613f616
        .quad   0xf2612f5c6f136c7c
        .quad   0xafead107f6dd11be
        .quad   0x527e9ad213de6f33
        .quad   0x1e79cb358188f75d

        // 2^184 * 6 * G

        .quad   0x013436c3eef7e3f1
        .quad   0x828b6a7ffe9e10f8
        .quad   0x7ff908e5bcf9defc
        .quad   0x65d7951b3a3b3831
        .quad   0x77e953d8f5e08181
        .quad   0x84a50c44299dded9
        .quad   0xdc6c2d0c864525e5
        .quad   0x478ab52d39d1f2f4
        .quad   0x66a6a4d39252d159
        .quad   0xe5dde1bc871ac807
        .quad   0xb82c6b40a6c1c96f
        .quad   0x16d87a411a212214

        // 2^184 * 7 * G

        .quad   0xb3bd7e5a42066215
        .quad   0x879be3cd0c5a24c1
        .quad   0x57c05db1d6f994b7
        .quad   0x28f87c8165f38ca6
        .quad   0xfba4d5e2d54e0583
        .quad   0xe21fafd72ebd99fa
        .quad   0x497ac2736ee9778f
        .quad   0x1f990b577a5a6dde
        .quad   0xa3344ead1be8f7d6
        .quad   0x7d1e50ebacea798f
        .quad   0x77c6569e520de052
        .quad   0x45882fe1534d6d3e

        // 2^184 * 8 * G

        .quad   0x6669345d757983d6
        .quad   0x62b6ed1117aa11a6
        .quad   0x7ddd1857985e128f
        .quad   0x688fe5b8f626f6dd
        .quad   0xd8ac9929943c6fe4
        .quad   0xb5f9f161a38392a2
        .quad   0x2699db13bec89af3
        .quad   0x7dcf843ce405f074
        .quad   0x6c90d6484a4732c0
        .quad   0xd52143fdca563299
        .quad   0xb3be28c3915dc6e1
        .quad   0x6739687e7327191b

        // 2^188 * 1 * G

        .quad   0x9f65c5ea200814cf
        .quad   0x840536e169a31740
        .quad   0x8b0ed13925c8b4ad
        .quad   0x0080dbafe936361d
        .quad   0x8ce5aad0c9cb971f
        .quad   0x1156aaa99fd54a29
        .quad   0x41f7247015af9b78
        .quad   0x1fe8cca8420f49aa
        .quad   0x72a1848f3c0cc82a
        .quad   0x38c560c2877c9e54
        .quad   0x5004e228ce554140
        .quad   0x042418a103429d71

        // 2^188 * 2 * G

        .quad   0x899dea51abf3ff5f
        .quad   0x9b93a8672fc2d8ba
        .quad   0x2c38cb97be6ebd5c
        .quad   0x114d578497263b5d
        .quad   0x58e84c6f20816247
        .quad   0x8db2b2b6e36fd793
        .quad   0x977182561d484d85
        .quad   0x0822024f8632abd7
        .quad   0xb301bb7c6b1beca3
        .quad   0x55393f6dc6eb1375
        .quad   0x910d281097b6e4eb
        .quad   0x1ad4548d9d479ea3

        // 2^188 * 3 * G

        .quad   0xcd5a7da0389a48fd
        .quad   0xb38fa4aa9a78371e
        .quad   0xc6d9761b2cdb8e6c
        .quad   0x35cf51dbc97e1443
        .quad   0xa06fe66d0fe9fed3
        .quad   0xa8733a401c587909
        .quad   0x30d14d800df98953
        .quad   0x41ce5876c7b30258
        .quad   0x59ac3bc5d670c022
        .quad   0xeae67c109b119406
        .quad   0x9798bdf0b3782fda
        .quad   0x651e3201fd074092

        // 2^188 * 4 * G

        .quad   0xd63d8483ef30c5cf
        .quad   0x4cd4b4962361cc0c
        .quad   0xee90e500a48426ac
        .quad   0x0af51d7d18c14eeb
        .quad   0xa57ba4a01efcae9e
        .quad   0x769f4beedc308a94
        .quad   0xd1f10eeb3603cb2e
        .quad   0x4099ce5e7e441278
        .quad   0x1ac98e4f8a5121e9
        .quad   0x7dae9544dbfa2fe0
        .quad   0x8320aa0dd6430df9
        .quad   0x667282652c4a2fb5

        // 2^188 * 5 * G

        .quad   0x874621f4d86bc9ab
        .quad   0xb54c7bbe56fe6fea
        .quad   0x077a24257fadc22c
        .quad   0x1ab53be419b90d39
        .quad   0xada8b6e02946db23
        .quad   0x1c0ce51a7b253ab7
        .quad   0x8448c85a66dd485b
        .quad   0x7f1fc025d0675adf
        .quad   0xd8ee1b18319ea6aa
        .quad   0x004d88083a21f0da
        .quad   0x3bd6aa1d883a4f4b
        .quad   0x4db9a3a6dfd9fd14

        // 2^188 * 6 * G

        .quad   0x8ce7b23bb99c0755
        .quad   0x35c5d6edc4f50f7a
        .quad   0x7e1e2ed2ed9b50c3
        .quad   0x36305f16e8934da1
        .quad   0xd95b00bbcbb77c68
        .quad   0xddbc846a91f17849
        .quad   0x7cf700aebe28d9b3
        .quad   0x5ce1285c85d31f3e
        .quad   0x31b6972d98b0bde8
        .quad   0x7d920706aca6de5b
        .quad   0xe67310f8908a659f
        .quad   0x50fac2a6efdf0235

        // 2^188 * 7 * G

        .quad   0xf3d3a9f35b880f5a
        .quad   0xedec050cdb03e7c2
        .quad   0xa896981ff9f0b1a2
        .quad   0x49a4ae2bac5e34a4
        .quad   0x295b1c86f6f449bc
        .quad   0x51b2e84a1f0ab4dd
        .quad   0xc001cb30aa8e551d
        .quad   0x6a28d35944f43662
        .quad   0x28bb12ee04a740e0
        .quad   0x14313bbd9bce8174
        .quad   0x72f5b5e4e8c10c40
        .quad   0x7cbfb19936adcd5b

        // 2^188 * 8 * G

        .quad   0xa311ddc26b89792d
        .quad   0x1b30b4c6da512664
        .quad   0x0ca77b4ccf150859
        .quad   0x1de443df1b009408
        .quad   0x8e793a7acc36e6e0
        .quad   0xf9fab7a37d586eed
        .quad   0x3a4f9692bae1f4e4
        .quad   0x1c14b03eff5f447e
        .quad   0x19647bd114a85291
        .quad   0x57b76cb21034d3af
        .quad   0x6329db440f9d6dfa
        .quad   0x5ef43e586a571493

        // 2^192 * 1 * G

        .quad   0xef782014385675a6
        .quad   0xa2649f30aafda9e8
        .quad   0x4cd1eb505cdfa8cb
        .quad   0x46115aba1d4dc0b3
        .quad   0xa66dcc9dc80c1ac0
        .quad   0x97a05cf41b38a436
        .quad   0xa7ebf3be95dbd7c6
        .quad   0x7da0b8f68d7e7dab
        .quad   0xd40f1953c3b5da76
        .quad   0x1dac6f7321119e9b
        .quad   0x03cc6021feb25960
        .quad   0x5a5f887e83674b4b

        // 2^192 * 2 * G

        .quad   0x8f6301cf70a13d11
        .quad   0xcfceb815350dd0c4
        .quad   0xf70297d4a4bca47e
        .quad   0x3669b656e44d1434
        .quad   0x9e9628d3a0a643b9
        .quad   0xb5c3cb00e6c32064
        .quad   0x9b5302897c2dec32
        .quad   0x43e37ae2d5d1c70c
        .quad   0x387e3f06eda6e133
        .quad   0x67301d5199a13ac0
        .quad   0xbd5ad8f836263811
        .quad   0x6a21e6cd4fd5e9be

        // 2^192 * 3 * G

        .quad   0xf1c6170a3046e65f
        .quad   0x58712a2a00d23524
        .quad   0x69dbbd3c8c82b755
        .quad   0x586bf9f1a195ff57
        .quad   0xef4129126699b2e3
        .quad   0x71d30847708d1301
        .quad   0x325432d01182b0bd
        .quad   0x45371b07001e8b36
        .quad   0xa6db088d5ef8790b
        .quad   0x5278f0dc610937e5
        .quad   0xac0349d261a16eb8
        .quad   0x0eafb03790e52179

        // 2^192 * 4 * G

        .quad   0x960555c13748042f
        .quad   0x219a41e6820baa11
        .quad   0x1c81f73873486d0c
        .quad   0x309acc675a02c661
        .quad   0x5140805e0f75ae1d
        .quad   0xec02fbe32662cc30
        .quad   0x2cebdf1eea92396d
        .quad   0x44ae3344c5435bb3
        .quad   0x9cf289b9bba543ee
        .quad   0xf3760e9d5ac97142
        .quad   0x1d82e5c64f9360aa
        .quad   0x62d5221b7f94678f

        // 2^192 * 5 * G

        .quad   0x524c299c18d0936d
        .quad   0xc86bb56c8a0c1a0c
        .quad   0xa375052edb4a8631
        .quad   0x5c0efde4bc754562
        .quad   0x7585d4263af77a3c
        .quad   0xdfae7b11fee9144d
        .quad   0xa506708059f7193d
        .quad   0x14f29a5383922037
        .quad   0xdf717edc25b2d7f5
        .quad   0x21f970db99b53040
        .quad   0xda9234b7c3ed4c62
        .quad   0x5e72365c7bee093e

        // 2^192 * 6 * G

        .quad   0x575bfc074571217f
        .quad   0x3779675d0694d95b
        .quad   0x9a0a37bbf4191e33
        .quad   0x77f1104c47b4eabc
        .quad   0x7d9339062f08b33e
        .quad   0x5b9659e5df9f32be
        .quad   0xacff3dad1f9ebdfd
        .quad   0x70b20555cb7349b7
        .quad   0xbe5113c555112c4c
        .quad   0x6688423a9a881fcd
        .quad   0x446677855e503b47
        .quad   0x0e34398f4a06404a

        // 2^192 * 7 * G

        .quad   0xb67d22d93ecebde8
        .quad   0x09b3e84127822f07
        .quad   0x743fa61fb05b6d8d
        .quad   0x5e5405368a362372
        .quad   0x18930b093e4b1928
        .quad   0x7de3e10e73f3f640
        .quad   0xf43217da73395d6f
        .quad   0x6f8aded6ca379c3e
        .quad   0xe340123dfdb7b29a
        .quad   0x487b97e1a21ab291
        .quad   0xf9967d02fde6949e
        .quad   0x780de72ec8d3de97

        // 2^192 * 8 * G

        .quad   0x0ae28545089ae7bc
        .quad   0x388ddecf1c7f4d06
        .quad   0x38ac15510a4811b8
        .quad   0x0eb28bf671928ce4
        .quad   0x671feaf300f42772
        .quad   0x8f72eb2a2a8c41aa
        .quad   0x29a17fd797373292
        .quad   0x1defc6ad32b587a6
        .quad   0xaf5bbe1aef5195a7
        .quad   0x148c1277917b15ed
        .quad   0x2991f7fb7ae5da2e
        .quad   0x467d201bf8dd2867

        // 2^196 * 1 * G

        .quad   0x7906ee72f7bd2e6b
        .quad   0x05d270d6109abf4e
        .quad   0x8d5cfe45b941a8a4
        .quad   0x44c218671c974287
        .quad   0x745f9d56296bc318
        .quad   0x993580d4d8152e65
        .quad   0xb0e5b13f5839e9ce
        .quad   0x51fc2b28d43921c0
        .quad   0x1b8fd11795e2a98c
        .quad   0x1c4e5ee12b6b6291
        .quad   0x5b30e7107424b572
        .quad   0x6e6b9de84c4f4ac6

        // 2^196 * 2 * G

        .quad   0xdff25fce4b1de151
        .quad   0xd841c0c7e11c4025
        .quad   0x2554b3c854749c87
        .quad   0x2d292459908e0df9
        .quad   0x6b7c5f10f80cb088
        .quad   0x736b54dc56e42151
        .quad   0xc2b620a5c6ef99c4
        .quad   0x5f4c802cc3a06f42
        .quad   0x9b65c8f17d0752da
        .quad   0x881ce338c77ee800
        .quad   0xc3b514f05b62f9e3
        .quad   0x66ed5dd5bec10d48

        // 2^196 * 3 * G

        .quad   0x7d38a1c20bb2089d
        .quad   0x808334e196ccd412
        .quad   0xc4a70b8c6c97d313
        .quad   0x2eacf8bc03007f20
        .quad   0xf0adf3c9cbca047d
        .quad   0x81c3b2cbf4552f6b
        .quad   0xcfda112d44735f93
        .quad   0x1f23a0c77e20048c
        .quad   0xf235467be5bc1570
        .quad   0x03d2d9020dbab38c
        .quad   0x27529aa2fcf9e09e
        .quad   0x0840bef29d34bc50

        // 2^196 * 4 * G

        .quad   0x796dfb35dc10b287
        .quad   0x27176bcd5c7ff29d
        .quad   0x7f3d43e8c7b24905
        .quad   0x0304f5a191c54276
        .quad   0xcd54e06b7f37e4eb
        .quad   0x8cc15f87f5e96cca
        .quad   0xb8248bb0d3597dce
        .quad   0x246affa06074400c
        .quad   0x37d88e68fbe45321
        .quad   0x86097548c0d75032
        .quad   0x4e9b13ef894a0d35
        .quad   0x25a83cac5753d325

        // 2^196 * 5 * G

        .quad   0x10222f48eed8165e
        .quad   0x623fc1234b8bcf3a
        .quad   0x1e145c09c221e8f0
        .quad   0x7ccfa59fca782630
        .quad   0x9f0f66293952b6e2
        .quad   0x33db5e0e0934267b
        .quad   0xff45252bd609fedc
        .quad   0x06be10f5c506e0c9
        .quad   0x1a9615a9b62a345f
        .quad   0x22050c564a52fecc
        .quad   0xa7a2788528bc0dfe
        .quad   0x5e82770a1a1ee71d

        // 2^196 * 6 * G

        .quad   0x35425183ad896a5c
        .quad   0xe8673afbe78d52f6
        .quad   0x2c66f25f92a35f64
        .quad   0x09d04f3b3b86b102
        .quad   0xe802e80a42339c74
        .quad   0x34175166a7fffae5
        .quad   0x34865d1f1c408cae
        .quad   0x2cca982c605bc5ee
        .quad   0xfd2d5d35197dbe6e
        .quad   0x207c2eea8be4ffa3
        .quad   0x2613d8db325ae918
        .quad   0x7a325d1727741d3e

        // 2^196 * 7 * G

        .quad   0xd036b9bbd16dfde2
        .quad   0xa2055757c497a829
        .quad   0x8e6cc966a7f12667
        .quad   0x4d3b1a791239c180
        .quad   0xecd27d017e2a076a
        .quad   0xd788689f1636495e
        .quad   0x52a61af0919233e5
        .quad   0x2a479df17bb1ae64
        .quad   0x9e5eee8e33db2710
        .quad   0x189854ded6c43ca5
        .quad   0xa41c22c592718138
        .quad   0x27ad5538a43a5e9b

        // 2^196 * 8 * G

        .quad   0x2746dd4b15350d61
        .quad   0xd03fcbc8ee9521b7
        .quad   0xe86e365a138672ca
        .quad   0x510e987f7e7d89e2
        .quad   0xcb5a7d638e47077c
        .quad   0x8db7536120a1c059
        .quad   0x549e1e4d8bedfdcc
        .quad   0x080153b7503b179d
        .quad   0xdda69d930a3ed3e3
        .quad   0x3d386ef1cd60a722
        .quad   0xc817ad58bdaa4ee6
        .quad   0x23be8d554fe7372a

        // 2^200 * 1 * G

        .quad   0x95fe919a74ef4fad
        .quad   0x3a827becf6a308a2
        .quad   0x964e01d309a47b01
        .quad   0x71c43c4f5ba3c797
        .quad   0xbc1ef4bd567ae7a9
        .quad   0x3f624cb2d64498bd
        .quad   0xe41064d22c1f4ec8
        .quad   0x2ef9c5a5ba384001
        .quad   0xb6fd6df6fa9e74cd
        .quad   0xf18278bce4af267a
        .quad   0x8255b3d0f1ef990e
        .quad   0x5a758ca390c5f293

        // 2^200 * 2 * G

        .quad   0xa2b72710d9462495
        .quad   0x3aa8c6d2d57d5003
        .quad   0xe3d400bfa0b487ca
        .quad   0x2dbae244b3eb72ec
        .quad   0x8ce0918b1d61dc94
        .quad   0x8ded36469a813066
        .quad   0xd4e6a829afe8aad3
        .quad   0x0a738027f639d43f
        .quad   0x980f4a2f57ffe1cc
        .quad   0x00670d0de1839843
        .quad   0x105c3f4a49fb15fd
        .quad   0x2698ca635126a69c

        // 2^200 * 3 * G

        .quad   0xe765318832b0ba78
        .quad   0x381831f7925cff8b
        .quad   0x08a81b91a0291fcc
        .quad   0x1fb43dcc49caeb07
        .quad   0x2e3d702f5e3dd90e
        .quad   0x9e3f0918e4d25386
        .quad   0x5e773ef6024da96a
        .quad   0x3c004b0c4afa3332
        .quad   0x9aa946ac06f4b82b
        .quad   0x1ca284a5a806c4f3
        .quad   0x3ed3265fc6cd4787
        .quad   0x6b43fd01cd1fd217

        // 2^200 * 4 * G

        .quad   0xc7a75d4b4697c544
        .quad   0x15fdf848df0fffbf
        .quad   0x2868b9ebaa46785a
        .quad   0x5a68d7105b52f714
        .quad   0xb5c742583e760ef3
        .quad   0x75dc52b9ee0ab990
        .quad   0xbf1427c2072b923f
        .quad   0x73420b2d6ff0d9f0
        .quad   0xaf2cf6cb9e851e06
        .quad   0x8f593913c62238c4
        .quad   0xda8ab89699fbf373
        .quad   0x3db5632fea34bc9e

        // 2^200 * 5 * G

        .quad   0xf46eee2bf75dd9d8
        .quad   0x0d17b1f6396759a5
        .quad   0x1bf2d131499e7273
        .quad   0x04321adf49d75f13
        .quad   0x2e4990b1829825d5
        .quad   0xedeaeb873e9a8991
        .quad   0xeef03d394c704af8
        .quad   0x59197ea495df2b0e
        .quad   0x04e16019e4e55aae
        .quad   0xe77b437a7e2f92e9
        .quad   0xc7ce2dc16f159aa4
        .quad   0x45eafdc1f4d70cc0

        // 2^200 * 6 * G

        .quad   0x698401858045d72b
        .quad   0x4c22faa2cf2f0651
        .quad   0x941a36656b222dc6
        .quad   0x5a5eebc80362dade
        .quad   0xb60e4624cfccb1ed
        .quad   0x59dbc292bd5c0395
        .quad   0x31a09d1ddc0481c9
        .quad   0x3f73ceea5d56d940
        .quad   0xb7a7bfd10a4e8dc6
        .quad   0xbe57007e44c9b339
        .quad   0x60c1207f1557aefa
        .quad   0x26058891266218db

        // 2^200 * 7 * G

        .quad   0x59f704a68360ff04
        .quad   0xc3d93fde7661e6f4
        .quad   0x831b2a7312873551
        .quad   0x54ad0c2e4e615d57
        .quad   0x4c818e3cc676e542
        .quad   0x5e422c9303ceccad
        .quad   0xec07cccab4129f08
        .quad   0x0dedfa10b24443b8
        .quad   0xee3b67d5b82b522a
        .quad   0x36f163469fa5c1eb
        .quad   0xa5b4d2f26ec19fd3
        .quad   0x62ecb2baa77a9408

        // 2^200 * 8 * G

        .quad   0xe5ed795261152b3d
        .quad   0x4962357d0eddd7d1
        .quad   0x7482c8d0b96b4c71
        .quad   0x2e59f919a966d8be
        .quad   0x92072836afb62874
        .quad   0x5fcd5e8579e104a5
        .quad   0x5aad01adc630a14a
        .quad   0x61913d5075663f98
        .quad   0x0dc62d361a3231da
        .quad   0xfa47583294200270
        .quad   0x02d801513f9594ce
        .quad   0x3ddbc2a131c05d5c

        // 2^204 * 1 * G

        .quad   0x3f50a50a4ffb81ef
        .quad   0xb1e035093bf420bf
        .quad   0x9baa8e1cc6aa2cd0
        .quad   0x32239861fa237a40
        .quad   0xfb735ac2004a35d1
        .quad   0x31de0f433a6607c3
        .quad   0x7b8591bfc528d599
        .quad   0x55be9a25f5bb050c
        .quad   0x0d005acd33db3dbf
        .quad   0x0111b37c80ac35e2
        .quad   0x4892d66c6f88ebeb
        .quad   0x770eadb16508fbcd

        // 2^204 * 2 * G

        .quad   0x8451f9e05e4e89dd
        .quad   0xc06302ffbc793937
        .quad   0x5d22749556a6495c
        .quad   0x09a6755ca05603fb
        .quad   0xf1d3b681a05071b9
        .quad   0x2207659a3592ff3a
        .quad   0x5f0169297881e40e
        .quad   0x16bedd0e86ba374e
        .quad   0x5ecccc4f2c2737b5
        .quad   0x43b79e0c2dccb703
        .quad   0x33e008bc4ec43df3
        .quad   0x06c1b840f07566c0

        // 2^204 * 3 * G

        .quad   0x7688a5c6a388f877
        .quad   0x02a96c14deb2b6ac
        .quad   0x64c9f3431b8c2af8
        .quad   0x3628435554a1eed6
        .quad   0x69ee9e7f9b02805c
        .quad   0xcbff828a547d1640
        .quad   0x3d93a869b2430968
        .quad   0x46b7b8cd3fe26972
        .quad   0xe9812086fe7eebe0
        .quad   0x4cba6be72f515437
        .quad   0x1d04168b516efae9
        .quad   0x5ea1391043982cb9

        // 2^204 * 4 * G

        .quad   0x49125c9cf4702ee1
        .quad   0x4520b71f8b25b32d
        .quad   0x33193026501fef7e
        .quad   0x656d8997c8d2eb2b
        .quad   0x6f2b3be4d5d3b002
        .quad   0xafec33d96a09c880
        .quad   0x035f73a4a8bcc4cc
        .quad   0x22c5b9284662198b
        .quad   0xcb58c8fe433d8939
        .quad   0x89a0cb2e6a8d7e50
        .quad   0x79ca955309fbbe5a
        .quad   0x0c626616cd7fc106

        // 2^204 * 5 * G

        .quad   0x1ffeb80a4879b61f
        .quad   0x6396726e4ada21ed
        .quad   0x33c7b093368025ba
        .quad   0x471aa0c6f3c31788
        .quad   0x8fdfc379fbf454b1
        .quad   0x45a5a970f1a4b771
        .quad   0xac921ef7bad35915
        .quad   0x42d088dca81c2192
        .quad   0x8fda0f37a0165199
        .quad   0x0adadb77c8a0e343
        .quad   0x20fbfdfcc875e820
        .quad   0x1cf2bea80c2206e7

        // 2^204 * 6 * G

        .quad   0xc2ddf1deb36202ac
        .quad   0x92a5fe09d2e27aa5
        .quad   0x7d1648f6fc09f1d3
        .quad   0x74c2cc0513bc4959
        .quad   0x982d6e1a02c0412f
        .quad   0x90fa4c83db58e8fe
        .quad   0x01c2f5bcdcb18bc0
        .quad   0x686e0c90216abc66
        .quad   0x1fadbadba54395a7
        .quad   0xb41a02a0ae0da66a
        .quad   0xbf19f598bba37c07
        .quad   0x6a12b8acde48430d

        // 2^204 * 7 * G

        .quad   0xf8daea1f39d495d9
        .quad   0x592c190e525f1dfc
        .quad   0xdb8cbd04c9991d1b
        .quad   0x11f7fda3d88f0cb7
        .quad   0x793bdd801aaeeb5f
        .quad   0x00a2a0aac1518871
        .quad   0xe8a373a31f2136b4
        .quad   0x48aab888fc91ef19
        .quad   0x041f7e925830f40e
        .quad   0x002d6ca979661c06
        .quad   0x86dc9ff92b046a2e
        .quad   0x760360928b0493d1

        // 2^204 * 8 * G

        .quad   0x21bb41c6120cf9c6
        .quad   0xeab2aa12decda59b
        .quad   0xc1a72d020aa48b34
        .quad   0x215d4d27e87d3b68
        .quad   0xb43108e5695a0b05
        .quad   0x6cb00ee8ad37a38b
        .quad   0x5edad6eea3537381
        .quad   0x3f2602d4b6dc3224
        .quad   0xc8b247b65bcaf19c
        .quad   0x49779dc3b1b2c652
        .quad   0x89a180bbd5ece2e2
        .quad   0x13f098a3cec8e039

        // 2^208 * 1 * G

        .quad   0x9adc0ff9ce5ec54b
        .quad   0x039c2a6b8c2f130d
        .quad   0x028007c7f0f89515
        .quad   0x78968314ac04b36b
        .quad   0xf3aa57a22796bb14
        .quad   0x883abab79b07da21
        .quad   0xe54be21831a0391c
        .quad   0x5ee7fb38d83205f9
        .quad   0x538dfdcb41446a8e
        .quad   0xa5acfda9434937f9
        .quad   0x46af908d263c8c78
        .quad   0x61d0633c9bca0d09

        // 2^208 * 2 * G

        .quad   0x63744935ffdb2566
        .quad   0xc5bd6b89780b68bb
        .quad   0x6f1b3280553eec03
        .quad   0x6e965fd847aed7f5
        .quad   0xada328bcf8fc73df
        .quad   0xee84695da6f037fc
        .quad   0x637fb4db38c2a909
        .quad   0x5b23ac2df8067bdc
        .quad   0x9ad2b953ee80527b
        .quad   0xe88f19aafade6d8d
        .quad   0x0e711704150e82cf
        .quad   0x79b9bbb9dd95dedc

        // 2^208 * 3 * G

        .quad   0xebb355406a3126c2
        .quad   0xd26383a868c8c393
        .quad   0x6c0c6429e5b97a82
        .quad   0x5065f158c9fd2147
        .quad   0xd1997dae8e9f7374
        .quad   0xa032a2f8cfbb0816
        .quad   0xcd6cba126d445f0a
        .quad   0x1ba811460accb834
        .quad   0x708169fb0c429954
        .quad   0xe14600acd76ecf67
        .quad   0x2eaab98a70e645ba
        .quad   0x3981f39e58a4faf2

        // 2^208 * 4 * G

        .quad   0x18fb8a7559230a93
        .quad   0x1d168f6960e6f45d
        .quad   0x3a85a94514a93cb5
        .quad   0x38dc083705acd0fd
        .quad   0xc845dfa56de66fde
        .quad   0xe152a5002c40483a
        .quad   0xe9d2e163c7b4f632
        .quad   0x30f4452edcbc1b65
        .quad   0x856d2782c5759740
        .quad   0xfa134569f99cbecc
        .quad   0x8844fc73c0ea4e71
        .quad   0x632d9a1a593f2469

        // 2^208 * 5 * G

        .quad   0xf6bb6b15b807cba6
        .quad   0x1823c7dfbc54f0d7
        .quad   0xbb1d97036e29670b
        .quad   0x0b24f48847ed4a57
        .quad   0xbf09fd11ed0c84a7
        .quad   0x63f071810d9f693a
        .quad   0x21908c2d57cf8779
        .quad   0x3a5a7df28af64ba2
        .quad   0xdcdad4be511beac7
        .quad   0xa4538075ed26ccf2
        .quad   0xe19cff9f005f9a65
        .quad   0x34fcf74475481f63

        // 2^208 * 6 * G

        .quad   0xc197e04c789767ca
        .quad   0xb8714dcb38d9467d
        .quad   0x55de888283f95fa8
        .quad   0x3d3bdc164dfa63f7
        .quad   0xa5bb1dab78cfaa98
        .quad   0x5ceda267190b72f2
        .quad   0x9309c9110a92608e
        .quad   0x0119a3042fb374b0
        .quad   0x67a2d89ce8c2177d
        .quad   0x669da5f66895d0c1
        .quad   0xf56598e5b282a2b0
        .quad   0x56c088f1ede20a73

        // 2^208 * 7 * G

        .quad   0x336d3d1110a86e17
        .quad   0xd7f388320b75b2fa
        .quad   0xf915337625072988
        .quad   0x09674c6b99108b87
        .quad   0x581b5fac24f38f02
        .quad   0xa90be9febae30cbd
        .quad   0x9a2169028acf92f0
        .quad   0x038b7ea48359038f
        .quad   0x9f4ef82199316ff8
        .quad   0x2f49d282eaa78d4f
        .quad   0x0971a5ab5aef3174
        .quad   0x6e5e31025969eb65

        // 2^208 * 8 * G

        .quad   0xb16c62f587e593fb
        .quad   0x4999eddeca5d3e71
        .quad   0xb491c1e014cc3e6d
        .quad   0x08f5114789a8dba8
        .quad   0x3304fb0e63066222
        .quad   0xfb35068987acba3f
        .quad   0xbd1924778c1061a3
        .quad   0x3058ad43d1838620
        .quad   0x323c0ffde57663d0
        .quad   0x05c3df38a22ea610
        .quad   0xbdc78abdac994f9a
        .quad   0x26549fa4efe3dc99

        // 2^212 * 1 * G

        .quad   0x738b38d787ce8f89
        .quad   0xb62658e24179a88d
        .quad   0x30738c9cf151316d
        .quad   0x49128c7f727275c9
        .quad   0x04dbbc17f75396b9
        .quad   0x69e6a2d7d2f86746
        .quad   0xc6409d99f53eabc6
        .quad   0x606175f6332e25d2
        .quad   0x4021370ef540e7dd
        .quad   0x0910d6f5a1f1d0a5
        .quad   0x4634aacd5b06b807
        .quad   0x6a39e6356944f235

        // 2^212 * 2 * G

        .quad   0x96cd5640df90f3e7
        .quad   0x6c3a760edbfa25ea
        .quad   0x24f3ef0959e33cc4
        .quad   0x42889e7e530d2e58
        .quad   0x1da1965774049e9d
        .quad   0xfbcd6ea198fe352b
        .quad   0xb1cbcd50cc5236a6
        .quad   0x1f5ec83d3f9846e2
        .quad   0x8efb23c3328ccb75
        .quad   0xaf42a207dd876ee9
        .quad   0x20fbdadc5dfae796
        .quad   0x241e246b06bf9f51

        // 2^212 * 3 * G

        .quad   0x29e68e57ad6e98f6
        .quad   0x4c9260c80b462065
        .quad   0x3f00862ea51ebb4b
        .quad   0x5bc2c77fb38d9097
        .quad   0x7eaafc9a6280bbb8
        .quad   0x22a70f12f403d809
        .quad   0x31ce40bb1bfc8d20
        .quad   0x2bc65635e8bd53ee
        .quad   0xe8d5dc9fa96bad93
        .quad   0xe58fb17dde1947dc
        .quad   0x681532ea65185fa3
        .quad   0x1fdd6c3b034a7830

        // 2^212 * 4 * G

        .quad   0x0a64e28c55dc18fe
        .quad   0xe3df9e993399ebdd
        .quad   0x79ac432370e2e652
        .quad   0x35ff7fc33ae4cc0e
        .quad   0x9c13a6a52dd8f7a9
        .quad   0x2dbb1f8c3efdcabf
        .quad   0x961e32405e08f7b5
        .quad   0x48c8a121bbe6c9e5
        .quad   0xfc415a7c59646445
        .quad   0xd224b2d7c128b615
        .quad   0x6035c9c905fbb912
        .quad   0x42d7a91274429fab

        // 2^212 * 5 * G

        .quad   0x4e6213e3eaf72ed3
        .quad   0x6794981a43acd4e7
        .quad   0xff547cde6eb508cb
        .quad   0x6fed19dd10fcb532
        .quad   0xa9a48947933da5bc
        .quad   0x4a58920ec2e979ec
        .quad   0x96d8800013e5ac4c
        .quad   0x453692d74b48b147
        .quad   0xdd775d99a8559c6f
        .quad   0xf42a2140df003e24
        .quad   0x5223e229da928a66
        .quad   0x063f46ba6d38f22c

        // 2^212 * 6 * G

        .quad   0xd2d242895f536694
        .quad   0xca33a2c542939b2c
        .quad   0x986fada6c7ddb95c
        .quad   0x5a152c042f712d5d
        .quad   0x39843cb737346921
        .quad   0xa747fb0738c89447
        .quad   0xcb8d8031a245307e
        .quad   0x67810f8e6d82f068
        .quad   0x3eeb8fbcd2287db4
        .quad   0x72c7d3a301a03e93
        .quad   0x5473e88cbd98265a
        .quad   0x7324aa515921b403

        // 2^212 * 7 * G

        .quad   0x857942f46c3cbe8e
        .quad   0xa1d364b14730c046
        .quad   0x1c8ed914d23c41bf
        .quad   0x0838e161eef6d5d2
        .quad   0xad23f6dae82354cb
        .quad   0x6962502ab6571a6d
        .quad   0x9b651636e38e37d1
        .quad   0x5cac5005d1a3312f
        .quad   0x8cc154cce9e39904
        .quad   0x5b3a040b84de6846
        .quad   0xc4d8a61cb1be5d6e
        .quad   0x40fb897bd8861f02

        // 2^212 * 8 * G

        .quad   0x84c5aa9062de37a1
        .quad   0x421da5000d1d96e1
        .quad   0x788286306a9242d9
        .quad   0x3c5e464a690d10da
        .quad   0xe57ed8475ab10761
        .quad   0x71435e206fd13746
        .quad   0x342f824ecd025632
        .quad   0x4b16281ea8791e7b
        .quad   0xd1c101d50b813381
        .quad   0xdee60f1176ee6828
        .quad   0x0cb68893383f6409
        .quad   0x6183c565f6ff484a

        // 2^216 * 1 * G

        .quad   0x741d5a461e6bf9d6
        .quad   0x2305b3fc7777a581
        .quad   0xd45574a26474d3d9
        .quad   0x1926e1dc6401e0ff
        .quad   0xdb468549af3f666e
        .quad   0xd77fcf04f14a0ea5
        .quad   0x3df23ff7a4ba0c47
        .quad   0x3a10dfe132ce3c85
        .quad   0xe07f4e8aea17cea0
        .quad   0x2fd515463a1fc1fd
        .quad   0x175322fd31f2c0f1
        .quad   0x1fa1d01d861e5d15

        // 2^216 * 2 * G

        .quad   0xcc8055947d599832
        .quad   0x1e4656da37f15520
        .quad   0x99f6f7744e059320
        .quad   0x773563bc6a75cf33
        .quad   0x38dcac00d1df94ab
        .quad   0x2e712bddd1080de9
        .quad   0x7f13e93efdd5e262
        .quad   0x73fced18ee9a01e5
        .quad   0x06b1e90863139cb3
        .quad   0xa493da67c5a03ecd
        .quad   0x8d77cec8ad638932
        .quad   0x1f426b701b864f44

        // 2^216 * 3 * G

        .quad   0xefc9264c41911c01
        .quad   0xf1a3b7b817a22c25
        .quad   0x5875da6bf30f1447
        .quad   0x4e1af5271d31b090
        .quad   0xf17e35c891a12552
        .quad   0xb76b8153575e9c76
        .quad   0xfa83406f0d9b723e
        .quad   0x0b76bb1b3fa7e438
        .quad   0x08b8c1f97f92939b
        .quad   0xbe6771cbd444ab6e
        .quad   0x22e5646399bb8017
        .quad   0x7b6dd61eb772a955

        // 2^216 * 4 * G

        .quad   0xb7adc1e850f33d92
        .quad   0x7998fa4f608cd5cf
        .quad   0xad962dbd8dfc5bdb
        .quad   0x703e9bceaf1d2f4f
        .quad   0x5730abf9ab01d2c7
        .quad   0x16fb76dc40143b18
        .quad   0x866cbe65a0cbb281
        .quad   0x53fa9b659bff6afe
        .quad   0x6c14c8e994885455
        .quad   0x843a5d6665aed4e5
        .quad   0x181bb73ebcd65af1
        .quad   0x398d93e5c4c61f50

        // 2^216 * 5 * G

        .quad   0x1c4bd16733e248f3
        .quad   0xbd9e128715bf0a5f
        .quad   0xd43f8cf0a10b0376
        .quad   0x53b09b5ddf191b13
        .quad   0xc3877c60d2e7e3f2
        .quad   0x3b34aaa030828bb1
        .quad   0x283e26e7739ef138
        .quad   0x699c9c9002c30577
        .quad   0xf306a7235946f1cc
        .quad   0x921718b5cce5d97d
        .quad   0x28cdd24781b4e975
        .quad   0x51caf30c6fcdd907

        // 2^216 * 6 * G

        .quad   0xa60ba7427674e00a
        .quad   0x630e8570a17a7bf3
        .quad   0x3758563dcf3324cc
        .quad   0x5504aa292383fdaa
        .quad   0x737af99a18ac54c7
        .quad   0x903378dcc51cb30f
        .quad   0x2b89bc334ce10cc7
        .quad   0x12ae29c189f8e99a
        .quad   0xa99ec0cb1f0d01cf
        .quad   0x0dd1efcc3a34f7ae
        .quad   0x55ca7521d09c4e22
        .quad   0x5fd14fe958eba5ea

        // 2^216 * 7 * G

        .quad   0xb5dc2ddf2845ab2c
        .quad   0x069491b10a7fe993
        .quad   0x4daaf3d64002e346
        .quad   0x093ff26e586474d1
        .quad   0x3c42fe5ebf93cb8e
        .quad   0xbedfa85136d4565f
        .quad   0xe0f0859e884220e8
        .quad   0x7dd73f960725d128
        .quad   0xb10d24fe68059829
        .quad   0x75730672dbaf23e5
        .quad   0x1367253ab457ac29
        .quad   0x2f59bcbc86b470a4

        // 2^216 * 8 * G

        .quad   0x83847d429917135f
        .quad   0xad1b911f567d03d7
        .quad   0x7e7748d9be77aad1
        .quad   0x5458b42e2e51af4a
        .quad   0x7041d560b691c301
        .quad   0x85201b3fadd7e71e
        .quad   0x16c2e16311335585
        .quad   0x2aa55e3d010828b1
        .quad   0xed5192e60c07444f
        .quad   0x42c54e2d74421d10
        .quad   0x352b4c82fdb5c864
        .quad   0x13e9004a8a768664

        // 2^220 * 1 * G

        .quad   0xcbb5b5556c032bff
        .quad   0xdf7191b729297a3a
        .quad   0xc1ff7326aded81bb
        .quad   0x71ade8bb68be03f5
        .quad   0x1e6284c5806b467c
        .quad   0xc5f6997be75d607b
        .quad   0x8b67d958b378d262
        .quad   0x3d88d66a81cd8b70
        .quad   0x8b767a93204ed789
        .quad   0x762fcacb9fa0ae2a
        .quad   0x771febcc6dce4887
        .quad   0x343062158ff05fb3

        // 2^220 * 2 * G

        .quad   0xe05da1a7e1f5bf49
        .quad   0x26457d6dd4736092
        .quad   0x77dcb07773cc32f6
        .quad   0x0a5d94969cdd5fcd
        .quad   0xfce219072a7b31b4
        .quad   0x4d7adc75aa578016
        .quad   0x0ec276a687479324
        .quad   0x6d6d9d5d1fda4beb
        .quad   0x22b1a58ae9b08183
        .quad   0xfd95d071c15c388b
        .quad   0xa9812376850a0517
        .quad   0x33384cbabb7f335e

        // 2^220 * 3 * G

        .quad   0x3c6fa2680ca2c7b5
        .quad   0x1b5082046fb64fda
        .quad   0xeb53349c5431d6de
        .quad   0x5278b38f6b879c89
        .quad   0x33bc627a26218b8d
        .quad   0xea80b21fc7a80c61
        .quad   0x9458b12b173e9ee6
        .quad   0x076247be0e2f3059
        .quad   0x52e105f61416375a
        .quad   0xec97af3685abeba4
        .quad   0x26e6b50623a67c36
        .quad   0x5cf0e856f3d4fb01

        // 2^220 * 4 * G

        .quad   0xf6c968731ae8cab4
        .quad   0x5e20741ecb4f92c5
        .quad   0x2da53be58ccdbc3e
        .quad   0x2dddfea269970df7
        .quad   0xbeaece313db342a8
        .quad   0xcba3635b842db7ee
        .quad   0xe88c6620817f13ef
        .quad   0x1b9438aa4e76d5c6
        .quad   0x8a50777e166f031a
        .quad   0x067b39f10fb7a328
        .quad   0x1925c9a6010fbd76
        .quad   0x6df9b575cc740905

        // 2^220 * 5 * G

        .quad   0x42c1192927f6bdcf
        .quad   0x8f91917a403d61ca
        .quad   0xdc1c5a668b9e1f61
        .quad   0x1596047804ec0f8d
        .quad   0xecdfc35b48cade41
        .quad   0x6a88471fb2328270
        .quad   0x740a4a2440a01b6a
        .quad   0x471e5796003b5f29
        .quad   0xda96bbb3aced37ac
        .quad   0x7a2423b5e9208cea
        .quad   0x24cc5c3038aebae2
        .quad   0x50c356afdc5dae2f

        // 2^220 * 6 * G

        .quad   0x09dcbf4341c30318
        .quad   0xeeba061183181dce
        .quad   0xc179c0cedc1e29a1
        .quad   0x1dbf7b89073f35b0
        .quad   0xcfed9cdf1b31b964
        .quad   0xf486a9858ca51af3
        .quad   0x14897265ea8c1f84
        .quad   0x784a53dd932acc00
        .quad   0x2d99f9df14fc4920
        .quad   0x76ccb60cc4499fe5
        .quad   0xa4132cbbe5cf0003
        .quad   0x3f93d82354f000ea

        // 2^220 * 7 * G

        .quad   0x8183e7689e04ce85
        .quad   0x678fb71e04465341
        .quad   0xad92058f6688edac
        .quad   0x5da350d3532b099a
        .quad   0xeaac12d179e14978
        .quad   0xff923ff3bbebff5e
        .quad   0x4af663e40663ce27
        .quad   0x0fd381a811a5f5ff
        .quad   0xf256aceca436df54
        .quad   0x108b6168ae69d6e8
        .quad   0x20d986cb6b5d036c
        .quad   0x655957b9fee2af50

        // 2^220 * 8 * G

        .quad   0xaea8b07fa902030f
        .quad   0xf88c766af463d143
        .quad   0x15b083663c787a60
        .quad   0x08eab1148267a4a8
        .quad   0xbdc1409bd002d0ac
        .quad   0x66660245b5ccd9a6
        .quad   0x82317dc4fade85ec
        .quad   0x02fe934b6ad7df0d
        .quad   0xef5cf100cfb7ea74
        .quad   0x22897633a1cb42ac
        .quad   0xd4ce0c54cef285e2
        .quad   0x30408c048a146a55

        // 2^224 * 1 * G

        .quad   0x739d8845832fcedb
        .quad   0xfa38d6c9ae6bf863
        .quad   0x32bc0dcab74ffef7
        .quad   0x73937e8814bce45e
        .quad   0xbb2e00c9193b877f
        .quad   0xece3a890e0dc506b
        .quad   0xecf3b7c036de649f
        .quad   0x5f46040898de9e1a
        .quad   0xb9037116297bf48d
        .quad   0xa9d13b22d4f06834
        .quad   0xe19715574696bdc6
        .quad   0x2cf8a4e891d5e835

        // 2^224 * 2 * G

        .quad   0x6d93fd8707110f67
        .quad   0xdd4c09d37c38b549
        .quad   0x7cb16a4cc2736a86
        .quad   0x2049bd6e58252a09
        .quad   0x2cb5487e17d06ba2
        .quad   0x24d2381c3950196b
        .quad   0xd7659c8185978a30
        .quad   0x7a6f7f2891d6a4f6
        .quad   0x7d09fd8d6a9aef49
        .quad   0xf0ee60be5b3db90b
        .quad   0x4c21b52c519ebfd4
        .quad   0x6011aadfc545941d

        // 2^224 * 3 * G

        .quad   0x5f67926dcf95f83c
        .quad   0x7c7e856171289071
        .quad   0xd6a1e7f3998f7a5b
        .quad   0x6fc5cc1b0b62f9e0
        .quad   0x63ded0c802cbf890
        .quad   0xfbd098ca0dff6aaa
        .quad   0x624d0afdb9b6ed99
        .quad   0x69ce18b779340b1e
        .quad   0xd1ef5528b29879cb
        .quad   0xdd1aae3cd47e9092
        .quad   0x127e0442189f2352
        .quad   0x15596b3ae57101f1

        // 2^224 * 4 * G

        .quad   0x462739d23f9179a2
        .quad   0xff83123197d6ddcf
        .quad   0x1307deb553f2148a
        .quad   0x0d2237687b5f4dda
        .quad   0x09ff31167e5124ca
        .quad   0x0be4158bd9c745df
        .quad   0x292b7d227ef556e5
        .quad   0x3aa4e241afb6d138
        .quad   0x2cc138bf2a3305f5
        .quad   0x48583f8fa2e926c3
        .quad   0x083ab1a25549d2eb
        .quad   0x32fcaa6e4687a36c

        // 2^224 * 5 * G

        .quad   0x7bc56e8dc57d9af5
        .quad   0x3e0bd2ed9df0bdf2
        .quad   0xaac014de22efe4a3
        .quad   0x4627e9cefebd6a5c
        .quad   0x3207a4732787ccdf
        .quad   0x17e31908f213e3f8
        .quad   0xd5b2ecd7f60d964e
        .quad   0x746f6336c2600be9
        .quad   0x3f4af345ab6c971c
        .quad   0xe288eb729943731f
        .quad   0x33596a8a0344186d
        .quad   0x7b4917007ed66293

        // 2^224 * 6 * G

        .quad   0x2d85fb5cab84b064
        .quad   0x497810d289f3bc14
        .quad   0x476adc447b15ce0c
        .quad   0x122ba376f844fd7b
        .quad   0x54341b28dd53a2dd
        .quad   0xaa17905bdf42fc3f
        .quad   0x0ff592d94dd2f8f4
        .quad   0x1d03620fe08cd37d
        .quad   0xc20232cda2b4e554
        .quad   0x9ed0fd42115d187f
        .quad   0x2eabb4be7dd479d9
        .quad   0x02c70bf52b68ec4c

        // 2^224 * 7 * G

        .quad   0xa287ec4b5d0b2fbb
        .quad   0x415c5790074882ca
        .quad   0xe044a61ec1d0815c
        .quad   0x26334f0a409ef5e0
        .quad   0xace532bf458d72e1
        .quad   0x5be768e07cb73cb5
        .quad   0x56cf7d94ee8bbde7
        .quad   0x6b0697e3feb43a03
        .quad   0xb6c8f04adf62a3c0
        .quad   0x3ef000ef076da45d
        .quad   0x9c9cb95849f0d2a9
        .quad   0x1cc37f43441b2fae

        // 2^224 * 8 * G

        .quad   0x508f565a5cc7324f
        .quad   0xd061c4c0e506a922
        .quad   0xfb18abdb5c45ac19
        .quad   0x6c6809c10380314a
        .quad   0xd76656f1c9ceaeb9
        .quad   0x1c5b15f818e5656a
        .quad   0x26e72832844c2334
        .quad   0x3a346f772f196838
        .quad   0xd2d55112e2da6ac8
        .quad   0xe9bd0331b1e851ed
        .quad   0x960746dd8ec67262
        .quad   0x05911b9f6ef7c5d0

        // 2^228 * 1 * G

        .quad   0xe9dcd756b637ff2d
        .quad   0xec4c348fc987f0c4
        .quad   0xced59285f3fbc7b7
        .quad   0x3305354793e1ea87
        .quad   0x01c18980c5fe9f94
        .quad   0xcd656769716fd5c8
        .quad   0x816045c3d195a086
        .quad   0x6e2b7f3266cc7982
        .quad   0xcc802468f7c3568f
        .quad   0x9de9ba8219974cb3
        .quad   0xabb7229cb5b81360
        .quad   0x44e2017a6fbeba62

        // 2^228 * 2 * G

        .quad   0xc4c2a74354dab774
        .quad   0x8e5d4c3c4eaf031a
        .quad   0xb76c23d242838f17
        .quad   0x749a098f68dce4ea
        .quad   0x87f82cf3b6ca6ecd
        .quad   0x580f893e18f4a0c2
        .quad   0x058930072604e557
        .quad   0x6cab6ac256d19c1d
        .quad   0xdcdfe0a02cc1de60
        .quad   0x032665ff51c5575b
        .quad   0x2c0c32f1073abeeb
        .quad   0x6a882014cd7b8606

        // 2^228 * 3 * G

        .quad   0xa52a92fea4747fb5
        .quad   0xdc12a4491fa5ab89
        .quad   0xd82da94bb847a4ce
        .quad   0x4d77edce9512cc4e
        .quad   0xd111d17caf4feb6e
        .quad   0x050bba42b33aa4a3
        .quad   0x17514c3ceeb46c30
        .quad   0x54bedb8b1bc27d75
        .quad   0x77c8e14577e2189c
        .quad   0xa3e46f6aff99c445
        .quad   0x3144dfc86d335343
        .quad   0x3a96559e7c4216a9

        // 2^228 * 4 * G

        .quad   0x12550d37f42ad2ee
        .quad   0x8b78e00498a1fbf5
        .quad   0x5d53078233894cb2
        .quad   0x02c84e4e3e498d0c
        .quad   0x4493896880baaa52
        .quad   0x4c98afc4f285940e
        .quad   0xef4aa79ba45448b6
        .quad   0x5278c510a57aae7f
        .quad   0xa54dd074294c0b94
        .quad   0xf55d46b8df18ffb6
        .quad   0xf06fecc58dae8366
        .quad   0x588657668190d165

        // 2^228 * 5 * G

        .quad   0xd47712311aef7117
        .quad   0x50343101229e92c7
        .quad   0x7a95e1849d159b97
        .quad   0x2449959b8b5d29c9
        .quad   0xbf5834f03de25cc3
        .quad   0xb887c8aed6815496
        .quad   0x5105221a9481e892
        .quad   0x6760ed19f7723f93
        .quad   0x669ba3b7ac35e160
        .quad   0x2eccf73fba842056
        .quad   0x1aec1f17c0804f07
        .quad   0x0d96bc031856f4e7

        // 2^228 * 6 * G

        .quad   0x3318be7775c52d82
        .quad   0x4cb764b554d0aab9
        .quad   0xabcf3d27cc773d91
        .quad   0x3bf4d1848123288a
        .quad   0xb1d534b0cc7505e1
        .quad   0x32cd003416c35288
        .quad   0xcb36a5800762c29d
        .quad   0x5bfe69b9237a0bf8
        .quad   0x183eab7e78a151ab
        .quad   0xbbe990c999093763
        .quad   0xff717d6e4ac7e335
        .quad   0x4c5cddb325f39f88

        // 2^228 * 7 * G

        .quad   0xc0f6b74d6190a6eb
        .quad   0x20ea81a42db8f4e4
        .quad   0xa8bd6f7d97315760
        .quad   0x33b1d60262ac7c21
        .quad   0x57750967e7a9f902
        .quad   0x2c37fdfc4f5b467e
        .quad   0xb261663a3177ba46
        .quad   0x3a375e78dc2d532b
        .quad   0x8141e72f2d4dddea
        .quad   0xe6eafe9862c607c8
        .quad   0x23c28458573cafd0
        .quad   0x46b9476f4ff97346

        // 2^228 * 8 * G

        .quad   0x0c1ffea44f901e5c
        .quad   0x2b0b6fb72184b782
        .quad   0xe587ff910114db88
        .quad   0x37130f364785a142
        .quad   0x1215505c0d58359f
        .quad   0x2a2013c7fc28c46b
        .quad   0x24a0a1af89ea664e
        .quad   0x4400b638a1130e1f
        .quad   0x3a01b76496ed19c3
        .quad   0x31e00ab0ed327230
        .quad   0x520a885783ca15b1
        .quad   0x06aab9875accbec7

        // 2^232 * 1 * G

        .quad   0xc1339983f5df0ebb
        .quad   0xc0f3758f512c4cac
        .quad   0x2cf1130a0bb398e1
        .quad   0x6b3cecf9aa270c62
        .quad   0x5349acf3512eeaef
        .quad   0x20c141d31cc1cb49
        .quad   0x24180c07a99a688d
        .quad   0x555ef9d1c64b2d17
        .quad   0x36a770ba3b73bd08
        .quad   0x624aef08a3afbf0c
        .quad   0x5737ff98b40946f2
        .quad   0x675f4de13381749d

        // 2^232 * 2 * G

        .quad   0x0e2c52036b1782fc
        .quad   0x64816c816cad83b4
        .quad   0xd0dcbdd96964073e
        .quad   0x13d99df70164c520
        .quad   0xa12ff6d93bdab31d
        .quad   0x0725d80f9d652dfe
        .quad   0x019c4ff39abe9487
        .quad   0x60f450b882cd3c43
        .quad   0x014b5ec321e5c0ca
        .quad   0x4fcb69c9d719bfa2
        .quad   0x4e5f1c18750023a0
        .quad   0x1c06de9e55edac80

        // 2^232 * 3 * G

        .quad   0x990f7ad6a33ec4e2
        .quad   0x6608f938be2ee08e
        .quad   0x9ca143c563284515
        .quad   0x4cf38a1fec2db60d
        .quad   0xffd52b40ff6d69aa
        .quad   0x34530b18dc4049bb
        .quad   0x5e4a5c2fa34d9897
        .quad   0x78096f8e7d32ba2d
        .quad   0xa0aaaa650dfa5ce7
        .quad   0xf9c49e2a48b5478c
        .quad   0x4f09cc7d7003725b
        .quad   0x373cad3a26091abe

        // 2^232 * 4 * G

        .quad   0xb294634d82c9f57c
        .quad   0x1fcbfde124934536
        .quad   0x9e9c4db3418cdb5a
        .quad   0x0040f3d9454419fc
        .quad   0xf1bea8fb89ddbbad
        .quad   0x3bcb2cbc61aeaecb
        .quad   0x8f58a7bb1f9b8d9d
        .quad   0x21547eda5112a686
        .quad   0xdefde939fd5986d3
        .quad   0xf4272c89510a380c
        .quad   0xb72ba407bb3119b9
        .quad   0x63550a334a254df4

        // 2^232 * 5 * G

        .quad   0x6507d6edb569cf37
        .quad   0x178429b00ca52ee1
        .quad   0xea7c0090eb6bd65d
        .quad   0x3eea62c7daf78f51
        .quad   0x9bba584572547b49
        .quad   0xf305c6fae2c408e0
        .quad   0x60e8fa69c734f18d
        .quad   0x39a92bafaa7d767a
        .quad   0x9d24c713e693274e
        .quad   0x5f63857768dbd375
        .quad   0x70525560eb8ab39a
        .quad   0x68436a0665c9c4cd

        // 2^232 * 6 * G

        .quad   0xbc0235e8202f3f27
        .quad   0xc75c00e264f975b0
        .quad   0x91a4e9d5a38c2416
        .quad   0x17b6e7f68ab789f9
        .quad   0x1e56d317e820107c
        .quad   0xc5266844840ae965
        .quad   0xc1e0a1c6320ffc7a
        .quad   0x5373669c91611472
        .quad   0x5d2814ab9a0e5257
        .quad   0x908f2084c9cab3fc
        .quad   0xafcaf5885b2d1eca
        .quad   0x1cb4b5a678f87d11

        // 2^232 * 7 * G

        .quad   0xb664c06b394afc6c
        .quad   0x0c88de2498da5fb1
        .quad   0x4f8d03164bcad834
        .quad   0x330bca78de7434a2
        .quad   0x6b74aa62a2a007e7
        .quad   0xf311e0b0f071c7b1
        .quad   0x5707e438000be223
        .quad   0x2dc0fd2d82ef6eac
        .quad   0x982eff841119744e
        .quad   0xf9695e962b074724
        .quad   0xc58ac14fbfc953fb
        .quad   0x3c31be1b369f1cf5

        // 2^232 * 8 * G

        .quad   0xb0f4864d08948aee
        .quad   0x07dc19ee91ba1c6f
        .quad   0x7975cdaea6aca158
        .quad   0x330b61134262d4bb
        .quad   0xc168bc93f9cb4272
        .quad   0xaeb8711fc7cedb98
        .quad   0x7f0e52aa34ac8d7a
        .quad   0x41cec1097e7d55bb
        .quad   0xf79619d7a26d808a
        .quad   0xbb1fd49e1d9e156d
        .quad   0x73d7c36cdba1df27
        .quad   0x26b44cd91f28777d

        // 2^236 * 1 * G

        .quad   0x300a9035393aa6d8
        .quad   0x2b501131a12bb1cd
        .quad   0x7b1ff677f093c222
        .quad   0x4309c1f8cab82bad
        .quad   0xaf44842db0285f37
        .quad   0x8753189047efc8df
        .quad   0x9574e091f820979a
        .quad   0x0e378d6069615579
        .quad   0xd9fa917183075a55
        .quad   0x4bdb5ad26b009fdc
        .quad   0x7829ad2cd63def0e
        .quad   0x078fc54975fd3877

        // 2^236 * 2 * G

        .quad   0x87dfbd1428878f2d
        .quad   0x134636dd1e9421a1
        .quad   0x4f17c951257341a3
        .quad   0x5df98d4bad296cb8
        .quad   0xe2004b5bb833a98a
        .quad   0x44775dec2d4c3330
        .quad   0x3aa244067eace913
        .quad   0x272630e3d58e00a9
        .quad   0xf3678fd0ecc90b54
        .quad   0xf001459b12043599
        .quad   0x26725fbc3758b89b
        .quad   0x4325e4aa73a719ae

        // 2^236 * 3 * G

        .quad   0x657dc6ef433c3493
        .quad   0x65375e9f80dbf8c3
        .quad   0x47fd2d465b372dae
        .quad   0x4966ab79796e7947
        .quad   0xed24629acf69f59d
        .quad   0x2a4a1ccedd5abbf4
        .quad   0x3535ca1f56b2d67b
        .quad   0x5d8c68d043b1b42d
        .quad   0xee332d4de3b42b0a
        .quad   0xd84e5a2b16a4601c
        .quad   0x78243877078ba3e4
        .quad   0x77ed1eb4184ee437

        // 2^236 * 4 * G

        .quad   0xbfd4e13f201839a0
        .quad   0xaeefffe23e3df161
        .quad   0xb65b04f06b5d1fe3
        .quad   0x52e085fb2b62fbc0
        .quad   0x185d43f89e92ed1a
        .quad   0xb04a1eeafe4719c6
        .quad   0x499fbe88a6f03f4f
        .quad   0x5d8b0d2f3c859bdd
        .quad   0x124079eaa54cf2ba
        .quad   0xd72465eb001b26e7
        .quad   0x6843bcfdc97af7fd
        .quad   0x0524b42b55eacd02

        // 2^236 * 5 * G

        .quad   0xfd0d5dbee45447b0
        .quad   0x6cec351a092005ee
        .quad   0x99a47844567579cb
        .quad   0x59d242a216e7fa45
        .quad   0xbc18dcad9b829eac
        .quad   0x23ae7d28b5f579d0
        .quad   0xc346122a69384233
        .quad   0x1a6110b2e7d4ac89
        .quad   0x4f833f6ae66997ac
        .quad   0x6849762a361839a4
        .quad   0x6985dec1970ab525
        .quad   0x53045e89dcb1f546

        // 2^236 * 6 * G

        .quad   0xcb8bb346d75353db
        .quad   0xfcfcb24bae511e22
        .quad   0xcba48d40d50ae6ef
        .quad   0x26e3bae5f4f7cb5d
        .quad   0x84da3cde8d45fe12
        .quad   0xbd42c218e444e2d2
        .quad   0xa85196781f7e3598
        .quad   0x7642c93f5616e2b2
        .quad   0x2323daa74595f8e4
        .quad   0xde688c8b857abeb4
        .quad   0x3fc48e961c59326e
        .quad   0x0b2e73ca15c9b8ba

        // 2^236 * 7 * G

        .quad   0xd6bb4428c17f5026
        .quad   0x9eb27223fb5a9ca7
        .quad   0xe37ba5031919c644
        .quad   0x21ce380db59a6602
        .quad   0x0e3fbfaf79c03a55
        .quad   0x3077af054cbb5acf
        .quad   0xd5c55245db3de39f
        .quad   0x015e68c1476a4af7
        .quad   0xc1d5285220066a38
        .quad   0x95603e523570aef3
        .quad   0x832659a7226b8a4d
        .quad   0x5dd689091f8eedc9

        // 2^236 * 8 * G

        .quad   0xcbac84debfd3c856
        .quad   0x1624c348b35ff244
        .quad   0xb7f88dca5d9cad07
        .quad   0x3b0e574da2c2ebe8
        .quad   0x1d022591a5313084
        .quad   0xca2d4aaed6270872
        .quad   0x86a12b852f0bfd20
        .quad   0x56e6c439ad7da748
        .quad   0xc704ff4942bdbae6
        .quad   0x5e21ade2b2de1f79
        .quad   0xe95db3f35652fad8
        .quad   0x0822b5378f08ebc1

        // 2^240 * 1 * G

        .quad   0x51f048478f387475
        .quad   0xb25dbcf49cbecb3c
        .quad   0x9aab1244d99f2055
        .quad   0x2c709e6c1c10a5d6
        .quad   0xe1b7f29362730383
        .quad   0x4b5279ffebca8a2c
        .quad   0xdafc778abfd41314
        .quad   0x7deb10149c72610f
        .quad   0xcb62af6a8766ee7a
        .quad   0x66cbec045553cd0e
        .quad   0x588001380f0be4b5
        .quad   0x08e68e9ff62ce2ea

        // 2^240 * 2 * G

        .quad   0x34ad500a4bc130ad
        .quad   0x8d38db493d0bd49c
        .quad   0xa25c3d98500a89be
        .quad   0x2f1f3f87eeba3b09
        .quad   0x2f2d09d50ab8f2f9
        .quad   0xacb9218dc55923df
        .quad   0x4a8f342673766cb9
        .quad   0x4cb13bd738f719f5
        .quad   0xf7848c75e515b64a
        .quad   0xa59501badb4a9038
        .quad   0xc20d313f3f751b50
        .quad   0x19a1e353c0ae2ee8

        // 2^240 * 3 * G

        .quad   0x7d1c7560bafa05c3
        .quad   0xb3e1a0a0c6e55e61
        .quad   0xe3529718c0d66473
        .quad   0x41546b11c20c3486
        .quad   0xb42172cdd596bdbd
        .quad   0x93e0454398eefc40
        .quad   0x9fb15347b44109b5
        .quad   0x736bd3990266ae34
        .quad   0x85532d509334b3b4
        .quad   0x46fd114b60816573
        .quad   0xcc5f5f30425c8375
        .quad   0x412295a2b87fab5c

        // 2^240 * 4 * G

        .quad   0x19c99b88f57ed6e9
        .quad   0x5393cb266df8c825
        .quad   0x5cee3213b30ad273
        .quad   0x14e153ebb52d2e34
        .quad   0x2e655261e293eac6
        .quad   0x845a92032133acdb
        .quad   0x460975cb7900996b
        .quad   0x0760bb8d195add80
        .quad   0x413e1a17cde6818a
        .quad   0x57156da9ed69a084
        .quad   0x2cbf268f46caccb1
        .quad   0x6b34be9bc33ac5f2

        // 2^240 * 5 * G

        .quad   0xf3df2f643a78c0b2
        .quad   0x4c3e971ef22e027c
        .quad   0xec7d1c5e49c1b5a3
        .quad   0x2012c18f0922dd2d
        .quad   0x11fc69656571f2d3
        .quad   0xc6c9e845530e737a
        .quad   0xe33ae7a2d4fe5035
        .quad   0x01b9c7b62e6dd30b
        .quad   0x880b55e55ac89d29
        .quad   0x1483241f45a0a763
        .quad   0x3d36efdfc2e76c1f
        .quad   0x08af5b784e4bade8

        // 2^240 * 6 * G

        .quad   0x283499dc881f2533
        .quad   0x9d0525da779323b6
        .quad   0x897addfb673441f4
        .quad   0x32b79d71163a168d
        .quad   0xe27314d289cc2c4b
        .quad   0x4be4bd11a287178d
        .quad   0x18d528d6fa3364ce
        .quad   0x6423c1d5afd9826e
        .quad   0xcc85f8d9edfcb36a
        .quad   0x22bcc28f3746e5f9
        .quad   0xe49de338f9e5d3cd
        .quad   0x480a5efbc13e2dcc

        // 2^240 * 7 * G

        .quad   0x0b51e70b01622071
        .quad   0x06b505cf8b1dafc5
        .quad   0x2c6bb061ef5aabcd
        .quad   0x47aa27600cb7bf31
        .quad   0xb6614ce442ce221f
        .quad   0x6e199dcc4c053928
        .quad   0x663fb4a4dc1cbe03
        .quad   0x24b31d47691c8e06
        .quad   0x2a541eedc015f8c3
        .quad   0x11a4fe7e7c693f7c
        .quad   0xf0af66134ea278d6
        .quad   0x545b585d14dda094

        // 2^240 * 8 * G

        .quad   0x67bf275ea0d43a0f
        .quad   0xade68e34089beebe
        .quad   0x4289134cd479e72e
        .quad   0x0f62f9c332ba5454
        .quad   0x6204e4d0e3b321e1
        .quad   0x3baa637a28ff1e95
        .quad   0x0b0ccffd5b99bd9e
        .quad   0x4d22dc3e64c8d071
        .quad   0xfcb46589d63b5f39
        .quad   0x5cae6a3f57cbcf61
        .quad   0xfebac2d2953afa05
        .quad   0x1c0fa01a36371436

        // 2^244 * 1 * G

        .quad   0xe7547449bc7cd692
        .quad   0x0f9abeaae6f73ddf
        .quad   0x4af01ca700837e29
        .quad   0x63ab1b5d3f1bc183
        .quad   0xc11ee5e854c53fae
        .quad   0x6a0b06c12b4f3ff4
        .quad   0x33540f80e0b67a72
        .quad   0x15f18fc3cd07e3ef
        .quad   0x32750763b028f48c
        .quad   0x06020740556a065f
        .quad   0xd53bd812c3495b58
        .quad   0x08706c9b865f508d

        // 2^244 * 2 * G

        .quad   0xf37ca2ab3d343dff
        .quad   0x1a8c6a2d80abc617
        .quad   0x8e49e035d4ccffca
        .quad   0x48b46beebaa1d1b9
        .quad   0xcc991b4138b41246
        .quad   0x243b9c526f9ac26b
        .quad   0xb9ef494db7cbabbd
        .quad   0x5fba433dd082ed00
        .quad   0x9c49e355c9941ad0
        .quad   0xb9734ade74498f84
        .quad   0x41c3fed066663e5c
        .quad   0x0ecfedf8e8e710b3

        // 2^244 * 3 * G

        .quad   0x76430f9f9cd470d9
        .quad   0xb62acc9ba42f6008
        .quad   0x1898297c59adad5e
        .quad   0x7789dd2db78c5080
        .quad   0x744f7463e9403762
        .quad   0xf79a8dee8dfcc9c9
        .quad   0x163a649655e4cde3
        .quad   0x3b61788db284f435
        .quad   0xb22228190d6ef6b2
        .quad   0xa94a66b246ce4bfa
        .quad   0x46c1a77a4f0b6cc7
        .quad   0x4236ccffeb7338cf

        // 2^244 * 4 * G

        .quad   0x8497404d0d55e274
        .quad   0x6c6663d9c4ad2b53
        .quad   0xec2fb0d9ada95734
        .quad   0x2617e120cdb8f73c
        .quad   0x3bd82dbfda777df6
        .quad   0x71b177cc0b98369e
        .quad   0x1d0e8463850c3699
        .quad   0x5a71945b48e2d1f1
        .quad   0x6f203dd5405b4b42
        .quad   0x327ec60410b24509
        .quad   0x9c347230ac2a8846
        .quad   0x77de29fc11ffeb6a

        // 2^244 * 5 * G

        .quad   0xb0ac57c983b778a8
        .quad   0x53cdcca9d7fe912c
        .quad   0x61c2b854ff1f59dc
        .quad   0x3a1a2cf0f0de7dac
        .quad   0x835e138fecced2ca
        .quad   0x8c9eaf13ea963b9a
        .quad   0xc95fbfc0b2160ea6
        .quad   0x575e66f3ad877892
        .quad   0x99803a27c88fcb3a
        .quad   0x345a6789275ec0b0
        .quad   0x459789d0ff6c2be5
        .quad   0x62f882651e70a8b2

        // 2^244 * 6 * G

        .quad   0x085ae2c759ff1be4
        .quad   0x149145c93b0e40b7
        .quad   0xc467e7fa7ff27379
        .quad   0x4eeecf0ad5c73a95
        .quad   0x6d822986698a19e0
        .quad   0xdc9821e174d78a71
        .quad   0x41a85f31f6cb1f47
        .quad   0x352721c2bcda9c51
        .quad   0x48329952213fc985
        .quad   0x1087cf0d368a1746
        .quad   0x8e5261b166c15aa5
        .quad   0x2d5b2d842ed24c21

        // 2^244 * 7 * G

        .quad   0x02cfebd9ebd3ded1
        .quad   0xd45b217739021974
        .quad   0x7576f813fe30a1b7
        .quad   0x5691b6f9a34ef6c2
        .quad   0x5eb7d13d196ac533
        .quad   0x377234ecdb80be2b
        .quad   0xe144cffc7cf5ae24
        .quad   0x5226bcf9c441acec
        .quad   0x79ee6c7223e5b547
        .quad   0x6f5f50768330d679
        .quad   0xed73e1e96d8adce9
        .quad   0x27c3da1e1d8ccc03

        // 2^244 * 8 * G

        .quad   0x7eb9efb23fe24c74
        .quad   0x3e50f49f1651be01
        .quad   0x3ea732dc21858dea
        .quad   0x17377bd75bb810f9
        .quad   0x28302e71630ef9f6
        .quad   0xc2d4a2032b64cee0
        .quad   0x090820304b6292be
        .quad   0x5fca747aa82adf18
        .quad   0x232a03c35c258ea5
        .quad   0x86f23a2c6bcb0cf1
        .quad   0x3dad8d0d2e442166
        .quad   0x04a8933cab76862b

        // 2^248 * 1 * G

        .quad   0xd2c604b622943dff
        .quad   0xbc8cbece44cfb3a0
        .quad   0x5d254ff397808678
        .quad   0x0fa3614f3b1ca6bf
        .quad   0x69082b0e8c936a50
        .quad   0xf9c9a035c1dac5b6
        .quad   0x6fb73e54c4dfb634
        .quad   0x4005419b1d2bc140
        .quad   0xa003febdb9be82f0
        .quad   0x2089c1af3a44ac90
        .quad   0xf8499f911954fa8e
        .quad   0x1fba218aef40ab42

        // 2^248 * 2 * G

        .quad   0xab549448fac8f53e
        .quad   0x81f6e89a7ba63741
        .quad   0x74fd6c7d6c2b5e01
        .quad   0x392e3acaa8c86e42
        .quad   0x4f3e57043e7b0194
        .quad   0xa81d3eee08daaf7f
        .quad   0xc839c6ab99dcdef1
        .quad   0x6c535d13ff7761d5
        .quad   0x4cbd34e93e8a35af
        .quad   0x2e0781445887e816
        .quad   0x19319c76f29ab0ab
        .quad   0x25e17fe4d50ac13b

        // 2^248 * 3 * G

        .quad   0x0a289bd71e04f676
        .quad   0x208e1c52d6420f95
        .quad   0x5186d8b034691fab
        .quad   0x255751442a9fb351
        .quad   0x915f7ff576f121a7
        .quad   0xc34a32272fcd87e3
        .quad   0xccba2fde4d1be526
        .quad   0x6bba828f8969899b
        .quad   0xe2d1bc6690fe3901
        .quad   0x4cb54a18a0997ad5
        .quad   0x971d6914af8460d4
        .quad   0x559d504f7f6b7be4

        // 2^248 * 4 * G

        .quad   0xa7738378b3eb54d5
        .quad   0x1d69d366a5553c7c
        .quad   0x0a26cf62f92800ba
        .quad   0x01ab12d5807e3217
        .quad   0x9c4891e7f6d266fd
        .quad   0x0744a19b0307781b
        .quad   0x88388f1d6061e23b
        .quad   0x123ea6a3354bd50e
        .quad   0x118d189041e32d96
        .quad   0xb9ede3c2d8315848
        .quad   0x1eab4271d83245d9
        .quad   0x4a3961e2c918a154

        // 2^248 * 5 * G

        .quad   0x71dc3be0f8e6bba0
        .quad   0xd6cef8347effe30a
        .quad   0xa992425fe13a476a
        .quad   0x2cd6bce3fb1db763
        .quad   0x0327d644f3233f1e
        .quad   0x499a260e34fcf016
        .quad   0x83b5a716f2dab979
        .quad   0x68aceead9bd4111f
        .quad   0x38b4c90ef3d7c210
        .quad   0x308e6e24b7ad040c
        .quad   0x3860d9f1b7e73e23
        .quad   0x595760d5b508f597

        // 2^248 * 6 * G

        .quad   0x6129bfe104aa6397
        .quad   0x8f960008a4a7fccb
        .quad   0x3f8bc0897d909458
        .quad   0x709fa43edcb291a9
        .quad   0x882acbebfd022790
        .quad   0x89af3305c4115760
        .quad   0x65f492e37d3473f4
        .quad   0x2cb2c5df54515a2b
        .quad   0xeb0a5d8c63fd2aca
        .quad   0xd22bc1662e694eff
        .quad   0x2723f36ef8cbb03a
        .quad   0x70f029ecf0c8131f

        // 2^248 * 7 * G

        .quad   0x461307b32eed3e33
        .quad   0xae042f33a45581e7
        .quad   0xc94449d3195f0366
        .quad   0x0b7d5d8a6c314858
        .quad   0x2a6aafaa5e10b0b9
        .quad   0x78f0a370ef041aa9
        .quad   0x773efb77aa3ad61f
        .quad   0x44eca5a2a74bd9e1
        .quad   0x25d448327b95d543
        .quad   0x70d38300a3340f1d
        .quad   0xde1c531c60e1c52b
        .quad   0x272224512c7de9e4

        // 2^248 * 8 * G

        .quad   0x1abc92af49c5342e
        .quad   0xffeed811b2e6fad0
        .quad   0xefa28c8dfcc84e29
        .quad   0x11b5df18a44cc543
        .quad   0xbf7bbb8a42a975fc
        .quad   0x8c5c397796ada358
        .quad   0xe27fc76fcdedaa48
        .quad   0x19735fd7f6bc20a6
        .quad   0xe3ab90d042c84266
        .quad   0xeb848e0f7f19547e
        .quad   0x2503a1d065a497b9
        .quad   0x0fef911191df895f

        // 2^252 * 1 * G

        .quad   0xb1507ca1ab1c6eb9
        .quad   0xbd448f3e16b687b3
        .quad   0x3455fb7f2c7a91ab
        .quad   0x7579229e2f2adec1
        .quad   0x6ab5dcb85b1c16b7
        .quad   0x94c0fce83c7b27a5
        .quad   0xa4b11c1a735517be
        .quad   0x499238d0ba0eafaa
        .quad   0xecf46e527aba8b57
        .quad   0x15a08c478bd1647b
        .quad   0x7af1c6a65f706fef
        .quad   0x6345fa78f03a30d5

        // 2^252 * 2 * G

        .quad   0xdf02f95f1015e7a1
        .quad   0x790ec41da9b40263
        .quad   0x4d3a0ea133ea1107
        .quad   0x54f70be7e33af8c9
        .quad   0x93d3cbe9bdd8f0a4
        .quad   0xdb152c1bfd177302
        .quad   0x7dbddc6d7f17a875
        .quad   0x3e1a71cc8f426efe
        .quad   0xc83ca3e390babd62
        .quad   0x80ede3670291c833
        .quad   0xc88038ccd37900c4
        .quad   0x2c5fc0231ec31fa1

        // 2^252 * 3 * G

        .quad   0xfeba911717038b4f
        .quad   0xe5123721c9deef81
        .quad   0x1c97e4e75d0d8834
        .quad   0x68afae7a23dc3bc6
        .quad   0xc422e4d102456e65
        .quad   0x87414ac1cad47b91
        .quad   0x1592e2bba2b6ffdd
        .quad   0x75d9d2bff5c2100f
        .quad   0x5bd9b4763626e81c
        .quad   0x89966936bca02edd
        .quad   0x0a41193d61f077b3
        .quad   0x3097a24200ce5471

        // 2^252 * 4 * G

        .quad   0x57427734c7f8b84c
        .quad   0xf141a13e01b270e9
        .quad   0x02d1adfeb4e564a6
        .quad   0x4bb23d92ce83bd48
        .quad   0xa162e7246695c486
        .quad   0x131d633435a89607
        .quad   0x30521561a0d12a37
        .quad   0x56704bada6afb363
        .quad   0xaf6c4aa752f912b9
        .quad   0x5e665f6cd86770c8
        .quad   0x4c35ac83a3c8cd58
        .quad   0x2b7a29c010a58a7e

        // 2^252 * 5 * G

        .quad   0xc4007f77d0c1cec3
        .quad   0x8d1020b6bac492f8
        .quad   0x32ec29d57e69daaf
        .quad   0x599408759d95fce0
        .quad   0x33810a23bf00086e
        .quad   0xafce925ee736ff7c
        .quad   0x3d60e670e24922d4
        .quad   0x11ce9e714f96061b
        .quad   0x219ef713d815bac1
        .quad   0xf141465d485be25c
        .quad   0x6d5447cc4e513c51
        .quad   0x174926be5ef44393

        // 2^252 * 6 * G

        .quad   0xb5deb2f9fc5bd5bb
        .quad   0x92daa72ae1d810e1
        .quad   0xafc4cfdcb72a1c59
        .quad   0x497d78813fc22a24
        .quad   0x3ef5d41593ea022e
        .quad   0x5cbcc1a20ed0eed6
        .quad   0x8fd24ecf07382c8c
        .quad   0x6fa42ead06d8e1ad
        .quad   0xe276824a1f73371f
        .quad   0x7f7cf01c4f5b6736
        .quad   0x7e201fe304fa46e7
        .quad   0x785a36a357808c96

        // 2^252 * 7 * G

        .quad   0x825fbdfd63014d2b
        .quad   0xc852369c6ca7578b
        .quad   0x5b2fcd285c0b5df0
        .quad   0x12ab214c58048c8f
        .quad   0x070442985d517bc3
        .quad   0x6acd56c7ae653678
        .quad   0x00a27983985a7763
        .quad   0x5167effae512662b
        .quad   0xbd4ea9e10f53c4b6
        .quad   0x1673dc5f8ac91a14
        .quad   0xa8f81a4e2acc1aba
        .quad   0x33a92a7924332a25

        // 2^252 * 8 * G

        .quad   0x9dd1f49927996c02
        .quad   0x0cb3b058e04d1752
        .quad   0x1f7e88967fd02c3e
        .quad   0x2f964268cb8b3eb1
        .quad   0x7ba95ba0218f2ada
        .quad   0xcff42287330fb9ca
        .quad   0xdada496d56c6d907
        .quad   0x5380c296f4beee54
        .quad   0x9d4f270466898d0a
        .quad   0x3d0987990aff3f7a
        .quad   0xd09ef36267daba45
        .quad   0x7761455e7b1c669c

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", %progbits
#endif
