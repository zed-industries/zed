// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_POSTURE_NAVIGATOR_DEVICE_POSTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_POSTURE_NAVIGATOR_DEVICE_POSTURE_H_

#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class DevicePosture;

class NavigatorDevicePosture final
    : public GarbageCollected<NavigatorDevicePosture>,
      public Supplement<Navigator> {
 public:
  static const char kSupplementName[];
  static DevicePosture* devicePosture(Navigator&);

  explicit NavigatorDevicePosture(Navigator&);

  void Trace(Visitor*) const override;

 private:
  Member<DevicePosture> posture_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_POSTURE_NAVIGATOR_DEVICE_POSTURE_H_
