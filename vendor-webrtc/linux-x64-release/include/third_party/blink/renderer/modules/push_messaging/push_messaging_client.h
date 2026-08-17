// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_MESSAGING_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_MESSAGING_CLIENT_H_

#include "third_party/blink/public/mojom/manifest/manifest.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/push_messaging/push_messaging.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

namespace mojom {
enum class PushRegistrationStatus;
}  // namespace mojom

class KURL;
class LocalDOMWindow;
class PushSubscription;
class PushSubscriptionOptions;
class ServiceWorkerRegistration;

class PushMessagingClient final : public GarbageCollected<PushMessagingClient>,
                                  public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  explicit PushMessagingClient(LocalDOMWindow&);

  PushMessagingClient(const PushMessagingClient&) = delete;
  PushMessagingClient& operator=(const PushMessagingClient&) = delete;

  ~PushMessagingClient() = default;

  static PushMessagingClient* From(LocalDOMWindow&);

  void Subscribe(ServiceWorkerRegistration* service_worker_registration,
                 PushSubscriptionOptions* options,
                 bool user_gesture,
                 ScriptPromiseResolver<PushSubscription>* resolver);
  void Trace(Visitor*) const override;

 private:
  // Returns an initialized PushMessaging service. A connection will be
  // established after the first call to this method.
  mojom::blink::PushMessaging* GetPushMessagingRemote();

  void DidGetManifest(ServiceWorkerRegistration* service_worker_registration,
                      mojom::blink::PushSubscriptionOptionsPtr options,
                      bool user_gesture,
                      ScriptPromiseResolver<PushSubscription>* resolver,
                      mojom::blink::ManifestRequestResult result,
                      const KURL& manifest_url,
                      mojom::blink::ManifestPtr manifest);

  void DoSubscribe(ServiceWorkerRegistration* service_worker_registration,
                   mojom::blink::PushSubscriptionOptionsPtr options,
                   bool user_gesture,
                   ScriptPromiseResolver<PushSubscription>* resolver);

  void DidSubscribe(ServiceWorkerRegistration* service_worker_registration,
                    ScriptPromiseResolver<PushSubscription>* resolver,
                    mojom::blink::PushRegistrationStatus status,
                    mojom::blink::PushSubscriptionPtr subscription);

  HeapMojoRemote<mojom::blink::PushMessaging> push_messaging_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_MESSAGING_CLIENT_H_
