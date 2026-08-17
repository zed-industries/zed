// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENTS_VALIDATORS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENTS_VALIDATORS_H_

#include "third_party/blink/public/mojom/payments/payment_request.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class AddressErrors;
class ExceptionState;
class PayerErrors;
class PaymentValidationErrors;
class ScriptValue;

class MODULES_EXPORT PaymentsValidators final {
  STATIC_ONLY(PaymentsValidators);

 public:
  // The most common identifiers are three-letter alphabetic codes as defined by
  // [ISO4217] (for example, "USD" for US Dollars).
  static bool IsValidCurrencyCodeFormat(v8::Isolate* isolate,
                                        const String& code,
                                        String* optional_error_message);

  // Returns true if |amount| is a valid currency code as defined in
  // PaymentRequest standard.
  static bool IsValidAmountFormat(v8::Isolate* isolate,
                                  const String& amount,
                                  const String& item_name,
                                  String* optional_error_message);

  // Returns true if |code| is a valid ISO 3166 country code.
  static bool IsValidCountryCodeFormat(v8::Isolate* isolate,
                                       const String& code,
                                       String* optional_error_message);

  // Returns true if the payment address is valid:
  //  - Has a valid region code
  static bool IsValidShippingAddress(
      v8::Isolate* isolate,
      const payments::mojom::blink::PaymentAddressPtr&,
      String* optional_error_message);

  // Returns false if |error| is too long.
  static bool IsValidErrorMsgFormat(const String& code,
                                    String* optional_error_message);

  // Returns false if |errors| has too long string.
  static bool IsValidAddressErrorsFormat(const AddressErrors* errors,
                                         String* optional_error_message);

  // Returns false if |errors| has too long string.
  static bool IsValidPayerErrorsFormat(const PayerErrors* errors,
                                       String* optional_error_message);

  // Returns false if |errors| has too long string.
  static bool IsValidPaymentValidationErrorsFormat(
      const PaymentValidationErrors* errors,
      String* optional_error_message);

  // Implements the PMI validation algorithm from:
  // https://www.w3.org/TR/payment-method-id/#dfn-validate-a-payment-method-identifier
  static bool IsValidMethodFormat(v8::Isolate* isolate,
                                  const String& identifier);

  // Validates that |input| is a JavaScript object that can be serialized into
  // JSON string of a reasonable size.
  //
  // If the |input| is valid, the JSON serialization is saved in |output|.
  //
  // If the |input| is invalid, throws a TypeError through the
  // |exception_state|.
  static void ValidateAndStringifyObject(v8::Isolate* isolate,
                                         const ScriptValue& input,
                                         String& output,
                                         ExceptionState& exception_state);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENTS_VALIDATORS_H_
