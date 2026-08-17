// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PREDEFINED_COLOR_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PREDEFINED_COLOR_SPACE_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkColorSpace.h"
#include "third_party/skia/include/core/SkRefCnt.h"

namespace gfx {
class ColorSpace;
}

namespace blink {

enum class PredefinedColorSpace {
  kSRGB,
  kRec2020,
  kP3,
  kRec2100HLG,
  kRec2100PQ,
  kSRGBLinear,
};

// Return the gfx::ColorSpace or SkColorSpace for a PredefinedColorSpace.
PLATFORM_EXPORT gfx::ColorSpace PredefinedColorSpaceToGfxColorSpace(
    PredefinedColorSpace color_space);
PLATFORM_EXPORT sk_sp<SkColorSpace> PredefinedColorSpaceToSkColorSpace(
    PredefinedColorSpace color_space);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PREDEFINED_COLOR_SPACE_H_
