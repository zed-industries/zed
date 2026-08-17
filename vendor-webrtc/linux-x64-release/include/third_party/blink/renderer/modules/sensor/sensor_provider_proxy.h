// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_PROVIDER_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_PROVIDER_PROXY_H_

#include "services/device/public/mojom/sensor.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/sensor/web_sensor_provider.mojom-blink.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class SensorProxy;

// This class wraps 'SensorProvider' mojo interface and it manages
// 'SensorProxy' instances.
class SensorProviderProxy final : public GarbageCollected<SensorProviderProxy>,
                                  public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  static SensorProviderProxy* From(LocalDOMWindow*);

  explicit SensorProviderProxy(LocalDOMWindow&);

  SensorProviderProxy(const SensorProviderProxy&) = delete;
  SensorProviderProxy& operator=(const SensorProviderProxy&) = delete;

  ~SensorProviderProxy();

  SensorProxy* CreateSensorProxy(device::mojom::blink::SensorType, Page*);
  SensorProxy* GetSensorProxy(device::mojom::blink::SensorType);
  void GetSensor(device::mojom::blink::SensorType,
                 mojom::blink::WebSensorProviderProxy::GetSensorCallback);

  void Trace(Visitor*) const override;

 private:
  friend class SensorProxy;

  // For SensorProviderProxy friends' use.
  void RemoveSensorProxy(SensorProxy* proxy);

  // For SensorProviderProxy personal use.
  void InitializeIfNeeded();
  void OnSensorProviderConnectionError();

  HeapHashSet<WeakMember<SensorProxy>> sensor_proxies_;
  HeapMojoRemote<mojom::blink::WebSensorProvider> sensor_provider_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SENSOR_SENSOR_PROVIDER_PROXY_H_
