// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_DIGITAL_GOODS_TYPE_CONVERTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_DIGITAL_GOODS_TYPE_CONVERTERS_H_

#include "components/digital_goods/mojom/digital_goods.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_item_details.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_purchase_details.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace mojo {

template <typename T, typename U>
struct TypeConverter;

template <>
struct MODULES_EXPORT
    TypeConverter<WTF::String,
                  payments::mojom::blink::CreateDigitalGoodsResponseCode> {
  static WTF::String Convert(
      const payments::mojom::blink::CreateDigitalGoodsResponseCode& input);
};

// Converts a mojo ItemDetails into a WebIDL ItemDetails.
// Returns a null IDL struct when a null mojo struct is given as input.
template <>
struct MODULES_EXPORT
    TypeConverter<blink::ItemDetails*, payments::mojom::blink::ItemDetailsPtr> {
  static blink::ItemDetails* Convert(
      const payments::mojom::blink::ItemDetailsPtr& input);
};

template <>
struct MODULES_EXPORT
    TypeConverter<WTF::String, payments::mojom::blink::BillingResponseCode> {
  static WTF::String Convert(
      const payments::mojom::blink::BillingResponseCode& input);
};

// Converts a mojo PurchaseReference into a WebIDL PurchaseDetails.
// Returns a null IDL struct when a null mojo struct is given as input.
template <>
struct MODULES_EXPORT
    TypeConverter<blink::PurchaseDetails*,
                  payments::mojom::blink::PurchaseReferencePtr> {
  static blink::PurchaseDetails* Convert(
      const payments::mojom::blink::PurchaseReferencePtr& input);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_GOODS_DIGITAL_GOODS_TYPE_CONVERTERS_H_
