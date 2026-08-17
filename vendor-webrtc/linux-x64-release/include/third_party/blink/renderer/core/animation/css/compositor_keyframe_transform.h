// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_TRANSFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_TRANSFORM_H_

#include "third_party/blink/renderer/core/animation/css/compositor_keyframe_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/transforms/transform_operations.h"

namespace blink {

class CORE_EXPORT CompositorKeyframeTransform final
    : public CompositorKeyframeValue {
 public:
  explicit CompositorKeyframeTransform(const TransformOperations& transform,
                                       double zoom)
      : transform_(transform), zoom_(zoom) {}
  ~CompositorKeyframeTransform() override = default;

  void Trace(Visitor* visitor) const final {
    CompositorKeyframeValue::Trace(visitor);
    visitor->Trace(transform_);
  }

  const TransformOperations& GetTransformOperations() const {
    return transform_;
  }
  double Zoom() const { return zoom_; }

 private:
  Type GetType() const override { return Type::kTransform; }

  const TransformOperations transform_;
  const double zoom_;
};

template <>
struct DowncastTraits<CompositorKeyframeTransform> {
  static bool AllowFrom(const CompositorKeyframeValue& value) {
    return value.IsTransform();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_TRANSFORM_H_
