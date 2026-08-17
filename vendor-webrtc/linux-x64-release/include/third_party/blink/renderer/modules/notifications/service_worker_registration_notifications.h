// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_SERVICE_WORKER_REGISTRATION_NOTIFICATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_SERVICE_WORKER_REGISTRATION_NOTIFICATIONS_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/notifications/notification.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExecutionContext;
class ExceptionState;
class GetNotificationOptions;
class Notification;
class NotificationOptions;
class NotificationResourcesLoader;
class ScriptState;
class SecurityOrigin;
class ServiceWorkerRegistration;

class ServiceWorkerRegistrationNotifications final
    : public GarbageCollected<ServiceWorkerRegistrationNotifications>,
      public Supplement<ServiceWorkerRegistration>,
      public ExecutionContextLifecycleObserver {
 public:
  static const char kSupplementName[];

  static ScriptPromise<IDLUndefined> showNotification(
      ScriptState* script_state,
      ServiceWorkerRegistration& registration,
      const String& title,
      const NotificationOptions* options,
      ExceptionState& exception_state);
  static ScriptPromise<IDLSequence<Notification>> getNotifications(
      ScriptState* script_state,
      ServiceWorkerRegistration& registration,
      const GetNotificationOptions* options);

  ServiceWorkerRegistrationNotifications(ExecutionContext*,
                                         ServiceWorkerRegistration*);

  ServiceWorkerRegistrationNotifications(
      const ServiceWorkerRegistrationNotifications&) = delete;
  ServiceWorkerRegistrationNotifications& operator=(
      const ServiceWorkerRegistrationNotifications&) = delete;

  // ExecutionContextLifecycleObserver interface.
  void ContextDestroyed() override;

  void Trace(Visitor* visitor) const override;

 private:
  static ServiceWorkerRegistrationNotifications& From(
      ExecutionContext* context,
      ServiceWorkerRegistration& registration);

  void PrepareShow(mojom::blink::NotificationDataPtr data,
                   ScriptPromiseResolver<IDLUndefined>* resolver);

  void DidLoadResources(scoped_refptr<const SecurityOrigin> origin,
                        mojom::blink::NotificationDataPtr data,
                        ScriptPromiseResolver<IDLUndefined>* resolver,
                        NotificationResourcesLoader* loader);

  HeapHashSet<Member<NotificationResourcesLoader>> loaders_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_SERVICE_WORKER_REGISTRATION_NOTIFICATIONS_H_
