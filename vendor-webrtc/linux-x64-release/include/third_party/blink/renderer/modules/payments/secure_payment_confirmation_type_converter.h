// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_SECURE_PAYMENT_CONFIRMATION_TYPE_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_SECURE_PAYMENT_CONFIRMATION_TYPE_CONVERTER_H_

#include "mojo/public/cpp/bindings/type_converter.h"
#include "third_party/blink/public/mojom/payments/payment_request.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_secure_payment_confirmation_request.h"

namespace mojo {

template <>
struct TypeConverter<
    payments::mojom::blink::SecurePaymentConfirmationRequestPtr,
    blink::SecurePaymentConfirmationRequest*> {
  static payments::mojom::blink::SecurePaymentConfirmationRequestPtr Convert(
      const blink::SecurePaymentConfirmationRequest* input);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_SECURE_PAYMENT_CONFIRMATION_TYPE_CONVERTER_H_
