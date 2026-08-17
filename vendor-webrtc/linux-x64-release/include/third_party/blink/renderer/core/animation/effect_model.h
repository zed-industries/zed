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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_EFFECT_MODEL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_EFFECT_MODEL_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/v8_composite_operation.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_composite_operation_or_auto.h"
#include "third_party/blink/renderer/core/animation/animation_time_delta.h"
#include "third_party/blink/renderer/core/animation/property_handle.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/platform/animation/timing_function.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Interpolation;

// Time independent representation of an Animation's content.
// Can be sampled for the active pairs of Keyframes (represented by
// Interpolations) at a given time fraction.
class CORE_EXPORT EffectModel : public GarbageCollected<EffectModel> {
 public:
  enum CompositeOperation {
    kCompositeReplace,
    kCompositeAdd,
    kCompositeAccumulate,
  };
  static CompositeOperation EnumToCompositeOperation(
      V8CompositeOperation::Enum);
  static std::optional<CompositeOperation> EnumToCompositeOperation(
      V8CompositeOperationOrAuto::Enum);
  static V8CompositeOperation::Enum CompositeOperationToEnum(
      CompositeOperation);

  EffectModel() = default;
  virtual ~EffectModel() = default;
  virtual bool Sample(int iteration,
                      double fraction,
                      TimingFunction::LimitDirection,
                      AnimationTimeDelta iteration_duration,
                      HeapVector<Member<Interpolation>>&) const = 0;

  virtual bool Affects(const PropertyHandle&) const { return false; }
  virtual bool AffectedByUnderlyingAnimations() const = 0;
  virtual bool IsTransformRelatedEffect() const { return false; }
  virtual bool IsKeyframeEffectModel() const { return false; }

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_EFFECT_MODEL_H_
