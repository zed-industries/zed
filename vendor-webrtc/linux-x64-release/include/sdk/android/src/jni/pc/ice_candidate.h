/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_PC_ICE_CANDIDATE_H_
#define SDK_ANDROID_SRC_JNI_PC_ICE_CANDIDATE_H_

#include <vector>

#include "api/data_channel_interface.h"
#include "api/jsep.h"
#include "api/jsep_ice_candidate.h"
#include "api/peer_connection_interface.h"
#include "api/rtp_parameters.h"
#include "rtc_base/ssl_identity.h"
#include "sdk/android/src/jni/jni_helpers.h"

namespace webrtc {
namespace jni {

Candidate JavaToNativeCandidate(JNIEnv* jni,
                                const JavaRef<jobject>& j_candidate);

ScopedJavaLocalRef<jobject> NativeToJavaCandidate(JNIEnv* env,
                                                  const Candidate& candidate);

ScopedJavaLocalRef<jobject> NativeToJavaIceCandidate(
    JNIEnv* env,
    const IceCandidateInterface& candidate);

ScopedJavaLocalRef<jobjectArray> NativeToJavaCandidateArray(
    JNIEnv* jni,
    const std::vector<Candidate>& candidates);

/*****************************************************
 * Below are all things that go into RTCConfiguration.
 *****************************************************/
PeerConnectionInterface::IceTransportsType JavaToNativeIceTransportsType(
    JNIEnv* jni,
    const JavaRef<jobject>& j_ice_transports_type);

PeerConnectionInterface::BundlePolicy JavaToNativeBundlePolicy(
    JNIEnv* jni,
    const JavaRef<jobject>& j_bundle_policy);

PeerConnectionInterface::RtcpMuxPolicy JavaToNativeRtcpMuxPolicy(
    JNIEnv* jni,
    const JavaRef<jobject>& j_rtcp_mux_policy);

PeerConnectionInterface::TcpCandidatePolicy JavaToNativeTcpCandidatePolicy(
    JNIEnv* jni,
    const JavaRef<jobject>& j_tcp_candidate_policy);

PeerConnectionInterface::CandidateNetworkPolicy
JavaToNativeCandidateNetworkPolicy(
    JNIEnv* jni,
    const JavaRef<jobject>& j_candidate_network_policy);

KeyType JavaToNativeKeyType(JNIEnv* jni, const JavaRef<jobject>& j_key_type);

PeerConnectionInterface::ContinualGatheringPolicy
JavaToNativeContinualGatheringPolicy(
    JNIEnv* jni,
    const JavaRef<jobject>& j_gathering_policy);

webrtc::PortPrunePolicy JavaToNativePortPrunePolicy(
    JNIEnv* jni,
    const JavaRef<jobject>& j_port_prune_policy);

PeerConnectionInterface::TlsCertPolicy JavaToNativeTlsCertPolicy(
    JNIEnv* jni,
    const JavaRef<jobject>& j_ice_server_tls_cert_policy);

std::optional<AdapterType> JavaToNativeNetworkPreference(
    JNIEnv* jni,
    const JavaRef<jobject>& j_network_preference);

}  // namespace jni
}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_PC_ICE_CANDIDATE_H_
