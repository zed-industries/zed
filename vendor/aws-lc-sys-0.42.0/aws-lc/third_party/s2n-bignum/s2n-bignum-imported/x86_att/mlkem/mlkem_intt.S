// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Inverse number-theoretic transform from ML-KEM
// Input a[256], z_01234[80], z_56[384] (all signed 16-bit words); output a[256] (signed 16-bit words).
//
// The transform is in-place with input and output a[256], with the input in
// bitreversed order and the output mapped into the Montgomery domain via
// x |-> (2^16 * x) mod 3329. The second parameter is expected to point to
// a table of constants whose definitions can be found in the mlkem-native
// repo or our "tests/test.c".
//
// extern void mlkem_intt_x86(int16_t a[static 256], const int16_t qdata[static 624]);
//
// Standard x86-64 ABI: RDI = a, RSI = qdata
// Microsoft x64 ABI:   RCX = a, RDX = qdata
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_intt_x86)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_intt_x86)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_intt_x86)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_intt_x86):
 CFI_START
  _CET_ENDBR

#if WINDOWS_ABI
        CFI_DEC_RSP(176)
        CFI_STACKSAVEU(%xmm6,0)
        CFI_STACKSAVEU(%xmm7,16)
        CFI_STACKSAVEU(%xmm8,32)
        CFI_STACKSAVEU(%xmm9,48)
        CFI_STACKSAVEU(%xmm10,64)
        CFI_STACKSAVEU(%xmm11,80)
        CFI_STACKSAVEU(%xmm12,96)
        CFI_STACKSAVEU(%xmm13,112)
        CFI_STACKSAVEU(%xmm14,128)
        CFI_STACKSAVEU(%xmm15,144)
        CFI_STACKSAVE(%rdi,160)
        CFI_STACKSAVE(%rsi,168)
        mov %rcx, %rdi
        mov %rdx, %rsi
#endif

 mov $0xd010d01, %eax
 vmovd %eax, %xmm0
 vpbroadcastd %xmm0, %ymm0
 mov $0xd8a1d8a1, %eax
 vmovd %eax, %xmm2
 vpbroadcastd %xmm2, %ymm2
 mov $0x5a105a1, %eax
 vmovd %eax, %xmm3
 vpbroadcastd %xmm3, %ymm3
 vmovdqa (%rdi), %ymm4
 vmovdqa 64(%rdi), %ymm6
 vmovdqa 32(%rdi), %ymm5
 vmovdqa 96(%rdi), %ymm7
 vpmullw %ymm2, %ymm4, %ymm12
 vpmulhw %ymm3, %ymm4, %ymm4
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm4, %ymm4
 vpmullw %ymm2, %ymm6, %ymm12
 vpmulhw %ymm3, %ymm6, %ymm6
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm6, %ymm6
 vpmullw %ymm2, %ymm5, %ymm12
 vpmulhw %ymm3, %ymm5, %ymm5
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm5, %ymm5
 vpmullw %ymm2, %ymm7, %ymm12
 vpmulhw %ymm3, %ymm7, %ymm7
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm7, %ymm7
 vmovdqa 128(%rdi), %ymm8
 vmovdqa 192(%rdi), %ymm10
 vmovdqa 160(%rdi), %ymm9
 vmovdqa 224(%rdi), %ymm11
 vpmullw %ymm2, %ymm8, %ymm12
 vpmulhw %ymm3, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm8, %ymm8
 vpmullw %ymm2, %ymm10, %ymm12
 vpmulhw %ymm3, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm10, %ymm10
 vpmullw %ymm2, %ymm9, %ymm12
 vpmulhw %ymm3, %ymm9, %ymm9
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm9, %ymm9
 vpmullw %ymm2, %ymm11, %ymm12
 vpmulhw %ymm3, %ymm11, %ymm11
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm11, %ymm11
 vpermq $0x4e, 928(%rsi), %ymm15
 vpermq $0x4e, 864(%rsi), %ymm1
 vpermq $0x4e, 960(%rsi), %ymm2
 vpermq $0x4e, 896(%rsi), %ymm3
 vmovdqa (%rsi), %ymm12
 vpshufb %ymm12, %ymm15, %ymm15
 vpshufb %ymm12, %ymm1, %ymm1
 vpshufb %ymm12, %ymm2, %ymm2
 vpshufb %ymm12, %ymm3, %ymm3
 vpsubw %ymm4, %ymm6, %ymm12
 vpaddw %ymm6, %ymm4, %ymm4
 vpsubw %ymm5, %ymm7, %ymm13
 vpmullw %ymm15, %ymm12, %ymm6
 vpaddw %ymm7, %ymm5, %ymm5
 vpsubw %ymm8, %ymm10, %ymm14
 vpmullw %ymm15, %ymm13, %ymm7
 vpaddw %ymm10, %ymm8, %ymm8
 vpsubw %ymm9, %ymm11, %ymm15
 vpmullw %ymm1, %ymm14, %ymm10
 vpaddw %ymm11, %ymm9, %ymm9
 vpmullw %ymm1, %ymm15, %ymm11
 vpmulhw %ymm2, %ymm12, %ymm12
 vpmulhw %ymm2, %ymm13, %ymm13
 vpmulhw %ymm3, %ymm14, %ymm14
 vpmulhw %ymm3, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm6, %ymm6
 vpmulhw %ymm0, %ymm7, %ymm7
 vpmulhw %ymm0, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm6, %ymm12, %ymm6
 vpsubw %ymm7, %ymm13, %ymm7
 vpsubw %ymm10, %ymm14, %ymm10
 vpsubw %ymm11, %ymm15, %ymm11
 vpermq $0x4e, 800(%rsi), %ymm2
 vpermq $0x4e, 832(%rsi), %ymm3
 vmovdqa (%rsi), %ymm1
 vpshufb %ymm1, %ymm2, %ymm2
 vpshufb %ymm1, %ymm3, %ymm3
 vpsubw %ymm4, %ymm8, %ymm12
 vpaddw %ymm8, %ymm4, %ymm4
 vpsubw %ymm5, %ymm9, %ymm13
 vpmullw %ymm2, %ymm12, %ymm8
 vpaddw %ymm9, %ymm5, %ymm5
 vpsubw %ymm6, %ymm10, %ymm14
 vpmullw %ymm2, %ymm13, %ymm9
 vpaddw %ymm10, %ymm6, %ymm6
 vpsubw %ymm7, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm10
 vpaddw %ymm11, %ymm7, %ymm7
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm3, %ymm12, %ymm12
 vpmulhw %ymm3, %ymm13, %ymm13
 vpmulhw %ymm3, %ymm14, %ymm14
 vpmulhw %ymm3, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm9, %ymm9
 vpmulhw %ymm0, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm8, %ymm12, %ymm8
 vpsubw %ymm9, %ymm13, %ymm9
 vpsubw %ymm10, %ymm14, %ymm10
 vpsubw %ymm11, %ymm15, %ymm11
 vpslld $0x10, %ymm5, %ymm3
 vpblendw $0xaa, %ymm3, %ymm4, %ymm3
 vpsrld $0x10, %ymm4, %ymm4
 vpblendw $0xaa, %ymm5, %ymm4, %ymm5
 vpslld $0x10, %ymm7, %ymm4
 vpblendw $0xaa, %ymm4, %ymm6, %ymm4
 vpsrld $0x10, %ymm6, %ymm6
 vpblendw $0xaa, %ymm7, %ymm6, %ymm7
 vpslld $0x10, %ymm9, %ymm6
 vpblendw $0xaa, %ymm6, %ymm8, %ymm6
 vpsrld $0x10, %ymm8, %ymm8
 vpblendw $0xaa, %ymm9, %ymm8, %ymm9
 vpslld $0x10, %ymm11, %ymm8
 vpblendw $0xaa, %ymm8, %ymm10, %ymm8
 vpsrld $0x10, %ymm10, %ymm10
 vpblendw $0xaa, %ymm11, %ymm10, %ymm11
 vmovdqa 32(%rsi), %ymm12
 vpermd 736(%rsi), %ymm12, %ymm2
 vpermd 768(%rsi), %ymm12, %ymm10
 vpsubw %ymm3, %ymm5, %ymm12
 vpaddw %ymm5, %ymm3, %ymm3
 vpsubw %ymm4, %ymm7, %ymm13
 vpmullw %ymm2, %ymm12, %ymm5
 vpaddw %ymm7, %ymm4, %ymm4
 vpsubw %ymm6, %ymm9, %ymm14
 vpmullw %ymm2, %ymm13, %ymm7
 vpaddw %ymm9, %ymm6, %ymm6
 vpsubw %ymm8, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm9
 vpaddw %ymm11, %ymm8, %ymm8
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm10, %ymm12, %ymm12
 vpmulhw %ymm10, %ymm13, %ymm13
 vpmulhw %ymm10, %ymm14, %ymm14
 vpmulhw %ymm10, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm5, %ymm5
 vpmulhw %ymm0, %ymm7, %ymm7
 vpmulhw %ymm0, %ymm9, %ymm9
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm5, %ymm12, %ymm5
 vpsubw %ymm7, %ymm13, %ymm7
 vpsubw %ymm9, %ymm14, %ymm9
 vpsubw %ymm11, %ymm15, %ymm11
 mov $0x4ebf4ebf, %eax
 vmovd %eax, %xmm1
 vpbroadcastd %xmm1, %ymm1
 vpmulhw %ymm1, %ymm3, %ymm12
 vpsraw $0xa, %ymm12, %ymm12
 vpmullw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm3, %ymm3
 vmovsldup %ymm4, %ymm10
 vpblendd $0xaa, %ymm10, %ymm3, %ymm10
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm4, %ymm3, %ymm4
 vmovsldup %ymm8, %ymm3
 vpblendd $0xaa, %ymm3, %ymm6, %ymm3
 vpsrlq $0x20, %ymm6, %ymm6
 vpblendd $0xaa, %ymm8, %ymm6, %ymm8
 vmovsldup %ymm7, %ymm6
 vpblendd $0xaa, %ymm6, %ymm5, %ymm6
 vpsrlq $0x20, %ymm5, %ymm5
 vpblendd $0xaa, %ymm7, %ymm5, %ymm7
 vmovsldup %ymm11, %ymm5
 vpblendd $0xaa, %ymm5, %ymm9, %ymm5
 vpsrlq $0x20, %ymm9, %ymm9
 vpblendd $0xaa, %ymm11, %ymm9, %ymm11
 vpermq $0x1b, 672(%rsi), %ymm2
 vpermq $0x1b, 704(%rsi), %ymm9
 vpsubw %ymm10, %ymm4, %ymm12
 vpaddw %ymm4, %ymm10, %ymm10
 vpsubw %ymm3, %ymm8, %ymm13
 vpmullw %ymm2, %ymm12, %ymm4
 vpaddw %ymm8, %ymm3, %ymm3
 vpsubw %ymm6, %ymm7, %ymm14
 vpmullw %ymm2, %ymm13, %ymm8
 vpaddw %ymm7, %ymm6, %ymm6
 vpsubw %ymm5, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm7
 vpaddw %ymm11, %ymm5, %ymm5
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm9, %ymm12, %ymm12
 vpmulhw %ymm9, %ymm13, %ymm13
 vpmulhw %ymm9, %ymm14, %ymm14
 vpmulhw %ymm9, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm4, %ymm4
 vpmulhw %ymm0, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm7, %ymm7
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm4, %ymm12, %ymm4
 vpsubw %ymm8, %ymm13, %ymm8
 vpsubw %ymm7, %ymm14, %ymm7
 vpsubw %ymm11, %ymm15, %ymm11
 vpmulhw %ymm1, %ymm10, %ymm12
 vpsraw $0xa, %ymm12, %ymm12
 vpmullw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm10, %ymm10
 vpunpcklqdq %ymm3, %ymm10, %ymm9
 vpunpckhqdq %ymm3, %ymm10, %ymm3
 vpunpcklqdq %ymm5, %ymm6, %ymm10
 vpunpckhqdq %ymm5, %ymm6, %ymm5
 vpunpcklqdq %ymm8, %ymm4, %ymm6
 vpunpckhqdq %ymm8, %ymm4, %ymm8
 vpunpcklqdq %ymm11, %ymm7, %ymm4
 vpunpckhqdq %ymm11, %ymm7, %ymm11
 vpermq $0x4e, 608(%rsi), %ymm2
 vpermq $0x4e, 640(%rsi), %ymm7
 vpsubw %ymm9, %ymm3, %ymm12
 vpaddw %ymm3, %ymm9, %ymm9
 vpsubw %ymm10, %ymm5, %ymm13
 vpmullw %ymm2, %ymm12, %ymm3
 vpaddw %ymm5, %ymm10, %ymm10
 vpsubw %ymm6, %ymm8, %ymm14
 vpmullw %ymm2, %ymm13, %ymm5
 vpaddw %ymm8, %ymm6, %ymm6
 vpsubw %ymm4, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm8
 vpaddw %ymm11, %ymm4, %ymm4
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm7, %ymm12, %ymm12
 vpmulhw %ymm7, %ymm13, %ymm13
 vpmulhw %ymm7, %ymm14, %ymm14
 vpmulhw %ymm7, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm3, %ymm3
 vpmulhw %ymm0, %ymm5, %ymm5
 vpmulhw %ymm0, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm3, %ymm12, %ymm3
 vpsubw %ymm5, %ymm13, %ymm5
 vpsubw %ymm8, %ymm14, %ymm8
 vpsubw %ymm11, %ymm15, %ymm11
 vpmulhw %ymm1, %ymm9, %ymm12
 vpsraw $0xa, %ymm12, %ymm12
 vpmullw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm9, %ymm9
 vperm2i128 $0x20, %ymm10, %ymm9, %ymm7
 vperm2i128 $0x31, %ymm10, %ymm9, %ymm10
 vperm2i128 $0x20, %ymm4, %ymm6, %ymm9
 vperm2i128 $0x31, %ymm4, %ymm6, %ymm4
 vperm2i128 $0x20, %ymm5, %ymm3, %ymm6
 vperm2i128 $0x31, %ymm5, %ymm3, %ymm5
 vperm2i128 $0x20, %ymm11, %ymm8, %ymm3
 vperm2i128 $0x31, %ymm11, %ymm8, %ymm11
 vmovdqa 544(%rsi), %ymm2
 vmovdqa 576(%rsi), %ymm8
 vpsubw %ymm7, %ymm10, %ymm12
 vpaddw %ymm10, %ymm7, %ymm7
 vpsubw %ymm9, %ymm4, %ymm13
 vpmullw %ymm2, %ymm12, %ymm10
 vpaddw %ymm4, %ymm9, %ymm9
 vpsubw %ymm6, %ymm5, %ymm14
 vpmullw %ymm2, %ymm13, %ymm4
 vpaddw %ymm5, %ymm6, %ymm6
 vpsubw %ymm3, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm5
 vpaddw %ymm11, %ymm3, %ymm3
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm8, %ymm12, %ymm12
 vpmulhw %ymm8, %ymm13, %ymm13
 vpmulhw %ymm8, %ymm14, %ymm14
 vpmulhw %ymm8, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm4, %ymm4
 vpmulhw %ymm0, %ymm5, %ymm5
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm10, %ymm12, %ymm10
 vpsubw %ymm4, %ymm13, %ymm4
 vpsubw %ymm5, %ymm14, %ymm5
 vpsubw %ymm11, %ymm15, %ymm11
 vpmulhw %ymm1, %ymm7, %ymm12
 vpsraw $0xa, %ymm12, %ymm12
 vpmullw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm7, %ymm7
 vmovdqa %ymm7, (%rdi)
 vmovdqa %ymm9, 32(%rdi)
 vmovdqa %ymm6, 64(%rdi)
 vmovdqa %ymm3, 96(%rdi)
 vmovdqa %ymm10, 128(%rdi)
 vmovdqa %ymm4, 160(%rdi)
 vmovdqa %ymm5, 192(%rdi)
 vmovdqa %ymm11, 224(%rdi)
 mov $0xd8a1d8a1, %eax
 vmovd %eax, %xmm2
 vpbroadcastd %xmm2, %ymm2
 mov $0x5a105a1, %eax
 vmovd %eax, %xmm3
 vpbroadcastd %xmm3, %ymm3
 vmovdqa 256(%rdi), %ymm4
 vmovdqa 320(%rdi), %ymm6
 vmovdqa 288(%rdi), %ymm5
 vmovdqa 352(%rdi), %ymm7
 vpmullw %ymm2, %ymm4, %ymm12
 vpmulhw %ymm3, %ymm4, %ymm4
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm4, %ymm4
 vpmullw %ymm2, %ymm6, %ymm12
 vpmulhw %ymm3, %ymm6, %ymm6
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm6, %ymm6
 vpmullw %ymm2, %ymm5, %ymm12
 vpmulhw %ymm3, %ymm5, %ymm5
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm5, %ymm5
 vpmullw %ymm2, %ymm7, %ymm12
 vpmulhw %ymm3, %ymm7, %ymm7
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm7, %ymm7
 vmovdqa 384(%rdi), %ymm8
 vmovdqa 448(%rdi), %ymm10
 vmovdqa 416(%rdi), %ymm9
 vmovdqa 480(%rdi), %ymm11
 vpmullw %ymm2, %ymm8, %ymm12
 vpmulhw %ymm3, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm8, %ymm8
 vpmullw %ymm2, %ymm10, %ymm12
 vpmulhw %ymm3, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm10, %ymm10
 vpmullw %ymm2, %ymm9, %ymm12
 vpmulhw %ymm3, %ymm9, %ymm9
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm9, %ymm9
 vpmullw %ymm2, %ymm11, %ymm12
 vpmulhw %ymm3, %ymm11, %ymm11
 vpmulhw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm11, %ymm11
 vpermq $0x4e, 480(%rsi), %ymm15
 vpermq $0x4e, 416(%rsi), %ymm1
 vpermq $0x4e, 512(%rsi), %ymm2
 vpermq $0x4e, 448(%rsi), %ymm3
 vmovdqa (%rsi), %ymm12
 vpshufb %ymm12, %ymm15, %ymm15
 vpshufb %ymm12, %ymm1, %ymm1
 vpshufb %ymm12, %ymm2, %ymm2
 vpshufb %ymm12, %ymm3, %ymm3
 vpsubw %ymm4, %ymm6, %ymm12
 vpaddw %ymm6, %ymm4, %ymm4
 vpsubw %ymm5, %ymm7, %ymm13
 vpmullw %ymm15, %ymm12, %ymm6
 vpaddw %ymm7, %ymm5, %ymm5
 vpsubw %ymm8, %ymm10, %ymm14
 vpmullw %ymm15, %ymm13, %ymm7
 vpaddw %ymm10, %ymm8, %ymm8
 vpsubw %ymm9, %ymm11, %ymm15
 vpmullw %ymm1, %ymm14, %ymm10
 vpaddw %ymm11, %ymm9, %ymm9
 vpmullw %ymm1, %ymm15, %ymm11
 vpmulhw %ymm2, %ymm12, %ymm12
 vpmulhw %ymm2, %ymm13, %ymm13
 vpmulhw %ymm3, %ymm14, %ymm14
 vpmulhw %ymm3, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm6, %ymm6
 vpmulhw %ymm0, %ymm7, %ymm7
 vpmulhw %ymm0, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm6, %ymm12, %ymm6
 vpsubw %ymm7, %ymm13, %ymm7
 vpsubw %ymm10, %ymm14, %ymm10
 vpsubw %ymm11, %ymm15, %ymm11
 vpermq $0x4e, 352(%rsi), %ymm2
 vpermq $0x4e, 384(%rsi), %ymm3
 vmovdqa (%rsi), %ymm1
 vpshufb %ymm1, %ymm2, %ymm2
 vpshufb %ymm1, %ymm3, %ymm3
 vpsubw %ymm4, %ymm8, %ymm12
 vpaddw %ymm8, %ymm4, %ymm4
 vpsubw %ymm5, %ymm9, %ymm13
 vpmullw %ymm2, %ymm12, %ymm8
 vpaddw %ymm9, %ymm5, %ymm5
 vpsubw %ymm6, %ymm10, %ymm14
 vpmullw %ymm2, %ymm13, %ymm9
 vpaddw %ymm10, %ymm6, %ymm6
 vpsubw %ymm7, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm10
 vpaddw %ymm11, %ymm7, %ymm7
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm3, %ymm12, %ymm12
 vpmulhw %ymm3, %ymm13, %ymm13
 vpmulhw %ymm3, %ymm14, %ymm14
 vpmulhw %ymm3, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm9, %ymm9
 vpmulhw %ymm0, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm8, %ymm12, %ymm8
 vpsubw %ymm9, %ymm13, %ymm9
 vpsubw %ymm10, %ymm14, %ymm10
 vpsubw %ymm11, %ymm15, %ymm11
 vpslld $0x10, %ymm5, %ymm3
 vpblendw $0xaa, %ymm3, %ymm4, %ymm3
 vpsrld $0x10, %ymm4, %ymm4
 vpblendw $0xaa, %ymm5, %ymm4, %ymm5
 vpslld $0x10, %ymm7, %ymm4
 vpblendw $0xaa, %ymm4, %ymm6, %ymm4
 vpsrld $0x10, %ymm6, %ymm6
 vpblendw $0xaa, %ymm7, %ymm6, %ymm7
 vpslld $0x10, %ymm9, %ymm6
 vpblendw $0xaa, %ymm6, %ymm8, %ymm6
 vpsrld $0x10, %ymm8, %ymm8
 vpblendw $0xaa, %ymm9, %ymm8, %ymm9
 vpslld $0x10, %ymm11, %ymm8
 vpblendw $0xaa, %ymm8, %ymm10, %ymm8
 vpsrld $0x10, %ymm10, %ymm10
 vpblendw $0xaa, %ymm11, %ymm10, %ymm11
 vmovdqa 32(%rsi), %ymm12
 vpermd 288(%rsi), %ymm12, %ymm2
 vpermd 320(%rsi), %ymm12, %ymm10
 vpsubw %ymm3, %ymm5, %ymm12
 vpaddw %ymm5, %ymm3, %ymm3
 vpsubw %ymm4, %ymm7, %ymm13
 vpmullw %ymm2, %ymm12, %ymm5
 vpaddw %ymm7, %ymm4, %ymm4
 vpsubw %ymm6, %ymm9, %ymm14
 vpmullw %ymm2, %ymm13, %ymm7
 vpaddw %ymm9, %ymm6, %ymm6
 vpsubw %ymm8, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm9
 vpaddw %ymm11, %ymm8, %ymm8
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm10, %ymm12, %ymm12
 vpmulhw %ymm10, %ymm13, %ymm13
 vpmulhw %ymm10, %ymm14, %ymm14
 vpmulhw %ymm10, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm5, %ymm5
 vpmulhw %ymm0, %ymm7, %ymm7
 vpmulhw %ymm0, %ymm9, %ymm9
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm5, %ymm12, %ymm5
 vpsubw %ymm7, %ymm13, %ymm7
 vpsubw %ymm9, %ymm14, %ymm9
 vpsubw %ymm11, %ymm15, %ymm11
 mov $0x4ebf4ebf, %eax
 vmovd %eax, %xmm1
 vpbroadcastd %xmm1, %ymm1
 vpmulhw %ymm1, %ymm3, %ymm12
 vpsraw $0xa, %ymm12, %ymm12
 vpmullw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm3, %ymm3
 vmovsldup %ymm4, %ymm10
 vpblendd $0xaa, %ymm10, %ymm3, %ymm10
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm4, %ymm3, %ymm4
 vmovsldup %ymm8, %ymm3
 vpblendd $0xaa, %ymm3, %ymm6, %ymm3
 vpsrlq $0x20, %ymm6, %ymm6
 vpblendd $0xaa, %ymm8, %ymm6, %ymm8
 vmovsldup %ymm7, %ymm6
 vpblendd $0xaa, %ymm6, %ymm5, %ymm6
 vpsrlq $0x20, %ymm5, %ymm5
 vpblendd $0xaa, %ymm7, %ymm5, %ymm7
 vmovsldup %ymm11, %ymm5
 vpblendd $0xaa, %ymm5, %ymm9, %ymm5
 vpsrlq $0x20, %ymm9, %ymm9
 vpblendd $0xaa, %ymm11, %ymm9, %ymm11
 vpermq $0x1b, 224(%rsi), %ymm2
 vpermq $0x1b, 256(%rsi), %ymm9
 vpsubw %ymm10, %ymm4, %ymm12
 vpaddw %ymm4, %ymm10, %ymm10
 vpsubw %ymm3, %ymm8, %ymm13
 vpmullw %ymm2, %ymm12, %ymm4
 vpaddw %ymm8, %ymm3, %ymm3
 vpsubw %ymm6, %ymm7, %ymm14
 vpmullw %ymm2, %ymm13, %ymm8
 vpaddw %ymm7, %ymm6, %ymm6
 vpsubw %ymm5, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm7
 vpaddw %ymm11, %ymm5, %ymm5
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm9, %ymm12, %ymm12
 vpmulhw %ymm9, %ymm13, %ymm13
 vpmulhw %ymm9, %ymm14, %ymm14
 vpmulhw %ymm9, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm4, %ymm4
 vpmulhw %ymm0, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm7, %ymm7
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm4, %ymm12, %ymm4
 vpsubw %ymm8, %ymm13, %ymm8
 vpsubw %ymm7, %ymm14, %ymm7
 vpsubw %ymm11, %ymm15, %ymm11
 vpmulhw %ymm1, %ymm10, %ymm12
 vpsraw $0xa, %ymm12, %ymm12
 vpmullw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm10, %ymm10
 vpunpcklqdq %ymm3, %ymm10, %ymm9
 vpunpckhqdq %ymm3, %ymm10, %ymm3
 vpunpcklqdq %ymm5, %ymm6, %ymm10
 vpunpckhqdq %ymm5, %ymm6, %ymm5
 vpunpcklqdq %ymm8, %ymm4, %ymm6
 vpunpckhqdq %ymm8, %ymm4, %ymm8
 vpunpcklqdq %ymm11, %ymm7, %ymm4
 vpunpckhqdq %ymm11, %ymm7, %ymm11
 vpermq $0x4e, 160(%rsi), %ymm2
 vpermq $0x4e, 192(%rsi), %ymm7
 vpsubw %ymm9, %ymm3, %ymm12
 vpaddw %ymm3, %ymm9, %ymm9
 vpsubw %ymm10, %ymm5, %ymm13
 vpmullw %ymm2, %ymm12, %ymm3
 vpaddw %ymm5, %ymm10, %ymm10
 vpsubw %ymm6, %ymm8, %ymm14
 vpmullw %ymm2, %ymm13, %ymm5
 vpaddw %ymm8, %ymm6, %ymm6
 vpsubw %ymm4, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm8
 vpaddw %ymm11, %ymm4, %ymm4
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm7, %ymm12, %ymm12
 vpmulhw %ymm7, %ymm13, %ymm13
 vpmulhw %ymm7, %ymm14, %ymm14
 vpmulhw %ymm7, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm3, %ymm3
 vpmulhw %ymm0, %ymm5, %ymm5
 vpmulhw %ymm0, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm3, %ymm12, %ymm3
 vpsubw %ymm5, %ymm13, %ymm5
 vpsubw %ymm8, %ymm14, %ymm8
 vpsubw %ymm11, %ymm15, %ymm11
 vpmulhw %ymm1, %ymm9, %ymm12
 vpsraw $0xa, %ymm12, %ymm12
 vpmullw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm9, %ymm9
 vperm2i128 $0x20, %ymm10, %ymm9, %ymm7
 vperm2i128 $0x31, %ymm10, %ymm9, %ymm10
 vperm2i128 $0x20, %ymm4, %ymm6, %ymm9
 vperm2i128 $0x31, %ymm4, %ymm6, %ymm4
 vperm2i128 $0x20, %ymm5, %ymm3, %ymm6
 vperm2i128 $0x31, %ymm5, %ymm3, %ymm5
 vperm2i128 $0x20, %ymm11, %ymm8, %ymm3
 vperm2i128 $0x31, %ymm11, %ymm8, %ymm11
 vmovdqa 96(%rsi), %ymm2
 vmovdqa 128(%rsi), %ymm8
 vpsubw %ymm7, %ymm10, %ymm12
 vpaddw %ymm10, %ymm7, %ymm7
 vpsubw %ymm9, %ymm4, %ymm13
 vpmullw %ymm2, %ymm12, %ymm10
 vpaddw %ymm4, %ymm9, %ymm9
 vpsubw %ymm6, %ymm5, %ymm14
 vpmullw %ymm2, %ymm13, %ymm4
 vpaddw %ymm5, %ymm6, %ymm6
 vpsubw %ymm3, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm5
 vpaddw %ymm11, %ymm3, %ymm3
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm8, %ymm12, %ymm12
 vpmulhw %ymm8, %ymm13, %ymm13
 vpmulhw %ymm8, %ymm14, %ymm14
 vpmulhw %ymm8, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm4, %ymm4
 vpmulhw %ymm0, %ymm5, %ymm5
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm10, %ymm12, %ymm10
 vpsubw %ymm4, %ymm13, %ymm4
 vpsubw %ymm5, %ymm14, %ymm5
 vpsubw %ymm11, %ymm15, %ymm11
 vpmulhw %ymm1, %ymm7, %ymm12
 vpsraw $0xa, %ymm12, %ymm12
 vpmullw %ymm0, %ymm12, %ymm12
 vpsubw %ymm12, %ymm7, %ymm7
 vmovdqa %ymm7, 256(%rdi)
 vmovdqa %ymm9, 288(%rdi)
 vmovdqa %ymm6, 320(%rdi)
 vmovdqa %ymm3, 352(%rdi)
 vmovdqa %ymm10, 384(%rdi)
 vmovdqa %ymm4, 416(%rdi)
 vmovdqa %ymm5, 448(%rdi)
 vmovdqa %ymm11, 480(%rdi)
 vmovdqa (%rdi), %ymm4
 vmovdqa 256(%rdi), %ymm8
 vmovdqa 32(%rdi), %ymm5
 vmovdqa 288(%rdi), %ymm9
 vpbroadcastq 64(%rsi), %ymm2
 vmovdqa 64(%rdi), %ymm6
 vmovdqa 320(%rdi), %ymm10
 vmovdqa 96(%rdi), %ymm7
 vmovdqa 352(%rdi), %ymm11
 vpbroadcastq 72(%rsi), %ymm3
 vpsubw %ymm4, %ymm8, %ymm12
 vpaddw %ymm8, %ymm4, %ymm4
 vpsubw %ymm5, %ymm9, %ymm13
 vpmullw %ymm2, %ymm12, %ymm8
 vpaddw %ymm9, %ymm5, %ymm5
 vpsubw %ymm6, %ymm10, %ymm14
 vpmullw %ymm2, %ymm13, %ymm9
 vpaddw %ymm10, %ymm6, %ymm6
 vpsubw %ymm7, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm10
 vpaddw %ymm11, %ymm7, %ymm7
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm3, %ymm12, %ymm12
 vpmulhw %ymm3, %ymm13, %ymm13
 vpmulhw %ymm3, %ymm14, %ymm14
 vpmulhw %ymm3, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm9, %ymm9
 vpmulhw %ymm0, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm8, %ymm12, %ymm8
 vpsubw %ymm9, %ymm13, %ymm9
 vpsubw %ymm10, %ymm14, %ymm10
 vpsubw %ymm11, %ymm15, %ymm11
 vmovdqa %ymm4, (%rdi)
 vmovdqa %ymm5, 32(%rdi)
 vmovdqa %ymm6, 64(%rdi)
 vmovdqa %ymm7, 96(%rdi)
 vmovdqa %ymm8, 256(%rdi)
 vmovdqa %ymm9, 288(%rdi)
 vmovdqa %ymm10, 320(%rdi)
 vmovdqa %ymm11, 352(%rdi)
 vmovdqa 128(%rdi), %ymm4
 vmovdqa 384(%rdi), %ymm8
 vmovdqa 160(%rdi), %ymm5
 vmovdqa 416(%rdi), %ymm9
 vpbroadcastq 64(%rsi), %ymm2
 vmovdqa 192(%rdi), %ymm6
 vmovdqa 448(%rdi), %ymm10
 vmovdqa 224(%rdi), %ymm7
 vmovdqa 480(%rdi), %ymm11
 vpbroadcastq 72(%rsi), %ymm3
 vpsubw %ymm4, %ymm8, %ymm12
 vpaddw %ymm8, %ymm4, %ymm4
 vpsubw %ymm5, %ymm9, %ymm13
 vpmullw %ymm2, %ymm12, %ymm8
 vpaddw %ymm9, %ymm5, %ymm5
 vpsubw %ymm6, %ymm10, %ymm14
 vpmullw %ymm2, %ymm13, %ymm9
 vpaddw %ymm10, %ymm6, %ymm6
 vpsubw %ymm7, %ymm11, %ymm15
 vpmullw %ymm2, %ymm14, %ymm10
 vpaddw %ymm11, %ymm7, %ymm7
 vpmullw %ymm2, %ymm15, %ymm11
 vpmulhw %ymm3, %ymm12, %ymm12
 vpmulhw %ymm3, %ymm13, %ymm13
 vpmulhw %ymm3, %ymm14, %ymm14
 vpmulhw %ymm3, %ymm15, %ymm15
 vpmulhw %ymm0, %ymm8, %ymm8
 vpmulhw %ymm0, %ymm9, %ymm9
 vpmulhw %ymm0, %ymm10, %ymm10
 vpmulhw %ymm0, %ymm11, %ymm11
 vpsubw %ymm8, %ymm12, %ymm8
 vpsubw %ymm9, %ymm13, %ymm9
 vpsubw %ymm10, %ymm14, %ymm10
 vpsubw %ymm11, %ymm15, %ymm11
 vmovdqa %ymm4, 128(%rdi)
 vmovdqa %ymm5, 160(%rdi)
 vmovdqa %ymm6, 192(%rdi)
 vmovdqa %ymm7, 224(%rdi)
 vmovdqa %ymm8, 384(%rdi)
 vmovdqa %ymm9, 416(%rdi)
 vmovdqa %ymm10, 448(%rdi)
 vmovdqa %ymm11, 480(%rdi)

#if WINDOWS_ABI
        CFI_STACKLOADU(%xmm6,0)
        CFI_STACKLOADU(%xmm7,16)
        CFI_STACKLOADU(%xmm8,32)
        CFI_STACKLOADU(%xmm9,48)
        CFI_STACKLOADU(%xmm10,64)
        CFI_STACKLOADU(%xmm11,80)
        CFI_STACKLOADU(%xmm12,96)
        CFI_STACKLOADU(%xmm13,112)
        CFI_STACKLOADU(%xmm14,128)
        CFI_STACKLOADU(%xmm15,144)
        CFI_STACKLOAD(%rdi,160)
        CFI_STACKLOAD(%rsi,168)
        CFI_INC_RSP(176)
#endif

 CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_intt_x86)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
