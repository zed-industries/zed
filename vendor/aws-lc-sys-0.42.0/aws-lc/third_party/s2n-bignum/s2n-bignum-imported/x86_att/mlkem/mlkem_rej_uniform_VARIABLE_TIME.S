// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Uniform rejection sampling for ML-KEM
// Inputs *buf (unsigned bytes), buflen, table (unsigned bytes); output r[256] (signed 16-bit words), return
//
// extern uint64_t mlkem_rej_uniform_VARIABLE_TIME
//                     (int16_t r[S2N_BIGNUM_STATIC 256],
//                      const uint8_t *buf,uint64_t buflen,
//                      const uint8_t *table);
//
// Interprets the input buffer as packed 12-bit numbers with a length of
// buflen bytes, assumed to be a multiple of 12. Fills the output array
// with those numbers from the packed buffer that are < 3329, in the order
// of appearance, returning the total number of entries written, with a
// maximum of 256. The table argument is a specific precomputed table of
// constants that is defined in this file (see also our test code):
//
//   https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/native/aarch64/src/rej_uniform_table.c
//
// Unique (at the moment) among s2n-bignum functions this is *not* a
// constant-time function. The time taken depends not only on the
// buffer size "buflen", but also how many elements of the buffer are
// needed to provide the 256 entries for the output.
//
// Standard x86-64 ABI: RDI = r, RSI = buf, RDX = buflen, RCX = table
// Microsoft x64 ABI:   RCX = r, RDX = buf, R8 = buflen, R9 = table
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_rej_uniform_VARIABLE_TIME)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_rej_uniform_VARIABLE_TIME)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_rej_uniform_VARIABLE_TIME)
        .text
        .balign 4

// This code is effectively the same as the original here:
// https://github.com/pq-code-package/mlkem-native/blob/main/mlkem/src/native/x86_64/src/rej_uniform_asm.S
//
// There are a few changes:
//
// * Use %xmm3 in place of %xmm6, so we don't need to save/restore it for Windows
// * Directly test for null input (buflen = 0), and skip body and return 0

S2N_BN_SYMBOL(mlkem_rej_uniform_VARIABLE_TIME):
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

        CFI_DEC_RSP(0x210)

        xorl    %eax, %eax
        testq   %rdx, %rdx
        jz      Lmlkem_rej_uniform_VARIABLE_TIME_end

        movabsq $0xd010d010d010d01, %rax
        movq    %rax, %xmm0
        pinsrq  $0x1, %rax, %xmm0
        movabsq $0xfff0fff0fff0fff, %rax
        movq    %rax, %xmm5
        pinsrq  $0x1, %rax, %xmm5
        movabsq $0x504040302010100, %rax
        movq    %rax, %xmm4
        movabsq $0xb0a0a0908070706, %rax
        pinsrq  $0x1, %rax, %xmm4
        movq    $0x0, %rax
        movq    $0x0, %r8
        movq    $0x5555, %r9

Lmlkem_rej_uniform_VARIABLE_TIME_loop:
        movq    (%rsi,%r8), %xmm2
        pinsrd  $0x2, 8(%rsi,%r8,1), %xmm2
        pshufb  %xmm4, %xmm2
        movdqa  %xmm2, %xmm3
        psrlw   $0x4, %xmm3
        pblendw $0xaa, %xmm3, %xmm2
        pand    %xmm5, %xmm2
        movdqa  %xmm0, %xmm1
        pcmpgtw %xmm2, %xmm1
        pmovmskb %xmm1, %r11d
        pext    %r9, %r11, %r11
        movq    %r11, %r10
        shlq    $0x4, %r10
        movdqu  (%rcx,%r10), %xmm3
        pshufb  %xmm3, %xmm2
        movdqu  %xmm2, (%rsp,%rax,2)
        popcnt  %r11, %r11
        addq    %r11, %rax
        cmpq    $0x100, %rax
        jae     Lmlkem_rej_uniform_VARIABLE_TIME_final_copy
        addq    $0xc, %r8
        cmpq    %r8, %rdx
        ja      Lmlkem_rej_uniform_VARIABLE_TIME_loop

Lmlkem_rej_uniform_VARIABLE_TIME_final_copy:
        movq    $0x100, %rcx
        cmpq    $0x100, %rax
        cmova   %rcx, %rax
        movq    %rsp, %rsi
        movq    %rax, %rcx
        shlq    $1, %rcx
        repz  movsb

Lmlkem_rej_uniform_VARIABLE_TIME_end:
        CFI_INC_RSP(0x210)

#if WINDOWS_ABI
        CFI_POP(%rsi)
        CFI_POP(%rdi)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_rej_uniform_VARIABLE_TIME)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
