// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_PALETTE_INTERPOLATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_PALETTE_INTERPOLATION_H_

#include <optional>

#include "third_party/blink/renderer/platform/fonts/font_palette.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "third_party/skia/include/core/SkTypeface.h"

namespace blink {

class PLATFORM_EXPORT PaletteInterpolation {
 public:
  explicit PaletteInterpolation(sk_sp<SkTypeface> typeface)
      : typeface_(typeface) {}
  std::optional<uint16_t> RetrievePaletteIndex(
      const FontPalette* palette) const;
  Vector<FontPalette::FontPaletteOverride> ComputeInterpolableFontPalette(
      const FontPalette* palette) const;

 private:
  Vector<FontPalette::FontPaletteOverride> RetrieveColorRecords(
      const FontPalette* palette,
      unsigned int palette_index) const;
  static Vector<FontPalette::FontPaletteOverride> MixColorRecords(
      Vector<FontPalette::FontPaletteOverride>&& start_color_records,
      Vector<FontPalette::FontPaletteOverride>&& end_color_records,
      double percentage,
      double alpha_multiplier,
      Color::ColorSpace color_interpolation_space,
      std::optional<Color::HueInterpolationMethod> hue_interpolation_method);
  sk_sp<SkTypeface> typeface_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_PALETTE_INTERPOLATION_H_
