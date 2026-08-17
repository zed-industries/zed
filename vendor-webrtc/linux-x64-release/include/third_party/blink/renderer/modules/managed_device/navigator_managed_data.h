// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MANAGED_DEVICE_NAVIGATOR_MANAGED_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MANAGED_DEVICE_NAVIGATOR_MANAGED_DATA_H_

#include "third_party/blink/public/mojom/device/device.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Navigator;
class ExecutionContext;
class ScriptState;

class MODULES_EXPORT NavigatorManagedData final
    : public EventTarget,
      public ActiveScriptWrappable<NavigatorManagedData>,
      public Supplement<Navigator>,
      public mojom::blink::ManagedConfigurationObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Web-based getter for navigator.managed.
  static NavigatorManagedData* managed(Navigator&);

  explicit NavigatorManagedData(Navigator&);
  NavigatorManagedData(const NavigatorManagedData&) = delete;
  NavigatorManagedData& operator=(const NavigatorManagedData&) = delete;

  void Trace(Visitor*) const override;

  // EventTarget:
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;
  void AddedEventListener(
      const AtomicString& event_type,
      RegisteredEventListener& registered_listener) override;
  void RemovedEventListener(
      const AtomicString& event_type,
      const RegisteredEventListener& registered_listener) override;

  // ScriptWrappable implementation.
  bool HasPendingActivity() const final;

  // Managed Configuration API:
  ScriptPromise<IDLRecord<IDLString, IDLAny>> getManagedConfiguration(
      ScriptState* script_state,
      Vector<String> keys);
  DEFINE_ATTRIBUTE_EVENT_LISTENER(managedconfigurationchange,
                                  kManagedconfigurationchange)

  // Device Attributes API:
  ScriptPromise<IDLNullable<IDLString>> getDirectoryId(ScriptState*,
                                                       ExceptionState&);
  ScriptPromise<IDLNullable<IDLString>> getHostname(ScriptState*,
                                                    ExceptionState&);
  ScriptPromise<IDLNullable<IDLString>> getSerialNumber(ScriptState*,
                                                        ExceptionState&);
  ScriptPromise<IDLNullable<IDLString>> getAnnotatedAssetId(ScriptState*,
                                                            ExceptionState&);
  ScriptPromise<IDLNullable<IDLString>> getAnnotatedLocation(ScriptState*,
                                                             ExceptionState&);

 private:
  bool CheckDeviceAttributesAllowed(ExceptionState&);
  // ManagedConfigurationObserver:
  void OnConfigurationChanged() override;

  void OnConfigurationReceived(
      ScriptPromiseResolver<IDLRecord<IDLString, IDLAny>>* scoped_resolver,
      const std::optional<HashMap<String, String>>& configurations);

  void OnAttributeReceived(
      ScriptState* script_state,
      ScriptPromiseResolver<IDLNullable<IDLString>>* resolver,
      mojom::blink::DeviceAttributeResultPtr result);

  // Lazily binds mojo interface.
  mojom::blink::DeviceAPIService* GetService();

  // Lazily binds mojo interface.
  mojom::blink::ManagedConfigurationService* GetManagedConfigurationService();

  void OnServiceConnectionError();
  void StopObserving();

  HeapMojoRemote<mojom::blink::DeviceAPIService> device_api_service_;
  HeapMojoRemote<mojom::blink::ManagedConfigurationService>
      managed_configuration_service_;

  HeapMojoReceiver<mojom::blink::ManagedConfigurationObserver,
                   NavigatorManagedData>
      configuration_observer_;
  HeapHashSet<Member<ScriptPromiseResolverBase>> pending_promises_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MANAGED_DEVICE_NAVIGATOR_MANAGED_DATA_H_
