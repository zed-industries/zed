// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_TEST_INTEREST_GROUP_BUILDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_TEST_INTEREST_GROUP_BUILDER_H_

#include <stdint.h>

#include <optional>
#include <set>
#include <string>
#include <vector>

#include "base/containers/enum_set.h"
#include "base/containers/flat_map.h"
#include "base/time/time.h"
#include "third_party/blink/public/common/interest_group/interest_group.h"
#include "third_party/blink/public/mojom/interest_group/interest_group_types.mojom-shared.h"
#include "url/gurl.h"
#include "url/origin.h"

namespace blink {

// Test-only single-use builder for interest groups. Uses empty defaults, and a
// 30 day (from construction time) expiry.
class TestInterestGroupBuilder {
 public:
  TestInterestGroupBuilder(url::Origin owner, std::string name);

  ~TestInterestGroupBuilder();

  InterestGroup Build();

  TestInterestGroupBuilder& SetExpiry(base::Time expiry);
  TestInterestGroupBuilder& SetPriority(double priority);
  TestInterestGroupBuilder& SetEnableBiddingSignalsPrioritization(
      bool enable_bidding_signals_prioritization);
  TestInterestGroupBuilder& SetPriorityVector(
      std::optional<base::flat_map<std::string, double>> priority_vector);
  TestInterestGroupBuilder& SetPrioritySignalsOverrides(
      std::optional<base::flat_map<std::string, double>>
          priority_signals_overrides);
  TestInterestGroupBuilder& SetSellerCapabilities(
      std::optional<base::flat_map<url::Origin, SellerCapabilitiesType>>
          seller_capabilities);
  TestInterestGroupBuilder& SetAllSellersCapabilities(
      SellerCapabilitiesType all_sellers_capabilities);
  TestInterestGroupBuilder& SetExecutionMode(
      InterestGroup::ExecutionMode execution_mode);
  TestInterestGroupBuilder& SetBiddingUrl(std::optional<GURL> bidding_url);
  TestInterestGroupBuilder& SetBiddingWasmHelperUrl(
      std::optional<GURL> bidding_wasm_helper_url);
  TestInterestGroupBuilder& SetUpdateUrl(std::optional<GURL> update_url);
  TestInterestGroupBuilder& SetTrustedBiddingSignalsUrl(
      std::optional<GURL> trusted_bidding_signals_url);
  TestInterestGroupBuilder& SetTrustedBiddingSignalsKeys(
      std::optional<std::vector<std::string>> trusted_bidding_signals_keys);
  TestInterestGroupBuilder& SetTrustedBiddingSignalsSlotSizeMode(
      InterestGroup::TrustedBiddingSignalsSlotSizeMode
          trusted_bidding_signals_slot_size_mode);
  TestInterestGroupBuilder& SetMaxTrustedBiddingSignalsURLLength(
      int32_t max_trusted_bidding_signals_url_length);
  TestInterestGroupBuilder& SetTrustedBiddingSignalsCoordinator(
      std::optional<url::Origin> trusted_bidding_signals_coordinator);
  TestInterestGroupBuilder& SetViewAndClickCountsProviders(
      std::optional<std::vector<url::Origin>> view_and_click_counts_providers);
  TestInterestGroupBuilder& SetUserBiddingSignals(
      std::optional<std::string> user_bidding_signals);
  TestInterestGroupBuilder& SetAds(
      std::optional<std::vector<InterestGroup::Ad>> ads);
  TestInterestGroupBuilder& SetAdComponents(
      std::optional<std::vector<InterestGroup::Ad>> ad_components);
  TestInterestGroupBuilder& SetAdSizes(
      std::optional<base::flat_map<std::string, blink::AdSize>> ad_sizes);
  TestInterestGroupBuilder& SetSizeGroups(
      std::optional<base::flat_map<std::string, std::vector<std::string>>>
          size_groups);
  TestInterestGroupBuilder& SetAuctionServerRequestFlags(
      AuctionServerRequestFlags flags);
  TestInterestGroupBuilder& SetAdditionalBidKey(
      std::optional<blink::InterestGroup::AdditionalBidKey> key);
  TestInterestGroupBuilder& SetAggregationCoordinatorOrigin(
      std::optional<url::Origin> agg_coordinator_origin);

 private:
  InterestGroup interest_group_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_TEST_INTEREST_GROUP_BUILDER_H_
