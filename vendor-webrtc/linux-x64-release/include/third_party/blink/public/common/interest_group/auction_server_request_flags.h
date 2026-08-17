// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AUCTION_SERVER_REQUEST_FLAGS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AUCTION_SERVER_REQUEST_FLAGS_H_

#include "base/containers/enum_set.h"

namespace blink {

enum class AuctionServerRequestFlagsEnum : uint32_t {
  kOmitAds,
  kIncludeFullAds,
  kOmitUserBiddingSignals,
  kMaxValue = kOmitUserBiddingSignals
};

using AuctionServerRequestFlags =
    base::EnumSet<AuctionServerRequestFlagsEnum,
                  AuctionServerRequestFlagsEnum::kOmitAds,
                  AuctionServerRequestFlagsEnum::kMaxValue>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AUCTION_SERVER_REQUEST_FLAGS_H_
