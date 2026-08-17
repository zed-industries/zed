// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_TRACK_SIZE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_TRACK_SIZE_H_

#include <memory>
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/css/css_to_length_conversion_data.h"
#include "third_party/blink/renderer/core/style/grid_track_size.h"

namespace blink {

// Represents a blink::GridTrackSize, converted into a form that can be
// interpolated from/to.
// This class is a representation of <track-size> values:
// <track-breadth> | minmax( <inflexible-breadth> , <track-breadth> ) |
// fit-content( <length-percentage> )
class CORE_EXPORT InterpolableGridTrackSize final : public InterpolableValue {
 public:
  InterpolableGridTrackSize(InterpolableValue* min_value,
                            InterpolableValue* max_value,
                            const GridTrackSizeType type);
  static InterpolableGridTrackSize* Create(const GridTrackSize& grid_track_size,
                                           const CSSProperty& property,
                                           float zoom);

  GridTrackSize CreateTrackSize(
      const CSSToLengthConversionData& conversion_data) const;

  // InterpolableValue implementation:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsGridTrackSize() const final { return true; }
  bool Equals(const InterpolableValue& other) const final;
  void Scale(double scale) final;
  void Add(const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(min_value_);
    v->Trace(max_value_);
  }

 private:
  InterpolableGridTrackSize* RawClone() const final;
  InterpolableGridTrackSize* RawCloneAndZero() const final;

  // We have a min and max representation as a generalization of the three
  // different <track-size> types.
  Member<InterpolableValue> min_value_;
  Member<InterpolableValue> max_value_;
  GridTrackSizeType type_;
};

template <>
struct DowncastTraits<InterpolableGridTrackSize> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsGridTrackSize();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_TRACK_SIZE_H_
