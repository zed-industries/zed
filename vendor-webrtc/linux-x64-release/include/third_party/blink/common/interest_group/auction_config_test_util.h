// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_COMMON_INTEREST_GROUP_AUCTION_CONFIG_TEST_UTIL_H_
#define THIRD_PARTY_BLINK_COMMON_INTEREST_GROUP_AUCTION_CONFIG_TEST_UTIL_H_

#include "third_party/blink/public/common/interest_group/auction_config.h"
#include "url/gurl.h"
#include "url/origin.h"

namespace blink {

// Creates a minimal valid AuctionConfig, with a seller and the passed in
// decision logic URL. Seller is derived from `decision_logic_url`.
AuctionConfig CreateBasicAuctionConfig(
    const GURL& decision_logic_url = GURL("https://seller.test/foo"));

// Creates an AuctionConfig with all fields except `component_auctions`
// populated.
AuctionConfig CreateFullAuctionConfig();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_COMMON_INTEREST_GROUP_AUCTION_CONFIG_TEST_UTIL_H_
