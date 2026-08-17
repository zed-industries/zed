// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Modular exponentiation for arbitrary odd modulus
// Inputs a[k], p[k], m[k]; output z[k], temporary buffer t[>=3*k]
//
//   extern void bignum_modexp
//    (uint64_t k,uint64_t *z, const uint64_t *a,const uint64_t *p,
//     const uint64_t *m,uint64_t *t);
//
// Does z := (a^p) mod m where all numbers are k-digit and m is odd
//
// Standard x86-64 ABI: RDI = k, RSI = z, RDX = a, RCX = p, R8 = m, R9 = t
// Microsoft x64 ABI:   RCX = k, RDX = z, R8 = a, R9 = p, [RSP+40] = m, [RSP+48] = t
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_modexp)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_modexp)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_modexp)


        .text

// Local variables, all kept on the stack

#define k (%rsp)
#define res 8(%rsp)
#define a 16(%rsp)
#define p 24(%rsp)
#define m 32(%rsp)
#define x 40(%rsp)
#define i 48(%rsp)
#define y 56(%rsp)
#define z 64(%rsp)

#define VARSIZE 72

S2N_BN_SYMBOL(bignum_modexp):
        CFI_START
        _CET_ENDBR

// The Windows version literally calls the standard ABI version.
// This simplifies the proofs since subroutine offsets are fixed.

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
        movq    %r9, %rcx
        movq    56(%rsp), %r8
        movq    64(%rsp), %r9
        CFI_CALL(Lbignum_modexp_standard)
        CFI_POP(%rsi)
        CFI_POP(%rdi)
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(bignum_modexp)

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_modexp_standard)

Lbignum_modexp_standard:
        CFI_START
#endif

// Real start of the standard ABI code.
// Bump down the stack to make room for local variables

        CFI_DEC_RSP(VARSIZE)

// If size is zero (which falsifies the oddness condition) do nothing

        testq   %rdi, %rdi
        jz      Lbignum_modexp_end

// Set up local variables based on input parameters

        movq    %rdi, k
        movq    %rsi, res
        movq    %rdx, a
        movq    %rcx, p
        movq    %r8, m
        movq    %r9, x
        leaq    (%r9,%rdi,8), %rax
        movq    %rax, y
        leaq    (%rax,%rdi,8), %rax
        movq    %rax, z

// Let x == 2^64k * a (mod m) and initialize z == 2^64k * 1 (mod m)

        movq    k, %rdi
        movq    z, %rsi
        movq    m, %rdx
        movq    y, %rcx
        CFI_CALL(Lbignum_modexp_local_amontifier)

        movq    k, %rdi
        movq    x, %rsi
        movq    z, %rdx
        movq    a, %rcx
        movq    m, %r8
        CFI_CALL(Lbignum_modexp_local_amontmul)

        movq    k, %rdi
        movq    z, %rsi
        movq    z, %rdx
        movq    m, %rcx
        CFI_CALL(Lbignum_modexp_local_demont)

// Main loop with z == 2^64k * a^(p >> 2^i) (mod m)

        movq    k, %rax
        shlq    $6, %rax
        movq    %rax, i

Lbignum_modexp_loop:
        subq    $1, %rax
        movq    %rax, i

        movq    k, %rdi
        movq    y, %rsi
        movq    z, %rdx
        movq    z, %rcx
        movq    m, %r8
        CFI_CALL(Lbignum_modexp_local_amontmul)

        movq    k, %rdi
        movq    z, %rsi
        movq    x, %rdx
        movq    y, %rcx
        movq    m, %r8
        CFI_CALL(Lbignum_modexp_local_amontmul)

        movq    i, %rdx
        movq    %rdx, %rcx
        shrq    $6, %rdx
        movq    p, %rsi
        movq    (%rsi,%rdx,8), %rdi
        shrq    %cl, %rdi
        andq    $1, %rdi

        movq    k, %rsi
        movq    z, %rdx
        movq    z, %rcx
        movq    y, %r8
        CFI_CALL(Lbignum_modexp_local_mux)

        movq    i, %rax
        testq   %rax, %rax
        jnz     Lbignum_modexp_loop

// Convert back from Montgomery representation and copy the result
// (via a degenerate case of multiplexing) into the output buffer

        movq    k, %rdi
        movq    z, %rsi
        movq    z, %rdx
        movq    m, %rcx
        CFI_CALL(Lbignum_modexp_local_demont)

        xorl    %edi, %edi
        movq    k, %rsi
        movq    res, %rdx
        movq    z, %rcx
        movq    z, %r8
        CFI_CALL(Lbignum_modexp_local_mux)

// Restore the stack pointer and return

Lbignum_modexp_end:
        CFI_INC_RSP(VARSIZE)
        CFI_RET

#if WINDOWS_ABI
S2N_BN_SIZE_DIRECTIVE(Lbignum_modexp_standard)
#else
S2N_BN_SIZE_DIRECTIVE(bignum_modexp)
#endif

// Local copy of bignum_amontifier

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_modexp_local_amontifier)

Lbignum_modexp_local_amontifier:
        CFI_START
         CFI_PUSH(%rbp)
         CFI_PUSH(%rbx)
         CFI_PUSH(%r12)
         CFI_PUSH(%r13)
         movq   %rdx, %r12
         movq   %rcx, %r13
         testq  %rdi, %rdi
         je     Lbignum_modexp_amontifier_end
         xorq   %rbx, %rbx
Lbignum_modexp_copyinloop:
         movq   (%r12,%rbx,8), %rcx
         movq   %rcx, (%r13,%rbx,8)
         incq   %rbx
         cmpq   %rdi, %rbx
         jb     Lbignum_modexp_copyinloop
         movq   %rdi, %rbx
         decq   %rbx
         je     Lbignum_modexp_normalized
Lbignum_modexp_normloop:
         xorq   %rbp, %rbp
         movq   %rdi, %r11
         negq   %rcx
         movl   $0x0, %eax
Lbignum_modexp_shufloop:
         movq   %rax, %rcx
         movq   (%r13,%rbp,8), %rax
         cmovbq %rax, %rcx
         movq   %rcx, (%r13,%rbp,8)
         incq   %rbp
         decq   %r11
         jne    Lbignum_modexp_shufloop
         decq   %rbx
         jne    Lbignum_modexp_normloop
Lbignum_modexp_normalized:
         bsrq   %rcx, %rcx
         xorq   $0x3f, %rcx
         xorq   %r9, %r9
         xorq   %rbx, %rbx
Lbignum_modexp_bitloop:
         movq   (%r13,%rbx,8), %rax
         movq   %rax, %rbp
         shldq  %cl, %r9, %rax
         movq   %rax, (%r13,%rbx,8)
         movq   %rbp, %r9
         incq   %rbx
         cmpq   %rdi, %rbx
         jb     Lbignum_modexp_bitloop
         movq   -0x8(%r13,%rdi,8), %r11
         movl   $0x1, %r8d
         movq   %r11, %r9
         negq   %r9
         movl   $0x3e, %ebx
Lbignum_modexp_estloop:
         addq   %r8, %r8
         movq   %r11, %rax
         subq   %r9, %rax
         cmpq   %rax, %r9
         sbbq   %rax, %rax
         notq   %rax
         subq   %rax, %r8
         addq   %r9, %r9
         andq   %r11, %rax
         subq   %rax, %r9
         decq   %rbx
         jne    Lbignum_modexp_estloop
         incq   %r9
         cmpq   %r9, %r11
         adcq   $0x0, %r8
         xorq   %rcx, %rcx
         xorq   %rbx, %rbx
Lbignum_modexp_mulloop:
         movq   (%r13,%rbx,8), %rax
         mulq   %r8
         addq   %rcx, %rax
         adcq   $0x0, %rdx
         movq   %rax, (%rsi,%rbx,8)
         movq   %rdx, %rcx
         incq   %rbx
         cmpq   %rdi, %rbx
         jb     Lbignum_modexp_mulloop
         movabs $0x4000000000000000, %rax
         subq   %rax, %rcx
         sbbq   %r8, %r8
         notq   %r8
         xorq   %rcx, %rcx
         xorq   %rbx, %rbx
Lbignum_modexp_remloop:
         movq   (%r13,%rbx,8), %rax
         andq   %r8, %rax
         negq   %rcx
         sbbq   (%rsi,%rbx,8), %rax
         sbbq   %rcx, %rcx
         movq   %rax, (%rsi,%rbx,8)
         incq   %rbx
         cmpq   %rdi, %rbx
         jb     Lbignum_modexp_remloop
         xorq   %rcx, %rcx
         xorq   %rbp, %rbp
         xorq   %r9, %r9
Lbignum_modexp_dubloop1:
         movq   (%rsi,%rbp,8), %rax
         shrdq  $0x3f, %rax, %rcx
         negq   %r9
         sbbq   (%r13,%rbp,8), %rcx
         sbbq   %r9, %r9
         movq   %rcx, (%rsi,%rbp,8)
         movq   %rax, %rcx
         incq   %rbp
         cmpq   %rdi, %rbp
         jb     Lbignum_modexp_dubloop1
         shrq   $0x3f, %rcx
         addq   %r9, %rcx
         xorq   %rbp, %rbp
         xorq   %r9, %r9
Lbignum_modexp_corrloop1:
         movq   (%r13,%rbp,8), %rax
         andq   %rcx, %rax
         negq   %r9
         adcq   (%rsi,%rbp,8), %rax
         sbbq   %r9, %r9
         movq   %rax, (%rsi,%rbp,8)
         incq   %rbp
         cmpq   %rdi, %rbp
         jb     Lbignum_modexp_corrloop1
         xorq   %rcx, %rcx
         xorq   %rbp, %rbp
         xorq   %r9, %r9
Lbignum_modexp_dubloop2:
         movq   (%rsi,%rbp,8), %rax
         shrdq  $0x3f, %rax, %rcx
         negq   %r9
         sbbq   (%r13,%rbp,8), %rcx
         sbbq   %r9, %r9
         movq   %rcx, (%rsi,%rbp,8)
         movq   %rax, %rcx
         incq   %rbp
         cmpq   %rdi, %rbp
         jb     Lbignum_modexp_dubloop2
         shrq   $0x3f, %rcx
         addq   %r9, %rcx
         xorq   %rbp, %rbp
         xorq   %r9, %r9
Lbignum_modexp_corrloop2:
         movq   (%r13,%rbp,8), %rax
         andq   %rcx, %rax
         negq   %r9
         adcq   (%rsi,%rbp,8), %rax
         sbbq   %r9, %r9
         movq   %rax, (%rsi,%rbp,8)
         movq   %rax, (%r13,%rbp,8)
         incq   %rbp
         cmpq   %rdi, %rbp
         jb     Lbignum_modexp_corrloop2
         xorq   %r11, %r11
         movq   %rdi, %rbx
Lbignum_modexp_modloop:
         xorq   %r9, %r9
         movq   %rdi, %r8
         xorq   %rbp, %rbp
         xorq   %rcx, %rcx
Lbignum_modexp_cmaloop:
         adcq   %r9, %rcx
         sbbq   %r10, %r10
         movq   (%rsi,%rbp,8), %rax
         mulq   %r11
         subq   %r10, %rdx
         addq   %rcx, %rax
         movq   (%r13,%rbp,8), %r9
         movq   %rax, (%r13,%rbp,8)
         movq   %rdx, %rcx
         incq   %rbp
         decq   %r8
         jne    Lbignum_modexp_cmaloop
         adcq   %rcx, %r9
         movq   %r9, %r11
         sbbq   %r10, %r10
         xorq   %rbp, %rbp
         xorq   %rcx, %rcx
Lbignum_modexp_oaloop:
         movq   (%r13,%rbp,8), %rax
         movq   (%rsi,%rbp,8), %r9
         andq   %r10, %r9
         negq   %rcx
         adcq   %r9, %rax
         sbbq   %rcx, %rcx
         movq   %rax, (%r13,%rbp,8)
         incq   %rbp
         cmpq   %rdi, %rbp
         jb     Lbignum_modexp_oaloop
         subq   %rcx, %r11
         decq   %rbx
         jne    Lbignum_modexp_modloop
         movq   (%r12), %rax
         movq   %rax, %rcx
         movq   %rax, %r9
         shlq   $0x2, %rcx
         subq   %rcx, %r9
         xorq   $0x2, %r9
         movq   %r9, %rcx
         imulq  %rax, %rcx
         movl   $0x2, %eax
         addq   %rcx, %rax
         addq   $0x1, %rcx
         imulq  %rax, %r9
         imulq  %rcx, %rcx
         movl   $0x1, %eax
         addq   %rcx, %rax
         imulq  %rax, %r9
         imulq  %rcx, %rcx
         movl   $0x1, %eax
         addq   %rcx, %rax
         imulq  %rax, %r9
         imulq  %rcx, %rcx
         movl   $0x1, %eax
         addq   %rcx, %rax
         imulq  %rax, %r9
         movq   (%r13), %rcx
         imulq  %rcx, %r9
         movq   (%r12), %rax
         mulq   %r9
         addq   %rcx, %rax
         movq   %rdx, %rcx
         movl   $0x1, %ebp
         movq   %rdi, %r8
         decq   %r8
         je     Lbignum_modexp_montifend
Lbignum_modexp_montifloop:
         adcq   (%r13,%rbp,8), %rcx
         sbbq   %r10, %r10
         movq   (%r12,%rbp,8), %rax
         mulq   %r9
         subq   %r10, %rdx
         addq   %rcx, %rax
         movq   %rax, -0x8(%r13,%rbp,8)
         movq   %rdx, %rcx
         incq   %rbp
         decq   %r8
         jne    Lbignum_modexp_montifloop
Lbignum_modexp_montifend:
         adcq   %rcx, %r11
         sbbq   %r10, %r10
         movq   %r11, -0x8(%r13,%rdi,8)
         xorq   %rbp, %rbp
         xorq   %rcx, %rcx
Lbignum_modexp_osloop:
         movq   (%r13,%rbp,8), %rax
         movq   (%r12,%rbp,8), %r9
         andq   %r10, %r9
         negq   %rcx
         sbbq   %r9, %rax
         sbbq   %rcx, %rcx
         movq   %rax, (%rsi,%rbp,8)
         incq   %rbp
         cmpq   %rdi, %rbp
         jb     Lbignum_modexp_osloop
Lbignum_modexp_amontifier_end:
         CFI_POP(%r13)
         CFI_POP(%r12)
         CFI_POP(%rbx)
         CFI_POP(%rbp)
         CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_modexp_local_amontifier)

// Local copy of bignum_amontmul

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_modexp_local_amontmul)

Lbignum_modexp_local_amontmul:
        CFI_START
         CFI_PUSH(%rbx)
         CFI_PUSH(%rbp)
         CFI_PUSH(%r12)
         CFI_PUSH(%r13)
         CFI_PUSH(%r14)
         CFI_PUSH(%r15)
         CFI_DEC_RSP(8)
         testq  %rdi, %rdi
         je     Lbignum_modexp_amont_end
         movq   %rdx, %r9
         movq   (%r8), %rax
         movq   %rax, %rdx
         movq   %rax, %rbx
         shlq   $0x2, %rdx
         subq   %rdx, %rbx
         xorq   $0x2, %rbx
         movq   %rbx, %rdx
         imulq  %rax, %rdx
         movl   $0x2, %eax
         addq   %rdx, %rax
         addq   $0x1, %rdx
         imulq  %rax, %rbx
         imulq  %rdx, %rdx
         movl   $0x1, %eax
         addq   %rdx, %rax
         imulq  %rax, %rbx
         imulq  %rdx, %rdx
         movl   $0x1, %eax
         addq   %rdx, %rax
         imulq  %rax, %rbx
         imulq  %rdx, %rdx
         movl   $0x1, %eax
         addq   %rdx, %rax
         imulq  %rax, %rbx
         movq   %rbx, (%rsp)
         xorq   %r13, %r13
         xorq   %rbx, %rbx
Lbignum_modexp_zoop:
         movq   %r13, (%rsi,%rbx,8)
         incq   %rbx
         cmpq   %rdi, %rbx
         jb     Lbignum_modexp_zoop
         xorq   %r14, %r14
Lbignum_modexp_outeramontloop:
         movq   (%r9,%r13,8), %rbp
         xorq   %rbx, %rbx
         xorq   %r10, %r10
         xorq   %r15, %r15
         movq   %rdi, %r12
Lbignum_modexp_maddloop:
         adcq   (%rsi,%rbx,8), %r10
         sbbq   %r11, %r11
         movq   (%rcx,%rbx,8), %rax
         mulq   %rbp
         subq   %r11, %rdx
         addq   %r10, %rax
         movq   %rax, (%rsi,%rbx,8)
         movq   %rdx, %r10
         incq   %rbx
         decq   %r12
         jne    Lbignum_modexp_maddloop
         adcq   %r10, %r14
         adcq   %r15, %r15
         movq   (%rsi), %r11
         movq   (%rsp), %rbp
         imulq  %r11, %rbp
         movq   (%r8), %rax
         mulq   %rbp
         addq   %r11, %rax
         movq   %rdx, %r10
         movl   $0x1, %ebx
         movq   %rdi, %r12
         decq   %r12
         je     Lbignum_modexp_montend
Lbignum_modexp_montloop:
         adcq   (%rsi,%rbx,8), %r10
         sbbq   %r11, %r11
         movq   (%r8,%rbx,8), %rax
         mulq   %rbp
         subq   %r11, %rdx
         addq   %r10, %rax
         movq   %rax, -0x8(%rsi,%rbx,8)
         movq   %rdx, %r10
         incq   %rbx
         decq   %r12
         jne    Lbignum_modexp_montloop
Lbignum_modexp_montend:
         adcq   %r14, %r10
         adcq   $0x0, %r15
         movq   %r15, %r14
         movq   %r10, -0x8(%rsi,%rbx,8)
         incq   %r13
         cmpq   %rdi, %r13
         jb     Lbignum_modexp_outeramontloop
         xorq   %rbp, %rbp
         subq   %r14, %rbp
         xorq   %r11, %r11
         xorq   %rbx, %rbx
Lbignum_modexp_acorrloop:
         movq   (%r8,%rbx,8), %rax
         andq   %rbp, %rax
         negq   %r11
         sbbq   %rax, (%rsi,%rbx,8)
         sbbq   %r11, %r11
         incq   %rbx
         cmpq   %rdi, %rbx
         jb     Lbignum_modexp_acorrloop
Lbignum_modexp_amont_end:
         CFI_INC_RSP(8)
         CFI_POP(%r15)
         CFI_POP(%r14)
         CFI_POP(%r13)
         CFI_POP(%r12)
         CFI_POP(%rbp)
         CFI_POP(%rbx)
         CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_modexp_local_amontmul)

// Local copy of bignum_demont

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_modexp_local_demont)

Lbignum_modexp_local_demont:
        CFI_START
         CFI_PUSH(%rbx)
         CFI_PUSH(%rbp)
         CFI_PUSH(%r12)
         testq  %rdi, %rdi
         je     Lbignum_modexp_demont_end
         movq   (%rcx), %rax
         movq   %rax, %rbx
         movq   %rax, %r8
         shlq   $0x2, %rbx
         subq   %rbx, %r8
         xorq   $0x2, %r8
         movq   %r8, %rbx
         imulq  %rax, %rbx
         movl   $0x2, %eax
         addq   %rbx, %rax
         addq   $0x1, %rbx
         imulq  %rax, %r8
         imulq  %rbx, %rbx
         movl   $0x1, %eax
         addq   %rbx, %rax
         imulq  %rax, %r8
         imulq  %rbx, %rbx
         movl   $0x1, %eax
         addq   %rbx, %rax
         imulq  %rax, %r8
         imulq  %rbx, %rbx
         movl   $0x1, %eax
         addq   %rbx, %rax
         imulq  %rax, %r8
         xorq   %rbx, %rbx
Lbignum_modexp_iloop:
         movq   (%rdx,%rbx,8), %rax
         movq   %rax, (%rsi,%rbx,8)
         incq   %rbx
         cmpq   %rdi, %rbx
         jb     Lbignum_modexp_iloop
         xorq   %r9, %r9
Lbignum_modexp_outerdemontloop:
         movq   (%rsi), %r11
         movq   %r8, %rbp
         imulq  %r11, %rbp
         movq   (%rcx), %rax
         mulq   %rbp
         addq   %r11, %rax
         movq   %rdx, %r10
         movl   $0x1, %ebx
         movq   %rdi, %r12
         decq   %r12
         je     Lbignum_modexp_demontend
Lbignum_modexp_demontloop:
         adcq   (%rsi,%rbx,8), %r10
         sbbq   %r11, %r11
         movq   (%rcx,%rbx,8), %rax
         mulq   %rbp
         subq   %r11, %rdx
         addq   %r10, %rax
         movq   %rax, -0x8(%rsi,%rbx,8)
         movq   %rdx, %r10
         incq   %rbx
         decq   %r12
         jne    Lbignum_modexp_demontloop
Lbignum_modexp_demontend:
         adcq   $0x0, %r10
         movq   %r10, -0x8(%rsi,%rbx,8)
         incq   %r9
         cmpq   %rdi, %r9
         jb     Lbignum_modexp_outerdemontloop
         xorq   %rbx, %rbx
         movq   %rdi, %r12
Lbignum_modexp_cmploop:
         movq   (%rsi,%rbx,8), %rax
         sbbq   (%rcx,%rbx,8), %rax
         incq   %rbx
         decq   %r12
         jne    Lbignum_modexp_cmploop
         sbbq   %rbp, %rbp
         notq   %rbp
         xorq   %r11, %r11
         xorq   %rbx, %rbx
Lbignum_modexp_dcorrloop:
         movq   (%rcx,%rbx,8), %rax
         andq   %rbp, %rax
         negq   %r11
         sbbq   %rax, (%rsi,%rbx,8)
         sbbq   %r11, %r11
         incq   %rbx
         cmpq   %rdi, %rbx
         jb     Lbignum_modexp_dcorrloop
Lbignum_modexp_demont_end:
         CFI_POP(%r12)
         CFI_POP(%rbp)
         CFI_POP(%rbx)
         CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_modexp_local_demont)

// Local copy of bignum_mux

S2N_BN_FUNCTION_TYPE_DIRECTIVE(Lbignum_modexp_local_mux)

Lbignum_modexp_local_mux:
        CFI_START
         testq  %rsi, %rsi
         je     Lbignum_modexp_muxend
         xorq   %r9, %r9
         negq   %rdi
Lbignum_modexp_muxloop:
         movq   (%rcx,%r9,8), %rax
         movq   (%r8,%r9,8), %rdi
         cmovae %rdi, %rax
         movq   %rax, (%rdx,%r9,8)
         incq   %r9
         decq   %rsi
         jne    Lbignum_modexp_muxloop
Lbignum_modexp_muxend:
         CFI_RET

S2N_BN_SIZE_DIRECTIVE(Lbignum_modexp_local_mux)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
