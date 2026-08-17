// Copyright 2023 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_ABUSE_POLICY_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_ABUSE_POLICY_H

#include <grpc/support/port_platform.h>

#include <string>

#include "src/core/lib/channel/channel_args.h"
#include "src/core/util/time.h"

namespace grpc_core {

class Chttp2PingAbusePolicy {
 public:
  explicit Chttp2PingAbusePolicy(const ChannelArgs& args);

  static void SetDefaults(const ChannelArgs& args);

  // Record one received ping; returns true if the connection should be closed.
  // If transport_idle is true, we increase the allowed time between pings up to
  // TCP keep-alive check time.
  GRPC_MUST_USE_RESULT bool ReceivedOnePing(bool transport_idle);

  // Reset the ping clock, strike count.
  void ResetPingStrikes();

  int TestOnlyMaxPingStrikes() const { return max_ping_strikes_; }
  Duration TestOnlyMinPingIntervalWithoutData() const {
    return min_recv_ping_interval_without_data_;
  }

  std::string GetDebugString(bool transport_idle) const;

 private:
  Duration RecvPingIntervalWithoutData(bool transport_idle) const;

  Timestamp last_ping_recv_time_ = Timestamp::InfPast();
  const Duration min_recv_ping_interval_without_data_;
  int ping_strikes_ = 0;
  const int max_ping_strikes_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_ABUSE_POLICY_H
