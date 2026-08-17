// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_ABSOLUTE_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_ABSOLUTE_CONTROLLER_H_

#include "third_party/blink/renderer/modules/device_orientation/device_orientation_controller.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class MODULES_EXPORT DeviceOrientationAbsoluteController final
    : public DeviceOrientationController {
 public:
  static const char kSupplementName[];

  explicit DeviceOrientationAbsoluteController(LocalDOMWindow&);
  ~DeviceOrientationAbsoluteController() override;

  static DeviceOrientationAbsoluteController& From(LocalDOMWindow&);

  // Inherited from DeviceSingleWindowEventController.
  void DidAddEventListener(LocalDOMWindow*,
                           const AtomicString& event_type) override;

  void Trace(Visitor*) const override;

 private:
  // Inherited from PlatformEventController.
  void RegisterWithDispatcher() override;

  // Inherited from DeviceOrientationController.
  const AtomicString& EventTypeName() const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_ORIENTATION_ABSOLUTE_CONTROLLER_H_
