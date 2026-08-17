// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SCHEDULER_WEB_SCHEDULER_TRACKED_FEATURE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SCHEDULER_WEB_SCHEDULER_TRACKED_FEATURE_H_

#include <stdint.h>

#include <optional>
#include <string>

#include "base/containers/enum_set.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {
namespace scheduler {

// A list of features which influence scheduling behaviour (throttling /
// freezing / back-forward cache) and which might be sent to the browser process
// for metrics-related purposes.
// When you add a feature, be sure to add it in the lists either to
// kDisallowedFeatures or kAllowedFeatures in BackForwardCacheImpl.
// When you remove a feature
// - add its index in removed_features in
//   BackForwardCacheMetricsTest.AllFeaturesCovered.
// - add it to the list in IsRemovedFeature if it appear in finch configs.
enum class WebSchedulerTrackedFeature : uint32_t {
  kMinValue = 0,
  kWebSocket = 0,
  kWebRTC = 1,

  // TODO(rakina): Move tracking of cache-control usage from
  // WebSchedulerTrackedFeature to RenderFrameHost.
  kMainResourceHasCacheControlNoCache = 2,
  kMainResourceHasCacheControlNoStore = 3,
  kSubresourceHasCacheControlNoCache = 4,
  kSubresourceHasCacheControlNoStore = 5,

  // These are unused.
  // kPageShowEventListener = 6,
  // kPageHideEventListener = 7,
  // kBeforeUnloadEventListener = 8,
  // kUnloadEventListener = 9,
  // kFreezeEventListener = 10,
  // kResumeEventListener = 11,

  kContainsPlugins = 12,
  kDocumentLoaded = 13,

  // Removed in https://crbug.com/1146955
  // kDedicatedWorkerOrWorklet = 14,

  // There are some other values defined for specific request context types
  // (e.g., XHR). This value corresponds to a network requests not covered by
  // specific context types down below.
  kOutstandingNetworkRequestOthers = 15,

  // kServiceWorkerControlledPage = 16. Removed after implementing ServiceWorker
  // support.

  // No longer blocking after https://crbug.com/1378823.
  // kOutstandingIndexedDBTransaction = 17,

  // Whether the page tried to request a permission regardless of the outcome.
  // TODO(altimin): Track this more accurately depending on the data.
  // See permission.mojom for more details.
  // kRequestedGeolocationPermission = 19,   // No longer blocking.

  // Requesting for the notifications permission should never have blocked
  // back/forward cache entry.
  // https://crbug.com/1350944
  // kRequestedNotificationsPermission = 20,

  kRequestedMIDIPermission = 21,
  kRequestedAudioCapturePermission = 22,
  kRequestedVideoCapturePermission = 23,
  kRequestedBackForwardCacheBlockedSensors = 24,

  // This features is added together with the `kWebTransport` but
  // as a sticky feature (i.e. the feature won't be removed even after the
  // connection is closed). Pages with this feature registered will only be
  // eligible for BFCache if the page doesn't contain "Cache-control: no-store"
  // header.
  kWebTransportSticky = 25,

  // This covers all background-related permissions, including background sync,
  // background fetch and others.
  kRequestedBackgroundWorkPermission = 26,

  kBroadcastChannel = 27,

  // No longer blocking after https://crbug.com/1331187.
  // kIndexedDBConnection = 28,

  // kWebGL = 29. Removed after implementing WebGL support.
  // kWebVR = 30. The entire feature has been deleted.
  kWebXR = 31,

  kSharedWorker = 32,

  kWebLocks = 33,
  kWebHID = 34,

  // kWakeLock = 35, Removed because clean-up is done upon visibility change.

  kWebShare = 36,

  kRequestedStorageAccessGrant = 37,
  kWebNfc = 38,
  // kWebFileSystem = 39. Removed after implementing WebFilesystem support in
  // back/forward cache.

  kOutstandingNetworkRequestFetch = 40,
  kOutstandingNetworkRequestXHR = 41,
  // kAppBanner = 42. Removed after support added for back/forward cache.
  // See https://crbug.com/1276864.
  kPrinting = 43,
  // kWebDatabase = 44, Removed after WebSQL removal.
  kPictureInPicture = 45,
  // kPortal = 46. Removed
  kSpeechRecognizer = 47,
  kIdleManager = 48,
  kPaymentManager = 49,
  // kSpeechSynthesis = 50. Removed
  kKeyboardLock = 51,
  kWebOTPService = 52,
  kOutstandingNetworkRequestDirectSocket = 53,
  kInjectedJavascript = 54,
  kInjectedStyleSheet = 55,
  // kMediaSessionImplOnServiceCreated = 56, Removed after implementing
  // MediaSessionImplOnServiceCreated support in back/forward cache.
  kWebTransport = 57,
  // This should be used only for testing.
  kDummy = 58,
  kKeepaliveRequest = 59,
  // An JavaScript network request has been received with the "Cache-Control:
  // no-store" header. We record this whenever we see it but we only care about
  // this if the frame it was made from is same-origin with the main frame and
  // the main frame used "Cache-Control: no-store".
  kJsNetworkRequestReceivedCacheControlNoStoreResource = 60,
  // There is a pending IndexedDB event (e.g. versionchange event sent but the
  // connection is not closed yet) that requires the page not to enter BFCache.
  kIndexedDBEvent = 61,
  // Aggressive throttling is disabled when a serial port is opened and
  // re-enabled when the opened port is closed.
  kWebSerial = 62,

  // These features are added together with the `kWebSocket` and `kWebRTC` but
  // as sticky features.
  // See comments for `kWebTransportSticky`.
  kWebSocketSticky = 63,
  kWebRTCSticky = 64,
  kSmartCard = 65,
  // There is a "live" MediaStreamTrack.
  kLiveMediaStreamTrack = 66,

  // Originally kUnloadHandlerExistsInMain/SubFrame were not recorded in the
  // renderer side, but recorded in the browser side, making it impossible to
  // track the source location. Here we make them a WebSchedulerTrackedFeature,
  // so that the source location can be tracked. See https://crbug.com/1513120
  // for details.
  kUnloadHandler = 67,
  kParserAborted = 68,

  // Aggressive throttling is disabled when the page has an active Bluetooth
  // connection.
  kWebBluetooth = 69,

  // The back/forward cache is disabled during WebAuthn transactions.
  kWebAuthentication = 70,

  // Please keep in sync with WebSchedulerTrackedFeature in
  // tools/metrics/histograms/enums.xml. These values should not be renumbered.

  kMaxValue = kWebAuthentication,
};

using WebSchedulerTrackedFeatures =
    base::EnumSet<WebSchedulerTrackedFeature,
                  WebSchedulerTrackedFeature::kMinValue,
                  WebSchedulerTrackedFeature::kMaxValue>;

BLINK_COMMON_EXPORT std::string FeatureToHumanReadableString(
    WebSchedulerTrackedFeature feature);
BLINK_COMMON_EXPORT std::string FeatureToShortString(
    WebSchedulerTrackedFeature feature);

BLINK_COMMON_EXPORT std::optional<WebSchedulerTrackedFeature> StringToFeature(
    const std::string& str);
// Returns true if there was previously a feature by this name.
// It is not comprehensive, just enough to cover what was used in finch,
// in order to stop warnings at startup. See https://crbug.com/1363846.
BLINK_COMMON_EXPORT bool IsRemovedFeature(const std::string& feature);

// Sticky features can't be unregistered and remain active for the rest of the
// lifetime of the page.
BLINK_COMMON_EXPORT bool IsFeatureSticky(WebSchedulerTrackedFeature feature);

// All the sticky features.
BLINK_COMMON_EXPORT WebSchedulerTrackedFeatures StickyFeatures();

// Disables wake up alignment permanently for the process. This is called when a
// feature that is incompatible with wake up alignment is used. Thread-safe.
BLINK_COMMON_EXPORT void DisableAlignWakeUpsForProcess();
BLINK_COMMON_EXPORT bool IsAlignWakeUpsDisabledForProcess();

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SCHEDULER_WEB_SCHEDULER_TRACKED_FEATURE_H_
