// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_DEVTOOLS_SERIALIZATION_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_DEVTOOLS_SERIALIZATION_H_

#include "base/values.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

struct AuctionConfig;
struct InterestGroup;

// Serializes the configuration in a manner suitable for sending to devtools.
base::Value::Dict BLINK_COMMON_EXPORT
SerializeAuctionConfigForDevtools(const AuctionConfig& conf);

// Serializes the interest grouo in a manner suitable for sending to devtools.
base::Value::Dict BLINK_COMMON_EXPORT
SerializeInterestGroupForDevtools(const InterestGroup& ig);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_DEVTOOLS_SERIALIZATION_H_
