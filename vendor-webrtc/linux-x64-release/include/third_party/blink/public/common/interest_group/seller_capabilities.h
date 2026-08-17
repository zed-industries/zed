// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_SELLER_CAPABILITIES_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_SELLER_CAPABILITIES_H_

#include "base/containers/enum_set.h"

namespace blink {

enum class SellerCapabilities : uint32_t {
  kInterestGroupCounts,
  kLatencyStats,

  kMaxValue = kLatencyStats
};

using SellerCapabilitiesType =
    base::EnumSet<SellerCapabilities,
                  SellerCapabilities::kInterestGroupCounts,
                  SellerCapabilities::kMaxValue>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_SELLER_CAPABILITIES_H_
