// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_ERROR_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_ERROR_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_sensor_error_event_init.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SensorErrorEvent : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static SensorErrorEvent* Create(const AtomicString& event_type,
                                  DOMException* error) {
    return MakeGarbageCollected<SensorErrorEvent>(event_type, error);
  }

  static SensorErrorEvent* Create(const AtomicString& event_type,
                                  const SensorErrorEventInit* initializer) {
    return MakeGarbageCollected<SensorErrorEvent>(event_type, initializer);
  }

  SensorErrorEvent(const AtomicString& event_type, DOMException* error);
  SensorErrorEvent(const AtomicString& event_type,
                   const SensorErrorEventInit* initializer);
  ~SensorErrorEvent() override;

  void Trace(Visitor*) const override;

  const AtomicString& InterfaceName() const override;

  DOMException* error() { return error_.Get(); }

 private:
  Member<DOMException> error_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_ERROR_EVENT_H_
