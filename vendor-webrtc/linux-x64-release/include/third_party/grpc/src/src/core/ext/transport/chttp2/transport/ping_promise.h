//
//
// Copyright 2025 gRPC authors.
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
//
//
#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_PROMISE_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_PROMISE_H

#include <memory>

#include "src/core/ext/transport/chttp2/transport/ping_abuse_policy.h"
#include "src/core/ext/transport/chttp2/transport/ping_callbacks.h"
#include "src/core/ext/transport/chttp2/transport/ping_rate_policy.h"
#include "src/core/lib/promise/inter_activity_latch.h"
#include "src/core/lib/promise/map.h"
#include "src/core/lib/promise/promise.h"
#include "src/core/util/shared_bit_gen.h"
#include "src/core/util/time.h"

namespace grpc_core {
namespace http2 {
class PingInterface {
 public:
  struct SendPingArgs {
    bool ack = false;
    // RFC9113: PING frames MUST contain 8 octets of opaque data in the frame
    // payload. A sender can include any value it chooses and use those octets
    // in any fashion.
    uint64_t opaque_data = 0;
  };
  // TODO(tjagtap) : [PH2][P1] Change the return type of the promises to
  // Promise<Http2Status> type when that is submitted.

  // Returns a promise that creates and sends a ping frame to the peer.
  virtual Promise<absl::Status> SendPing(SendPingArgs args) = 0;

  // Returns a promise that triggers a write cycle on the transport.
  virtual Promise<absl::Status> TriggerWrite() = 0;

  // Returns a promise that handles the ping timeout.
  virtual Promise<absl::Status> PingTimeout() = 0;
  virtual ~PingInterface() = default;
};

// The code in this class is NOT thread safe. It has been designed to run on a
// single thread. This guarantee is achieved by spawning all the promises
// returned by this class on the same transport party.
class PingManager {
 public:
  PingManager(const ChannelArgs& channel_args,
              std::unique_ptr<PingInterface> ping_interface,
              std::shared_ptr<grpc_event_engine::experimental::EventEngine>
                  event_engine);

  // Returns a promise that determines if a ping frame should be sent to the
  // peer. If a ping frame is sent, it also spawns a timeout promise that
  // handles the ping timeout.
  Promise<absl::Status> MaybeSendPing(Duration next_allowed_ping_interval,
                                      Duration ping_timeout);

  // Ping Rate policy wrapper
  void ReceivedDataFrame() { ping_rate_policy_.ReceivedDataFrame(); }

  // Ping abuse policy wrapper
  bool NotifyPingAbusePolicy(const bool transport_idle) {
    return ping_abuse_policy_.ReceivedOnePing(transport_idle);
  }

  void ResetPingClock(bool is_client) {
    if (!is_client) {
      ping_abuse_policy_.ResetPingStrikes();
    }
    ping_rate_policy_.ResetPingsBeforeDataRequired();
  }

  // Ping callbacks wrapper

  // Returns a promise that resolves once a new ping is initiated and ack is
  // received for the same. The on_initiate callback is executed when the
  // ping is initiated.
  auto RequestPing(absl::AnyInvocable<void()> on_initiate) {
    return ping_callbacks_.RequestPing(std::move(on_initiate));
  }

  // Returns a promise that resolves once the next valid ping ack is received.
  auto WaitForPingAck() { return ping_callbacks_.WaitForPingAck(); }

  // Cancels all the callbacks for the inflight pings. This function does not
  // cancel the promises that are waiting on the ping ack.
  // This should be called as part of closing the transport to free up any
  // memory in use by the ping callbacks.
  void CancelCallbacks() { ping_callbacks_.CancelCallbacks(); }

  uint64_t StartPing() { return ping_callbacks_.StartPing(); }
  bool PingRequested() { return ping_callbacks_.PingRequested(); }
  bool AckPing(uint64_t id) { return ping_callbacks_.AckPing(id); }
  size_t CountPingInflight() { return ping_callbacks_.CountPingInflight(); }

 private:
  class PingPromiseCallbacks {
   public:
    explicit PingPromiseCallbacks(
        std::shared_ptr<grpc_event_engine::experimental::EventEngine>
            event_engine)
        : event_engine_(event_engine) {}
    Promise<absl::Status> RequestPing(absl::AnyInvocable<void()> on_initiate);
    Promise<absl::Status> WaitForPingAck();
    void CancelCallbacks() { ping_callbacks_.CancelAll(event_engine_.get()); }
    uint64_t StartPing() { return ping_callbacks_.StartPing(SharedBitGen()); }
    bool PingRequested() { return ping_callbacks_.ping_requested(); }
    bool AckPing(uint64_t id) {
      return ping_callbacks_.AckPing(id, event_engine_.get());
    }
    size_t CountPingInflight() { return ping_callbacks_.pings_inflight(); }

    auto PingTimeout(Duration ping_timeout) {
      std::shared_ptr<InterActivityLatch<void>> latch =
          std::make_shared<InterActivityLatch<void>>();
      auto timeout_cb = [latch]() { latch->Set(); };
      auto id = ping_callbacks_.OnPingTimeout(ping_timeout, event_engine_.get(),
                                              std::move(timeout_cb));
      DCHECK(id.has_value());
      VLOG(2) << "Ping timeout of duration: " << ping_timeout
              << " initiated for ping id: " << *id;
      return Map(latch->Wait(), [latch](Empty) { return absl::OkStatus(); });
    }

   private:
    Chttp2PingCallbacks ping_callbacks_;
    std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine_;
  };

  PingPromiseCallbacks ping_callbacks_;
  Chttp2PingAbusePolicy ping_abuse_policy_;
  Chttp2PingRatePolicy ping_rate_policy_;
  bool delayed_ping_spawned_ = false;
  std::unique_ptr<PingInterface> ping_interface_;

  void TriggerDelayedPing(Duration wait);
  bool NeedToPing(Duration next_allowed_ping_interval);
  void SpawnTimeout(Duration ping_timeout, uint64_t opaque_data);

  void SentPing() { ping_rate_policy_.SentPing(); }
};
}  // namespace http2
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_PING_PROMISE_H
