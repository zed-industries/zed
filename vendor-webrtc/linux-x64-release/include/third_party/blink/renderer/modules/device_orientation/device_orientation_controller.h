// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_CONTROLLER_H_

#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/frame/device_single_window_event_controller.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class DeviceOrientationData;
class DeviceOrientationEventPump;
class Event;
class ScriptState;
class V8PermissionState;

class MODULES_EXPORT DeviceOrientationController
    : public DeviceSingleWindowEventController,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  explicit DeviceOrientationController(LocalDOMWindow&);
  ~DeviceOrientationController() override;

  static DeviceOrientationController& From(LocalDOMWindow&);

  // Inherited from DeviceSingleWindowEventController.
  void DidUpdateData() override;
  void DidAddEventListener(LocalDOMWindow*,
                           const AtomicString& event_type) override;

  void SetOverride(DeviceOrientationData*);
  void ClearOverride();

  void RestartPumpIfNeeded();

  void Trace(Visitor*) const override;

  static void LogToConsolePolicyFeaturesDisabled(
      LocalFrame&,
      const AtomicString& event_name);

  ScriptPromise<V8PermissionState> RequestPermission(ScriptState*);

 protected:
  void RegisterWithOrientationEventPump(bool absolute);

  Member<DeviceOrientationEventPump> orientation_event_pump_;

 private:
  // Inherited from PlatformEventController.
  void RegisterWithDispatcher() override;
  void UnregisterWithDispatcher() override;
  bool HasLastData() override;

  // Inherited from DeviceSingleWindowEventController.
  Event* LastEvent() const override;
  const AtomicString& EventTypeName() const override;
  bool IsNullEvent(Event*) const override;

  DeviceOrientationData* LastData() const;

  Member<DeviceOrientationData> override_orientation_data_;

  HeapMojoRemote<mojom::blink::PermissionService> permission_service_;
  bool has_requested_permission_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_CONTROLLER_H_
