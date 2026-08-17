// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_VIRTUAL_KEYBOARD_OVERLAY_CHANGED_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_VIRTUAL_KEYBOARD_OVERLAY_CHANGED_OBSERVER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace gfx {
class Rect;
}

namespace blink {

class LocalFrame;

// This observer is used to register for VK overlay geometry change
// notifications that is sent from Browser process to |LocalFrame|.
// Browser process receives these VK showing/hiding events from the OS input
// services. It is reported as a rectangle that occludes the web content.
class CORE_EXPORT VirtualKeyboardOverlayChangedObserver
    : public GarbageCollectedMixin {
 public:
  // This is used to fire a VK overlay geometry change JS event.
  // The |Rect| is the VK rectangle that occludes the web content.
  // This is called while the keyboard is shown or hidden.
  virtual void VirtualKeyboardOverlayChanged(const gfx::Rect&) = 0;

 protected:
  // Input to this function should be a valid |LocalFrame| that gets the
  // VK overlay geometry change notification from the Browser process.
  // This is created when |VirtualKeyboard| object is initialized which is
  // part of the |Navigator| object. If this is passed as |nullptr|, then
  // it won't be registered in |LocalFrame| to get notified about the VK overlay
  // geometry change.
  explicit VirtualKeyboardOverlayChangedObserver(LocalFrame*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_VIRTUAL_KEYBOARD_OVERLAY_CHANGED_OBSERVER_H_
