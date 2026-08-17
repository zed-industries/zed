// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_INTEREST_GROUP_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_INTEREST_GROUP_MOJOM_TRAITS_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/interest_group/ad_display_size.h"
#include "third_party/blink/public/common/interest_group/interest_group.h"
#include "third_party/blink/public/mojom/interest_group/interest_group_types.mojom.h"
#include "url/gurl.h"
#include "url/origin.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::InterestGroupAdDataView,
                                        blink::InterestGroup::Ad> {
  static const std::string& render_url(const blink::InterestGroup::Ad& ad) {
    return ad.render_url_;
  }

  static const std::optional<std::string>& size_group(
      const blink::InterestGroup::Ad& ad) {
    return ad.size_group;
  }

  static const std::optional<std::string>& buyer_reporting_id(
      const blink::InterestGroup::Ad& ad) {
    return ad.buyer_reporting_id;
  }

  static const std::optional<std::string>& buyer_and_seller_reporting_id(
      const blink::InterestGroup::Ad& ad) {
    return ad.buyer_and_seller_reporting_id;
  }

  static const std::optional<std::vector<std::string>>&
  selectable_buyer_and_seller_reporting_ids(
      const blink::InterestGroup::Ad& ad) {
    return ad.selectable_buyer_and_seller_reporting_ids;
  }

  static const std::optional<std::string>& metadata(
      const blink::InterestGroup::Ad& ad) {
    return ad.metadata;
  }

  static const std::optional<std::string>& ad_render_id(
      const blink::InterestGroup::Ad& ad) {
    return ad.ad_render_id;
  }

  static const std::optional<std::vector<url::Origin>>&
  allowed_reporting_origins(const blink::InterestGroup::Ad& ad) {
    return ad.allowed_reporting_origins;
  }

  static const std::optional<std::string>& creative_scanning_metadata(
      const blink::InterestGroup::Ad& ad) {
    return ad.creative_scanning_metadata;
  }

  static bool Read(blink::mojom::InterestGroupAdDataView data,
                   blink::InterestGroup::Ad* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::SellerCapabilitiesDataView,
                 blink::SellerCapabilitiesType> {
  static bool allows_interest_group_counts(
      const blink::SellerCapabilitiesType& capabilities) {
    return capabilities.Has(blink::SellerCapabilities::kInterestGroupCounts);
  }

  static bool allows_latency_stats(
      const blink::SellerCapabilitiesType& capabilities) {
    return capabilities.Has(blink::SellerCapabilities::kLatencyStats);
  }

  static bool Read(blink::mojom::SellerCapabilitiesDataView data,
                   blink::SellerCapabilitiesType* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AuctionServerRequestFlagsDataView,
                 blink::AuctionServerRequestFlags> {
  static bool omit_ads(const blink::AuctionServerRequestFlags& capabilities) {
    return capabilities.Has(blink::AuctionServerRequestFlagsEnum::kOmitAds);
  }

  static bool include_full_ads(
      const blink::AuctionServerRequestFlags& capabilities) {
    return capabilities.Has(
        blink::AuctionServerRequestFlagsEnum::kIncludeFullAds);
  }

  static bool omit_user_bidding_signals(
      const blink::AuctionServerRequestFlags& capabilities) {
    return capabilities.Has(
        blink::AuctionServerRequestFlagsEnum::kOmitUserBiddingSignals);
  }

  static bool Read(blink::mojom::AuctionServerRequestFlagsDataView data,
                   blink::AuctionServerRequestFlags* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::InterestGroupDataView, blink::InterestGroup> {
  static base::Time expiry(const blink::InterestGroup& interest_group) {
    return interest_group.expiry;
  }

  static const url::Origin& owner(const blink::InterestGroup& interest_group) {
    return interest_group.owner;
  }

  static const std::string& name(const blink::InterestGroup& interest_group) {
    return interest_group.name;
  }

  static double priority(const blink::InterestGroup& interest_group) {
    return interest_group.priority;
  }

  static bool enable_bidding_signals_prioritization(
      const blink::InterestGroup& interest_group) {
    return interest_group.enable_bidding_signals_prioritization;
  }

  static const std::optional<base::flat_map<std::string, double>>&
  priority_vector(const blink::InterestGroup& interest_group) {
    return interest_group.priority_vector;
  }

  static const std::optional<base::flat_map<std::string, double>>&
  priority_signals_overrides(const blink::InterestGroup& interest_group) {
    return interest_group.priority_signals_overrides;
  }

  static const std::optional<
      base::flat_map<url::Origin, blink::SellerCapabilitiesType>>&
  seller_capabilities(const blink::InterestGroup& interest_group) {
    return interest_group.seller_capabilities;
  }

  static blink::SellerCapabilitiesType all_sellers_capabilities(
      const blink::InterestGroup& interest_group) {
    return interest_group.all_sellers_capabilities;
  }

  static blink::InterestGroup::ExecutionMode execution_mode(
      const blink::InterestGroup& interest_group) {
    return interest_group.execution_mode;
  }

  static const std::optional<GURL>& bidding_url(
      const blink::InterestGroup& interest_group) {
    return interest_group.bidding_url;
  }

  static const std::optional<GURL>& bidding_wasm_helper_url(
      const blink::InterestGroup& interest_group) {
    return interest_group.bidding_wasm_helper_url;
  }

  static const std::optional<GURL>& update_url(
      const blink::InterestGroup& interest_group) {
    return interest_group.update_url;
  }

  static const std::optional<GURL>& trusted_bidding_signals_url(
      const blink::InterestGroup& interest_group) {
    return interest_group.trusted_bidding_signals_url;
  }

  static const std::optional<std::vector<std::string>>&
  trusted_bidding_signals_keys(const blink::InterestGroup& interest_group) {
    return interest_group.trusted_bidding_signals_keys;
  }

  static blink::InterestGroup::TrustedBiddingSignalsSlotSizeMode
  trusted_bidding_signals_slot_size_mode(
      const blink::InterestGroup& interest_group) {
    return interest_group.trusted_bidding_signals_slot_size_mode;
  }

  static int32_t max_trusted_bidding_signals_url_length(
      const blink::InterestGroup& interest_group) {
    return interest_group.max_trusted_bidding_signals_url_length;
  }

  static const std::optional<url::Origin>& trusted_bidding_signals_coordinator(
      const blink::InterestGroup& interest_group) {
    return interest_group.trusted_bidding_signals_coordinator;
  }

  static const std::optional<std::vector<url::Origin>>&
  view_and_click_counts_providers(const blink::InterestGroup& interest_group) {
    return interest_group.view_and_click_counts_providers;
  }

  static const std::optional<std::string>& user_bidding_signals(
      const blink::InterestGroup& interest_group) {
    return interest_group.user_bidding_signals;
  }

  static const std::optional<std::vector<blink::InterestGroup::Ad>>& ads(
      const blink::InterestGroup& interest_group) {
    return interest_group.ads;
  }

  static const std::optional<std::vector<blink::InterestGroup::Ad>>&
  ad_components(const blink::InterestGroup& interest_group) {
    return interest_group.ad_components;
  }

  static const std::optional<base::flat_map<std::string, blink::AdSize>>&
  ad_sizes(const blink::InterestGroup& interest_group) {
    return interest_group.ad_sizes;
  }

  static const std::optional<
      base::flat_map<std::string, std::vector<std::string>>>&
  size_groups(const blink::InterestGroup& interest_group) {
    return interest_group.size_groups;
  }

  static blink::AuctionServerRequestFlags auction_server_request_flags(
      const blink::InterestGroup& interest_group) {
    return interest_group.auction_server_request_flags;
  }

  static const std::optional<blink::InterestGroup::AdditionalBidKey>&
  additional_bid_key(const blink::InterestGroup& interest_group) {
    return interest_group.additional_bid_key;
  }

  static const std::optional<url::Origin>& aggregation_coordinator_origin(
      const blink::InterestGroup& interest_group) {
    return interest_group.aggregation_coordinator_origin;
  }

  static bool Read(blink::mojom::InterestGroupDataView data,
                   blink::InterestGroup* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_INTEREST_GROUP_MOJOM_TRAITS_H_
