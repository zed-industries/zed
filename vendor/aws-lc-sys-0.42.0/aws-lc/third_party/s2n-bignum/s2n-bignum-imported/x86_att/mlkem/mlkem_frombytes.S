// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Unppack ML-KEM polynomial coefficients as 12-bit numbers
// Input a[384] (bytes); output r[256] (signed 16-bit words)
//
// This accepts an array of 384 bytes and unpcaks them into 256 16-bit numbers
// in the range 0 <= a[i] < 2^12 (typically they will be < 3329, the ML-KEM prime).
//
// extern void mlkem_frombytes(int16_t r[static 256],const uint8_t a[static 384]);

// Standard x86-64 ABI: RDI = r, RSI = a
// Microsoft x64 ABI:   RCX = r, RDX = a
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_frombytes)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_frombytes)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_frombytes)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_frombytes):
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

 mov $0xfff0fff, %eax
 vmovd %eax, %xmm0
 vpbroadcastd %xmm0, %ymm0
 vmovdqu (%rsi), %ymm4
 vmovdqu 32(%rsi), %ymm5
 vmovdqu 64(%rsi), %ymm6
 vmovdqu 96(%rsi), %ymm7
 vmovdqu 128(%rsi), %ymm8
 vmovdqu 160(%rsi), %ymm9
 vperm2i128 $0x20, %ymm7, %ymm4, %ymm3
 vperm2i128 $0x31, %ymm7, %ymm4, %ymm7
 vperm2i128 $0x20, %ymm8, %ymm5, %ymm4
 vperm2i128 $0x31, %ymm8, %ymm5, %ymm8
 vperm2i128 $0x20, %ymm9, %ymm6, %ymm5
 vperm2i128 $0x31, %ymm9, %ymm6, %ymm9
 vpunpcklqdq %ymm8, %ymm3, %ymm6
 vpunpckhqdq %ymm8, %ymm3, %ymm8
 vpunpcklqdq %ymm5, %ymm7, %ymm3
 vpunpckhqdq %ymm5, %ymm7, %ymm5
 vpunpcklqdq %ymm9, %ymm4, %ymm7
 vpunpckhqdq %ymm9, %ymm4, %ymm9
 vmovsldup %ymm5, %ymm4
 vpblendd $0xaa, %ymm4, %ymm6, %ymm4
 vpsrlq $0x20, %ymm6, %ymm6
 vpblendd $0xaa, %ymm5, %ymm6, %ymm5
 vmovsldup %ymm7, %ymm6
 vpblendd $0xaa, %ymm6, %ymm8, %ymm6
 vpsrlq $0x20, %ymm8, %ymm8
 vpblendd $0xaa, %ymm7, %ymm8, %ymm7
 vmovsldup %ymm9, %ymm8
 vpblendd $0xaa, %ymm8, %ymm3, %ymm8
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm9, %ymm3, %ymm9
 vpslld $0x10, %ymm7, %ymm10
 vpblendw $0xaa, %ymm10, %ymm4, %ymm10
 vpsrld $0x10, %ymm4, %ymm4
 vpblendw $0xaa, %ymm7, %ymm4, %ymm7
 vpslld $0x10, %ymm8, %ymm4
 vpblendw $0xaa, %ymm4, %ymm5, %ymm4
 vpsrld $0x10, %ymm5, %ymm5
 vpblendw $0xaa, %ymm8, %ymm5, %ymm8
 vpslld $0x10, %ymm9, %ymm5
 vpblendw $0xaa, %ymm5, %ymm6, %ymm5
 vpsrld $0x10, %ymm6, %ymm6
 vpblendw $0xaa, %ymm9, %ymm6, %ymm9
 vpsrlw $0xc, %ymm10, %ymm11
 vpsllw $0x4, %ymm7, %ymm12
 vpor %ymm11, %ymm12, %ymm11
 vpand %ymm0, %ymm10, %ymm10
 vpand %ymm0, %ymm11, %ymm11
 vpsrlw $0x8, %ymm7, %ymm12
 vpsllw $0x8, %ymm4, %ymm13
 vpor %ymm12, %ymm13, %ymm12
 vpand %ymm0, %ymm12, %ymm12
 vpsrlw $0x4, %ymm4, %ymm13
 vpand %ymm0, %ymm13, %ymm13
 vpsrlw $0xc, %ymm8, %ymm14
 vpsllw $0x4, %ymm5, %ymm15
 vpor %ymm14, %ymm15, %ymm14
 vpand %ymm0, %ymm8, %ymm8
 vpand %ymm0, %ymm14, %ymm14
 vpsrlw $0x8, %ymm5, %ymm15
 vpsllw $0x8, %ymm9, %ymm1
 vpor %ymm15, %ymm1, %ymm15
 vpand %ymm0, %ymm15, %ymm15
 vpsrlw $0x4, %ymm9, %ymm1
 vpand %ymm0, %ymm1, %ymm1
 vmovdqa %ymm10, (%rdi)
 vmovdqa %ymm11, 32(%rdi)
 vmovdqa %ymm12, 64(%rdi)
 vmovdqa %ymm13, 96(%rdi)
 vmovdqa %ymm8, 128(%rdi)
 vmovdqa %ymm14, 160(%rdi)
 vmovdqa %ymm15, 192(%rdi)
 vmovdqa %ymm1, 224(%rdi)
 vmovdqu 192(%rsi), %ymm4
 vmovdqu 224(%rsi), %ymm5
 vmovdqu 256(%rsi), %ymm6
 vmovdqu 288(%rsi), %ymm7
 vmovdqu 320(%rsi), %ymm8
 vmovdqu 352(%rsi), %ymm9
 vperm2i128 $0x20, %ymm7, %ymm4, %ymm3
 vperm2i128 $0x31, %ymm7, %ymm4, %ymm7
 vperm2i128 $0x20, %ymm8, %ymm5, %ymm4
 vperm2i128 $0x31, %ymm8, %ymm5, %ymm8
 vperm2i128 $0x20, %ymm9, %ymm6, %ymm5
 vperm2i128 $0x31, %ymm9, %ymm6, %ymm9
 vpunpcklqdq %ymm8, %ymm3, %ymm6
 vpunpckhqdq %ymm8, %ymm3, %ymm8
 vpunpcklqdq %ymm5, %ymm7, %ymm3
 vpunpckhqdq %ymm5, %ymm7, %ymm5
 vpunpcklqdq %ymm9, %ymm4, %ymm7
 vpunpckhqdq %ymm9, %ymm4, %ymm9
 vmovsldup %ymm5, %ymm4
 vpblendd $0xaa, %ymm4, %ymm6, %ymm4
 vpsrlq $0x20, %ymm6, %ymm6
 vpblendd $0xaa, %ymm5, %ymm6, %ymm5
 vmovsldup %ymm7, %ymm6
 vpblendd $0xaa, %ymm6, %ymm8, %ymm6
 vpsrlq $0x20, %ymm8, %ymm8
 vpblendd $0xaa, %ymm7, %ymm8, %ymm7
 vmovsldup %ymm9, %ymm8
 vpblendd $0xaa, %ymm8, %ymm3, %ymm8
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm9, %ymm3, %ymm9
 vpslld $0x10, %ymm7, %ymm10
 vpblendw $0xaa, %ymm10, %ymm4, %ymm10
 vpsrld $0x10, %ymm4, %ymm4
 vpblendw $0xaa, %ymm7, %ymm4, %ymm7
 vpslld $0x10, %ymm8, %ymm4
 vpblendw $0xaa, %ymm4, %ymm5, %ymm4
 vpsrld $0x10, %ymm5, %ymm5
 vpblendw $0xaa, %ymm8, %ymm5, %ymm8
 vpslld $0x10, %ymm9, %ymm5
 vpblendw $0xaa, %ymm5, %ymm6, %ymm5
 vpsrld $0x10, %ymm6, %ymm6
 vpblendw $0xaa, %ymm9, %ymm6, %ymm9
 vpsrlw $0xc, %ymm10, %ymm11
 vpsllw $0x4, %ymm7, %ymm12
 vpor %ymm11, %ymm12, %ymm11
 vpand %ymm0, %ymm10, %ymm10
 vpand %ymm0, %ymm11, %ymm11
 vpsrlw $0x8, %ymm7, %ymm12
 vpsllw $0x8, %ymm4, %ymm13
 vpor %ymm12, %ymm13, %ymm12
 vpand %ymm0, %ymm12, %ymm12
 vpsrlw $0x4, %ymm4, %ymm13
 vpand %ymm0, %ymm13, %ymm13
 vpsrlw $0xc, %ymm8, %ymm14
 vpsllw $0x4, %ymm5, %ymm15
 vpor %ymm14, %ymm15, %ymm14
 vpand %ymm0, %ymm8, %ymm8
 vpand %ymm0, %ymm14, %ymm14
 vpsrlw $0x8, %ymm5, %ymm15
 vpsllw $0x8, %ymm9, %ymm1
 vpor %ymm15, %ymm1, %ymm15
 vpand %ymm0, %ymm15, %ymm15
 vpsrlw $0x4, %ymm9, %ymm1
 vpand %ymm0, %ymm1, %ymm1
 vmovdqa %ymm10, 256(%rdi)
 vmovdqa %ymm11, 288(%rdi)
 vmovdqa %ymm12, 320(%rdi)
 vmovdqa %ymm13, 352(%rdi)
 vmovdqa %ymm8, 384(%rdi)
 vmovdqa %ymm14, 416(%rdi)
 vmovdqa %ymm15, 448(%rdi)
 vmovdqa %ymm1, 480(%rdi)

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


S2N_BN_SIZE_DIRECTIVE(mlkem_frombytes)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
