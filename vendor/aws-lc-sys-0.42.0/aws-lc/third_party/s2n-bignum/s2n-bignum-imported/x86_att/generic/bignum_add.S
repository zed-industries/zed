// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Add, z := x + y
// Inputs x[m], y[n]; outputs function return (carry-out) and z[p]
//
//    extern uint64_t bignum_add(uint64_t p, uint64_t *z, uint64_t m,
//                               const uint64_t *x, uint64_t n, const uint64_t *y);
//
// Does the z := x + y operation, truncating modulo p words in general and
// returning a top carry (0 or 1) in the p'th place, only adding the input
// words below p (as well as m and n respectively) to get the sum and carry.
//
// Standard x86-64 ABI: RDI = p, RSI = z, RDX = m, RCX = x, R8 = n, R9 = y, returns RAX
// Microsoft x64 ABI:   RCX = p, RDX = z, R8 = m, R9 = x, [RSP+40] = n, [RSP+48] = y, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_add)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_add)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_add)
        .text

#define p %rdi
#define z %rsi
#define m %rdx
#define x %rcx
#define n %r8
#define y %r9
#define i %r10
#define a %rax

#define ashort %eax



S2N_BN_SYMBOL(bignum_add):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
        movq    56(%rsp), %r8
        movq    64(%rsp), %r9
#endif

// Zero the main index counter for both branches

        xorq    i, i

// First clamp the two input sizes m := min(p,m) and n := min(p,n) since
// we'll never need words past the p'th. Can now assume m <= p and n <= p.
// Then compare the modified m and n and branch accordingly

        cmpq    m, p
        cmovcq  p, m
        cmpq    n, p
        cmovcq  p, n
        cmpq    n, m
        jc      Lbignum_add_ylonger

// The case where x is longer or of the same size (p >= m >= n)

        subq    m, p
        subq    n, m
        incq    m
        testq   n, n
        jz      Lbignum_add_xtest
Lbignum_add_xmainloop:
        movq    (x,i,8), a
        adcq    (y,i,8), a
        movq    a, (z,i,8)
        incq    i
        decq    n
        jnz     Lbignum_add_xmainloop
        jmp     Lbignum_add_xtest
Lbignum_add_xtoploop:
        movq    (x,i,8), a
        adcq    $0, a
        movq    a, (z,i,8)
        incq    i
Lbignum_add_xtest:
        decq    m
        jnz     Lbignum_add_xtoploop
        movl    $0, ashort
        adcq    $0, a
        testq   p, p
        jnz     Lbignum_add_tails
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        ret

// The case where y is longer (p >= n > m)

#if WINDOWS_ABI
  .cfi_def_cfa %rsp, 24
#else
  .cfi_def_cfa %rsp, 8
#endif

Lbignum_add_ylonger:
        subq    n, p
        subq    m, n
        testq   m, m
        jz      Lbignum_add_ytoploop
Lbignum_add_ymainloop:
        movq    (x,i,8), a
        adcq    (y,i,8), a
        movq    a, (z,i,8)
        incq    i
        decq    m
        jnz     Lbignum_add_ymainloop
Lbignum_add_ytoploop:
        movq    (y,i,8), a
        adcq    $0, a
        movq    a, (z,i,8)
        incq    i
        decq    n
        jnz     Lbignum_add_ytoploop
        movl    $0, ashort
        adcq    $0, a
        testq   p, p
        jnz     Lbignum_add_tails
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        ret

// Adding a non-trivial tail, when p > max(m,n)

Lbignum_add_tails:
        movq    a, (z,i,8)
        xorq    a, a
        jmp     Lbignum_add_tail
Lbignum_add_tailloop:
        movq    a, (z,i,8)
Lbignum_add_tail:
        incq    i
        decq    p
        jnz     Lbignum_add_tailloop
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_add)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
