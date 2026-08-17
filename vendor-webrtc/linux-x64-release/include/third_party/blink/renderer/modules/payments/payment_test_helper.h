// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_TEST_HELPER_H_

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/mojom/payments/payment_request.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_function.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_testing.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_dom_exception.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_payment_details_init.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_payment_details_update.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_payment_item.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_payment_shipping_option.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_secure_payment_confirmation_request.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class PaymentMethodData;
class V8TestingScope;

enum PaymentTestDetailToChange {
  kPaymentTestDetailNone,
  kPaymentTestDetailTotal,
  kPaymentTestDetailItem,
  kPaymentTestDetailShippingOption,
  kPaymentTestDetailModifierTotal,
  kPaymentTestDetailModifierItem,
  kPaymentTestDetailError
};

enum PaymentTestDataToChange {
  kPaymentTestDataNone,
  kPaymentTestDataId,
  kPaymentTestDataLabel,
  kPaymentTestDataAmount,
  kPaymentTestDataCurrencyCode,
  kPaymentTestDataValue,
};

enum PaymentTestModificationType {
  kPaymentTestOverwriteValue,
  kPaymentTestRemoveKey
};

PaymentItem* BuildPaymentItemForTest(
    PaymentTestDataToChange = kPaymentTestDataNone,
    PaymentTestModificationType = kPaymentTestOverwriteValue,
    const String& value_to_use = String());

PaymentShippingOption* BuildShippingOptionForTest(
    PaymentTestDataToChange = kPaymentTestDataNone,
    PaymentTestModificationType = kPaymentTestOverwriteValue,
    const String& value_to_use = String());

PaymentDetailsModifier* BuildPaymentDetailsModifierForTest(
    PaymentTestDetailToChange = kPaymentTestDetailNone,
    PaymentTestDataToChange = kPaymentTestDataNone,
    PaymentTestModificationType = kPaymentTestOverwriteValue,
    const String& value_to_use = String());

PaymentDetailsInit* BuildPaymentDetailsInitForTest(
    PaymentTestDetailToChange = kPaymentTestDetailNone,
    PaymentTestDataToChange = kPaymentTestDataNone,
    PaymentTestModificationType = kPaymentTestOverwriteValue,
    const String& value_to_use = String());

PaymentDetailsUpdate* BuildPaymentDetailsUpdateForTest(
    PaymentTestDetailToChange = kPaymentTestDetailNone,
    PaymentTestDataToChange = kPaymentTestDataNone,
    PaymentTestModificationType = kPaymentTestOverwriteValue,
    const String& value_to_use = String());

PaymentDetailsUpdate* BuildPaymentDetailsErrorMsgForTest(
    const String& value_to_use = String());

HeapVector<Member<PaymentMethodData>> BuildPaymentMethodDataForTest();

payments::mojom::blink::PaymentResponsePtr BuildPaymentResponseForTest();

payments::mojom::blink::PaymentAddressPtr BuildPaymentAddressForTest();

class PaymentRequestV8TestingScope : public V8TestingScope {
  STACK_ALLOCATED();

 public:
  PaymentRequestV8TestingScope();
};

const uint8_t kSecurePaymentConfirmationCredentialId[] = {
    0x63, 0x72, 0x65, 0x64, 0x65, 0x6E, 0x74, 0x69, 0x61, 0x6C};
const uint8_t kSecurePaymentConfirmationChallenge[] = {
    0x63, 0x68, 0x61, 0x6C, 0x6C, 0x65, 0x6E, 0x67, 0x65};

// Creates and returns a minimal SecurePaymentConfirmationRequest object with
// only required fields filled in to pass parsing.
//
// If include_payee_name is set to false, this function will not include the
// payeeName field which is not required by IDL (and thus not required for
// conversion to ScriptValue), but is required by the parsing code.
SecurePaymentConfirmationRequest* CreateSecurePaymentConfirmationRequest(
    const V8TestingScope& scope,
    const bool include_payee_name = true);

HeapVector<Member<PaymentMethodData>>
BuildSecurePaymentConfirmationMethodDataForTest(const V8TestingScope& scope);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_TEST_HELPER_H_
