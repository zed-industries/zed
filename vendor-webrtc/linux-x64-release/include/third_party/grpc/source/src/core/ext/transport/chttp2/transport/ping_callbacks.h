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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_CALLBACKS_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_CALLBACKS_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <optional>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/functional/any_invocable.h"
#include "absl/hash/hash.h"
#include "absl/random/bit_gen_ref.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/util/time.h"

namespace grpc_core {

class Chttp2PingCallbacks {
 public:
  // One callback from OnPing/OnPingAck or the timeout.
  using Callback = absl::AnyInvocable<void()>;

  // Request a ping (but one we don't need any notification for when it begins
  // or ends).
  void RequestPing() { ping_requested_ = true; }

  // Request a ping, and specify callbacks for when it begins and ends.
  // on_start is invoked during the call to StartPing.
  // on_ack is invoked during the call to AckPing.
  void OnPing(Callback on_start, Callback on_ack);

  // Request a notification when *some* ping is acked:
  // If there is no ping in flight, one will be scheduled and the callback
  // will be invoked when it is acked. (ie as per OnPing([]{}, on_ack)).
  // If there is a ping in flight, the callback will be invoked when the most
  // recently sent ping is acked.
  // on_ack is invoked during the call to AckPing.
  void OnPingAck(Callback on_ack);

  // Write path: begin a ping.
  // Uses bitgen to generate a randomized id for the ping.
  // Sets started_new_ping_without_setting_timeout.
  GRPC_MUST_USE_RESULT uint64_t StartPing(absl::BitGenRef bitgen);
  bool AckPing(uint64_t id,
               grpc_event_engine::experimental::EventEngine* event_engine);

  // Cancel all the ping callbacks.
  // Sufficient state is maintained such that AckPing will still return true
  // if a ping is acked after this call.
  // No timeouts or start or ack callbacks previously scheduled will be invoked.
  void CancelAll(grpc_event_engine::experimental::EventEngine* event_engine);

  // Return true if a ping needs to be started due to
  // RequestPing/OnPing/OnPingAck.
  bool ping_requested() const { return ping_requested_; }

  // Returns the number of pings currently in flight.
  size_t pings_inflight() const { return inflight_.size(); }

  // Returns true if a ping was started without setting a timeout yet.
  bool started_new_ping_without_setting_timeout() const {
    return started_new_ping_without_setting_timeout_;
  }

  // Add a ping timeout for the most recently started ping.
  // started_new_ping_without_setting_timeout must be set.
  // Clears started_new_ping_without_setting_timeout.
  // Returns the ping id of the ping the timeout was attached to if a timer was
  // started, or nullopt otherwise.
  std::optional<uint64_t> OnPingTimeout(
      Duration ping_timeout,
      grpc_event_engine::experimental::EventEngine* event_engine,
      Callback callback);

 private:
  using CallbackVec = std::vector<Callback>;
  struct InflightPing {
    grpc_event_engine::experimental::EventEngine::TaskHandle on_timeout =
        grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid;
    CallbackVec on_ack;
  };
  absl::flat_hash_map<uint64_t, InflightPing> inflight_;
  uint64_t most_recent_inflight_ = 0;
  bool ping_requested_ = false;
  bool started_new_ping_without_setting_timeout_ = false;
  CallbackVec on_start_;
  CallbackVec on_ack_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_CALLBACKS_H
