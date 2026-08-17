// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_REQUEST_RESPOND_WITH_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_REQUEST_RESPOND_WITH_OBSERVER_H_

#include "third_party/blink/public/mojom/payments/payment_app.mojom-blink.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_error_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/respond_with_observer.h"

namespace blink {

class ExecutionContext;
class PaymentHandlerResponse;
class WaitUntilObserver;

// Implementation for PaymentRequestEvent.respondWith(), which is used by the
// payment handler to provide a payment response when the payment successfully
// completes.
class MODULES_EXPORT PaymentRequestRespondWithObserver final
    : public RespondWithObserver {
 public:
  PaymentRequestRespondWithObserver(ExecutionContext*,
                                    int event_id,
                                    WaitUntilObserver*);
  ~PaymentRequestRespondWithObserver() override = default;

  static PaymentRequestRespondWithObserver* Create(ExecutionContext*,
                                                   int event_id,
                                                   WaitUntilObserver*);

  void OnResponseFulfilled(ScriptState*, PaymentHandlerResponse*);
  void OnResponseRejected(mojom::ServiceWorkerResponseError) override;
  void OnNoResponse(ScriptState*) override;

  void Trace(Visitor*) const override;

  void set_should_have_payer_name(bool should_have_payer_name) {
    should_have_payer_name_ = should_have_payer_name;
  }
  void set_should_have_payer_email(bool should_have_payer_email) {
    should_have_payer_email_ = should_have_payer_email;
  }
  void set_should_have_payer_phone(bool should_have_payer_phone) {
    should_have_payer_phone_ = should_have_payer_phone;
  }
  void set_should_have_shipping_info(bool should_have_shipping_info) {
    should_have_shipping_info_ = should_have_shipping_info;
  }

 private:
  void Respond(const String& method_name,
               const String& stringified_details,
               payments::mojom::blink::PaymentEventResponseType response_type,
               const String& payer_name,
               const String& payer_email,
               const String& payer_phone,
               payments::mojom::blink::PaymentAddressPtr shipping_address,
               const String& selected_shipping_option_id);
  void BlankResponseWithError(
      payments::mojom::blink::PaymentEventResponseType response_type);
  bool should_have_payer_name_ = false;
  bool should_have_payer_email_ = false;
  bool should_have_payer_phone_ = false;
  bool should_have_shipping_info_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_REQUEST_RESPOND_WITH_OBSERVER_H_
