// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Reorder ML-KEM polynomial coefficients for x86 implementation
// Input a[256] (signed 16-bit words); output a[256] (signed 16-bit words)
//
// This accepts an array of 256 16-bit numbers and reorders them.
//
// extern void mlkem_unpack(int16_t a[static 256]);

// Standard x86-64 ABI: RDI = a
// Microsoft x64 ABI:   RCX = a
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_unpack)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_unpack)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_unpack)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_unpack):
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

 vmovdqa (%rdi), %ymm4
 vmovdqa 32(%rdi), %ymm5
 vmovdqa 64(%rdi), %ymm6
 vmovdqa 96(%rdi), %ymm7
 vmovdqa 128(%rdi), %ymm8
 vmovdqa 160(%rdi), %ymm9
 vmovdqa 192(%rdi), %ymm10
 vmovdqa 224(%rdi), %ymm11
 vperm2i128 $0x20, %ymm8, %ymm4, %ymm3
 vperm2i128 $0x31, %ymm8, %ymm4, %ymm8
 vperm2i128 $0x20, %ymm9, %ymm5, %ymm4
 vperm2i128 $0x31, %ymm9, %ymm5, %ymm9
 vperm2i128 $0x20, %ymm10, %ymm6, %ymm5
 vperm2i128 $0x31, %ymm10, %ymm6, %ymm10
 vperm2i128 $0x20, %ymm11, %ymm7, %ymm6
 vperm2i128 $0x31, %ymm11, %ymm7, %ymm11
 vpunpcklqdq %ymm5, %ymm3, %ymm7
 vpunpckhqdq %ymm5, %ymm3, %ymm5
 vpunpcklqdq %ymm10, %ymm8, %ymm3
 vpunpckhqdq %ymm10, %ymm8, %ymm10
 vpunpcklqdq %ymm6, %ymm4, %ymm8
 vpunpckhqdq %ymm6, %ymm4, %ymm6
 vpunpcklqdq %ymm11, %ymm9, %ymm4
 vpunpckhqdq %ymm11, %ymm9, %ymm11
 vmovsldup %ymm8, %ymm9
 vpblendd $0xaa, %ymm9, %ymm7, %ymm9
 vpsrlq $0x20, %ymm7, %ymm7
 vpblendd $0xaa, %ymm8, %ymm7, %ymm8
 vmovsldup %ymm6, %ymm7
 vpblendd $0xaa, %ymm7, %ymm5, %ymm7
 vpsrlq $0x20, %ymm5, %ymm5
 vpblendd $0xaa, %ymm6, %ymm5, %ymm6
 vmovsldup %ymm4, %ymm5
 vpblendd $0xaa, %ymm5, %ymm3, %ymm5
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm4, %ymm3, %ymm4
 vmovsldup %ymm11, %ymm3
 vpblendd $0xaa, %ymm3, %ymm10, %ymm3
 vpsrlq $0x20, %ymm10, %ymm10
 vpblendd $0xaa, %ymm11, %ymm10, %ymm11
 vpslld $0x10, %ymm5, %ymm10
 vpblendw $0xaa, %ymm10, %ymm9, %ymm10
 vpsrld $0x10, %ymm9, %ymm9
 vpblendw $0xaa, %ymm5, %ymm9, %ymm5
 vpslld $0x10, %ymm4, %ymm9
 vpblendw $0xaa, %ymm9, %ymm8, %ymm9
 vpsrld $0x10, %ymm8, %ymm8
 vpblendw $0xaa, %ymm4, %ymm8, %ymm4
 vpslld $0x10, %ymm3, %ymm8
 vpblendw $0xaa, %ymm8, %ymm7, %ymm8
 vpsrld $0x10, %ymm7, %ymm7
 vpblendw $0xaa, %ymm3, %ymm7, %ymm3
 vpslld $0x10, %ymm11, %ymm7
 vpblendw $0xaa, %ymm7, %ymm6, %ymm7
 vpsrld $0x10, %ymm6, %ymm6
 vpblendw $0xaa, %ymm11, %ymm6, %ymm11
 vmovdqa %ymm10, (%rdi)
 vmovdqa %ymm5, 32(%rdi)
 vmovdqa %ymm9, 64(%rdi)
 vmovdqa %ymm4, 96(%rdi)
 vmovdqa %ymm8, 128(%rdi)
 vmovdqa %ymm3, 160(%rdi)
 vmovdqa %ymm7, 192(%rdi)
 vmovdqa %ymm11, 224(%rdi)
 vmovdqa 256(%rdi), %ymm4
 vmovdqa 288(%rdi), %ymm5
 vmovdqa 320(%rdi), %ymm6
 vmovdqa 352(%rdi), %ymm7
 vmovdqa 384(%rdi), %ymm8
 vmovdqa 416(%rdi), %ymm9
 vmovdqa 448(%rdi), %ymm10
 vmovdqa 480(%rdi), %ymm11
 vperm2i128 $0x20, %ymm8, %ymm4, %ymm3
 vperm2i128 $0x31, %ymm8, %ymm4, %ymm8
 vperm2i128 $0x20, %ymm9, %ymm5, %ymm4
 vperm2i128 $0x31, %ymm9, %ymm5, %ymm9
 vperm2i128 $0x20, %ymm10, %ymm6, %ymm5
 vperm2i128 $0x31, %ymm10, %ymm6, %ymm10
 vperm2i128 $0x20, %ymm11, %ymm7, %ymm6
 vperm2i128 $0x31, %ymm11, %ymm7, %ymm11
 vpunpcklqdq %ymm5, %ymm3, %ymm7
 vpunpckhqdq %ymm5, %ymm3, %ymm5
 vpunpcklqdq %ymm10, %ymm8, %ymm3
 vpunpckhqdq %ymm10, %ymm8, %ymm10
 vpunpcklqdq %ymm6, %ymm4, %ymm8
 vpunpckhqdq %ymm6, %ymm4, %ymm6
 vpunpcklqdq %ymm11, %ymm9, %ymm4
 vpunpckhqdq %ymm11, %ymm9, %ymm11
 vmovsldup %ymm8, %ymm9
 vpblendd $0xaa, %ymm9, %ymm7, %ymm9
 vpsrlq $0x20, %ymm7, %ymm7
 vpblendd $0xaa, %ymm8, %ymm7, %ymm8
 vmovsldup %ymm6, %ymm7
 vpblendd $0xaa, %ymm7, %ymm5, %ymm7
 vpsrlq $0x20, %ymm5, %ymm5
 vpblendd $0xaa, %ymm6, %ymm5, %ymm6
 vmovsldup %ymm4, %ymm5
 vpblendd $0xaa, %ymm5, %ymm3, %ymm5
 vpsrlq $0x20, %ymm3, %ymm3
 vpblendd $0xaa, %ymm4, %ymm3, %ymm4
 vmovsldup %ymm11, %ymm3
 vpblendd $0xaa, %ymm3, %ymm10, %ymm3
 vpsrlq $0x20, %ymm10, %ymm10
 vpblendd $0xaa, %ymm11, %ymm10, %ymm11
 vpslld $0x10, %ymm5, %ymm10
 vpblendw $0xaa, %ymm10, %ymm9, %ymm10
 vpsrld $0x10, %ymm9, %ymm9
 vpblendw $0xaa, %ymm5, %ymm9, %ymm5
 vpslld $0x10, %ymm4, %ymm9
 vpblendw $0xaa, %ymm9, %ymm8, %ymm9
 vpsrld $0x10, %ymm8, %ymm8
 vpblendw $0xaa, %ymm4, %ymm8, %ymm4
 vpslld $0x10, %ymm3, %ymm8
 vpblendw $0xaa, %ymm8, %ymm7, %ymm8
 vpsrld $0x10, %ymm7, %ymm7
 vpblendw $0xaa, %ymm3, %ymm7, %ymm3
 vpslld $0x10, %ymm11, %ymm7
 vpblendw $0xaa, %ymm7, %ymm6, %ymm7
 vpsrld $0x10, %ymm6, %ymm6
 vpblendw $0xaa, %ymm11, %ymm6, %ymm11
 vmovdqa %ymm10, 256(%rdi)
 vmovdqa %ymm5, 288(%rdi)
 vmovdqa %ymm9, 320(%rdi)
 vmovdqa %ymm4, 352(%rdi)
 vmovdqa %ymm8, 384(%rdi)
 vmovdqa %ymm3, 416(%rdi)
 vmovdqa %ymm7, 448(%rdi)
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

S2N_BN_SIZE_DIRECTIVE(mlkem_unpack)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
