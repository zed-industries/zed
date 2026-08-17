/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INERT_EFFECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INERT_EFFECT_H_

#include "third_party/blink/renderer/core/animation/animation_effect.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect_model.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class AnimationProxy {
 public:
  virtual bool AtScrollTimelineBoundary() const = 0;
  virtual std::optional<AnimationTimeDelta> TimelineDuration() const = 0;
  virtual AnimationTimeDelta IntrinsicIterationDuration() const = 0;
  virtual double PlaybackRate() const = 0;
  virtual bool Paused() const = 0;
  virtual std::optional<AnimationTimeDelta> InheritedTime() const = 0;
};

// Lightweight subset of KeyframeEffect.
// Used to transport data for deferred KeyframeEffect construction and one off
// Interpolation sampling.
class CORE_EXPORT InertEffect final : public AnimationEffect {
 public:
  InertEffect(KeyframeEffectModelBase*, const Timing&, const AnimationProxy&);

  void Sample(HeapVector<Member<Interpolation>>&) const;
  KeyframeEffectModelBase* Model() const { return model_.Get(); }
  bool Paused() const { return paused_; }

  bool IsInertEffect() const final { return true; }

  bool Affects(const PropertyHandle&) const override;

  void Trace(Visitor*) const override;

 protected:
  void UpdateChildrenAndEffects() const override {}
  AnimationTimeDelta CalculateTimeToEffectChange(
      bool forwards,
      std::optional<AnimationTimeDelta> inherited_time,
      AnimationTimeDelta time_to_next_iteration) const override;
  std::optional<AnimationTimeDelta> TimelineDuration() const override;
  AnimationTimeDelta IntrinsicIterationDuration() const override;

 private:
  Member<KeyframeEffectModelBase> model_;
  bool paused_;
  std::optional<AnimationTimeDelta> inherited_time_;
  std::optional<TimelinePhase> inherited_phase_;
  std::optional<AnimationTimeDelta> timeline_duration_;
  AnimationTimeDelta intrinsic_iteration_duration_;
  double playback_rate_;
  bool at_scroll_timeline_boundary_;
};

template <>
struct DowncastTraits<InertEffect> {
  static bool AllowFrom(const AnimationEffect& animationEffect) {
    return animationEffect.IsInertEffect();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INERT_EFFECT_H_
