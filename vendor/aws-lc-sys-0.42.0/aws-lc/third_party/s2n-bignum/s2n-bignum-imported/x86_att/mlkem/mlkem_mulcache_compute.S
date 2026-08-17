// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Precompute the mulcache data for a polynomial in the NTT domain
// Inputs a[256], qdata[624] (all signed 16-bit words); output x[128] (signed 16-bit words)
//
// The input array a is assumed to represent 128 linear polynomials
// in the NTT domain, p_i = a[f(i)] + a[g(i)] * X where each p_i is in
// Fq[X]/(X^2-zeta^i'), with zeta^i' being a power of zeta = 17, with i
// bit-reversed as used for NTTs, and f(i) and g(i) are ordering functions
// that map the coefficients per the specific order used in the AVX2
// implementation. For each such polynomial, the mulcache value is
// a[g(i)] * zeta^i' (modulo 3329 as usual), a value useful to
// perform base multiplication of polynomials efficiently. The The second
// parameter is expected to point to a table of constants whose definitions
// can be found in the mlkem-native repo or our "tests/test.c".
//
// extern void mlkem_mulcache_compute_x86
//      (int16_t x[static 128],const int16_t a[static 256],
//       const int16_t qdata[static 624]);
//
// Standard x86-64 ABI: RDI = x, RSI = a, RDX = qdata
// Microsoft x64 ABI:   RCX = x, RDX = a, R8 = qdata
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_mulcache_compute_x86)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_mulcache_compute_x86)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_mulcache_compute_x86)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_mulcache_compute_x86):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
  CFI_DEC_RSP(104)
  CFI_STACKSAVEU(%xmm6,0)
  CFI_STACKSAVEU(%xmm7,16)
  CFI_STACKSAVEU(%xmm8,32)
  CFI_STACKSAVEU(%xmm9,48)
  CFI_STACKSAVEU(%xmm10,64)
  CFI_STACKSAVE(%rdi,80)
  CFI_STACKSAVE(%rsi,88)
  CFI_STACKSAVE(%rdx,96)
  mov %rcx, %rdi
  mov %rdx, %rsi
  mov %r8, %rdx
#endif

 mov $0xd010d01, %eax
 vmovd %eax, %xmm0
 vpbroadcastd %xmm0, %ymm0
 vmovdqa 32(%rsi), %ymm2
 vmovdqa 96(%rsi), %ymm3
 vmovdqa 992(%rdx), %ymm4
 vmovdqa 1120(%rdx), %ymm1
 vpmullw %ymm2, %ymm1, %ymm5
 vpmullw %ymm3, %ymm1, %ymm6
 vpmulhw %ymm2, %ymm4, %ymm7
 vpmulhw %ymm3, %ymm4, %ymm8
 vpmulhw %ymm5, %ymm0, %ymm9
 vpmulhw %ymm6, %ymm0, %ymm10
 vpsubw %ymm9, %ymm7, %ymm7
 vpsubw %ymm10, %ymm8, %ymm8
 vmovdqa %ymm7, (%rdi)
 vmovdqa %ymm8, 32(%rdi)
 vmovdqa 160(%rsi), %ymm2
 vmovdqa 224(%rsi), %ymm3
 vmovdqa 1024(%rdx), %ymm4
 vmovdqa 1152(%rdx), %ymm1
 vpmullw %ymm2, %ymm1, %ymm5
 vpmullw %ymm3, %ymm1, %ymm6
 vpmulhw %ymm2, %ymm4, %ymm7
 vpmulhw %ymm3, %ymm4, %ymm8
 vpmulhw %ymm5, %ymm0, %ymm9
 vpmulhw %ymm6, %ymm0, %ymm10
 vpsubw %ymm9, %ymm7, %ymm7
 vpsubw %ymm10, %ymm8, %ymm8
 vmovdqa %ymm7, 64(%rdi)
 vmovdqa %ymm8, 96(%rdi)
 vmovdqa 288(%rsi), %ymm2
 vmovdqa 352(%rsi), %ymm3
 vmovdqa 1056(%rdx), %ymm4
 vmovdqa 1184(%rdx), %ymm1
 vpmullw %ymm2, %ymm1, %ymm5
 vpmullw %ymm3, %ymm1, %ymm6
 vpmulhw %ymm2, %ymm4, %ymm7
 vpmulhw %ymm3, %ymm4, %ymm8
 vpmulhw %ymm5, %ymm0, %ymm9
 vpmulhw %ymm6, %ymm0, %ymm10
 vpsubw %ymm9, %ymm7, %ymm7
 vpsubw %ymm10, %ymm8, %ymm8
 vmovdqa %ymm7, 128(%rdi)
 vmovdqa %ymm8, 160(%rdi)
 vmovdqa 416(%rsi), %ymm2
 vmovdqa 480(%rsi), %ymm3
 vmovdqa 1088(%rdx), %ymm4
 vmovdqa 1216(%rdx), %ymm1
 vpmullw %ymm2, %ymm1, %ymm5
 vpmullw %ymm3, %ymm1, %ymm6
 vpmulhw %ymm2, %ymm4, %ymm7
 vpmulhw %ymm3, %ymm4, %ymm8
 vpmulhw %ymm5, %ymm0, %ymm9
 vpmulhw %ymm6, %ymm0, %ymm10
 vpsubw %ymm9, %ymm7, %ymm7
 vpsubw %ymm10, %ymm8, %ymm8
 vmovdqa %ymm7, 192(%rdi)
 vmovdqa %ymm8, 224(%rdi)

#if WINDOWS_ABI
  CFI_STACKLOADU(%xmm6,0)
  CFI_STACKLOADU(%xmm7,16)
  CFI_STACKLOADU(%xmm8,32)
  CFI_STACKLOADU(%xmm9,48)
  CFI_STACKLOADU(%xmm10,64)
  CFI_STACKLOAD(%rdi,80)
  CFI_STACKLOAD(%rsi,88)
  CFI_STACKLOAD(%rdx,96)
  CFI_INC_RSP(104)
#endif

        CFI_RET


S2N_BN_SIZE_DIRECTIVE(mlkem_mulcache_compute_x86)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
