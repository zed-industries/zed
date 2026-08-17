// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_TRACK_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_TRACK_LIST_H_

#include <memory>
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/core/css/css_to_length_conversion_data.h"
#include "third_party/blink/renderer/core/style/grid_track_list.h"

namespace blink {

class CSSProperty;

// Represents a blink::NGGridTrackList, converted into a form that can be
// interpolated from/to.
class CORE_EXPORT InterpolableGridTrackList : public InterpolableValue {
 public:
  InterpolableGridTrackList(InterpolableList* values, double progress);
  static InterpolableGridTrackList* MaybeCreate(
      const NGGridTrackList& track_list,
      const CSSProperty& property,
      float zoom);

  NGGridTrackList CreateNGGridTrackList(
      const CSSToLengthConversionData& conversion_data) const;

  // InterpolableValue implementation:
  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;
  bool IsGridTrackList() const final { return true; }
  bool Equals(const InterpolableValue& other) const final;
  void Scale(double scale) final;
  void Add(const InterpolableValue& other) final;
  void AssertCanInterpolateWith(const InterpolableValue& other) const final;

  // Two grid track lists are compatible when they have the same number of
  // tracks and each of the |InterpolableGridTrackRepeater| values are equal. If
  // two grid track lists are not compatible, then they combine discretely.
  bool IsCompatibleWith(const InterpolableValue& other) const;
  double GetProgress() const { return progress_; }

  void Trace(Visitor* v) const override {
    InterpolableValue::Trace(v);
    v->Trace(values_);
  }

 private:
  InterpolableGridTrackList* RawClone() const final;
  InterpolableGridTrackList* RawCloneAndZero() const final;

  // Represents a list of repeaters.
  Member<InterpolableList> values_;
  // Represents the progress of the interpolation, this is needed to flip
  // |CSSGridTrackListNonInterpolableValue|.
  double progress_;
};

template <>
struct DowncastTraits<InterpolableGridTrackList> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsGridTrackList();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_GRID_TRACK_LIST_H_
