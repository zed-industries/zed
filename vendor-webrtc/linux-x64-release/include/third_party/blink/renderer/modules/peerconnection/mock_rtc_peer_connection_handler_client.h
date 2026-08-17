// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MOCK_RTC_PEER_CONNECTION_HANDLER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MOCK_RTC_PEER_CONNECTION_HANDLER_CLIENT_H_

#include <memory>
#include <string>
#include <vector>

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_ice_candidate_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_peer_connection_handler_client.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_receiver_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_transceiver_platform.h"

namespace blink {

class MockRTCPeerConnectionHandlerClient
    : public GarbageCollected<MockRTCPeerConnectionHandlerClient>,
      public RTCPeerConnectionHandlerClient {
 public:
  MockRTCPeerConnectionHandlerClient();

  MockRTCPeerConnectionHandlerClient(
      const MockRTCPeerConnectionHandlerClient&) = delete;
  MockRTCPeerConnectionHandlerClient& operator=(
      const MockRTCPeerConnectionHandlerClient&) = delete;

  ~MockRTCPeerConnectionHandlerClient() override;

  // RTCPeerConnectionHandlerClient implementation.
  MOCK_METHOD0(NegotiationNeeded, void());
  MOCK_METHOD1(DidGenerateICECandidate,
               void(RTCIceCandidatePlatform* candidate));
  MOCK_METHOD6(DidFailICECandidate,
               void(const String& address,
                    std::optional<uint16_t> port,
                    const String& host_candidate,
                    const String& url,
                    int error_code,
                    const String& error_text));
  MOCK_METHOD4(DidChangeSessionDescriptions,
               void(RTCSessionDescriptionPlatform*,
                    RTCSessionDescriptionPlatform*,
                    RTCSessionDescriptionPlatform*,
                    RTCSessionDescriptionPlatform*));
  MOCK_METHOD1(DidChangeIceGatheringState,
               void(webrtc::PeerConnectionInterface::IceGatheringState state));
  MOCK_METHOD1(
      DidChangePeerConnectionState,
      void(webrtc::PeerConnectionInterface::PeerConnectionState state));
  MOCK_METHOD1(DidModifySctpTransport,
               void(blink::WebRTCSctpTransportSnapshot snapshot));
  void DidModifyTransceivers(
      webrtc::PeerConnectionInterface::SignalingState signaling_state,
      Vector<std::unique_ptr<RTCRtpTransceiverPlatform>> platform_transceivers,
      Vector<uintptr_t> removed_transceivers,
      bool is_remote_description) override {
    DidModifyTransceiversForMock(signaling_state, &platform_transceivers,
                                 is_remote_description);
  }
  MOCK_METHOD1(DidAddRemoteDataChannel,
               void(webrtc::scoped_refptr<webrtc::DataChannelInterface>));
  MOCK_METHOD1(DidNoteInterestingUsage, void(int));
  MOCK_METHOD0(UnregisterPeerConnectionHandler, void());

  // Move-only arguments do not play nicely with MOCK, the workaround is to
  // EXPECT_CALL with these instead.
  MOCK_METHOD3(DidModifyTransceiversForMock,
               void(webrtc::PeerConnectionInterface::SignalingState,
                    Vector<std::unique_ptr<RTCRtpTransceiverPlatform>>*,
                    bool));

  void didGenerateICECandidateWorker(RTCIceCandidatePlatform* candidate);
  void didModifyReceiversWorker(
      webrtc::PeerConnectionInterface::SignalingState,
      Vector<std::unique_ptr<RTCRtpReceiverPlatform>>* receivers_added,
      Vector<std::unique_ptr<RTCRtpReceiverPlatform>>* receivers_removed);

  const std::string& candidate_sdp() const { return candidate_sdp_; }
  const std::optional<uint16_t>& candidate_mlineindex() const {
    return candidate_mline_index_;
  }
  const std::string& candidate_mid() const { return candidate_mid_; }
  const String& remote_stream_id() const { return remote_stream_id_; }

 private:
  String remote_stream_id_;
  std::string candidate_sdp_;
  std::optional<uint16_t> candidate_mline_index_;
  std::string candidate_mid_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_MOCK_RTC_PEER_CONNECTION_HANDLER_CLIENT_H_
