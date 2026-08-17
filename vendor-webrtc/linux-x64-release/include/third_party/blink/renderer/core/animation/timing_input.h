// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMING_INPUT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMING_INPUT_H_

#include "third_party/blink/renderer/core/animation/timing.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Document;
class ExceptionState;
class V8UnionKeyframeAnimationOptionsOrUnrestrictedDouble;
class V8UnionKeyframeEffectOptionsOrUnrestrictedDouble;

class CORE_EXPORT TimingInput {
  STATIC_ONLY(TimingInput);

 public:
  // Implements steps 3 and 4 of the KeyframeEffect constructor, converting
  // the 'options' parameter into timing information.
  //
  // https://drafts.csswg.org/web-animations-1/#dom-keyframeeffect-keyframeeffect
  static Timing Convert(
      const V8UnionKeyframeEffectOptionsOrUnrestrictedDouble* options,
      Document* document,
      ExceptionState& exception_state);

  // Implements step 2 of the Animatable::animate() method, converting the
  // 'options' parameter into timing information.
  //
  // https://drafts.csswg.org/web-animations-1/#dom-animatable-animate
  static Timing Convert(
      const V8UnionKeyframeAnimationOptionsOrUnrestrictedDouble* options,
      Document* document,
      ExceptionState& exception_state);

  // Implements the procedure to 'update the timing properties of an animation
  // effect'.
  //
  // Returns true if any property in the timing properties was changed, false if
  // the input resulted in no change.
  //
  // https://drafts.csswg.org/web-animations-1/#update-the-timing-properties-of-an-animation-effect
  template <class TimingInput>
  static bool Update(Timing&, const TimingInput*, Document*, ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMING_INPUT_H_
