// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_APP_SERVICE_WORKER_REGISTRATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_APP_SERVICE_WORKER_REGISTRATION_H_

#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class PaymentManager;
class ScriptState;
class ServiceWorkerRegistration;

class PaymentAppServiceWorkerRegistration final
    : public GarbageCollected<PaymentAppServiceWorkerRegistration>,
      public Supplement<ServiceWorkerRegistration> {
 public:
  static const char kSupplementName[];

  explicit PaymentAppServiceWorkerRegistration(ServiceWorkerRegistration*);

  PaymentAppServiceWorkerRegistration(
      const PaymentAppServiceWorkerRegistration&) = delete;
  PaymentAppServiceWorkerRegistration& operator=(
      const PaymentAppServiceWorkerRegistration&) = delete;

  ~PaymentAppServiceWorkerRegistration();

  static PaymentAppServiceWorkerRegistration& From(ServiceWorkerRegistration&);

  static PaymentManager* paymentManager(ScriptState*,
                                        ServiceWorkerRegistration&,
                                        ExceptionState&);
  PaymentManager* paymentManager(ScriptState*, ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  Member<PaymentManager> payment_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_APP_SERVICE_WORKER_REGISTRATION_H_
