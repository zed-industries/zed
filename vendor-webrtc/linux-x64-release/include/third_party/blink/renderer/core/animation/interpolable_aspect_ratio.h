// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_ASPECT_RATIO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_ASPECT_RATIO_H_

#include <memory>

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/style_aspect_ratio.h"

namespace blink {

// Represents a blink::StyleAspectRatio, converted into its logarithm for
// interpolation.
class CORE_EXPORT InterpolableAspectRatio final : public InterpolableValue {
 public:
  explicit InterpolableAspectRatio(const gfx::SizeF& ratio);
  explicit InterpolableAspectRatio(InterpolableValue* value) : value_(value) {}

  static InterpolableAspectRatio* MaybeCreate(const StyleAspectRatio&);

  gfx::SizeF GetRatio() const;

  // InterpolableValue implementation:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsAspectRatio() const final { return true; }
  bool Equals(const InterpolableValue& other) const final { NOTREACHED(); }
  void Scale(double scale) final;
  void Add(const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(value_);
  }

 private:
  InterpolableAspectRatio* RawClone() const final {
    return MakeGarbageCollected<InterpolableAspectRatio>(value_->Clone());
  }
  InterpolableAspectRatio* RawCloneAndZero() const final {
    return MakeGarbageCollected<InterpolableAspectRatio>(
        value_->CloneAndZero());
  }

  // Interpolable aspect ratio value is stored and interpolated as the log of
  // the real aspect ratio.
  Member<InterpolableValue> value_;
};

template <>
struct DowncastTraits<InterpolableAspectRatio> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsAspectRatio();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_ASPECT_RATIO_H_
