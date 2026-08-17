// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_INTEREST_GROUP_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_INTEREST_GROUP_H_

#include <stdint.h>

#include <optional>
#include <set>
#include <string>
#include <vector>

#include "base/containers/flat_map.h"
#include "base/time/time.h"
#include "base/types/optional_ref.h"
#include "base/types/pass_key.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/interest_group/ad_display_size.h"
#include "third_party/blink/public/common/interest_group/auction_server_request_flags.h"
#include "third_party/blink/public/common/interest_group/seller_capabilities.h"
#include "third_party/blink/public/mojom/interest_group/interest_group_types.mojom-shared.h"
#include "third_party/boringssl/src/include/openssl/curve25519.h"
#include "url/gurl.h"
#include "url/origin.h"

namespace content {
class InterestGroupStorage;
}
namespace blink {

constexpr char kKAnonKeyForAdComponentBidPrefix[] = "ComponentBid\n";
constexpr char kKAnonKeyForAdBidPrefix[] = "AdBid\n";

// Interest group used by FLEDGE auctions. Typemapped to
// blink::mojom::InterestGroup, primarily so the typemap can include validity
// checks on the origins of the provided URLs.
//
// All URLs and origins must use https, and same origin to `owner`.
//
// https://github.com/WICG/turtledove/blob/main/FLEDGE.md#11-joining-interest-groups
struct BLINK_COMMON_EXPORT InterestGroup {
  using ExecutionMode = blink::mojom::InterestGroup_ExecutionMode;
  using TrustedBiddingSignalsSlotSizeMode =
      blink::mojom::InterestGroup_TrustedBiddingSignalsSlotSizeMode;
  using AdditionalBidKey = std::array<uint8_t, ED25519_PUBLIC_KEY_LEN>;
  // An advertisement to display for an interest group. Typemapped to
  // blink::mojom::InterestGroupAd.
  // https://github.com/WICG/turtledove/blob/main/FLEDGE.md#12-interest-group-attributes
  class BLINK_COMMON_EXPORT Ad {
   public:
    Ad();
    // Must use https. This string must have been the result of GURL().spec().
    // DO NOT set this to a value that has never passed through GURL.
    explicit Ad(base::PassKey<content::InterestGroupStorage>,
                std::string&& render_url);
    explicit Ad(base::PassKey<content::InterestGroupStorage>,
                const std::string& render_url);
    Ad(const GURL& render_url,
       std::optional<std::string> metadata,
       std::optional<std::string> size_group = std::nullopt,
       std::optional<std::string> buyer_reporting_id = std::nullopt,
       std::optional<std::string> buyer_and_seller_reporting_id = std::nullopt,
       std::optional<std::vector<std::string>>
           selectable_buyer_and_seller_reporting_ids = std::nullopt,
       std::optional<std::string> ad_render_id = std::nullopt,
       std::optional<std::vector<url::Origin>> allowed_reporting_origins =
           std::nullopt,
       std::optional<std::string> creative_scanning_metadata = std::nullopt);
    ~Ad();

    // Returns the approximate size of the contents of this InterestGroup::Ad,
    // in bytes.
    size_t EstimateSize() const;

    const std::string& render_url() const { return render_url_; }

    // Optional size group assigned to this Ad.
    std::optional<std::string> size_group;
    // Opaque JSON data, passed as an object to auction worklet.
    std::optional<std::string> metadata;

    // Optional alternative identifiers for reporting purposes that can be
    // passed to reporting scripts in lieu of group name if they pass k-anon
    // checks. These are only set on ads, not on component ads.
    std::optional<std::string> buyer_reporting_id;
    std::optional<std::string> buyer_and_seller_reporting_id;
    std::optional<std::vector<std::string>>
        selectable_buyer_and_seller_reporting_ids;

    // Optional alias to use for B&A auctions
    std::optional<std::string> ad_render_id;

    // Optional origins that can receive macro expanded reports.
    std::optional<std::vector<url::Origin>> allowed_reporting_origins;

    // Optional metadata to provide v1 trusted seller signals server.
    std::optional<std::string> creative_scanning_metadata;

    // TODO(crbug.com/355010821): Remove once all callers have been migrated.
    bool operator==(const Ad& other) const;

   private:
    std::string render_url_;
    friend struct mojo::StructTraits<blink::mojom::InterestGroupAdDataView,
                                     blink::InterestGroup::Ad>;
  };

  InterestGroup();
  ~InterestGroup();

  InterestGroup(InterestGroup&& other);
  InterestGroup& operator=(InterestGroup&& other);
  InterestGroup(const InterestGroup& other);
  InterestGroup& operator=(const InterestGroup&);

  // Checks for validity. Performs same checks as ValidateBlinkInterestGroup().
  // Automatically checked when passing InterestGroups over Mojo.
  bool IsValid() const;

  // Additional checks for validity performed only at join and update time.
  // Performs same checks as PerformAdditionalJoinAndUpdateTimeValidations().
  bool IsValidForJoinAndUpdate() const;

  // Returns the approximate size of the contents of this InterestGroup, in
  // bytes.
  size_t EstimateSize() const;

  // Returns true if this is a negative interest group, only usable for negative
  // targeting.
  bool IsNegativeInterestGroup() const {
    return additional_bid_key.has_value();
  }

  // Returns all of the k-anonymity keys used by this interest group.
  std::vector<std::string> GetAllKAnonKeys() const;

  // Parses string representation of a TrustedBiddingSignalsSlotSizeMode. A
  // template so it works on wtf::Strings and std::strings. Returns kNone when
  // passed an unrecognized mode, for forward compatibility.
  template <class StringType>
  static TrustedBiddingSignalsSlotSizeMode
  ParseTrustedBiddingSignalsSlotSizeMode(const StringType& mode) {
    if (mode == "slot-size") {
      return TrustedBiddingSignalsSlotSizeMode::kSlotSize;
    }
    if (mode == "all-slots-requested-sizes") {
      return TrustedBiddingSignalsSlotSizeMode::kAllSlotsRequestedSizes;
    }
    // All unrecognized strings (as well as "none") are mapped to kNone.
    return TrustedBiddingSignalsSlotSizeMode::kNone;
  }

  // Takes a TrustedBiddingSignalsSlotSizeMode and returns the corresponding
  // string.
  static std::string_view TrustedBiddingSignalsSlotSizeModeToString(
      TrustedBiddingSignalsSlotSizeMode slot_size_mode);

  base::Time expiry;
  url::Origin owner;
  std::string name;

  double priority = 0;
  bool enable_bidding_signals_prioritization = false;
  std::optional<base::flat_map<std::string, double>> priority_vector;
  std::optional<base::flat_map<std::string, double>> priority_signals_overrides;

  std::optional<base::flat_map<url::Origin, SellerCapabilitiesType>>
      seller_capabilities;
  SellerCapabilitiesType all_sellers_capabilities;
  ExecutionMode execution_mode = ExecutionMode::kCompatibilityMode;
  std::optional<GURL> bidding_url;
  std::optional<GURL> bidding_wasm_helper_url;
  std::optional<GURL> update_url;
  std::optional<GURL> trusted_bidding_signals_url;
  std::optional<std::vector<std::string>> trusted_bidding_signals_keys;
  TrustedBiddingSignalsSlotSizeMode trusted_bidding_signals_slot_size_mode =
      TrustedBiddingSignalsSlotSizeMode::kNone;
  int32_t max_trusted_bidding_signals_url_length = 0;
  std::optional<url::Origin> trusted_bidding_signals_coordinator;
  std::optional<std::vector<url::Origin>> view_and_click_counts_providers;
  std::optional<std::string> user_bidding_signals;
  std::optional<std::vector<InterestGroup::Ad>> ads, ad_components;
  std::optional<base::flat_map<std::string, blink::AdSize>> ad_sizes;
  std::optional<base::flat_map<std::string, std::vector<std::string>>>
      size_groups;

  AuctionServerRequestFlags auction_server_request_flags;

  std::optional<AdditionalBidKey> additional_bid_key;
  std::optional<url::Origin> aggregation_coordinator_origin;

  static_assert(__LINE__ == 194, R"(
If modifying InterestGroup fields, make sure to also modify:

* IsValid(), EstimateSize(), and in this class
* IgExpect[Not]EqualsForTesting() in interest_group_test_utils.cc
* SerializeInterestGroupForDevtools()
    (in devtools_serialization.cc; test in devtools_serialization_unittest.cc)
* auction_ad_interest_group.idl
* navigator_auction.cc (test logic in interest_group_browsertest.cc)
* interest_group_types.mojom
* validate_blink_interest_group.cc (includes blink version of EstimateSize)
* validate_blink_interest_group_test.cc
    (at least CreateFullyPopulatedInterestGroup)
* test_interest_group_builder[.h/.cc]
* interest_group_mojom_traits[.h/.cc/_test.cc]
* bidder_lazy_filler.cc (to pass the InterestGroup to generateBid())
* shared_storage_worklet_global_scope.cc (interestGroups())
* shared_storage_worklet_unittest.cc (SharedStorageWorkletTest.InterestGroups)

In interest_group_storage.cc, add the new field and any respective indices, add
a new database version and migration. Run InterestGroupStorageTest and follow
the test failure message instructions to update the
InterestGroupStorageTest.MultiVersionUpgradeTest database upgrade test.

If the new field is to be updatable via updateURL, also update *all* of
these:

* Add field to content::InterestGroupUpdate
* DoUpdateInterestGroup in interest_group_storage.cc
* ParseUpdateJson in interest_group_update_manager.cc
* Update AdAuctionServiceImplTest.UpdateAllUpdatableFields

If the new field is a required Mojo field, set a value for it in all the
textprotos in the ad_auction_service_mojolpm_fuzzer/ directory.

See crrev.com/c/3517534 for an example (adding the priority field), and also
remember to update bidder_worklet.cc too.

If the new field should be sent to the B&A server for server-side auctions then
SerializeInterestGroup() in bidding_and_auction_serializer.cc needs modified to
support the new field.
)");
};

// A unique identifier for interest groups.
struct InterestGroupKey {
  InterestGroupKey() = default;
  InterestGroupKey(url::Origin o, std::string n)
      : owner(std::move(o)), name(std::move(n)) {}
  inline bool operator<(const InterestGroupKey& other) const {
    return owner != other.owner ? owner < other.owner : name < other.name;
  }
  inline bool operator==(const InterestGroupKey& other) const {
    return owner == other.owner && name == other.name;
  }
  url::Origin owner;
  std::string name;
};

// A set of interest groups, identified by owner and name. Used to log which
// interest groups bid in an auction. A sets is used to avoid double-counting
// interest groups that bid in multiple components auctions in a component
// auction.
using InterestGroupSet = std::set<InterestGroupKey>;

// Calculates the k-anonymity key for an Ad that is used for determining if an
// ad is k-anonymous for the purposes of bidding and winning an auction.
// We want to avoid providing too much identifying information for event level
// reporting in reportWin. This key is used to check that providing the interest
// group owner and ad URL to the bidding script doesn't identify the user. It is
// used to gate whether an ad can participate in a FLEDGE auction because event
// level reports need to include both the owner and ad URL for the purposes of
// an auction.
// DEPRECATED_KAnonKeyForAdBid should only be used for upgrades of
// the InterestGroups database. Use HashedKAnonKeyForAdBid instead.
std::string BLINK_COMMON_EXPORT
DEPRECATED_KAnonKeyForAdBid(const InterestGroup& group,
                            const std::string& ad_url_from_gurl_spec);
std::string BLINK_COMMON_EXPORT
HashedKAnonKeyForAdBid(const url::Origin& owner,
                       const GURL& bidding_url,
                       const std::string& ad_url_from_gurl_spec);
std::string BLINK_COMMON_EXPORT
HashedKAnonKeyForAdBid(const InterestGroup& group,
                       const std::string& ad_url_from_gurl_spec);
std::string BLINK_COMMON_EXPORT
HashedKAnonKeyForAdBid(const InterestGroup& group,
                       const blink::AdDescriptor& ad_descriptor);
// Calculates the k-anonymity key for an ad component that is used for
// determining if an ad component is k-anonymous for the purposes of bidding and
// winning an auction. Since ad components are not provided to reporting, we
// only are concerned with micro-targetting. This means we can just use the ad
// url as the k-anonymity key.
// DEPRECATED_KAnonKeyForAdComponentBid should only be used for upgrades of
// the InterestGroups database. Use HashedKAnonKeyForAdComponentBid instead.
std::string BLINK_COMMON_EXPORT
DEPRECATED_KAnonKeyForAdComponentBid(const std::string& ad_url_from_gurl_spec);
std::string BLINK_COMMON_EXPORT
HashedKAnonKeyForAdComponentBid(const std::string& ad_url_from_gurl_spec);
std::string BLINK_COMMON_EXPORT
HashedKAnonKeyForAdComponentBid(const blink::AdDescriptor& ad_descriptor);
std::string BLINK_COMMON_EXPORT
HashedKAnonKeyForAdComponentBid(const GURL& ad_url);

// Calculates the k-anonymity key for reporting the interest group name in
// reportWin along with the given Ad.
// We want to avoid providing too much identifying information for event level
// reporting in reportWin. This key is used to check if including the passed in
// identifier --- either a specific explicit campaign ID or, as fallback
// the interest group name  --- along with the interest group owner and ad URL
// would make the user too identifiable. If this key is not k-anonymous then we
// do not provide the interest group name to reportWin.
// DEPRECATED_KAnonKeyForAdNameReporting should only be used for upgrades of
// the InterestGroups database. Use HashedKAnonKeyForAdNameReporting instead.
std::string BLINK_COMMON_EXPORT DEPRECATED_KAnonKeyForAdNameReporting(
    const InterestGroup& group,
    const InterestGroup::Ad& ad,
    base::optional_ref<const std::string>
        selected_buyer_and_seller_reporting_id);

std::string BLINK_COMMON_EXPORT
HashedKAnonKeyForAdNameReporting(const InterestGroup& group,
                                 const InterestGroup::Ad& ad,
                                 base::optional_ref<const std::string>
                                     selected_buyer_and_seller_reporting_id);

std::string BLINK_COMMON_EXPORT
HashedKAnonKeyForAdNameReportingWithoutInterestGroup(
    const url::Origin& interest_group_owner,
    const std::string& interest_group_name,
    const GURL& interest_group_bidding_url,
    const std::string& ad_render_url,
    base::optional_ref<const std::string> buyer_reporting_id,
    base::optional_ref<const std::string> buyer_and_seller_reporting_id,
    base::optional_ref<const std::string>
        selected_buyer_and_seller_reporting_id);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_INTEREST_GROUP_H_
