// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_TEST_UTILS_H_

#include "services/device/public/cpp/test/fake_sensor_and_provider.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_testing.h"
#include "third_party/blink/renderer/core/dom/events/native_event_listener.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class EventTarget;
class ExecutionContext;
class ScriptState;

class SensorTestContext final {
  STACK_ALLOCATED();

 public:
  SensorTestContext();

  ~SensorTestContext();

  ExecutionContext* GetExecutionContext() const;
  ScriptState* GetScriptState() const;

  device::FakeSensorProvider* sensor_provider() { return &sensor_provider_; }

 private:
  void BindSensorProviderRequest(mojo::ScopedMessagePipeHandle handle);

  device::FakeSensorProvider sensor_provider_;
  V8TestingScope testing_scope_;
};

class SensorTestUtils final {
 public:
  // An event listener that can be used to count the number of times a
  // particular event has been fired.
  //
  // Usage:
  // auto* event_counter =
  //     MakeGarbageCollected<SensorTestUtils::EventCounter>();
  // my_event_target->addEventListener(..., event_counter);
  // [...]
  // EXPECT_EQ(42U, event_counter->event_count());
  class EventCounter : public NativeEventListener {
   public:
    void Invoke(ExecutionContext*, Event*) override { event_count_++; }

    size_t event_count() const { return event_count_; }

   private:
    size_t event_count_ = 0;
  };

  // Synchronously waits for |event_type| to be delivered to |event_target|.
  static void WaitForEvent(EventTarget* event_target,
                           const WTF::AtomicString& event_type);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_TEST_UTILS_H_
