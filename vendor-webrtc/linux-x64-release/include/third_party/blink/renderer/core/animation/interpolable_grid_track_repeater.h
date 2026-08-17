// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_TRACK_REPEATER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_TRACK_REPEATER_H_

#include <memory>
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/css/css_to_length_conversion_data.h"
#include "third_party/blink/renderer/core/style/grid_track_list.h"

namespace blink {

class CSSProperty;

// Represents a blink::NGGridTrackRepeater, converted into a form that can be
// interpolated from/to.
class CORE_EXPORT InterpolableGridTrackRepeater final
    : public InterpolableValue {
 public:
  InterpolableGridTrackRepeater(InterpolableList* values,
                                const NGGridTrackRepeater& repeater);
  static InterpolableGridTrackRepeater* Create(
      const NGGridTrackRepeater& repeater,
      const Vector<GridTrackSize, 1>& repeater_track_sizes,
      const CSSProperty& property,
      float zoom);

  Vector<GridTrackSize, 1> CreateTrackSizes(
      const CSSToLengthConversionData& conversion_data) const;

  wtf_size_t RepeatSize() const { return repeater_.repeat_size; }
  wtf_size_t RepeatCount() const { return repeater_.repeat_count; }
  NGGridTrackRepeater::RepeatType RepeatType() const {
    return repeater_.repeat_type;
  }

  // InterpolableValue implementation:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsGridTrackRepeater() const final { return true; }
  bool Equals(const InterpolableValue& other) const final;
  void Scale(double scale) final;
  void Add(const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  // Interpolable grid track repeaters are compatible when the lengths of the
  // values and their |NGGridTrackRepeater| variable are equal. Two
  // |NGGridTrackRepeater| variables are equal when their index, size, count and
  // type are the same. If two grid track repeaters are not compatible, then
  // they combine discretely.
  bool IsCompatibleWith(const InterpolableValue& other) const;

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(values_);
  }

 private:
  InterpolableGridTrackRepeater* RawClone() const final;
  InterpolableGridTrackRepeater* RawCloneAndZero() const final;

  // Stores the track sizes of a repeater.
  Member<InterpolableList> values_;
  NGGridTrackRepeater repeater_;
};

template <>
struct DowncastTraits<InterpolableGridTrackRepeater> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsGridTrackRepeater();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_TRACK_REPEATER_H_
