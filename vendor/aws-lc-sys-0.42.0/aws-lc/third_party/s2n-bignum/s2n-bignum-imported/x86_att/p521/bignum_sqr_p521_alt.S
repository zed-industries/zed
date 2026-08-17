// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square modulo p_521, z := (x^2) mod p_521, assuming x reduced
// Input x[9]; output z[9]
//
//    extern void bignum_sqr_p521_alt(uint64_t z[static 9],
//                                    const uint64_t x[static 9]);
//
// Standard x86-64 ABI: RDI = z, RSI = x
// Microsoft x64 ABI:   RCX = z, RDX = x
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_sqr_p521_alt)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_sqr_p521_alt)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_sqr_p521_alt)
        .text

// Input arguments

#define z %rdi
#define x %rsi

// Macro for the key "multiply and add to (c,h,l)" step

#define combadd(c,h,l,numa,numb)                \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

// Set up initial window (c,h,l) = numa * numb

#define combaddz(c,h,l,numa,numb)               \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        xorq    c, c ;                           \
        movq    %rax, l ;                         \
        movq    %rdx, h

// Doubling step (c,h,l) = 2 * (c,hh,ll) + (0,h,l)

#define doubladd(c,h,l,hh,ll)                   \
        addq    ll, ll ;                         \
        adcq    hh, hh ;                         \
        adcq    c, c ;                           \
        addq    ll, l ;                          \
        adcq    hh, h ;                          \
        adcq    $0, c

// Square term incorporation (c,h,l) += numba^2

#define combadd1(c,h,l,numa)                    \
        movq    numa, %rax ;                      \
        mulq    %rax;                            \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

// A short form where we don't expect a top carry

#define combads(h,l,numa)                       \
        movq    numa, %rax ;                      \
        mulq    %rax;                            \
        addq    %rax, l ;                         \
        adcq    %rdx, h

// A version doubling directly before adding, for single non-square terms

#define combadd2(c,h,l,numa,numb)               \
        movq    numa, %rax ;                      \
        mulq     numb;                 \
        addq    %rax, %rax ;                       \
        adcq    %rdx, %rdx ;                       \
        adcq    $0, c ;                           \
        addq    %rax, l ;                         \
        adcq    %rdx, h ;                         \
        adcq    $0, c

S2N_BN_SYMBOL(bignum_sqr_p521_alt):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
#endif

// Make more registers available and make temporary space on stack

        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(72)

// Start doing a conventional columnwise squaring,
// temporarily storing the lower 9 digits on the stack.
// Start with result term 0

        movq    (x), %rax
        mulq    %rax

        movq    %rax, (%rsp)
        movq    %rdx, %r9
        xorq    %r10, %r10

// Result term 1

        xorq    %r11, %r11
        combadd2(%r11,%r10,%r9,(x),8(x))
        movq    %r9, 8(%rsp)

// Result term 2

        xorq    %r12, %r12
        combadd1(%r12,%r11,%r10,8(x))
        combadd2(%r12,%r11,%r10,(x),16(x))
        movq    %r10, 16(%rsp)

// Result term 3

        combaddz(%r13,%rcx,%rbx,(x),24(x))
        combadd(%r13,%rcx,%rbx,8(x),16(x))
        doubladd(%r13,%r12,%r11,%rcx,%rbx)
        movq    %r11, 24(%rsp)

// Result term 4

        combaddz(%r14,%rcx,%rbx,(x),32(x))
        combadd(%r14,%rcx,%rbx,8(x),24(x))
        doubladd(%r14,%r13,%r12,%rcx,%rbx)
        combadd1(%r14,%r13,%r12,16(x))
        movq    %r12, 32(%rsp)

// Result term 5

        combaddz(%r15,%rcx,%rbx,(x),40(x))
        combadd(%r15,%rcx,%rbx,8(x),32(x))
        combadd(%r15,%rcx,%rbx,16(x),24(x))
        doubladd(%r15,%r14,%r13,%rcx,%rbx)
        movq    %r13, 40(%rsp)

// Result term 6

        combaddz(%r8,%rcx,%rbx,(x),48(x))
        combadd(%r8,%rcx,%rbx,8(x),40(x))
        combadd(%r8,%rcx,%rbx,16(x),32(x))
        doubladd(%r8,%r15,%r14,%rcx,%rbx)
        combadd1(%r8,%r15,%r14,24(x))
        movq    %r14, 48(%rsp)

// Result term 7

        combaddz(%r9,%rcx,%rbx,(x),56(x))
        combadd(%r9,%rcx,%rbx,8(x),48(x))
        combadd(%r9,%rcx,%rbx,16(x),40(x))
        combadd(%r9,%rcx,%rbx,24(x),32(x))
        doubladd(%r9,%r8,%r15,%rcx,%rbx)
        movq    %r15, 56(%rsp)

// Result term 8

        combaddz(%r10,%rcx,%rbx,(x),64(x))
        combadd(%r10,%rcx,%rbx,8(x),56(x))
        combadd(%r10,%rcx,%rbx,16(x),48(x))
        combadd(%r10,%rcx,%rbx,24(x),40(x))
        doubladd(%r10,%r9,%r8,%rcx,%rbx)
        combadd1(%r10,%r9,%r8,32(x))
        movq    %r8, 64(%rsp)

// We now stop writing back and keep remaining results in a register window.
// Continue with result term 9

        combaddz(%r11,%rcx,%rbx,8(x),64(x))
        combadd(%r11,%rcx,%rbx,16(x),56(x))
        combadd(%r11,%rcx,%rbx,24(x),48(x))
        combadd(%r11,%rcx,%rbx,32(x),40(x))
        doubladd(%r11,%r10,%r9,%rcx,%rbx)

// Result term 10

        combaddz(%r12,%rcx,%rbx,16(x),64(x))
        combadd(%r12,%rcx,%rbx,24(x),56(x))
        combadd(%r12,%rcx,%rbx,32(x),48(x))
        doubladd(%r12,%r11,%r10,%rcx,%rbx)
        combadd1(%r12,%r11,%r10,40(x))

// Result term 11

        combaddz(%r13,%rcx,%rbx,24(x),64(x))
        combadd(%r13,%rcx,%rbx,32(x),56(x))
        combadd(%r13,%rcx,%rbx,40(x),48(x))
        doubladd(%r13,%r12,%r11,%rcx,%rbx)

// Result term 12

        combaddz(%r14,%rcx,%rbx,32(x),64(x))
        combadd(%r14,%rcx,%rbx,40(x),56(x))
        doubladd(%r14,%r13,%r12,%rcx,%rbx)
        combadd1(%r14,%r13,%r12,48(x))

// Result term 13

        combaddz(%r15,%rcx,%rbx,40(x),64(x))
        combadd(%r15,%rcx,%rbx,48(x),56(x))
        doubladd(%r15,%r14,%r13,%rcx,%rbx);

// Result term 14

        xorq    %r8, %r8
        combadd1(%r8,%r15,%r14,56(x))
        combadd2(%r8,%r15,%r14,48(x),64(x))

// Result term 15

        movq    56(x), %rax
        mulq     64(x)
        addq    %rax, %rax
        adcq    %rdx, %rdx
        addq    %rax, %r15
        adcq    %rdx, %r8

// Result term 16

        movq    64(x), %rax
        imulq   %rax, %rax
        addq    %r8, %rax

// Now the upper portion is [%rax;%r15;%r14;%r13;%r12;%r11;%r10;%r9;[%rsp+64]].
// Rotate the upper portion right 9 bits since 2^512 == 2^-9 (mod p_521)
// Let rotated result %rdx,%r15,%r14,...,%r8 be h (high) and %rsp[0..7] be l (low)

        movq    64(%rsp), %r8
        movq    %r8, %rdx
        andq    $0x1FF, %rdx
        shrdq   $9, %r9, %r8
        shrdq   $9, %r10, %r9
        shrdq   $9, %r11, %r10
        shrdq   $9, %r12, %r11
        shrdq   $9, %r13, %r12
        shrdq   $9, %r14, %r13
        shrdq   $9, %r15, %r14
        shrdq   $9, %rax, %r15
        shrq    $9, %rax
        addq    %rax, %rdx

// Force carry-in then add to get s = h + l + 1
// but actually add all 1s in the top 53 bits to get simple carry out

        stc
        adcq    (%rsp), %r8
        adcq    8(%rsp), %r9
        adcq    16(%rsp), %r10
        adcq    24(%rsp), %r11
        adcq    32(%rsp), %r12
        adcq    40(%rsp), %r13
        adcq    48(%rsp), %r14
        adcq    56(%rsp), %r15
        adcq    $~0x1FF, %rdx

// Now CF is set <=> h + l + 1 >= 2^521 <=> h + l >= p_521,
// in which case the lower 521 bits are already right. Otherwise if
// CF is clear, we want to subtract 1. Hence subtract the complement
// of the carry flag then mask the top word, which scrubs the
// padding in either case. Write digits back as they are created.

        cmc
        sbbq    $0, %r8
        movq    %r8, (z)
        sbbq    $0, %r9
        movq    %r9, 8(z)
        sbbq    $0, %r10
        movq    %r10, 16(z)
        sbbq    $0, %r11
        movq    %r11, 24(z)
        sbbq    $0, %r12
        movq    %r12, 32(z)
        sbbq    $0, %r13
        movq    %r13, 40(z)
        sbbq    $0, %r14
        movq    %r14, 48(z)
        sbbq    $0, %r15
        movq    %r15, 56(z)
        sbbq    $0, %rdx
        andq    $0x1FF, %rdx
        movq    %rdx, 64(z)

// Restore registers and return

        CFI_INC_RSP(72)
        CFI_POP(%r15)
        CFI_POP(%r14)
        CFI_POP(%r13)
        CFI_POP(%r12)
        CFI_POP(%rbx)
#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_sqr_p521_alt)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
