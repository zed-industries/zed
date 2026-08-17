/*
 * Copyright (C) 2012 Gabor Rapcsanyi (rgabor@inf.u-szeged.hu), University of
 * Szeged
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY UNIVERSITY OF SZEGED ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL UNIVERSITY OF SZEGED OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CPU_ARM_WEBGL_IMAGE_CONVERSION_NEON_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CPU_ARM_WEBGL_IMAGE_CONVERSION_NEON_H_

#include "base/compiler_specific.h"
#include "build/build_config.h"

#if defined(CPU_ARM_NEON)

#include <arm_neon.h>

namespace blink {

namespace simd {

ALWAYS_INLINE void UnpackOneRowOfRGBA16LittleToRGBA8(const uint16_t*& source,
                                                     uint8_t*& destination,
                                                     unsigned& pixels_per_row) {
  unsigned components_per_row = pixels_per_row * 4;
  unsigned tail_components = components_per_row % 16;
  unsigned components_size = components_per_row - tail_components;
  const uint8_t* src = reinterpret_cast<const uint8_t*>(source);

  for (unsigned i = 0; i < components_size; i += 16) {
    uint8x16x2_t components = vld2q_u8(src + i * 2);
    vst1q_u8(destination + i, components.val[1]);
  }

  source += components_size;
  destination += components_size;
  pixels_per_row = tail_components / 4;
}

ALWAYS_INLINE void UnpackOneRowOfRGB16LittleToRGBA8(const uint16_t*& source,
                                                    uint8_t*& destination,
                                                    unsigned& pixels_per_row) {
  unsigned components_per_row = pixels_per_row * 3;
  unsigned tail_components = components_per_row % 24;
  unsigned components_size = components_per_row - tail_components;

  uint8x8_t component_a = vdup_n_u8(0xFF);
  for (unsigned i = 0; i < components_size; i += 24) {
    uint16x8x3_t rgb16 = vld3q_u16(source + i);
    uint8x8_t component_r = vqmovn_u16(vshrq_n_u16(rgb16.val[0], 8));
    uint8x8_t component_g = vqmovn_u16(vshrq_n_u16(rgb16.val[1], 8));
    uint8x8_t component_b = vqmovn_u16(vshrq_n_u16(rgb16.val[2], 8));
    uint8x8x4_t rgba8 = {{component_r, component_g, component_b, component_a}};
    vst4_u8(destination, rgba8);
    destination += 32;
  }

  source += components_size;
  pixels_per_row = tail_components / 3;
}

ALWAYS_INLINE void UnpackOneRowOfARGB16LittleToRGBA8(const uint16_t*& source,
                                                     uint8_t*& destination,
                                                     unsigned& pixels_per_row) {
  unsigned components_per_row = pixels_per_row * 4;
  unsigned tail_components = components_per_row % 32;
  unsigned components_size = components_per_row - tail_components;

  for (unsigned i = 0; i < components_size; i += 32) {
    uint16x8x4_t argb16 = vld4q_u16(source + i);
    uint8x8_t component_a = vqmovn_u16(vshrq_n_u16(argb16.val[0], 8));
    uint8x8_t component_r = vqmovn_u16(vshrq_n_u16(argb16.val[1], 8));
    uint8x8_t component_g = vqmovn_u16(vshrq_n_u16(argb16.val[2], 8));
    uint8x8_t component_b = vqmovn_u16(vshrq_n_u16(argb16.val[3], 8));
    uint8x8x4_t rgba8 = {{component_r, component_g, component_b, component_a}};
    vst4_u8(destination + i, rgba8);
  }

  source += components_size;
  destination += components_size;
  pixels_per_row = tail_components / 4;
}

ALWAYS_INLINE void UnpackOneRowOfBGRA16LittleToRGBA8(const uint16_t*& source,
                                                     uint8_t*& destination,
                                                     unsigned& pixels_per_row) {
  unsigned components_per_row = pixels_per_row * 4;
  unsigned tail_components = components_per_row % 32;
  unsigned components_size = components_per_row - tail_components;

  for (unsigned i = 0; i < components_size; i += 32) {
    uint16x8x4_t argb16 = vld4q_u16(source + i);
    uint8x8_t component_b = vqmovn_u16(vshrq_n_u16(argb16.val[0], 8));
    uint8x8_t component_g = vqmovn_u16(vshrq_n_u16(argb16.val[1], 8));
    uint8x8_t component_r = vqmovn_u16(vshrq_n_u16(argb16.val[2], 8));
    uint8x8_t component_a = vqmovn_u16(vshrq_n_u16(argb16.val[3], 8));
    uint8x8x4_t rgba8 = {{component_r, component_g, component_b, component_a}};
    vst4_u8(destination + i, rgba8);
  }

  source += components_size;
  destination += components_size;
  pixels_per_row = tail_components / 4;
}

ALWAYS_INLINE void UnpackOneRowOfRGBA4444ToRGBA8(const uint16_t*& source,
                                                 uint8_t*& destination,
                                                 unsigned& pixels_per_row) {
  unsigned tail_pixels = pixels_per_row % 8;
  unsigned pixel_size = pixels_per_row - tail_pixels;

  uint16x8_t immediate0x0f = vdupq_n_u16(0x0F);
  for (unsigned i = 0; i < pixel_size; i += 8) {
    uint16x8_t eight_pixels = vld1q_u16(source + i);

    uint8x8_t component_r = vqmovn_u16(vshrq_n_u16(eight_pixels, 12));
    uint8x8_t component_g =
        vqmovn_u16(vandq_u16(vshrq_n_u16(eight_pixels, 8), immediate0x0f));
    uint8x8_t component_b =
        vqmovn_u16(vandq_u16(vshrq_n_u16(eight_pixels, 4), immediate0x0f));
    uint8x8_t component_a = vqmovn_u16(vandq_u16(eight_pixels, immediate0x0f));

    component_r = vorr_u8(vshl_n_u8(component_r, 4), component_r);
    component_g = vorr_u8(vshl_n_u8(component_g, 4), component_g);
    component_b = vorr_u8(vshl_n_u8(component_b, 4), component_b);
    component_a = vorr_u8(vshl_n_u8(component_a, 4), component_a);

    uint8x8x4_t dest_components = {
        {component_r, component_g, component_b, component_a}};
    vst4_u8(destination, dest_components);
    destination += 32;
  }

  source += pixel_size;
  pixels_per_row = tail_pixels;
}

ALWAYS_INLINE void PackOneRowOfRGBA8ToUnsignedShort4444(
    const uint8_t*& source,
    uint16_t*& destination,
    unsigned& pixels_per_row) {
  unsigned components_per_row = pixels_per_row * 4;
  unsigned tail_components = components_per_row % 32;
  unsigned components_size = components_per_row - tail_components;

  uint8_t* dst = reinterpret_cast<uint8_t*>(destination);
  uint8x8_t immediate0xf0 = vdup_n_u8(0xF0);
  for (unsigned i = 0; i < components_size; i += 32) {
    uint8x8x4_t rgba8 = vld4_u8(source + i);

    uint8x8_t component_r = vand_u8(rgba8.val[0], immediate0xf0);
    uint8x8_t component_g = vshr_n_u8(vand_u8(rgba8.val[1], immediate0xf0), 4);
    uint8x8_t component_b = vand_u8(rgba8.val[2], immediate0xf0);
    uint8x8_t component_a = vshr_n_u8(vand_u8(rgba8.val[3], immediate0xf0), 4);

    uint8x8x2_t rgba4;
    rgba4.val[0] = vorr_u8(component_b, component_a);
    rgba4.val[1] = vorr_u8(component_r, component_g);
    vst2_u8(dst, rgba4);
    dst += 16;
  }

  source += components_size;
  destination += components_size / 4;
  pixels_per_row = tail_components / 4;
}

ALWAYS_INLINE void UnpackOneRowOfRGBA5551ToRGBA8(const uint16_t*& source,
                                                 uint8_t*& destination,
                                                 unsigned& pixels_per_row) {
  unsigned tail_pixels = pixels_per_row % 8;
  unsigned pixel_size = pixels_per_row - tail_pixels;

  uint8x8_t immediate0x7 = vdup_n_u8(0x7);
  uint8x8_t immediate0xff = vdup_n_u8(0xFF);
  uint16x8_t immediate0x1f = vdupq_n_u16(0x1F);
  uint16x8_t immediate0x1 = vdupq_n_u16(0x1);

  for (unsigned i = 0; i < pixel_size; i += 8) {
    uint16x8_t eight_pixels = vld1q_u16(source + i);

    uint8x8_t component_r = vqmovn_u16(vshrq_n_u16(eight_pixels, 11));
    uint8x8_t component_g =
        vqmovn_u16(vandq_u16(vshrq_n_u16(eight_pixels, 6), immediate0x1f));
    uint8x8_t component_b =
        vqmovn_u16(vandq_u16(vshrq_n_u16(eight_pixels, 1), immediate0x1f));
    uint8x8_t component_a = vqmovn_u16(vandq_u16(eight_pixels, immediate0x1));

    component_r =
        vorr_u8(vshl_n_u8(component_r, 3), vand_u8(component_r, immediate0x7));
    component_g =
        vorr_u8(vshl_n_u8(component_g, 3), vand_u8(component_g, immediate0x7));
    component_b =
        vorr_u8(vshl_n_u8(component_b, 3), vand_u8(component_b, immediate0x7));
    component_a = vmul_u8(component_a, immediate0xff);

    uint8x8x4_t dest_components = {
        {component_r, component_g, component_b, component_a}};
    vst4_u8(destination, dest_components);
    destination += 32;
  }

  source += pixel_size;
  pixels_per_row = tail_pixels;
}

ALWAYS_INLINE void PackOneRowOfRGBA8ToUnsignedShort5551(
    const uint8_t*& source,
    uint16_t*& destination,
    unsigned& pixels_per_row) {
  unsigned components_per_row = pixels_per_row * 4;
  unsigned tail_components = components_per_row % 32;
  unsigned components_size = components_per_row - tail_components;

  uint8_t* dst = reinterpret_cast<uint8_t*>(destination);

  uint8x8_t immediate0xf8 = vdup_n_u8(0xF8);
  uint8x8_t immediate0x18 = vdup_n_u8(0x18);
  for (unsigned i = 0; i < components_size; i += 32) {
    uint8x8x4_t rgba8 = vld4_u8(source + i);

    uint8x8_t component_r = vand_u8(rgba8.val[0], immediate0xf8);
    uint8x8_t component_g3bit = vshr_n_u8(rgba8.val[1], 5);

    uint8x8_t component_g2bit =
        vshl_n_u8(vand_u8(rgba8.val[1], immediate0x18), 3);
    uint8x8_t component_b = vshr_n_u8(vand_u8(rgba8.val[2], immediate0xf8), 2);
    uint8x8_t component_a = vshr_n_u8(rgba8.val[3], 7);

    uint8x8x2_t rgba5551;
    rgba5551.val[0] =
        vorr_u8(vorr_u8(component_g2bit, component_b), component_a);
    rgba5551.val[1] = vorr_u8(component_r, component_g3bit);
    vst2_u8(dst, rgba5551);
    dst += 16;
  }

  source += components_size;
  destination += components_size / 4;
  pixels_per_row = tail_components / 4;
}

ALWAYS_INLINE void PackOneRowOfRGBA8ToUnsignedShort565(
    const uint8_t*& source,
    uint16_t*& destination,
    unsigned& pixels_per_row) {
  unsigned components_per_row = pixels_per_row * 4;
  unsigned tail_components = components_per_row % 32;
  unsigned components_size = components_per_row - tail_components;
  uint8_t* dst = reinterpret_cast<uint8_t*>(destination);

  uint8x8_t immediate0xf8 = vdup_n_u8(0xF8);
  uint8x8_t immediate0x1c = vdup_n_u8(0x1C);
  for (unsigned i = 0; i < components_size; i += 32) {
    uint8x8x4_t rgba8 = vld4_u8(source + i);

    uint8x8_t component_r = vand_u8(rgba8.val[0], immediate0xf8);
    uint8x8_t component_g_left = vshr_n_u8(rgba8.val[1], 5);
    uint8x8_t component_g_right =
        vshl_n_u8(vand_u8(rgba8.val[1], immediate0x1c), 3);
    uint8x8_t component_b = vshr_n_u8(vand_u8(rgba8.val[2], immediate0xf8), 3);

    uint8x8x2_t rgb565;
    rgb565.val[0] = vorr_u8(component_g_right, component_b);
    rgb565.val[1] = vorr_u8(component_r, component_g_left);
    vst2_u8(dst, rgb565);
    dst += 16;
  }

  source += components_size;
  destination += components_size / 4;
  pixels_per_row = tail_components / 4;
}

}  // namespace simd

}  // namespace blink

#endif  // CPU_ARM_NEON

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CPU_ARM_WEBGL_IMAGE_CONVERSION_NEON_H_
