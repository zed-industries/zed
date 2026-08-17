// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_IMPRESSION_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_IMPRESSION_MOJOM_TRAITS_H_

#include "mojo/public/cpp/bindings/struct_traits.h"
#include "services/network/public/mojom/attribution.mojom-shared.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/navigation/impression.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/conversions/conversions.mojom.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::ImpressionDataView, blink::Impression> {
  static const blink::AttributionSrcToken& attribution_src_token(
      const blink::Impression& r) {
    return r.attribution_src_token;
  }

  static bool is_empty_attribution_src_tag(const blink::Impression& r) {
    return r.is_empty_attribution_src_tag;
  }

  static bool Read(blink::mojom::ImpressionDataView r, blink::Impression* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_IMPRESSION_MOJOM_TRAITS_H_
