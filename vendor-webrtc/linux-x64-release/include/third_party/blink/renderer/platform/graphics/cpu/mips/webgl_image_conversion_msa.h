// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CPU_MIPS_WEBGL_IMAGE_CONVERSION_MSA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CPU_MIPS_WEBGL_IMAGE_CONVERSION_MSA_H_

#include "base/compiler_specific.h"
#include "build/build_config.h"

#if defined(HAVE_MIPS_MSA_INTRINSICS)

#include "third_party/blink/renderer/platform/cpu/mips/common_macros_msa.h"

namespace blink {

namespace simd {

#define SEPERATE_RGBA_FRM_16BIT_5551INPUT(in, out_r, out_g, out_b, out_a) \
  cnst31 = (v8u16)__msa_ldi_h(0x1F);                                      \
  cnst7 = (v8u16)__msa_ldi_h(0x7);                                        \
  cnst1 = (v8u16)__msa_ldi_h(0x1);                                        \
  out_r = (v8u16)SRLI_H(in, 11);                                          \
  out_g = ((v8u16)SRLI_H(in, 6)) & cnst31;                                \
  out_b = ((v8u16)SRLI_H(in, 1)) & cnst31;                                \
  out_a = in & cnst1;                                                     \
  out_r = ((v8u16)SLLI_H(out_r, 3)) | (out_r & cnst7);                    \
  out_g = ((v8u16)SLLI_H(out_g, 3)) | (out_g & cnst7);                    \
  out_b = ((v8u16)SLLI_H(out_b, 3)) | (out_b & cnst7);                    \
  out_a = (v8u16)CEQI_H((v8i16)out_a, 1);

#define SEPERATE_RGBA_FRM_16BIT_4444INPUT(in, out_rb, out_ga) \
  out_rb = (v16u8)SRLI_B((v16u8)in, 4);                       \
  out_ga = ANDI_B((v16u8)in, 15);                             \
  out_rb = ((v16u8)SLLI_B(out_rb, 4)) | out_rb;               \
  out_ga = ((v16u8)SLLI_B(out_ga, 4)) | out_ga;

ALWAYS_INLINE void unpackOneRowOfRGBA5551ToRGBA8MSA(const uint16_t*& source,
                                                    uint8_t*& destination,
                                                    unsigned& pixelsPerRow) {
  unsigned i;
  v8u16 src0, src1, src2, src3;
  v8u16 src0r, src0g, src0b, src0a, src1r, src1g, src1b, src1a;
  v8u16 src2r, src2g, src2b, src2a, src3r, src3g, src3b, src3a;
  v8u16 cnst31, cnst7, cnst1;
  v16u8 dst0, dst1, dst2, dst3, dst4, dst5, dst6, dst7;
  v16u8 dst8, dst9, dst10, dst11, dst12, dst13, dst14, dst15;
  v16u8 out0, out1, out2, out3, out4, out5, out6, out7;

  for (i = (pixelsPerRow >> 5); i--;) {
    LD_UH4(source, 8, src0, src1, src2, src3);
    SEPERATE_RGBA_FRM_16BIT_5551INPUT(src0, src0r, src0g, src0b, src0a);
    SEPERATE_RGBA_FRM_16BIT_5551INPUT(src1, src1r, src1g, src1b, src1a);
    SEPERATE_RGBA_FRM_16BIT_5551INPUT(src2, src2r, src2g, src2b, src2a);
    SEPERATE_RGBA_FRM_16BIT_5551INPUT(src3, src3r, src3g, src3b, src3a);
    ILVRL_B2_UB(src0g, src0r, dst0, dst1);
    ILVRL_B2_UB(src0a, src0b, dst2, dst3);
    ILVRL_B2_UB(src1g, src1r, dst4, dst5);
    ILVRL_B2_UB(src1a, src1b, dst6, dst7);
    ILVRL_B2_UB(src2g, src2r, dst8, dst9);
    ILVRL_B2_UB(src2a, src2b, dst10, dst11);
    ILVRL_B2_UB(src3g, src3r, dst12, dst13);
    ILVRL_B2_UB(src3a, src3b, dst14, dst15);
    ILVEV_H2_UB(dst0, dst2, dst1, dst3, out0, out1);
    ILVEV_H2_UB(dst4, dst6, dst5, dst7, out2, out3);
    ILVEV_H2_UB(dst8, dst10, dst9, dst11, out4, out5);
    ILVEV_H2_UB(dst12, dst14, dst13, dst15, out6, out7);
    ST_UB8(out0, out1, out2, out3, out4, out5, out6, out7, destination, 16);
  }

  if (pixelsPerRow & 31) {
    if ((pixelsPerRow & 16) && (pixelsPerRow & 8)) {
      LD_UH3(source, 8, src0, src1, src2);
      SEPERATE_RGBA_FRM_16BIT_5551INPUT(src0, src0r, src0g, src0b, src0a);
      SEPERATE_RGBA_FRM_16BIT_5551INPUT(src1, src1r, src1g, src1b, src1a);
      SEPERATE_RGBA_FRM_16BIT_5551INPUT(src2, src2r, src2g, src2b, src2a);
      ILVRL_B2_UB(src0g, src0r, dst0, dst1);
      ILVRL_B2_UB(src0a, src0b, dst2, dst3);
      ILVRL_B2_UB(src1g, src1r, dst4, dst5);
      ILVRL_B2_UB(src1a, src1b, dst6, dst7);
      ILVRL_B2_UB(src2g, src2r, dst8, dst9);
      ILVRL_B2_UB(src2a, src2b, dst10, dst11);
      ILVEV_H2_UB(dst0, dst2, dst1, dst3, out0, out1);
      ILVEV_H2_UB(dst4, dst6, dst5, dst7, out2, out3);
      ILVEV_H2_UB(dst8, dst10, dst9, dst11, out4, out5);
      ST_UB6(out0, out1, out2, out3, out4, out5, destination, 16);
    } else if (pixelsPerRow & 16) {
      LD_UH2(source, 8, src0, src1);
      SEPERATE_RGBA_FRM_16BIT_5551INPUT(src0, src0r, src0g, src0b, src0a);
      SEPERATE_RGBA_FRM_16BIT_5551INPUT(src1, src1r, src1g, src1b, src1a);
      ILVRL_B2_UB(src0g, src0r, dst0, dst1);
      ILVRL_B2_UB(src0a, src0b, dst2, dst3);
      ILVRL_B2_UB(src1g, src1r, dst4, dst5);
      ILVRL_B2_UB(src1a, src1b, dst6, dst7);
      ILVEV_H2_UB(dst0, dst2, dst1, dst3, out0, out1);
      ILVEV_H2_UB(dst4, dst6, dst5, dst7, out2, out3);
      ST_UB4(out0, out1, out2, out3, destination, 16);
    } else if (pixelsPerRow & 8) {
      src0 = LD_UH(source);
      source += 8;
      SEPERATE_RGBA_FRM_16BIT_5551INPUT(src0, src0r, src0g, src0b, src0a);
      ILVRL_B2_UB(src0g, src0r, dst0, dst1);
      ILVRL_B2_UB(src0a, src0b, dst2, dst3);
      ILVEV_H2_UB(dst0, dst2, dst1, dst3, out0, out1);
      ST_UB2(out0, out1, destination, 16);
    }
  }

  pixelsPerRow &= 7;
}

ALWAYS_INLINE void unpackOneRowOfBGRA8LittleToRGBA8MSA(const uint32_t*& source,
                                                       uint32_t*& destination,
                                                       unsigned& pixelsPerRow) {
  unsigned i;
  v16u8 src0, src1, src2, src3, src4, src5, src6, src7;
  v16u8 src8, src9, src10, src11, src12, src13, src14, src15;

  for (i = (pixelsPerRow >> 6); i--;) {
    LD_UB8(source, 4, src0, src1, src2, src3, src4, src5, src6, src7);
    LD_UB8(source, 4, src8, src9, src10, src11, src12, src13, src14, src15);
    SHF_B4_UB(src0, src1, src2, src3, 198);
    SHF_B4_UB(src4, src5, src6, src7, 198);
    SHF_B4_UB(src8, src9, src10, src11, 198);
    SHF_B4_UB(src12, src13, src14, src15, 198);
    ST_UB8(src0, src1, src2, src3, src4, src5, src6, src7, destination, 4);
    ST_UB8(src8, src9, src10, src11, src12, src13, src14, src15, destination,
           4);
  }

  if (pixelsPerRow & 63) {
    if (pixelsPerRow & 32) {
      if ((pixelsPerRow & 16) && (pixelsPerRow & 8)) {
        LD_UB8(source, 4, src0, src1, src2, src3, src4, src5, src6, src7);
        LD_UB6(source, 4, src8, src9, src10, src11, src12, src13);
        SHF_B4_UB(src0, src1, src2, src3, 198);
        SHF_B4_UB(src4, src5, src6, src7, 198);
        SHF_B4_UB(src8, src9, src10, src11, 198);
        SHF_B2_UB(src12, src13, 198);
        ST_UB8(src0, src1, src2, src3, src4, src5, src6, src7, destination, 4);
        ST_UB6(src8, src9, src10, src11, src12, src13, destination, 4);
      } else if (pixelsPerRow & 16) {
        LD_UB8(source, 4, src0, src1, src2, src3, src4, src5, src6, src7);
        LD_UB4(source, 4, src8, src9, src10, src11);
        SHF_B4_UB(src0, src1, src2, src3, 198);
        SHF_B4_UB(src4, src5, src6, src7, 198);
        SHF_B4_UB(src8, src9, src10, src11, 198);
        ST_UB8(src0, src1, src2, src3, src4, src5, src6, src7, destination, 4);
        ST_UB4(src8, src9, src10, src11, destination, 4);
      } else if (pixelsPerRow & 8) {
        LD_UB8(source, 4, src0, src1, src2, src3, src4, src5, src6, src7);
        LD_UB2(source, 4, src8, src9);
        SHF_B4_UB(src0, src1, src2, src3, 198);
        SHF_B4_UB(src4, src5, src6, src7, 198);
        SHF_B2_UB(src8, src9, 198);
        ST_UB8(src0, src1, src2, src3, src4, src5, src6, src7, destination, 4);
        ST_UB2(src8, src9, destination, 4);
      } else {
        LD_UB8(source, 4, src0, src1, src2, src3, src4, src5, src6, src7);
        SHF_B4_UB(src0, src1, src2, src3, 198);
        SHF_B4_UB(src4, src5, src6, src7, 198);
        ST_UB8(src0, src1, src2, src3, src4, src5, src6, src7, destination, 4);
      }
    } else if ((pixelsPerRow & 16) && (pixelsPerRow & 8)) {
      LD_UB6(source, 4, src0, src1, src2, src3, src4, src5);
      SHF_B4_UB(src0, src1, src2, src3, 198);
      SHF_B2_UB(src4, src5, 198);
      ST_UB6(src0, src1, src2, src3, src4, src5, destination, 4);
    } else if (pixelsPerRow & 16) {
      LD_UB4(source, 4, src0, src1, src2, src3);
      SHF_B4_UB(src0, src1, src2, src3, 198);
      ST_UB4(src0, src1, src2, src3, destination, 4);
    } else if (pixelsPerRow & 8) {
      LD_UB2(source, 4, src0, src1);
      SHF_B2_UB(src0, src1, 198);
      ST_UB2(src0, src1, destination, 4);
    }

    if (pixelsPerRow & 4) {
      src0 = LD_UB(source);
      source += 4;
      src0 = (v16u8)__msa_shf_b((v16i8)src0, 198);
      ST_UB(src0, destination);
      destination += 4;
    }
  }

  pixelsPerRow &= 3;
}

ALWAYS_INLINE void unpackOneRowOfRGBA4444ToRGBA8MSA(const uint16_t*& source,
                                                    uint8_t*& destination,
                                                    unsigned& pixelsPerRow) {
  unsigned i;
  v8u16 src0, src1, src2, src3;
  v16u8 src0rb, src0ga, src1rb, src1ga, src2rb, src2ga, src3rb, src3ga;
  v16u8 dst0, dst1, dst2, dst3, dst4, dst5, dst6, dst7;
  v16u8 out0, out1, out2, out3, out4, out5, out6, out7;

  for (i = (pixelsPerRow >> 5); i--;) {
    LD_UH4(source, 8, src0, src1, src2, src3);
    SEPERATE_RGBA_FRM_16BIT_4444INPUT(src0, src0rb, src0ga);
    SEPERATE_RGBA_FRM_16BIT_4444INPUT(src1, src1rb, src1ga);
    SEPERATE_RGBA_FRM_16BIT_4444INPUT(src2, src2rb, src2ga);
    SEPERATE_RGBA_FRM_16BIT_4444INPUT(src3, src3rb, src3ga);
    ILVODEV_B2_UB(src0ga, src0rb, dst0, dst1);
    ILVODEV_B2_UB(src1ga, src1rb, dst2, dst3);
    ILVODEV_B2_UB(src2ga, src2rb, dst4, dst5);
    ILVODEV_B2_UB(src3ga, src3rb, dst6, dst7);
    ILVRL_H2_UB(dst1, dst0, out0, out1);
    ILVRL_H2_UB(dst3, dst2, out2, out3);
    ILVRL_H2_UB(dst5, dst4, out4, out5);
    ILVRL_H2_UB(dst7, dst6, out6, out7);
    ST_UB8(out0, out1, out2, out3, out4, out5, out6, out7, destination, 16);
  }

  if (pixelsPerRow & 31) {
    if ((pixelsPerRow & 16) && (pixelsPerRow & 8)) {
      LD_UH3(source, 8, src0, src1, src2);
      SEPERATE_RGBA_FRM_16BIT_4444INPUT(src0, src0rb, src0ga);
      SEPERATE_RGBA_FRM_16BIT_4444INPUT(src1, src1rb, src1ga);
      SEPERATE_RGBA_FRM_16BIT_4444INPUT(src2, src2rb, src2ga);
      ILVODEV_B2_UB(src0ga, src0rb, dst0, dst1);
      ILVODEV_B2_UB(src1ga, src1rb, dst2, dst3);
      ILVODEV_B2_UB(src2ga, src2rb, dst4, dst5);
      ILVRL_H2_UB(dst1, dst0, out0, out1);
      ILVRL_H2_UB(dst3, dst2, out2, out3);
      ILVRL_H2_UB(dst5, dst4, out4, out5);
      ST_UB6(out0, out1, out2, out3, out4, out5, destination, 16);
    } else if (pixelsPerRow & 16) {
      LD_UH2(source, 8, src0, src1);
      SEPERATE_RGBA_FRM_16BIT_4444INPUT(src0, src0rb, src0ga);
      SEPERATE_RGBA_FRM_16BIT_4444INPUT(src1, src1rb, src1ga);
      ILVODEV_B2_UB(src0ga, src0rb, dst0, dst1);
      ILVODEV_B2_UB(src1ga, src1rb, dst2, dst3);
      ILVRL_H2_UB(dst1, dst0, out0, out1);
      ILVRL_H2_UB(dst3, dst2, out2, out3);
      ST_UB4(out0, out1, out2, out3, destination, 16);
    } else if (pixelsPerRow & 8) {
      src0 = LD_UH(source);
      source += 8;
      SEPERATE_RGBA_FRM_16BIT_4444INPUT(src0, src0rb, src0ga);
      ILVODEV_B2_UB(src0ga, src0rb, dst0, dst1);
      ILVRL_H2_UB(dst1, dst0, out0, out1);
      ST_UB2(out0, out1, destination, 16);
    }
  }

  pixelsPerRow &= 7;
}

ALWAYS_INLINE void packOneRowOfRGBA8LittleToRGBA8MSA(const uint8_t*& source,
                                                     uint8_t*& destination,
                                                     unsigned& pixelsPerRow) {
  unsigned i;
  v16u8 src0, src1, src2, src3, out0, out1, out2, out3;
  v16u8 src0R, src1R, src2R, src3R, src0G, src1G, src2G, src3G;
  v16u8 src0B, src1B, src2B, src3B, src0A, src1A, src2A, src3A;
  v16u8 dst0R, dst1R, dst2R, dst3R, dst0G, dst1G, dst2G, dst3G;
  v16u8 dst0B, dst1B, dst2B, dst3B, dst0A, dst1A, dst2A, dst3A;
  v16u8 dst0RG, dst1RG, dst2RG, dst3RG, dst0BA, dst1BA, dst2BA, dst3BA;
  v4f32 fsrc0R, fsrc1R, fsrc2R, fsrc3R, fsrc0G, fsrc1G, fsrc2G, fsrc3G;
  v4f32 fsrc0B, fsrc1B, fsrc2B, fsrc3B, fsrc0A, fsrc1A, fsrc2A, fsrc3A;
  v4u32 vCnst255 = (v4u32)__msa_ldi_w(255);
  v16u8 alphaMask = {0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255};
  v4f32 vfCnst255 = __msa_ffint_u_w(vCnst255);

  for (i = (pixelsPerRow >> 4); i--;) {
    LD_UB4(source, 16, src0, src1, src2, src3);
    CEQI_B4_UB(src0, src1, src2, src3, 0, src0A, src1A, src2A, src3A);
    src0A = __msa_bmnz_v(src0, alphaMask, src0A);
    src1A = __msa_bmnz_v(src1, alphaMask, src1A);
    src2A = __msa_bmnz_v(src2, alphaMask, src2A);
    src3A = __msa_bmnz_v(src3, alphaMask, src3A);
    AND_V4_UB(src0A, src1A, src2A, src3A, alphaMask, src0A, src1A, src2A,
              src3A);
    src0A = SLDI_UB(src0A, src0A, 3);
    src1A = SLDI_UB(src1A, src1A, 3);
    src2A = SLDI_UB(src2A, src2A, 3);
    src3A = SLDI_UB(src3A, src3A, 3);
    FFINTU_W4_SP(src0A, src1A, src2A, src3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
    DIV4(vfCnst255, fsrc0A, vfCnst255, fsrc1A, vfCnst255, fsrc2A, vfCnst255,
         fsrc3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
    AND_V4_UB(src0, src1, src2, src3, vCnst255, src0R, src1R, src2R, src3R);
    FFINTU_W4_SP(src0R, src1R, src2R, src3R, fsrc0R, fsrc1R, fsrc2R, fsrc3R);
    MUL4(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fsrc2R, fsrc2A, fsrc3R, fsrc3A, fsrc0R,
         fsrc1R, fsrc2R, fsrc3R);
    src0G = SLDI_UB(src0, src0, 1);
    src1G = SLDI_UB(src1, src1, 1);
    src2G = SLDI_UB(src2, src2, 1);
    src3G = SLDI_UB(src3, src3, 1);
    AND_V4_UB(src0G, src1G, src2G, src3G, vCnst255, src0G, src1G, src2G, src3G);
    FFINTU_W4_SP(src0G, src1G, src2G, src3G, fsrc0G, fsrc1G, fsrc2G, fsrc3G);
    MUL4(fsrc0G, fsrc0A, fsrc1G, fsrc1A, fsrc2G, fsrc2A, fsrc3G, fsrc3A, fsrc0G,
         fsrc1G, fsrc2G, fsrc3G);
    src0B = SLDI_UB(src0, src0, 2);
    src1B = SLDI_UB(src1, src1, 2);
    src2B = SLDI_UB(src2, src2, 2);
    src3B = SLDI_UB(src3, src3, 2);
    AND_V4_UB(src0B, src1B, src2B, src3B, vCnst255, src0B, src1B, src2B, src3B);
    FFINTU_W4_SP(src0B, src1B, src2B, src3B, fsrc0B, fsrc1B, fsrc2B, fsrc3B);
    MUL4(fsrc0B, fsrc0A, fsrc1B, fsrc1A, fsrc2B, fsrc2A, fsrc3B, fsrc3A, fsrc0B,
         fsrc1B, fsrc2B, fsrc3B);
    FTRUNCU_W4_UB(fsrc0R, fsrc1R, fsrc2R, fsrc3R, dst0R, dst1R, dst2R, dst3R);
    FTRUNCU_W4_UB(fsrc0G, fsrc1G, fsrc2G, fsrc3G, dst0G, dst1G, dst2G, dst3G);
    FTRUNCU_W4_UB(fsrc0B, fsrc1B, fsrc2B, fsrc3B, dst0B, dst1B, dst2B, dst3B);
    dst0A = SLDI_UB(src0, src0, 3);
    dst1A = SLDI_UB(src1, src1, 3);
    dst2A = SLDI_UB(src2, src2, 3);
    dst3A = SLDI_UB(src3, src3, 3);
    ILVEV_B2_UB(dst0R, dst0G, dst1R, dst1G, dst0RG, dst1RG);
    ILVEV_B2_UB(dst2R, dst2G, dst3R, dst3G, dst2RG, dst3RG);
    ILVEV_B2_UB(dst0B, dst0A, dst1B, dst1A, dst0BA, dst1BA);
    ILVEV_B2_UB(dst2B, dst2A, dst3B, dst3A, dst2BA, dst3BA);
    ILVEV_H2_UB(dst0RG, dst0BA, dst1RG, dst1BA, out0, out1);
    ILVEV_H2_UB(dst2RG, dst2BA, dst3RG, dst3BA, out2, out3);
    ST_UB4(out0, out1, out2, out3, destination, 16);
  }

  if (pixelsPerRow & 15) {
    if (pixelsPerRow & 8) {
      LD_UB2(source, 16, src0, src1);
      CEQI_B2_UB(src0, src1, 0, src0A, src1A);
      src0A = __msa_bmnz_v(src0, alphaMask, src0A);
      src1A = __msa_bmnz_v(src1, alphaMask, src1A);
      AND_V2_UB(src0A, src1A, alphaMask, src0A, src1A);
      src0A = SLDI_UB(src0A, src0A, 3);
      src1A = SLDI_UB(src1A, src1A, 3);
      FFINTU_W2_SP(src0A, src1A, fsrc0A, fsrc1A);
      DIV2(vfCnst255, fsrc0A, vfCnst255, fsrc1A, fsrc0A, fsrc1A);
      AND_V2_UB(src0, src1, vCnst255, src0R, src1R);
      FFINTU_W2_SP(src0R, src1R, fsrc0R, fsrc1R);
      MUL2(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fsrc0R, fsrc1R);
      src0G = SLDI_UB(src0, src0, 1);
      src1G = SLDI_UB(src1, src1, 1);
      AND_V2_UB(src0G, src1G, vCnst255, src0G, src1G);
      FFINTU_W2_SP(src0G, src1G, fsrc0G, fsrc1G);
      MUL2(fsrc0G, fsrc0A, fsrc1G, fsrc1A, fsrc0G, fsrc1G);
      src0B = SLDI_UB(src0, src0, 2);
      src1B = SLDI_UB(src1, src1, 2);
      AND_V2_UB(src0B, src1B, vCnst255, src0B, src1B);
      FFINTU_W2_SP(src0B, src1B, fsrc0B, fsrc1B);
      MUL2(fsrc0B, fsrc0A, fsrc1B, fsrc1A, fsrc0B, fsrc1B);
      FTRUNCU_W2_UB(fsrc0R, fsrc1R, dst0R, dst1R);
      FTRUNCU_W2_UB(fsrc0G, fsrc1G, dst0G, dst1G);
      FTRUNCU_W2_UB(fsrc0B, fsrc1B, dst0B, dst1B);
      dst0A = SLDI_UB(src0, src0, 3);
      dst1A = SLDI_UB(src1, src1, 3);
      ILVEV_B2_UB(dst0R, dst0G, dst1R, dst1G, dst0RG, dst1RG);
      ILVEV_B2_UB(dst0B, dst0A, dst1B, dst1A, dst0BA, dst1BA);
      ILVEV_H2_UB(dst0RG, dst0BA, dst1RG, dst1BA, out0, out1);
      ST_UB2(out0, out1, destination, 16);
    }

    if (pixelsPerRow & 4) {
      src0 = LD_UB(source);
      source += 16;
      src0A = CEQI_B(src0, 0);
      src0A = __msa_bmnz_v(src0, alphaMask, src0A);
      src0A = src0A & alphaMask;
      src0A = SLDI_UB(src0A, src0A, 3);
      fsrc0A = __msa_ffint_u_w((v4u32)src0A);
      fsrc0A = vfCnst255 / fsrc0A;
      src0R = src0 & (v16u8)vCnst255;
      fsrc0R = __msa_ffint_u_w((v4u32)src0R);
      fsrc0R *= fsrc0A;
      src0G = SLDI_UB(src0, src0, 1);
      src0G &= (v16u8)vCnst255;
      fsrc0G = __msa_ffint_u_w((v4u32)src0G);
      fsrc0G *= fsrc0A;
      src0B = SLDI_UB(src0, src0, 2);
      src0B &= (v16u8)vCnst255;
      fsrc0B = __msa_ffint_u_w((v4u32)src0B);
      fsrc0B *= fsrc0A;
      dst0R = (v16u8)__msa_ftrunc_u_w(fsrc0R);
      dst0G = (v16u8)__msa_ftrunc_u_w(fsrc0G);
      dst0B = (v16u8)__msa_ftrunc_u_w(fsrc0B);
      dst0A = SLDI_UB(src0, src0, 3);
      dst0RG = (v16u8)__msa_ilvev_b((v16i8)dst0G, (v16i8)dst0R);
      dst0BA = (v16u8)__msa_ilvev_b((v16i8)dst0A, (v16i8)dst0B);
      out0 = (v16u8)__msa_ilvev_h((v8i16)dst0BA, (v8i16)dst0RG);
      ST_UB(out0, destination);
      destination += 16;
    }
  }

  pixelsPerRow &= 3;
}

ALWAYS_INLINE void packOneRowOfRGBA8ToUnsignedShort5551MSA(
    const uint8_t*& source,
    uint16_t*& destination,
    unsigned& pixelsPerRow) {
  unsigned i;
  v16u8 src0, src1, src2, src3, src4, src5, src6, src7;
  v16u8 src0r, src0b, src1r, src1b, src2r, src2b, src3r, src3b;
  v16u8 src0g = {0}, src0a = {0}, src1g = {0}, src1a = {0};
  v16u8 src2g = {0}, src2a = {0}, src3g = {0}, src3a = {0};
  v16u8 src0gt, src1gt, src2gt, src3gt;
  v8u16 dst0, dst1, dst2, dst3;

  for (i = (pixelsPerRow >> 5); i--;) {
    LD_UB8(source, 16, src0, src1, src2, src3, src4, src5, src6, src7);
    PCKEV_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0r, src1r,
                src2r, src3r);
    PCKOD_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0b, src1b,
                src2b, src3b);
    SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
    SLDI_B2_UB(src2g, src3g, src2r, src3r, src2g, src3g, 1);
    SLDI_B2_UB(src0a, src1a, src0b, src1b, src0a, src1a, 1);
    SLDI_B2_UB(src2a, src3a, src2b, src3b, src2a, src3a, 1);
    src0gt = (v16u8)SLLI_B(src0g, 3);
    src1gt = (v16u8)SLLI_B(src1g, 3);
    src2gt = (v16u8)SLLI_B(src2g, 3);
    src3gt = (v16u8)SLLI_B(src3g, 3);
    SRLI_B4_UB(src0g, src1g, src2g, src3g, 5);
    SRLI_B4_UB(src0b, src1b, src2b, src3b, 2);
    SRLI_B4_UB(src0a, src1a, src2a, src3a, 7);
    BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
    BINSRI_B2_UB(src2r, src2g, src3r, src3g, src2r, src3r, 2);
    BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 5);
    BINSRI_B2_UB(src2gt, src2b, src3gt, src3b, src2b, src3b, 5);
    BINSRI_B2_UB(src0b, src0a, src1b, src1a, src0b, src1b, 0);
    BINSRI_B2_UB(src2b, src2a, src3b, src3a, src2b, src3b, 0);
    ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
    ILVEV_B2_UH(src2b, src2r, src3b, src3r, dst2, dst3);
    ST_UH4(dst0, dst1, dst2, dst3, destination, 8);
  }

  if (pixelsPerRow & 31) {
    if ((pixelsPerRow & 16) && (pixelsPerRow & 8)) {
      LD_UB6(source, 16, src0, src1, src2, src3, src4, src5);
      PCKEV_H3_UB(src1, src0, src3, src2, src5, src4, src0r, src1r, src2r);
      PCKOD_H3_UB(src1, src0, src3, src2, src5, src4, src0b, src1b, src2b);
      SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
      SLDI_B2_UB(src2g, src0a, src2r, src0b, src2g, src0a, 1);
      SLDI_B2_UB(src1a, src2a, src1b, src2b, src1a, src2a, 1);
      src0gt = (v16u8)SLLI_B(src0g, 3);
      src1gt = (v16u8)SLLI_B(src1g, 3);
      src2gt = (v16u8)SLLI_B(src2g, 3);
      SRLI_B3_UB(src0g, src1g, src2g, 5);
      SRLI_B3_UB(src0b, src1b, src2b, 2);
      SRLI_B3_UB(src0a, src1a, src2a, 7);
      BINSRI_B3_UB(src0r, src0g, src1r, src1g, src2r, src2g, src0r, src1r,
                   src2r, 2);
      BINSRI_B3_UB(src0gt, src0b, src1gt, src1b, src2gt, src2b, src0b, src1b,
                   src2b, 5);
      BINSRI_B3_UB(src0b, src0a, src1b, src1a, src2b, src2a, src0b, src1b,
                   src2b, 0);
      ILVEV_B3_UH(src0b, src0r, src1b, src1r, src2b, src2r, dst0, dst1, dst2);
      ST_UH3(dst0, dst1, dst2, destination, 8);
    } else if (pixelsPerRow & 16) {
      LD_UB4(source, 16, src0, src1, src2, src3);
      PCKEV_H2_UB(src1, src0, src3, src2, src0r, src1r);
      PCKOD_H2_UB(src1, src0, src3, src2, src0b, src1b);
      SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
      SLDI_B2_UB(src0a, src1a, src0b, src1b, src0a, src1a, 1);
      src0gt = (v16u8)SLLI_B(src0g, 3);
      src1gt = (v16u8)SLLI_B(src1g, 3);
      SRLI_B2_UB(src0g, src1g, 5);
      SRLI_B2_UB(src0b, src1b, 2);
      SRLI_B2_UB(src0a, src1a, 7);
      BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
      BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 5);
      BINSRI_B2_UB(src0b, src0a, src1b, src1a, src0b, src1b, 0);
      ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
      ST_UH2(dst0, dst1, destination, 8);
    } else if (pixelsPerRow & 8) {
      LD_UB2(source, 16, src0, src1);
      src0r = (v16u8)__msa_pckev_h((v8i16)src1, (v8i16)src0);
      src0b = (v16u8)__msa_pckod_h((v8i16)src1, (v8i16)src0);
      SLDI_B2_UB(src0g, src0a, src0r, src0b, src0g, src0a, 1);
      src0gt = (v16u8)SLLI_B(src0g, 3);
      src0g = (v16u8)SRLI_B(src0g, 5);
      src0b = (v16u8)SRLI_B(src0b, 2);
      src0a = (v16u8)SRLI_B(src0a, 7);
      src0r = (v16u8)__msa_binsri_b((v16u8)src0r, (v16u8)src0g, 2);
      src0b = (v16u8)__msa_binsri_b((v16u8)src0gt, (v16u8)src0b, 5);
      src0b = (v16u8)__msa_binsri_b((v16u8)src0b, (v16u8)src0a, 0);
      dst0 = (v8u16)__msa_ilvev_b((v16i8)src0r, (v16i8)src0b);
      ST_UH(dst0, destination);
      destination += 8;
    }
  }

  pixelsPerRow &= 7;
}

ALWAYS_INLINE void packOneRowOfRGBA8ToUnsignedShort565MSA(
    const uint8_t*& source,
    uint16_t*& destination,
    unsigned& pixelsPerRow) {
  unsigned i;
  v16u8 src0, src1, src2, src3, src4, src5, src6, src7;
  v16u8 src0r, src0b, src1r, src1b, src2r, src2b, src3r, src3b;
  v16u8 src0g = {0}, src1g = {0}, src2g = {0}, src3g = {0};
  v16u8 src0gt, src1gt, src2gt, src3gt;
  v8u16 dst0, dst1, dst2, dst3;

  for (i = (pixelsPerRow >> 6); i--;) {
    LD_UB8(source, 16, src0, src1, src2, src3, src4, src5, src6, src7);
    PCKEV_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0r, src1r,
                src2r, src3r);
    PCKOD_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0b, src1b,
                src2b, src3b);
    SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
    SLDI_B2_UB(src2g, src3g, src2r, src3r, src2g, src3g, 1);
    src0gt = (v16u8)SLLI_B(src0g, 3);
    src1gt = (v16u8)SLLI_B(src1g, 3);
    src2gt = (v16u8)SLLI_B(src2g, 3);
    src3gt = (v16u8)SLLI_B(src3g, 3);
    SRLI_B4_UB(src0g, src1g, src2g, src3g, 5);
    SRLI_B4_UB(src0b, src1b, src2b, src3b, 3);
    BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
    BINSRI_B2_UB(src2r, src2g, src3r, src3g, src2r, src3r, 2);
    BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 4);
    BINSRI_B2_UB(src2gt, src2b, src3gt, src3b, src2b, src3b, 4);
    ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
    ILVEV_B2_UH(src2b, src2r, src3b, src3r, dst2, dst3);
    LD_UB4(source, 16, src0, src1, src2, src3);
    ST_UH4(dst0, dst1, dst2, dst3, destination, 8);
    LD_UB4(source, 16, src4, src5, src6, src7);
    PCKEV_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0r, src1r,
                src2r, src3r);
    PCKOD_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0b, src1b,
                src2b, src3b);
    SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
    SLDI_B2_UB(src2g, src3g, src2r, src3r, src2g, src3g, 1);
    src0gt = (v16u8)SLLI_B(src0g, 3);
    src1gt = (v16u8)SLLI_B(src1g, 3);
    src2gt = (v16u8)SLLI_B(src2g, 3);
    src3gt = (v16u8)SLLI_B(src3g, 3);
    SRLI_B4_UB(src0g, src1g, src2g, src3g, 5);
    SRLI_B4_UB(src0b, src1b, src2b, src3b, 3);
    BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
    BINSRI_B2_UB(src2r, src2g, src3r, src3g, src2r, src3r, 2);
    BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 4);
    BINSRI_B2_UB(src2gt, src2b, src3gt, src3b, src2b, src3b, 4);
    ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
    ILVEV_B2_UH(src2b, src2r, src3b, src3r, dst2, dst3);
    ST_UH4(dst0, dst1, dst2, dst3, destination, 8);
  }

  if (pixelsPerRow & 63) {
    if (pixelsPerRow & 32) {
      if ((pixelsPerRow & 16) && (pixelsPerRow & 8)) {
        LD_UB8(source, 16, src0, src1, src2, src3, src4, src5, src6, src7);
        PCKEV_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0r,
                    src1r, src2r, src3r);
        PCKOD_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0b,
                    src1b, src2b, src3b);
        SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
        SLDI_B2_UB(src2g, src3g, src2r, src3r, src2g, src3g, 1);
        src0gt = (v16u8)SLLI_B(src0g, 3);
        src1gt = (v16u8)SLLI_B(src1g, 3);
        src2gt = (v16u8)SLLI_B(src2g, 3);
        src3gt = (v16u8)SLLI_B(src3g, 3);
        SRLI_B4_UB(src0g, src1g, src2g, src3g, 5);
        SRLI_B4_UB(src0b, src1b, src2b, src3b, 3);
        BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
        BINSRI_B2_UB(src2r, src2g, src3r, src3g, src2r, src3r, 2);
        BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 4);
        BINSRI_B2_UB(src2gt, src2b, src3gt, src3b, src2b, src3b, 4);
        ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
        ILVEV_B2_UH(src2b, src2r, src3b, src3r, dst2, dst3);
        LD_UB6(source, 16, src0, src1, src2, src3, src4, src5);
        ST_UH4(dst0, dst1, dst2, dst3, destination, 8);
        PCKEV_H3_UB(src1, src0, src3, src2, src5, src4, src0r, src1r, src2r);
        PCKOD_H3_UB(src1, src0, src3, src2, src5, src4, src0b, src1b, src2b);
        src0g = SLDI_UB(src0g, src0r, 1);
        src1g = SLDI_UB(src1g, src1r, 1);
        src2g = SLDI_UB(src2g, src2r, 1);
        src0gt = (v16u8)SLLI_B(src0g, 3);
        src1gt = (v16u8)SLLI_B(src1g, 3);
        src2gt = (v16u8)SLLI_B(src2g, 3);
        SRLI_B3_UB(src0g, src1g, src2g, 5);
        SRLI_B3_UB(src0b, src1b, src2b, 3);
        BINSRI_B3_UB(src0r, src0g, src1r, src1g, src2r, src2g, src0r, src1r,
                     src2r, 2);
        BINSRI_B3_UB(src0gt, src0b, src1gt, src1b, src2gt, src2b, src0b, src1b,
                     src2b, 4);
        ILVEV_B3_UH(src0b, src0r, src1b, src1r, src2b, src2r, dst0, dst1, dst2);
        ST_UH3(dst0, dst1, dst2, destination, 8);
      } else if (pixelsPerRow & 16) {
        LD_UB8(source, 16, src0, src1, src2, src3, src4, src5, src6, src7);
        PCKEV_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0r,
                    src1r, src2r, src3r);
        PCKOD_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0b,
                    src1b, src2b, src3b);
        SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
        SLDI_B2_UB(src2g, src3g, src2r, src3r, src2g, src3g, 1);
        src0gt = (v16u8)SLLI_B(src0g, 3);
        src1gt = (v16u8)SLLI_B(src1g, 3);
        src2gt = (v16u8)SLLI_B(src2g, 3);
        src3gt = (v16u8)SLLI_B(src3g, 3);
        SRLI_B4_UB(src0g, src1g, src2g, src3g, 5);
        SRLI_B4_UB(src0b, src1b, src2b, src3b, 3);
        BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
        BINSRI_B2_UB(src2r, src2g, src3r, src3g, src2r, src3r, 2);
        BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 4);
        BINSRI_B2_UB(src2gt, src2b, src3gt, src3b, src2b, src3b, 4);
        ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
        ILVEV_B2_UH(src2b, src2r, src3b, src3r, dst2, dst3);
        LD_UB4(source, 16, src0, src1, src2, src3);
        ST_UH4(dst0, dst1, dst2, dst3, destination, 8);
        PCKEV_H2_UB(src1, src0, src3, src2, src0r, src1r);
        PCKOD_H2_UB(src1, src0, src3, src2, src0b, src1b);
        SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
        src0gt = (v16u8)SLLI_B(src0g, 3);
        src1gt = (v16u8)SLLI_B(src1g, 3);
        SRLI_B2_UB(src0g, src1g, 5);
        SRLI_B2_UB(src0b, src1b, 3);
        BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
        BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 4);
        ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
        ST_UH2(dst0, dst1, destination, 8);
      } else if (pixelsPerRow & 8) {
        LD_UB8(source, 16, src0, src1, src2, src3, src4, src5, src6, src7);
        PCKEV_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0r,
                    src1r, src2r, src3r);
        PCKOD_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0b,
                    src1b, src2b, src3b);
        SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
        SLDI_B2_UB(src2g, src3g, src2r, src3r, src2g, src3g, 1);
        src0gt = (v16u8)SLLI_B(src0g, 3);
        src1gt = (v16u8)SLLI_B(src1g, 3);
        src2gt = (v16u8)SLLI_B(src2g, 3);
        src3gt = (v16u8)SLLI_B(src3g, 3);
        SRLI_B4_UB(src0g, src1g, src2g, src3g, 5);
        SRLI_B4_UB(src0b, src1b, src2b, src3b, 3);
        BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
        BINSRI_B2_UB(src2r, src2g, src3r, src3g, src2r, src3r, 2);
        BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 4);
        BINSRI_B2_UB(src2gt, src2b, src3gt, src3b, src2b, src3b, 4);
        ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
        ILVEV_B2_UH(src2b, src2r, src3b, src3r, dst2, dst3);
        LD_UB2(source, 16, src0, src1);
        ST_UH4(dst0, dst1, dst2, dst3, destination, 8);
        src0r = (v16u8)__msa_pckev_h((v8i16)src1, (v8i16)src0);
        src0b = (v16u8)__msa_pckod_h((v8i16)src1, (v8i16)src0);
        src0g = SLDI_UB(src0g, src0r, 1);
        src0gt = (v16u8)SLLI_B(src0g, 3);
        src0g = (v16u8)SRLI_B(src0g, 5);
        src0b = (v16u8)SRLI_B(src0b, 3);
        src0r = (v16u8)__msa_binsri_b((v16u8)src0r, (v16u8)src0g, 2);
        src0b = (v16u8)__msa_binsri_b((v16u8)src0gt, (v16u8)src0b, 4);
        dst0 = (v8u16)__msa_ilvev_b((v16i8)src0r, (v16i8)src0b);
        ST_UH(dst0, destination);
        destination += 8;
      } else {
        LD_UB8(source, 16, src0, src1, src2, src3, src4, src5, src6, src7);
        PCKEV_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0r,
                    src1r, src2r, src3r);
        PCKOD_H4_UB(src1, src0, src3, src2, src5, src4, src7, src6, src0b,
                    src1b, src2b, src3b);
        SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
        SLDI_B2_UB(src2g, src3g, src2r, src3r, src2g, src3g, 1);
        src0gt = (v16u8)SLLI_B(src0g, 3);
        src1gt = (v16u8)SLLI_B(src1g, 3);
        src2gt = (v16u8)SLLI_B(src2g, 3);
        src3gt = (v16u8)SLLI_B(src3g, 3);
        SRLI_B4_UB(src0g, src1g, src2g, src3g, 5);
        SRLI_B4_UB(src0b, src1b, src2b, src3b, 3);
        BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
        BINSRI_B2_UB(src2r, src2g, src3r, src3g, src2r, src3r, 2);
        BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 4);
        BINSRI_B2_UB(src2gt, src2b, src3gt, src3b, src2b, src3b, 4);
        ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
        ILVEV_B2_UH(src2b, src2r, src3b, src3r, dst2, dst3);
        ST_UH4(dst0, dst1, dst2, dst3, destination, 8);
      }
    } else if ((pixelsPerRow & 16) && (pixelsPerRow & 8)) {
      LD_UB6(source, 16, src0, src1, src2, src3, src4, src5);
      PCKEV_H3_UB(src1, src0, src3, src2, src5, src4, src0r, src1r, src2r);
      PCKOD_H3_UB(src1, src0, src3, src2, src5, src4, src0b, src1b, src2b);
      src0g = SLDI_UB(src0g, src0r, 1);
      src1g = SLDI_UB(src1g, src1r, 1);
      src2g = SLDI_UB(src2g, src2r, 1);
      src0gt = (v16u8)SLLI_B(src0g, 3);
      src1gt = (v16u8)SLLI_B(src1g, 3);
      src2gt = (v16u8)SLLI_B(src2g, 3);
      SRLI_B3_UB(src0g, src1g, src2g, 5);
      SRLI_B3_UB(src0b, src1b, src2b, 3);
      BINSRI_B3_UB(src0r, src0g, src1r, src1g, src2r, src2g, src0r, src1r,
                   src2r, 2);
      BINSRI_B3_UB(src0gt, src0b, src1gt, src1b, src2gt, src2b, src0b, src1b,
                   src2b, 4);
      ILVEV_B3_UH(src0b, src0r, src1b, src1r, src2b, src2r, dst0, dst1, dst2);
      ST_UH3(dst0, dst1, dst2, destination, 8);
    } else if (pixelsPerRow & 16) {
      LD_UB4(source, 16, src0, src1, src2, src3);
      PCKEV_H2_UB(src1, src0, src3, src2, src0r, src1r);
      PCKOD_H2_UB(src1, src0, src3, src2, src0b, src1b);
      SLDI_B2_UB(src0g, src1g, src0r, src1r, src0g, src1g, 1);
      src0gt = (v16u8)SLLI_B(src0g, 3);
      src1gt = (v16u8)SLLI_B(src1g, 3);
      SRLI_B2_UB(src0g, src1g, 5);
      SRLI_B2_UB(src0b, src1b, 3);
      BINSRI_B2_UB(src0r, src0g, src1r, src1g, src0r, src1r, 2);
      BINSRI_B2_UB(src0gt, src0b, src1gt, src1b, src0b, src1b, 4);
      ILVEV_B2_UH(src0b, src0r, src1b, src1r, dst0, dst1);
      ST_UH2(dst0, dst1, destination, 8);
    } else if (pixelsPerRow & 8) {
      LD_UB2(source, 16, src0, src1);
      src0r = (v16u8)__msa_pckev_h((v8i16)src1, (v8i16)src0);
      src0b = (v16u8)__msa_pckod_h((v8i16)src1, (v8i16)src0);
      src0g = SLDI_UB(src0g, src0r, 1);
      src0gt = (v16u8)SLLI_B(src0g, 3);
      src0g = (v16u8)SRLI_B(src0g, 5);
      src0b = (v16u8)SRLI_B(src0b, 3);
      src0r = (v16u8)__msa_binsri_b((v16u8)src0r, (v16u8)src0g, 2);
      src0b = (v16u8)__msa_binsri_b((v16u8)src0gt, (v16u8)src0b, 4);
      dst0 = (v8u16)__msa_ilvev_b((v16i8)src0r, (v16i8)src0b);
      ST_UH(dst0, destination);
      destination += 8;
    }
  }

  pixelsPerRow &= 7;
}

ALWAYS_INLINE void packOneRowOfRGBA8ToUnsignedShort4444MSA(
    const uint8_t*& source,
    uint16_t*& destination,
    unsigned& pixelsPerRow) {
  unsigned i;
  v16u8 src0, src1, src2, src3, src4, src5, src6, src7;
  v16u8 vec0, vec1, vec2, vec3, vec4, vec5, vec6, vec7;
  v8u16 dst0, dst1, dst2, dst3;

  for (i = (pixelsPerRow >> 5); i--;) {
    LD_UB8(source, 16, src0, src1, src2, src3, src4, src5, src6, src7);
    SRLI_H4_UB(src0, src1, src2, src3, vec0, vec1, vec2, vec3, 12);
    SRLI_H4_UB(src4, src5, src6, src7, vec4, vec5, vec6, vec7, 12);
    BINSLI_B2_UB(vec0, src0, vec1, src1, vec0, vec1, 3);
    BINSLI_B2_UB(vec2, src2, vec3, src3, vec2, vec3, 3);
    BINSLI_B2_UB(vec4, src4, vec5, src5, vec4, vec5, 3);
    BINSLI_B2_UB(vec6, src6, vec7, src7, vec6, vec7, 3);
    PCKEV_B4_UH(vec1, vec0, vec3, vec2, vec5, vec4, vec7, vec6, dst0, dst1,
                dst2, dst3);
    SHF_B4_UH(dst0, dst1, dst2, dst3, 177);
    ST_UH4(dst0, dst1, dst2, dst3, destination, 8);
  }

  if (pixelsPerRow & 31) {
    if (pixelsPerRow & 16) {
      if ((pixelsPerRow & 8) && (pixelsPerRow & 4)) {
        LD_UB7(source, 16, src0, src1, src2, src3, src4, src5, src6);
        SRLI_H4_UB(src0, src1, src2, src3, vec0, vec1, vec2, vec3, 12);
        SRLI_H2_UB(src4, src5, vec4, vec5, 12);
        vec6 = (v16u8)SRLI_H(src6, 12);
        BINSLI_B2_UB(vec0, src0, vec1, src1, vec0, vec1, 3);
        BINSLI_B2_UB(vec2, src2, vec3, src3, vec2, vec3, 3);
        BINSLI_B2_UB(vec4, src4, vec5, src5, vec4, vec5, 3);
        vec6 = (v16u8)__msa_binsli_b((v16u8)vec6, (v16u8)src6, 3);
        PCKEV_B2_UH(vec1, vec0, vec3, vec2, dst0, dst1);
        PCKEV_B2_UH(vec5, vec4, vec6, vec6, dst2, dst3);
        SHF_B4_UH(dst0, dst1, dst2, dst3, 177);
        ST_UH3(dst0, dst1, dst2, destination, 8);
        ST8x1_UB(dst3, destination);
        destination += 4;
      } else if (pixelsPerRow & 8) {
        LD_UB6(source, 16, src0, src1, src2, src3, src4, src5);
        SRLI_H4_UB(src0, src1, src2, src3, vec0, vec1, vec2, vec3, 12);
        SRLI_H2_UB(src4, src5, vec4, vec5, 12);
        BINSLI_B2_UB(vec0, src0, vec1, src1, vec0, vec1, 3);
        BINSLI_B2_UB(vec2, src2, vec3, src3, vec2, vec3, 3);
        BINSLI_B2_UB(vec4, src4, vec5, src5, vec4, vec5, 3);
        PCKEV_B3_UH(vec1, vec0, vec3, vec2, vec5, vec4, dst0, dst1, dst2);
        SHF_B3_UH(dst0, dst1, dst2, 177);
        ST_UH3(dst0, dst1, dst2, destination, 8);
      } else if (pixelsPerRow & 4) {
        LD_UB5(source, 16, src0, src1, src2, src3, src4);
        SRLI_H4_UB(src0, src1, src2, src3, vec0, vec1, vec2, vec3, 12);
        vec4 = (v16u8)SRLI_H(src4, 12);
        BINSLI_B2_UB(vec0, src0, vec1, src1, vec0, vec1, 3);
        BINSLI_B2_UB(vec2, src2, vec3, src3, vec2, vec3, 3);
        vec4 = (v16u8)__msa_binsli_b((v16u8)vec4, (v16u8)src4, 3);
        PCKEV_B3_UH(vec1, vec0, vec3, vec2, vec4, vec4, dst0, dst1, dst2);
        SHF_B3_UH(dst0, dst1, dst2, 177);
        ST_UH2(dst0, dst1, destination, 8);
        ST8x1_UB(dst2, destination);
        destination += 4;
      } else {
        LD_UB4(source, 16, src0, src1, src2, src3);
        SRLI_H4_UB(src0, src1, src2, src3, vec0, vec1, vec2, vec3, 12);
        BINSLI_B2_UB(vec0, src0, vec1, src1, vec0, vec1, 3);
        BINSLI_B2_UB(vec2, src2, vec3, src3, vec2, vec3, 3);
        PCKEV_B2_UH(vec1, vec0, vec3, vec2, dst0, dst1);
        SHF_B2_UH(dst0, dst1, 177);
        ST_UH2(dst0, dst1, destination, 8);
      }
    } else if ((pixelsPerRow & 8) && (pixelsPerRow & 4)) {
      LD_UB3(source, 16, src0, src1, src2);
      SRLI_H2_UB(src0, src1, vec0, vec1, 12);
      vec2 = (v16u8)SRLI_H(src2, 12);
      BINSLI_B2_UB(vec0, src0, vec1, src1, vec0, vec1, 3);
      vec2 = (v16u8)__msa_binsli_b((v16u8)vec2, (v16u8)src2, 3);
      PCKEV_B2_UH(vec1, vec0, vec2, vec2, dst0, dst1);
      SHF_B2_UH(dst0, dst1, 177);
      ST_UH(dst0, destination);
      destination += 8;
      ST8x1_UB(dst1, destination);
      destination += 4;
    } else if (pixelsPerRow & 16) {
      LD_UB4(source, 16, src0, src1, src2, src3);
      SRLI_H4_UB(src0, src1, src2, src3, vec0, vec1, vec2, vec3, 12);
      BINSLI_B2_UB(vec0, src0, vec1, src1, vec0, vec1, 3);
      BINSLI_B2_UB(vec2, src2, vec3, src3, vec2, vec3, 3);
      PCKEV_B2_UH(vec1, vec0, vec3, vec2, dst0, dst1);
      SHF_B2_UH(dst0, dst1, 177);
      ST_UH2(dst0, dst1, destination, 8);
    } else if (pixelsPerRow & 8) {
      LD_UB2(source, 16, src0, src1);
      SRLI_H2_UB(src0, src1, vec0, vec1, 12);
      BINSLI_B2_UB(vec0, src0, vec1, src1, vec0, vec1, 3);
      dst0 = (v8u16)__msa_pckev_b((v16i8)vec1, (v16i8)vec0);
      dst0 = (v8u16)__msa_shf_b((v16i8)dst0, 177);
      ST_UH(dst0, destination);
      destination += 8;
    } else if (pixelsPerRow & 4) {
      src0 = LD_UB(source);
      source += 16;
      vec0 = (v16u8)SRLI_H(src0, 12);
      vec0 = (v16u8)__msa_binsli_b((v16u8)vec0, (v16u8)src0, 3);
      dst0 = (v8u16)__msa_pckev_b((v16i8)vec0, (v16i8)vec0);
      dst0 = (v8u16)__msa_shf_b((v16i8)dst0, 177);
      ST8x1_UB(dst0, destination);
      destination += 4;
    }
  }

  pixelsPerRow &= 3;
}

ALWAYS_INLINE void packOneRowOfRGBA8LittleToR8MSA(const uint8_t*& source,
                                                  uint8_t*& destination,
                                                  unsigned& pixelsPerRow) {
  unsigned i;
  v16u8 src0, src1, src2, src3, src4, src5, src6, src7;
  v16u8 src0A, src1A, src2A, src3A, src4A, src5A, src6A, src7A;
  v16u8 src0R, src1R, src2R, src3R, src4R, src5R, src6R, src7R;
  v16u8 dst0, dst1, dst2, dst3, dst4, dst5, dst6, dst7;
  v4f32 fsrc0A, fsrc1A, fsrc2A, fsrc3A, fsrc4A, fsrc5A, fsrc6A, fsrc7A;
  v4f32 fsrc0R, fsrc1R, fsrc2R, fsrc3R, fsrc4R, fsrc5R, fsrc6R, fsrc7R;
  v4f32 fdst0R, fdst1R, fdst2R, fdst3R, fdst4R, fdst5R, fdst6R, fdst7R;
  const v16u8 alphaMask = {0, 0, 0, 255, 0, 0, 0, 255,
                           0, 0, 0, 255, 0, 0, 0, 255};
  const v4u32 vCnst255 = (v4u32)__msa_ldi_w(255);
  const v4f32 vfCnst255 = __msa_ffint_u_w(vCnst255);

  for (i = (pixelsPerRow >> 5); i--;) {
    LD_UB8(source, 16, src0, src1, src2, src3, src4, src5, src6, src7);
    CEQI_B4_UB(src0, src1, src2, src3, 0, src0A, src1A, src2A, src3A);
    CEQI_B4_UB(src4, src5, src6, src7, 0, src4A, src5A, src6A, src7A);
    src0A = __msa_bmnz_v(src0, alphaMask, src0A);
    src1A = __msa_bmnz_v(src1, alphaMask, src1A);
    src2A = __msa_bmnz_v(src2, alphaMask, src2A);
    src3A = __msa_bmnz_v(src3, alphaMask, src3A);
    src4A = __msa_bmnz_v(src4, alphaMask, src4A);
    src5A = __msa_bmnz_v(src5, alphaMask, src5A);
    src6A = __msa_bmnz_v(src6, alphaMask, src6A);
    src7A = __msa_bmnz_v(src7, alphaMask, src7A);
    AND_V4_UB(src0A, src1A, src2A, src3A, alphaMask, src0A, src1A, src2A,
              src3A);
    AND_V4_UB(src4A, src5A, src6A, src7A, alphaMask, src4A, src5A, src6A,
              src7A);
    src0A = SLDI_UB(src0A, src0A, 3);
    src1A = SLDI_UB(src1A, src1A, 3);
    src2A = SLDI_UB(src2A, src2A, 3);
    src3A = SLDI_UB(src3A, src3A, 3);
    src4A = SLDI_UB(src4A, src4A, 3);
    src5A = SLDI_UB(src5A, src5A, 3);
    src6A = SLDI_UB(src6A, src6A, 3);
    src7A = SLDI_UB(src7A, src7A, 3);
    AND_V4_UB(src0, src1, src2, src3, vCnst255, src0R, src1R, src2R, src3R);
    AND_V4_UB(src4, src5, src6, src7, vCnst255, src4R, src5R, src6R, src7R);
    FFINTU_W4_SP(src0A, src1A, src2A, src3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
    FFINTU_W4_SP(src4A, src5A, src6A, src7A, fsrc4A, fsrc5A, fsrc6A, fsrc7A);
    FFINTU_W4_SP(src0R, src1R, src2R, src3R, fsrc0R, fsrc1R, fsrc2R, fsrc3R);
    FFINTU_W4_SP(src4R, src5R, src6R, src7R, fsrc4R, fsrc5R, fsrc6R, fsrc7R);
    DIV4(vfCnst255, fsrc0A, vfCnst255, fsrc1A, vfCnst255, fsrc2A, vfCnst255,
         fsrc3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
    DIV4(vfCnst255, fsrc4A, vfCnst255, fsrc5A, vfCnst255, fsrc6A, vfCnst255,
         fsrc7A, fsrc4A, fsrc5A, fsrc6A, fsrc7A);
    MUL4(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fsrc2R, fsrc2A, fsrc3R, fsrc3A, fdst0R,
         fdst1R, fdst2R, fdst3R);
    MUL4(fsrc4R, fsrc4A, fsrc5R, fsrc5A, fsrc6R, fsrc6A, fsrc7R, fsrc7A, fdst4R,
         fdst5R, fdst6R, fdst7R);
    FTRUNCU_W4_UB(fdst0R, fdst1R, fdst2R, fdst3R, dst0, dst1, dst2, dst3);
    FTRUNCU_W4_UB(fdst4R, fdst5R, fdst6R, fdst7R, dst4, dst5, dst6, dst7);
    PCKEV_H4_UB(dst1, dst0, dst3, dst2, dst5, dst4, dst7, dst6, dst0, dst2,
                dst4, dst6);
    PCKEV_B2_UB(dst2, dst0, dst6, dst4, dst0, dst1);
    ST_UB2(dst0, dst1, destination, 16);
  }

  if (pixelsPerRow & 31) {
    if ((pixelsPerRow & 16) && (pixelsPerRow & 8)) {
      LD_UB6(source, 16, src0, src1, src2, src3, src4, src5);
      CEQI_B4_UB(src0, src1, src2, src3, 0, src0A, src1A, src2A, src3A);
      CEQI_B2_UB(src4, src5, 0, src4A, src5A);
      src0A = __msa_bmnz_v(src0, alphaMask, src0A);
      src1A = __msa_bmnz_v(src1, alphaMask, src1A);
      src2A = __msa_bmnz_v(src2, alphaMask, src2A);
      src3A = __msa_bmnz_v(src3, alphaMask, src3A);
      src4A = __msa_bmnz_v(src4, alphaMask, src4A);
      src5A = __msa_bmnz_v(src5, alphaMask, src5A);
      AND_V4_UB(src0A, src1A, src2A, src3A, alphaMask, src0A, src1A, src2A,
                src3A);
      AND_V2_UB(src4A, src5A, alphaMask, src4A, src5A);
      src0A = SLDI_UB(src0A, src0A, 3);
      src1A = SLDI_UB(src1A, src1A, 3);
      src2A = SLDI_UB(src2A, src2A, 3);
      src3A = SLDI_UB(src3A, src3A, 3);
      src4A = SLDI_UB(src4A, src4A, 3);
      src5A = SLDI_UB(src5A, src5A, 3);
      AND_V4_UB(src0, src1, src2, src3, vCnst255, src0R, src1R, src2R, src3R);
      AND_V2_UB(src4, src5, vCnst255, src4R, src5R);
      FFINTU_W4_SP(src0A, src1A, src2A, src3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
      FFINTU_W2_SP(src4A, src5A, fsrc4A, fsrc5A);
      FFINTU_W4_SP(src0R, src1R, src2R, src3R, fsrc0R, fsrc1R, fsrc2R, fsrc3R);
      FFINTU_W2_SP(src4R, src5R, fsrc4R, fsrc5R);
      DIV4(vfCnst255, fsrc0A, vfCnst255, fsrc1A, vfCnst255, fsrc2A, vfCnst255,
           fsrc3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
      DIV2(vfCnst255, fsrc4A, vfCnst255, fsrc5A, fsrc4A, fsrc5A);
      MUL4(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fsrc2R, fsrc2A, fsrc3R, fsrc3A,
           fdst0R, fdst1R, fdst2R, fdst3R);
      MUL2(fsrc4R, fsrc4A, fsrc5R, fsrc5A, fdst4R, fdst5R);
      FTRUNCU_W4_UB(fdst0R, fdst1R, fdst2R, fdst3R, dst0, dst1, dst2, dst3);
      FTRUNCU_W2_UB(fdst4R, fdst5R, dst4, dst5);
      PCKEV_H3_UB(dst1, dst0, dst3, dst2, dst5, dst4, dst0, dst2, dst4);
      PCKEV_B2_UB(dst2, dst0, dst4, dst4, dst0, dst1);
      ST_UB(dst0, destination);
      destination += 16;
      ST8x1_UB(dst1, destination);
      destination += 8;
    } else if (pixelsPerRow & 16) {
      LD_UB4(source, 16, src0, src1, src2, src3);
      CEQI_B4_UB(src0, src1, src2, src3, 0, src0A, src1A, src2A, src3A);
      src0A = __msa_bmnz_v(src0, alphaMask, src0A);
      src1A = __msa_bmnz_v(src1, alphaMask, src1A);
      src2A = __msa_bmnz_v(src2, alphaMask, src2A);
      src3A = __msa_bmnz_v(src3, alphaMask, src3A);
      AND_V4_UB(src0A, src1A, src2A, src3A, alphaMask, src0A, src1A, src2A,
                src3A);
      src0A = SLDI_UB(src0A, src0A, 3);
      src1A = SLDI_UB(src1A, src1A, 3);
      src2A = SLDI_UB(src2A, src2A, 3);
      src3A = SLDI_UB(src3A, src3A, 3);
      AND_V4_UB(src0, src1, src2, src3, vCnst255, src0R, src1R, src2R, src3R);
      FFINTU_W4_SP(src0A, src1A, src2A, src3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
      FFINTU_W4_SP(src0R, src1R, src2R, src3R, fsrc0R, fsrc1R, fsrc2R, fsrc3R);
      DIV4(vfCnst255, fsrc0A, vfCnst255, fsrc1A, vfCnst255, fsrc2A, vfCnst255,
           fsrc3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
      MUL4(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fsrc2R, fsrc2A, fsrc3R, fsrc3A,
           fdst0R, fdst1R, fdst2R, fdst3R);
      FTRUNCU_W4_UB(fdst0R, fdst1R, fdst2R, fdst3R, dst0, dst1, dst2, dst3);
      PCKEV_H2_UB(dst1, dst0, dst3, dst2, dst0, dst2);
      dst0 = (v16u8)__msa_pckev_b((v16i8)dst2, (v16i8)dst0);
      ST_UB(dst0, destination);
      destination += 16;
    } else if (pixelsPerRow & 8) {
      LD_UB2(source, 16, src0, src1);
      CEQI_B2_UB(src0, src1, 0, src0A, src1A);
      src0A = __msa_bmnz_v(src0, alphaMask, src0A);
      src1A = __msa_bmnz_v(src1, alphaMask, src1A);
      AND_V2_UB(src0A, src1A, alphaMask, src0A, src1A);
      src0A = SLDI_UB(src0A, src0A, 3);
      src1A = SLDI_UB(src1A, src1A, 3);
      AND_V2_UB(src0, src1, vCnst255, src0R, src1R);
      FFINTU_W2_SP(src0A, src1A, fsrc0A, fsrc1A);
      FFINTU_W2_SP(src0R, src1R, fsrc0R, fsrc1R);
      DIV2(vfCnst255, fsrc0A, vfCnst255, fsrc1A, fsrc0A, fsrc1A);
      MUL2(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fdst0R, fdst1R);
      FTRUNCU_W2_UB(fdst0R, fdst1R, dst0, dst1);
      dst0 = (v16u8)__msa_pckev_h((v8i16)dst1, (v8i16)dst0);
      dst0 = (v16u8)__msa_pckev_b((v16i8)dst0, (v16i8)dst0);
      ST8x1_UB(dst0, destination);
      destination += 8;
    }
  }

  pixelsPerRow &= 7;
}

ALWAYS_INLINE void packOneRowOfRGBA8LittleToRA8MSA(const uint8_t*& source,
                                                   uint8_t*& destination,
                                                   unsigned& pixelsPerRow) {
  unsigned i;
  v16u8 src0, src1, src2, src3, src4, src5, src6, src7;
  v16u8 src0A, src1A, src2A, src3A, src4A, src5A, src6A, src7A;
  v16u8 src0R, src1R, src2R, src3R, src4R, src5R, src6R, src7R;
  v16u8 dst0, dst1, dst2, dst3, dst4, dst5, dst6, dst7;
  v4f32 fsrc0A, fsrc1A, fsrc2A, fsrc3A, fsrc4A, fsrc5A, fsrc6A, fsrc7A;
  v4f32 fsrc0R, fsrc1R, fsrc2R, fsrc3R, fsrc4R, fsrc5R, fsrc6R, fsrc7R;
  v4f32 fdst0R, fdst1R, fdst2R, fdst3R, fdst4R, fdst5R, fdst6R, fdst7R;
  const v16u8 alphaMask = {0, 0, 0, 255, 0, 0, 0, 255,
                           0, 0, 0, 255, 0, 0, 0, 255};
  const v16i8 vshfm = {0, 19, 4, 23, 8, 27, 12, 31, 0, 0, 0, 0, 0, 0, 0, 0};
  const v4u32 vCnst255 = (v4u32)__msa_ldi_w(255);
  const v4f32 vfCnst255 = __msa_ffint_u_w(vCnst255);

  for (i = (pixelsPerRow >> 5); i--;) {
    LD_UB8(source, 16, src0, src1, src2, src3, src4, src5, src6, src7);
    CEQI_B4_UB(src0, src1, src2, src3, 0, src0A, src1A, src2A, src3A);
    CEQI_B4_UB(src4, src5, src6, src7, 0, src4A, src5A, src6A, src7A);
    src0A = __msa_bmnz_v(src0, alphaMask, src0A);
    src1A = __msa_bmnz_v(src1, alphaMask, src1A);
    src2A = __msa_bmnz_v(src2, alphaMask, src2A);
    src3A = __msa_bmnz_v(src3, alphaMask, src3A);
    src4A = __msa_bmnz_v(src4, alphaMask, src4A);
    src5A = __msa_bmnz_v(src5, alphaMask, src5A);
    src6A = __msa_bmnz_v(src6, alphaMask, src6A);
    src7A = __msa_bmnz_v(src7, alphaMask, src7A);
    AND_V4_UB(src0A, src1A, src2A, src3A, alphaMask, src0A, src1A, src2A,
              src3A);
    AND_V4_UB(src4A, src5A, src6A, src7A, alphaMask, src4A, src5A, src6A,
              src7A);
    src0A = SLDI_UB(src0A, src0A, 3);
    src1A = SLDI_UB(src1A, src1A, 3);
    src2A = SLDI_UB(src2A, src2A, 3);
    src3A = SLDI_UB(src3A, src3A, 3);
    src4A = SLDI_UB(src4A, src4A, 3);
    src5A = SLDI_UB(src5A, src5A, 3);
    src6A = SLDI_UB(src6A, src6A, 3);
    src7A = SLDI_UB(src7A, src7A, 3);
    AND_V4_UB(src0, src1, src2, src3, vCnst255, src0R, src1R, src2R, src3R);
    AND_V4_UB(src4, src5, src6, src7, vCnst255, src4R, src5R, src6R, src7R);
    FFINTU_W4_SP(src0A, src1A, src2A, src3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
    FFINTU_W4_SP(src4A, src5A, src6A, src7A, fsrc4A, fsrc5A, fsrc6A, fsrc7A);
    FFINTU_W4_SP(src0R, src1R, src2R, src3R, fsrc0R, fsrc1R, fsrc2R, fsrc3R);
    FFINTU_W4_SP(src4R, src5R, src6R, src7R, fsrc4R, fsrc5R, fsrc6R, fsrc7R);
    DIV4(vfCnst255, fsrc0A, vfCnst255, fsrc1A, vfCnst255, fsrc2A, vfCnst255,
         fsrc3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
    DIV4(vfCnst255, fsrc4A, vfCnst255, fsrc5A, vfCnst255, fsrc6A, vfCnst255,
         fsrc7A, fsrc4A, fsrc5A, fsrc6A, fsrc7A);
    MUL4(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fsrc2R, fsrc2A, fsrc3R, fsrc3A, fdst0R,
         fdst1R, fdst2R, fdst3R);
    MUL4(fsrc4R, fsrc4A, fsrc5R, fsrc5A, fsrc6R, fsrc6A, fsrc7R, fsrc7A, fdst4R,
         fdst5R, fdst6R, fdst7R);
    FTRUNCU_W4_UB(fdst0R, fdst1R, fdst2R, fdst3R, dst0, dst1, dst2, dst3);
    FTRUNCU_W4_UB(fdst4R, fdst5R, fdst6R, fdst7R, dst4, dst5, dst6, dst7);
    dst0 = VSHF_UB(dst0, src0, vshfm);
    dst1 = VSHF_UB(dst1, src1, vshfm);
    dst2 = VSHF_UB(dst2, src2, vshfm);
    dst3 = VSHF_UB(dst3, src3, vshfm);
    dst4 = VSHF_UB(dst4, src4, vshfm);
    dst5 = VSHF_UB(dst5, src5, vshfm);
    dst6 = VSHF_UB(dst6, src6, vshfm);
    dst7 = VSHF_UB(dst7, src7, vshfm);
    ILVR_D4_UB(dst1, dst0, dst3, dst2, dst5, dst4, dst7, dst6, dst0, dst1, dst2,
               dst3);
    ST_UB4(dst0, dst1, dst2, dst3, destination, 16);
  }

  if (pixelsPerRow & 31) {
    if ((pixelsPerRow & 16) && (pixelsPerRow & 8)) {
      LD_UB6(source, 16, src0, src1, src2, src3, src4, src5);
      CEQI_B4_UB(src0, src1, src2, src3, 0, src0A, src1A, src2A, src3A);
      CEQI_B2_UB(src4, src5, 0, src4A, src5A);
      src0A = __msa_bmnz_v(src0, alphaMask, src0A);
      src1A = __msa_bmnz_v(src1, alphaMask, src1A);
      src2A = __msa_bmnz_v(src2, alphaMask, src2A);
      src3A = __msa_bmnz_v(src3, alphaMask, src3A);
      src4A = __msa_bmnz_v(src4, alphaMask, src4A);
      src5A = __msa_bmnz_v(src5, alphaMask, src5A);
      AND_V4_UB(src0A, src1A, src2A, src3A, alphaMask, src0A, src1A, src2A,
                src3A);
      AND_V2_UB(src4A, src5A, alphaMask, src4A, src5A);
      src0A = SLDI_UB(src0A, src0A, 3);
      src1A = SLDI_UB(src1A, src1A, 3);
      src2A = SLDI_UB(src2A, src2A, 3);
      src3A = SLDI_UB(src3A, src3A, 3);
      src4A = SLDI_UB(src4A, src4A, 3);
      src5A = SLDI_UB(src5A, src5A, 3);
      AND_V4_UB(src0, src1, src2, src3, vCnst255, src0R, src1R, src2R, src3R);
      AND_V2_UB(src4, src5, vCnst255, src4R, src5R);
      FFINTU_W4_SP(src0A, src1A, src2A, src3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
      FFINTU_W2_SP(src4A, src5A, fsrc4A, fsrc5A);
      FFINTU_W4_SP(src0R, src1R, src2R, src3R, fsrc0R, fsrc1R, fsrc2R, fsrc3R);
      FFINTU_W2_SP(src4R, src5R, fsrc4R, fsrc5R);
      DIV4(vfCnst255, fsrc0A, vfCnst255, fsrc1A, vfCnst255, fsrc2A, vfCnst255,
           fsrc3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
      DIV2(vfCnst255, fsrc4A, vfCnst255, fsrc5A, fsrc4A, fsrc5A);
      MUL4(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fsrc2R, fsrc2A, fsrc3R, fsrc3A,
           fdst0R, fdst1R, fdst2R, fdst3R);
      MUL2(fsrc4R, fsrc4A, fsrc5R, fsrc5A, fdst4R, fdst5R);
      FTRUNCU_W4_UB(fdst0R, fdst1R, fdst2R, fdst3R, dst0, dst1, dst2, dst3);
      FTRUNCU_W2_UB(fdst4R, fdst5R, dst4, dst5);
      dst0 = VSHF_UB(dst0, src0, vshfm);
      dst1 = VSHF_UB(dst1, src1, vshfm);
      dst2 = VSHF_UB(dst2, src2, vshfm);
      dst3 = VSHF_UB(dst3, src3, vshfm);
      dst4 = VSHF_UB(dst4, src4, vshfm);
      dst5 = VSHF_UB(dst5, src5, vshfm);
      ILVR_D3_UB(dst1, dst0, dst3, dst2, dst5, dst4, dst0, dst1, dst2);
      ST_UB3(dst0, dst1, dst2, destination, 16);
    } else if (pixelsPerRow & 16) {
      LD_UB4(source, 16, src0, src1, src2, src3);
      CEQI_B4_UB(src0, src1, src2, src3, 0, src0A, src1A, src2A, src3A);
      src0A = __msa_bmnz_v(src0, alphaMask, src0A);
      src1A = __msa_bmnz_v(src1, alphaMask, src1A);
      src2A = __msa_bmnz_v(src2, alphaMask, src2A);
      src3A = __msa_bmnz_v(src3, alphaMask, src3A);
      AND_V4_UB(src0A, src1A, src2A, src3A, alphaMask, src0A, src1A, src2A,
                src3A);
      src0A = SLDI_UB(src0A, src0A, 3);
      src1A = SLDI_UB(src1A, src1A, 3);
      src2A = SLDI_UB(src2A, src2A, 3);
      src3A = SLDI_UB(src3A, src3A, 3);
      AND_V4_UB(src0, src1, src2, src3, vCnst255, src0R, src1R, src2R, src3R);
      FFINTU_W4_SP(src0A, src1A, src2A, src3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
      FFINTU_W4_SP(src0R, src1R, src2R, src3R, fsrc0R, fsrc1R, fsrc2R, fsrc3R);
      DIV4(vfCnst255, fsrc0A, vfCnst255, fsrc1A, vfCnst255, fsrc2A, vfCnst255,
           fsrc3A, fsrc0A, fsrc1A, fsrc2A, fsrc3A);
      MUL4(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fsrc2R, fsrc2A, fsrc3R, fsrc3A,
           fdst0R, fdst1R, fdst2R, fdst3R);
      FTRUNCU_W4_UB(fdst0R, fdst1R, fdst2R, fdst3R, dst0, dst1, dst2, dst3);
      dst0 = VSHF_UB(dst0, src0, vshfm);
      dst1 = VSHF_UB(dst1, src1, vshfm);
      dst2 = VSHF_UB(dst2, src2, vshfm);
      dst3 = VSHF_UB(dst3, src3, vshfm);
      ILVR_D2_UB(dst1, dst0, dst3, dst2, dst0, dst1);
      ST_UB2(dst0, dst1, destination, 16);
    } else if (pixelsPerRow & 8) {
      LD_UB2(source, 16, src0, src1);
      CEQI_B2_UB(src0, src1, 0, src0A, src1A);
      src0A = __msa_bmnz_v(src0, alphaMask, src0A);
      src1A = __msa_bmnz_v(src1, alphaMask, src1A);
      AND_V2_UB(src0A, src1A, alphaMask, src0A, src1A);
      src0A = SLDI_UB(src0A, src0A, 3);
      src1A = SLDI_UB(src1A, src1A, 3);
      AND_V2_UB(src0, src1, vCnst255, src0R, src1R);
      FFINTU_W2_SP(src0A, src1A, fsrc0A, fsrc1A);
      FFINTU_W2_SP(src0R, src1R, fsrc0R, fsrc1R);
      DIV2(vfCnst255, fsrc0A, vfCnst255, fsrc1A, fsrc0A, fsrc1A);
      MUL2(fsrc0R, fsrc0A, fsrc1R, fsrc1A, fdst0R, fdst1R);
      FTRUNCU_W2_UB(fdst0R, fdst1R, dst0, dst1);
      dst0 = VSHF_UB(dst0, src0, vshfm);
      dst1 = VSHF_UB(dst1, src1, vshfm);
      dst0 = (v16u8)__msa_ilvr_d((v2i64)dst1, (v2i64)dst0);
      ST_UB(dst0, destination);
      destination += 16;
    }
  }

  pixelsPerRow &= 7;
}

}  // namespace simd

}  // namespace blink

#endif  // HAVE_MIPS_MSA_INTRINSICS

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CPU_MIPS_WEBGL_IMAGE_CONVERSION_MSA_H_
