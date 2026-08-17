// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Pack ML-KEM polynomial coefficients as 12-bit numbers
// Input a[256] (signed 16-bit words); output r[384] (bytes)
//
// This accepts an array of 256 16-bit numbers assumed to be in the range
// 0 <= a[i] < 2^12 (typically they will be < 3329, the ML-KEM prime).
// It packs them into the output array as 12-bit unsigned numbers.
//
// extern void mlkem_tobytes(uint8_t r[static 384],const int16_t a[static 256]);

// Standard x86-64 ABI: RDI = r, RSI = a
// Microsoft x64 ABI:   RCX = r, RDX = a
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_tobytes)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_tobytes)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_tobytes)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_tobytes):
 CFI_START
 _CET_ENDBR

#if WINDOWS_ABI
  CFI_DEC_RSP(128)
  CFI_STACKSAVEU(%xmm6,0)
  CFI_STACKSAVEU(%xmm7,16)
  CFI_STACKSAVEU(%xmm8,32)
  CFI_STACKSAVEU(%xmm9,48)
  CFI_STACKSAVEU(%xmm10,64)
  CFI_STACKSAVEU(%xmm11,80)
  CFI_STACKSAVEU(%xmm12,96)
  CFI_STACKSAVE(%rdi,112)
  CFI_STACKSAVE(%rsi,120)
  mov %rcx, %rdi
  mov %rdx, %rsi
#endif

 mov $0xd010d01, %eax
 vmovd %eax, %xmm0
 vpbroadcastd %xmm0, %ymm0
 vmovdqa (%rsi), %ymm5
 vmovdqa 32(%rsi), %ymm6
 vmovdqa 64(%rsi), %ymm7
 vmovdqa 96(%rsi), %ymm8
 vmovdqa 128(%rsi), %ymm9
 vmovdqa 160(%rsi), %ymm10
 vmovdqa 192(%rsi), %ymm11
 vmovdqa 224(%rsi), %ymm12
 vpsllw $0xc, %ymm6, %ymm4
 vpor %ymm4, %ymm5, %ymm4
 vpsrlw $0x4, %ymm6, %ymm5
 vpsllw $0x8, %ymm7, %ymm6
 vpor %ymm5, %ymm6, %ymm5
 vpsrlw $0x8, %ymm7, %ymm6
 vpsllw $0x4, %ymm8, %ymm7
 vpor %ymm6, %ymm7, %ymm6
 vpsllw $0xc, %ymm10, %ymm7
 vpor %ymm7, %ymm9, %ymm7
 vpsrlw $0x4, %ymm10, %ymm8
 vpsllw $0x8, %ymm11, %ymm9
 vpor %ymm8, %ymm9, %ymm8
 vpsrlw $0x8, %ymm11, %ymm9
 vpsllw $0x4, %ymm12, %ymm10
 vpor %ymm9, %ymm10, %ymm9
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
 vmovsldup %ymm4, %ymm8
 vpblendd $0xaa, %ymm8, %ymm3, %ymm8
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm4, %ymm3, %ymm4
 vmovsldup %ymm5, %ymm3
 vpblendd $0xaa, %ymm3, %ymm6, %ymm3
 vpsrlq $0x20, %ymm6, %ymm6
 vpblendd $0xaa, %ymm5, %ymm6, %ymm5
 vmovsldup %ymm9, %ymm6
 vpblendd $0xaa, %ymm6, %ymm7, %ymm6
 vpsrlq $0x20, %ymm7, %ymm7
 vpblendd $0xaa, %ymm9, %ymm7, %ymm9
 vpunpcklqdq %ymm3, %ymm8, %ymm7
 vpunpckhqdq %ymm3, %ymm8, %ymm3
 vpunpcklqdq %ymm4, %ymm6, %ymm8
 vpunpckhqdq %ymm4, %ymm6, %ymm4
 vpunpcklqdq %ymm9, %ymm5, %ymm6
 vpunpckhqdq %ymm9, %ymm5, %ymm9
 vperm2i128 $0x20, %ymm8, %ymm7, %ymm5
 vperm2i128 $0x31, %ymm8, %ymm7, %ymm8
 vperm2i128 $0x20, %ymm3, %ymm6, %ymm7
 vperm2i128 $0x31, %ymm3, %ymm6, %ymm3
 vperm2i128 $0x20, %ymm9, %ymm4, %ymm6
 vperm2i128 $0x31, %ymm9, %ymm4, %ymm9
 vmovdqu %ymm5, (%rdi)
 vmovdqu %ymm7, 32(%rdi)
 vmovdqu %ymm6, 64(%rdi)
 vmovdqu %ymm8, 96(%rdi)
 vmovdqu %ymm3, 128(%rdi)
 vmovdqu %ymm9, 160(%rdi)
 vmovdqa 256(%rsi), %ymm5
 vmovdqa 288(%rsi), %ymm6
 vmovdqa 320(%rsi), %ymm7
 vmovdqa 352(%rsi), %ymm8
 vmovdqa 384(%rsi), %ymm9
 vmovdqa 416(%rsi), %ymm10
 vmovdqa 448(%rsi), %ymm11
 vmovdqa 480(%rsi), %ymm12
 vpsllw $0xc, %ymm6, %ymm4
 vpor %ymm4, %ymm5, %ymm4
 vpsrlw $0x4, %ymm6, %ymm5
 vpsllw $0x8, %ymm7, %ymm6
 vpor %ymm5, %ymm6, %ymm5
 vpsrlw $0x8, %ymm7, %ymm6
 vpsllw $0x4, %ymm8, %ymm7
 vpor %ymm6, %ymm7, %ymm6
 vpsllw $0xc, %ymm10, %ymm7
 vpor %ymm7, %ymm9, %ymm7
 vpsrlw $0x4, %ymm10, %ymm8
 vpsllw $0x8, %ymm11, %ymm9
 vpor %ymm8, %ymm9, %ymm8
 vpsrlw $0x8, %ymm11, %ymm9
 vpsllw $0x4, %ymm12, %ymm10
 vpor %ymm9, %ymm10, %ymm9
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
 vmovsldup %ymm4, %ymm8
 vpblendd $0xaa, %ymm8, %ymm3, %ymm8
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm4, %ymm3, %ymm4
 vmovsldup %ymm5, %ymm3
 vpblendd $0xaa, %ymm3, %ymm6, %ymm3
 vpsrlq $0x20, %ymm6, %ymm6
 vpblendd $0xaa, %ymm5, %ymm6, %ymm5
 vmovsldup %ymm9, %ymm6
 vpblendd $0xaa, %ymm6, %ymm7, %ymm6
 vpsrlq $0x20, %ymm7, %ymm7
 vpblendd $0xaa, %ymm9, %ymm7, %ymm9
 vpunpcklqdq %ymm3, %ymm8, %ymm7
 vpunpckhqdq %ymm3, %ymm8, %ymm3
 vpunpcklqdq %ymm4, %ymm6, %ymm8
 vpunpckhqdq %ymm4, %ymm6, %ymm4
 vpunpcklqdq %ymm9, %ymm5, %ymm6
 vpunpckhqdq %ymm9, %ymm5, %ymm9
 vperm2i128 $0x20, %ymm8, %ymm7, %ymm5
 vperm2i128 $0x31, %ymm8, %ymm7, %ymm8
 vperm2i128 $0x20, %ymm3, %ymm6, %ymm7
 vperm2i128 $0x31, %ymm3, %ymm6, %ymm3
 vperm2i128 $0x20, %ymm9, %ymm4, %ymm6
 vperm2i128 $0x31, %ymm9, %ymm4, %ymm9
 vmovdqu %ymm5, 192(%rdi)
 vmovdqu %ymm7, 224(%rdi)
 vmovdqu %ymm6, 256(%rdi)
 vmovdqu %ymm8, 288(%rdi)
 vmovdqu %ymm3, 320(%rdi)
 vmovdqu %ymm9, 352(%rdi)

#if WINDOWS_ABI
 CFI_STACKLOADU(%xmm6,0)
 CFI_STACKLOADU(%xmm7,16)
 CFI_STACKLOADU(%xmm8,32)
 CFI_STACKLOADU(%xmm9,48)
 CFI_STACKLOADU(%xmm10,64)
 CFI_STACKLOADU(%xmm11,80)
 CFI_STACKLOADU(%xmm12,96)
 CFI_STACKLOAD(%rdi,112)
 CFI_STACKLOAD(%rsi,120)
 CFI_INC_RSP(128)
#endif
 CFI_RET


S2N_BN_SIZE_DIRECTIVE(mlkem_tobytes)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
