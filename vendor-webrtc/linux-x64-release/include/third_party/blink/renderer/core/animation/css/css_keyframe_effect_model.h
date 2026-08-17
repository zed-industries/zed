// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// This variant of the keyframe effect model performs additional processing
// for computed keyframes. The algorithm for constructing keyframes for a CSS
// animation is covered in the following spec:
// https://drafts.csswg.org/css-animations-2/#keyframes
//
// Most of the steps for constructing computed keyframes are handled during
// the construction process; however, evaluation of computed property values
// is handled as a lazy operation when fetching the keyframes.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_KEYFRAME_EFFECT_MODEL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_KEYFRAME_EFFECT_MODEL_H_

#include "third_party/blink/renderer/core/animation/keyframe_effect_model.h"

namespace blink {

class CssKeyframeEffectModel : public StringKeyframeEffectModel {
 public:
  explicit CssKeyframeEffectModel(
      const KeyframeVector& keyframes,
      CompositeOperation composite = kCompositeReplace,
      scoped_refptr<TimingFunction> default_keyframe_easing = nullptr,
      bool has_named_range_keyframes = false)
      : StringKeyframeEffectModel(keyframes,
                                  composite,
                                  std::move(default_keyframe_easing),
                                  has_named_range_keyframes) {}

  // Overridden to fill in missing property values for generated "from" and "to"
  // keyframes. TODO(crbug.com/1070627): Also perform the following steps:
  // 1) filter variables from keyframes, 2) use computed values for properties
  // rather than values from the keyframes rule, and 3) switch logical to
  // physical properties.
  KeyframeEffectModelBase::KeyframeVector GetComputedKeyframes(
      Element* element) override;

  bool IsCssKeyframeEffectModel() override { return true; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_KEYFRAME_EFFECT_MODEL_H_
