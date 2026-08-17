// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AUCTION_CONFIG_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AUCTION_CONFIG_MOJOM_TRAITS_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "base/containers/flat_map.h"
#include "base/time/time.h"
#include "base/uuid.h"
#include "third_party/abseil-cpp/absl/numeric/int128.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/interest_group/ad_auction_currencies.h"
#include "third_party/blink/public/common/interest_group/auction_config.h"
#include "third_party/blink/public/mojom/interest_group/ad_auction_service.mojom-forward.h"
#include "third_party/blink/public/mojom/interest_group/interest_group_types.mojom-shared.h"

class GURL;

namespace url {

class Origin;

}  // namespace url

namespace base {

class UnguessableToken;

}  // namespace base

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::DirectFromSellerSignalsSubresourceDataView,
                 blink::DirectFromSellerSignalsSubresource> {
  static const GURL& bundle_url(
      const blink::DirectFromSellerSignalsSubresource& params) {
    return params.bundle_url;
  }

  static const base::UnguessableToken& token(
      const blink::DirectFromSellerSignalsSubresource& params) {
    return params.token;
  }

  static bool Read(
      blink::mojom::DirectFromSellerSignalsSubresourceDataView data,
      blink::DirectFromSellerSignalsSubresource* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::DirectFromSellerSignalsDataView,
                 blink::DirectFromSellerSignals> {
  static const GURL& prefix(const blink::DirectFromSellerSignals& params) {
    return params.prefix;
  }

  static const base::flat_map<url::Origin,
                              blink::DirectFromSellerSignalsSubresource>&
  per_buyer_signals(const blink::DirectFromSellerSignals& params) {
    return params.per_buyer_signals;
  }

  static const std::optional<blink::DirectFromSellerSignalsSubresource>&
  seller_signals(const blink::DirectFromSellerSignals& params) {
    return params.seller_signals;
  }

  static const std::optional<blink::DirectFromSellerSignalsSubresource>&
  auction_signals(const blink::DirectFromSellerSignals& params) {
    return params.auction_signals;
  }

  static bool Read(blink::mojom::DirectFromSellerSignalsDataView data,
                   blink::DirectFromSellerSignals* out);
};

template <class View, class Wrapper>
struct BLINK_COMMON_EXPORT AdConfigMaybePromiseTraitsHelper {
  using ViewTag = typename View::Tag;
  using ValueType = typename Wrapper::ValueType;
  static ViewTag GetTag(const Wrapper& wrapper) {
    switch (wrapper.tag()) {
      case Wrapper::Tag::kPromise:
        return ViewTag::kPromise;
      case Wrapper::Tag::kValue:
        return ViewTag::kValue;
    }
    NOTREACHED();
  }

  static uint32_t promise(const Wrapper& wrapper) {
    return 0u;  // Ignored placeholder value.
  }

  static const ValueType& value(const Wrapper& wrapper) {
    return wrapper.value();
  }

  static bool Read(View in, Wrapper* out);
};

template <>
struct BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::AuctionAdConfigMaybePromiseJsonDataView,
                blink::AuctionConfig::MaybePromiseJson>
    : public AdConfigMaybePromiseTraitsHelper<
          blink::mojom::AuctionAdConfigMaybePromiseJsonDataView,
          blink::AuctionConfig::MaybePromiseJson> {};

template <>
struct BLINK_COMMON_EXPORT UnionTraits<
    blink::mojom::AuctionAdConfigMaybePromisePerBuyerSignalsDataView,
    blink::AuctionConfig::MaybePromisePerBuyerSignals>
    : public AdConfigMaybePromiseTraitsHelper<
          blink::mojom::AuctionAdConfigMaybePromisePerBuyerSignalsDataView,
          blink::AuctionConfig::MaybePromisePerBuyerSignals> {};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AuctionAdConfigBuyerTimeoutsDataView,
                 blink::AuctionConfig::BuyerTimeouts> {
  static const std::optional<base::flat_map<url::Origin, base::TimeDelta>>&
  per_buyer_timeouts(const blink::AuctionConfig::BuyerTimeouts& params) {
    return params.per_buyer_timeouts;
  }

  static const std::optional<base::TimeDelta>& all_buyers_timeout(
      const blink::AuctionConfig::BuyerTimeouts& params) {
    return params.all_buyers_timeout;
  }

  static bool Read(blink::mojom::AuctionAdConfigBuyerTimeoutsDataView data,
                   blink::AuctionConfig::BuyerTimeouts* out);
};

template <>
struct BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::AuctionAdConfigMaybePromiseBuyerTimeoutsDataView,
                blink::AuctionConfig::MaybePromiseBuyerTimeouts>
    : public AdConfigMaybePromiseTraitsHelper<
          blink::mojom::AuctionAdConfigMaybePromiseBuyerTimeoutsDataView,
          blink::AuctionConfig::MaybePromiseBuyerTimeouts> {};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AdCurrencyDataView, blink::AdCurrency> {
  static const std::string& currency_code(const blink::AdCurrency& params) {
    return params.currency_code();
  }

  static bool Read(blink::mojom::AdCurrencyDataView data,
                   blink::AdCurrency* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AuctionAdConfigBuyerCurrenciesDataView,
                 blink::AuctionConfig::BuyerCurrencies> {
  static const std::optional<base::flat_map<url::Origin, blink::AdCurrency>>&
  per_buyer_currencies(const blink::AuctionConfig::BuyerCurrencies& params) {
    return params.per_buyer_currencies;
  }

  static const std::optional<blink::AdCurrency>& all_buyers_currency(
      const blink::AuctionConfig::BuyerCurrencies& params) {
    return params.all_buyers_currency;
  }

  static bool Read(blink::mojom::AuctionAdConfigBuyerCurrenciesDataView data,
                   blink::AuctionConfig::BuyerCurrencies* out);
};

template <>
struct BLINK_COMMON_EXPORT UnionTraits<
    blink::mojom::AuctionAdConfigMaybePromiseBuyerCurrenciesDataView,
    blink::AuctionConfig::MaybePromiseBuyerCurrencies>
    : public AdConfigMaybePromiseTraitsHelper<
          blink::mojom::AuctionAdConfigMaybePromiseBuyerCurrenciesDataView,
          blink::AuctionConfig::MaybePromiseBuyerCurrencies> {};

template <>
struct BLINK_COMMON_EXPORT UnionTraits<
    blink::mojom::AuctionAdConfigMaybePromiseDirectFromSellerSignalsDataView,
    blink::AuctionConfig::MaybePromiseDirectFromSellerSignals>
    : public AdConfigMaybePromiseTraitsHelper<
          blink::mojom::
              AuctionAdConfigMaybePromiseDirectFromSellerSignalsDataView,
          blink::AuctionConfig::MaybePromiseDirectFromSellerSignals> {};

template <>
struct BLINK_COMMON_EXPORT UnionTraits<
    blink::mojom::
        AuctionAdConfigMaybePromiseDeprecatedRenderURLReplacementsDataView,
    blink::AuctionConfig::MaybePromiseDeprecatedRenderURLReplacements>
    : public AdConfigMaybePromiseTraitsHelper<
          blink::mojom::
              AuctionAdConfigMaybePromiseDeprecatedRenderURLReplacementsDataView,
          blink::AuctionConfig::MaybePromiseDeprecatedRenderURLReplacements> {};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AdKeywordReplacementDataView,
                 blink::AuctionConfig::AdKeywordReplacement> {
  static std::string match(
      const blink::AuctionConfig::AdKeywordReplacement& params) {
    return params.match;
  }

  static std::string replacement(
      const blink::AuctionConfig::AdKeywordReplacement& params) {
    return params.replacement;
  }

  static bool Read(blink::mojom::AdKeywordReplacementDataView data,
                   blink::AuctionConfig::AdKeywordReplacement* out);
};
template <>
struct BLINK_COMMON_EXPORT StructTraits<
    blink::mojom::AuctionReportBuyersConfigDataView,
    blink::AuctionConfig::NonSharedParams::AuctionReportBuyersConfig> {
  static absl::uint128 bucket(
      const blink::AuctionConfig::NonSharedParams::AuctionReportBuyersConfig&
          params) {
    return params.bucket;
  }

  static double scale(
      const blink::AuctionConfig::NonSharedParams::AuctionReportBuyersConfig&
          params) {
    return params.scale;
  }

  static bool Read(
      blink::mojom::AuctionReportBuyersConfigDataView data,
      blink::AuctionConfig::NonSharedParams::AuctionReportBuyersConfig* out);
};

template <>
struct BLINK_COMMON_EXPORT StructTraits<
    blink::mojom::AuctionReportBuyerDebugModeConfigDataView,
    blink::AuctionConfig::NonSharedParams::AuctionReportBuyerDebugModeConfig> {
  static bool is_enabled(const blink::AuctionConfig::NonSharedParams::
                             AuctionReportBuyerDebugModeConfig& params) {
    return params.is_enabled;
  }

  static std::optional<uint64_t> debug_key(
      const blink::AuctionConfig::NonSharedParams::
          AuctionReportBuyerDebugModeConfig& params) {
    return params.debug_key;
  }

  static bool Read(
      blink::mojom::AuctionReportBuyerDebugModeConfigDataView data,
      blink::AuctionConfig::NonSharedParams::AuctionReportBuyerDebugModeConfig*
          out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AuctionAdServerResponseConfigDataView,
                 blink::AuctionConfig::ServerResponseConfig> {
  static const base::Uuid& request_id(
      const blink::AuctionConfig::ServerResponseConfig& params) {
    return params.request_id;
  }

  static bool Read(blink::mojom::AuctionAdServerResponseConfigDataView data,
                   blink::AuctionConfig::ServerResponseConfig* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AuctionAdConfigNonSharedParamsDataView,
                 blink::AuctionConfig::NonSharedParams> {
  static const std::optional<std::vector<url::Origin>>& interest_group_buyers(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.interest_group_buyers;
  }

  static const blink::AuctionConfig::MaybePromiseJson& auction_signals(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.auction_signals;
  }

  static const blink::AuctionConfig::MaybePromiseJson& seller_signals(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.seller_signals;
  }

  static const blink::AuctionConfig::MaybePromiseJson& seller_tkv_signals(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.seller_tkv_signals;
  }

  static std::optional<base::TimeDelta> seller_timeout(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.seller_timeout;
  }

  static const blink::AuctionConfig::MaybePromisePerBuyerSignals&
  per_buyer_signals(const blink::AuctionConfig::NonSharedParams& params) {
    return params.per_buyer_signals;
  }

  static const base::flat_map<url::Origin, std::string>& per_buyer_tkv_signals(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.per_buyer_tkv_signals;
  }

  static const blink::AuctionConfig::MaybePromiseBuyerTimeouts& buyer_timeouts(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.buyer_timeouts;
  }

  static std::optional<base::TimeDelta> reporting_timeout(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.reporting_timeout;
  }

  static const std::optional<blink::AdCurrency>& seller_currency(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.seller_currency;
  }

  static const blink::AuctionConfig::MaybePromiseBuyerCurrencies&
  buyer_currencies(const blink::AuctionConfig::NonSharedParams& params) {
    return params.buyer_currencies;
  }

  static const blink::AuctionConfig::MaybePromiseBuyerTimeouts&
  buyer_cumulative_timeouts(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.buyer_cumulative_timeouts;
  }

  static const base::flat_map<url::Origin, std::uint16_t>&
  per_buyer_group_limits(const blink::AuctionConfig::NonSharedParams& params) {
    return params.per_buyer_group_limits;
  }

  static std::uint16_t all_buyers_group_limit(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.all_buyers_group_limit;
  }

  static const std::optional<
      base::flat_map<url::Origin, base::flat_map<std::string, double>>>&
  per_buyer_priority_signals(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.per_buyer_priority_signals;
  }

  static const std::optional<base::flat_map<std::string, double>>&
  all_buyers_priority_signals(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.all_buyers_priority_signals;
  }

  static const std::optional<std::vector<absl::uint128>>&
  auction_report_buyer_keys(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.auction_report_buyer_keys;
  }

  static const std::optional<base::flat_map<
      blink::AuctionConfig::NonSharedParams::BuyerReportType,
      blink::AuctionConfig::NonSharedParams::AuctionReportBuyersConfig>>&
  auction_report_buyers(const blink::AuctionConfig::NonSharedParams& params) {
    return params.auction_report_buyers;
  }

  static const std::optional<
      blink::AuctionConfig::NonSharedParams::AuctionReportBuyerDebugModeConfig>&
  auction_report_buyer_debug_mode_config(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.auction_report_buyer_debug_mode_config;
  }

  static const blink::SellerCapabilitiesType required_seller_capabilities(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.required_seller_capabilities;
  }

  static const std::optional<blink::AdSize> requested_size(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.requested_size;
  }

  static const std::optional<std::vector<blink::AdSize>>
  all_slots_requested_sizes(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.all_slots_requested_sizes;
  }

  static const blink::AuctionConfig::
      MaybePromiseDeprecatedRenderURLReplacements&
      deprecated_render_url_replacements(
          const blink::AuctionConfig::NonSharedParams& params) {
    return params.deprecated_render_url_replacements;
  }

  static const base::flat_map<url::Origin, uint16_t>&
  per_buyer_multi_bid_limits(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.per_buyer_multi_bid_limits;
  }

  static uint16_t all_buyers_multi_bid_limit(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.all_buyers_multi_bid_limit;
  }

  static const std::optional<base::Uuid>& auction_nonce(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.auction_nonce;
  }

  static const std::optional<
      blink::AuctionConfig::NonSharedParams::RealTimeReportingType>&
  seller_real_time_reporting_type(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.seller_real_time_reporting_type;
  }

  static const std::optional<base::flat_map<
      url::Origin,
      blink::AuctionConfig::NonSharedParams::RealTimeReportingType>>&
  per_buyer_real_time_reporting_types(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.per_buyer_real_time_reporting_types;
  }

  static const std::vector<blink::AuctionConfig>& component_auctions(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.component_auctions;
  }

  static int32_t max_trusted_scoring_signals_url_length(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.max_trusted_scoring_signals_url_length;
  }

  static const std::optional<url::Origin>& trusted_scoring_signals_coordinator(
      const blink::AuctionConfig::NonSharedParams& params) {
    return params.trusted_scoring_signals_coordinator;
  }

  static bool Read(blink::mojom::AuctionAdConfigNonSharedParamsDataView data,
                   blink::AuctionConfig::NonSharedParams* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::AuctionAdConfigDataView, blink::AuctionConfig> {
  static const url::Origin& seller(const blink::AuctionConfig& config) {
    return config.seller;
  }

  static const std::optional<blink::AuctionConfig::ServerResponseConfig>&
  server_response(const blink::AuctionConfig& config) {
    return config.server_response;
  }

  static const std::optional<GURL>& decision_logic_url(
      const blink::AuctionConfig& config) {
    return config.decision_logic_url;
  }

  static const std::optional<GURL>& trusted_scoring_signals_url(
      const blink::AuctionConfig& config) {
    return config.trusted_scoring_signals_url;
  }

  static const blink::AuctionConfig::NonSharedParams&
  auction_ad_config_non_shared_params(const blink::AuctionConfig& config) {
    return config.non_shared_params;
  }

  static const blink::AuctionConfig::MaybePromiseDirectFromSellerSignals&
  direct_from_seller_signals(const blink::AuctionConfig& params) {
    return params.direct_from_seller_signals;
  }

  static bool expects_direct_from_seller_signals_header_ad_slot(
      const blink::AuctionConfig& params) {
    return params.expects_direct_from_seller_signals_header_ad_slot;
  }

  static std::optional<std::uint16_t> seller_experiment_group_id(
      const blink::AuctionConfig& config) {
    return config.seller_experiment_group_id;
  }

  static std::optional<std::uint16_t> all_buyer_experiment_group_id(
      const blink::AuctionConfig& config) {
    return config.all_buyer_experiment_group_id;
  }

  static const base::flat_map<url::Origin, uint16_t>&
  per_buyer_experiment_group_ids(const blink::AuctionConfig& config) {
    return config.per_buyer_experiment_group_ids;
  }


  static bool expects_additional_bids(const blink::AuctionConfig& config) {
    return config.expects_additional_bids;
  }

  static std::optional<url::Origin> aggregation_coordinator_origin(
      const blink::AuctionConfig& config) {
    return config.aggregation_coordinator_origin;
  }

  static std::optional<bool> send_creative_scanning_metadata(
      const blink::AuctionConfig& config) {
    return config.send_creative_scanning_metadata;
  }

  static bool Read(blink::mojom::AuctionAdConfigDataView data,
                   blink::AuctionConfig* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AUCTION_CONFIG_MOJOM_TRAITS_H_
