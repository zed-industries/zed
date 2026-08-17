// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SAMPLED_EFFECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SAMPLED_EFFECT_H_

#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/animation/interpolation.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// Associates the results of sampling an EffectModel with metadata used for
// effect ordering and managing composited animations.
class SampledEffect final : public GarbageCollected<SampledEffect> {
 public:
  SampledEffect(KeyframeEffect*, unsigned sequence_number);
  SampledEffect(const SampledEffect&) = delete;
  SampledEffect& operator=(const SampledEffect&) = delete;

  void Clear();

  const HeapVector<Member<Interpolation>>& Interpolations() const {
    return interpolations_;
  }
  HeapVector<Member<Interpolation>>& MutableInterpolations() {
    return interpolations_;
  }

  KeyframeEffect* Effect() const { return effect_.Get(); }
  unsigned SequenceNumber() const { return sequence_number_; }
  KeyframeEffect::Priority GetPriority() const { return priority_; }
  bool WillNeverChange() const;
  void RemoveReplacedInterpolations(const HashSet<PropertyHandle>&);
  void UpdateReplacedProperties(HashSet<PropertyHandle>&);

  void Trace(Visitor*) const;

 private:
  WeakMember<KeyframeEffect> effect_;
  HeapVector<Member<Interpolation>> interpolations_;
  const unsigned sequence_number_;
  KeyframeEffect::Priority priority_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SAMPLED_EFFECT_H_
