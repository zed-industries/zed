/*
 *  Copyright 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains enums related to Chrome UMA histograms. See
// https://chromium.googlesource.com/chromium/src.git/+/HEAD/tools/metrics/histograms/README.md#requirements
// for requirements when adding or changing metrics.

#ifndef API_UMA_METRICS_H_
#define API_UMA_METRICS_H_

namespace webrtc {

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum PeerConnectionAddressFamilyCounter {
  kPeerConnection_IPv4 = 0,
  kPeerConnection_IPv6 = 1,
  kBestConnections_IPv4 = 2,
  kBestConnections_IPv6 = 3,
  kPeerConnectionAddressFamilyCounter_Max
};

// This enum defines types for UMA samples, which will have a range.
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum PeerConnectionMetricsName {
  kNetworkInterfaces_IPv4 = 0,  // Number of IPv4 interfaces.
  kNetworkInterfaces_IPv6 = 1,  // Number of IPv6 interfaces.
  kTimeToConnect = 2,           // In milliseconds.
  kLocalCandidates_IPv4 = 3,    // Number of IPv4 local candidates.
  kLocalCandidates_IPv6 = 4,    // Number of IPv6 local candidates.
  kPeerConnectionMetricsName_Max
};

// The IceCandidatePairType has the format of
// <local_candidate_type>_<remote_candidate_type>. It is recorded based on the
// type of candidate pair used when the PeerConnection first goes to a completed
// state. When BUNDLE is enabled, only the first transport gets recorded.
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum IceCandidatePairType {
  // HostHost is deprecated. It was replaced with the set of types at the bottom
  // to report private or public host IP address.
  kIceCandidatePairHostHost = 0,
  kIceCandidatePairHostSrflx = 1,
  kIceCandidatePairHostRelay = 2,
  kIceCandidatePairHostPrflx = 3,
  kIceCandidatePairSrflxHost = 4,
  kIceCandidatePairSrflxSrflx = 5,
  kIceCandidatePairSrflxRelay = 6,
  kIceCandidatePairSrflxPrflx = 7,
  kIceCandidatePairRelayHost = 8,
  kIceCandidatePairRelaySrflx = 9,
  kIceCandidatePairRelayRelay = 10,
  kIceCandidatePairRelayPrflx = 11,
  kIceCandidatePairPrflxHost = 12,
  kIceCandidatePairPrflxSrflx = 13,
  kIceCandidatePairPrflxRelay = 14,

  // The following 9 types tell whether local and remote hosts have hostname,
  // private or public IP addresses.
  kIceCandidatePairHostPrivateHostPrivate = 15,
  kIceCandidatePairHostPrivateHostPublic = 16,
  kIceCandidatePairHostPublicHostPrivate = 17,
  kIceCandidatePairHostPublicHostPublic = 18,
  kIceCandidatePairHostNameHostName = 19,
  kIceCandidatePairHostNameHostPrivate = 20,
  kIceCandidatePairHostNameHostPublic = 21,
  kIceCandidatePairHostPrivateHostName = 22,
  kIceCandidatePairHostPublicHostName = 23,
  kIceCandidatePairMax
};

// The difference between PeerConnectionEnumCounter and
// PeerConnectionMetricsName is that the "EnumCounter" is only counting the
// occurrences of events, while "Name" has a value associated with it which is
// used to form a histogram.

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum KeyExchangeProtocolMedia {
  kEnumCounterKeyProtocolMediaTypeDtlsAudio = 0,
  kEnumCounterKeyProtocolMediaTypeDtlsVideo = 1,
  kEnumCounterKeyProtocolMediaTypeDtlsData = 2,
  kEnumCounterKeyProtocolMediaTypeSdesAudio = 3,
  kEnumCounterKeyProtocolMediaTypeSdesVideo = 4,
  kEnumCounterKeyProtocolMediaTypeSdesData = 5,
  kEnumCounterKeyProtocolMediaTypeMax
};

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum SdpSemanticRequested {
  kSdpSemanticRequestDefault = 0,
  kSdpSemanticRequestPlanB = 1,
  kSdpSemanticRequestUnifiedPlan = 2,
  kSdpSemanticRequestMax
};

// Metric for counting the outcome of adding an ICE candidate
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum AddIceCandidateResult {
  kAddIceCandidateSuccess = 0,
  kAddIceCandidateFailClosed = 1,
  kAddIceCandidateFailNoRemoteDescription = 2,
  kAddIceCandidateFailNullCandidate = 3,
  kAddIceCandidateFailNotValid = 4,
  kAddIceCandidateFailNotReady = 5,
  kAddIceCandidateFailInAddition = 6,
  kAddIceCandidateFailNotUsable = 7,
  kAddIceCandidateMax
};

// Metrics for reporting usage of BUNDLE.
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum BundleUsage {
  // There are no m-lines in the SDP, only a session description.
  kBundleUsageEmpty = 0,
  // Only a data channel is negotiated but BUNDLE is not negotiated.
  kBundleUsageNoBundleDatachannelOnly = 1,
  // BUNDLE is not negotiated and there is at most one m-line per media type,
  kBundleUsageNoBundleSimple = 2,
  // BUNDLE is not negotiated and there are multiple m-lines per media type,
  kBundleUsageNoBundleComplex = 3,
  // Only a data channel is negotiated and BUNDLE is negotiated.
  kBundleUsageBundleDatachannelOnly = 4,
  // BUNDLE is negotiated but there is at most one m-line per media type,
  kBundleUsageBundleSimple = 5,
  // BUNDLE is negotiated and there are multiple m-lines per media type,
  kBundleUsageBundleComplex = 6,
  // Legacy plan-b metrics.
  kBundleUsageNoBundlePlanB = 7,
  kBundleUsageBundlePlanB = 8,
  kBundleUsageMax
};

// Metrics for reporting configured BUNDLE policy, mapping directly to
// https://w3c.github.io/webrtc-pc/#rtcbundlepolicy-enum
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum BundlePolicyUsage {
  kBundlePolicyUsageBalanced = 0,
  kBundlePolicyUsageMaxBundle = 1,
  kBundlePolicyUsageMaxCompat = 2,
  kBundlePolicyUsageMax
};

// Metrics for provisional answers as described in
// https://datatracker.ietf.org/doc/html/rfc8829#section-4.1.10.1
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum ProvisionalAnswerUsage {
  kProvisionalAnswerNotUsed = 0,
  kProvisionalAnswerLocal = 1,
  kProvisionalAnswerRemote = 2,
  kProvisionalAnswerMax
};

// Metrics for RTCRtpMuxPolicy. The only defined value is
// https://w3c.github.io/webrtc-pc/#rtcrtcpmuxpolicy-enum
// "require" but there is a legacy option "negotiate" which
// was removed from the spec.
enum RtcpMuxPolicyUsage {
  kRtcpMuxPolicyUsageRequire = 0,
  kRtcpMuxPolicyUsageNegotiate = 1,
  kRtcpMuxPolicyUsageMax
};

// Metrics for SDP munging.
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused. Keep in sync with SdpMungingType from
// tools/metrics/histograms/metadata/web_rtc/enums.xml
enum SdpMungingType {
  kNoModification = 0,
  kUnknownModification = 1,
  kWithoutCreateAnswer = 2,
  kWithoutCreateOffer = 3,
  kNumberOfContents = 4,
  // Transport-related munging.
  kIceOptions = 20,
  kIcePwd = 21,
  kIceUfrag = 22,
  kIceMode = 23,
  kDtlsSetup = 24,
  kMid = 25,
  kPayloadTypes = 26,
  kSsrcs = 27,
  kIceOptionsRenomination = 28,
  // RTP header extension munging.
  kRtpHeaderExtensionRemoved = 40,
  kRtpHeaderExtensionAdded = 41,
  kRtpHeaderExtensionModified = 42,
  // Audio-related munging.
  kAudioCodecsRemoved = 60,
  kAudioCodecsAdded = 61,
  kAudioCodecsReordered = 62,
  kAudioCodecsAddedMultiOpus = 63,
  kAudioCodecsAddedL16 = 64,
  kAudioCodecsRtcpFbAudioNack = 65,
  kAudioCodecsFmtpOpusFec = 66,
  kAudioCodecsFmtpOpusCbr = 67,
  kAudioCodecsFmtpOpusStereo = 68,
  kAudioCodecsFmtpOpusDtx = 69,
  kAudioCodecsFmtp = 70,
  kAudioCodecsRtcpFb = 71,
  kAudioCodecsRtcpFbRrtr = 72,
  // Video-related munging.
  kVideoCodecsRemoved = 80,
  kVideoCodecsAdded = 81,
  kVideoCodecsReordered = 82,
  kVideoCodecsLegacySimulcast = 83,
  kVideoCodecsFmtpH264SpsPpsIdrInKeyframe = 84,
  kVideoCodecsFmtp = 85,
  kVideoCodecsRtcpFb = 86,
  kMaxValue,
};

// When adding new metrics please consider using the style described in
// https://chromium.googlesource.com/chromium/src.git/+/HEAD/tools/metrics/histograms/README.md#usage
// instead of the legacy enums used above.

}  // namespace webrtc

#endif  // API_UMA_METRICS_H_
