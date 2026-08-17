// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_WORKLET_ANIMATION_EFFECT_TIMINGS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_WORKLET_ANIMATION_EFFECT_TIMINGS_H_

#include "third_party/blink/renderer/core/animation/timing.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation.h"

namespace blink {

class MODULES_EXPORT WorkletAnimationEffectTimings final
    : public cc::AnimationEffectTimings {
 public:
  explicit WorkletAnimationEffectTimings(
      scoped_refptr<base::RefCountedData<Vector<Timing>>>,
      scoped_refptr<base::RefCountedData<Vector<Timing::NormalizedTiming>>>);

  std::unique_ptr<cc::AnimationEffectTimings> Clone() const override;

  ~WorkletAnimationEffectTimings() override;

  const scoped_refptr<base::RefCountedData<Vector<Timing>>>& GetTimings() {
    return timings_;
  }
  const scoped_refptr<base::RefCountedData<Vector<Timing::NormalizedTiming>>>&
  GetNormalizedTimings() {
    return normalized_timings_;
  }

 private:
  scoped_refptr<base::RefCountedData<Vector<Timing>>> timings_;
  scoped_refptr<base::RefCountedData<Vector<Timing::NormalizedTiming>>>
      normalized_timings_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_WORKLET_ANIMATION_EFFECT_TIMINGS_H_
