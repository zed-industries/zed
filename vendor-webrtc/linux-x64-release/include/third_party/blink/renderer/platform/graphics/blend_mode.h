// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BLEND_MODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BLEND_MODE_H_

#include <stdint.h>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkBlendMode.h"

namespace WTF {
class String;
}

namespace blink {

enum CompositeOperator {
  kCompositeClear,
  kCompositeCopy,
  kCompositeSourceOver,
  kCompositeSourceIn,
  kCompositeSourceOut,
  kCompositeSourceAtop,
  kCompositeDestinationOver,
  kCompositeDestinationIn,
  kCompositeDestinationOut,
  kCompositeDestinationAtop,
  kCompositeXOR,
  kCompositePlusLighter
};

enum class BlendMode : uint8_t {
  kNormal,
  kMultiply,
  kScreen,
  kOverlay,
  kDarken,
  kLighten,
  kColorDodge,
  kColorBurn,
  kHardLight,
  kSoftLight,
  kDifference,
  kExclusion,
  kHue,
  kSaturation,
  kColor,
  kLuminosity,
  // The following is only used in CSS mix-blend-mode, and maps to a composite
  // operator. Canvas uses the same enum but the kPlusLighter is not a valid
  // canvas value. We should consider splitting the enums.
  kPlusLighter,

  kMaxBlendMode = kPlusLighter,
};

PLATFORM_EXPORT SkBlendMode ToSkBlendMode(CompositeOperator,
                                          BlendMode = BlendMode::kNormal);
PLATFORM_EXPORT SkBlendMode ToSkBlendMode(BlendMode);

PLATFORM_EXPORT WTF::String BlendModeToString(BlendMode);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_BLEND_MODE_H_
