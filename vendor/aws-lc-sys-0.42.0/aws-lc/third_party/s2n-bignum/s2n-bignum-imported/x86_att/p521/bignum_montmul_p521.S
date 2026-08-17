// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery multiply, z := (x * y / 2^576) mod p_521
// Inputs x[9], y[9]; output z[9]
//
//    extern void bignum_montmul_p521(uint64_t z[static 9],
//                                    const uint64_t x[static 9],
//                                    const uint64_t y[static 9]);
//
// Does z := (x * y / 2^576) mod p_521, assuming x < p_521, y < p_521. This
// means the Montgomery base is the "native size" 2^{9*64} = 2^576; since
// p_521 is a Mersenne prime the basic modular multiplication bignum_mul_p521
// can be considered a Montgomery operation to base 2^521.
//
// Standard x86-64 ABI: RDI = z, RSI = x, RDX = y
// Microsoft x64 ABI:   RCX = z, RDX = x, R8 = y
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_montmul_p521)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(bignum_montmul_p521)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_montmul_p521)
        .text

#define z %rdi
#define x %rsi

// Copied in

#define y %rcx

// mulpadd (high,low,x) adds rdx * x to a register-pair (high,low)
// maintaining consistent double-carrying with adcx and adox,
// using %rax and %rbx as temporaries.

#define mulpadd(high,low,x)             \
        mulxq   x, %rax, %rbx ;            \
        adcxq   %rax, low ;               \
        adoxq   %rbx, high

S2N_BN_SYMBOL(bignum_montmul_p521):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_PUSH(%rdi)
        CFI_PUSH(%rsi)
        movq    %rcx, %rdi
        movq    %rdx, %rsi
        movq    %r8, %rdx
#endif

// Save more registers to play with and make temporary space on stack

        CFI_PUSH(%rbp)
        CFI_PUSH(%rbx)
        CFI_PUSH(%r12)
        CFI_PUSH(%r13)
        CFI_PUSH(%r14)
        CFI_PUSH(%r15)
        CFI_DEC_RSP(64)

// Copy y into a safe register to start with

        movq    %rdx, y

// Clone of the main body of bignum_8_16, writing back the low 8 words
// to the stack and keeping the top half in %r15,...,%r8

        xorl   %ebp, %ebp
        movq   (y), %rdx
        mulxq  (x), %r8, %r9
        movq   %r8, (%rsp)
        mulxq  0x8(x), %rbx, %r10
        adcq   %rbx, %r9
        mulxq  0x10(x), %rbx, %r11
        adcq   %rbx, %r10
        mulxq  0x18(x), %rbx, %r12
        adcq   %rbx, %r11
        mulxq  0x20(x), %rbx, %r13
        adcq   %rbx, %r12
        mulxq  0x28(x), %rbx, %r14
        adcq   %rbx, %r13
        mulxq  0x30(x), %rbx, %r15
        adcq   %rbx, %r14
        mulxq  0x38(x), %rbx, %r8
        adcq   %rbx, %r15
        adcq   %rbp, %r8
        movq   0x8(y), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        movq   %r9, 0x8(%rsp)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x38(x), %rax, %r9
        adcxq  %rax, %r8
        adoxq  %rbp, %r9
        adcq   %rbp, %r9
        movq   0x10(y), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        movq   %r10, 0x10(%rsp)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x38(x), %rax, %r10
        adcxq  %rax, %r9
        adoxq  %rbp, %r10
        adcq   %rbp, %r10
        movq   0x18(y), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        movq   %r11, 0x18(%rsp)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x38(x), %rax, %r11
        adcxq  %rax, %r10
        adoxq  %rbp, %r11
        adcq   %rbp, %r11
        movq   0x20(y), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        movq   %r12, 0x20(%rsp)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x38(x), %rax, %r12
        adcxq  %rax, %r11
        adoxq  %rbp, %r12
        adcq   %rbp, %r12
        movq   0x28(y), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        movq   %r13, 0x28(%rsp)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x38(x), %rax, %r13
        adcxq  %rax, %r12
        adoxq  %rbp, %r13
        adcq   %rbp, %r13
        movq   0x30(y), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r14
        adoxq  %rbx, %r15
        movq   %r14, 0x30(%rsp)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x38(x), %rax, %r14
        adcxq  %rax, %r13
        adoxq  %rbp, %r14
        adcq   %rbp, %r14
        movq   0x38(y), %rdx
        xorl   %ebp, %ebp
        mulxq  (x), %rax, %rbx
        adcxq  %rax, %r15
        adoxq  %rbx, %r8
        movq   %r15, 0x38(%rsp)
        mulxq  0x8(x), %rax, %rbx
        adcxq  %rax, %r8
        adoxq  %rbx, %r9
        mulxq  0x10(x), %rax, %rbx
        adcxq  %rax, %r9
        adoxq  %rbx, %r10
        mulxq  0x18(x), %rax, %rbx
        adcxq  %rax, %r10
        adoxq  %rbx, %r11
        mulxq  0x20(x), %rax, %rbx
        adcxq  %rax, %r11
        adoxq  %rbx, %r12
        mulxq  0x28(x), %rax, %rbx
        adcxq  %rax, %r12
        adoxq  %rbx, %r13
        mulxq  0x30(x), %rax, %rbx
        adcxq  %rax, %r13
        adoxq  %rbx, %r14
        mulxq  0x38(x), %rax, %r15
        adcxq  %rax, %r14
        adoxq  %rbp, %r15
        adcq   %rbp, %r15

// Accumulate x[8] * y[0..7], extending the window to %rbp,%r15,...,%r8

        movq    64(x), %rdx
        xorl    %ebp, %ebp
        mulpadd(%r9,%r8,(y))
        mulpadd(%r10,%r9,8(y))
        mulpadd(%r11,%r10,16(y))
        mulpadd(%r12,%r11,24(y))
        mulpadd(%r13,%r12,32(y))
        mulpadd(%r14,%r13,40(y))
        mulpadd(%r15,%r14,48(y))
        mulxq   56(y), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbp, %rbx
        adcq    %rbx, %rbp

// Accumulate y[8] * x[0..8] within this extended window %rbp,%r15,...,%r8

        movq    64(y), %rdx
        xorl    %eax, %eax
        mulpadd(%r9,%r8,(x))
        mulpadd(%r10,%r9,8(x))
        mulpadd(%r11,%r10,16(x))
        mulpadd(%r12,%r11,24(x))
        mulpadd(%r13,%r12,32(x))
        mulpadd(%r14,%r13,40(x))
        mulpadd(%r15,%r14,48(x))
        mulxq   56(x), %rax, %rbx
        adcxq   %rax, %r15
        adoxq   %rbx, %rbp
        mulxq   64(x), %rax, %rbx
        adcq    %rax, %rbp

// Rotate the upper portion right 9 bits since 2^512 == 2^-9 (mod p_521)
// Let rotated result %rbp,%r15,%r14,...,%r8 be h (high) and %rsp[0..7] be l (low)

        movq    %r8, %rax
        andq    $0x1FF, %rax
        shrdq   $9, %r9, %r8
        shrdq   $9, %r10, %r9
        shrdq   $9, %r11, %r10
        shrdq   $9, %r12, %r11
        shrdq   $9, %r13, %r12
        shrdq   $9, %r14, %r13
        shrdq   $9, %r15, %r14
        shrdq   $9, %rbp, %r15
        shrq    $9, %rbp
        addq    %rax, %rbp

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
        adcq    $~0x1FF, %rbp

// Now CF is set <=> h + l + 1 >= 2^521 <=> h + l >= p_521,
// in which case the lower 521 bits are already right. Otherwise if
// CF is clear, we want to subtract 1. Hence subtract the complement
// of the carry flag then mask the top word, which scrubs the
// padding in either case.

        cmc
        sbbq    $0, %r8
        sbbq    $0, %r9
        sbbq    $0, %r10
        sbbq    $0, %r11
        sbbq    $0, %r12
        sbbq    $0, %r13
        sbbq    $0, %r14
        sbbq    $0, %r15
        sbbq    $0, %rbp
        andq    $0x1FF, %rbp

// So far, this has been the same as a pure modular multiply.
// Now finally the Montgomery ingredient, which is just a 521-bit
// rotation by 9*64 - 521 = 55 bits right. Write digits back as
// they are created.

        movq    %r8, %rax
        shrdq   $55, %r9, %r8
        movq    %r8, (z)
        shrdq   $55, %r10, %r9
        movq    %r9, 8(z)
        shrdq   $55, %r11, %r10
        shlq    $9, %rax
        movq    %r10, 16(z)
        shrdq   $55, %r12, %r11
        movq    %r11, 24(z)
        shrdq   $55, %r13, %r12
        movq    %r12, 32(z)
        orq     %rax, %rbp
        shrdq   $55, %r14, %r13
        movq    %r13, 40(z)
        shrdq   $55, %r15, %r14
        movq    %r14, 48(z)
        shrdq   $55, %rbp, %r15
        movq    %r15, 56(z)
        shrq    $55, %rbp
        movq    %rbp, 64(z)

// Restore registers and return

        CFI_INC_RSP(64)
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

S2N_BN_SIZE_DIRECTIVE(bignum_montmul_p521)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
