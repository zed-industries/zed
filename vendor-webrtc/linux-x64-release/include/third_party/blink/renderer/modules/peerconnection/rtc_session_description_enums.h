// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_SESSION_DESCRIPTION_ENUMS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_SESSION_DESCRIPTION_ENUMS_H_

namespace blink {

// RTCPeerConnection methods that are used to create session descriptions (SDP).
// See RTCPeerConnection::createOffer() and RTCPeerConnection::createAnswer().
enum class RTCCreateSessionDescriptionOperation {
  kCreateOffer,
  kCreateAnswer,
};

// RTCPeerConnection operations that are used to set session descriptions (SDP).
// See methods RTCPeerConnection::setLocalDescription() and
// RTCPeerConnection::setRemoteDescription() - the SDP parameter can have the
// type "offer" or "answer".
enum class RTCSetSessionDescriptionOperation {
  kSetLocalDescriptionOffer,
  kSetLocalDescriptionAnswer,
  kSetLocalDescriptionInvalidType,
  kSetRemoteDescriptionOffer,
  kSetRemoteDescriptionAnswer,
  kSetRemoteDescriptionInvalidType,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_SESSION_DESCRIPTION_ENUMS_H_
