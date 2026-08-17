/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_TEST_MOCK_ICE_TRANSPORT_H_
#define P2P_TEST_MOCK_ICE_TRANSPORT_H_

#include <cstddef>
#include <optional>
#include <string>

#include "api/candidate.h"
#include "api/transport/enums.h"
#include "p2p/base/candidate_pair_interface.h"
#include "p2p/base/connection.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/transport_description.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/socket.h"
#include "test/gmock.h"

namespace webrtc {

// Used in Chromium/remoting/protocol/channel_socket_adapter_unittest.cc
class MockIceTransport : public IceTransportInternal {
 public:
  MockIceTransport() {
    SignalReadyToSend(this);
    SignalWritableState(this);
  }

  MOCK_METHOD(int,
              SendPacket,
              (const char* data,
               size_t len,
               const AsyncSocketPacketOptions& options,
               int flags),
              (override));
  MOCK_METHOD(int, SetOption, (Socket::Option opt, int value), (override));
  MOCK_METHOD(int, GetError, (), (override));
  MOCK_METHOD(IceRole, GetIceRole, (), (const, override));
  MOCK_METHOD(bool,
              GetStats,
              (IceTransportStats * ice_transport_stats),
              (override));
  MOCK_METHOD(IceTransportStateInternal, GetState, (), (const override));
  MOCK_METHOD(IceTransportState, GetIceTransportState, (), (const override));

  MOCK_METHOD(const std::string&, transport_name, (), (const override));
  MOCK_METHOD(int, component, (), (const override));
  MOCK_METHOD(void, SetIceRole, (IceRole), (override));
  // The ufrag and pwd in `ice_params` must be set
  // before candidate gathering can start.
  MOCK_METHOD(void, SetIceParameters, (const IceParameters&), (override));
  MOCK_METHOD(void, SetRemoteIceParameters, (const IceParameters&), (override));
  MOCK_METHOD(IceParameters*, local_ice_parameters, (), (const, override));
  MOCK_METHOD(IceParameters*, remote_ice_parameters, (), (const, override));
  MOCK_METHOD(void, SetRemoteIceMode, (IceMode), (override));
  MOCK_METHOD(void, SetIceConfig, (const IceConfig& config), (override));
  MOCK_METHOD(const IceConfig&, config, (), (const override));
  MOCK_METHOD(std::optional<int>, GetRttEstimate, (), (override));
  MOCK_METHOD(const Connection*, selected_connection, (), (const, override));
  MOCK_METHOD(std::optional<const CandidatePair>,
              GetSelectedCandidatePair,
              (),
              (const, override));
  MOCK_METHOD(void, MaybeStartGathering, (), (override));
  MOCK_METHOD(void, AddRemoteCandidate, (const Candidate&), (override));
  MOCK_METHOD(void, RemoveRemoteCandidate, (const Candidate&), (override));
  MOCK_METHOD(void, RemoveAllRemoteCandidates, (), (override));
  MOCK_METHOD(IceGatheringState, gathering_state, (), (const override));

  MOCK_METHOD(bool, receiving, (), (const override));
  MOCK_METHOD(bool, writable, (), (const override));
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::MockIceTransport;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_TEST_MOCK_ICE_TRANSPORT_H_
