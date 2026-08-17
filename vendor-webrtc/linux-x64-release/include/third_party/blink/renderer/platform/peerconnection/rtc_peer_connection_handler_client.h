/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_PEER_CONNECTION_HANDLER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_PEER_CONNECTION_HANDLER_CLIENT_H_

#include <memory>
#include <optional>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/peer_connection_interface.h"
#include "third_party/webrtc/api/sctp_transport_interface.h"

namespace blink {

class RTCIceCandidatePlatform;
class RTCRtpTransceiverPlatform;
class RTCSessionDescriptionPlatform;

struct PLATFORM_EXPORT WebRTCSctpTransportSnapshot {
  webrtc::scoped_refptr<webrtc::SctpTransportInterface> transport;
  webrtc::SctpTransportInformation sctp_transport_state =
      webrtc::SctpTransportInformation(webrtc::SctpTransportState::kNew);
  webrtc::DtlsTransportInformation dtls_transport_state =
      webrtc::DtlsTransportInformation(webrtc::DtlsTransportState::kNew);
};

class PLATFORM_EXPORT RTCPeerConnectionHandlerClient
    : public GarbageCollectedMixin {
 public:
  virtual ~RTCPeerConnectionHandlerClient();

  virtual void NegotiationNeeded() = 0;
  virtual void DidGenerateICECandidate(RTCIceCandidatePlatform*) = 0;
  virtual void DidFailICECandidate(const String& address,
                                   std::optional<uint16_t> port,
                                   const String& host_candidate,
                                   const String& url,
                                   int error_code,
                                   const String& error_text) = 0;
  virtual void DidChangeSessionDescriptions(
      RTCSessionDescriptionPlatform* pending_local_description,
      RTCSessionDescriptionPlatform* current_local_description,
      RTCSessionDescriptionPlatform* pending_remote_description,
      RTCSessionDescriptionPlatform* current_remote_description) = 0;
  virtual void DidChangeIceGatheringState(
      webrtc::PeerConnectionInterface::IceGatheringState) = 0;
  virtual void DidChangePeerConnectionState(
      webrtc::PeerConnectionInterface::PeerConnectionState) {}
  virtual void DidModifyTransceivers(
      webrtc::PeerConnectionInterface::SignalingState,
      Vector<std::unique_ptr<RTCRtpTransceiverPlatform>>,
      Vector<uintptr_t>,
      bool is_remote_description_or_rollback) = 0;
  virtual void DidModifySctpTransport(WebRTCSctpTransportSnapshot) = 0;
  virtual void DidAddRemoteDataChannel(
      webrtc::scoped_refptr<webrtc::DataChannelInterface>) = 0;
  virtual void DidNoteInterestingUsage(int usage_pattern) = 0;
  virtual void UnregisterPeerConnectionHandler() = 0;
  virtual void ClosePeerConnection();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_PEER_CONNECTION_HANDLER_CLIENT_H_
