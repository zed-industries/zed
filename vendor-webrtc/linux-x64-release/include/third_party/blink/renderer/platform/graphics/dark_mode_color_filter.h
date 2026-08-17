// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_COLOR_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_COLOR_FILTER_H_

#include <memory>

#include "third_party/blink/renderer/platform/graphics/dark_mode_settings.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkColor.h"
#include "third_party/skia/include/core/SkRefCnt.h"

namespace cc {
class ColorFilter;
}

namespace blink {

// Contains logic specific to modifying colors drawn when dark mode is active.
class PLATFORM_EXPORT DarkModeColorFilter {
 public:
  static std::unique_ptr<DarkModeColorFilter> FromSettings(
      const DarkModeSettings& settings);

  virtual ~DarkModeColorFilter();
  virtual SkColor4f InvertColor(const SkColor4f& color) const = 0;
  virtual sk_sp<cc::ColorFilter> ToColorFilter() const = 0;
  virtual SkColor4f AdjustColorForHigherConstrast(
      const SkColor4f& adjusted_color,
      const SkColor4f& background,
      float reference_contrast_ratio) {
    return adjusted_color;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_COLOR_FILTER_H_
