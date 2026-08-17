// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LINK_PREVIEW_TRIGGERER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LINK_PREVIEW_TRIGGERER_H_

#include "base/functional/callback.h"
#include "third_party/blink/public/common/input/web_mouse_event.h"
#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/public/web/web_document.h"
#include "third_party/blink/public/web/web_element.h"

namespace blink {

// Experimental
// Observes signals of user actions and determines to trigger Link Preview.
//
// It will be instantiated per LocalFrame. Notified events are bare and intended
// to be filtered by implementations.
//
// For concrete triggers, see the implementations.
class WebLinkPreviewTriggerer {
 public:
  virtual ~WebLinkPreviewTriggerer() = default;

  // Events to track current modifier state.
  //
  // This is called for every key event and mouse leave event. For mouse leave
  // event, kNoModifier blink::WebInputEvent::kNoModifiers is passed as an
  // argument.
  //
  // The argument `modifiers` is a bit mask consisting of
  // `blink::WebInputEvent::Modifiers`.
  //
  // Note that we don't still have a right way to track modifiers state (But
  // it's enough for Link Preview because it can trigger preview only if a mouse
  // is on anchor element.) because
  // 1. we can't get changes of modifiers outside the window, and 2. we don't
  // have a reliable way to `KeyobardEventManager::GetCurrentModifierState`
  // except for macOS. For example, consider the following cases:
  //
  // A.  A user presses and holds Alt button, leaves the window, releases Alt,
  //     enters to the window.
  // A'. Same for subframe -> parent frame -> subframe.
  // B.  A user presses and holds Alt button, leaves the window, releases Alt,
  //     presses Alt, enters to the window.
  //
  // In A and A' (resp. B), the ideal `GetCurrentModifierState` should return
  // `kNoModifiers` (resp `kAltKey`), but we don't know how to correctly get to
  // know it. So, for safety, mouseleave event emits kNoModifiers.
  virtual void MaybeChangedKeyEventModifier(int modifiers) {}
  // Called when the hover element changed.
  virtual void DidChangeHoverElement(blink::WebElement element) {}
  // Called when an anchor element with valid link received a mouse event.
  virtual void DidAnchorElementReceiveMouseDownEvent(
      blink::WebElement anchor_element,
      blink::WebMouseEvent::Button button,
      int click_count) {}
  virtual void DidAnchorElementReceiveMouseUpEvent(
      blink::WebElement anchor_element,
      blink::WebMouseEvent::Button button,
      int click_count) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LINK_PREVIEW_TRIGGERER_H_
