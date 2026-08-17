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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_RATE_POLICY_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_RATE_POLICY_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <iosfwd>
#include <string>
#include <variant>

#include "src/core/lib/channel/channel_args.h"
#include "src/core/util/time.h"

namespace grpc_core {

// How many pings do we allow to be inflight at any given time?
// In older versions of gRPC this was implicitly 1.
// With the multiping experiment we allow this to rise to 100 by default.
// TODO(ctiller): consider making this public API
#define GRPC_ARG_HTTP2_MAX_INFLIGHT_PINGS "grpc.http2.max_inflight_pings"

class Chttp2PingRatePolicy {
 public:
  explicit Chttp2PingRatePolicy(const ChannelArgs& args, bool is_client);

  static void SetDefaults(const ChannelArgs& args);

  struct SendGranted {
    bool operator==(const SendGranted&) const { return true; }
  };
  struct TooManyRecentPings {
    bool operator==(const TooManyRecentPings&) const { return true; }
  };
  struct TooSoon {
    Duration next_allowed_ping_interval;
    Timestamp last_ping;
    Duration wait;
    bool operator==(const TooSoon& other) const {
      return next_allowed_ping_interval == other.next_allowed_ping_interval &&
             last_ping == other.last_ping && wait == other.wait;
    }
  };
  using RequestSendPingResult =
      std::variant<SendGranted, TooManyRecentPings, TooSoon>;

  // Request that one ping be sent.
  // Returns:
  //  - SendGranted if a ping can be sent.
  //  - TooManyRecentPings if too many pings have been sent recently and we
  //    should wait for some future write.
  //  - TooSoon if we should wait for some time before sending the ping.
  RequestSendPingResult RequestSendPing(Duration next_allowed_ping_interval,
                                        size_t inflight_pings) const;
  // Notify the policy that one ping has been sent.
  void SentPing();
  // Notify the policy that some data has been sent and so we should no longer
  // block pings on that basis.
  void ResetPingsBeforeDataRequired();
  // Notify the policy that we've received some data.
  void ReceivedDataFrame();
  std::string GetDebugString() const;

  int TestOnlyMaxPingsWithoutData() const {
    return max_pings_without_data_sent_;
  }

 private:
  const int max_pings_without_data_sent_;
  const int max_inflight_pings_;
  int pings_before_data_sending_required_ = 0;
  Timestamp last_ping_sent_time_ = Timestamp::InfPast();
};

std::ostream& operator<<(std::ostream& out,
                         const Chttp2PingRatePolicy::RequestSendPingResult& r);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_RATE_POLICY_H
