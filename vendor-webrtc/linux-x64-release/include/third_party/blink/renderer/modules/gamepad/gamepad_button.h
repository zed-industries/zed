// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_BUTTON_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_BUTTON_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace device {
class GamepadButton;
}

namespace blink {

class GamepadButton final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  GamepadButton();

  double value() const { return value_; }
  void SetValue(double val) { value_ = val; }

  bool pressed() const { return pressed_; }
  void SetPressed(bool val) { pressed_ = val; }

  bool touched() const { return touched_; }
  void SetTouched(bool val) { touched_ = val; }

  bool IsEqual(const device::GamepadButton&) const;
  void UpdateValuesFrom(const device::GamepadButton&);

 private:
  double value_;
  bool pressed_;
  bool touched_;
};

typedef HeapVector<Member<GamepadButton>> GamepadButtonVector;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_BUTTON_H_
