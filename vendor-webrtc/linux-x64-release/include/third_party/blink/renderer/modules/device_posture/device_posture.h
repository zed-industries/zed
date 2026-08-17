// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_POSTURE_DEVICE_POSTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_POSTURE_DEVICE_POSTURE_H_

#include "third_party/blink/public/mojom/device_posture/device_posture_provider.mojom-blink.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class LocalDOMWindow;
class V8DevicePostureType;

class MODULES_EXPORT DevicePosture : public EventTarget,
                                     public ExecutionContextClient,
                                     public mojom::blink::DevicePostureClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit DevicePosture(LocalDOMWindow*);
  ~DevicePosture() override;

  // Web-exposed interfaces
  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)
  V8DevicePostureType type();

  // EventTarget overrides.
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  void Trace(blink::Visitor*) const override;

 private:
  // DevicePostureClient
  void OnPostureChanged(mojom::blink::DevicePostureType posture) override;
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;
  void OnServiceConnectionError();
  void EnsureServiceConnection();

  mojom::blink::DevicePostureType posture_ =
      mojom::blink::DevicePostureType::kContinuous;
  HeapMojoReceiver<mojom::blink::DevicePostureClient, DevicePosture> receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DEVICE_POSTURE_DEVICE_POSTURE_H_
