// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_ENTER_FULLSCREEN_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_ENTER_FULLSCREEN_OBSERVER_H_

#include "base/functional/callback.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"

namespace blink {

// Native event listener for fullscreen change / error events for use when
// entering fullscreen.
class XrEnterFullscreenObserver : public NativeEventListener {
 public:
  XrEnterFullscreenObserver();

  XrEnterFullscreenObserver(const XrEnterFullscreenObserver&) = delete;
  XrEnterFullscreenObserver& operator=(const XrEnterFullscreenObserver&) =
      delete;

  ~XrEnterFullscreenObserver() override;

  // NativeEventListener
  void Invoke(ExecutionContext*, Event*) override;

  // Attempt to enter fullscreen with |element| as the root. |on_completed| will
  // be notified with whether or not fullscreen was successfully entered.
  // Set |may_have_camera_access| if entering fullscreen for a session that may
  // have camera access available to it - this would ensure that there is space
  // reserved for status bar.
  void RequestFullscreen(Element* element,
                         bool setup_for_dom_overlay,
                         bool may_have_camera_access,
                         base::OnceCallback<void(bool)> on_completed);

  void Trace(Visitor*) const override;

 private:
  Member<Element> fullscreen_element_;
  base::OnceCallback<void(bool)> on_completed_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_ENTER_FULLSCREEN_OBSERVER_H_
