/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_TOOLS_DATA_CHANNEL_BENCHMARK_PEER_CONNECTION_CLIENT_H_
#define RTC_TOOLS_DATA_CHANNEL_BENCHMARK_PEER_CONNECTION_CLIENT_H_

#include <stdint.h>

#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "api/data_channel_interface.h"
#include "api/jsep.h"
#include "api/peer_connection_interface.h"
#include "api/rtp_receiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/set_local_description_observer_interface.h"
#include "rtc_base/logging.h"
#include "rtc_base/thread.h"
#include "rtc_tools/data_channel_benchmark/signaling_interface.h"

namespace webrtc {

// Handles all the details for creating a PeerConnection and negotiation using a
// SignalingInterface object.
class PeerConnectionClient : public webrtc::PeerConnectionObserver {
 public:
  explicit PeerConnectionClient(webrtc::PeerConnectionFactoryInterface* factory,
                                webrtc::SignalingInterface* signaling);

  ~PeerConnectionClient() override;

  PeerConnectionClient(const PeerConnectionClient&) = delete;
  PeerConnectionClient& operator=(const PeerConnectionClient&) = delete;

  // Set the local description and send offer using the SignalingInterface,
  // initiating the negotiation process.
  bool StartPeerConnection();

  // Whether the peer connection is connected to the remote peer.
  bool IsConnected();

  // Disconnect from the call.
  void Disconnect();

  scoped_refptr<webrtc::PeerConnectionInterface> peerConnection() {
    return peer_connection_;
  }

  // Set a callback to run when a DataChannel is created by the remote peer.
  void SetOnDataChannel(
      std::function<void(webrtc::scoped_refptr<webrtc::DataChannelInterface>)>
          callback);

  std::vector<scoped_refptr<webrtc::DataChannelInterface>>& dataChannels() {
    return data_channels_;
  }

  // Creates a default PeerConnectionFactory object.
  static scoped_refptr<webrtc::PeerConnectionFactoryInterface>
  CreateDefaultFactory(Thread* signaling_thread);

 private:
  void AddIceCandidate(
      std::unique_ptr<webrtc::IceCandidateInterface> candidate);
  bool SetRemoteDescription(
      std::unique_ptr<webrtc::SessionDescriptionInterface> desc);

  // Initialize the PeerConnection with a given PeerConnectionFactory.
  bool InitializePeerConnection(
      webrtc::PeerConnectionFactoryInterface* factory);
  void DeletePeerConnection();

  // PeerConnectionObserver implementation.
  void OnSignalingChange(
      webrtc::PeerConnectionInterface::SignalingState new_state) override {
    RTC_LOG(LS_INFO) << __FUNCTION__ << " new state: " << new_state;
  }
  void OnDataChannel(
      scoped_refptr<webrtc::DataChannelInterface> channel) override;
  void OnNegotiationNeededEvent(uint32_t event_id) override;
  void OnIceConnectionChange(
      webrtc::PeerConnectionInterface::IceConnectionState new_state) override;
  void OnIceGatheringChange(
      webrtc::PeerConnectionInterface::IceGatheringState new_state) override;
  void OnIceCandidate(const webrtc::IceCandidateInterface* candidate) override;
  void OnIceConnectionReceivingChange(bool receiving) override {
    RTC_LOG(LS_INFO) << __FUNCTION__ << " receiving? " << receiving;
  }

  scoped_refptr<webrtc::PeerConnectionInterface> peer_connection_;
  std::function<void(webrtc::scoped_refptr<webrtc::DataChannelInterface>)>
      on_data_channel_callback_;
  std::vector<scoped_refptr<webrtc::DataChannelInterface>> data_channels_;
  webrtc::SignalingInterface* signaling_;
};

}  // namespace webrtc

#endif  // RTC_TOOLS_DATA_CHANNEL_BENCHMARK_PEER_CONNECTION_CLIENT_H_
