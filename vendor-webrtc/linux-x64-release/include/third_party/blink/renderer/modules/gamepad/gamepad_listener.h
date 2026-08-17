// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_LISTENER_H_

namespace device {
template <class T>
class GamepadImpl;
using Gamepad = GamepadImpl<void>;
}

namespace blink {

class GamepadListener {
 public:
  virtual ~GamepadListener() = default;

  // Called when a gamepad is connected. |index| is the index of the gamepad in
  // the gamepad array, and |gamepad| is a reference to the connected gamepad.
  virtual void DidConnectGamepad(uint32_t index,
                                 const device::Gamepad& gamepad) = 0;

  // Called when a gamepad is disconnected. |index| is the former index of the
  // gamepad in the gamepad array, and |gamepad| is a reference to the
  // connected gamepad.
  virtual void DidDisconnectGamepad(uint32_t index,
                                    const device::Gamepad& gamepad) = 0;

  // Called when a button or axis is changed on a connected gamepad. |index| is
  // the index of the gamepad in the gamepad array, and |gamepad| is a reference
  // to the gamepad.
  virtual void ButtonOrAxisDidChange(uint32_t index,
                                     const device::Gamepad& gamepad) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_LISTENER_H_
