// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_SECURE_PAYMENT_CONFIRMATION_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_SECURE_PAYMENT_CONFIRMATION_HELPER_H_

#include "third_party/blink/public/mojom/payments/payment_request.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ScriptValue;
class ExecutionContext;
class ExceptionState;

class MODULES_EXPORT SecurePaymentConfirmationHelper {
  STATIC_ONLY(SecurePaymentConfirmationHelper);

 public:
  // Parse 'secure-payment-confirmation' data in |input| and return the result
  // or throw an exception.
  static ::payments::mojom::blink::SecurePaymentConfirmationRequestPtr
  ParseSecurePaymentConfirmationData(const ScriptValue& input,
                                     ExecutionContext&,
                                     ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_SECURE_PAYMENT_CONFIRMATION_HELPER_H_
