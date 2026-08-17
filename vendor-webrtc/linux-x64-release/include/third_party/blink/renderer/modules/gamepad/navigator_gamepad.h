/*
 * Copyright (C) 2011, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_NAVIGATOR_GAMEPAD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_NAVIGATOR_GAMEPAD_H_

#include <array>

#include "base/time/time.h"
#include "device/gamepad/public/cpp/gamepads.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/core/frame/platform_event_controller.h"
#include "third_party/blink/renderer/modules/gamepad/gamepad.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace device {
template <class T>
class GamepadImpl;
using Gamepad = GamepadImpl<void>;
}

namespace blink {

class GamepadDispatcher;
class GamepadHapticActuator;
class Navigator;

class MODULES_EXPORT NavigatorGamepad final
    : public GarbageCollected<NavigatorGamepad>,
      public Supplement<Navigator>,
      public ExecutionContextClient,
      public PlatformEventController,
      public LocalDOMWindow::EventListenerObserver,
      public Gamepad::Client {
 public:
  static const char kSupplementName[];

  static NavigatorGamepad& From(Navigator&);

  explicit NavigatorGamepad(Navigator&);
  ~NavigatorGamepad() override;

  static HeapVector<Member<Gamepad>> getGamepads(Navigator&, ExceptionState&);
  HeapVector<Member<Gamepad>> Gamepads();

  void Trace(Visitor*) const override;

 private:
  void SampleGamepads();

  void DidRemoveGamepadEventListeners();
  bool StartUpdatingIfAttached();
  void SampleAndCompareGamepadState();
  void DispatchGamepadEvent(const AtomicString&, Gamepad*);

  // PageVisibilityObserver
  void PageVisibilityChanged() override;

  // PlatformEventController
  void RegisterWithDispatcher() override;
  void UnregisterWithDispatcher() override;
  bool HasLastData() override;
  void DidUpdateData() override;

  // LocalDOMWindow::EventListenerObserver
  void DidAddEventListener(LocalDOMWindow*, const AtomicString&) override;
  void DidRemoveEventListener(LocalDOMWindow*, const AtomicString&) override;
  void DidRemoveAllEventListeners(LocalDOMWindow*) override;

  // Gamepad::Client
  GamepadHapticActuator* GetVibrationActuatorForGamepad(
      const Gamepad&) override;
  void SetTouchEvents(const Gamepad&,
                      GamepadTouchVector&,
                      base::span<const device::GamepadTouch>) override;

  // A reference to the buffer containing the last-received gamepad state. May
  // be nullptr if no data has been received yet. Do not overwrite this buffer
  // as it may have already been returned to the page. Instead, write to
  // |gamepads_back_| and swap buffers.
  HeapVector<Member<Gamepad>> gamepads_;

  // True if the buffer referenced by |gamepads_| has been exposed to the page.
  // When the buffer is not exposed, prefer to reuse it.
  bool is_gamepads_exposed_ = false;

  // A reference to the buffer for receiving new gamepad state. May be
  // overwritten.
  HeapVector<Member<Gamepad>> gamepads_back_;

  HeapVector<Member<GamepadHapticActuator>> vibration_actuators_;

  // Together the following keep track of the nextTouchId per Gamepad
  using TouchIdMap =
      WTF::HashMap<uint32_t, uint32_t, WTF::IntWithZeroKeyHashTraits<uint32_t>>;

  std::array<TouchIdMap, device::Gamepads::kItemsLengthCap> touch_id_map_;
  std::array<uint32_t, device::Gamepads::kItemsLengthCap> next_touch_id_;

  // The timestamp for the navigationStart attribute. Gamepad timestamps are
  // reported relative to this value.
  base::TimeTicks navigation_start_;

  // The timestamp when gamepads were made available to the page. If no data has
  // been received from the hardware, the gamepad timestamp should be equal to
  // this value.
  base::TimeTicks gamepads_start_;

  // True if there is at least one listener for gamepad connection or
  // disconnection events.
  bool has_connection_event_listener_ = false;

  // True while processing gamepad events.
  bool processing_events_ = false;

  Member<GamepadDispatcher> gamepad_dispatcher_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_NAVIGATOR_GAMEPAD_H_
