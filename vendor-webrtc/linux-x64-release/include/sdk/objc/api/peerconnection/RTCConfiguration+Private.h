/*
 *  Copyright 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#import "RTCConfiguration.h"

#include "api/peer_connection_interface.h"

NS_ASSUME_NONNULL_BEGIN

@interface RTC_OBJC_TYPE (RTCConfiguration)
()

    + (webrtc::PeerConnectionInterface::IceTransportsType)nativeTransportsTypeForTransportPolicy
    : (RTC_OBJC_TYPE(RTCIceTransportPolicy))policy;

+ (RTC_OBJC_TYPE(RTCIceTransportPolicy))transportPolicyForTransportsType:
    (webrtc::PeerConnectionInterface::IceTransportsType)nativeType;

+ (NSString *)stringForTransportPolicy:(RTC_OBJC_TYPE(RTCIceTransportPolicy))policy;

+ (webrtc::PeerConnectionInterface::BundlePolicy)nativeBundlePolicyForPolicy:
    (RTC_OBJC_TYPE(RTCBundlePolicy))policy;

+ (RTC_OBJC_TYPE(RTCBundlePolicy))bundlePolicyForNativePolicy:
    (webrtc::PeerConnectionInterface::BundlePolicy)nativePolicy;

+ (NSString *)stringForBundlePolicy:(RTC_OBJC_TYPE(RTCBundlePolicy))policy;

+ (webrtc::PeerConnectionInterface::RtcpMuxPolicy)nativeRtcpMuxPolicyForPolicy:
    (RTC_OBJC_TYPE(RTCRtcpMuxPolicy))policy;

+ (RTC_OBJC_TYPE(RTCRtcpMuxPolicy))rtcpMuxPolicyForNativePolicy:
    (webrtc::PeerConnectionInterface::RtcpMuxPolicy)nativePolicy;

+ (NSString *)stringForRtcpMuxPolicy:(RTC_OBJC_TYPE(RTCRtcpMuxPolicy))policy;

+ (webrtc::PeerConnectionInterface::TcpCandidatePolicy)nativeTcpCandidatePolicyForPolicy:
    (RTC_OBJC_TYPE(RTCTcpCandidatePolicy))policy;

+ (RTC_OBJC_TYPE(RTCTcpCandidatePolicy))tcpCandidatePolicyForNativePolicy:
    (webrtc::PeerConnectionInterface::TcpCandidatePolicy)nativePolicy;

+ (NSString *)stringForTcpCandidatePolicy:(RTC_OBJC_TYPE(RTCTcpCandidatePolicy))policy;

+ (webrtc::PeerConnectionInterface::CandidateNetworkPolicy)nativeCandidateNetworkPolicyForPolicy:
    (RTC_OBJC_TYPE(RTCCandidateNetworkPolicy))policy;

+ (RTC_OBJC_TYPE(RTCCandidateNetworkPolicy))candidateNetworkPolicyForNativePolicy:
    (webrtc::PeerConnectionInterface::CandidateNetworkPolicy)nativePolicy;

+ (NSString *)stringForCandidateNetworkPolicy:(RTC_OBJC_TYPE(RTCCandidateNetworkPolicy))policy;

+ (rtc::KeyType)nativeEncryptionKeyTypeForKeyType:(RTC_OBJC_TYPE(RTCEncryptionKeyType))keyType;

+ (NSString *)stringForSdpSemantics:(RTC_OBJC_TYPE(RTCSdpSemantics))sdpSemantics;
+ (webrtc::SdpSemantics)nativeSdpSemanticsForSdpSemantics:(RTC_OBJC_TYPE(RTCSdpSemantics))sdpSemantics;

+ (RTC_OBJC_TYPE(RTCSdpSemantics))sdpSemanticsForNativeSdpSemantics:(webrtc::SdpSemantics)sdpSemantics;

/**
 * RTCConfiguration struct representation of this RTCConfiguration.
 * This is needed to pass to the underlying C++ APIs.
 */
- (nullable webrtc::PeerConnectionInterface::RTCConfiguration *)
    createNativeConfiguration;

- (instancetype)initWithNativeConfiguration:
    (const webrtc::PeerConnectionInterface::RTCConfiguration &)config
    NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
