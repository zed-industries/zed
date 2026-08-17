// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_gamepad_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/gamepad/gamepad.h"

namespace blink {

class GamepadEvent : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GamepadEvent* Create(const AtomicString& type,
                              Bubbles bubbles,
                              Cancelable cancelable,
                              Gamepad* gamepad) {
    return MakeGarbageCollected<GamepadEvent>(type, bubbles, cancelable,
                                              gamepad);
  }
  static GamepadEvent* Create(const AtomicString& type,
                              const GamepadEventInit* initializer) {
    return MakeGarbageCollected<GamepadEvent>(type, initializer);
  }

  GamepadEvent(const AtomicString& type, Bubbles, Cancelable, Gamepad*);
  GamepadEvent(const AtomicString&, const GamepadEventInit*);
  ~GamepadEvent() override;

  Gamepad* getGamepad() const { return gamepad_.Get(); }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  Member<Gamepad> gamepad_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_EVENT_H_
