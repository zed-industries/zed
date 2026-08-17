// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_ADDRESS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_ADDRESS_H_

#include "third_party/blink/public/mojom/payments/payment_request.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class MODULES_EXPORT PaymentAddress : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit PaymentAddress(payments::mojom::blink::PaymentAddressPtr);

  PaymentAddress(const PaymentAddress&) = delete;
  PaymentAddress& operator=(const PaymentAddress&) = delete;

  ~PaymentAddress() override;

  ScriptObject toJSONForBinding(ScriptState*) const;

  const String& country() const { return country_; }
  const Vector<String>& addressLine() const { return address_line_; }
  const String& region() const { return region_; }
  const String& city() const { return city_; }
  const String& dependentLocality() const { return dependent_locality_; }
  const String& postalCode() const { return postal_code_; }
  const String& sortingCode() const { return sorting_code_; }
  const String& organization() const { return organization_; }
  const String& recipient() const { return recipient_; }
  const String& phone() const { return phone_; }

 private:
  String country_;
  Vector<String> address_line_;
  String region_;
  String city_;
  String dependent_locality_;
  String postal_code_;
  String sorting_code_;
  String organization_;
  String recipient_;
  String phone_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_PAYMENT_ADDRESS_H_
