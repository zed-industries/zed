// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// ----------------------------------------------------------------------------
// Extend Montgomery reduce in 8-digit blocks, uses an extra storage to
// temporarily cache multiplied differences appearing in ADK.
// Results are stored in input-output buffer (z).
// Inputs z[2*k], m[k], w;
// Outputs function return (extra result bit) and z[2*k]
// Temporary buffer m_precalc[12*(k/4-1)]
//
//    extern uint64_t bignum_emontredc_8n_cdiff
//     (uint64_t k, uint64_t *z, uint64_t *m, uint64_t w, uint64_t *m_precalc);
//
// Standard ARM ABI: X0 = k, X1 = z, X2 = m, X3 = w, X4 = m_precalc
//                   returns X0
//
// This is an unoptimized version of bignum_emontredc_8n_cdiff.
// ----------------------------------------------------------------------------
#include "_internal_s2n_bignum_arm.h"

                                        S2N_BN_SYM_VISIBILITY_DIRECTIVE(bignum_emontredc_8n_cdiff_base)
                                        S2N_BN_SYM_PRIVACY_DIRECTIVE(bignum_emontredc_8n_cdiff_base)
                                        .text
                                        .balign 4

        // Silly SLOTHY limitation: It needs the loop counter to have the name 'count'
        count .req x27 // inner loop counter

        // Semantically transparent instruction wrapper which is used by SLOTHY
        // for dependency tracking through memory. SLOTHY itself has no notion of
        // memory, only registers; to still track static dependencies through memory
        // (register spills), a 'hint' register type is introduced (syntax t{i}, t{i}{j})
        // that's written to in store instructions and read from in the corresponding
        // load instruction.
        //
        // The 'slothy:no-unfold' annotation prevents SLOTHY from opening the macro,
        // and instead makes it treat `stph` as an instruction specified in the
        // Arch and uArch models provided to it.
        .macro stph a, b, addr, imm, hint // slothy:no-unfold
            stp \a\(), \b\(), [\addr, \imm]
        .endm

        .macro ldph a, b, addr, imm, hint // slothy:no-unfold
            ldp \a\(), \b\(), [\addr, \imm]
        .endm

        .macro ldrh a, addr, imm, hint // slothy:no-unfold
            ldr \a\(), [\addr, \imm]
        .endm

        // Helper macro for the pre-computations
        .macro cdiff t, c, x, y
          subs    \t, \x, \y
          cneg    \t, \t, cc
          csetm   \c, cc
        .endm

        // Some immediate offsets for cached differences+carry used
        // in the inner ADK multiplications
        #define cache_a01 (32+0*16)
        #define cache_a02 (32+1*16)
        #define cache_a03 (32+2*16)
        #define cache_a12 (32+3*16)
        #define cache_a13 (32+4*16)
        #define cache_a23 (32+5*16)
        #define cache_m10 (0*16)
        #define cache_m20 (1*16)
        #define cache_m30 (2*16)
        #define cache_m21 (3*16)
        #define cache_m31 (4*16)
        #define cache_m32 (5*16)

        a0 .req x4
        a1 .req x5
        a2 .req x6
        a3 .req x7

        vpre00 .req v30
        vpre01 .req v28
        vpre02 .req v17
        vpre10 .req v18
        vpre11 .req v19
        vpre12 .req v20

        // Computes two 64x64->128-bit multiplication a*x and a*y
        // v_in0: 128-bit input vector viewed as pair (x,y) of 64-bit numbers
        // x_in: 64-bit common multiplicand a
        // v_out0: 128-bit output vector to hold a*x
        // v_out1: 128-bit output vector to hold a*y
        //
        // Uses temporaries as indicated in the following defines:
        #define v_in0_p   v3
        #define v_in0_pp  v5
        #define v_in0_ppp v28
        #define v_in1 v0
        #define vtmp0 v4
        #define vtmp1 v6
        #define vtmp2 v7
        #define vtmp3 v16
        #define vtmp4 v2

        .macro vmul_2x_64_64_128 v_in0, x_in, v_out0, v_out1 // slothy:no-unfold
          dup     v_in1.2d, \x_in
          uzp2    v_in0_p.4s, \v_in0\().4s, \v_in0\().4s
          xtn     vtmp0.2s, v_in1.2d
          xtn     v_in0_pp.2s, \v_in0\().2d
          rev64   v_in0_ppp.4s, \v_in0\().4s
          umull   vtmp1.2d, vtmp0.2s, v_in0_pp.2s
          umull   vtmp2.2d, vtmp0.2s, v_in0_p.2s
          uzp2    vtmp3.4s, v_in1.4s, v_in1.4s
          mul     v_in1.4s, v_in0_ppp.4s, v_in1.4s
          usra    vtmp2.2d, vtmp1.2d, #32
          umull   \v_out1\().2d, vtmp3.2s, v_in0_p.2s
          uaddlp  v_in1.2d, v_in1.4s
          and     vtmp4.16b, vtmp2.16b, v29.16b
          umlal   vtmp4.2d, vtmp3.2s, v_in0_pp.2s
          shl     \v_out0\().2d, v_in1.2d, #32
          usra    \v_out1\().2d, vtmp2.2d, #32
          umlal   \v_out0\().2d, vtmp0.2s, v_in0_pp.2s
          usra    \v_out1\().2d, vtmp4.2d, #32
        .endm

        // SLOTHY version of the above multiplication macro, using symbolic
        // registers instead of hardcoded registers. This is only used during
        // SLOTHY optimization (the above macro is ignored because of
        // 'slothy:no-unfold').
#if defined(SLOTHY)
        .macro vmul_2x_64_64_128 v_in0, x_in, v_out0, v_out1
          dup     V<in1>.2d, \x_in
          uzp2    V<in0_p>.4s, \v_in0\().4s, \v_in0\().4s
          xtn     V<tmp0>.2s, V<in1>.2d
          xtn     V<in0_pp>.2s, \v_in0\().2d
          rev64   V<in0_ppp>.4s, \v_in0\().4s
          umull   V<tmp1>.2d, V<tmp0>.2s, V<in0_pp>.2s
          umull   V<tmp2>.2d, V<tmp0>.2s, V<in0_p>.2s
          uzp2    V<tmp3>.4s, V<in1>.4s, V<in1>.4s
          mul     V<in1>.4s, V<in0_ppp>.4s, V<in1>.4s
          usra    V<tmp2>.2d, V<tmp1>.2d, #32
          umull   \v_out1\().2d, V<tmp3>.2s, V<in0_p>.2s
          uaddlp  V<in1>.2d, V<in1>.4s
          and     V<tmp4>.16b, V<tmp2>.16b, v29.16b
          umlal   V<tmp4>.2d, V<tmp3>.2s, V<in0_pp>.2s
          shl     \v_out0\().2d, V<in1>.2d, #32
          usra    \v_out1\().2d, V<tmp2>.2d, #32
          umlal   \v_out0\().2d, V<tmp0>.2s, V<in0_pp>.2s
          usra    \v_out1\().2d, V<tmp4>.2d, #32
        .endm
#endif

S2N_BN_SYMBOL(bignum_emontredc_8n_cdiff_base):

           sub sp, sp, #(6*16)
           stp x19, x20, [sp, #(5*16)]
           stp x21, x22, [sp, #(4*16)]
           stp x23, x24, [sp, #(3*16)]
           stp x25, x26, [sp, #(2*16)]
           stp x27, x28, [sp, #(1*16)]
           stp x29, x30, [sp, #(0*16)]

           // Leave space for cached differences in inner loop
           sub sp, sp, #(6*16)

           sub sp, sp, #32
           lsr x0, x0, #2
           mov x26, x0
           subs x12, x0, #1
           bcc bignum_emontredc_8n_cdiff_base_end

           // x30 = buffer holding precomputed ADK carry-differences for modulus
           mov w30, #(12*8)
           mul w30, w12, w30
           sub x30, sp, x30

           //
           // Start of precomputation
           //
           // Precompute and cache signed differences of modulus components
           // used in the ADK multiplication in the inner loop.
           //
           // THIS SHOULD BE HOISTED OUT
           // (and until then, comment out for benchmarking to get accurate estimates)
           //

           // Number of extra limbs required:
           // 6 * (number of limbs / 4 - 1) * 2 = 12 * (number_of_limbs/4 - 1)
           //
           // For now, just put them on the stack
           mov sp, x30

           // Save modulus pointer
           mov x25, x2

           mov count, x12
bignum_emontredc_8n_cdiff_base_precomp:
           ldp a0, a1, [x2, #32]!
           ldp a2, a3, [x2, #16]

           t .req x28
           c .req x29

           cdiff t, c, a1, a0
           stp   t, c, [sp, #cache_m10]
           cdiff t, c, a2, a0
           stp   t, c, [sp, #cache_m20]
           cdiff t, c, a3, a0
           stp   t, c, [sp, #cache_m30]
           cdiff t, c, a2, a1
           stp   t, c, [sp, #cache_m21]
           cdiff t, c, a3, a1
           stp   t, c, [sp, #cache_m31]
           cdiff t, c, a3, a2
           stp   t, c, [sp, #cache_m32]

           add sp, sp, #(6*16)

           subs count, count, #1
           cbnz count, bignum_emontredc_8n_cdiff_base_precomp

           // Set modulus pointer back to its original value
           mov x2, x25

           //
           // End of precomputation
           //

           stp x3, x30, [sp]
           //stp x3, xzr, [sp]
           stp x26, xzr, [sp, #16]
           mov x28, xzr
           lsl x0, x12, #5

           movi    v29.2d, #0x000000ffffffff

bignum_emontredc_8n_cdiff_base_outerloop:
          ldr x3, [sp]
          ldph x17, x19, x1, #0, t0
          ldph x20, x21, x1, #16, t1
          ldp x8, x9, [x2, #0]
          ldp x10, x11, [x2, #16]
          ldr q21, [x2, #16]

          // Montgomery step 0

          mul x4, x17, x3
          // NEON: Calculate x4 * (x10, x11) that does two 64x64->128-bit multiplications.
          vmul_2x_64_64_128 v21, x4, v0, v1
          mov x14, v0.d[0]
          mov x15, v0.d[1]
          mul   x12, x4, x8
          adds x17, x17, x12
          umulh x12, x4, x8
          mul   x13, x4, x9
          adcs x19, x19, x13
          umulh x13, x4, x9
          adcs x20, x20, x14
          adcs x21, x21, x15
          mov x14, v1.d[0]
          mov x15, v1.d[1]
          adc x22, xzr, xzr
          adds x19, x19, x12
          adcs x20, x20, x13
          adcs x21, x21, x14
          adc x22, x22, x15

          // Montgomery step 1

          mul x5, x19, x3
          // NEON: Calculate x5 * (x10, x11) that does two 64x64->128-bit multiplications.
          vmul_2x_64_64_128 v21, x5, v0, v1
          mov x14, v0.d[0]
          mov x15, v0.d[1]
          mul   x12, x5, x8
          adds  x19, x19, x12
          umulh x12, x5, x8
          mul   x13, x5, x9
          adcs  x20, x20, x13
          umulh x13, x5, x9
          adcs x21, x21, x14
          adcs x22, x22, x15
          mov x14, v1.d[0]
          mov x15, v1.d[1]
          adc x23, xzr, xzr
          adds x20, x20, x12
          adcs x21, x21, x13
          adcs x22, x22, x14
          adc x23, x23, x15
          stph x4, x5, x1, #0, t0

          // Montgomery step 2

          mul   x6, x20, x3
          // NEON: Calculate x6 * (x10, x11) that does two 64x64->128-bit multiplications.
          vmul_2x_64_64_128 v21, x6, v21, v1
          mov   x14, v21.d[0]
          mov   x15, v21.d[1]
          mul   x12, x6, x8
          adds  x20, x20, x12
          umulh x12, x6, x8
          mul   x13, x6, x9
          adcs  x21, x21, x13
          umulh x13, x6, x9
          adcs  x22, x22, x14
          adcs  x23, x23, x15
          mov   x14, v1.d[0]
          mov   x15, v1.d[1]
          adc   x24, xzr, xzr
          adds  x21, x21, x12
          mul   x7, x21, x3
          adcs  x22, x22, x13
          adcs  x23, x23, x14
          adc   x24, x24, x15

          stph x6, x7, x1, #16, t1

          // Montgomery step 3

          mul   x12, x7, x8
          mul   x13, x7, x9
          mul   x14, x7, x10
          mul   x15, x7, x11
          adds  x21, x21, x12
          umulh x12, x7, x8
          adcs  x22, x22, x13
          umulh x13, x7, x9
          adcs  x23, x23, x14
          umulh x14, x7, x10
          adcs  x24, x24, x15
          umulh x15, x7, x11
          adc   x25, xzr, xzr
          adds  x12, x22, x12
          adcs  x13, x23, x13
          adcs  x14, x24, x14
          adc   x15, x25, x15

          lsr count, x0, #5

          ldrh q20, x1, #0, t0
          ldrh q21, x1, #16, t1

          // Precompute and cache differences required in the
          // ADK multiplication conducted by the innerl oop.
          // Save each difference (somewhat inefficiently)
          // as a pair (t,c) of 64-bit + carry.
          //
          // The same caching trick is applied to the modulus,
          // for which the various differences can even be hoisted
          // out of the entire multiplication routine.

          // a0 - a1 with carry
          cdiff x16,x26,a0,a1
          stph x16, x26, sp, #cache_a01, t01
          // a0 - a2 with carry
          cdiff x16,x26,a0,a2
          stph x16, x26, sp, #cache_a02, t02
          // a0 - a3 with carry
          cdiff x16,x26,a0,a3
          stph x16, x26, sp, #cache_a03, t03
          // a1 - a2 with carry
          cdiff x16,x26,a1,a2
          stph x16, x26, sp, #cache_a12, t12
          // a1 - a3 with carry
          cdiff x16,x26,a1,a3
          stph x16, x26, sp, #cache_a13, t13
          // a2 - a3 with carry
          cdiff x16,x26,a2,a3
          stph x16, x26, sp, #cache_a23, t23

          // Precompute and cache some precomputations for
          // the Neon multiplications in the inner loop
          uzp2    vpre00.4s, v20.4s, v20.4s
          xtn     vpre01.2s, v20.2d
          rev64   vpre02.4s, v20.4s
          uzp2    vpre10.4s, v21.4s, v21.4s
          xtn     vpre11.2s, v21.2d
          rev64   vpre12.4s, v21.4s

bignum_emontredc_8n_cdiff_base_maddloop_neon:

          ldr q22, [x2, #32]!
          ldr q23, [x2, #16]

          xtn     v4.2s, v22.2d
          umull   v6.2d, v4.2s, vpre01.2s
          umull   v7.2d, v4.2s, vpre00.2s
          uzp2    v16.4s, v22.4s, v22.4s
          mul     v0.4s, vpre02.4s, v22.4s
          usra    v7.2d, v6.2d, #32
          umull   v25.2d, v16.2s, vpre00.2s
          uaddlp  v0.2d, v0.4s
          and     v2.16b, v7.16b, v29.16b
          umlal   v2.2d, v16.2s, vpre01.2s
          shl     v24.2d, v0.2d, #32
          usra    v25.2d, v7.2d, #32
          umlal   v24.2d, v4.2s, vpre01.2s
          usra    v25.2d, v2.2d, #32

          // Original version without caching
          // uzp2    v3.4s, v22.4s, v22.4s
          // xtn     v4.2s, v20.2d
          // xtn     v5.2s, v22.2d
          // rev64   v1.4s, v22.4s
          // umull   v6.2d, v4.2s, v5.2s
          // umull   v7.2d, v4.2s, v3.2s
          // uzp2    v16.4s, v20.4s, v20.4s
          // mul     v0.4s, v1.4s, v20.4s
          // usra    v7.2d, v6.2d, #32
          // umull   v25.2d, v16.2s, v3.2s
          // uaddlp  v0.2d, v0.4s
          // and     v2.16b, v7.16b, v29.16b
          // umlal   v2.2d, v16.2s, v5.2s
          // shl     v24.2d, v0.2d, #32
          // usra    v25.2d, v7.2d, #32
          // umlal   v24.2d, v4.2s, v5.2s
          // usra    v25.2d, v2.2d, #32

          xtn     v4.2s, v23.2d
          umull   v6.2d, v4.2s, vpre11.2s
          umull   v7.2d, v4.2s, vpre10.2s
          uzp2    v16.4s, v23.4s, v23.4s
          mul     v0.4s, vpre12.4s, v23.4s
          usra    v7.2d, v6.2d, #32
          umull   v27.2d, v16.2s, vpre10.2s
          uaddlp  v0.2d, v0.4s
          and     v2.16b, v7.16b, v29.16b
          umlal   v2.2d, v16.2s, vpre11.2s
          shl     v26.2d, v0.2d, #32
          usra    v27.2d, v7.2d, #32
          umlal   v26.2d, v4.2s, vpre11.2s
          usra    v27.2d, v2.2d, #32

          // Original version without caching
          // uzp2    v3.4s, v23.4s, v23.4s
          // xtn     v4.2s, v21.2d
          // xtn     v5.2s, v23.2d
          // rev64   v1.4s, v23.4s
          // umull   v6.2d, v4.2s, v5.2s
          // umull   v7.2d, v4.2s, v3.2s
          // uzp2    v16.4s, v21.4s, v21.4s
          // mul     v0.4s, v1.4s, v21.4s
          // usra    v7.2d, v6.2d, #32
          // umull   v27.2d, v16.2s, v3.2s
          // uaddlp  v0.2d, v0.4s
          // and     v2.16b, v7.16b, v29.16b
          // umlal   v2.2d, v16.2s, v5.2s
          // shl     v26.2d, v0.2d, #32
          // usra    v27.2d, v7.2d, #32
          // umlal   v26.2d, v4.2s, v5.2s
          // usra    v27.2d, v2.2d, #32

          mov x16, v25.d[0] // hi bits of (x4 * x8)
          mov x26, v27.d[0] // hi bits of (x6 * x10)
          mov x3,  v25.d[1] // hi bits of (x5 * x9)
          mov x17, v27.d[1] // hi bits of (x6 * x10)

          mov x20, v24.d[1] // lo bits of (x5 * x9)
          mov x21, v26.d[0] // lo bits of (x6 * x10)
          mov x24, v26.d[1] // lo bits of (x7 * x11)

          // Not necessary if one uses cached differences for the modulus
          //ldp x8,  x9,  [x2, #0]
          //ldp x10, x11, [x2, #16]

          adds x22, x20, x16
          adcs x23, x21, x3
          adcs x24, x24, x26
          adc  x25, x17, xzr
          mov  x17, v24.d[0] // lo bits of (x4 * x8)
          ldp  x20, x21, [x1, #32]!
          adds x12, x12, x20
          adcs x13, x13, x21
          ldp  x20, x21, [x1, #16]
          adcs x14, x14, x20
          adcs x15, x15, x21
          adc  x16, xzr, xzr
          adds x19, x22, x17
          adcs x22, x23, x22
          adcs x23, x24, x23
          adcs x24, x25, x24
          adc  x25, xzr, x25
          adds x20, x22, x17
          adcs x21, x23, x19
          adcs x22, x24, x22
          adcs x23, x25, x23
          adcs x24, xzr, x24
          adc  x25, xzr, x25
          adds x17, x17, x12
          adcs x19, x19, x13
          adcs x20, x20, x14
          adcs x21, x21, x15
          adcs x22, x22, x16
          adcs x23, x23, xzr
          adcs x24, x24, xzr
          adc x25, x25, xzr

          ldph x15, x12, sp, #cache_a23, t23
          // Original code without caching
          //subs x15, x6, x7
          //cneg x15, x15, cc
          //csetm x12, cc

          ldp x13, x14, [x30, #cache_m32]
          eor x12, x12, x14
          // Original code without caching
          //cdiff x13, x14, x11, x10
          //subs x13, x11, x10
          //cneg x13, x13, cc

          mul   x14, x15, x13
          umulh x13, x15, x13
          adds  xzr, x12, #1
          eor   x14, x14, x12
          adcs  x23, x23, x14
          eor   x13, x13, x12
          adcs  x24, x24, x13
          adc   x25, x25, x12

          ldph x15, x12, sp, #cache_a01, t01
          //subs x15, x4, x5
          //cneg x15, x15, cc
          //csetm x12, cc

          ldp x13, x14, [x30, #cache_m10]
          eor x12, x12, x14
          // Original code without caching
          //subs x13, x9, x8
          //cneg x13, x13, cc
          //cinv x12, x12, cc

          mul   x14, x15, x13
          umulh x13, x15, x13
          adds  xzr, x12, #1
          eor   x14, x14, x12
          adcs  x19, x19, x14
          eor   x13, x13, x12
          adcs  x20, x20, x13
          adcs  x21, x21, x12
          adcs  x22, x22, x12
          adcs  x23, x23, x12
          adcs  x24, x24, x12
          adc   x25, x25, x12

          stp x17, x19, [x1, #0]

          ldph x15, x12, sp, #cache_a13, t13
          //subs x15, x5, x7
          //cneg x15, x15, cc
          //csetm x12, cc

          ldp x13, x14, [x30, #cache_m31]
          eor x12, x12, x14
          // Original code without caching
          //subs x13, x11, x9
          //cneg x13, x13, cc
          //cinv x12, x12, cc

          mul   x14, x15, x13
          umulh x13, x15, x13
          adds  xzr, x12, #1
          eor   x14, x14, x12
          adcs  x22, x22, x14
          eor   x13, x13, x12
          adcs  x23, x23, x13
          adcs  x24, x24, x12
          adc   x25, x25, x12

          ldph x15, x12, sp, #cache_a02, t02
          //subs x15, x4, x6
          //cneg x15, x15, cc
          //csetm x12, cc

          ldp x13, x14, [x30, #cache_m20]
          eor x12, x12, x14
          // Original code without caching
          //subs x13, x10, x8
          //cneg x13, x13, cc
          //cinv x12, x12, cc

          mul   x14, x15, x13
          umulh x13, x15, x13
          adds  xzr, x12, #1
          eor   x14, x14, x12
          adcs  x20, x20, x14
          eor   x13, x13, x12
          adcs  x21, x21, x13
          adcs  x22, x22, x12
          adcs  x23, x23, x12
          adcs  x24, x24, x12
          adc   x25, x25, x12

          ldph x15, x12, sp, #cache_a03, t03
          //subs x15, x4, x7
          //cneg x15, x15, cc
          //csetm x12, cc

          ldp x13, x14, [x30, #cache_m30]
          eor x12, x12, x14
          // Original code without caching
          //subs x13, x11, x8
          //cneg x13, x13, cc
          //cinv x12, x12, cc

          mul   x14, x15, x13
          umulh x13, x15, x13
          adds  xzr, x12, #1
          eor   x14, x14, x12
          adcs  x21, x21, x14
          eor   x13, x13, x12
          adcs  x22, x22, x13
          adcs  x23, x23, x12
          adcs  x24, x24, x12
          adc   x25, x25, x12

          ldph x15, x12, sp, #cache_a12, t12
          //subs x15, x5, x6
          //cneg x15, x15, cc
          //csetm x12, cc

          ldp x13, x14, [x30, #cache_m21]
          eor x12, x12, x14
          // Original code without caching
          //subs x13, x10, x9
          //cneg x13, x13, cc
          //cinv x12, x12, cc

          mul   x14, x15, x13
          umulh x13, x15, x13
          adds  xzr, x12, #1
          eor   x14, x14, x12
          adcs  x21, x21, x14

          stp x20, x21, [x1, #16]
          eor x13, x13, x12
          adcs x22, x22, x13
          adcs x13, x23, x12
          adcs x14, x24, x12
          adc x15, x25, x12
          mov x12, x22

          add x30, x30, #96

          sub count, count, #1
          cbnz count, bignum_emontredc_8n_cdiff_base_maddloop_neon

          ldp x17, x19, [x1, #32]
          ldp x20, x21, [x1, #48]
          ldp x26, xzr, [sp, #16]
          adds xzr, x28, x28
          adcs x17, x17, x12
          adcs x19, x19, x13
          adcs x20, x20, x14
          adcs x21, x21, x15
          csetm x28, cs
          stp x17, x19, [x1, #32]
          stp x20, x21, [x1, #48]
          sub x1, x1, x0
          sub x2, x2, x0
          add x1, x1, #32
          subs x26, x26, #1
          stp x26, xzr, [sp, #16]

          // Restore buffer base for cached modulus differences
          ldr x30, [sp, #8]

          bne bignum_emontredc_8n_cdiff_base_outerloop
          neg x0, x28

bignum_emontredc_8n_cdiff_base_end:
           add sp, sp, #32
           add sp, sp, #(6*16)

           ldp x29, x30, [sp, #(0*16)]
           ldp x27, x28, [sp, #(1*16)]
           ldp x25, x26, [sp, #(2*16)]
           ldp x23, x24, [sp, #(3*16)]
           ldp x21, x22, [sp, #(4*16)]
           ldp x19, x20, [sp, #(5*16)]
           add sp, sp, #(6*16)

           ret

