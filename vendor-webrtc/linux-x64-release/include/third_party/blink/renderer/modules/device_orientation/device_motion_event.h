/*
 * Copyright (C) 2010 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_MOTION_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_MOTION_EVENT_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class DeviceMotionEventAcceleration;
class DeviceMotionData;
class DeviceMotionEventInit;
class DeviceMotionEventRotationRate;
class ScriptState;
class V8PermissionState;

class DeviceMotionEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  DeviceMotionEvent();
  DeviceMotionEvent(const AtomicString&, const DeviceMotionEventInit*);
  DeviceMotionEvent(const AtomicString& event_type, const DeviceMotionData*);
  ~DeviceMotionEvent() override;

  static DeviceMotionEvent* Create() {
    return MakeGarbageCollected<DeviceMotionEvent>();
  }
  static DeviceMotionEvent* Create(const AtomicString& event_type,
                                   const DeviceMotionEventInit* initializer) {
    return MakeGarbageCollected<DeviceMotionEvent>(event_type, initializer);
  }
  static DeviceMotionEvent* Create(const AtomicString& event_type,
                                   const DeviceMotionData* device_motion_data) {
    return MakeGarbageCollected<DeviceMotionEvent>(event_type,
                                                   device_motion_data);
  }

  const DeviceMotionData* GetDeviceMotionData() const {
    return device_motion_data_.Get();
  }

  DeviceMotionEventAcceleration* acceleration();
  DeviceMotionEventAcceleration* accelerationIncludingGravity();
  DeviceMotionEventRotationRate* rotationRate();
  double interval() const;

  static ScriptPromise<V8PermissionState> requestPermission(ScriptState*);

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  Member<const DeviceMotionData> device_motion_data_;
};

template <>
struct DowncastTraits<DeviceMotionEvent> {
  static bool AllowFrom(const Event& event) {
    return event.InterfaceName() == event_interface_names::kDeviceMotionEvent;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_ORIENTATION_DEVICE_MOTION_EVENT_H_
