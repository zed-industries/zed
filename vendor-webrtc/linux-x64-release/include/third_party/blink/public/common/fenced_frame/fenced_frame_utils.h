// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FENCED_FRAME_FENCED_FRAME_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FENCED_FRAME_FENCED_FRAME_UTILS_H_

#include <optional>

#include "services/network/public/cpp/web_sandbox_flags.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-shared.h"
#include "third_party/blink/public/common/common_export.h"

class GURL;

namespace blink {

// To prevent overloading the RAM, limit the maximum automatic beacon length
// to 64KB.
const size_t kFencedFrameMaxBeaconLength = 64000;

// The maximum length of `blink::FencedFrameConfig::shared_storage_context_`.
// When setting, longer strings are truncated to this length.
const size_t kFencedFrameConfigSharedStorageContextMaxLength = 2048;

// Histogram names for fenced frame.
inline constexpr char kFencedFrameCreationOrNavigationOutcomeHistogram[] =
    "Blink.FencedFrame.CreationOrNavigationOutcome";

inline constexpr char kIsOpaqueFencedFrameSizeCoercedHistogram[] =
    "Blink.FencedFrame.IsOpaqueFrameSizeCoerced";

inline constexpr char kIsFencedFrameResizedAfterSizeFrozen[] =
    "Blink.FencedFrame.IsFrameResizedAfterSizeFrozen";

inline constexpr char kFencedFrameMandatoryUnsandboxedFlagsSandboxed[] =
    "Blink.FencedFrame.MandatoryUnsandboxedFlagsSandboxed";

inline constexpr char kFencedFrameFailedSandboxLoadInTopLevelFrame[] =
    "Blink.FencedFrame.FailedSandboxLoadInTopLevelFrame";

inline constexpr char kFencedFrameTopNavigationHistogram[] =
    "Navigation.FencedFrameTopNavigation";

inline constexpr char kAutomaticBeaconOutcomeHistogram[] =
    "Navigation.AutomaticBeaconOutcome";

inline constexpr char kAutomaticBeaconEventTypeHistogram[] =
    "Navigation.FencedFrameAutomaticBeaconEventType";

inline constexpr char kFencedFrameBeaconReportingHttpResultUMA[] =
    "Blink.FencedFrame.BeaconReportingHttpResult";

inline constexpr char kFencedFrameBeaconReportingCountUMA[] =
    "Navigation.FencedFrameBeaconReportingCountSameOrigin";

inline constexpr char kFencedFrameBeaconReportingCountCrossOriginUMA[] =
    "Navigation.FencedFrameBeaconReportingCountCrossOrigin";

inline constexpr char kSameSiteAdComponentsMaxCountForWinningBidHistogram[] =
    "Ads.InterestGroup.Auction.SameSiteAdComponentsMaxCountForWinningBid";

inline constexpr char kAdComponentsCountForWinningBidHistogram[] =
    "Ads.InterestGroup.Auction.AdComponentsCountForWinningBid";

inline constexpr char kDisableUntrustedNetworkOutcome[] =
    "Blink.FencedFrame.DisableUntrustedNetworkOutcome";

inline constexpr char kSharedStorageGetInFencedFrameOutcome[] =
    "Blink.FencedFrame.SharedStorageGetInFencedFrameOutcome";

inline constexpr char kNotifyEventOutcome[] =
    "Blink.FencedFrame.NotifyEventOutcome";

// Corresponds to the "FencedFrameCreationOutcome" histogram enumeration type in
// tools/metrics/histograms/metadata/blink/enums.xml.
//
// PLEASE DO NOT REORDER, REMOVE, OR CHANGE THE MEANING OF THESE VALUES.
enum class FencedFrameCreationOutcome {
  kSuccessDefault = 0,  // creates/navigates in default mode
  kSuccessOpaque = 1,   // creates/navigates in opaque ads mode
  kSandboxFlagsNotSet = 2,
  kIncompatibleMode = 3,
  kInsecureContext = 4,
  kIncompatibleURLDefault = 5,
  kIncompatibleURLOpaque = 6,
  kResponseHeaderNotOptIn = 7,  // HTTP response header Supports-Loading-Mode
                                // is not opted-in with 'fenced-frame'
  kMaxValue = kResponseHeaderNotOptIn
};

// Corresponds to the "FencedFrameDisableUntrustedNetworkOutcome" histogram
// enumeration type in tools/metrics/histograms/metadata/blink/enums.xml.
//
// PLEASE DO NOT REORDER, REMOVE, OR CHANGE THE MEANING OF THESE VALUES.
//
// LINT.IfChange(DisableUntrustedNetworkOutcome)
enum class DisableUntrustedNetworkOutcome {
  kResolved = 0,  // The fenced frame has its network fully revoked.
  kNotActive = 1,
  kNotAllowed = 2,
  kMaxValue = kNotAllowed
};
// LINT.ThenChange(//tools/metrics/histograms/metadata/blink/enums.xml:FencedFrameDisableUntrustedNetworkOutcome)

// Corresponds to the "SharedStorageGetInFencedFrameOutcome" histogram
// enumeration type in tools/metrics/histograms/metadata/blink/enums.xml.
//
// PLEASE DO NOT REORDER, REMOVE, OR CHANGE THE MEANING OF THESE VALUES.
//
// LINT.IfChange(SharedStorageGetInFencedFrameOutcome)
enum class SharedStorageGetInFencedFrameOutcome {
  kSuccess = 0,  // A fenced frame successfully read from shared storage.
  kInsecureContext = 1,
  kDisabled = 2,
  kWithoutRevokeNetwork = 3,
  kKeyNotFound = 4,
  kGetError = 5,
  kFeatureDisabled = 6,
  kPermissionDisabled = 7,
  kMaxValue = kPermissionDisabled
};
// LINT.ThenChange(//tools/metrics/histograms/metadata/blink/enums.xml:SharedStorageGetInFencedFrameOutcome)

// Corresponds to the "FencedFrameNotifyEventOutcome" histogram enumeration type
// in tools/metrics/histograms/metadata/blink/enums.xml.
//
// PLEASE DO NOT REORDER, REMOVE, OR CHANGE THE MEANING OF THESE VALUES.
//
// LINT.IfChange(NotifyEventOutcome)
enum class NotifyEventOutcome {
  kSuccess = 0,
  kNotActive = 1,
  kNotFencedFrameRoot = 2,
  kInvalidEvent = 3,
  kUnsupportedEventType = 4,
  kNoTransientUserActivation = 5,
  kMaxValue = kNoTransientUserActivation
};
// LINT.ThenChange(//tools/metrics/histograms/metadata/blink/enums.xml:FencedFrameNotifyEventOutcome)

// Corresponds to the "AutomaticBeaconOutcome" histogram enumeration type in
// tools/metrics/histograms/metadata/navigation/histograms.xml.
//
// PLEASE DO NOT REORDER, REMOVE, OR CHANGE THE MEANING OF THESE VALUES.
enum class AutomaticBeaconOutcome {
  kSuccess = 0,
  kNoUserActivation,
  kNotSameOriginNotOptedIn,
  kMaxValue = kNotSameOriginNotOptedIn,
};

// Corresponds to the "FencedFrameNavigationState" histogram enumeration type in
// tools/metrics/histograms/metadata/navigation/enums.xml.
//
// PLEASE DO NOT REORDER, REMOVE, OR CHANGE THE MEANING OF THESE VALUES.
enum class FencedFrameNavigationState {
  kBegin = 0,
  kCommit = 1,
  kMaxValue = kCommit
};

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class FencedFrameBeaconReportingResult {
  kUnknownResult = 0,
  kDestinationEnumInvalid = 1,
  kDestinationEnumSuccess = 2,
  kDestinationEnumFailure = 3,
  kDestinationUrlInvalid = 4,
  kDestinationUrlSuccess = 5,
  kDestinationUrlFailure = 6,
  kAutomaticInvalid = 7,
  kAutomaticSuccess = 8,
  kAutomaticFailure = 9,
  kMaxValue = kAutomaticFailure
};

// Whether or not a fenced frame is allowed to be navigated to `url`. For now
// this is described by
// https://github.com/WICG/fenced-frame/blob/master/explainer/modes.md.
BLINK_COMMON_EXPORT bool IsValidFencedFrameURL(const GURL& url);

// Whether or not a URL is a valid "urn uuid URL" depends not only on just the
// scheme being "urn", but that the URL's prefix is "urn:uuid".
BLINK_COMMON_EXPORT bool IsValidUrnUuidURL(const GURL& url);

// Record fenced frame related UMAs.
BLINK_COMMON_EXPORT void RecordFencedFrameCreationOutcome(
    const FencedFrameCreationOutcome outcome);
BLINK_COMMON_EXPORT void RecordOpaqueFencedFrameSizeCoercion(bool did_coerce);
BLINK_COMMON_EXPORT void RecordFencedFrameResizedAfterSizeFrozen();
BLINK_COMMON_EXPORT void RecordFencedFrameUnsandboxedFlags(
    network::mojom::WebSandboxFlags flags);
BLINK_COMMON_EXPORT void RecordFencedFrameFailedSandboxLoadInTopLevelFrame(
    bool is_main_frame);
BLINK_COMMON_EXPORT void RecordDisableUntrustedNetworkOutcome(
    const DisableUntrustedNetworkOutcome outcome);
BLINK_COMMON_EXPORT void RecordSharedStorageGetInFencedFrameOutcome(
    const SharedStorageGetInFencedFrameOutcome outcome);
BLINK_COMMON_EXPORT void RecordNotifyEventOutcome(
    const NotifyEventOutcome outcome);

// Returns true if the DOM event type name `event_type` is allowed to be
// propagated from a fenced frame to its embedder. Returns false otherwise.
BLINK_COMMON_EXPORT bool CanNotifyEventTypeAcrossFence(
    const std::string& event_type);

// Automatic beacon type definitions
inline constexpr char kDeprecatedFencedFrameTopNavigationBeaconType[] =
    "reserved.top_navigation";
inline constexpr char kFencedFrameTopNavigationStartBeaconType[] =
    "reserved.top_navigation_start";
inline constexpr char kFencedFrameTopNavigationCommitBeaconType[] =
    "reserved.top_navigation_commit";

inline constexpr const char* kFencedFrameAutomaticBeaconTypes[] = {
    kDeprecatedFencedFrameTopNavigationBeaconType,
    kFencedFrameTopNavigationStartBeaconType,
    kFencedFrameTopNavigationCommitBeaconType};

// Prefix of reserved event types for private aggregation API
inline constexpr char kFencedFrameReservedPAEventPrefix[] = "reserved.";

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FENCED_FRAME_FENCED_FRAME_UTILS_H_
