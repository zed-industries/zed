// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_DOM_WINDOW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_DOM_WINDOW_H_

#include "third_party/blink/renderer/core/frame/dom_window.h"
#include "third_party/blink/renderer/core/frame/remote_frame.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class RemoteDOMWindow final : public DOMWindow {
 public:
  explicit RemoteDOMWindow(RemoteFrame&);

  RemoteFrame* GetFrame() const {
    return To<RemoteFrame>(DOMWindow::GetFrame());
  }

  // EventTarget overrides:
  ExecutionContext* GetExecutionContext() const override;

  // DOMWindow overrides:
  void Trace(Visitor*) const override;

  void FrameDetached();

 protected:
  // Protected DOMWindow overrides:
  void SchedulePostMessage(PostedMessage*) override;

 private:
  // Intentionally private to prevent redundant checks.
  bool IsLocalDOMWindow() const override { return false; }

  void ForwardPostMessage(PostedMessage*);
};

template <>
struct DowncastTraits<RemoteDOMWindow> {
  static bool AllowFrom(const DOMWindow& window) {
    return !window.IsLocalDOMWindow();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_REMOTE_DOM_WINDOW_H_
