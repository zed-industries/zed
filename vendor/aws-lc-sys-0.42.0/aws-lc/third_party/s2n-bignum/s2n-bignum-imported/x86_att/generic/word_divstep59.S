// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Perform 59 "divstep" iterations and return signed matrix of updates
// Inputs d, f, g; output m[2][2] and function return
//
// extern int64_t word_divstep59
//  (int64_t m[2][2],int64_t d,uint64_t f,uint64_t g);
//
// Standard x86-64 ABI: RDI = m, RSI = d, RDX = f, RCX = g, returns RAX
// Microsoft x64 ABI:   RCX = m, RDX = d, R8 = f, R9 = g, returns RAX
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(word_divstep59)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(word_divstep59)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(word_divstep59)
        .text

#define mat %rdi

#define d %rsi
#define fuv %rbx
#define grs %rcx

#define f %r12
#define g %r13

#define m %r8
#define t %r9

#define zero %rbp
#define zeroe %ebp
#define minus2 %rax
#define minus2e %eax
#define plus2 %rdx
#define plus2e %edx

#define m00 %r8
#define m01 %r9
#define m10 %r10
#define m11 %r11

S2N_BN_SYMBOL(word_divstep59):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
#endif

// Save extra registers

        CFI_PUSH(%rbx)
        CFI_PUSH(%rbp)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)

// Pack f and g into single registers with (negated) update matrices,
// initially the identity matrix. The f_lo and g_lo are initially
// the 20 lowest bits of f and g.
//
// fuv = f_lo - 2^41 * 1 - 2^62 * 0
// grs = g_lo - 2^41 * 0 - 2^62 * 1

        movq    %rdx, fuv
        movq    %rdx, f
        andq    $0xFFFFF, fuv
        movq    $0xFFFFFE0000000000, %rax
        orq     %rax, fuv

        movq    %rcx, g
        andq    $0xFFFFF, grs
        movq    $0xc000000000000000, %rax
        orq     %rax, grs

// Now do 20 divsteps on that packed format.
//
// At the i'th iteration (starting at i = 0, ending at i = 20)
// the intermediate packed values are of the form
//
// fuv = f_lo - 2^{41-i} * m00 - 2^{62-i} * m01
// grs = g_lo - 2^{41-i} * m10 - 2^{62-i} * m11
//
// where the following matrix indicates the updates to apply
// to the original (full-sized) f and g for those iterations.
//
// [m00 m01] * [f_0] = [f_i]
// [m10 m11]   [g_0]   [g_i]

        movq    $-2, minus2
        xorl    zeroe, zeroe
        movl    $2, plus2e
        movq    fuv, t
        movq    minus2, m
        testq   d, d
        cmovs   zero, m
        testq   $1, grs
.set i, 0
.rep 20
        cmovzq zero, m
        cmovzq zero, t
.if (i != 0)
        sarq    $1, grs
.endif
        xorq    m, t
        xorq    m, d
        btq     $63, m
        cmovcq  grs, fuv
        movq    minus2, m
        addq    plus2, d
        leaq    (grs,t), grs
.if (i != 19)
        cmovs   zero, m
        movq    fuv, t
        testq   plus2, grs
.endif
.set i, (i+1)
.endr
        sarq    $1, grs

// Extract the matrix entries, but keep them in negated form.
// Store them in the output buffer temporarily.

        movl    $1048576, %eax
        leaq    (fuv,%rax), m00
        leaq    (grs,%rax), m10
        shlq    $22, m00
        shlq    $22, m10
        sarq    $43, m00
        sarq    $43, m10

        movq    $2199024304128, %rax
        leaq    (fuv,%rax), m01
        leaq    (grs,%rax), m11
        sarq    $42, m01
        sarq    $42, m11

        movq    m00, (mat)
        movq    m01, 8(mat)
        movq    m10, 16(mat)
        movq    m11, 24(mat)

// Compute updated f and g using the negated matrix entries;
// this flips the signs of f and g but it doesn't matter.
//
//   f = (m00 * f + m01 * g) / 2^20
//   g = (m10 * f + m11 * g) / 2^20
//
// Since we only need another 40 bits, we can do all of that
// computation naively using (implicitly signed) 64-bit words.

        imulq   f, m10
        imulq   m00, f
        imulq   g, m01
        imulq   m11, g
        addq    m01, f
        addq    m10, g
        sarq    $20, f
        sarq    $20, g

// Re-pack for 20 more rounds

        movq    f, fuv
        andq    $0xFFFFF, fuv
        movq    $0xFFFFFE0000000000, %rax
        orq     %rax, fuv

        movq    g, grs
        andq    $0xFFFFF, grs
        movq    $0xc000000000000000, %rax
        orq     %rax, grs

// Second block of 20 divsteps in the same style

        movq    $-2, minus2
        movl    $2, plus2e
        movq    fuv, t
        movq    minus2, m
        testq   d, d
        cmovs   zero, m
        testq   $1, grs
.set i, 0
.rep 20
        cmovzq zero, m
        cmovzq zero, t
.if (i != 0)
        sarq    $1, grs
.endif
        xorq    m, t
        xorq    m, d
        btq     $63, m
        cmovcq  grs, fuv
        movq    minus2, m
        addq    plus2, d
        leaq    (grs,t), grs
.if (i != 19)
        cmovs   zero, m
        movq    fuv, t
        testq   plus2, grs
.endif
.set i, (i+1)
.endr
        sarq    $1, grs

// Extract the next matrix entries, in negated form again

        movl    $1048576, %eax
        leaq    (fuv,%rax), m00
        leaq    (grs,%rax), m10
        shlq    $22, m00
        shlq    $22, m10
        sarq    $43, m00
        sarq    $43, m10

        movq    $2199024304128, %rax
        leaq    (fuv,%rax), m01
        leaq    (grs,%rax), m11
        sarq    $42, m01
        sarq    $42, m11

// Compute updated f and g using the negated matrix entries,
// and so again flipping (thus actually restoring) the signs.
//
//   f = (n00 * f + n01 * g) / 2^20
//   g = (n10 * f + n11 * g) / 2^20

        movq    g, fuv
        movq    f, grs
        imulq   m00, f
        imulq   m01, fuv
        addq    fuv, f
        imulq   m11, g
        imulq   m10, grs
        addq    grs, g
        sarq    $20, f
        sarq    $20, g

// Re-pack for 20 more rounds

        movq    f, fuv
        andq    $0xFFFFF, fuv
        movq    $0xFFFFFE0000000000, %rax
        orq     %rax, fuv

        movq    g, grs
        andq    $0xFFFFF, grs
        movq    $0xc000000000000000, %rax
        orq     %rax, grs

// Multiply the first two matrices, and re-store in the output buffer.
//
// [m00_new  m01_new] = [m00  m01] * [m00_prev  m01_prev]
// [m10_new  m11_new]   [m10  m11]   [m10_prev  m11_prev]
//
// The resulting matrix entries are:
//
//   m00_new = m00 * m00_prev + m01 * m10_prev
//   m01_new = m00 * m01_prev + m01 * m11_prev
//   m10_new = m10 * m00_prev + m11 * m10_prev
//   m11_new = m10 * m01_prev + m11 * m11_prev
//
// At this point the sign is right since both matrices were negated.

        movq    (mat), %rax
        imulq   m00, %rax
        movq    16(mat), %rdx
        imulq   m01, %rdx
        imulq   8(mat), m00
        imulq   24(mat), m01
        addq    m00, m01
        leaq    (%rax,%rdx), m00

        movq    (mat), %rax
        imulq   m10, %rax
        movq    16(mat), %rdx
        imulq   m11, %rdx
        imulq   8(mat), m10
        imulq   24(mat), m11
        addq    m10, m11
        leaq    (%rax,%rdx), m10

        movq    m00, (mat)
        movq    m01, 8(mat)
        movq    m10, 16(mat)
        movq    m11, 24(mat)

// Third block of divsteps, same style but a total of 19 not 20

        movq    $-2, minus2
        movl    $2, plus2e
        movq    fuv, t
        movq    minus2, m
        testq   d, d
        cmovs   zero, m
        testq   $1, grs
.set i, 0
.rep 19
        cmovzq zero, m
        cmovzq zero, t
.if (i != 0)
        sarq    $1, grs
.endif
        xorq    m, t
        xorq    m, d
        btq     $63, m
        cmovcq  grs, fuv
        movq    minus2, m
        addq    plus2, d
        leaq    (grs,t), grs
.if (i != 18)
        cmovs   zero, m
        movq    fuv, t
        testq   plus2, grs
.endif
.set i, (i+1)
.endr
        sarq    $1, grs

// Extract the matrix entries from the final 19 divsteps

        movl    $1048576, %eax
        leaq    (fuv,%rax), m00
        leaq    (grs,%rax), m10
        shlq    $21, m00
        shlq    $21, m10
        sarq    $43, m00
        sarq    $43, m10

        movq    $2199024304128, %rax
        leaq    (fuv,%rax), m01
        leaq    (grs,%rax), m11
        sarq    $43, m01
        sarq    $43, m11

// Multiply by this new matrix
//
// [m00_new  m01_new] = [m00  m01] * [m00_prev  m01_prev]
// [m10_new  m11_new]   [m10  m11]   [m10_prev  m11_prev]
//
// The resulting matrix entries are:
//
//   m00_new = m00 * m00_prev + m01 * m10_prev
//   m01_new = m00 * m01_prev + m01 * m11_prev
//   m10_new = m10 * m00_prev + m11 * m10_prev
//   m11_new = m10 * m01_prev + m11 * m11_prev
//
// Since we didn't negate the n matrix, all products are negated
// and so we insert negations

        movq    (mat), %rax
        imulq   m00, %rax
        movq    16(mat), %rdx
        imulq   m01, %rdx
        imulq   8(mat), m00
        imulq   24(mat), m01
        addq    m00, m01
        leaq    (%rax,%rdx), m00
        negq    m01
        negq    m00

        movq    (mat), %rax
        imulq   m10, %rax
        movq    16(mat), %rdx
        imulq   m11, %rdx
        imulq   8(mat), m10
        imulq   24(mat), m11
        addq    m10, m11
        leaq    (%rax,%rdx), m10
        negq    m11
        negq    m10

// Now write back the final matrix and d for the whole 59 steps

        movq    m00, (mat)
        movq    m01, 8(mat)
        movq    m10, 16(mat)
        movq    m11, 24(mat)
        movq    d, %rax

// Restore registers and return

        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbp)
        CFI_POP(%rbx)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif

        CFI_RET

S2N_BN_SIZE_DIRECTIVE(word_divstep59)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
