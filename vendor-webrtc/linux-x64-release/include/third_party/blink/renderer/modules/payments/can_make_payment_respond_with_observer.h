// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_CAN_MAKE_PAYMENT_RESPOND_WITH_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_CAN_MAKE_PAYMENT_RESPOND_WITH_OBSERVER_H_

#include "third_party/blink/public/mojom/payments/payment_app.mojom-blink.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_error_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/respond_with_observer.h"

namespace blink {

class ExecutionContext;
class WaitUntilObserver;

// Implementation for CanMakePaymentEvent.respondWith(), which is used by the
// payment handler to indicate whether it can respond to a payment request.
class MODULES_EXPORT CanMakePaymentRespondWithObserver final
    : public RespondWithObserver {
 public:
  CanMakePaymentRespondWithObserver(ExecutionContext*,
                                    int event_id,
                                    WaitUntilObserver*);
  ~CanMakePaymentRespondWithObserver() override = default;

  void OnResponseFulfilled(ScriptState*, bool);
  void OnResponseRejected(mojom::blink::ServiceWorkerResponseError) override;
  void OnNoResponse(ScriptState*) override;

  void Trace(Visitor*) const override;

 private:
  void Respond(
      payments::mojom::blink::CanMakePaymentEventResponseType response_type,
      bool can_make_payment);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_CAN_MAKE_PAYMENT_RESPOND_WITH_OBSERVER_H_
