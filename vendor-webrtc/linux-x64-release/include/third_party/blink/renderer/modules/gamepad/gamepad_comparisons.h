// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_COMPARISONS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_COMPARISONS_H_

#include <array>
#include <bitset>

#include "device/gamepad/public/cpp/gamepads.h"
#include "third_party/blink/renderer/modules/gamepad/gamepad.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Stores the result of a comparison between two GamepadLists.
class MODULES_EXPORT GamepadStateCompareResult {
  STACK_ALLOCATED();

 public:
  GamepadStateCompareResult(const HeapVector<Member<Gamepad>> old_gamepads,
                            const HeapVector<Member<Gamepad>> new_gamepads,
                            bool compare_all_axes,
                            bool compare_all_buttons);
  ~GamepadStateCompareResult() = default;

  // True if any difference was detected (besides timestamp).
  bool IsDifferent() const;

  // True if the corresponding gamepad event should be dispatched.
  bool IsGamepadConnected(size_t pad_index) const;
  bool IsGamepadDisconnected(size_t pad_index) const;
  bool IsAxisChanged(size_t pad_index, size_t axis_index) const;
  bool IsButtonChanged(size_t pad_index, size_t button_index) const;
  bool IsButtonDown(size_t pad_index, size_t button_index) const;
  bool IsButtonUp(size_t pad_index, size_t button_index) const;

 private:
  bool CompareGamepads(const HeapVector<Member<Gamepad>> old_gamepads,
                       const HeapVector<Member<Gamepad>> new_gamepads,
                       bool compare_all_axes,
                       bool compare_all_buttons);
  bool CompareAxes(Gamepad* old_gamepad,
                   Gamepad* new_gamepad,
                   size_t gamepad_index,
                   bool compare_all);
  bool CompareButtons(Gamepad* old_gamepad,
                      Gamepad* new_gamepad,
                      size_t gamepad_index,
                      bool compare_all);
  bool CompareTouches(Gamepad* old_gamepad, Gamepad* new_gamepad);

  bool any_change_ = false;
  std::bitset<device::Gamepads::kItemsLengthCap> gamepad_connected_;
  std::bitset<device::Gamepads::kItemsLengthCap> gamepad_disconnected_;
  std::array<std::bitset<device::Gamepad::kAxesLengthCap>,
             device::Gamepads::kItemsLengthCap>
      axis_changed_;
  std::array<std::bitset<device::Gamepad::kButtonsLengthCap>,
             device::Gamepads::kItemsLengthCap>
      button_changed_;
  std::array<std::bitset<device::Gamepad::kButtonsLengthCap>,
             device::Gamepads::kItemsLengthCap>
      button_down_;
  std::array<std::bitset<device::Gamepad::kButtonsLengthCap>,
             device::Gamepads::kItemsLengthCap>
      button_up_;
};

class MODULES_EXPORT GamepadComparisons {
  STATIC_ONLY(GamepadComparisons);

 public:
  // Inspect the gamepad state in |gamepads| and return true if any gamepads
  // have a user activation gesture.
  static bool HasUserActivation(const HeapVector<Member<Gamepad>> gamepads);

  // Given the connection state of a gamepad in consecutive samples and whether
  // the ID string changed, return whether the gamepad was newly connected in
  // |gamepad_found| and whether it was newly disconnected in |gamepad_lost|.
  static void HasGamepadConnectionChanged(bool old_connected,
                                          bool new_connected,
                                          bool id_changed,
                                          bool* gamepad_found,
                                          bool* gamepad_lost);

  // Compare the previously sampled gamepad state in |old_gamepads| with a new
  // sample in |new_gamepads|. If |compare_all_axes| or |compare_all_buttons|
  // is true, all axes or buttons will be compared. Otherwise, the comparison
  // will short-circuit after the first difference.
  static GamepadStateCompareResult Compare(
      const HeapVector<Member<Gamepad>> old_gamepads,
      const HeapVector<Member<Gamepad>> new_gamepads,
      bool compare_all_axes,
      bool compare_all_buttons);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GAMEPAD_GAMEPAD_COMPARISONS_H_
