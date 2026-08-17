// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DOM_WINDOW_DEVICE_MOTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DOM_WINDOW_DEVICE_MOTION_H_

#include "third_party/blink/renderer/modules/event_target_modules.h"

namespace blink {

class DOMWindowDeviceMotion {
 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(devicemotion, kDevicemotion)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DOM_WINDOW_DEVICE_MOTION_H_
