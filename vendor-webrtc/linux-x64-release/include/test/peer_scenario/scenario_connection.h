/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_PEER_SCENARIO_SCENARIO_CONNECTION_H_
#define TEST_PEER_SCENARIO_SCENARIO_CONNECTION_H_

#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "api/candidate.h"
#include "api/environment/environment.h"
#include "api/jsep.h"
#include "p2p/base/transport_description.h"
#include "test/network/network_emulation_manager.h"
#include "test/scoped_key_value_config.h"

namespace webrtc {

// ScenarioIceConnection provides the transport level functionality of a
// PeerConnection for use in peer connection scenario tests. This allows
// implementing custom server side behavior in tests.
class ScenarioIceConnection {
 public:
  class IceConnectionObserver {
   public:
    // Called on network thread.
    virtual void OnPacketReceived(CopyOnWriteBuffer packet) = 0;
    // Called on signaling thread.
    virtual void OnIceCandidates(const std::string& mid,
                                 const std::vector<Candidate>& candidates) = 0;

   protected:
    ~IceConnectionObserver() = default;
  };
  static std::unique_ptr<ScenarioIceConnection> Create(
      const Environment& env,
      test::NetworkEmulationManagerImpl* net,
      IceConnectionObserver* observer);

  virtual ~ScenarioIceConnection() = default;

  // Posts tasks to send packets to network thread.
  virtual void SendRtpPacket(ArrayView<const uint8_t> packet_view) = 0;
  virtual void SendRtcpPacket(ArrayView<const uint8_t> packet_view) = 0;

  // Used for ICE configuration, called on signaling thread.
  virtual void SetRemoteSdp(SdpType type, const std::string& remote_sdp) = 0;
  virtual void SetLocalSdp(SdpType type, const std::string& local_sdp) = 0;

  virtual EmulatedEndpoint* endpoint() = 0;
  virtual const TransportDescription& transport_description() const = 0;

  webrtc::test::ScopedKeyValueConfig field_trials;
};

}  // namespace webrtc

#endif  // TEST_PEER_SCENARIO_SCENARIO_CONNECTION_H_
