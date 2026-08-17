// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PLATFORM_EVENT_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PLATFORM_EVENT_CONTROLLER_H_

#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/page_visibility_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class LocalDOMWindow;

// Base controller class for registering controllers with a dispatcher.
// It watches page visibility and calls stopUpdating when page is not visible.
// It provides a DidUpdateData() callback method which is called when new data
// it available.
class CORE_EXPORT PlatformEventController : public PageVisibilityObserver {
 public:
  void StartUpdating();
  void StopUpdating();

  // This is called when new data becomes available.
  virtual void DidUpdateData() = 0;

  void Trace(Visitor*) const override;
  LocalDOMWindow& GetWindow() const { return *window_; }

 protected:
  explicit PlatformEventController(LocalDOMWindow&);
  virtual ~PlatformEventController();

  virtual void RegisterWithDispatcher() = 0;
  virtual void UnregisterWithDispatcher() = 0;

  // When true initiates a one-shot DidUpdateData() when StartUpdating() is
  // called.
  virtual bool HasLastData() = 0;

  bool has_event_listener_;

 private:
  // Inherited from PageVisibilityObserver.
  void PageVisibilityChanged() override;

  void UpdateCallback();

  bool is_active_;
  Member<LocalDOMWindow> window_;
  TaskHandle update_callback_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PLATFORM_EVENT_CONTROLLER_H_
