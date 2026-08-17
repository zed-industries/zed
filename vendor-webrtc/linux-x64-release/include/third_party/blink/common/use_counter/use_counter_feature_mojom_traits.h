// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_COMMON_USE_COUNTER_USE_COUNTER_FEATURE_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_COMMON_USE_COUNTER_USE_COUNTER_FEATURE_MOJOM_TRAITS_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/use_counter/use_counter_feature.h"
#include "third_party/blink/public/mojom/use_counter/use_counter_feature.mojom-shared.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::UseCounterFeatureDataView,
                                        blink::UseCounterFeature> {
 public:
  static blink::mojom::UseCounterFeatureType type(
      const blink::UseCounterFeature& feature) {
    return feature.type();
  }

  static uint32_t value(const blink::UseCounterFeature& feature) {
    return feature.value();
  }

  static bool Read(blink::mojom::UseCounterFeatureDataView in,
                   blink::UseCounterFeature* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_COMMON_USE_COUNTER_USE_COUNTER_FEATURE_MOJOM_TRAITS_H_
