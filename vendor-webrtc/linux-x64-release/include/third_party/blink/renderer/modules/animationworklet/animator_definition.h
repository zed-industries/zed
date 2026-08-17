// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATOR_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATOR_DEFINITION_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class V8AnimateCallback;
class V8AnimatorConstructor;
class V8StateCallback;

// Represents a valid registered Javascript animator.  In particular it owns two
// |v8::Function|s that are the "constructor" and "animate" functions of the
// registered class. It does not do any validation itself and relies on
// |AnimationWorkletGlobalScope::registerAnimator| to validate the provided
// Javascript class before completing the registration.
class MODULES_EXPORT AnimatorDefinition final
    : public GarbageCollected<AnimatorDefinition>,
      public NameClient {
 public:
  explicit AnimatorDefinition(V8AnimatorConstructor* constructor,
                              V8AnimateCallback* animate,
                              V8StateCallback* state);
  ~AnimatorDefinition() override = default;
  void Trace(Visitor* visitor) const;
  const char* NameInHeapSnapshot() const override {
    return "AnimatorDefinition";
  }

  V8AnimatorConstructor* ConstructorFunction() const {
    return constructor_.Get();
  }
  V8AnimateCallback* AnimateFunction() const { return animate_.Get(); }
  V8StateCallback* StateFunction() const { return state_.Get(); }
  bool IsStateful() const { return state_ != nullptr; }

 private:
  // This object keeps the constructor function, animate, and state function
  // alive. It participates in wrapper tracing as it holds onto V8 wrappers.
  Member<V8AnimatorConstructor> constructor_;
  Member<V8AnimateCallback> animate_;
  Member<V8StateCallback> state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ANIMATIONWORKLET_ANIMATOR_DEFINITION_H_
