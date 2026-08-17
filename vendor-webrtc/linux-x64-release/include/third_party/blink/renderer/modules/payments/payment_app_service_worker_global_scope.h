// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_APP_SERVICE_WORKER_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_APP_SERVICE_WORKER_GLOBAL_SCOPE_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PaymentAppServiceWorkerGlobalScope {
  STATIC_ONLY(PaymentAppServiceWorkerGlobalScope);

 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(abortpayment, kAbortpayment)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(canmakepayment, kCanmakepayment)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(paymentrequest, kPaymentrequest)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_APP_SERVICE_WORKER_GLOBAL_SCOPE_H_
