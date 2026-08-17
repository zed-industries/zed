// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_UPDATE_PAYMENT_DETAILS_FUNCTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_UPDATE_PAYMENT_DETAILS_FUNCTION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"

namespace blink {
class PaymentDetailsUpdate;
class PaymentRequestDelegate;
class ScriptState;
class ScriptValue;

class UpdatePaymentDetailsResolve
    : public ThenCallable<PaymentDetailsUpdate, UpdatePaymentDetailsResolve> {
 public:
  explicit UpdatePaymentDetailsResolve(PaymentRequestDelegate*);
  void Trace(Visitor*) const override;
  void React(ScriptState*, PaymentDetailsUpdate*);

 private:
  Member<PaymentRequestDelegate> delegate_;
};

class UpdatePaymentDetailsReject
    : public ThenCallable<IDLAny, UpdatePaymentDetailsReject> {
 public:
  explicit UpdatePaymentDetailsReject(PaymentRequestDelegate*);
  void Trace(Visitor*) const override;
  void React(ScriptState*, ScriptValue);

 private:
  Member<PaymentRequestDelegate> delegate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_UPDATE_PAYMENT_DETAILS_FUNCTION_H_
