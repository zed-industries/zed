// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COLOR_SPACE_GAMUT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COLOR_SPACE_GAMUT_H_

#include "third_party/blink/renderer/platform/platform_export.h"

struct skcms_ICCProfile;

namespace display {
struct ScreenInfo;
}

namespace blink {

enum class ColorSpaceGamut {
  // Values synced with 'Gamut' in src/tools/metrics/histograms/histograms.xml
  kUnknown = 0,
  kLessThanNTSC = 1,
  NTSC = 2,
  SRGB = 3,
  kAlmostP3 = 4,
  P3 = 5,
  kAdobeRGB = 6,
  kWide = 7,
  BT2020 = 8,
  kProPhoto = 9,
  kUltraWide = 10,
  kEnd
};

namespace color_space_utilities {

PLATFORM_EXPORT ColorSpaceGamut GetColorSpaceGamut(const display::ScreenInfo&);
ColorSpaceGamut GetColorSpaceGamut(const skcms_ICCProfile*);

}  // namespace color_space_utilities

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COLOR_SPACE_GAMUT_H_
