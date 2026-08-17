// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Forward number-theoretic transform from ML-KEM
// Input a[256], qdata[624] (all signed 16-bit words); output a[256] (signed 16-bit words).
//
// The transform is in-place with input and output a[256], with the output
// in bitreversed order. The second parameter is expected to point to a
// table of constants whose definitions can be found in the mlkem-native
// repo or our "tests/test.c".
//
// extern void mlkem_ntt_x86(int16_t a[static 256], const int16_t qdata[static 624]);
//
// Standard x86-64 ABI: RDI = a, RSI = qdata
// Microsoft x64 ABI:   RCX = a, RDX = qdata
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_ntt_x86)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_ntt_x86)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_ntt_x86)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_ntt_x86):
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
 vpbroadcastq 64(%rsi), %ymm15
 vmovdqa 256(%rdi), %ymm8
 vmovdqa 288(%rdi), %ymm9
 vmovdqa 320(%rdi), %ymm10
 vmovdqa 352(%rdi), %ymm11
 vpbroadcastq 72(%rsi), %ymm2
 vpmullw %ymm15, %ymm8, %ymm12
 vpmullw %ymm15, %ymm9, %ymm13
 vpmullw %ymm15, %ymm10, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm8, %ymm8
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm10, %ymm10
 vpmulhw %ymm2, %ymm11, %ymm11
 vmovdqa (%rdi), %ymm4
 vmovdqa 32(%rdi), %ymm5
 vmovdqa 64(%rdi), %ymm6
 vmovdqa 96(%rdi), %ymm7
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm8, %ymm4, %ymm3
 vpsubw %ymm8, %ymm4, %ymm8
 vpaddw %ymm9, %ymm5, %ymm4
 vpsubw %ymm9, %ymm5, %ymm9
 vpaddw %ymm10, %ymm6, %ymm5
 vpsubw %ymm10, %ymm6, %ymm10
 vpaddw %ymm11, %ymm7, %ymm6
 vpsubw %ymm11, %ymm7, %ymm11
 vpsubw %ymm12, %ymm3, %ymm3
 vpaddw %ymm12, %ymm8, %ymm8
 vpsubw %ymm13, %ymm4, %ymm4
 vpaddw %ymm13, %ymm9, %ymm9
 vpsubw %ymm14, %ymm5, %ymm5
 vpaddw %ymm14, %ymm10, %ymm10
 vpsubw %ymm15, %ymm6, %ymm6
 vpaddw %ymm15, %ymm11, %ymm11
 vmovdqa %ymm3, (%rdi)
 vmovdqa %ymm4, 32(%rdi)
 vmovdqa %ymm5, 64(%rdi)
 vmovdqa %ymm6, 96(%rdi)
 vmovdqa %ymm8, 256(%rdi)
 vmovdqa %ymm9, 288(%rdi)
 vmovdqa %ymm10, 320(%rdi)
 vmovdqa %ymm11, 352(%rdi)
 vpbroadcastq 64(%rsi), %ymm15
 vmovdqa 384(%rdi), %ymm8
 vmovdqa 416(%rdi), %ymm9
 vmovdqa 448(%rdi), %ymm10
 vmovdqa 480(%rdi), %ymm11
 vpbroadcastq 72(%rsi), %ymm2
 vpmullw %ymm15, %ymm8, %ymm12
 vpmullw %ymm15, %ymm9, %ymm13
 vpmullw %ymm15, %ymm10, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm8, %ymm8
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm10, %ymm10
 vpmulhw %ymm2, %ymm11, %ymm11
 vmovdqa 128(%rdi), %ymm4
 vmovdqa 160(%rdi), %ymm5
 vmovdqa 192(%rdi), %ymm6
 vmovdqa 224(%rdi), %ymm7
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm8, %ymm4, %ymm3
 vpsubw %ymm8, %ymm4, %ymm8
 vpaddw %ymm9, %ymm5, %ymm4
 vpsubw %ymm9, %ymm5, %ymm9
 vpaddw %ymm10, %ymm6, %ymm5
 vpsubw %ymm10, %ymm6, %ymm10
 vpaddw %ymm11, %ymm7, %ymm6
 vpsubw %ymm11, %ymm7, %ymm11
 vpsubw %ymm12, %ymm3, %ymm3
 vpaddw %ymm12, %ymm8, %ymm8
 vpsubw %ymm13, %ymm4, %ymm4
 vpaddw %ymm13, %ymm9, %ymm9
 vpsubw %ymm14, %ymm5, %ymm5
 vpaddw %ymm14, %ymm10, %ymm10
 vpsubw %ymm15, %ymm6, %ymm6
 vpaddw %ymm15, %ymm11, %ymm11
 vmovdqa %ymm3, 128(%rdi)
 vmovdqa %ymm4, 160(%rdi)
 vmovdqa %ymm5, 192(%rdi)
 vmovdqa %ymm6, 224(%rdi)
 vmovdqa %ymm8, 384(%rdi)
 vmovdqa %ymm9, 416(%rdi)
 vmovdqa %ymm10, 448(%rdi)
 vmovdqa %ymm11, 480(%rdi)
 vmovdqa 96(%rsi), %ymm15
 vmovdqa 128(%rdi), %ymm8
 vmovdqa 160(%rdi), %ymm9
 vmovdqa 192(%rdi), %ymm10
 vmovdqa 224(%rdi), %ymm11
 vmovdqa 128(%rsi), %ymm2
 vpmullw %ymm15, %ymm8, %ymm12
 vpmullw %ymm15, %ymm9, %ymm13
 vpmullw %ymm15, %ymm10, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm8, %ymm8
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm10, %ymm10
 vpmulhw %ymm2, %ymm11, %ymm11
 vmovdqa (%rdi), %ymm4
 vmovdqa 32(%rdi), %ymm5
 vmovdqa 64(%rdi), %ymm6
 vmovdqa 96(%rdi), %ymm7
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm8, %ymm4, %ymm3
 vpsubw %ymm8, %ymm4, %ymm8
 vpaddw %ymm9, %ymm5, %ymm4
 vpsubw %ymm9, %ymm5, %ymm9
 vpaddw %ymm10, %ymm6, %ymm5
 vpsubw %ymm10, %ymm6, %ymm10
 vpaddw %ymm11, %ymm7, %ymm6
 vpsubw %ymm11, %ymm7, %ymm11
 vpsubw %ymm12, %ymm3, %ymm3
 vpaddw %ymm12, %ymm8, %ymm8
 vpsubw %ymm13, %ymm4, %ymm4
 vpaddw %ymm13, %ymm9, %ymm9
 vpsubw %ymm14, %ymm5, %ymm5
 vpaddw %ymm14, %ymm10, %ymm10
 vpsubw %ymm15, %ymm6, %ymm6
 vpaddw %ymm15, %ymm11, %ymm11
 vperm2i128 $0x20, %ymm10, %ymm5, %ymm7
 vperm2i128 $0x31, %ymm10, %ymm5, %ymm10
 vperm2i128 $0x20, %ymm11, %ymm6, %ymm5
 vperm2i128 $0x31, %ymm11, %ymm6, %ymm11
 vmovdqa 160(%rsi), %ymm15
 vmovdqa 192(%rsi), %ymm2
 vpmullw %ymm15, %ymm7, %ymm12
 vpmullw %ymm15, %ymm10, %ymm13
 vpmullw %ymm15, %ymm5, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm7, %ymm7
 vpmulhw %ymm2, %ymm10, %ymm10
 vpmulhw %ymm2, %ymm5, %ymm5
 vpmulhw %ymm2, %ymm11, %ymm11
 vperm2i128 $0x20, %ymm8, %ymm3, %ymm6
 vperm2i128 $0x31, %ymm8, %ymm3, %ymm8
 vperm2i128 $0x20, %ymm9, %ymm4, %ymm3
 vperm2i128 $0x31, %ymm9, %ymm4, %ymm9
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm7, %ymm6, %ymm4
 vpsubw %ymm7, %ymm6, %ymm7
 vpaddw %ymm10, %ymm8, %ymm6
 vpsubw %ymm10, %ymm8, %ymm10
 vpaddw %ymm5, %ymm3, %ymm8
 vpsubw %ymm5, %ymm3, %ymm5
 vpaddw %ymm11, %ymm9, %ymm3
 vpsubw %ymm11, %ymm9, %ymm11
 vpsubw %ymm12, %ymm4, %ymm4
 vpaddw %ymm12, %ymm7, %ymm7
 vpsubw %ymm13, %ymm6, %ymm6
 vpaddw %ymm13, %ymm10, %ymm10
 vpsubw %ymm14, %ymm8, %ymm8
 vpaddw %ymm14, %ymm5, %ymm5
 vpsubw %ymm15, %ymm3, %ymm3
 vpaddw %ymm15, %ymm11, %ymm11
 vpunpcklqdq %ymm5, %ymm8, %ymm9
 vpunpckhqdq %ymm5, %ymm8, %ymm5
 vpunpcklqdq %ymm11, %ymm3, %ymm8
 vpunpckhqdq %ymm11, %ymm3, %ymm11
 vmovdqa 224(%rsi), %ymm15
 vmovdqa 256(%rsi), %ymm2
 vpmullw %ymm15, %ymm9, %ymm12
 vpmullw %ymm15, %ymm5, %ymm13
 vpmullw %ymm15, %ymm8, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm5, %ymm5
 vpmulhw %ymm2, %ymm8, %ymm8
 vpmulhw %ymm2, %ymm11, %ymm11
 vpunpcklqdq %ymm7, %ymm4, %ymm3
 vpunpckhqdq %ymm7, %ymm4, %ymm7
 vpunpcklqdq %ymm10, %ymm6, %ymm4
 vpunpckhqdq %ymm10, %ymm6, %ymm10
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm9, %ymm3, %ymm6
 vpsubw %ymm9, %ymm3, %ymm9
 vpaddw %ymm5, %ymm7, %ymm3
 vpsubw %ymm5, %ymm7, %ymm5
 vpaddw %ymm8, %ymm4, %ymm7
 vpsubw %ymm8, %ymm4, %ymm8
 vpaddw %ymm11, %ymm10, %ymm4
 vpsubw %ymm11, %ymm10, %ymm11
 vpsubw %ymm12, %ymm6, %ymm6
 vpaddw %ymm12, %ymm9, %ymm9
 vpsubw %ymm13, %ymm3, %ymm3
 vpaddw %ymm13, %ymm5, %ymm5
 vpsubw %ymm14, %ymm7, %ymm7
 vpaddw %ymm14, %ymm8, %ymm8
 vpsubw %ymm15, %ymm4, %ymm4
 vpaddw %ymm15, %ymm11, %ymm11
 vmovsldup %ymm8, %ymm10
 vpblendd $0xaa, %ymm10, %ymm7, %ymm10
 vpsrlq $0x20, %ymm7, %ymm7
 vpblendd $0xaa, %ymm8, %ymm7, %ymm8
 vmovsldup %ymm11, %ymm7
 vpblendd $0xaa, %ymm7, %ymm4, %ymm7
 vpsrlq $0x20, %ymm4, %ymm4
 vpblendd $0xaa, %ymm11, %ymm4, %ymm11
 vmovdqa 288(%rsi), %ymm15
 vmovdqa 320(%rsi), %ymm2
 vpmullw %ymm15, %ymm10, %ymm12
 vpmullw %ymm15, %ymm8, %ymm13
 vpmullw %ymm15, %ymm7, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm10, %ymm10
 vpmulhw %ymm2, %ymm8, %ymm8
 vpmulhw %ymm2, %ymm7, %ymm7
 vpmulhw %ymm2, %ymm11, %ymm11
 vmovsldup %ymm9, %ymm4
 vpblendd $0xaa, %ymm4, %ymm6, %ymm4
 vpsrlq $0x20, %ymm6, %ymm6
 vpblendd $0xaa, %ymm9, %ymm6, %ymm9
 vmovsldup %ymm5, %ymm6
 vpblendd $0xaa, %ymm6, %ymm3, %ymm6
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm5, %ymm3, %ymm5
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm10, %ymm4, %ymm3
 vpsubw %ymm10, %ymm4, %ymm10
 vpaddw %ymm8, %ymm9, %ymm4
 vpsubw %ymm8, %ymm9, %ymm8
 vpaddw %ymm7, %ymm6, %ymm9
 vpsubw %ymm7, %ymm6, %ymm7
 vpaddw %ymm11, %ymm5, %ymm6
 vpsubw %ymm11, %ymm5, %ymm11
 vpsubw %ymm12, %ymm3, %ymm3
 vpaddw %ymm12, %ymm10, %ymm10
 vpsubw %ymm13, %ymm4, %ymm4
 vpaddw %ymm13, %ymm8, %ymm8
 vpsubw %ymm14, %ymm9, %ymm9
 vpaddw %ymm14, %ymm7, %ymm7
 vpsubw %ymm15, %ymm6, %ymm6
 vpaddw %ymm15, %ymm11, %ymm11
 vpslld $0x10, %ymm7, %ymm5
 vpblendw $0xaa, %ymm5, %ymm9, %ymm5
 vpsrld $0x10, %ymm9, %ymm9
 vpblendw $0xaa, %ymm7, %ymm9, %ymm7
 vpslld $0x10, %ymm11, %ymm9
 vpblendw $0xaa, %ymm9, %ymm6, %ymm9
 vpsrld $0x10, %ymm6, %ymm6
 vpblendw $0xaa, %ymm11, %ymm6, %ymm11
 vmovdqa 352(%rsi), %ymm15
 vmovdqa 384(%rsi), %ymm2
 vpmullw %ymm15, %ymm5, %ymm12
 vpmullw %ymm15, %ymm7, %ymm13
 vpmullw %ymm15, %ymm9, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm5, %ymm5
 vpmulhw %ymm2, %ymm7, %ymm7
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm11, %ymm11
 vpslld $0x10, %ymm10, %ymm6
 vpblendw $0xaa, %ymm6, %ymm3, %ymm6
 vpsrld $0x10, %ymm3, %ymm3
 vpblendw $0xaa, %ymm10, %ymm3, %ymm10
 vpslld $0x10, %ymm8, %ymm3
 vpblendw $0xaa, %ymm3, %ymm4, %ymm3
 vpsrld $0x10, %ymm4, %ymm4
 vpblendw $0xaa, %ymm8, %ymm4, %ymm8
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm5, %ymm6, %ymm4
 vpsubw %ymm5, %ymm6, %ymm5
 vpaddw %ymm7, %ymm10, %ymm6
 vpsubw %ymm7, %ymm10, %ymm7
 vpaddw %ymm9, %ymm3, %ymm10
 vpsubw %ymm9, %ymm3, %ymm9
 vpaddw %ymm11, %ymm8, %ymm3
 vpsubw %ymm11, %ymm8, %ymm11
 vpsubw %ymm12, %ymm4, %ymm4
 vpaddw %ymm12, %ymm5, %ymm5
 vpsubw %ymm13, %ymm6, %ymm6
 vpaddw %ymm13, %ymm7, %ymm7
 vpsubw %ymm14, %ymm10, %ymm10
 vpaddw %ymm14, %ymm9, %ymm9
 vpsubw %ymm15, %ymm3, %ymm3
 vpaddw %ymm15, %ymm11, %ymm11
 vmovdqa 416(%rsi), %ymm14
 vmovdqa 480(%rsi), %ymm15
 vmovdqa 448(%rsi), %ymm8
 vmovdqa 512(%rsi), %ymm2
 vpmullw %ymm14, %ymm10, %ymm12
 vpmullw %ymm14, %ymm3, %ymm13
 vpmullw %ymm15, %ymm9, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm8, %ymm10, %ymm10
 vpmulhw %ymm8, %ymm3, %ymm3
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm11, %ymm11
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm10, %ymm4, %ymm8
 vpsubw %ymm10, %ymm4, %ymm10
 vpaddw %ymm3, %ymm6, %ymm4
 vpsubw %ymm3, %ymm6, %ymm3
 vpaddw %ymm9, %ymm5, %ymm6
 vpsubw %ymm9, %ymm5, %ymm9
 vpaddw %ymm11, %ymm7, %ymm5
 vpsubw %ymm11, %ymm7, %ymm11
 vpsubw %ymm12, %ymm8, %ymm8
 vpaddw %ymm12, %ymm10, %ymm10
 vpsubw %ymm13, %ymm4, %ymm4
 vpaddw %ymm13, %ymm3, %ymm3
 vpsubw %ymm14, %ymm6, %ymm6
 vpaddw %ymm14, %ymm9, %ymm9
 vpsubw %ymm15, %ymm5, %ymm5
 vpaddw %ymm15, %ymm11, %ymm11
 vmovdqa %ymm8, (%rdi)
 vmovdqa %ymm4, 32(%rdi)
 vmovdqa %ymm10, 64(%rdi)
 vmovdqa %ymm3, 96(%rdi)
 vmovdqa %ymm6, 128(%rdi)
 vmovdqa %ymm5, 160(%rdi)
 vmovdqa %ymm9, 192(%rdi)
 vmovdqa %ymm11, 224(%rdi)
 vmovdqa 544(%rsi), %ymm15
 vmovdqa 384(%rdi), %ymm8
 vmovdqa 416(%rdi), %ymm9
 vmovdqa 448(%rdi), %ymm10
 vmovdqa 480(%rdi), %ymm11
 vmovdqa 576(%rsi), %ymm2
 vpmullw %ymm15, %ymm8, %ymm12
 vpmullw %ymm15, %ymm9, %ymm13
 vpmullw %ymm15, %ymm10, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm8, %ymm8
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm10, %ymm10
 vpmulhw %ymm2, %ymm11, %ymm11
 vmovdqa 256(%rdi), %ymm4
 vmovdqa 288(%rdi), %ymm5
 vmovdqa 320(%rdi), %ymm6
 vmovdqa 352(%rdi), %ymm7
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm8, %ymm4, %ymm3
 vpsubw %ymm8, %ymm4, %ymm8
 vpaddw %ymm9, %ymm5, %ymm4
 vpsubw %ymm9, %ymm5, %ymm9
 vpaddw %ymm10, %ymm6, %ymm5
 vpsubw %ymm10, %ymm6, %ymm10
 vpaddw %ymm11, %ymm7, %ymm6
 vpsubw %ymm11, %ymm7, %ymm11
 vpsubw %ymm12, %ymm3, %ymm3
 vpaddw %ymm12, %ymm8, %ymm8
 vpsubw %ymm13, %ymm4, %ymm4
 vpaddw %ymm13, %ymm9, %ymm9
 vpsubw %ymm14, %ymm5, %ymm5
 vpaddw %ymm14, %ymm10, %ymm10
 vpsubw %ymm15, %ymm6, %ymm6
 vpaddw %ymm15, %ymm11, %ymm11
 vperm2i128 $0x20, %ymm10, %ymm5, %ymm7
 vperm2i128 $0x31, %ymm10, %ymm5, %ymm10
 vperm2i128 $0x20, %ymm11, %ymm6, %ymm5
 vperm2i128 $0x31, %ymm11, %ymm6, %ymm11
 vmovdqa 608(%rsi), %ymm15
 vmovdqa 640(%rsi), %ymm2
 vpmullw %ymm15, %ymm7, %ymm12
 vpmullw %ymm15, %ymm10, %ymm13
 vpmullw %ymm15, %ymm5, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm7, %ymm7
 vpmulhw %ymm2, %ymm10, %ymm10
 vpmulhw %ymm2, %ymm5, %ymm5
 vpmulhw %ymm2, %ymm11, %ymm11
 vperm2i128 $0x20, %ymm8, %ymm3, %ymm6
 vperm2i128 $0x31, %ymm8, %ymm3, %ymm8
 vperm2i128 $0x20, %ymm9, %ymm4, %ymm3
 vperm2i128 $0x31, %ymm9, %ymm4, %ymm9
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm7, %ymm6, %ymm4
 vpsubw %ymm7, %ymm6, %ymm7
 vpaddw %ymm10, %ymm8, %ymm6
 vpsubw %ymm10, %ymm8, %ymm10
 vpaddw %ymm5, %ymm3, %ymm8
 vpsubw %ymm5, %ymm3, %ymm5
 vpaddw %ymm11, %ymm9, %ymm3
 vpsubw %ymm11, %ymm9, %ymm11
 vpsubw %ymm12, %ymm4, %ymm4
 vpaddw %ymm12, %ymm7, %ymm7
 vpsubw %ymm13, %ymm6, %ymm6
 vpaddw %ymm13, %ymm10, %ymm10
 vpsubw %ymm14, %ymm8, %ymm8
 vpaddw %ymm14, %ymm5, %ymm5
 vpsubw %ymm15, %ymm3, %ymm3
 vpaddw %ymm15, %ymm11, %ymm11
 vpunpcklqdq %ymm5, %ymm8, %ymm9
 vpunpckhqdq %ymm5, %ymm8, %ymm5
 vpunpcklqdq %ymm11, %ymm3, %ymm8
 vpunpckhqdq %ymm11, %ymm3, %ymm11
 vmovdqa 672(%rsi), %ymm15
 vmovdqa 704(%rsi), %ymm2
 vpmullw %ymm15, %ymm9, %ymm12
 vpmullw %ymm15, %ymm5, %ymm13
 vpmullw %ymm15, %ymm8, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm5, %ymm5
 vpmulhw %ymm2, %ymm8, %ymm8
 vpmulhw %ymm2, %ymm11, %ymm11
 vpunpcklqdq %ymm7, %ymm4, %ymm3
 vpunpckhqdq %ymm7, %ymm4, %ymm7
 vpunpcklqdq %ymm10, %ymm6, %ymm4
 vpunpckhqdq %ymm10, %ymm6, %ymm10
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm9, %ymm3, %ymm6
 vpsubw %ymm9, %ymm3, %ymm9
 vpaddw %ymm5, %ymm7, %ymm3
 vpsubw %ymm5, %ymm7, %ymm5
 vpaddw %ymm8, %ymm4, %ymm7
 vpsubw %ymm8, %ymm4, %ymm8
 vpaddw %ymm11, %ymm10, %ymm4
 vpsubw %ymm11, %ymm10, %ymm11
 vpsubw %ymm12, %ymm6, %ymm6
 vpaddw %ymm12, %ymm9, %ymm9
 vpsubw %ymm13, %ymm3, %ymm3
 vpaddw %ymm13, %ymm5, %ymm5
 vpsubw %ymm14, %ymm7, %ymm7
 vpaddw %ymm14, %ymm8, %ymm8
 vpsubw %ymm15, %ymm4, %ymm4
 vpaddw %ymm15, %ymm11, %ymm11
 vmovsldup %ymm8, %ymm10
 vpblendd $0xaa, %ymm10, %ymm7, %ymm10
 vpsrlq $0x20, %ymm7, %ymm7
 vpblendd $0xaa, %ymm8, %ymm7, %ymm8
 vmovsldup %ymm11, %ymm7
 vpblendd $0xaa, %ymm7, %ymm4, %ymm7
 vpsrlq $0x20, %ymm4, %ymm4
 vpblendd $0xaa, %ymm11, %ymm4, %ymm11
 vmovdqa 736(%rsi), %ymm15
 vmovdqa 768(%rsi), %ymm2
 vpmullw %ymm15, %ymm10, %ymm12
 vpmullw %ymm15, %ymm8, %ymm13
 vpmullw %ymm15, %ymm7, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm10, %ymm10
 vpmulhw %ymm2, %ymm8, %ymm8
 vpmulhw %ymm2, %ymm7, %ymm7
 vpmulhw %ymm2, %ymm11, %ymm11
 vmovsldup %ymm9, %ymm4
 vpblendd $0xaa, %ymm4, %ymm6, %ymm4
 vpsrlq $0x20, %ymm6, %ymm6
 vpblendd $0xaa, %ymm9, %ymm6, %ymm9
 vmovsldup %ymm5, %ymm6
 vpblendd $0xaa, %ymm6, %ymm3, %ymm6
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm5, %ymm3, %ymm5
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm10, %ymm4, %ymm3
 vpsubw %ymm10, %ymm4, %ymm10
 vpaddw %ymm8, %ymm9, %ymm4
 vpsubw %ymm8, %ymm9, %ymm8
 vpaddw %ymm7, %ymm6, %ymm9
 vpsubw %ymm7, %ymm6, %ymm7
 vpaddw %ymm11, %ymm5, %ymm6
 vpsubw %ymm11, %ymm5, %ymm11
 vpsubw %ymm12, %ymm3, %ymm3
 vpaddw %ymm12, %ymm10, %ymm10
 vpsubw %ymm13, %ymm4, %ymm4
 vpaddw %ymm13, %ymm8, %ymm8
 vpsubw %ymm14, %ymm9, %ymm9
 vpaddw %ymm14, %ymm7, %ymm7
 vpsubw %ymm15, %ymm6, %ymm6
 vpaddw %ymm15, %ymm11, %ymm11
 vpslld $0x10, %ymm7, %ymm5
 vpblendw $0xaa, %ymm5, %ymm9, %ymm5
 vpsrld $0x10, %ymm9, %ymm9
 vpblendw $0xaa, %ymm7, %ymm9, %ymm7
 vpslld $0x10, %ymm11, %ymm9
 vpblendw $0xaa, %ymm9, %ymm6, %ymm9
 vpsrld $0x10, %ymm6, %ymm6
 vpblendw $0xaa, %ymm11, %ymm6, %ymm11
 vmovdqa 800(%rsi), %ymm15
 vmovdqa 832(%rsi), %ymm2
 vpmullw %ymm15, %ymm5, %ymm12
 vpmullw %ymm15, %ymm7, %ymm13
 vpmullw %ymm15, %ymm9, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm2, %ymm5, %ymm5
 vpmulhw %ymm2, %ymm7, %ymm7
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm11, %ymm11
 vpslld $0x10, %ymm10, %ymm6
 vpblendw $0xaa, %ymm6, %ymm3, %ymm6
 vpsrld $0x10, %ymm3, %ymm3
 vpblendw $0xaa, %ymm10, %ymm3, %ymm10
 vpslld $0x10, %ymm8, %ymm3
 vpblendw $0xaa, %ymm3, %ymm4, %ymm3
 vpsrld $0x10, %ymm4, %ymm4
 vpblendw $0xaa, %ymm8, %ymm4, %ymm8
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm5, %ymm6, %ymm4
 vpsubw %ymm5, %ymm6, %ymm5
 vpaddw %ymm7, %ymm10, %ymm6
 vpsubw %ymm7, %ymm10, %ymm7
 vpaddw %ymm9, %ymm3, %ymm10
 vpsubw %ymm9, %ymm3, %ymm9
 vpaddw %ymm11, %ymm8, %ymm3
 vpsubw %ymm11, %ymm8, %ymm11
 vpsubw %ymm12, %ymm4, %ymm4
 vpaddw %ymm12, %ymm5, %ymm5
 vpsubw %ymm13, %ymm6, %ymm6
 vpaddw %ymm13, %ymm7, %ymm7
 vpsubw %ymm14, %ymm10, %ymm10
 vpaddw %ymm14, %ymm9, %ymm9
 vpsubw %ymm15, %ymm3, %ymm3
 vpaddw %ymm15, %ymm11, %ymm11
 vmovdqa 864(%rsi), %ymm14
 vmovdqa 928(%rsi), %ymm15
 vmovdqa 896(%rsi), %ymm8
 vmovdqa 960(%rsi), %ymm2
 vpmullw %ymm14, %ymm10, %ymm12
 vpmullw %ymm14, %ymm3, %ymm13
 vpmullw %ymm15, %ymm9, %ymm14
 vpmullw %ymm15, %ymm11, %ymm15
 vpmulhw %ymm8, %ymm10, %ymm10
 vpmulhw %ymm8, %ymm3, %ymm3
 vpmulhw %ymm2, %ymm9, %ymm9
 vpmulhw %ymm2, %ymm11, %ymm11
 vpmulhw %ymm0, %ymm12, %ymm12
 vpmulhw %ymm0, %ymm13, %ymm13
 vpmulhw %ymm0, %ymm14, %ymm14
 vpmulhw %ymm0, %ymm15, %ymm15
 vpaddw %ymm10, %ymm4, %ymm8
 vpsubw %ymm10, %ymm4, %ymm10
 vpaddw %ymm3, %ymm6, %ymm4
 vpsubw %ymm3, %ymm6, %ymm3
 vpaddw %ymm9, %ymm5, %ymm6
 vpsubw %ymm9, %ymm5, %ymm9
 vpaddw %ymm11, %ymm7, %ymm5
 vpsubw %ymm11, %ymm7, %ymm11
 vpsubw %ymm12, %ymm8, %ymm8
 vpaddw %ymm12, %ymm10, %ymm10
 vpsubw %ymm13, %ymm4, %ymm4
 vpaddw %ymm13, %ymm3, %ymm3
 vpsubw %ymm14, %ymm6, %ymm6
 vpaddw %ymm14, %ymm9, %ymm9
 vpsubw %ymm15, %ymm5, %ymm5
 vpaddw %ymm15, %ymm11, %ymm11
 vmovdqa %ymm8, 256(%rdi)
 vmovdqa %ymm4, 288(%rdi)
 vmovdqa %ymm10, 320(%rdi)
 vmovdqa %ymm3, 352(%rdi)
 vmovdqa %ymm6, 384(%rdi)
 vmovdqa %ymm5, 416(%rdi)
 vmovdqa %ymm9, 448(%rdi)
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

S2N_BN_SIZE_DIRECTIVE(mlkem_ntt_x86)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
