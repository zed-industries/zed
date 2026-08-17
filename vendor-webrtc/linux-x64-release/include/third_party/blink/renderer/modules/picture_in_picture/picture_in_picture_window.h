// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_PICTURE_IN_PICTURE_WINDOW_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_PICTURE_IN_PICTURE_WINDOW_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class ExecutionContext;

// The PictureInPictureWindow is meant to be used only by
// PictureInPictureController and is fundamentally just a simple proxy to get
// information such as dimensions about the current Picture-in-Picture window.
class PictureInPictureWindow
    : public EventTarget,
      public ActiveScriptWrappable<PictureInPictureWindow>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PictureInPictureWindow() = delete;

  // Create a Picture-in-Picture window for an HTMLVideoElement.
  PictureInPictureWindow(ExecutionContext*, const gfx::Size& size);

  PictureInPictureWindow(const PictureInPictureWindow&) = delete;
  PictureInPictureWindow& operator=(const PictureInPictureWindow&) = delete;

  int width() const { return size_.width(); }
  int height() const { return size_.height(); }

  // Called when Picture-in-Picture window state is closed.
  void OnClose();

  // Called when the Picture-in-Picture window is resized.
  void OnResize(const gfx::Size&);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(resize, kResize)

  // EventTarget overrides.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextClient::GetExecutionContext();
  }

  // ActiveScriptWrappable overrides.
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

 protected:
  // EventTarget overrides.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

 private:
  // The Picture-in-Picture window size in pixels.
  gfx::Size size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_PICTURE_IN_PICTURE_WINDOW_H_
