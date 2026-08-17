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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATABLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATABLE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class Animation;
class Element;
class ExceptionState;
class GetAnimationsOptions;
class ScriptState;
class ScriptValue;
class V8UnionKeyframeAnimationOptionsOrUnrestrictedDouble;

struct GetAnimationsOptionsResolved {
  bool use_subtree;
};

// https://drafts.csswg.org/web-animations-1/#the-animatable-interface-mixin
class CORE_EXPORT Animatable {
 public:
  // Returns the target element of the animation that these methods are being
  // called on.
  virtual Element* GetAnimationTarget() = 0;

  Animation* animate(
      ScriptState* script_state,
      const ScriptValue& keyframes,
      const V8UnionKeyframeAnimationOptionsOrUnrestrictedDouble* options,
      ExceptionState& exception_state);

  Animation* animate(ScriptState*, const ScriptValue&, ExceptionState&);

  HeapVector<Member<Animation>> getAnimations(
      GetAnimationsOptions* options = nullptr);

  HeapVector<Member<Animation>> GetAnimationsInternal(
      GetAnimationsOptionsResolved options);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATABLE_H_
