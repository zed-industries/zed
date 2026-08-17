// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Canonical reduction of polynomial coefficients for ML-DSA
// Input a[256] (signed 32-bit words); output a[256] (signed 32-bit words)
//
// This reduces each element of the 256-element array of 32-bit signed
// integers modulo 8380417 with the result being centered around zero,
// specifically -6283009 <= r <= 6283008, in-place.
//
// extern void mldsa_reduce(int32_t a[static 256]);
//
// Standard x86-64 ABI: RDI = a
// Microsoft x64 ABI:   RCX = a
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mldsa_reduce)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mldsa_reduce)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mldsa_reduce)
        .text

#define a %rdi

S2N_BN_SYMBOL(mldsa_reduce):
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
        movq    %rcx, %rdi
#endif

// Load constants - poly_reduce only takes one parameter (the polynomial)
        movl    $8380417, %eax // MLDSA_Q
        movd    %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0 // q in all lanes

        // Load offset constant inline to avoid PIE issues
        movl    $4194304, %eax // 1 << 22
        movd    %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1 // off = _mm256_set1_epi32(1<<22)

// Process all 32 vectors (256 coefficients) using NTT-style block processing
// First batch: vectors 0-7
        vmovdqa 0(%rdi), %ymm4 // Load f0
        vmovdqa 32(%rdi), %ymm5 // Load f1
        vmovdqa 64(%rdi), %ymm6 // Load f2
        vmovdqa 96(%rdi), %ymm7 // Load f3
        vmovdqa 128(%rdi), %ymm8 // Load f4
        vmovdqa 160(%rdi), %ymm9 // Load f5
        vmovdqa 192(%rdi), %ymm10 // Load f6
        vmovdqa 224(%rdi), %ymm11 // Load f7

// Process all 8 vectors in parallel - first half of pipeline
        vpaddd  %ymm1, %ymm4, %ymm12 // g0 = f0 + off
        vpaddd  %ymm1, %ymm5, %ymm13 // g1 = f1 + off
        vpaddd  %ymm1, %ymm6, %ymm14 // g2 = f2 + off
        vpaddd  %ymm1, %ymm7, %ymm15 // g3 = f3 + off
        vpsrad  $23, %ymm12, %ymm12 // g0 >>= 23
        vpsrad  $23, %ymm13, %ymm13 // g1 >>= 23
        vpsrad  $23, %ymm14, %ymm14 // g2 >>= 23
        vpsrad  $23, %ymm15, %ymm15 // g3 >>= 23
        vpmulld %ymm0, %ymm12, %ymm12 // g0 *= q
        vpmulld %ymm0, %ymm13, %ymm13 // g1 *= q
        vpmulld %ymm0, %ymm14, %ymm14 // g2 *= q
        vpmulld %ymm0, %ymm15, %ymm15 // g3 *= q
        vpsubd  %ymm12, %ymm4, %ymm4 // f0 -= g0
        vpsubd  %ymm13, %ymm5, %ymm5 // f1 -= g1
        vpsubd  %ymm14, %ymm6, %ymm6 // f2 -= g2
        vpsubd  %ymm15, %ymm7, %ymm7 // f3 -= g3

// Process second half of vectors
        vpaddd  %ymm1, %ymm8, %ymm12 // g4 = f4 + off
        vpaddd  %ymm1, %ymm9, %ymm13 // g5 = f5 + off
        vpaddd  %ymm1, %ymm10, %ymm14 // g6 = f6 + off
        vpaddd  %ymm1, %ymm11, %ymm15 // g7 = f7 + off
        vpsrad  $23, %ymm12, %ymm12 // g4 >>= 23
        vpsrad  $23, %ymm13, %ymm13 // g5 >>= 23
        vpsrad  $23, %ymm14, %ymm14 // g6 >>= 23
        vpsrad  $23, %ymm15, %ymm15 // g7 >>= 23
        vpmulld %ymm0, %ymm12, %ymm12 // g4 *= q
        vpmulld %ymm0, %ymm13, %ymm13 // g5 *= q
        vpmulld %ymm0, %ymm14, %ymm14 // g6 *= q
        vpmulld %ymm0, %ymm15, %ymm15 // g7 *= q
        vpsubd  %ymm12, %ymm8, %ymm8 // f4 -= g4
        vpsubd  %ymm13, %ymm9, %ymm9 // f5 -= g5
        vpsubd  %ymm14, %ymm10, %ymm10 // f6 -= g6
        vpsubd  %ymm15, %ymm11, %ymm11 // f7 -= g7

// Store first batch results
        vmovdqa   %ymm4, 0(%rdi) // Store f0
        vmovdqa  %ymm5, 32(%rdi) // Store f1
        vmovdqa  %ymm6, 64(%rdi) // Store f2
        vmovdqa  %ymm7, 96(%rdi) // Store f3
        vmovdqa %ymm8, 128(%rdi) // Store f4
        vmovdqa %ymm9, 160(%rdi) // Store f5
        vmovdqa %ymm10, 192(%rdi) // Store f6
        vmovdqa %ymm11, 224(%rdi) // Store f7

// Second batch: vectors 8-15
        vmovdqa 256(%rdi), %ymm4 // Load f8
        vmovdqa 288(%rdi), %ymm5 // Load f9
        vmovdqa 320(%rdi), %ymm6 // Load f10
        vmovdqa 352(%rdi), %ymm7 // Load f11
        vmovdqa 384(%rdi), %ymm8 // Load f12
        vmovdqa 416(%rdi), %ymm9 // Load f13
        vmovdqa 448(%rdi), %ymm10 // Load f14
        vmovdqa 480(%rdi), %ymm11 // Load f15

// Process second batch in parallel
        vpaddd  %ymm1, %ymm4, %ymm12 // g8 = f8 + off
        vpaddd  %ymm1, %ymm5, %ymm13 // g9 = f9 + off
        vpaddd  %ymm1, %ymm6, %ymm14 // g10 = f10 + off
        vpaddd  %ymm1, %ymm7, %ymm15 // g11 = f11 + off
        vpsrad  $23, %ymm12, %ymm12 // g8 >>= 23
        vpsrad  $23, %ymm13, %ymm13 // g9 >>= 23
        vpsrad  $23, %ymm14, %ymm14 // g10 >>= 23
        vpsrad  $23, %ymm15, %ymm15 // g11 >>= 23
        vpmulld %ymm0, %ymm12, %ymm12 // g8 *= q
        vpmulld %ymm0, %ymm13, %ymm13 // g9 *= q
        vpmulld %ymm0, %ymm14, %ymm14 // g10 *= q
        vpmulld %ymm0, %ymm15, %ymm15 // g11 *= q
        vpsubd  %ymm12, %ymm4, %ymm4 // f8 -= g8
        vpsubd  %ymm13, %ymm5, %ymm5 // f9 -= g9
        vpsubd  %ymm14, %ymm6, %ymm6 // f10 -= g10
        vpsubd  %ymm15, %ymm7, %ymm7 // f11 -= g11

        vpaddd  %ymm1, %ymm8, %ymm12 // g12 = f12 + off
        vpaddd  %ymm1, %ymm9, %ymm13 // g13 = f13 + off
        vpaddd  %ymm1, %ymm10, %ymm14 // g14 = f14 + off
        vpaddd  %ymm1, %ymm11, %ymm15 // g15 = f15 + off
        vpsrad  $23, %ymm12, %ymm12 // g12 >>= 23
        vpsrad  $23, %ymm13, %ymm13 // g13 >>= 23
        vpsrad  $23, %ymm14, %ymm14 // g14 >>= 23
        vpsrad  $23, %ymm15, %ymm15 // g15 >>= 23
        vpmulld %ymm0, %ymm12, %ymm12 // g12 *= q
        vpmulld %ymm0, %ymm13, %ymm13 // g13 *= q
        vpmulld %ymm0, %ymm14, %ymm14 // g14 *= q
        vpmulld %ymm0, %ymm15, %ymm15 // g15 *= q
        vpsubd  %ymm12, %ymm8, %ymm8 // f12 -= g12
        vpsubd  %ymm13, %ymm9, %ymm9 // f13 -= g13
        vpsubd  %ymm14, %ymm10, %ymm10 // f14 -= g14
        vpsubd  %ymm15, %ymm11, %ymm11 // f15 -= g15

// Store second batch
        vmovdqa %ymm4, 256(%rdi) // Store f8
        vmovdqa %ymm5, 288(%rdi) // Store f9
        vmovdqa %ymm6, 320(%rdi) // Store f10
        vmovdqa %ymm7, 352(%rdi) // Store f11
        vmovdqa %ymm8, 384(%rdi) // Store f12
        vmovdqa %ymm9, 416(%rdi) // Store f13
        vmovdqa %ymm10, 448(%rdi) // Store f14
        vmovdqa %ymm11, 480(%rdi) // Store f15

// Third batch: vectors 16-23
        vmovdqa 512(%rdi), %ymm4 // Load f16
        vmovdqa 544(%rdi), %ymm5 // Load f17
        vmovdqa 576(%rdi), %ymm6 // Load f18
        vmovdqa 608(%rdi), %ymm7 // Load f19
        vmovdqa 640(%rdi), %ymm8 // Load f20
        vmovdqa 672(%rdi), %ymm9 // Load f21
        vmovdqa 704(%rdi), %ymm10 // Load f22
        vmovdqa 736(%rdi), %ymm11 // Load f23

// Process third batch
        vpaddd  %ymm1, %ymm4, %ymm12 // g16 = f16 + off
        vpaddd  %ymm1, %ymm5, %ymm13 // g17 = f17 + off
        vpaddd  %ymm1, %ymm6, %ymm14 // g18 = f18 + off
        vpaddd  %ymm1, %ymm7, %ymm15 // g19 = f19 + off
        vpsrad  $23, %ymm12, %ymm12 // g16 >>= 23
        vpsrad  $23, %ymm13, %ymm13 // g17 >>= 23
        vpsrad  $23, %ymm14, %ymm14 // g18 >>= 23
        vpsrad  $23, %ymm15, %ymm15 // g19 >>= 23
        vpmulld %ymm0, %ymm12, %ymm12 // g16 *= q
        vpmulld %ymm0, %ymm13, %ymm13 // g17 *= q
        vpmulld %ymm0, %ymm14, %ymm14 // g18 *= q
        vpmulld %ymm0, %ymm15, %ymm15 // g19 *= q
        vpsubd  %ymm12, %ymm4, %ymm4 // f16 -= g16
        vpsubd  %ymm13, %ymm5, %ymm5 // f17 -= g17
        vpsubd  %ymm14, %ymm6, %ymm6 // f18 -= g18
        vpsubd  %ymm15, %ymm7, %ymm7 // f19 -= g19

        vpaddd  %ymm1, %ymm8, %ymm12 // g20 = f20 + off
        vpaddd  %ymm1, %ymm9, %ymm13 // g21 = f21 + off
        vpaddd  %ymm1, %ymm10, %ymm14 // g22 = f22 + off
        vpaddd  %ymm1, %ymm11, %ymm15 // g23 = f23 + off
        vpsrad  $23, %ymm12, %ymm12 // g20 >>= 23
        vpsrad  $23, %ymm13, %ymm13 // g21 >>= 23
        vpsrad  $23, %ymm14, %ymm14 // g22 >>= 23
        vpsrad  $23, %ymm15, %ymm15 // g23 >>= 23
        vpmulld %ymm0, %ymm12, %ymm12 // g20 *= q
        vpmulld %ymm0, %ymm13, %ymm13 // g21 *= q
        vpmulld %ymm0, %ymm14, %ymm14 // g22 *= q
        vpmulld %ymm0, %ymm15, %ymm15 // g23 *= q
        vpsubd  %ymm12, %ymm8, %ymm8 // f20 -= g20
        vpsubd  %ymm13, %ymm9, %ymm9 // f21 -= g21
        vpsubd  %ymm14, %ymm10, %ymm10 // f22 -= g22
        vpsubd  %ymm15, %ymm11, %ymm11 // f23 -= g23

// Store third batch
        vmovdqa %ymm4, 512(%rdi) // Store f16
        vmovdqa %ymm5, 544(%rdi) // Store f17
        vmovdqa %ymm6, 576(%rdi) // Store f18
        vmovdqa %ymm7, 608(%rdi) // Store f19
        vmovdqa %ymm8, 640(%rdi) // Store f20
        vmovdqa %ymm9, 672(%rdi) // Store f21
        vmovdqa %ymm10, 704(%rdi) // Store f22
        vmovdqa %ymm11, 736(%rdi) // Store f23

// Fourth batch: vectors 24-31
        vmovdqa 768(%rdi), %ymm4 // Load f24
        vmovdqa 800(%rdi), %ymm5 // Load f25
        vmovdqa 832(%rdi), %ymm6 // Load f26
        vmovdqa 864(%rdi), %ymm7 // Load f27
        vmovdqa 896(%rdi), %ymm8 // Load f28
        vmovdqa 928(%rdi), %ymm9 // Load f29
        vmovdqa 960(%rdi), %ymm10 // Load f30
        vmovdqa 992(%rdi), %ymm11 // Load f31

// Process final batch
        vpaddd  %ymm1, %ymm4, %ymm12 // g24 = f24 + off
        vpaddd  %ymm1, %ymm5, %ymm13 // g25 = f25 + off
        vpaddd  %ymm1, %ymm6, %ymm14 // g26 = f26 + off
        vpaddd  %ymm1, %ymm7, %ymm15 // g27 = f27 + off
        vpsrad  $23, %ymm12, %ymm12 // g24 >>= 23
        vpsrad  $23, %ymm13, %ymm13 // g25 >>= 23
        vpsrad  $23, %ymm14, %ymm14 // g26 >>= 23
        vpsrad  $23, %ymm15, %ymm15 // g27 >>= 23
        vpmulld %ymm0, %ymm12, %ymm12 // g24 *= q
        vpmulld %ymm0, %ymm13, %ymm13 // g25 *= q
        vpmulld %ymm0, %ymm14, %ymm14 // g26 *= q
        vpmulld %ymm0, %ymm15, %ymm15 // g27 *= q
        vpsubd  %ymm12, %ymm4, %ymm4 // f24 -= g24
        vpsubd  %ymm13, %ymm5, %ymm5 // f25 -= g25
        vpsubd  %ymm14, %ymm6, %ymm6 // f26 -= g26
        vpsubd  %ymm15, %ymm7, %ymm7 // f27 -= g27

        vpaddd  %ymm1, %ymm8, %ymm12 // g28 = f28 + off
        vpaddd  %ymm1, %ymm9, %ymm13 // g29 = f29 + off
        vpaddd  %ymm1, %ymm10, %ymm14 // g30 = f30 + off
        vpaddd  %ymm1, %ymm11, %ymm15 // g31 = f31 + off
        vpsrad  $23, %ymm12, %ymm12 // g28 >>= 23
        vpsrad  $23, %ymm13, %ymm13 // g29 >>= 23
        vpsrad  $23, %ymm14, %ymm14 // g30 >>= 23
        vpsrad  $23, %ymm15, %ymm15 // g31 >>= 23
        vpmulld %ymm0, %ymm12, %ymm12 // g28 *= q
        vpmulld %ymm0, %ymm13, %ymm13 // g29 *= q
        vpmulld %ymm0, %ymm14, %ymm14 // g30 *= q
        vpmulld %ymm0, %ymm15, %ymm15 // g31 *= q
        vpsubd  %ymm12, %ymm8, %ymm8 // f28 -= g28
        vpsubd  %ymm13, %ymm9, %ymm9 // f29 -= g29
        vpsubd  %ymm14, %ymm10, %ymm10 // f30 -= g30
        vpsubd  %ymm15, %ymm11, %ymm11 // f31 -= g31

// Store final batch
        vmovdqa %ymm4, 768(%rdi) // Store f24
        vmovdqa %ymm5, 800(%rdi) // Store f25
        vmovdqa %ymm6, 832(%rdi) // Store f26
        vmovdqa %ymm7, 864(%rdi) // Store f27
        vmovdqa %ymm8, 896(%rdi) // Store f28
        vmovdqa %ymm9, 928(%rdi) // Store f29
        vmovdqa %ymm10, 960(%rdi) // Store f30
        vmovdqa %ymm11, 992(%rdi) // Store f31

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
        CFI_INC_RSP(176)
#endif
        CFI_RET

S2N_BN_SIZE_DIRECTIVE(mldsa_reduce)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
