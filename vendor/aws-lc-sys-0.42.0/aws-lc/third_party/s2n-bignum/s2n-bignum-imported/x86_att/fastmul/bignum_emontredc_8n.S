// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Extended Montgomery reduce in 8-digit blocks, results in input-output buffer
// Inputs z[2*k], m[k], w; outputs function return (extra result bit) and z[2*k]
//
//    extern uint64_t bignum_emontredc_8n(uint64_t k, uint64_t *z, const uint64_t *m,
//                                        uint64_t w);
//
// Functionally equivalent to bignum_emontredc (see that file for more detail).
// But in general assumes that the input k is a multiple of 8.
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = m, RCX = w, returns RAX
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = m, R9 = w, returns RAX
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_emontredc_8n)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_emontredc_8n)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_emontredc_8n)
        .text

// Original input parameters are here

#define z %rsi
#define w %rcx

// This is copied in early once we stash away k

#define m %rdi

// A variable z pointer

#define zz %rbp

// Stack-based variables

#define carry  (%rsp)
#define innercount  8(%rsp)
#define outercount  16(%rsp)
#define k8m1  24(%rsp)

// -----------------------------------------------------------------------------
// Standard macros as used in pure multiplier arrays
// -----------------------------------------------------------------------------

// mulpadd i, j adds z[i] * rdx (now assumed = m[j]) into the window at i+j

.macro mulpadd arg1,arg2
        mulxq   8*\arg1(z), %rax, %rbx
.if ((\arg1 + \arg2) % 8 == 0)
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
.elseif ((\arg1 + \arg2) % 8 == 1)
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
.elseif ((\arg1 + \arg2) % 8 == 2)
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
.elseif ((\arg1 + \arg2) % 8 == 3)
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
.elseif ((\arg1 + \arg2) % 8 == 4)
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
.elseif ((\arg1 + \arg2) % 8 == 5)
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
.elseif ((\arg1 + \arg2) % 8 == 6)
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
.elseif ((\arg1 + \arg2) % 8 == 7)
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
.endif

.endm

// addrow i adds z[i] + zz[0..7] * m[j] into the window

.macro addrow arg1
        movq    8*\arg1(m), %rdx
        xorl    %eax, %eax // Get a known flag state

.if (\arg1 % 8 == 0)
        adoxq   8*\arg1(zz), %r8
.elseif (\arg1 % 8 == 1)
        adoxq   8*\arg1(zz), %r9
.elseif (\arg1 % 8 == 2)
        adoxq   8*\arg1(zz), %r10
.elseif (\arg1 % 8 == 3)
        adoxq   8*\arg1(zz), %r11
.elseif (\arg1 % 8 == 4)
        adoxq   8*\arg1(zz), %r12
.elseif (\arg1 % 8 == 5)
        adoxq   8*\arg1(zz), %r13
.elseif (\arg1 % 8 == 6)
        adoxq   8*\arg1(zz), %r14
.elseif (\arg1 % 8 == 7)
        adoxq   8*\arg1(zz), %r15
.endif

        mulpadd 0, \arg1

.if (\arg1 % 8 == 0)
        movq    %r8, 8*\arg1(zz)
        movl    $0, %r8d
.elseif (\arg1 % 8 == 1)
        movq    %r9, 8*\arg1(zz)
        movl    $0, %r9d
.elseif (\arg1 % 8 == 2)
        movq    %r10, 8*\arg1(zz)
        movl    $0, %r10d
.elseif (\arg1 % 8 == 3)
        movq    %r11, 8*\arg1(zz)
        movl    $0, %r11d
.elseif (\arg1 % 8 == 4)
        movq    %r12, 8*\arg1(zz)
        movl    $0, %r12d
.elseif (\arg1 % 8 == 5)
        movq    %r13, 8*\arg1(zz)
        movl    $0, %r13d
.elseif (\arg1 % 8 == 6)
        movq    %r14, 8*\arg1(zz)
        movl    $0, %r14d
.elseif (\arg1 % 8 == 7)
        movq    %r15, 8*\arg1(zz)
        movl    $0, %r15d
.endif

        mulpadd 1, \arg1
        mulpadd 2, \arg1
        mulpadd 3, \arg1
        mulpadd 4, \arg1
        mulpadd 5, \arg1
        mulpadd 6, \arg1
        mulpadd 7, \arg1

.if (\arg1 % 8 == 0)
        adcq    $0, %r8
.elseif (\arg1 % 8 == 1)
        adcq    $0, %r9
.elseif (\arg1 % 8 == 2)
        adcq    $0, %r10
.elseif (\arg1 % 8 == 3)
        adcq    $0, %r11
.elseif (\arg1 % 8 == 4)
        adcq    $0, %r12
.elseif (\arg1 % 8 == 5)
        adcq    $0, %r13
.elseif (\arg1 % 8 == 6)
        adcq    $0, %r14
.elseif (\arg1 % 8 == 7)
        adcq    $0, %r15
.endif


.endm

// -----------------------------------------------------------------------------
// Anti-matter versions with z and m switched, and also not writing back the z
// words, but the inverses instead, *and* also adding in the z[0..7] at the
// beginning. The aim is to use this in Montgomery where we discover z[j]
// entries as we go along.
// -----------------------------------------------------------------------------

.macro mulpadda arg1,arg2
        mulxq   8*\arg1(m), %rax, %rbx
.if ((\arg1 + \arg2) % 8 == 0)
        adcxq   %rax, %r8
        adoxq   %rbx, %r9
.elseif ((\arg1 + \arg2) % 8 == 1)
        adcxq   %rax, %r9
        adoxq   %rbx, %r10
.elseif ((\arg1 + \arg2) % 8 == 2)
        adcxq   %rax, %r10
        adoxq   %rbx, %r11
.elseif ((\arg1 + \arg2) % 8 == 3)
        adcxq   %rax, %r11
        adoxq   %rbx, %r12
.elseif ((\arg1 + \arg2) % 8 == 4)
        adcxq   %rax, %r12
        adoxq   %rbx, %r13
.elseif ((\arg1 + \arg2) % 8 == 5)
        adcxq   %rax, %r13
        adoxq   %rbx, %r14
.elseif ((\arg1 + \arg2) % 8 == 6)
        adcxq   %rax, %r14
        adoxq   %rbx, %r15
.elseif ((\arg1 + \arg2) % 8 == 7)
        adcxq   %rax, %r15
        adoxq   %rbx, %r8
.endif

.endm

.macro adurowa arg1
        movq    w, %rdx // Get the word-level modular inverse
        xorl    %eax, %eax // Get a known flag state
.if (\arg1 % 8 == 0)
        mulxq   %r8, %rdx, %rax
.elseif (\arg1 % 8 == 1)
        mulxq   %r9, %rdx, %rax
.elseif (\arg1 % 8 == 2)
        mulxq   %r10, %rdx, %rax
.elseif (\arg1 % 8 == 3)
        mulxq   %r11, %rdx, %rax
.elseif (\arg1 % 8 == 4)
        mulxq   %r12, %rdx, %rax
.elseif (\arg1 % 8 == 5)
        mulxq   %r13, %rdx, %rax
.elseif (\arg1 % 8 == 6)
        mulxq   %r14, %rdx, %rax
.elseif (\arg1 % 8 == 7)
        mulxq   %r15, %rdx, %rax
.endif

        movq    %rdx, 8*\arg1(z) // Store multiplier word

        mulpadda 0, \arg1

        // Note that the bottom reg of the window is zero by construction
        // So it's safe just to use "mulpadda 7" here

        mulpadda 1, \arg1
        mulpadda 2, \arg1
        mulpadda 3, \arg1
        mulpadda 4, \arg1
        mulpadda 5, \arg1
        mulpadda 6, \arg1
        mulpadda 7, \arg1          // window lowest = 0 beforehand by construction

.if (\arg1 % 8 == 0)
        adcq    $0, %r8
.elseif (\arg1 % 8 == 1)
        adcq    $0, %r9
.elseif (\arg1 % 8 == 2)
        adcq    $0, %r10
.elseif (\arg1 % 8 == 3)
        adcq    $0, %r11
.elseif (\arg1 % 8 == 4)
        adcq    $0, %r12
.elseif (\arg1 % 8 == 5)
        adcq    $0, %r13
.elseif (\arg1 % 8 == 6)
        adcq    $0, %r14
.elseif (\arg1 % 8 == 7)
        adcq    $0, %r15
.endif

.endm

.macro adurowza
        movq    w, %rdx // Get the word-level modular inverse
        xorl    %eax, %eax // Get a known flag state

        movq    (z), %r8 // %r8 = zeroth word
        mulxq   %r8, %rdx, %rax // Compute multiplier word
        movq    %rdx, (z) // Store multiplier word
        movq    8(z), %r9

        mulpadda 0, 0
        movq    16(z), %r10
        mulpadda 1, 0
        movq    24(z), %r11
        mulpadda 2, 0
        movq    32(z), %r12
        mulpadda 3, 0
        movq    40(z), %r13
        mulpadda 4, 0
        movq    48(z), %r14
        mulpadda 5, 0
        movq    56(z), %r15
        mulpadda 6, 0
        mulpadda 7, 0           // r8 = 0 beforehand by construction
        adcq    $0, %r8
.endm

// -----------------------------------------------------------------------------
// Hybrid top, doing an 8 block specially then multiple additional 8 blocks
// -----------------------------------------------------------------------------

// Multiply-add: z := z + x[i...i+7] * m

.macro addrows

        adurowza
        adurowa 1
        adurowa 2
        adurowa 3
        adurowa 4
        adurowa 5
        adurowa 6
        adurowa 7

        movq    z, zz

        movq    k8m1, %rax
        testq   %rax, %rax
        jz      Lbignum_emontredc_8n_innerend
        movq    %rax, innercount
Lbignum_emontredc_8n_innerloop:
        addq    $64, zz
        addq    $64, m
        addrow 0
        addrow 1
        addrow 2
        addrow 3
        addrow 4
        addrow 5
        addrow 6
        addrow 7
        subq    $64, innercount
        jnz     Lbignum_emontredc_8n_innerloop

        movq    k8m1, %rax
Lbignum_emontredc_8n_innerend:
        subq    %rax, m

        movq    carry, %rbx
        negq    %rbx
        adcq    %r8, 64(z,%rax,1)
        adcq    %r9, 72(z,%rax,1)
        adcq    %r10, 80(z,%rax,1)
        adcq    %r11, 88(z,%rax,1)
        adcq    %r12, 96(z,%rax,1)
        adcq    %r13, 104(z,%rax,1)
        adcq    %r14, 112(z,%rax,1)
        adcq    %r15, 120(z,%rax,1)
        movl    $0, %eax
        adcq    $0, %rax
        movq    %rax, carry
.endm

// -----------------------------------------------------------------------------
// Main code.
// -----------------------------------------------------------------------------

S2N_BN_SYMBOL(bignum_emontredc_8n):
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

// Save more registers to play with

        CFI_PUSH(%rbp)
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)

// Pre-initialize the return value to 0 just in case of early exit below

        xorl    %eax, %eax

// Divide the input k by 8, and push k8m1 = (k/8 - 1)<<6 which is used as
// the scaled inner loop counter / pointer adjustment repeatedly. Also push
// k/8 itself which is here initializing the outer loop count.

        shrq    $3, %rdi
        jz      Lbignum_emontredc_8n_end

        leaq    -1(%rdi), %rbx
        shlq    $6, %rbx
        CFI_PUSH(%rbx)
        CFI_PUSH(%rdi)

// Make space for two more variables, and set between-stages carry to 0

        CFI_DEC_RSP(16)
        movq    $0, carry

// Copy m into its main home

        movq    %rdx, m

// Now just systematically add in the rows

Lbignum_emontredc_8n_outerloop:
        addrows
        addq    $64, z
        subq    $1, outercount
        jnz     Lbignum_emontredc_8n_outerloop

// Pop the carry-out "p", which was stored at [%rsp], put in %rax for return

        CFI_POP(%rax)

// Adjust the stack

        CFI_INC_RSP(24)

// Reset of epilog

Lbignum_emontredc_8n_end:

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

S2N_BN_SIZE_DIRECTIVE(bignum_emontredc_8n)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
