// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_PROVIDER_H_

#include "third_party/blink/public/mojom/push_messaging/push_messaging.mojom-blink.h"
#include "third_party/blink/public/mojom/push_messaging/push_messaging_status.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

namespace mojom {
enum class PushGetRegistrationStatus;
enum class PushRegistrationStatus;
}  // namespace mojom

class PushSubscription;
class PushSubscriptionOptions;

class PushProvider final : public GarbageCollected<PushProvider>,
                           public Supplement<ServiceWorkerRegistration> {
 public:
  static const char kSupplementName[];

  explicit PushProvider(ServiceWorkerRegistration& registration);

  PushProvider(const PushProvider&) = delete;
  PushProvider& operator=(const PushProvider&) = delete;

  ~PushProvider() = default;

  static PushProvider* From(ServiceWorkerRegistration* registration);

  void Subscribe(PushSubscriptionOptions* options,
                 bool user_gesture,
                 ScriptPromiseResolver<PushSubscription>* resolver);
  void Unsubscribe(ScriptPromiseResolver<IDLBoolean>* resolver);
  void GetSubscription(
      ScriptPromiseResolver<IDLNullable<PushSubscription>>* resolver);

  void Trace(Visitor*) const override;

 private:
  // Returns an initialized PushMessaging service. A connection will be
  // established after the first call to this method.
  mojom::blink::PushMessaging* GetPushMessagingRemote();

  void DidSubscribe(ScriptPromiseResolver<PushSubscription>* resolver,
                    mojom::blink::PushRegistrationStatus status,
                    mojom::blink::PushSubscriptionPtr subscription);

  void DidUnsubscribe(ScriptPromiseResolver<IDLBoolean>* resolver,
                      mojom::blink::PushErrorType error_type,
                      bool did_unsubscribe,
                      const WTF::String& error_message);

  void DidGetSubscription(
      ScriptPromiseResolver<IDLNullable<PushSubscription>>* resolver,
      mojom::blink::PushGetRegistrationStatus status,
      mojom::blink::PushSubscriptionPtr subscription);

  HeapMojoRemote<mojom::blink::PushMessaging> push_messaging_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_PROVIDER_H_
