/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TRANSPORT_TEST_FEEDBACK_GENERATOR_INTERFACE_H_
#define API_TRANSPORT_TEST_FEEDBACK_GENERATOR_INTERFACE_H_

#include <cstddef>
#include <vector>

#include "api/test/simulated_network.h"
#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"

namespace webrtc {
class FeedbackGenerator {
 public:
  struct Config {
    BuiltInNetworkBehaviorConfig send_link;
    BuiltInNetworkBehaviorConfig return_link;
    TimeDelta feedback_interval = TimeDelta::Millis(50);
    DataSize feedback_packet_size = DataSize::Bytes(20);
  };
  virtual ~FeedbackGenerator() = default;
  virtual Timestamp Now() = 0;
  virtual void Sleep(TimeDelta duration) = 0;
  virtual void SendPacket(size_t size) = 0;
  virtual std::vector<TransportPacketsFeedback> PopFeedback() = 0;
  virtual void SetSendConfig(BuiltInNetworkBehaviorConfig config) = 0;
  virtual void SetReturnConfig(BuiltInNetworkBehaviorConfig config) = 0;
  virtual void SetSendLinkCapacity(DataRate capacity) = 0;
};
}  // namespace webrtc
#endif  // API_TRANSPORT_TEST_FEEDBACK_GENERATOR_INTERFACE_H_
