/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TRANSPORT_TEST_MOCK_NETWORK_CONTROL_H_
#define API_TRANSPORT_TEST_MOCK_NETWORK_CONTROL_H_

#include <optional>

#include "api/transport/network_control.h"
#include "api/transport/network_types.h"
#include "test/gmock.h"

namespace webrtc {

class MockNetworkControllerInterface : public NetworkControllerInterface {
 public:
  MOCK_METHOD(NetworkControlUpdate,
              OnNetworkAvailability,
              (NetworkAvailability),
              (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnProcessInterval,
              (ProcessInterval),
              (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnNetworkRouteChange,
              (NetworkRouteChange),
              (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnRemoteBitrateReport,
              (RemoteBitrateReport),
              (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnRoundTripTimeUpdate,
              (RoundTripTimeUpdate),
              (override));
  MOCK_METHOD(NetworkControlUpdate, OnSentPacket, (SentPacket), (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnReceivedPacket,
              (ReceivedPacket),
              (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnStreamsConfig,
              (StreamsConfig),
              (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnTargetRateConstraints,
              (TargetRateConstraints),
              (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnTransportLossReport,
              (TransportLossReport),
              (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnTransportPacketsFeedback,
              (TransportPacketsFeedback),
              (override));
  MOCK_METHOD(NetworkControlUpdate,
              OnNetworkStateEstimate,
              (NetworkStateEstimate),
              (override));
};

class MockNetworkStateEstimator : public NetworkStateEstimator {
 public:
  MOCK_METHOD(std::optional<NetworkStateEstimate>,
              GetCurrentEstimate,
              (),
              (override));
  MOCK_METHOD(void,
              OnTransportPacketsFeedback,
              (const TransportPacketsFeedback&),
              (override));
  MOCK_METHOD(void, OnReceivedPacket, (const PacketResult&), (override));
  MOCK_METHOD(void, OnRouteChange, (const NetworkRouteChange&), (override));
};

}  // namespace webrtc

#endif  // API_TRANSPORT_TEST_MOCK_NETWORK_CONTROL_H_
