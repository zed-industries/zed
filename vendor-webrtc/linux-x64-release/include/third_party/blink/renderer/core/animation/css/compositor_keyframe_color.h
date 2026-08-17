// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_COLOR_H_

#include "third_party/blink/renderer/core/animation/css/compositor_keyframe_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/skia/include/core/SkColor.h"

namespace blink {

class CORE_EXPORT CompositorKeyframeColor final
    : public CompositorKeyframeValue {
 public:
  CompositorKeyframeColor(SkColor color) : color_(color) {}
  ~CompositorKeyframeColor() override = default;

  SkColor ToColor() const { return color_; }

 private:
  Type GetType() const override { return Type::kColor; }

  SkColor color_;
};

template <>
struct DowncastTraits<CompositorKeyframeColor> {
  static bool AllowFrom(const CompositorKeyframeValue& value) {
    return value.IsColor();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_COLOR_H_
