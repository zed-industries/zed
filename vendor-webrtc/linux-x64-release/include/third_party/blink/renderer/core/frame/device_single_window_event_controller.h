// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DEVICE_SINGLE_WINDOW_EVENT_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DEVICE_SINGLE_WINDOW_EVENT_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/frame/platform_event_controller.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Event;

class CORE_EXPORT DeviceSingleWindowEventController
    : public GarbageCollected<DeviceSingleWindowEventController>,
      public PlatformEventController,
      public LocalDOMWindow::EventListenerObserver {
 public:
  ~DeviceSingleWindowEventController() override;

  // Inherited from PlatformEventController.
  void DidUpdateData() override;
  void Trace(Visitor*) const override;

  // Inherited from LocalDOMWindow::EventListenerObserver.
  void DidAddEventListener(LocalDOMWindow*, const AtomicString&) override;
  void DidRemoveEventListener(LocalDOMWindow*, const AtomicString&) override;
  void DidRemoveAllEventListeners(LocalDOMWindow*) override;

 protected:
  explicit DeviceSingleWindowEventController(LocalDOMWindow&);

  bool CheckPolicyFeatures(
      const Vector<network::mojom::PermissionsPolicyFeature>& features) const;

  void DispatchDeviceEvent(Event*);

  virtual Event* LastEvent() const = 0;
  virtual const AtomicString& EventTypeName() const = 0;
  virtual bool IsNullEvent(Event*) const = 0;

  void set_needs_checking_null_events(bool enabled) {
    needs_checking_null_events_ = enabled;
  }

 private:
  bool needs_checking_null_events_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_DEVICE_SINGLE_WINDOW_EVENT_CONTROLLER_H_
