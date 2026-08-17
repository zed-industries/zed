/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CONTAINER_H_

#include <memory>

#include "third_party/blink/public/mojom/script/script_type.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_registration.mojom-blink-forward.h"
#include "third_party/blink/public/platform/modules/service_worker/web_service_worker_provider.h"
#include "third_party/blink/public/platform/modules/service_worker/web_service_worker_provider_client.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_registration_options.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/message_from_service_worker.h"
#include "third_party/blink/renderer/modules/service_worker/service_worker.h"
#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExecutionContext;
class ExceptionState;
class LocalDOMWindow;
class ServiceWorkerRegistration;

class MODULES_EXPORT ServiceWorkerContainer final
    : public EventTarget,
      public Supplement<LocalDOMWindow>,
      public ExecutionContextLifecycleObserver,
      public WebServiceWorkerProviderClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  static ServiceWorkerContainer* From(LocalDOMWindow&);

  static ServiceWorkerContainer* CreateForTesting(
      LocalDOMWindow&,
      std::unique_ptr<WebServiceWorkerProvider>);

  explicit ServiceWorkerContainer(LocalDOMWindow&);
  ~ServiceWorkerContainer() override;

  void Trace(Visitor*) const override;

  ServiceWorker* controller() { return controller_.Get(); }
  ScriptPromise<ServiceWorkerRegistration> ready(ScriptState*, ExceptionState&);

  ScriptPromise<ServiceWorkerRegistration> registerServiceWorker(
      ScriptState*,
      const String& pattern,
      const RegistrationOptions*);
  ScriptPromise<ServiceWorkerRegistration> getRegistration(
      ScriptState*,
      const String& document_url);
  ScriptPromise<IDLSequence<ServiceWorkerRegistration>> getRegistrations(
      ScriptState*);

  void startMessages();

  void ContextDestroyed() override;

  // WebServiceWorkerProviderClient implementation.
  void SetController(WebServiceWorkerObjectInfo,
                     bool should_notify_controller_change) override;
  void ReceiveMessage(WebServiceWorkerObjectInfo source,
                      TransferableMessage) override;
  void CountFeature(mojom::WebFeature) override;

  // EventTarget overrides.
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(controllerchange, kControllerchange)

  void setOnmessage(EventListener* listener);
  EventListener* onmessage();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(messageerror, kMessageerror)

  // Returns the ServiceWorkerRegistration object described by the given info.
  // Creates a new object if needed, or else returns the existing one.
  ServiceWorkerRegistration* GetOrCreateServiceWorkerRegistration(
      WebServiceWorkerRegistrationObjectInfo);

  // Returns the ServiceWorker object described by the given info. Creates a new
  // object if needed, or else returns the existing one.
  ServiceWorker* GetOrCreateServiceWorker(WebServiceWorkerObjectInfo);

 private:
  class DomContentLoadedListener;

  using RegistrationCallbacks =
      WebServiceWorkerProvider::WebServiceWorkerRegistrationCallbacks;

  void RegisterServiceWorkerInternal(
      const KURL& scope_url,
      const KURL& script_url,
      std::optional<mojom::blink::ScriptType> script_type,
      mojom::blink::ServiceWorkerUpdateViaCache update_via_cache,
      WebFetchClientSettingsObject fetch_client_settings_object,
      std::unique_ptr<RegistrationCallbacks> callbacks);

  using ReadyProperty = ScriptPromiseProperty<ServiceWorkerRegistration,
                                              ServiceWorkerRegistration>;
  ReadyProperty* CreateReadyProperty();

  void EnableClientMessageQueue();
  void DispatchMessageEvent(WebServiceWorkerObjectInfo source,
                            TransferableMessage);

  void OnGetRegistrationForReady(WebServiceWorkerRegistrationObjectInfo info);

  std::unique_ptr<WebServiceWorkerProvider> provider_;
  Member<ServiceWorker> controller_;
  Member<ReadyProperty> ready_;

  // Map from service worker registration id to JavaScript
  // ServiceWorkerRegistration object in current execution context.
  HeapHashMap<int64_t,
              WeakMember<ServiceWorkerRegistration>,
              IntWithZeroKeyHashTraits<int64_t>>
      service_worker_registration_objects_;
  // Map from service worker version id to JavaScript ServiceWorker object in
  // current execution context.
  HeapHashMap<int64_t,
              WeakMember<ServiceWorker>,
              IntWithZeroKeyHashTraits<int64_t>>
      service_worker_objects_;

  // For https://w3c.github.io/ServiceWorker/#dfn-client-message-queue
  // Rather than pausing/resuming the task runner, we implement enabling the
  // queue using this flag since the task runner is shared with other task
  // sources.
  bool is_client_message_queue_enabled_ = false;
  Vector<std::unique_ptr<MessageFromServiceWorker>> queued_messages_;
  Member<DomContentLoadedListener> dom_content_loaded_observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CONTAINER_H_
