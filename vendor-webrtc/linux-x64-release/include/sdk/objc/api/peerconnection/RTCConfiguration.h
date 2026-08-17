/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import <Foundation/Foundation.h>

#import "RTCCertificate.h"
#import "RTCCryptoOptions.h"
#import "sdk/objc/base/RTCMacros.h"

@class RTC_OBJC_TYPE(RTCIceServer);

/**
 * Represents the ice transport policy. This exposes the same states in C++,
 * which include one more state than what exists in the W3C spec.
 */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCIceTransportPolicy)) {
  RTC_OBJC_TYPE(RTCIceTransportPolicyNone),
  RTC_OBJC_TYPE(RTCIceTransportPolicyRelay),
  RTC_OBJC_TYPE(RTCIceTransportPolicyNoHost),
  RTC_OBJC_TYPE(RTCIceTransportPolicyAll)
};

/** Represents the bundle policy. */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCBundlePolicy)) {
  RTC_OBJC_TYPE(RTCBundlePolicyBalanced),
  RTC_OBJC_TYPE(RTCBundlePolicyMaxCompat),
  RTC_OBJC_TYPE(RTCBundlePolicyMaxBundle)
};

/** Represents the rtcp mux policy. */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCRtcpMuxPolicy)) {
  RTC_OBJC_TYPE(RTCRtcpMuxPolicyNegotiate),
  RTC_OBJC_TYPE(RTCRtcpMuxPolicyRequire)
};

/** Represents the tcp candidate policy. */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCTcpCandidatePolicy)) {
  RTC_OBJC_TYPE(RTCTcpCandidatePolicyEnabled),
  RTC_OBJC_TYPE(RTCTcpCandidatePolicyDisabled)
};

/** Represents the candidate network policy. */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCCandidateNetworkPolicy)) {
  RTC_OBJC_TYPE(RTCCandidateNetworkPolicyAll),
  RTC_OBJC_TYPE(RTCCandidateNetworkPolicyLowCost)
};

/** Represents the continual gathering policy. */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCContinualGatheringPolicy)) {
  RTC_OBJC_TYPE(RTCContinualGatheringPolicyGatherOnce),
  RTC_OBJC_TYPE(RTCContinualGatheringPolicyGatherContinually)
};

/** Represents the encryption key type. */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCEncryptionKeyType)) {
  RTC_OBJC_TYPE(RTCEncryptionKeyTypeRSA),
  RTC_OBJC_TYPE(RTCEncryptionKeyTypeECDSA),
};

/** Represents the chosen SDP semantics for the RTCPeerConnection. */
typedef NS_ENUM(NSInteger, RTC_OBJC_TYPE(RTCSdpSemantics)) {
  // TODO(https://crbug.com/webrtc/13528): Remove support for Plan B.
  RTC_OBJC_TYPE(RTCSdpSemanticsPlanB),
  RTC_OBJC_TYPE(RTCSdpSemanticsUnifiedPlan),
};

NS_ASSUME_NONNULL_BEGIN

RTC_OBJC_EXPORT
@interface RTC_OBJC_TYPE (RTCConfiguration) : NSObject

/** If true, allows DSCP codes to be set on outgoing packets, configured using
 *  networkPriority field of RTCRtpEncodingParameters. Defaults to false.
 */
@property(nonatomic, assign) BOOL enableDscp;

/** An array of Ice Servers available to be used by ICE. */
@property(nonatomic, copy) NSArray<RTC_OBJC_TYPE(RTCIceServer) *> *iceServers;

/** An RTCCertificate for 're' use. */
@property(nonatomic, nullable) RTC_OBJC_TYPE(RTCCertificate) * certificate;

/** Which candidates the ICE agent is allowed to use. The W3C calls it
 * `iceTransportPolicy`, while in C++ it is called `type`. */
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCIceTransportPolicy) iceTransportPolicy;

/** The media-bundling policy to use when gathering ICE candidates. */
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCBundlePolicy) bundlePolicy;

/** The rtcp-mux policy to use when gathering ICE candidates. */
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCRtcpMuxPolicy) rtcpMuxPolicy;
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCTcpCandidatePolicy) tcpCandidatePolicy;
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCCandidateNetworkPolicy) candidateNetworkPolicy;
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCContinualGatheringPolicy) continualGatheringPolicy;

/** If set to YES, don't gather IPv6 ICE candidates on Wi-Fi.
 *  Only intended to be used on specific devices. Certain phones disable IPv6
 *  when the screen is turned off and it would be better to just disable the
 *  IPv6 ICE candidates on Wi-Fi in those cases.
 *  Default is NO.
 */
@property(nonatomic, assign) BOOL disableIPV6OnWiFi;

/** By default, the PeerConnection will use a limited number of IPv6 network
 *  interfaces, in order to avoid too many ICE candidate pairs being created
 *  and delaying ICE completion.
 *
 *  Can be set to INT_MAX to effectively disable the limit.
 */
@property(nonatomic, assign) int maxIPv6Networks;

/** Exclude link-local network interfaces
 *  from considertaion for gathering ICE candidates.
 *  Defaults to NO.
 */
@property(nonatomic, assign) BOOL disableLinkLocalNetworks;

@property(nonatomic, assign) int audioJitterBufferMaxPackets;
@property(nonatomic, assign) BOOL audioJitterBufferFastAccelerate;
@property(nonatomic, assign) int iceConnectionReceivingTimeout;
@property(nonatomic, assign) int iceBackupCandidatePairPingInterval;

/** Key type used to generate SSL identity. Default is ECDSA. */
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCEncryptionKeyType) keyType;

/** ICE candidate pool size as defined in JSEP. Default is 0. */
@property(nonatomic, assign) int iceCandidatePoolSize;

/** Prune turn ports on the same network to the same turn server.
 *  Default is NO.
 */
@property(nonatomic, assign) BOOL shouldPruneTurnPorts;

/** If set to YES, this means the ICE transport should presume TURN-to-TURN
 *  candidate pairs will succeed, even before a binding response is received.
 */
@property(nonatomic, assign) BOOL shouldPresumeWritableWhenFullyRelayed;

/* This flag is only effective when `continualGatheringPolicy` is
 * RTCContinualGatheringPolicyGatherContinually.
 *
 * If YES, after the ICE transport type is changed such that new types of
 * ICE candidates are allowed by the new transport type, e.g. from
 * RTCIceTransportPolicyRelay to RTCIceTransportPolicyAll, candidates that
 * have been gathered by the ICE transport but not matching the previous
 * transport type and as a result not observed by PeerConnectionDelegateAdapter,
 * will be surfaced to the delegate.
 */
@property(nonatomic, assign)
    BOOL shouldSurfaceIceCandidatesOnIceTransportTypeChanged;

/** If set to non-nil, controls the minimal interval between consecutive ICE
 *  check packets.
 */
@property(nonatomic, copy, nullable) NSNumber *iceCheckMinInterval;

/**
 * Configure the SDP semantics used by this PeerConnection. By default, this
 * is RTCSdpSemanticsUnifiedPlan which is compliant to the WebRTC 1.0
 * specification. It is possible to overrwite this to the deprecated
 * RTCSdpSemanticsPlanB SDP format, but note that RTCSdpSemanticsPlanB will be
 * deleted at some future date, see https://crbug.com/webrtc/13528.
 *
 * RTCSdpSemanticsUnifiedPlan will cause RTCPeerConnection to create offers and
 * answers with multiple m= sections where each m= section maps to one
 * RTCRtpSender and one RTCRtpReceiver (an RTCRtpTransceiver), either both audio
 * or both video. This will also cause RTCPeerConnection to ignore all but the
 * first a=ssrc lines that form a Plan B stream.
 *
 * RTCSdpSemanticsPlanB will cause RTCPeerConnection to create offers and
 * answers with at most one audio and one video m= section with multiple
 * RTCRtpSenders and RTCRtpReceivers specified as multiple a=ssrc lines within
 * the section. This will also cause RTCPeerConnection to ignore all but the
 * first m= section of the same media type.
 */
@property(nonatomic, assign) RTC_OBJC_TYPE(RTCSdpSemantics) sdpSemantics;

/** Actively reset the SRTP parameters when the DTLS transports underneath are
 *  changed after offer/answer negotiation. This is only intended to be a
 *  workaround for crbug.com/835958
 */
@property(nonatomic, assign) BOOL activeResetSrtpParams;

/**
 * Defines advanced optional cryptographic settings related to SRTP and
 * frame encryption for native WebRTC. Setting this will overwrite any
 * options set through the PeerConnectionFactory (which is deprecated).
 */
@property(nonatomic, nullable) RTC_OBJC_TYPE(RTCCryptoOptions) * cryptoOptions;

/**
 * An optional string that will be attached to the TURN_ALLOCATE_REQUEST which
 * which can be used to correlate client logs with backend logs.
 */
@property(nonatomic, nullable, copy) NSString *turnLoggingId;

/**
 * Time interval between audio RTCP reports.
 */
@property(nonatomic, assign) int rtcpAudioReportIntervalMs;

/**
 * Time interval between video RTCP reports.
 */
@property(nonatomic, assign) int rtcpVideoReportIntervalMs;

/**
 * Allow implicit rollback of local description when remote description
 * conflicts with local description.
 * See: https://w3c.github.io/webrtc-pc/#dom-peerconnection-setremotedescription
 */
@property(nonatomic, assign) BOOL enableImplicitRollback;

/**
 * Control if "a=extmap-allow-mixed" is included in the offer.
 * See: https://www.chromestatus.com/feature/6269234631933952
 */
@property(nonatomic, assign) BOOL offerExtmapAllowMixed;

/**
 * Defines the interval applied to ALL candidate pairs
 * when ICE is strongly connected, and it overrides the
 * default value of this interval in the ICE implementation;
 */
@property(nonatomic, copy, nullable)
    NSNumber *iceCheckIntervalStrongConnectivity;

/**
 * Defines the counterpart for ALL pairs when ICE is
 * weakly connected, and it overrides the default value of
 * this interval in the ICE implementation
 */
@property(nonatomic, copy, nullable) NSNumber *iceCheckIntervalWeakConnectivity;

/**
 * The min time period for which a candidate pair must wait for response to
 * connectivity checks before it becomes unwritable. This parameter
 * overrides the default value in the ICE implementation if set.
 */
@property(nonatomic, copy, nullable) NSNumber *iceUnwritableTimeout;

/**
 * The min number of connectivity checks that a candidate pair must sent
 * without receiving response before it becomes unwritable. This parameter
 * overrides the default value in the ICE implementation if set.
 */
@property(nonatomic, copy, nullable) NSNumber *iceUnwritableMinChecks;

/**
 * The min time period for which a candidate pair must wait for response to
 * connectivity checks it becomes inactive. This parameter overrides the
 * default value in the ICE implementation if set.
 */
@property(nonatomic, copy, nullable) NSNumber *iceInactiveTimeout;

/**
  * When this flag is set, ports not bound to any specific network interface
  * will be used, in addition to normal ports bound to the enumerated
  * interfaces. Without this flag, these "any address" ports would only be
  * used when network enumeration fails or is disabled. But under certain
  * conditions, these ports may succeed where others fail, so they may allow
  * the application to work in a wider variety of environments, at the expense
  * of having to allocate additional candidates.
  */
@property(nonatomic, assign) BOOL enableIceGatheringOnAnyAddressPorts;

- (instancetype)init;

@end

NS_ASSUME_NONNULL_END
