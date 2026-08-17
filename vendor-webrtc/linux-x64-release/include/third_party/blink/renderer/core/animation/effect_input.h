// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_EFFECT_INPUT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_EFFECT_INPUT_H_

#include "third_party/blink/renderer/core/animation/effect_model.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect_model.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Element;
class ExceptionState;
class ScriptState;
class ScriptValue;

class CORE_EXPORT EffectInput {
  STATIC_ONLY(EffectInput);

 public:
  static KeyframeEffectModelBase* Convert(Element*,
                                          const ScriptValue& keyframes,
                                          EffectModel::CompositeOperation,
                                          ScriptState*,
                                          ExceptionState&);

  // Implements "Processing a keyframes argument" from the web-animations spec.
  // https://w3.org/TR/web-animations-1/#processing-a-keyframes-argument
  static StringKeyframeVector ParseKeyframesArgument(
      Element*,
      const ScriptValue& keyframes,
      ScriptState*,
      ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_EFFECT_INPUT_H_
