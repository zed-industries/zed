// Copyright (c) 2024 The mlkem-native project authors
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT

// ----------------------------------------------------------------------------
// Canonical reduction of polynomial coefficients for ML-KEM
// Input a[256] (signed 16-bit words); output a[256] (signed 16-bit words)
//
// This reduces each element of the 256-element array of 16-bit signed
// integers modulo 3329 with the result being 0 <= r < 3329, in-place.
// This is intended for use when that array represents polynomial
// coefficients for ML-KEM, but that is not relevant to its operation.
//
// extern void mlkem_reduce(int16_t a[static 256]);
//
// Standard x86-64 ABI: RDI = a
// Microsoft x64 ABI:   RCX = a
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(mlkem_reduce)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(mlkem_reduce)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(mlkem_reduce)
        .text
        .balign 4

S2N_BN_SYMBOL(mlkem_reduce):
        CFI_START
        _CET_ENDBR

#if WINDOWS_ABI
        CFI_DEC_RSP(96)
        CFI_STACKSAVEU(%xmm6,0)
        CFI_STACKSAVEU(%xmm7,16)
        CFI_STACKSAVEU(%xmm8,32)
        CFI_STACKSAVEU(%xmm9,48)
        CFI_STACKSAVEU(%xmm12,64)
        CFI_STACKSAVE(%rdi,80)
        movq    %rcx, %rdi
#endif

        // Load 3329 (0x0D01) into all elements of %ymm0:
        movl $0x0D010D01, %eax
        movd %eax, %xmm0
        vpbroadcastd %xmm0, %ymm0

        // Load 20159 (0x4EBF) into all elements of %ymm1:
        movl $0x4EBF4EBF, %eax
        movd %eax, %xmm1
        vpbroadcastd %xmm1, %ymm1

        // We process 128 coefficients (8 %ymm regs) at once.
        // Reduce the fist 128 coefficients:
        vmovdqa 0x00(%rdi), %ymm2
        vmovdqa 0x20(%rdi), %ymm3
        vmovdqa 0x40(%rdi), %ymm4
        vmovdqa 0x60(%rdi), %ymm5
        vmovdqa 0x80(%rdi), %ymm6
        vmovdqa 0xa0(%rdi), %ymm7
        vmovdqa 0xc0(%rdi), %ymm8
        vmovdqa 0xe0(%rdi), %ymm9

        vpmulhw %ymm1, %ymm2, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm2, %ymm2
        vpmulhw %ymm1, %ymm3, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm3, %ymm3
        vpmulhw %ymm1, %ymm4, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm4, %ymm4
        vpmulhw %ymm1, %ymm5, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm5, %ymm5
        vpmulhw %ymm1, %ymm6, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm6, %ymm6
        vpmulhw %ymm1, %ymm7, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm7, %ymm7
        vpmulhw %ymm1, %ymm8, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm8, %ymm8
        vpmulhw %ymm1, %ymm9, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm9, %ymm9

        vpsubw  %ymm0, %ymm2, %ymm2
        vpsraw  $0xf, %ymm2, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm2, %ymm2
        vpsubw  %ymm0, %ymm3, %ymm3
        vpsraw  $0xf, %ymm3, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm3, %ymm3
        vpsubw  %ymm0, %ymm4, %ymm4
        vpsraw  $0xf, %ymm4, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm4, %ymm4
        vpsubw  %ymm0, %ymm5, %ymm5
        vpsraw  $0xf, %ymm5, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm5, %ymm5
        vpsubw  %ymm0, %ymm6, %ymm6
        vpsraw  $0xf, %ymm6, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm6, %ymm6
        vpsubw  %ymm0, %ymm7, %ymm7
        vpsraw  $0xf, %ymm7, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm7, %ymm7
        vpsubw  %ymm0, %ymm8, %ymm8
        vpsraw  $0xf, %ymm8, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm8, %ymm8
        vpsubw  %ymm0, %ymm9, %ymm9
        vpsraw  $0xf, %ymm9, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm9, %ymm9

        vmovdqa %ymm2, 0x00(%rdi)
        vmovdqa %ymm3, 0x20(%rdi)
        vmovdqa %ymm4, 0x40(%rdi)
        vmovdqa %ymm5, 0x60(%rdi)
        vmovdqa %ymm6, 0x80(%rdi)
        vmovdqa %ymm7, 0xa0(%rdi)
        vmovdqa %ymm8, 0xc0(%rdi)
        vmovdqa %ymm9, 0xe0(%rdi)

        // Reduce the second 128 coefficients:
        vmovdqa 0x100(%rdi), %ymm2
        vmovdqa 0x120(%rdi), %ymm3
        vmovdqa 0x140(%rdi), %ymm4
        vmovdqa 0x160(%rdi), %ymm5
        vmovdqa 0x180(%rdi), %ymm6
        vmovdqa 0x1a0(%rdi), %ymm7
        vmovdqa 0x1c0(%rdi), %ymm8
        vmovdqa 0x1e0(%rdi), %ymm9

        vpmulhw %ymm1, %ymm2, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm2, %ymm2
        vpmulhw %ymm1, %ymm3, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm3, %ymm3
        vpmulhw %ymm1, %ymm4, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm4, %ymm4
        vpmulhw %ymm1, %ymm5, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm5, %ymm5
        vpmulhw %ymm1, %ymm6, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm6, %ymm6
        vpmulhw %ymm1, %ymm7, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm7, %ymm7
        vpmulhw %ymm1, %ymm8, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm8, %ymm8
        vpmulhw %ymm1, %ymm9, %ymm12
        vpsraw  $0xa, %ymm12, %ymm12
        vpmullw %ymm0, %ymm12, %ymm12
        vpsubw  %ymm12, %ymm9, %ymm9

        vpsubw  %ymm0, %ymm2, %ymm2
        vpsraw  $0xf, %ymm2, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm2, %ymm2
        vpsubw  %ymm0, %ymm3, %ymm3
        vpsraw  $0xf, %ymm3, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm3, %ymm3
        vpsubw  %ymm0, %ymm4, %ymm4
        vpsraw  $0xf, %ymm4, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm4, %ymm4
        vpsubw  %ymm0, %ymm5, %ymm5
        vpsraw  $0xf, %ymm5, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm5, %ymm5
        vpsubw  %ymm0, %ymm6, %ymm6
        vpsraw  $0xf, %ymm6, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm6, %ymm6
        vpsubw  %ymm0, %ymm7, %ymm7
        vpsraw  $0xf, %ymm7, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm7, %ymm7
        vpsubw  %ymm0, %ymm8, %ymm8
        vpsraw  $0xf, %ymm8, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm8, %ymm8
        vpsubw  %ymm0, %ymm9, %ymm9
        vpsraw  $0xf, %ymm9, %ymm12
        vpand   %ymm0, %ymm12, %ymm12
        vpaddw  %ymm12, %ymm9, %ymm9

        vmovdqa %ymm2, 0x100(%rdi)
        vmovdqa %ymm3, 0x120(%rdi)
        vmovdqa %ymm4, 0x140(%rdi)
        vmovdqa %ymm5, 0x160(%rdi)
        vmovdqa %ymm6, 0x180(%rdi)
        vmovdqa %ymm7, 0x1a0(%rdi)
        vmovdqa %ymm8, 0x1c0(%rdi)
        vmovdqa %ymm9, 0x1e0(%rdi)

#if WINDOWS_ABI
        CFI_STACKLOADU(%xmm6,0)
        CFI_STACKLOADU(%xmm7,16)
        CFI_STACKLOADU(%xmm8,32)
        CFI_STACKLOADU(%xmm9,48)
        CFI_STACKLOADU(%xmm12,64)
        CFI_STACKLOAD(%rdi,80)
        CFI_INC_RSP(96)
#endif
        CFI_RET


S2N_BN_SIZE_DIRECTIVE(mlkem_reduce)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",%progbits
#endif
