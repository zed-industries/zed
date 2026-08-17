// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_DISPATCHER_H_

#include "base/memory/scoped_refptr.h"
#include "device/gamepad/public/mojom/gamepad.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/platform_event_dispatcher.h"
#include "third_party/blink/renderer/modules/gamepad/gamepad_listener.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace device {
template <class T>
class GamepadImpl;
using Gamepad = GamepadImpl<void>;
class Gamepads;
}  // namespace device

namespace blink {

class GamepadSharedMemoryReader;

class GamepadDispatcher final : public GarbageCollected<GamepadDispatcher>,
                                public PlatformEventDispatcher,
                                public GamepadListener {
 public:
  explicit GamepadDispatcher(ExecutionContext& context);
  ~GamepadDispatcher() override;

  void SampleGamepads(device::Gamepads&);

  void PlayVibrationEffectOnce(uint32_t pad_index,
                               device::mojom::blink::GamepadHapticEffectType,
                               device::mojom::blink::GamepadEffectParametersPtr,
                               device::mojom::blink::GamepadHapticsManager::
                                   PlayVibrationEffectOnceCallback);
  void ResetVibrationActuator(uint32_t pad_index,
                              device::mojom::blink::GamepadHapticsManager::
                                  ResetVibrationActuatorCallback);

  void Trace(Visitor*) const override;

 private:
  void InitializeHaptics();

  // GamepadListener
  void DidConnectGamepad(uint32_t index, const device::Gamepad&) override;
  void DidDisconnectGamepad(uint32_t index, const device::Gamepad&) override;
  void ButtonOrAxisDidChange(uint32_t index, const device::Gamepad&) override;

  // PlatformEventDispatcher
  void StartListening(LocalDOMWindow*) override;
  void StopListening() override;

  void DispatchDidConnectOrDisconnectGamepad(uint32_t index,
                                             const device::Gamepad&,
                                             bool connected);

  WeakMember<ExecutionContext> execution_context_;
  Member<GamepadSharedMemoryReader> reader_;
  HeapMojoRemote<device::mojom::blink::GamepadHapticsManager>
      gamepad_haptics_manager_remote_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_DISPATCHER_H_
