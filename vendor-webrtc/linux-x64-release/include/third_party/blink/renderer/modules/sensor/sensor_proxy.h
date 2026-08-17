// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_PROXY_H_

#include "services/device/public/cpp/generic_sensor/sensor_reading.h"
#include "services/device/public/cpp/generic_sensor/sensor_reading_shared_buffer.h"
#include "services/device/public/cpp/generic_sensor/sensor_reading_shared_buffer_reader.h"
#include "services/device/public/mojom/sensor.mojom-blink-forward.h"
#include "services/device/public/mojom/sensor_provider.mojom-blink.h"
#include "third_party/blink/renderer/core/page/focus_changed_observer.h"
#include "third_party/blink/renderer/core/page/page_visibility_observer.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SensorProviderProxy;

// This class wraps 'Sensor' mojo interface and used by multiple
// JS sensor instances of the same type (within a single frame).
class SensorProxy : public GarbageCollected<SensorProxy>,
                    public PageVisibilityObserver,
                    public FocusChangedObserver {
 public:
  class Observer : public GarbageCollectedMixin {
   public:
    // Has valid 'Sensor' binding, {add, remove}Configuration()
    // methods can be called.
    virtual void OnSensorInitialized() {}
    // Observer should update its cached reading and send 'onchange'
    // event if needed.
    virtual void OnSensorReadingChanged() {}
    // An error has occurred.
    virtual void OnSensorError(DOMExceptionCode,
                               const String& sanitized_message,
                               const String& unsanitized_message) {}
  };

  SensorProxy(const SensorProxy&) = delete;
  SensorProxy& operator=(const SensorProxy&) = delete;

  ~SensorProxy() override;

  void Dispose();

  void AddObserver(Observer*);
  void RemoveObserver(Observer*);

  // Public methods to be implemented by descendants.
  virtual void Initialize() = 0;
  virtual void AddConfiguration(device::mojom::blink::SensorConfigurationPtr,
                                base::OnceCallback<void(bool)>) = 0;
  virtual void RemoveConfiguration(
      device::mojom::blink::SensorConfigurationPtr) = 0;
  virtual double GetDefaultFrequency() const = 0;
  virtual std::pair<double, double> GetFrequencyLimits() const = 0;

  virtual void ReportError(DOMExceptionCode code, const String& description);
  // Getters.
  bool IsInitializing() const { return state_ == kInitializing; }
  bool IsInitialized() const { return state_ == kInitialized; }
  device::mojom::blink::SensorType type() const { return type_; }
  // Note: do not use the stored references to the returned value
  // outside the current call chain.
  const device::SensorReading& GetReading(bool remapped = false) const;

  // Detach from the local frame's SensorProviderProxy.
  void Detach();

  void Trace(Visitor*) const override;

  static const char kDefaultErrorDescription[];

 protected:
  SensorProxy(device::mojom::blink::SensorType, SensorProviderProxy*, Page*);
  void UpdateSuspendedStatus();

  // Protected methods to be implemented by descendants.
  virtual void Suspend() {}
  virtual void Resume() {}

  SensorProviderProxy* sensor_provider_proxy() const { return provider_.Get(); }

  device::mojom::blink::SensorType type_;
  using ObserversSet = HeapHashSet<WeakMember<Observer>>;
  ObserversSet observers_;

  enum State { kUninitialized, kInitializing, kInitialized };
  State state_ = kUninitialized;

  device::SensorReading reading_;
  mutable device::SensorReading remapped_reading_;

 private:
  // PageVisibilityObserver overrides.
  void PageVisibilityChanged() override;

  // FocusChangedObserver overrides.
  void FocusedFrameChanged() override;

  // Returns true if conditions to suspend sensor reading updates are met.
  bool ShouldSuspendUpdates() const;

  Member<SensorProviderProxy> provider_;
  bool detached_ = false;

  static_assert(
      sizeof(device::SensorReadingSharedBuffer) ==
          device::mojom::blink::SensorInitParams::kReadBufferSizeForTests,
      "Check reading buffer size for tests");
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_PROXY_H_
