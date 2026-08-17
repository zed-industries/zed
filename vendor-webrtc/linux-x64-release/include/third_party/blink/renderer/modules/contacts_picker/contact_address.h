// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTACTS_PICKER_CONTACT_ADDRESS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTACTS_PICKER_CONTACT_ADDRESS_H_

#include "third_party/blink/public/mojom/payments/payment_request.mojom-blink.h"
#include "third_party/blink/renderer/modules/payments/payment_address.h"

namespace blink {

class ContactAddress : public PaymentAddress {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ContactAddress(
      payments::mojom::blink::PaymentAddressPtr payment_address);
  ~ContactAddress() override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTACTS_PICKER_CONTACT_ADDRESS_H_
