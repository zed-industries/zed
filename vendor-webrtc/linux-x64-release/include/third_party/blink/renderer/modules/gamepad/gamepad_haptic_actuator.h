// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_HAPTIC_ACTUATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_HAPTIC_ACTUATOR_H_

#include "device/gamepad/public/cpp/gamepad.h"
#include "device/gamepad/public/mojom/gamepad.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_gamepad_haptic_actuator_type.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class GamepadDispatcher;
class GamepadEffectParameters;
enum class GamepadHapticActuatorType;
class ScriptState;
class V8GamepadHapticEffectType;
class V8GamepadHapticsResult;

class GamepadHapticActuator final : public ScriptWrappable,
                                    public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  GamepadHapticActuator(ExecutionContext& context,
                        int pad_index,
                        device::GamepadHapticActuatorType type);
  ~GamepadHapticActuator() override;

  const Vector<V8GamepadHapticEffectType>& effects() const {
    return supported_effects_;
  }

  V8GamepadHapticActuatorType type() const {
    return V8GamepadHapticActuatorType(type_);
  }
  void SetType(device::GamepadHapticActuatorType);

  ScriptPromise<V8GamepadHapticsResult> playEffect(
      ScriptState*,
      const V8GamepadHapticEffectType&,
      const GamepadEffectParameters*);

  ScriptPromise<V8GamepadHapticsResult> reset(ScriptState*);

  void Trace(Visitor*) const override;

 private:
  void OnPlayEffectCompleted(ScriptPromiseResolver<V8GamepadHapticsResult>*,
                             device::mojom::GamepadHapticsResult);
  void OnResetCompleted(ScriptPromiseResolver<V8GamepadHapticsResult>*,
                        device::mojom::GamepadHapticsResult);
  void ResetVibrationIfNotPreempted();

  int pad_index_;
  V8GamepadHapticActuatorType::Enum type_;
  bool should_reset_ = false;
  Vector<V8GamepadHapticEffectType> supported_effects_;

  Member<GamepadDispatcher> gamepad_dispatcher_;
};

typedef HeapVector<Member<GamepadHapticActuator>> GamepadHapticActuatorVector;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_HAPTIC_ACTUATOR_H_
