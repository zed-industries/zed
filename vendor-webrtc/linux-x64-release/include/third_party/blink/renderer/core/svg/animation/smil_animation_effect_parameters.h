// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_ANIMATION_EFFECT_PARAMETERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_ANIMATION_EFFECT_PARAMETERS_H_

namespace blink {

// This struct describes the parameters needed to apply the animation
// effect function.
// https://www.w3.org/TR/SMIL/smil-animation.html#animationNS-AnimationEffectFcn
struct SMILAnimationEffectParameters {
  bool is_discrete = false;
  bool is_additive = false;
  bool is_cumulative = false;
};

// Compute the animated number value, excluding additive behavior, based on
// effect parameters and timing data.
inline float ComputeAnimatedNumber(
    const SMILAnimationEffectParameters& parameters,
    float percentage,
    unsigned repeat_count,
    float from_number,
    float to_number,
    float to_at_end_of_duration_number) {
  float number;
  if (parameters.is_discrete)
    number = percentage < 0.5 ? from_number : to_number;
  else
    number = (to_number - from_number) * percentage + from_number;
  if (repeat_count && parameters.is_cumulative)
    number += to_at_end_of_duration_number * repeat_count;
  return number;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_ANIMATION_EFFECT_PARAMETERS_H_
