// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_RESPONSE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_RESPONSE_H_

#include "third_party/blink/public/mojom/payments/payment_request.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/world_safe_v8_reference.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_payment_currency_amount.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class PaymentAddress;
class PaymentStateResolver;
class PaymentValidationErrors;
class ScriptState;
class V8PaymentComplete;

class MODULES_EXPORT PaymentResponse final
    : public EventTarget,
      public ExecutionContextClient,
      public ActiveScriptWrappable<PaymentResponse> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PaymentResponse(ScriptState* script_state,
                  payments::mojom::blink::PaymentResponsePtr response,
                  PaymentAddress* shipping_address,
                  PaymentStateResolver* payment_state_resolver,
                  const String& request_id);

  PaymentResponse(const PaymentResponse&) = delete;
  PaymentResponse& operator=(const PaymentResponse&) = delete;

  ~PaymentResponse() override;

  void Update(ScriptState* script_state,
              payments::mojom::blink::PaymentResponsePtr response,
              PaymentAddress* shipping_address);
  void UpdatePayerDetail(payments::mojom::blink::PayerDetailPtr);

  ScriptObject toJSONForBinding(ScriptState*) const;

  const String& requestId() const { return request_id_; }
  const String& methodName() const { return method_name_; }
  ScriptObject details() const { return details_; }
  PaymentAddress* shippingAddress() const { return shipping_address_.Get(); }
  const String& shippingOption() const { return shipping_option_; }
  const String& payerName() const { return payer_name_; }
  const String& payerEmail() const { return payer_email_; }
  const String& payerPhone() const { return payer_phone_; }

  ScriptPromise<IDLUndefined> complete(ScriptState*,
                                       const V8PaymentComplete& result,
                                       ExceptionState&);
  ScriptPromise<IDLUndefined> retry(ScriptState*,
                                    const PaymentValidationErrors*,
                                    ExceptionState&);

  bool HasPendingActivity() const override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(payerdetailchange, kPayerdetailchange)

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  void Trace(Visitor*) const override;

 private:
  String request_id_;
  String method_name_;
  ScriptObject details_;
  Member<PaymentAddress> shipping_address_;
  String shipping_option_;
  String payer_name_;
  String payer_email_;
  String payer_phone_;
  Member<PaymentStateResolver> payment_state_resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_RESPONSE_H_
