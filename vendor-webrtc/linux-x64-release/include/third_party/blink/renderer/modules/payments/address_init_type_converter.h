// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_ADDRESS_INIT_TYPE_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_ADDRESS_INIT_TYPE_CONVERTER_H_

#include "components/payments/mojom/payment_request_data.mojom-blink.h"
#include "mojo/public/cpp/bindings/type_converter.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_address_init.h"

namespace mojo {

template <>
struct TypeConverter<payments::mojom::blink::PaymentAddressPtr,
                     blink::AddressInit*> {
  static payments::mojom::blink::PaymentAddressPtr Convert(
      const blink::AddressInit* input);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_ADDRESS_INIT_TYPE_CONVERTER_H_
