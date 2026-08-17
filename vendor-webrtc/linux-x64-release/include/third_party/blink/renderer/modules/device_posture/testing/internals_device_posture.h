// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_POSTURE_TESTING_INTERNALS_DEVICE_POSTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_POSTURE_TESTING_INTERNALS_DEVICE_POSTURE_H_

#include "third_party/blink/public/mojom/device_posture/device_posture_provider.mojom-shared.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_device_posture_type.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Internals;
class ScriptState;

class InternalsDevicePosture {
  STATIC_ONLY(InternalsDevicePosture);

 public:
  static ScriptPromise<IDLUndefined> setDevicePostureOverride(
      ScriptState* script_state,
      Internals&,
      V8DevicePostureType posture);

  static ScriptPromise<IDLUndefined> clearDevicePostureOverride(
      ScriptState* script_state,
      Internals&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_POSTURE_TESTING_INTERNALS_DEVICE_POSTURE_H_
