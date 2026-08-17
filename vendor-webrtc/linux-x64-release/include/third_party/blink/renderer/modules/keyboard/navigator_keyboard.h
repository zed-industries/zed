// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_NAVIGATOR_KEYBOARD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_NAVIGATOR_KEYBOARD_H_

#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Keyboard;

// Navigator supplement which exposes keyboard related functionality.
class NavigatorKeyboard final : public GarbageCollected<NavigatorKeyboard>,
                                public Supplement<Navigator> {
 public:
  static const char kSupplementName[];
  static Keyboard* keyboard(Navigator&);

  explicit NavigatorKeyboard(Navigator&);

  NavigatorKeyboard(const NavigatorKeyboard&) = delete;
  NavigatorKeyboard& operator=(const NavigatorKeyboard&) = delete;

  void Trace(Visitor*) const override;

 private:
  Member<Keyboard> keyboard_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_NAVIGATOR_KEYBOARD_H_
