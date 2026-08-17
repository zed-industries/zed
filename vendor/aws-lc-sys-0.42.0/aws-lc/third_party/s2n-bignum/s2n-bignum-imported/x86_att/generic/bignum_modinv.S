// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Invert modulo m, z = (1/a) mod b, assuming b is an odd number > 1, coprime a
// Inputs a[k], b[k]; output z[k]; temporary buffer t[>=3*k]
//
//    extern void bignum_modinv(uint64_t k, uint64_t *z, const uint64_t *a,
//                              const uint64_t *b, uint64_t *t);
//
// k-digit (digit=64 bits) "z := a^-1 mod b" (modular inverse of a modulo b)
// using t as a temporary buffer (t at least 3*k words = 24*k bytes), and
// assuming that a and b are coprime *and* that b is an odd number > 1.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = a, RCX = b, R8 = t
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = a, R9 = b, [RSP+40] = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_modinv)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_modinv)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_modinv)
        .text

// We get CHUNKSIZE bits per outer iteration, 64 minus a few for proxy errors

#define CHUNKSIZE 58

// These variables are so fundamental we keep them consistently in registers.
// k actually stays where it was at the beginning, while l gets set up  later

#define k %rdi
#define l %r13

// These are kept on the stack since there aren't enough registers

#define mat_mm      (%rsp)
#define mat_mn      8(%rsp)
#define mat_nm      16(%rsp)
#define mat_nn      24(%rsp)
#define t           32(%rsp)
// Modular inverse
#define v           40(%rsp)
// We reconstruct n as m + 8*k as needed
#define m           48(%rsp)
#define w           56(%rsp)
#define z           64(%rsp)
// Original b pointer, not b the temp
#define bm          72(%rsp)

#define STACKVARSIZE 80

// These get set to m/n or w/z during the cross-multiplications etc.
// Otherwise they can be used as additional temporaries

#define p1 %r8
#define p2 %r15

// These are shorthands for common temporary registers

#define a %rax
#define b %rbx
#define c %rcx
#define d %rdx
#define i %r9

// Temporaries for the top proxy selection part

#define c1        %r10
#define c2        %r11
#define h1        %r12
#define h2        %rbp
#define l1        %r14
#define l2        %rsi

// Re-use for the actual proxies; m_hi = h1 and n_hi = h2 are assumed

#define m_hi    %r12
#define n_hi    %rbp
#define m_lo    %r14
#define n_lo    %rsi

// Re-use for the matrix entries in the inner loop, though they
// get spilled to the corresponding memory locations mat_...

#define m_m     %r10
#define m_n     %r11
#define n_m     %rcx
#define n_n     %rdx

#define ashort %eax
#define ishort %r9d
#define m_mshort %r10d
#define m_nshort %r11d
#define n_mshort %ecx
#define n_nshort %edx

S2N_BN_SYMBOL(bignum_modinv):
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
#endif

// Save all required registers and make room on stack for all the above vars

        CFI_PUSH(%rbp)
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(STACKVARSIZE)

// If k = 0 then do nothing (this is out of scope anyway)

        testq   k, k
        jz      Lbignum_modinv_end

// Set up the additional two buffers m and n beyond w in temp space
// and record all pointers m, n, w and z in stack-based variables

        movq    %rsi, z
        movq    %r8, w
        movq    %rcx, bm
        leaq    (%r8,k,8), %r10
        movq    %r10, m
        leaq    (%r10,k,8), p2

// Initialize the main buffers with their starting values:
// m = a, n = b, w = b (to be tweaked to b - 1) and z = 0

        xorq    %r11, %r11
        xorq    i, i
Lbignum_modinv_copyloop:
        movq    (%rdx,i,8), a
        movq    (%rcx,i,8), b
        movq    a, (%r10,i,8)
        movq    b, (p2,i,8)
        movq    b, (%r8,i,8)
        movq    %r11, (%rsi,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_modinv_copyloop

// Tweak down w to b - 1 (this crude approach is safe as b needs to be odd
// for it to be in scope). We have then established the congruence invariant:
//
//   a * w == -m (mod b)
//   a * z == n (mod b)
//
// This, with the bounds w <= b and z <= b, is maintained round the outer loop

        movq    (%r8), a
        movq    a, b
        decq    b
        movq    b, (%r8)

// Compute v = negated modular inverse of b mod 2^64, reusing a from above
// This is used for Montgomery reduction operations each time round the loop

        movq    a, h2
        movq    a, h1
        shlq    $2, h2
        subq    h2, h1
        xorq    $2, h1

        movq    h1, h2
        imulq   a, h2
        movl    $2, ashort
        addq    h2, a
        addq    $1, h2

        imulq   a, h1

        imulq   h2, h2
        movl    $1, ashort
        addq    h2, a
        imulq   a, h1

        imulq   h2, h2
        movl    $1, ashort
        addq    h2, a
        imulq   a, h1

        imulq   h2, h2
        movl    $1, ashort
        addq    h2, a
        imulq   a, h1

        movq    h1, v

// Set up the outer loop count of 128 * k
// The invariant is that m * n < 2^t at all times.

        movq    k, a
        shlq    $7, a
        movq    a, t

// Start of the main outer loop iterated t / CHUNKSIZE times

Lbignum_modinv_outerloop:

// We need only bother with sharper l = min k (ceil(t/64)) digits
// for the computations on m and n (but we still need k for w and z).
// Either both m and n fit in l digits, or m has become zero and so
// nothing happens in the loop anyway and this makes no difference.

        movq    t, l
        addq    $63, l
        shrq    $6, l
        cmpq    k, l
        cmovncq k, l

// Select upper and lower proxies for both m and n to drive the inner
// loop. The lower proxies are simply the lowest digits themselves,
// m_lo = m[0] and n_lo = n[0], while the upper proxies are bitfields
// of the two inputs selected so their top bit (63) aligns with the
// most significant bit of *either* of the two inputs.

        xorq    h1, h1 // Previous high and low for m
        xorq    l1, l1
        xorq    h2, h2 // Previous high and low for n
        xorq    l2, l2
        xorq    c2, c2 // Mask flag: previous word of one was nonzero
        // and in this case h1 and h2 are those words

        movq    m, p1
        leaq    (p1,k,8), p2
        xorq    i, i
Lbignum_modinv_toploop:
        movq    (p1,i,8), b
        movq    (p2,i,8), c
        movq    c2, c1
        andq    h1, c1
        andq    h2, c2
        movq    b, a
        orq     c, a
        negq    a
        cmovcq  c1, l1
        cmovcq  c2, l2
        cmovcq  b, h1
        cmovcq  c, h2
        sbbq    c2, c2
        incq    i
        cmpq    l, i
        jc      Lbignum_modinv_toploop

        movq    h1, a
        orq     h2, a
        bsrq    a, c
        xorq    $63, c
        shldq   %cl, l1, h1
        shldq   %cl, l2, h2

// m_lo = m[0], n_lo = n[0];

        movq    (p1), %rax
        movq    %rax, m_lo

        movq    (p2), %rax
        movq    %rax, n_lo

// Now the inner loop, with i as loop counter from CHUNKSIZE down.
// This records a matrix of updates to apply to the initial
// values of m and n with, at stage j:
//
//     sgn * m' = (m_m * m - m_n * n) / 2^j
//    -sgn * n' = (n_m * m - n_n * n) / 2^j
//
// where "sgn" is either +1 or -1, and we lose track of which except
// that both instance above are the same. This throwing away the sign
// costs nothing (since we have to correct in general anyway because
// of the proxied comparison) and makes things a bit simpler. But it
// is simply the parity of the number of times the first condition,
// used as the swapping criterion, fires in this loop.

        movl    $1, m_mshort
        movl    $0, m_nshort
        movl    $0, n_mshort
        movl    $1, n_nshort
        movl    $CHUNKSIZE, ishort

// Stash more variables over the inner loop to free up regs

        movq    k, mat_mn
        movq    l, mat_nm
        movq    p1, mat_mm
        movq    p2, mat_nn

// Conceptually in the inner loop we follow these steps:
//
// * If m_lo is odd and m_hi < n_hi, then swap the four pairs
//    (m_hi,n_hi); (m_lo,n_lo); (m_m,n_m); (m_n,n_n)
//
// * Now, if m_lo is odd (old or new, doesn't matter as initial n_lo is odd)
//    m_hi := m_hi - n_hi, m_lo := m_lo - n_lo
//    m_m  := m_m + n_m, m_n := m_n + n_n
//
// * Halve and double them
//     m_hi := m_hi / 2, m_lo := m_lo / 2
//     n_m := n_m * 2, n_n := n_n * 2
//
// The actual computation computes updates before actually swapping and
// then corrects as needed.

Lbignum_modinv_innerloop:

        xorl    %eax, %eax
        xorl    %ebx, %ebx
        xorq    p1, p1
        xorq    p2, p2
        btq     $0, m_lo

        cmovcq  n_hi, %rax
        cmovcq  n_lo, %rbx
        cmovcq  n_m, p1
        cmovcq  n_n, p2

        movq    m_lo, l
        subq    %rbx, m_lo
        subq    l, %rbx
        movq    m_hi, k
        subq    %rax, k
        cmovcq  m_hi, n_hi
        leaq    -1(k), m_hi
        cmovcq  %rbx, m_lo
        cmovcq  l, n_lo
        notq    m_hi
        cmovcq  m_m, n_m
        cmovcq  m_n, n_n
        cmovncq k, m_hi

        shrq    $1, m_lo
        addq    p1, m_m
        addq    p2, m_n
        shrq    $1, m_hi
        addq    n_m, n_m
        addq    n_n, n_n

// End of the inner for-loop

        decq    i
        jnz     Lbignum_modinv_innerloop

// Unstash the temporary variables

        movq    mat_mn, k
        movq    mat_nm, l
        movq    mat_mm, p1
        movq    mat_nn, p2

// Put the matrix entries in memory since we're out of registers
// We pull them out repeatedly in the next loop

        movq    m_m, mat_mm
        movq    m_n, mat_mn
        movq    n_m, mat_nm
        movq    n_n, mat_nn

// Apply the update to w and z, using addition in this case, and also take
// the chance to shift an additional 6 = 64-CHUNKSIZE bits to be ready for a
// Montgomery multiplication. Because we know that m_m + m_n <= 2^58 and
// w, z <= b < 2^{64k}, we know that both of these fit in k+1 words.
// We do this before the m-n update to allow us to play with c1 and c2 here.
//
//    l1::w = 2^6 * (m_m * w + m_n * z)
//    l2::z = 2^6 * (n_m * w + n_n * z)
//
// with c1 and c2 recording previous words for the shifting part

        movq    w, p1
        movq    z, p2
        xorq    l1, l1
        xorq    l2, l2
        xorq    c1, c1
        xorq    c2, c2
        xorq    i, i
Lbignum_modinv_congloop:

        movq    (p1,i,8), c
        movq    mat_mm, a
        mulq    c
        addq    a, l1
        adcq    $0, d
        movq    d, h1 // Now h1::l1 := m_m * w + l1_in

        movq    mat_nm, a
        mulq    c
        addq    a, l2
        adcq    $0, d
        movq    d, h2 // Now h2::l2 := n_m * w + l2_in

        movq    (p2,i,8), c
        movq    mat_mn, a
        mulq    c
        addq    a, l1
        adcq    d, h1 // h1::l1 := m_m * w + m_n * z + l1_in
        shrdq   $CHUNKSIZE, l1, c1
        movq    c1, (p1,i,8)
        movq    l1, c1
        movq    h1, l1

        movq    mat_nn, a
        mulq    c
        addq    a, l2
        adcq    d, h2 // h2::l2 := n_m * w + n_n * z + l2_in
        shrdq   $CHUNKSIZE, l2, c2
        movq    c2, (p2,i,8)
        movq    l2, c2
        movq    h2, l2

        incq    i
        cmpq    k, i
        jc      Lbignum_modinv_congloop

        shldq   $64-CHUNKSIZE, c1, l1
        shldq   $64-CHUNKSIZE, c2, l2

// Do a Montgomery reduction of l1::w

        movq    bm, p2

        movq    (p1), b
        movq    v, h1
        imulq   b, h1
        movq    (p2), a
        mulq    h1
        addq    b, a // Will be zero but want the carry
        movq    %rdx, c1
        movl    $1, ishort
        movq    k, c
        decq    c
        jz      Lbignum_modinv_wmontend

Lbignum_modinv_wmontloop:
        adcq    (p1,i,8), c1
        sbbq    b, b
        movq    (p2,i,8), a
        mulq    h1
        subq    b, %rdx
        addq    c1, a
        movq    a, -8(p1,i,8)
        movq    %rdx, c1
        incq    i
        decq    c
        jnz     Lbignum_modinv_wmontloop

Lbignum_modinv_wmontend:
        adcq    l1, c1
        movq    c1, -8(p1,k,8)
        sbbq    c1, c1
        negq    c1

        movq    k, c
        xorq    i, i
Lbignum_modinv_wcmploop:
        movq    (p1,i,8), a
        sbbq    (p2,i,8), a
        incq    i
        decq    c
        jnz     Lbignum_modinv_wcmploop
        sbbq    $0, c1
        sbbq    c1, c1
        notq    c1

        xorq    c, c
        xorq    i, i
Lbignum_modinv_wcorrloop:
        movq    (p1,i,8), a
        movq    (p2,i,8), b
        andq    c1, b
        negq    c
        sbbq    b, a
        sbbq    c, c
        movq    a, (p1,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_modinv_wcorrloop

// Do a Montgomery reduction of l2::z

        movq    z, p1

        movq    (p1), b
        movq    v, h2
        imulq   b, h2
        movq    (p2), a
        mulq    h2
        addq    b, a // Will be zero but want the carry
        movq    %rdx, c2
        movl    $1, ishort
        movq    k, c
        decq    c
        jz      Lbignum_modinv_zmontend

Lbignum_modinv_zmontloop:
        adcq    (p1,i,8), c2
        sbbq    b, b
        movq    (p2,i,8), a
        mulq    h2
        subq    b, %rdx
        addq    c2, a
        movq    a, -8(p1,i,8)
        movq    %rdx, c2
        incq    i
        decq    c
        jnz     Lbignum_modinv_zmontloop

Lbignum_modinv_zmontend:
        adcq    l2, c2
        movq    c2, -8(p1,k,8)
        sbbq    c2, c2
        negq    c2

        movq    k, c
        xorq    i, i
Lbignum_modinv_zcmploop:
        movq    (p1,i,8), a
        sbbq    (p2,i,8), a
        incq    i
        decq    c
        jnz     Lbignum_modinv_zcmploop
        sbbq    $0, c2
        sbbq    c2, c2
        notq    c2

        xorq    c, c
        xorq    i, i
Lbignum_modinv_zcorrloop:
        movq    (p1,i,8), a
        movq    (p2,i,8), b
        andq    c2, b
        negq    c
        sbbq    b, a
        sbbq    c, c
        movq    a, (p1,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_modinv_zcorrloop

// Now actually compute the updates to m and n corresponding to the matrix,
// and correct the signs if they have gone negative. First we compute the
// (k+1)-sized updates with the following invariant (here h1 and h2 are in
// fact carry bitmasks, either 0 or -1):
//
//    h1::l1::m = m_m * m - m_n * n
//    h2::l2::n = n_m * m - n_n * n

        movq    m, p1
        leaq    (p1,k,8), p2
        xorq    i, i
        xorq    h1, h1
        xorq    l1, l1
        xorq    h2, h2
        xorq    l2, l2
Lbignum_modinv_crossloop:

        movq    (p1,i,8), c
        movq    mat_mm, a
        mulq    c
        addq    a, l1
        adcq    $0, d
        movq    d, c1 // Now c1::l1 is +ve part 1

        movq    mat_nm, a
        mulq    c
        addq    a, l2
        adcq    $0, d
        movq    d, c2 // Now c2::l2 is +ve part 2

        movq    (p2,i,8), c
        movq    mat_mn, a
        mulq    c
        subq    h1, d // Now d::a is -ve part 1

        subq    a, l1
        sbbq    d, c1
        sbbq    h1, h1
        movq    l1, (p1,i,8)
        movq    c1, l1

        movq    mat_nn, a
        mulq    c
        subq    h2, d // Now d::a is -ve part 2

        subq    a, l2
        sbbq    d, c2
        sbbq    h2, h2
        movq    l2, (p2,i,8)
        movq    c2, l2

        incq    i
        cmpq    l, i
        jc      Lbignum_modinv_crossloop

// Now fix the signs of m and n if they have gone negative

        xorq    i, i
        movq    h1, c1 // carry-in coded up as well
        movq    h2, c2 // carry-in coded up as well
        xorq    h1, l1 // for the Lbignum_modinv_end digit
        xorq    h2, l2 // for the Lbignum_modinv_end digit
Lbignum_modinv_optnegloop:
        movq    (p1,i,8), a
        xorq    h1, a
        negq    c1
        adcq    $0, a
        sbbq    c1, c1
        movq    a, (p1,i,8)
        movq    (p2,i,8), a
        xorq    h2, a
        negq    c2
        adcq    $0, a
        sbbq    c2, c2
        movq    a, (p2,i,8)
        incq    i
        cmpq    l, i
        jc      Lbignum_modinv_optnegloop
        subq    c1, l1
        subq    c2, l2

// Now shift them right CHUNKSIZE bits

        movq    l, i
Lbignum_modinv_shiftloop:
        movq    -8(p1,i,8), a
        movq    a, c1
        shrdq   $CHUNKSIZE, l1, a
        movq    a, -8(p1,i,8)
        movq    c1, l1
        movq    -8(p2,i,8), a
        movq    a, c2
        shrdq   $CHUNKSIZE, l2, a
        movq    a, -8(p2,i,8)
        movq    c2, l2
        decq    i
        jnz     Lbignum_modinv_shiftloop

// Finally, use the signs h1 and h2 to do optional modular negations of
// w and z respectively, flipping h2 to make signs work. We don't make
// any checks for zero values, but we certainly retain w <= b and z <= b.
// This is enough for the Montgomery step in the next iteration to give
// strict reduction w < b amd z < b, and anyway when we terminate we
// could not have z = b since it violates the coprimality assumption for
// in-scope cases.

        notq    h2
        movq    bm, c
        movq    w, p1
        movq    z, p2
        movq    h1, c1
        movq    h2, c2
        xorq    i, i
Lbignum_modinv_fliploop:
        movq    h2, d
        movq    (c,i,8), a
        andq    a, d
        andq    h1, a
        movq    (p1,i,8), b
        xorq    h1, b
        negq    c1
        adcq    b, a
        sbbq    c1, c1
        movq    a, (p1,i,8)
        movq    (p2,i,8), b
        xorq    h2, b
        negq    c2
        adcq    b, d
        sbbq    c2, c2
        movq    d, (p2,i,8)
        incq    i
        cmpq    k, i
        jc      Lbignum_modinv_fliploop

// End of main loop. We can stop if t' <= 0 since then m * n < 2^0, which
// since n is odd and m and n are coprime (in the in-scope cases) means
// m = 0, n = 1 and hence from the congruence invariant a * z == 1 (mod b).
// Moreover we do in fact need to maintain strictly t > 0 in the main loop,
// or the computation of the optimized digit bound l could collapse to 0.

        subq    $CHUNKSIZE, t
        jnbe    Lbignum_modinv_outerloop

Lbignum_modinv_end:
        CFI_INC_RSP(STACKVARSIZE)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
        CFI_POP(%rbp)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_modinv)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
