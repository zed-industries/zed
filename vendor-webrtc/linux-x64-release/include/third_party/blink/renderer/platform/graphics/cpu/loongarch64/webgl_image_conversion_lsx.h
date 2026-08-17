// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CPU_LOONGARCH64_WEBGL_IMAGE_CONVERSION_LSX_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CPU_LOONGARCH64_WEBGL_IMAGE_CONVERSION_LSX_H_

#include "build/build_config.h"

#if defined(ARCH_CPU_LOONGARCH_FAMILY)
#include <lsxintrin.h>

namespace blink {

namespace simd {

ALWAYS_INLINE  void UnpackOneRowOfRGBA4444LittleToRGBA8(
    const uint16_t*& source,
    uint8_t*& destination,
    unsigned& pixels_per_row) {
  v8u16 immediate0x0f = __lsx_vreplgr2vr_h(0x0F);
  unsigned pixels_per_row_trunc = (pixels_per_row / 8) * 8;
  for (unsigned i = 0; i < pixels_per_row_trunc; i += 8) {
    v8u16 packed_value =  *((v8u16*)(source));
    __m128i r = __lsx_vsrli_h(packed_value, 12);
    __m128i g = __lsx_vand_v(__lsx_vsrli_h(packed_value, 8), immediate0x0f);
    __m128i b = __lsx_vand_v(__lsx_vsrli_h(packed_value, 4), immediate0x0f);
    __m128i a = __lsx_vand_v(packed_value, immediate0x0f);
    __m128i component_r = __lsx_vor_v(__lsx_vslli_b(r, 4), r);
    __m128i component_g = __lsx_vor_v(__lsx_vslli_b(g, 4), g);
    __m128i component_b = __lsx_vor_v(__lsx_vslli_b(b, 4), b);
    __m128i component_a = __lsx_vor_v(__lsx_vslli_b(a, 4), a);

    __m128i component_rb = __lsx_vpackev_b(component_b, component_r);
    __m128i component_ga = __lsx_vpackev_b(component_a, component_g);
    __m128i component_rgba1 = __lsx_vilvl_b(component_ga, component_rb);
    __m128i component_rgba2 = __lsx_vilvh_b(component_ga, component_rb);

    __lsx_vst(component_rgba1, (void*)destination, 0);
    __lsx_vst(component_rgba2, (void*)destination, 16);

    source += 8;
    destination += 32;
  }
  pixels_per_row -= pixels_per_row_trunc;
}

ALWAYS_INLINE void UnpackOneRowOfRGBA5551LittleToRGBA8(
    const uint16_t*& source,
    uint8_t*& destination,
    unsigned& pixels_per_row) {
  __m128i immediate0x1f = __lsx_vreplgr2vr_h(0x1F);
  __m128i immediate0x7 = __lsx_vreplgr2vr_h(0x7);
  __m128i immediate0x1 = __lsx_vreplgr2vr_h(0x1);
  unsigned pixels_per_row_trunc = (pixels_per_row / 8) * 8;
  for (unsigned i = 0; i < pixels_per_row_trunc; i += 8) {
    v8u16 packed_value =  *((v8u16*)(source));
    __m128i r = __lsx_vsrli_h(packed_value, 11);
    __m128i g = __lsx_vand_v(__lsx_vsrli_h(packed_value, 6), immediate0x1f);
    __m128i b = __lsx_vand_v(__lsx_vsrli_h(packed_value, 1), immediate0x1f);
    __m128i component_r =
        __lsx_vor_v(__lsx_vslli_b(r, 3), __lsx_vand_v(r, immediate0x7));
    __m128i component_g =
        __lsx_vor_v(__lsx_vslli_b(g, 3), __lsx_vand_v(g, immediate0x7));
    __m128i component_b =
        __lsx_vor_v(__lsx_vslli_b(b, 3), __lsx_vand_v(b, immediate0x7));
    __m128i component_a = __lsx_vseq_h(
        __lsx_vand_v(packed_value, immediate0x1), immediate0x1);

    __m128i component_rb = __lsx_vpackev_b(component_b, component_r);
    __m128i component_ga = __lsx_vpackev_b(component_a, component_g);
    __m128i component_rgba1 = __lsx_vilvl_b(component_ga, component_rb);
    __m128i component_rgba2 = __lsx_vilvh_b(component_ga, component_rb);

    __lsx_vst(component_rgba1, (void*)destination, 0);
    __lsx_vst(component_rgba2, (void*)destination, 16);

    source += 8;
    destination += 32;
  }
  pixels_per_row -= pixels_per_row_trunc;
}

ALWAYS_INLINE void PackOneRowOfRGBA8LittleToRA8(const uint8_t*& source,
                                                uint8_t*& destination,
                                                unsigned& pixels_per_row) {

  unsigned pixels_per_row_trunc = (pixels_per_row / 4) * 4;
  v16u8 mask_zero = __lsx_vldi(0);
  v4u32 mask_lalpha = __lsx_vreplgr2vr_w(0x0ff);
  v4f32 mask_falpha = __lsx_vffint_s_w(mask_lalpha);
  v16u8 ra_index = {0,19, 4,23, 8,27, 12,31};
  for (unsigned i = 0; i < pixels_per_row_trunc; i += 4) {
    v16u8 bgra = *((__m128i*)(source));
    //if A !=0, A=0; else A=0xFF
    v4f32 alpha_factor = __lsx_vseq_b(bgra, mask_zero);
    //if A!=0, A=A; else A=0xFF
    alpha_factor = __lsx_vor_v(bgra, alpha_factor);
    alpha_factor = __lsx_vsrli_w(alpha_factor, 24);
    alpha_factor = __lsx_vffint_s_w(alpha_factor);
    alpha_factor = __lsx_vfdiv_s(mask_falpha, alpha_factor);

    v16u8 component_r = __lsx_vand_v(bgra, mask_lalpha);
    component_r = __lsx_vffint_s_w(component_r);
    component_r = __lsx_vfmul_s(component_r, alpha_factor);
    component_r = __lsx_vftintrz_w_s(component_r);

    v2u64 ra = __lsx_vshuf_b(bgra, component_r, ra_index);
    __lsx_vstelm_d(ra, destination, 0, 0);

    source += 16;
    destination += 8;
  }

  pixels_per_row -= pixels_per_row_trunc;
}

ALWAYS_INLINE void UnpackOneRowOfBGRA8LittleToRGBA8(const uint32_t*& source,
                                                    uint32_t*& destination,
                                                    unsigned& pixels_per_row) {
  __m128i bgra, rgba;
  unsigned pixels_per_row_trunc = (pixels_per_row / 4) * 4;
  for (unsigned i = 0; i < pixels_per_row_trunc; i += 4) {
    bgra = *((__m128i*)(source));
    rgba =  __lsx_vshuf4i_b(bgra, 0xc6);
    __lsx_vst(rgba, destination, 0);
    source += 4;
    destination += 4;
  }
  pixels_per_row -= pixels_per_row_trunc;
}

ALWAYS_INLINE void PackOneRowOfRGBA8LittleToR8(const uint8_t*& source,
                                               uint8_t*& destination,
                                               unsigned& pixels_per_row) {
  unsigned pixels_per_row_trunc = (pixels_per_row / 4) * 4;
  v16u8 mask_zero = __lsx_vldi(0);
  v4u32 mask_lalpha = __lsx_vreplgr2vr_w(0x0ff);
  v4f32 mask_falpha = __lsx_vffint_s_w(mask_lalpha);
  for (unsigned i = 0; i < pixels_per_row_trunc; i += 4) {
    v16u8 bgra = *((__m128i*)(source));
    //if A !=0, A=0; else A=0xFF
    v4f32 alpha_factor = __lsx_vseq_b(bgra, mask_zero);
    //if A!=0, A=A; else A=0xFF
    alpha_factor = __lsx_vor_v(bgra, alpha_factor);
    alpha_factor = __lsx_vsrli_w(alpha_factor, 24);
    alpha_factor = __lsx_vffint_s_w(alpha_factor);
    alpha_factor = __lsx_vfdiv_s(mask_falpha, alpha_factor);

    v16u8 component_r = __lsx_vand_v(bgra, mask_lalpha);
    component_r = __lsx_vffint_s_w(component_r);
    component_r = __lsx_vfmul_s(component_r, alpha_factor);
    component_r = __lsx_vftintrz_w_s(component_r);

    component_r = __lsx_vpickev_b(component_r, component_r);
    component_r = __lsx_vpickev_b(component_r, component_r);
    __lsx_vstelm_w(component_r, destination, 0, 0);

    source += 16;
    destination += 4;
  }

  pixels_per_row -= pixels_per_row_trunc;
}

ALWAYS_INLINE void PackOneRowOfRGBA8LittleToRGBA8(const uint8_t*& source,
                                                  uint8_t*& destination,
                                                  unsigned& pixels_per_row) {
  unsigned pixels_per_row_trunc = (pixels_per_row / 4) * 4;
  v16u8 mask_zero = __lsx_vldi(0);
  v4u32 mask_lalpha = __lsx_vreplgr2vr_w(0x0ff);
  v4f32 mask_falpha = __lsx_vffint_s_w(mask_lalpha);
  v16u8 rgba_index = {0,1,2,19, 4,5,6,23, 8,9,10,27, 12,13,14,31};
  for (unsigned i = 0; i < pixels_per_row_trunc; i += 4) {
    v16u8 bgra = *((__m128i*)(source));
    //if A !=0, A=0; else A=0xFF
    v4f32 alpha_factor = __lsx_vseq_b(bgra, mask_zero);
    //if A!=0, A=A; else A=0xFF
    alpha_factor = __lsx_vor_v(bgra, alpha_factor);
    alpha_factor = __lsx_vsrli_w(alpha_factor, 24);
    alpha_factor = __lsx_vffint_s_w(alpha_factor);
    alpha_factor = __lsx_vfdiv_s(mask_falpha, alpha_factor);

    v16u8 bgra_01 = __lsx_vilvl_b(mask_zero, bgra);
    v16u8 bgra_23 = __lsx_vilvh_b(mask_zero, bgra);
    v16u8 bgra_0 = __lsx_vilvl_b(mask_zero, bgra_01);
    v16u8 bgra_1 = __lsx_vilvh_b(mask_zero, bgra_01);
    v16u8 bgra_2 = __lsx_vilvl_b(mask_zero, bgra_23);
    v16u8 bgra_3 = __lsx_vilvh_b(mask_zero, bgra_23);

    bgra_0 = __lsx_vffint_s_w(bgra_0);
    bgra_1 = __lsx_vffint_s_w(bgra_1);
    bgra_2 = __lsx_vffint_s_w(bgra_2);
    bgra_3 = __lsx_vffint_s_w(bgra_3);

    v4f32 alpha_factor_0 = __lsx_vreplvei_w(alpha_factor, 0);
    v4f32 alpha_factor_1 = __lsx_vreplvei_w(alpha_factor, 1);
    v4f32 alpha_factor_2 = __lsx_vreplvei_w(alpha_factor, 2);
    v4f32 alpha_factor_3 = __lsx_vreplvei_w(alpha_factor, 3);

    bgra_0 = __lsx_vfmul_s(alpha_factor_0, bgra_0);
    bgra_1 = __lsx_vfmul_s(alpha_factor_1, bgra_1);
    bgra_2 = __lsx_vfmul_s(alpha_factor_2, bgra_2);
    bgra_3 = __lsx_vfmul_s(alpha_factor_3, bgra_3);

    bgra_0 = __lsx_vftintrz_w_s(bgra_0);
    bgra_1 = __lsx_vftintrz_w_s(bgra_1);
    bgra_2 = __lsx_vftintrz_w_s(bgra_2);
    bgra_3 = __lsx_vftintrz_w_s(bgra_3);

    bgra_01 = __lsx_vpickev_b(bgra_1, bgra_0);
    bgra_23 = __lsx_vpickev_b(bgra_3, bgra_2);

    v4u32 rgba = __lsx_vpickev_b(bgra_23, bgra_01);
    rgba = __lsx_vshuf_b(bgra, rgba, rgba_index);
    __lsx_vst(rgba, destination, 0);

    source += 16;
    destination += 16;
  }
  pixels_per_row -= pixels_per_row_trunc;
}

}  // namespace simd
}  // namespace blink

#endif  // ARCH_CPU_LOONGARCH_FAMILY

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CPU_LOONGARCH64_WEBGL_IMAGE_CONVERSION_LSX_H_
