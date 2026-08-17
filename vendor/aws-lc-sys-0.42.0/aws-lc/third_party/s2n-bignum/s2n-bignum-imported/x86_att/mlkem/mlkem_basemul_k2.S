// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Scalar product of 2-element polynomial vectors in NTT domain, with mulcache
// Inputs a[512], b[512], bt[256] (signed 16-bit words); output r[256] (signed 16-bit words)
//
// The inputs a and b are considered as 2-element vectors of linear
// polynomials in the NTT domain (in Montgomery form), and the bt
// argument an analogous 2-element vector of mulcaches for the bi:
//
//   a0 = a[0..255], a1 = a[256..511]
//   b0 = b[0..255], b1 = b[256..511]
//   bt0 = bt[0..127], bt1 = bt[128..255]
//
// Scalar multiplication of those 2-element vectors is performed,
// with base multiplication in Fq[X]/(X^2-zeta^i'), with zeta^i'
// being a power of zeta = 17, with i bit-reversed as used for NTTs,
// making use of the mulcache for optimization.
//
// Coefficients of a0 and a1 are assumed <= 2^12 and the bts are
// assumed to be as computed by mlkem_mulcache_compute.
//
// extern void mlkem_basemul_k2
//      (int16_t r[static 256],const int16_t a[static 512],
//       const int16_t b[static 512], const int16_t bt[static 256]);
//
// Standard x86-64 ABI: RDI = a
// Microsoft x64 ABI:   RCX = a
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_basemul_k2)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_basemul_k2)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_basemul_k2)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_basemul_k2):
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
        CFI_STACKSAVE(%rdi,144)
        CFI_STACKSAVE(%rsi,152)
        CFI_STACKSAVE(%rdx,160)
        CFI_STACKSAVE(%rcx,168)
        mov %rcx, %rdi
        mov %rdx, %rsi
        mov %r8, %rdx
        mov %r9, %rcx
#endif

        mov $0xd010d01, %eax
        vmovd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0
        mov $0xf301f301, %eax
        vmovd %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1
        vmovdqa (%rsi), %ymm2
        vmovdqa 32(%rsi), %ymm3
        vmovdqa (%rdx), %ymm4
        vmovdqa 32(%rdx), %ymm5
        vmovdqa (%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, (%rdi)
        vmovdqa %ymm9, 32(%rdi)
        vmovdqa 64(%rsi), %ymm2
        vmovdqa 96(%rsi), %ymm3
        vmovdqa 64(%rdx), %ymm4
        vmovdqa 96(%rdx), %ymm5
        vmovdqa 32(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm13, %ymm8, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 64(%rdi)
        vmovdqa %ymm9, 96(%rdi)
        vmovdqa 128(%rsi), %ymm2
        vmovdqa 160(%rsi), %ymm3
        vmovdqa 128(%rdx), %ymm4
        vmovdqa 160(%rdx), %ymm5
        vmovdqa 64(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 128(%rdi)
        vmovdqa %ymm9, 160(%rdi)
        vmovdqa 192(%rsi), %ymm2
        vmovdqa 224(%rsi), %ymm3
        vmovdqa 192(%rdx), %ymm4
        vmovdqa 224(%rdx), %ymm5
        vmovdqa 96(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm13, %ymm8, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 192(%rdi)
        vmovdqa %ymm9, 224(%rdi)
        vmovdqa 256(%rsi), %ymm2
        vmovdqa 288(%rsi), %ymm3
        vmovdqa 256(%rdx), %ymm4
        vmovdqa 288(%rdx), %ymm5
        vmovdqa 128(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 256(%rdi)
        vmovdqa %ymm9, 288(%rdi)
        vmovdqa 320(%rsi), %ymm2
        vmovdqa 352(%rsi), %ymm3
        vmovdqa 320(%rdx), %ymm4
        vmovdqa 352(%rdx), %ymm5
        vmovdqa 160(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm13, %ymm8, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 320(%rdi)
        vmovdqa %ymm9, 352(%rdi)
        vmovdqa 384(%rsi), %ymm2
        vmovdqa 416(%rsi), %ymm3
        vmovdqa 384(%rdx), %ymm4
        vmovdqa 416(%rdx), %ymm5
        vmovdqa 192(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 384(%rdi)
        vmovdqa %ymm9, 416(%rdi)
        vmovdqa 448(%rsi), %ymm2
        vmovdqa 480(%rsi), %ymm3
        vmovdqa 448(%rdx), %ymm4
        vmovdqa 480(%rdx), %ymm5
        vmovdqa 224(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm13, %ymm8, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 448(%rdi)
        vmovdqa %ymm9, 480(%rdi)
        vmovdqa 512(%rsi), %ymm2
        vmovdqa 544(%rsi), %ymm3
        vmovdqa 512(%rdx), %ymm4
        vmovdqa 544(%rdx), %ymm5
        vmovdqa 256(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa (%rdi), %ymm8
        vmovdqa 32(%rdi), %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, (%rdi)
        vmovdqa %ymm9, 32(%rdi)
        vmovdqa 576(%rsi), %ymm2
        vmovdqa 608(%rsi), %ymm3
        vmovdqa 576(%rdx), %ymm4
        vmovdqa 608(%rdx), %ymm5
        vmovdqa 288(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm13, %ymm8, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa 64(%rdi), %ymm8
        vmovdqa 96(%rdi), %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 64(%rdi)
        vmovdqa %ymm9, 96(%rdi)
        vmovdqa 640(%rsi), %ymm2
        vmovdqa 672(%rsi), %ymm3
        vmovdqa 640(%rdx), %ymm4
        vmovdqa 672(%rdx), %ymm5
        vmovdqa 320(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa 128(%rdi), %ymm8
        vmovdqa 160(%rdi), %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 128(%rdi)
        vmovdqa %ymm9, 160(%rdi)
        vmovdqa 704(%rsi), %ymm2
        vmovdqa 736(%rsi), %ymm3
        vmovdqa 704(%rdx), %ymm4
        vmovdqa 736(%rdx), %ymm5
        vmovdqa 352(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm13, %ymm8, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa 192(%rdi), %ymm8
        vmovdqa 224(%rdi), %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 192(%rdi)
        vmovdqa %ymm9, 224(%rdi)
        vmovdqa 768(%rsi), %ymm2
        vmovdqa 800(%rsi), %ymm3
        vmovdqa 768(%rdx), %ymm4
        vmovdqa 800(%rdx), %ymm5
        vmovdqa 384(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa 256(%rdi), %ymm8
        vmovdqa 288(%rdi), %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 256(%rdi)
        vmovdqa %ymm9, 288(%rdi)
        vmovdqa 832(%rsi), %ymm2
        vmovdqa 864(%rsi), %ymm3
        vmovdqa 832(%rdx), %ymm4
        vmovdqa 864(%rdx), %ymm5
        vmovdqa 416(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm13, %ymm8, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa 320(%rdi), %ymm8
        vmovdqa 352(%rdi), %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 320(%rdi)
        vmovdqa %ymm9, 352(%rdi)
        vmovdqa 896(%rsi), %ymm2
        vmovdqa 928(%rsi), %ymm3
        vmovdqa 896(%rdx), %ymm4
        vmovdqa 928(%rdx), %ymm5
        vmovdqa 448(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm8, %ymm13, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa 384(%rdi), %ymm8
        vmovdqa 416(%rdi), %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 384(%rdi)
        vmovdqa %ymm9, 416(%rdi)
        vmovdqa 960(%rsi), %ymm2
        vmovdqa 992(%rsi), %ymm3
        vmovdqa 960(%rdx), %ymm4
        vmovdqa 992(%rdx), %ymm5
        vmovdqa 480(%rcx), %ymm6
        vpmullw %ymm2, %ymm1, %ymm13
        vpmullw %ymm3, %ymm1, %ymm14
        vpmullw %ymm13, %ymm4, %ymm7
        vpmullw %ymm13, %ymm5, %ymm9
        vpmullw %ymm14, %ymm6, %ymm8
        vpmullw %ymm14, %ymm4, %ymm10
        vpmulhw %ymm7, %ymm0, %ymm7
        vpmulhw %ymm9, %ymm0, %ymm9
        vpmulhw %ymm8, %ymm0, %ymm8
        vpmulhw %ymm10, %ymm0, %ymm10
        vpmulhw %ymm2, %ymm4, %ymm11
        vpmulhw %ymm2, %ymm5, %ymm12
        vpmulhw %ymm3, %ymm6, %ymm13
        vpmulhw %ymm3, %ymm4, %ymm14
        vpsubw %ymm7, %ymm11, %ymm7
        vpsubw %ymm9, %ymm12, %ymm9
        vpsubw %ymm13, %ymm8, %ymm8
        vpsubw %ymm10, %ymm14, %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa 448(%rdi), %ymm8
        vmovdqa 480(%rdi), %ymm10
        vpaddw %ymm7, %ymm8, %ymm7
        vpaddw %ymm9, %ymm10, %ymm9
        vmovdqa %ymm7, 448(%rdi)
        vmovdqa %ymm9, 480(%rdi)

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
        CFI_STACKLOAD(%rdi,144)
        CFI_STACKLOAD(%rsi,152)
        CFI_STACKLOAD(%rdx,160)
        CFI_STACKLOAD(%rcx,168)
        CFI_INC_RSP(176)
#endif

 CFI_RET

S2N_BN_SIZE_DIRECTIVE(mlkem_basemul_k2)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
