// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_DOUBLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_DOUBLE_H_

#include "third_party/blink/renderer/core/animation/css/compositor_keyframe_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT CompositorKeyframeDouble final
    : public CompositorKeyframeValue {
 public:
  CompositorKeyframeDouble(double number) : number_(number) {}
  ~CompositorKeyframeDouble() override = default;

  double ToDouble() const { return number_; }

 private:
  Type GetType() const override { return Type::kDouble; }

  double number_;
};

template <>
struct DowncastTraits<CompositorKeyframeDouble> {
  static bool AllowFrom(const CompositorKeyframeValue& value) {
    return value.IsDouble();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_DOUBLE_H_
