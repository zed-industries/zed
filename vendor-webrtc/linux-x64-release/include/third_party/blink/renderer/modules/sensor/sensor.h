// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_H_

#include "services/network/public/mojom/permissions_policy/permissions_policy_feature.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_sensor_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_spatial_sensor_options.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/dom/dom_time_stamp.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/platform_event_controller.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/sensor/sensor_proxy.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace blink {

class DOMException;
class ExceptionState;
class ExecutionContext;

class Sensor : public EventTarget,
               public ActiveScriptWrappable<Sensor>,
               public ExecutionContextLifecycleObserver,
               public SensorProxy::Observer {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class SensorState { kIdle, kActivating, kActivated };

  ~Sensor() override;

  void start();
  void stop();

  // EventTarget overrides.
  const AtomicString& InterfaceName() const override {
    return event_target_names::kSensor;
  }
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextLifecycleObserver::GetExecutionContext();
  }

  // Getters
  bool activated() const;
  bool hasReading() const;
  std::optional<DOMHighResTimeStamp> timestamp(ScriptState*) const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(reading, kReading)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(activate, kActivate)

  // ActiveScriptWrappable overrides.
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

 protected:
  Sensor(ExecutionContext*,
         const SensorOptions*,
         ExceptionState&,
         device::mojom::blink::SensorType,
         const Vector<network::mojom::PermissionsPolicyFeature>&);

  Sensor(ExecutionContext*,
         const SpatialSensorOptions*,
         ExceptionState&,
         device::mojom::blink::SensorType,
         const Vector<network::mojom::PermissionsPolicyFeature>&);

  using SensorConfigurationPtr = device::mojom::blink::SensorConfigurationPtr;
  using SensorConfiguration = device::mojom::blink::SensorConfiguration;

  // The default implementation will init frequency configuration parameter,
  // concrete sensor implementations can override this method to handle other
  // parameters if needed.
  virtual SensorConfigurationPtr CreateSensorConfig();

  bool IsIdleOrErrored() const;
  const device::SensorReading& GetReading() const;

  // SensorProxy::Observer overrides.
  void OnSensorInitialized() override;
  void OnSensorReadingChanged() override;
  void OnSensorError(DOMExceptionCode,
                     const String& sanitized_message,
                     const String& unsanitized_message) override;

 private:
  void InitSensorProxyIfNeeded();

  // ExecutionContextLifecycleObserver overrides.
  void ContextDestroyed() override;

  void OnAddConfigurationRequestCompleted(bool);

  void Activate();
  void Deactivate();

  void RequestAddConfiguration();

  void HandleError(DOMExceptionCode,
                   const String& sanitized_message,
                   const String& unsanitized_message = String());

  void NotifyReading();
  void NotifyActivated();
  void NotifyError(DOMException* error);

  double frequency_;
  device::mojom::blink::SensorType type_;
  SensorState state_;
  Member<SensorProxy> sensor_proxy_;
  double last_reported_timestamp_;
  SensorConfigurationPtr configuration_;
  TaskHandle pending_reading_notification_;
  TaskHandle pending_activated_notification_;
  TaskHandle pending_error_notification_;
  bool use_screen_coords_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_H_
