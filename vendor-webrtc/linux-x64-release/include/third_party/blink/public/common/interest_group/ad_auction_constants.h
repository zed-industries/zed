// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_AUCTION_CONSTANTS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_AUCTION_CONSTANTS_H_

#include <cstddef>

#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// Default value for MaxAdAuctionAdComponents() if nothing is configured.
inline constexpr size_t kMaxAdAuctionAdComponentsDefault = 20;

// Highest value MaxAdAuctionAdComponents() can be set to via feature params.
inline constexpr size_t kMaxAdAuctionAdComponentsConfigLimit = 100;

// Maximum number of ad components a bid in an auction can have. When a renderer
// retrieves the component ad URLs from an auction, the API acts as if exactly
// this number of ad components were returned by padding the returned list with
// additional obfuscated URN URLs that map to about:blank, to avoid creating a
// side channel from a bidder worklet to the main ad.
//
// This is based on experiment configuration (so isn't entirely trivial to
// call), but will not exceed kMaxAdAuctionAdComponentsConfigLimit.
size_t BLINK_COMMON_EXPORT MaxAdAuctionAdComponents();

// The maximum lifetime for interest group expiration, controlled by feature
// params. Defaults to 30 days if the flag is off.
base::TimeDelta BLINK_COMMON_EXPORT MaxInterestGroupLifetime();

// The maximum lifetime for interest group metadata expiration (join/bid/win
// history), controlled by feature params. Defaults to 30 days if the flag is
// off.
base::TimeDelta BLINK_COMMON_EXPORT MaxInterestGroupLifetimeForMetadata();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_AUCTION_CONSTANTS_H_
