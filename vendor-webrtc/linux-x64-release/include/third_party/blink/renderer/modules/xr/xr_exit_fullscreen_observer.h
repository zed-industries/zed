// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_EXIT_FULLSCREEN_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_EXIT_FULLSCREEN_OBSERVER_H_

#include "base/functional/callback.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"

namespace blink {

// Native event listener used when waiting for fullscreen mode to fully exit.
// Typically used to either clear the way for starting fullscreen mode for AR
// when exiting an existing fullscreen, or for ensuring that fullscreen has
// exited when ending an AR session.
class XrExitFullscreenObserver : public NativeEventListener {
 public:
  XrExitFullscreenObserver();

  XrExitFullscreenObserver(const XrExitFullscreenObserver&) = delete;
  XrExitFullscreenObserver& operator=(const XrExitFullscreenObserver&) = delete;

  ~XrExitFullscreenObserver() override;

  // NativeEventListener
  void Invoke(ExecutionContext*, Event*) override;

  void ExitFullscreen(Document* doc, base::OnceClosure on_exited);

  void Trace(Visitor*) const override;

 private:
  Member<Document> document_;
  base::OnceClosure on_exited_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_EXIT_FULLSCREEN_OBSERVER_H_
