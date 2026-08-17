// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_DYNAMIC_RANGE_LIMIT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_DYNAMIC_RANGE_LIMIT_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context_types.h"

namespace blink {

class CORE_EXPORT InterpolableDynamicRangeLimit final
    : public InterpolableValue {
 public:
  explicit InterpolableDynamicRangeLimit(DynamicRangeLimit mix_value);

  static InterpolableDynamicRangeLimit* Create(
      DynamicRangeLimit dynamic_range_limit);

  DynamicRangeLimit GetDynamicRangeLimit() const;

  // InterpolableValue implementation:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsDynamicRangeLimit() const final { return true; }
  bool Equals(const InterpolableValue& other) const final;
  // dynamic-range-limit is not additive, therefore, as with FontPalette,
  // Scale() should not affect anything and Add() should work as a replacement.
  void Scale(double scale) final {}
  void Add(const InterpolableValue& other) final {
    dynamic_range_limit_ =
        To<InterpolableDynamicRangeLimit>(other).dynamic_range_limit_;
  }
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

 private:
  InterpolableDynamicRangeLimit* RawClone() const final;
  InterpolableDynamicRangeLimit* RawCloneAndZero() const final;

  DynamicRangeLimit dynamic_range_limit_;
};

template <>
struct DowncastTraits<InterpolableDynamicRangeLimit> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsDynamicRangeLimit();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_DYNAMIC_RANGE_LIMIT_H_
