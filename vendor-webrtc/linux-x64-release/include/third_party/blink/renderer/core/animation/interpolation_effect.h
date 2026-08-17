// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_EFFECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_EFFECT_H_

#include "third_party/blink/renderer/core/animation/interpolation.h"
#include "third_party/blink/renderer/core/animation/keyframe.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/animation/timing_function.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// Stores all adjacent pairs of keyframes (represented by Interpolations) in a
// KeyframeEffectModel with keyframe offset data preprocessed for more efficient
// active keyframe pair sampling.
class CORE_EXPORT InterpolationEffect
    : public GarbageCollected<InterpolationEffect> {
 public:
  InterpolationEffect() : is_populated_(false) {}

  bool IsPopulated() const { return is_populated_; }
  void SetPopulated() { is_populated_ = true; }

  void Clear() {
    is_populated_ = false;
    interpolations_.clear();
  }

  void GetActiveInterpolations(double fraction,
                               TimingFunction::LimitDirection limit_direction,
                               HeapVector<Member<Interpolation>>&) const;

  void AddInterpolation(Interpolation* interpolation,
                        scoped_refptr<TimingFunction> easing,
                        double start,
                        double end,
                        double apply_from,
                        double apply_to) {
    interpolations_.push_back(MakeGarbageCollected<InterpolationRecord>(
        interpolation, std::move(easing), start, end, apply_from, apply_to));
  }

  void AddInterpolationsFromKeyframes(
      const PropertyHandle&,
      const Keyframe::PropertySpecificKeyframe& keyframe_a,
      const Keyframe::PropertySpecificKeyframe& keyframe_b,
      double apply_from,
      double apply_to);

  void AddStaticValuedInterpolation(
      const PropertyHandle& property,
      const Keyframe::PropertySpecificKeyframe& keyframe);

  void Trace(Visitor*) const;

 private:
  class InterpolationRecord final
      : public GarbageCollected<InterpolationRecord> {
   public:
    InterpolationRecord(Interpolation* interpolation,
                        scoped_refptr<TimingFunction> easing,
                        double start,
                        double end,
                        double apply_from,
                        double apply_to)
        : interpolation_(interpolation),
          easing_(std::move(easing)),
          start_(start),
          end_(end),
          apply_from_(apply_from),
          apply_to_(apply_to),
          is_static_(false) {}

    // When a range is not specified, we mark the interpolation as static.
    explicit InterpolationRecord(Interpolation* interpolation)
        : interpolation_(interpolation),
          start_(0),
          end_(1),
          apply_from_(0),
          apply_to_(1),
          is_static_(true) {}

    Member<Interpolation> interpolation_;
    scoped_refptr<TimingFunction> easing_;
    double start_;
    double end_;
    double apply_from_;
    double apply_to_;
    bool is_static_;

    void Trace(Visitor* visitor) const { visitor->Trace(interpolation_); }
  };

  bool is_populated_;
  HeapVector<Member<InterpolationRecord>> interpolations_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_EFFECT_H_
