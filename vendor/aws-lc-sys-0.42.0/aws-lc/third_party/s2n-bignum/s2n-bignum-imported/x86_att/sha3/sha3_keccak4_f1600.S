// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Keccak-f1600 permutation for SHA3, batch of four independent operations
// Input a[100], rc[24]; output a[100]
//
// The input/output argument is in effect four 25-element Keccak arrays
// a[0...24], a[25..49], a[50..74] and a[75..99], which could be considered
// as type a[25][4].
//
// Keccak-f1600 permutation operation is at the core of SHA3 and SHAKE
// and is fully specified here:
//
//   https://keccak.team/files/Keccak-reference-3.0.pdf
//
//    extern void sha3_keccak4_f1600(uint64_t a[100], const uint64_t rc[24]);
//
// Standard x86-64 ABI: RDI = a, RSI = rc
// Microsoft x64 ABI:   RCX = a, RDX = rc
// ----------------------------------------------------------------------------

#include "_internal_s2n_bignum_x86_att.h"


        S2N_BN_SYM_VISIBILITY_DIRECTIVE(sha3_keccak4_f1600)
        S2N_BN_FUNCTION_TYPE_DIRECTIVE(sha3_keccak4_f1600)
        S2N_BN_SYM_PRIVACY_DIRECTIVE(sha3_keccak4_f1600)
        .text
        .balign 32

S2N_BN_SYMBOL(sha3_keccak4_f1600):
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

   movq   %rsp, %rcx
   .cfi_register %rsp, %rcx
   andq   $0xffffffffffffffe0, %rsp
   sub $0x360, %rsp

   // **** Bitstates Allocation Map on Stack **** //
   // %ymm5     	    A0	(state0[0), state1[0], state2[0], state3[0]]        Input (%rdi) offsets: 0x00, 0xC8, 0x190, 0x258
   // 0x0(%rsp)	    A1	(state0[1), state1[1], state2[1], state3[1]]        Input (%rdi) offsets: 0x08, 0xD0, 0x198, 0x260
   // 0x20(%rsp)	A2	(state0[2), state1[2], state2[2], state3[2]]        Input (%rdi) offsets: 0x10, 0xD8, 0x1A0, 0x268
   // 0x40(%rsp)	A3	(state0[3), state1[3], state2[3], state3[3]]        Input (%rdi) offsets: 0x18, 0xE0, 0x1A8, 0x270
   // 0x60(%rsp)	A4	(state0[4), state1[4], state2[4], state3[4]]        Input (%rdi) offsets: 0x20, 0xE8, 0x1B0, 0x278
   // 0x80(%rsp)	A5	(state0[5), state1[5], state2[5], state3[5]]        Input (%rdi) offsets: 0x28, 0xF0, 0x1B8, 0x280
   // 0xa0(%rsp)	A6	(state0[6), state1[6], state2[6], state3[6]]        Input (%rdi) offsets: 0x30, 0xF8, 0x1C0, 0x288
   // 0xc0(%rsp)	A7	(state0[7), state1[7], state2[7], state3[7]]        Input (%rdi) offsets: 0x38, 0x100, 0x1C8, 0x290
   // 0xe0(%rsp)	A8	(state0[8), state1[8], state2[8], state3[8]]        Input (%rdi) offsets: 0x40, 0x108, 0x1D0, 0x298
   // 0x100(%rsp)	A9	(state0[9), state1[9], state2[9], state3[9]]    	Input (%rdi) offsets: 0x48, 0x110, 0x1D8, 0x2A0
   // 0x120(%rsp)	A10	(state0[10), state1[10], state2[10], state3[10]]	Input (%rdi) offsets: 0x50, 0x118, 0x1E0, 0x2A8
   // 0x140(%rsp)	A11	(state0[11), state1[11], state2[11], state3[11]]	Input (%rdi) offsets: 0x58, 0x120, 0x1E8, 0x2B0
   // 0x160(%rsp)	A12	(state0[12), state1[12], state2[12], state3[12]]	Input (%rdi) offsets: 0x60, 0x128, 0x1F0, 0x2B8
   // 0x180(%rsp)	A13	(state0[13), state1[13], state2[13], state3[13]]	Input (%rdi) offsets: 0x68, 0x130, 0x1F8, 0x2C0
   // 0x1a0(%rsp)	A14	(state0[14), state1[14], state2[14], state3[14]]	Input (%rdi) offsets: 0x70, 0x138, 0x200, 0x2C8
   // 0x1c0(%rsp)	A15	(state0[15), state1[15], state2[15], state3[15]]	Input (%rdi) offsets: 0x78, 0x140, 0x208, 0x2D0
   // 0x1e0(%rsp)	A16	(state0[16), state1[16], state2[16], state3[16]]	Input (%rdi) offsets: 0x80, 0x148, 0x210, 0x2D8
   // 0x200(%rsp)	A17	(state0[17), state1[17], state2[17], state3[17]]	Input (%rdi) offsets: 0x88, 0x150, 0x218, 0x2E0
   // 0x220(%rsp)	A18	(state0[18), state1[18], state2[18], state3[18]]	Input (%rdi) offsets: 0x90, 0x158, 0x220, 0x2E8
   // 0x240(%rsp)	A19	(state0[19), state1[19], state2[19], state3[19]]	Input (%rdi) offsets: 0x98, 0x160, 0x228, 0x2F0
   // 0x260(%rsp)	A20	(state0[20), state1[20], state2[20], state3[20]]	Input (%rdi) offsets: 0xA0, 0x168, 0x230, 0x2F8
   // 0x280(%rsp)	A21	(state0[21), state1[21], state2[21], state3[21]]	Input (%rdi) offsets: 0xA8, 0x170, 0x238, 0x300
   // 0x2a0(%rsp)	A22	(state0[22), state1[22], state2[22], state3[22]]	Input (%rdi) offsets: 0xB0, 0x178, 0x240, 0x308
   // 0x2c0(%rsp)	A23	(state0[23), state1[23], state2[23], state3[23]]	Input (%rdi) offsets: 0xB8, 0x180, 0x248, 0x310
   // 0x2e0(%rsp)	A24	(state0[24), state1[24], state2[24], state3[24]]	Input (%rdi) offsets: 0xC0, 0x188, 0x250, 0x318

   // Load 32 bytes from each of the 4 states (A(0-3))
   vmovdqu (%rdi), %ymm5 // Load state0(0, 1, 2, 3) (32 bytes from Input (%rdi) offsets: 0x00)
   vmovdqu 0xc8(%rdi), %ymm1 // Load state1(0, 1, 2, 3) (32 bytes from Input (%rdi) offsets: 0xC8)
   vmovdqu 0x190(%rdi), %ymm2 // Load state2(0, 1, 2, 3) (32 bytes from Input (%rdi) offsets: 0x190)
   vmovdqu 0x258(%rdi), %ymm3 // Load state3(0, 1, 2, 3) (32 bytes from Input (%rdi) offsets: 0x258)

   // Interleave low and high qwords from %ymm5(state0(0, 1, 2, 3)) and %ymm1(state1[0, 1, 2, 3])
   vpunpcklqdq %ymm1, %ymm5, %ymm4 // %ymm4 = (state0[0) | state1[0] | state0[2] | state1[2]]
   vpunpckhqdq %ymm1, %ymm5, %ymm0 // %ymm0 = (state0[1) | state1[1] | state0[3] | state1[3]]

   // Interleave low and high qwords from %ymm2(state2(0, 1, 2, 3)) and %ymm3(state3[0, 1, 2, 3])
   vpunpcklqdq %ymm3, %ymm2, %ymm6 // %ymm6 = (state2[0) | state3[0] | state2[2] | state3[2]]
   vpunpckhqdq %ymm3, %ymm2, %ymm7 // %ymm7 = (state2[1) | state3[1] | state2[3] | state3[3]]

   // Permute 128-bit lanes to complete the interleave
   vperm2i128 $0x20, %ymm6, %ymm4, %ymm5 // A0 = %ymm5 = (state0[0) | state1[0] | state2[0] | state3[0]]
   vperm2i128 $0x31, %ymm6, %ymm4, %ymm2 // A2 = %ymm2 = (state0[2) | state1[2] | state2[2] | state3[2]]
   vperm2i128 $0x20, %ymm7, %ymm0, %ymm1 // A1 = %ymm1 = (state0[1) | state1[1] | state2[1] | state3[1]]
   vperm2i128 $0x31, %ymm7, %ymm0, %ymm3 // A3 = %ymm3 = (state0[3) | state1[3] | state2[3] | state3[3]]

   // Store results on the stack following the Bitstates Allocation Map
   // keep A0 -> %ymm5 (special value, not stored on stack)
   vmovdqu %ymm1, (%rsp) // store A(1) -> on stack
   vmovdqu %ymm2, 0x20(%rsp) // store A(2) -> on stack
   vmovdqu %ymm3, 0x40(%rsp) // store A(3) -> on stack

   // Load, Interleave, and Store 32 bytes from each of the 4 states (A(4-7))
   vmovdqu 0x20(%rdi), %ymm0
   vmovdqu 0xe8(%rdi), %ymm1
   vmovdqu 0x1b0(%rdi), %ymm2
   vmovdqu 0x278(%rdi), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0x60(%rsp)
   vmovdqu %ymm1, 0x80(%rsp)
   vmovdqu %ymm2, 0xa0(%rsp)
   vmovdqu %ymm3, 0xc0(%rsp)

   // Load, Interleave, and Store 32 bytes from each of the 4 states (A(8-11))
   vmovdqu 0x40(%rdi), %ymm0
   vmovdqu 0x108(%rdi), %ymm1
   vmovdqu 0x1d0(%rdi), %ymm2
   vmovdqu 0x298(%rdi), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0xe0(%rsp)
   vmovdqu %ymm1, 0x100(%rsp)
   vmovdqu %ymm2, 0x120(%rsp)
   vmovdqu %ymm3, 0x140(%rsp)

   // Load, Interleave, and Store 32 bytes from each of the 4 states (A(12-15))
   vmovdqu 0x60(%rdi), %ymm0
   vmovdqu 0x128(%rdi), %ymm1
   vmovdqu 0x1f0(%rdi), %ymm2
   vmovdqu 0x2b8(%rdi), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0x160(%rsp)
   vmovdqu %ymm1, 0x180(%rsp)
   vmovdqu %ymm2, 0x1a0(%rsp)
   vmovdqu %ymm3, 0x1c0(%rsp)

   // Load, Interleave, and Store 32 bytes from each of the 4 states (A(16-19))
   vmovdqu 0x80(%rdi), %ymm0
   vmovdqu 0x148(%rdi), %ymm1
   vmovdqu 0x210(%rdi), %ymm2
   vmovdqu 0x2d8(%rdi), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0x1e0(%rsp)
   vmovdqu %ymm1, 0x200(%rsp)
   vmovdqu %ymm2, 0x220(%rsp)
   vmovdqu %ymm3, 0x240(%rsp)

   // Load, Interleave, and Store 32 bytes from each of the 4 states (A(20-23))
   vmovdqu 0xa0(%rdi), %ymm0
   vmovdqu 0x168(%rdi), %ymm1
   vmovdqu 0x230(%rdi), %ymm2
   vmovdqu 0x2f8(%rdi), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0x260(%rsp)
   vmovdqu %ymm1, 0x280(%rsp)
   vmovdqu %ymm2, 0x2a0(%rsp)
   vmovdqu %ymm3, 0x2c0(%rsp)

   // Load, Interleave, and Store 32 bytes from each of the 4 states (A24)
   // A24 is the last element (only 8 bytes per state)
   vmovq  0xc0(%rdi), %xmm1 // Load state0(24) into lower 64 bits of %xmm1
   vpinsrq $0x1, 0x188(%rdi), %xmm1, %xmm1 // Insert state1(24) into upper 64 bits of %xmm1 = [state0[24] | state1[24]]
   vmovq  0x250(%rdi), %xmm2 // Load state2(24) into lower 64 bits of %xmm2
   vpinsrq $0x1, 0x318(%rdi), %xmm2, %xmm2 // Insert state3(24) into upper 64 bits of %xmm2 = [state2[24] | state3[24]]
   vinserti128 $0x1, %xmm2, %ymm1, %ymm3 // Interleave into %ymm3 = A24 = (state0[24) | state1[24] | state2[24] | state3[24]]
   vmovdqu %ymm3, 0x2e0(%rsp) // store A(24) -> on stack

   // Initialize the loop counter
   movq   $0x0, %rax

Lsha3_keccak4_f1600_loop:

   // Theta step
   // %ymm5                                       // state A(0,0) (A[0])
   vmovdqu 0x80(%rsp), %ymm12 // state A(0,1) (A[5])
   vmovdqu 0x120(%rsp), %ymm13 // state A(0,2) (A[10])
   vmovdqu 0x1c0(%rsp), %ymm11 // state A(0,3) (A[15])
   vmovdqu 0x260(%rsp), %ymm10 // state A(0,4) (A[20])
   vpxor  %ymm12, %ymm5, %ymm1 // A(0,0) xor A[0,1] (A[0] xor A[5])
   vpxor  %ymm13, %ymm11, %ymm0 // A(0,2) xor A[0,3] (A[10] xor A[15])
   vpxor  %ymm0, %ymm1, %ymm1 // (A(0,0) xor A[0,1]) xor (A[0,2] xor A[0,3]) ((A[0] xor A[5]) xor (A[10] xor A[15]))
   vpxor  %ymm10, %ymm1, %ymm1 // C0 = A(0,4) xor (A[0,0] xor A[0,1] xor A[0,2] xor A[0,3]) (A[20] xor (A[0] xor A[5] xor A[10] xor A[15]))

   vmovdqu (%rsp), %ymm4
   vmovdqu 0x1e0(%rsp), %ymm6
   vpxor  0xa0(%rsp), %ymm4, %ymm8
   vpxor  0x140(%rsp), %ymm6, %ymm0
   vpxor  %ymm0, %ymm8, %ymm8
   vpxor  0x280(%rsp), %ymm8, %ymm8 // C(1) = A[1,4] (A[21]) xor A[1,2] xor A[1,3] xor A[1,1] xor A[1,0] (A[21] xor A[11] xor A[16] xor A[6] xor A[1])

   vmovdqu 0x20(%rsp), %ymm0
   vpxor  0xc0(%rsp), %ymm0, %ymm7
   vmovdqu 0x200(%rsp), %ymm14
   vpxor  0x160(%rsp), %ymm14, %ymm0
   vpxor  %ymm0, %ymm7, %ymm7
   vpxor  0x2a0(%rsp), %ymm7, %ymm7 // C(2) = A[2,4] (A[22]) xor A[2,2] xor A[2,3] xor A[2,1] xor A[2,0] (A[22] xor A[12] xor A[17] xor A[7] xor A[2])

   vmovdqu 0x40(%rsp), %ymm3
   vpxor  0xe0(%rsp), %ymm3, %ymm6
   vmovdqu 0x220(%rsp), %ymm0
   vpxor  0x180(%rsp), %ymm0, %ymm0
   vpxor  %ymm0, %ymm6, %ymm6
   vpxor  0x2c0(%rsp), %ymm6, %ymm6 // C(3) = A[3,4] (A[23]) xor A[3,2] xor A[3,3] xor A[3,1] xor A[3,0] (A[23] xor A[13] xor A[18] xor A[8] xor A[3])

   vmovdqu 0x60(%rsp), %ymm4
   vpxor  0x100(%rsp), %ymm4, %ymm2
   vmovdqu 0x240(%rsp), %ymm0
   vpxor  0x1a0(%rsp), %ymm0, %ymm0
   vpxor  %ymm0, %ymm2, %ymm2
   vpxor  0x2e0(%rsp), %ymm2, %ymm2 // C(4) = A[4,4] (A[24]) xor A[4,2] xor A[4,3] xor A[4,1] xor A[4,0] (A[24] xor A[14] xor A[19] xor A[9] xor A[4])

   // C0 = %ymm1
   // C1 = %ymm8 -> on stack
   // C2 = %ymm7 -> on stack
   // C3 = %ymm6 -> on stack
   // C4 = %ymm2 -> on stack

   // ROL(C1, 1)
   vpsllq $0x1, %ymm8, %ymm4
   vpsrlq $0x3f, %ymm8, %ymm0
   vpor   %ymm0, %ymm4, %ymm4

   // ROL(C2, 1)
   vpsllq $0x1, %ymm7, %ymm3
   vpsrlq $0x3f, %ymm7, %ymm0
   vpor   %ymm0, %ymm3, %ymm3

   // ROL(C3, 1)
   vpsllq $0x1, %ymm6, %ymm0
   vpsrlq $0x3f, %ymm6, %ymm9
   vpor   %ymm9, %ymm0, %ymm0

   vpxor  %ymm2, %ymm4, %ymm4 // D0 = C(4) xor ROL(C(1), 1)
   vpxor  %ymm1, %ymm3, %ymm3 // D1 = C(0) xor ROL(C(2), 1)
   vpxor  %ymm8, %ymm0, %ymm0 // D2 = C(1) xor ROL(C(3), 1)

   // ROL(C4, 1)
   vpsllq $0x1, %ymm2, %ymm8
   vpsrlq $0x3f, %ymm2, %ymm2
   vpor   %ymm8, %ymm2, %ymm2

   vpxor  %ymm5, %ymm4, %ymm14 // A'0 = A'(0,0) = A[0,0] xor D[0]
   vpxor  0xc0(%rsp), %ymm0, %ymm9 // A'(7) = A'[2,1] = A[2,1] (A[7]) xor D[2]
   vmovdqu %ymm9, 0x320(%rsp) // store A'(2,1) (A'[7]) -> on stack
   vpxor  %ymm12, %ymm4, %ymm15 // A'5 = A'(0,1) = A[0,1] (A[5]) xor D[0]
   vpxor  (%rsp), %ymm3, %ymm12 // A'(1) = A'[1,0] = A[1,0] (A[1]) xor D[1]
   vpxor  0x140(%rsp), %ymm3, %ymm8 // A'(11) = A[1,2] (A[11]) xor D[1]
   vmovdqu %ymm14, 0x340(%rsp) // store A'(0,0) (A'[0]) -> on stack
   vpxor  %ymm13, %ymm4, %ymm14 // A'10 = A'(0,2) = A[0,2] (A[10]) xor D[0]
   vpxor  %ymm11, %ymm4, %ymm13 // A'15 = A'(0,3) = A[0,3] (A[15]) xor D[0]
   vpxor  0xa0(%rsp), %ymm3, %ymm11 // A'(6) = A'[1,1] = A[1,1] (A[6]) xor D[1]
   vpxor  %ymm10, %ymm4, %ymm4 // A'20 = A'(0,4) = A[0,4] (A[20]) xor D[0]
   vpxor  0x20(%rsp), %ymm0, %ymm10 // A'(2) = A'[2,0] = A[2,0] (A[2]) xor D[2]

   vpxor  %ymm7, %ymm2, %ymm2 // D3 = C(2) xor ROL(C(4), 1)

   // ROL(C0, 1)
   vpsllq $0x1, %ymm1, %ymm7
   vpsrlq $0x3f, %ymm1, %ymm1
   vpor   %ymm7, %ymm1, %ymm1
   vpxor  %ymm6, %ymm1, %ymm1 // D4 = C(3) xor ROL(C(0), 1)

   vpxor  0x1e0(%rsp), %ymm3, %ymm7 // A'(16) = A'[1,3] = A[1,3] (A[16]) xor D[1]
   vpxor  0x280(%rsp), %ymm3, %ymm3 // A'(21) = A'[1,4] = A[1,4] (A[21]) xor D[1]
   vpxor  0x40(%rsp), %ymm2, %ymm5 // A'(3) = A'[3,0] = A[3,0] (A[3]) xor D[3]
   vpxor  0x160(%rsp), %ymm0, %ymm9 // A'(12) = A'[2,2] = A[2,2] (A[12]) xor D[2]
   vpxor  0x200(%rsp), %ymm0, %ymm6 // A'(17) = A'[2,3] = A[2,3] (A[17]) xor D[2]
   vpxor  0x2a0(%rsp), %ymm0, %ymm0 // A'(22) =A'[2,4] = A[2,4] (A[22]) xor D[2]
   vmovdqu %ymm5, (%rsp) // store A'(3,0) (A'[3]) -> on stack
   vpxor  0xe0(%rsp), %ymm2, %ymm5 // A'(8) = A'[3,1] = A[3,1] (A[8]) xor D[3]
   vmovdqu %ymm5, 0xa0(%rsp) // store A'(3,1) (A'[8]) -> on stack
   vpxor  0x180(%rsp), %ymm2, %ymm5 // A'(13) = A'[3,2] = A[3,2] (A[13]) xor D[3]
   vmovdqu %ymm5, 0x140(%rsp) // store A'(3,2) (A'[13]) -> on stack
   vpxor  0x220(%rsp), %ymm2, %ymm5 // A'(18) = A'[3,3] = A[3,3] (A[18]) xor D[3]
   vpxor  0x2c0(%rsp), %ymm2, %ymm2 // store A'(3,3) (A'[18]) -> on stack
   vmovdqu %ymm5, 0x1e0(%rsp) // A'(23) = A'[3,4] = A[3,4] (A[23]) xor D[3]
   vpxor  0x60(%rsp), %ymm1, %ymm5 // A'(4) = A'[4,0] = A[4,0] (A[4]) xor D[4]
   vmovdqu %ymm5, 0x280(%rsp) // store A'(4,0) (A'[4]) -> on stack
   vpxor  0x100(%rsp), %ymm1, %ymm5 // A'(9) = A'[4,1] = A[4,1] (A[9]) xor D[4]
   vmovdqu %ymm5, 0x20(%rsp) // store A'(4,1) (A'[9]) -> on stack
   vpxor  0x1a0(%rsp), %ymm1, %ymm5 // A'(14) = A'[4,2] = A[4,2] (A[14]) xor D[4]
   vmovdqu %ymm5, 0x160(%rsp) // store A'(4,2) (A'[14]) -> on stack
   vpxor  0x240(%rsp), %ymm1, %ymm5 // A'(19) = A'[4,3] = A[4,3] (A[19]) xor D[4]
   vmovdqu %ymm5, 0x200(%rsp) // store A'(4,3) (A'[19]) -> on stack
   vpxor  0x2e0(%rsp), %ymm1, %ymm1 // A'(24) = A'[4,4] = A[4,4] (A[24]) xor D[4]
   vmovdqu %ymm1, 0x40(%rsp) // store A'(4,4) (A'[24]) -> on stack

   // **** State Array B(x,y) after Rho and Pi Steps - Register/Stack Allocation Map ****
   // B(0,0) (B[0])              [%rsp+0x340] (A'[0,0] unchanged, no rotation)
   // B(1,0) (B[1])              [%rsp+0x100]
   // B(2,0) (B[2])              [%rsp+0x320]
   // B(3,0) (B[3])              %ymm11
   // B(4,0) (B[4])              %ymm12
   // B(0,1) (B[5])              %ymm3
   // B(1,1) (B[6])              %ymm9
   // B(2,1) (B[7])              [%rsp+0x2c0]
   // B(3,1) (B[8])              %ymm7
   // B(4,1) (B[9])              %ymm0
   // B(0,2) (B[10])             [%rsp+0x1a0]
   // B(1,2) (B[11])             %ymm5
   // B(2,2) (B[12])             %ymm10
   // B(3,2) (B[13])             %ymm13
   // B(4,2) (B[14])             [%rsp+0x180]
   // B(0,3) (B[15])             %ymm1
   // B(1,3) (B[16])             [%rsp+0x240]
   // B(2,3) (B[17])             %ymm8
   // B(3,3) (B[18])             %ymm6
   // B(4,3) (B[19])             %ymm2
   // B(0,4) (B[20])             [%rsp+0x60]
   // B(1,4) (B[21])             %ymm4
   // B(2,4) (B[22])             %ymm14
   // B(3,4) (B[23])             [%rsp+0x2a0]
   // B(4,4) (B[24])             [%rsp+0x2e0]

   // Rho and Pi Steps
   // B(1,3) (B[16]): ROL(A'[0,1], 36)
   vpsrlq $0x1c, %ymm15, %ymm1
   vpsllq $0x24, %ymm15, %ymm15
   vpor   %ymm1, %ymm15, %ymm1
   vmovdqu %ymm1, 0x240(%rsp)

   // B(2,1) (B[7]): ROL(A'[0,2], 3)
   vpsrlq $0x3d, %ymm14, %ymm1
   vpsllq $0x3, %ymm14, %ymm14
   vpor   %ymm1, %ymm14, %ymm15
   vmovdqu %ymm15, 0x2c0(%rsp)

   // B(3,4) (B[23]): ROL(A'[0,3], 41)
   vpsrlq $0x17, %ymm13, %ymm1
   vpsllq $0x29, %ymm13, %ymm13
   vpor   %ymm1, %ymm13, %ymm13
   vmovdqu %ymm13, 0x2a0(%rsp)

   // B(4,2) (B[14]): ROL(A'[0,4], 18)
   vpsrlq $0x2e, %ymm4, %ymm1
   vpsllq $0x12, %ymm4, %ymm4
   vpor   %ymm1, %ymm4, %ymm4
   vmovdqu %ymm4, 0x180(%rsp)

   // B(0,2) (B[10]): ROL(A'[1,0], 1)
   vpsrlq $0x3f, %ymm12, %ymm1
   vpsllq $0x1, %ymm12, %ymm12
   vpor   %ymm1, %ymm12, %ymm12
   vmovdqu %ymm12, 0x1a0(%rsp)

   // B(1,0) (B[1]): ROL(A'[1,1], 44)
   vpsrlq $0x14, %ymm11, %ymm1
   vpsllq $0x2c, %ymm11, %ymm11
   vpor   %ymm1, %ymm11, %ymm11
   vmovdqu %ymm11, 0x100(%rsp)

   // B(2,3) (B[17]): ROL(A'[1,2], 10)
   vpsrlq $0x36, %ymm8, %ymm1
   vpsllq $0xa, %ymm8, %ymm8
   vpor   %ymm1, %ymm8, %ymm8

   // B(4,4) (B[24]): ROL(A'[1,4], 2)
   vpsrlq $0x13, %ymm7, %ymm1
   vpsllq $0x2d, %ymm7, %ymm7
   vpor   %ymm1, %ymm7, %ymm7

   // B(3,1) (B[8]): ROL(A'[1,3], 45)
   vpsrlq $0x3e, %ymm3, %ymm1
   vpsllq $0x2, %ymm3, %ymm3
   vpor   %ymm1, %ymm3, %ymm3
   vmovdqu %ymm3, 0x2e0(%rsp)

   // B(0,4) (B[20]): ROL(A'[2,0], 62)
   vpsrlq $0x2, %ymm10, %ymm1
   vpsllq $0x3e, %ymm10, %ymm10
   vpor   %ymm1, %ymm10, %ymm10
   vmovdqu %ymm10, 0x60(%rsp)

   // B(1,2) (B[11]): ROL(A'[2,1], 6)
   vmovdqu 0x320(%rsp), %ymm3
   vpsllq $0x6, %ymm3, %ymm5
   vpsrlq $0x3a, %ymm3, %ymm1
   vpor   %ymm1, %ymm5, %ymm5

   // B(2,0) (B[2]): ROL(A'[2,2], 43)
   vpsrlq $0x15, %ymm9, %ymm1
   vpsllq $0x2b, %ymm9, %ymm9
   vpor   %ymm1, %ymm9, %ymm9
   vmovdqu %ymm9, 0x320(%rsp)

   // B(3,3) (B[18]): ROL(A'[2,3], 15)
   vpsrlq $0x31, %ymm6, %ymm1
   vpsllq $0xf, %ymm6, %ymm6
   vpor   %ymm1, %ymm6, %ymm6
   vmovdqu (%rsp), %ymm4

   // B(0,1) (B[5]): ROL(A'[3,0], 28)
   vpsllq $0x1c, %ymm4, %ymm3
   vpsrlq $0x24, %ymm4, %ymm1
   vpor   %ymm1, %ymm3, %ymm3

   // B(2,2) (B[12]): ROL(A'[3,2], 25)
    vmovdqu 0x140(%rsp), %ymm9
   vpsllq $0x19, %ymm9, %ymm10
   vpsrlq $0x27, %ymm9, %ymm1
   vpor   %ymm1, %ymm10, %ymm10

   // B(4,1) (B[9]): ROL(A'[2,4], 61)
   vpsrlq $0x3, %ymm0, %ymm1
   vpsllq $0x3d, %ymm0, %ymm0
   vpor   %ymm1, %ymm0, %ymm0

   // B(2,4) (B[22]): ROL(A'[4,2], 39)
   vmovdqu 0x160(%rsp), %ymm13
   vpsllq $0x27, %ymm13, %ymm14
   vpsrlq $0x19, %ymm13, %ymm12
   vpor   %ymm12, %ymm14, %ymm14

   // B(1,4) (B[21]): ROL(A'[3,1], 55)
   vmovdqu 0xa0(%rsp), %ymm4
   vpsrlq $0x9, %ymm4, %ymm1
   vpsllq $0x37, %ymm4, %ymm4
   vpor   %ymm1, %ymm4, %ymm4

   // B(3,0) (B[3]): ROL(A'[3,3], 21)
   vmovdqu 0x1e0(%rsp), %ymm12
   vpsllq $0x15, %ymm12, %ymm11
   vpsrlq $0x2b, %ymm12, %ymm1
   vpor   %ymm1, %ymm11, %ymm11

   // B(4,3) (B[19]): ROL(A'[3,4], 56)
   vpsrlq $0x8, %ymm2, %ymm1
   vpsllq $0x38, %ymm2, %ymm2
   vpor   %ymm1, %ymm2, %ymm2

   // B(0,3) (B[15]): ROL(A'[4,0], 27)
   vmovdqu 0x280(%rsp), %ymm1
   vpsrlq $0x25, %ymm1, %ymm9
   vpsllq $0x1b, %ymm1, %ymm1
   vpor   %ymm9, %ymm1, %ymm1

   // B(1,1) (B[6]): ROL(A'[4,1], 20)
   vmovdqu 0x20(%rsp), %ymm9
   vpsrlq $0x2c, %ymm9, %ymm12
   vpsllq $0x14, %ymm9, %ymm9
   vpor   %ymm12, %ymm9, %ymm9

   // B(3,2) (B[13]): ROL(A'[4,3], 8)
   vmovdqu 0x200(%rsp), %ymm13
   vpsrlq $0x38, %ymm13, %ymm12
   vpsllq $0x8, %ymm13, %ymm13
   vpor   %ymm12, %ymm13, %ymm13

   // B(4,0) (B[4]): ROL(A'[4,4], 14)
   vmovdqu 0x40(%rsp), %ymm12
   vpsrlq $0x32, %ymm12, %ymm15
   vpsllq $0xe, %ymm12, %ymm12
   vpor   %ymm15, %ymm12, %ymm12

   // **** State Array B(0-24) after Rho and Pi Steps - Register/Stack Allocation Map ****
   // B0              0x340(%rsp) (A'(0) unchanged, no rotation)
   // B1              %ymm4
   // B2              %ymm14
   // B3              %ymm5
   // B4              0x60(%rsp)
   // B5              %ymm7
   // B6              %ymm1
   // B7              %ymm8
   // B8              %ymm10
   // B9              0x100(%rsp)
   // B10             %ymm2
   // B11             %ymm6
   // B12             %ymm9
   // B13             %ymm13
   // B14             0x1a0(%rsp)
   // B15             %ymm11
   // B16             0x180(%rsp)
   // B17             0x320(%rsp)
   // B18             %ymm3
   // B19             0x240(%rsp)
   // B20             %ymm0
   // B21             %ymm12
   // B22             0x2a0(%rsp)
   // B23             0x2c0(%rsp)
   // B24             0x2e0(%rsp)

   // Chi Step
   // (A''5) A''(0,1) = B[0,1] XOR ((NOT B[1,1]) AND B[2,1])
   vpandn 0x2c0(%rsp), %ymm9, %ymm15
   vpxor  %ymm3, %ymm15, %ymm15
   vmovdqu %ymm15, 0x80(%rsp)

   // (A''10) A''(0,2) = B[0,2] XOR ((NOT B[1,2]) AND B[2,2])
   vpandn %ymm10, %ymm5, %ymm15
   vpxor  0x1a0(%rsp), %ymm15, %ymm15
   vmovdqu %ymm15, 0x120(%rsp)

   // (A''15) A''(0,3) = B[0,3] XOR ((NOT B[1,3]) AND B[2,3])
   vmovdqu 0x240(%rsp), %ymm15
   vpandn %ymm8, %ymm15, %ymm15
   vpxor  %ymm1, %ymm15, %ymm15
   vmovdqu %ymm15, 0x1c0(%rsp)

   // (A''20) A''(0,4) = B[0,4] XOR ((NOT B[1,4]) AND B[2,4])
   vpandn %ymm14, %ymm4, %ymm15
   vpxor  0x60(%rsp), %ymm15, %ymm15
   vmovdqu %ymm15, 0x260(%rsp)

   // (A''1) A''(1,0) = B[1,0] XOR ((NOT B[2,0]) AND B[3,0])
   vmovdqu 0x320(%rsp), %ymm15
   vpandn %ymm11, %ymm15, %ymm15
   vpxor  0x100(%rsp), %ymm15, %ymm15
   vmovdqu %ymm15, (%rsp)

   // (A''6) A''(1,1) = B[1,1] XOR ((NOT B[2,1]) AND B[3,1])
   vmovdqu 0x2c0(%rsp), %ymm15
   vpandn %ymm7, %ymm15, %ymm15
   vpxor  %ymm9, %ymm15, %ymm15
   vmovdqu %ymm15, 0xa0(%rsp)

   // (A''11) A''(1,2) = B[1,2] XOR ((NOT B[2,2]) AND B[3,2])
   vpandn %ymm13, %ymm10, %ymm15
   vpxor  %ymm5, %ymm15, %ymm15
   vmovdqu %ymm15, 0x140(%rsp)

   // (A''16) A''(1,3) = B[1,3] XOR ((NOT B[2,3]) AND B[3,3])
   vpandn %ymm6, %ymm8, %ymm15
   vpxor  0x240(%rsp), %ymm15, %ymm15
   vmovdqu %ymm15, 0x1e0(%rsp)

   // (A''21) A''(1,4) = B[1,4] XOR ((NOT B[2,4]) AND B[3,4])
   vpandn 0x2a0(%rsp), %ymm14, %ymm15
   vpxor  %ymm4, %ymm15, %ymm15
   vmovdqu %ymm15, 0x280(%rsp)

   // (A''2) A''(2,0) = B[2,0] XOR ((NOT B[3,0]) AND B[4,0])
   vpandn %ymm12, %ymm11, %ymm15
   vpxor  0x320(%rsp), %ymm15, %ymm15
   vmovdqu %ymm15, 0x20(%rsp)

   // (A''7) A''(2,1) = B[2,1] XOR ((NOT B[3,1]) AND B[4,1])
   vpandn %ymm0, %ymm7, %ymm15
   vpxor  0x2c0(%rsp), %ymm15, %ymm15
   vmovdqu %ymm15, 0xc0(%rsp)

   // (A''12) A''(2,2) = B[2,2] XOR ((NOT B[3,2]) AND B[4,2])
   vpandn 0x180(%rsp), %ymm13, %ymm15
   vpxor  %ymm10, %ymm15, %ymm10
   vmovdqu %ymm10, 0x160(%rsp)

   // (A''17) A''(2,3) = B[2,3] XOR ((NOT B[3,3]) AND B[4,3])
   vpandn %ymm2, %ymm6, %ymm10
   vpxor  %ymm8, %ymm10, %ymm8
   vmovdqu %ymm8, 0x200(%rsp)

   // (A''22) A''(2,4) = B[2,4] XOR ((NOT B[3,4]) AND B[4,4])
   vmovdqu 0x2a0(%rsp), %ymm10
   vmovdqu 0x2e0(%rsp), %ymm15
   vpandn %ymm15, %ymm10, %ymm8
   vpxor  %ymm14, %ymm8, %ymm8
   vmovdqu %ymm8, 0x2a0(%rsp)

   // (A''3) A''(3,0) = B[3,0] XOR ((NOT B[4,0]) AND B[0,0])
   vmovdqu 0x340(%rsp), %ymm14
   vpandn %ymm14, %ymm12, %ymm8
   vpxor  %ymm11, %ymm8, %ymm8
   vmovdqu %ymm8, 0x40(%rsp)

   // (A''8) A''(3,1) = B[3,1] XOR ((NOT B[4,1]) AND B[0,1])
   vpandn %ymm3, %ymm0, %ymm8
   vpxor  %ymm7, %ymm8, %ymm7
   vmovdqu %ymm7, 0xe0(%rsp)

   // (A''13) A''(3,2) = B[3,2] XOR ((NOT B[4,2]) AND B[0,2])
   vmovdqu 0x180(%rsp), %ymm11
   vmovdqu 0x1a0(%rsp), %ymm8
   vpandn %ymm8, %ymm11, %ymm7
   vpxor  %ymm13, %ymm7, %ymm7
   vmovdqu %ymm7, 0x180(%rsp)

   // (A''18) A''(3,3) = B[3,3] XOR ((NOT B[4,3]) AND B[0,3])
   vmovdqu 0x100(%rsp), %ymm13
   vpandn %ymm9, %ymm3, %ymm3
   vpxor  %ymm0, %ymm3, %ymm3
   vmovdqu %ymm3, 0x100(%rsp)

   // (A''23) A''(3,4) = B[3,4] XOR ((NOT B[4,4]) AND B[0,4])
   vpandn %ymm5, %ymm8, %ymm0
   vpxor  %ymm11, %ymm0, %ymm3
   vmovdqu %ymm3, 0x1a0(%rsp)

   // (A''4) A''(4,0) = B[4,0] XOR ((NOT B[0,0]) AND B[1,0])
   vpandn %ymm1, %ymm2, %ymm7
   vpxor  %ymm6, %ymm7, %ymm6
   vmovdqu %ymm6, 0x220(%rsp)

   // (A''9) A''(4,1) = B[4,1] XOR ((NOT B[0,1]) AND B[1,1])
   vmovdqu %ymm15, %ymm6
   vmovdqu 0x60(%rsp), %ymm15
   vpandn %ymm15, %ymm6, %ymm6
   vpxor  %ymm10, %ymm6, %ymm7
   vmovdqu %ymm7, 0x2c0(%rsp)

   // (A''14) A''(4,2) = B[4,2] XOR ((NOT B[0,2]) AND B[1,2])
   vpandn 0x240(%rsp), %ymm1, %ymm1
   vpxor  %ymm2, %ymm1, %ymm2
   vmovdqu %ymm2, 0x240(%rsp)

   // (A''19) A''(4,3) = B[4,3] XOR ((NOT B[0,3]) AND B[1,3])
   vpandn 0x320(%rsp), %ymm13, %ymm5
   vpxor  %ymm14, %ymm5, %ymm5
   vpbroadcastq (%rsi), %ymm0
   vpxor  %ymm0, %ymm5, %ymm5

   // (A''24) A''(4,4) = B[4,4] XOR ((NOT B[0,4]) AND B[1,4])
   vpandn %ymm13, %ymm14, %ymm6
   vpxor  %ymm12, %ymm6, %ymm14
   vmovdqu %ymm14, 0x60(%rsp)
   vpandn %ymm4, %ymm15, %ymm0
   vpxor  0x2e0(%rsp), %ymm0, %ymm6
   vmovdqu %ymm6, 0x2e0(%rsp)

   addq   $0x8, %rsi
   addq   $0x1, %rax
   cmpq   $0x18, %rax
   jne    Lsha3_keccak4_f1600_loop

   // Load, De-interleave, and Store 32 bytes to each of the 4 states (A(0-3))
   // %ymm5 = A0 = (state0[0) | state1[0] | state2[0] | state3[0]] (already in register)
   vmovdqu (%rsp), %ymm1 // Load A(1) from stack = [state0[1] | state1[1] | state2[1] | state3[1]]
   vmovdqu 0x20(%rsp), %ymm2 // Load A(2) from stack = [state0[2] | state1[2] | state2[2] | state3[2]]
   vmovdqu 0x40(%rsp), %ymm3 // Load A(3) from stack = [state0[3] | state1[3] | state2[3] | state3[3]]

   // De-interleave %ymm5(A0) and %ymm1(A(1))
   vpunpcklqdq %ymm1, %ymm5, %ymm4 // %ymm4 = (state0[0) | state0[1] | state2[0] | state2[1]]
   vpunpckhqdq %ymm1, %ymm5, %ymm6 // %ymm6 = (state1[0) | state1[1] | state3[0] | state3[1]]
   // De-interleave %ymm2(A2) and %ymm3(A(3))
   vpunpcklqdq %ymm3, %ymm2, %ymm7 // %ymm7 = (state0[2) | state0[3] | state2[2] | state2[3]]
   vpunpckhqdq %ymm3, %ymm2, %ymm8 // %ymm8 = (state1[2) | state1[3] | state3[2] | state3[3]]

   // Permute 128-bit lanes to complete the de-interleave
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm5 // %ymm5 = (state0[0) | state0[1] | state0[2] | state0[3]]
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1 // %ymm1 = (state1[0) | state1[1] | state1[2] | state1[3]]
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2 // %ymm2 = (state2[0) | state2[1] | state2[2] | state2[3]]
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3 // %ymm3 = (state3[0) | state3[1] | state3[2] | state3[3]]

   // Store de-interleaved results back to output
   vmovdqu %ymm5, (%rdi) // Store state0(0, 1, 2, 3) (32 bytes to Output (%rdi) offsets: 0x00)
   vmovdqu %ymm1, 0xc8(%rdi) // Store state1(0, 1, 2, 3) (32 bytes to Output (%rdi) offsets: 0xC8)
   vmovdqu %ymm2, 0x190(%rdi) // Store state2(0, 1, 2, 3) (32 bytes to Output (%rdi) offsets: 0x190)
   vmovdqu %ymm3, 0x258(%rdi) // Store state3(0, 1, 2, 3) (32 bytes to Output (%rdi) offsets: 0x258)

   // Load, De-interleave, and Store 32 bytes to each of the 4 states (A(4-7))
   vmovdqu 0x60(%rsp), %ymm0
   vmovdqu 0x80(%rsp), %ymm1
   vmovdqu 0xa0(%rsp), %ymm2
   vmovdqu 0xc0(%rsp), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0x20(%rdi)
   vmovdqu %ymm1, 0xe8(%rdi)
   vmovdqu %ymm2, 0x1b0(%rdi)
   vmovdqu %ymm3, 0x278(%rdi)

   // Load, De-interleave, and Store 32 bytes to each of the 4 states (A(8-11))
   vmovdqu 0xe0(%rsp), %ymm0
   vmovdqu 0x100(%rsp), %ymm1
   vmovdqu 0x120(%rsp), %ymm2
   vmovdqu 0x140(%rsp), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0x40(%rdi)
   vmovdqu %ymm1, 0x108(%rdi)
   vmovdqu %ymm2, 0x1d0(%rdi)
   vmovdqu %ymm3, 0x298(%rdi)

   // Load, De-interleave, and Store 32 bytes to each of the 4 states (A(12-15))
   vmovdqu 0x160(%rsp), %ymm0
   vmovdqu 0x180(%rsp), %ymm1
   vmovdqu 0x1a0(%rsp), %ymm2
   vmovdqu 0x1c0(%rsp), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0x60(%rdi)
   vmovdqu %ymm1, 0x128(%rdi)
   vmovdqu %ymm2, 0x1f0(%rdi)
   vmovdqu %ymm3, 0x2b8(%rdi)

   // Load, De-interleave, and Store 32 bytes to each of the 4 states (A(16-19))
   vmovdqu 0x1e0(%rsp), %ymm0
   vmovdqu 0x200(%rsp), %ymm1
   vmovdqu 0x220(%rsp), %ymm2
   vmovdqu 0x240(%rsp), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0x80(%rdi)
   vmovdqu %ymm1, 0x148(%rdi)
   vmovdqu %ymm2, 0x210(%rdi)
   vmovdqu %ymm3, 0x2d8(%rdi)

   // Load, De-interleave, and Store 32 bytes to each of the 4 states (A(20-23))
   vmovdqu 0x260(%rsp), %ymm0
   vmovdqu 0x280(%rsp), %ymm1
   vmovdqu 0x2a0(%rsp), %ymm2
   vmovdqu 0x2c0(%rsp), %ymm3
   vpunpcklqdq %ymm1, %ymm0, %ymm4
   vpunpckhqdq %ymm1, %ymm0, %ymm6
   vpunpcklqdq %ymm3, %ymm2, %ymm7
   vpunpckhqdq %ymm3, %ymm2, %ymm8
   vperm2i128 $0x20, %ymm7, %ymm4, %ymm0
   vperm2i128 $0x20, %ymm8, %ymm6, %ymm1
   vperm2i128 $0x31, %ymm7, %ymm4, %ymm2
   vperm2i128 $0x31, %ymm8, %ymm6, %ymm3
   vmovdqu %ymm0, 0xa0(%rdi)
   vmovdqu %ymm1, 0x168(%rdi)
   vmovdqu %ymm2, 0x230(%rdi)
   vmovdqu %ymm3, 0x2f8(%rdi)

   // Load, De-interleave, and Store 8 bytes to each of the 4 states (A24)
   // A24 is the last element (only 8 bytes per state)
   vmovdqu 0x2e0(%rsp), %ymm3
   vextracti128 $0x0, %ymm3, %xmm0
   vextracti128 $0x1, %ymm3, %xmm1
   vmovq %xmm0, 0xc0(%rdi)
   vpextrq $0x1, %xmm0, 0x188(%rdi)
   vmovq %xmm1, 0x250(%rdi)
   vpextrq $0x1, %xmm1, 0x318(%rdi)

   movq   %rcx, %rsp
   .cfi_same_value %rcx
   .cfi_same_value %rsp

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

S2N_BN_SIZE_DIRECTIVE(sha3_keccak4_f1600)

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,  "",  %progbits
#endif

