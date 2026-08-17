//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FLOW_CONTROL_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FLOW_CONTROL_H

#include <grpc/support/port_platform.h>
#include <limits.h>
#include <stdint.h>

#include <iosfwd>
#include <optional>
#include <string>
#include <utility>

#include "absl/functional/function_ref.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/ext/transport/chttp2/transport/http2_settings.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/lib/transport/bdp_estimator.h"
#include "src/core/util/time.h"

namespace grpc {
namespace testing {
class TrickledCHTTP2;  // to make this a friend
}  // namespace testing
}  // namespace grpc

namespace grpc_core {
namespace chttp2 {

static constexpr uint32_t kDefaultWindow = 65535;
static constexpr uint32_t kDefaultFrameSize = 16384;
static constexpr int64_t kMaxWindow = static_cast<int64_t>((1u << 31) - 1);
// If smaller than this, advertise zero window.
static constexpr uint32_t kMinPositiveInitialWindowSize = 1024;
static constexpr const uint32_t kMaxInitialWindowSize = (1u << 30);
// The maximum per-stream flow control window delta to advertise.
static constexpr const int64_t kMaxWindowDelta = (1u << 20);
static constexpr const int kDefaultPreferredRxCryptoFrameSize = INT_MAX;

// TODO(ctiller): clean up when flow_control_fixes is enabled by default
static constexpr uint32_t kFrameSize = 1024 * 1024;
static constexpr const uint32_t kMinInitialWindowSize = 128;

class TransportFlowControl;
class StreamFlowControl;

enum class StallEdge { kNoChange, kStalled, kUnstalled };

// Encapsulates a collections of actions the transport needs to take with
// regard to flow control. Each action comes with urgencies that tell the
// transport how quickly the action must take place.
class GRPC_MUST_USE_RESULT FlowControlAction {
 public:
  enum class Urgency : uint8_t {
    // Nothing to be done.
    NO_ACTION_NEEDED = 0,
    // Initiate a write to update the initial window immediately.
    UPDATE_IMMEDIATELY,
    // Push the flow control update into a send buffer, to be sent
    // out the next time a write is initiated.
    QUEUE_UPDATE,
  };

  Urgency send_stream_update() const { return send_stream_update_; }
  Urgency send_transport_update() const { return send_transport_update_; }
  Urgency send_initial_window_update() const {
    return send_initial_window_update_;
  }
  Urgency send_max_frame_size_update() const {
    return send_max_frame_size_update_;
  }
  Urgency preferred_rx_crypto_frame_size_update() const {
    return preferred_rx_crypto_frame_size_update_;
  }
  uint32_t initial_window_size() const { return initial_window_size_; }
  uint32_t max_frame_size() const { return max_frame_size_; }
  uint32_t preferred_rx_crypto_frame_size() const {
    return preferred_rx_crypto_frame_size_;
  }

  FlowControlAction& set_send_stream_update(Urgency u) {
    send_stream_update_ = u;
    return *this;
  }
  FlowControlAction& set_send_transport_update(Urgency u) {
    send_transport_update_ = u;
    return *this;
  }
  FlowControlAction& set_send_initial_window_update(Urgency u,
                                                    uint32_t update) {
    send_initial_window_update_ = u;
    initial_window_size_ = update;
    return *this;
  }
  FlowControlAction& set_send_max_frame_size_update(Urgency u,
                                                    uint32_t update) {
    send_max_frame_size_update_ = u;
    max_frame_size_ = update;
    return *this;
  }
  FlowControlAction& set_preferred_rx_crypto_frame_size_update(
      Urgency u, uint32_t update) {
    preferred_rx_crypto_frame_size_update_ = u;
    preferred_rx_crypto_frame_size_ = update;
    return *this;
  }

  static const char* UrgencyString(Urgency u);
  std::string DebugString() const;

  void AssertEmpty() { CHECK(*this == FlowControlAction()); }

  bool operator==(const FlowControlAction& other) const {
    return send_stream_update_ == other.send_stream_update_ &&
           send_transport_update_ == other.send_transport_update_ &&
           send_initial_window_update_ == other.send_initial_window_update_ &&
           send_max_frame_size_update_ == other.send_max_frame_size_update_ &&
           (send_initial_window_update_ == Urgency::NO_ACTION_NEEDED ||
            initial_window_size_ == other.initial_window_size_) &&
           (send_max_frame_size_update_ == Urgency::NO_ACTION_NEEDED ||
            max_frame_size_ == other.max_frame_size_) &&
           (preferred_rx_crypto_frame_size_update_ ==
                Urgency::NO_ACTION_NEEDED ||
            preferred_rx_crypto_frame_size_ ==
                other.preferred_rx_crypto_frame_size_);
  }

 private:
  Urgency send_stream_update_ = Urgency::NO_ACTION_NEEDED;
  Urgency send_transport_update_ = Urgency::NO_ACTION_NEEDED;
  Urgency send_initial_window_update_ = Urgency::NO_ACTION_NEEDED;
  Urgency send_max_frame_size_update_ = Urgency::NO_ACTION_NEEDED;
  Urgency preferred_rx_crypto_frame_size_update_ = Urgency::NO_ACTION_NEEDED;
  uint32_t initial_window_size_ = 0;
  uint32_t max_frame_size_ = 0;
  uint32_t preferred_rx_crypto_frame_size_ = 0;
};

std::ostream& operator<<(std::ostream& out, FlowControlAction::Urgency urgency);
std::ostream& operator<<(std::ostream& out, const FlowControlAction& action);

// Implementation of flow control that abides to HTTP/2 spec and attempts
// to be as performant as possible.
class TransportFlowControl final {
 public:
  explicit TransportFlowControl(absl::string_view name, bool enable_bdp_probe,
                                MemoryOwner* memory_owner);
  ~TransportFlowControl() {}

  bool bdp_probe() const { return enable_bdp_probe_; }

  // returns an announce if we should send a transport update to our peer,
  // else returns zero; writing_anyway indicates if a write would happen
  // regardless of the send - if it is false and this function returns non-zero,
  // this announce will cause a write to occur
  uint32_t DesiredAnnounceSize(bool writing_anyway) const;
  // notify that we've actually sent a stream window update
  // (should be DesiredAnnounceSize())
  void SentUpdate(uint32_t announce);

  // Older API: combines getting the DesiredAnnounceSize() with SentUpdate()
  uint32_t MaybeSendUpdate(bool writing_anyway) {
    uint32_t n = DesiredAnnounceSize(writing_anyway);
    SentUpdate(n);
    return n;
  }

  // Track an update to the incoming flow control counters - that is how many
  // tokens we report to our peer that we're willing to accept.
  // Instantiators *must* call MakeAction before destruction of this value.
  class IncomingUpdateContext {
   public:
    explicit IncomingUpdateContext(TransportFlowControl* tfc) : tfc_(tfc) {}
    ~IncomingUpdateContext() { CHECK_EQ(tfc_, nullptr); }

    IncomingUpdateContext(const IncomingUpdateContext&) = delete;
    IncomingUpdateContext& operator=(const IncomingUpdateContext&) = delete;

    // Reads the flow control data and returns an actionable struct that will
    // tell chttp2 exactly what it needs to do
    FlowControlAction MakeAction() {
      return std::exchange(tfc_, nullptr)->UpdateAction(FlowControlAction());
    }

    // Notify of data receipt. Returns OkStatus if the data was accepted,
    // else an error status if the connection should be closed.
    absl::Status RecvData(
        int64_t incoming_frame_size, absl::FunctionRef<absl::Status()> stream =
                                         []() { return absl::OkStatus(); });

    // Update a stream announce window delta, keeping track of how much total
    // positive delta is present on the transport.
    void UpdateAnnouncedWindowDelta(int64_t* delta, int64_t change) {
      if (change == 0) return;
      if (*delta > 0) {
        tfc_->announced_stream_total_over_incoming_window_ -= *delta;
      }
      *delta += change;
      if (*delta > 0) {
        tfc_->announced_stream_total_over_incoming_window_ += *delta;
      }
    }

   private:
    TransportFlowControl* tfc_;
  };

  // Track an update to the outgoing flow control counters - that is how many
  // tokens our peer has said we can send.
  class OutgoingUpdateContext {
   public:
    explicit OutgoingUpdateContext(TransportFlowControl* tfc) : tfc_(tfc) {}
    void StreamSentData(int64_t size) { tfc_->remote_window_ -= size; }

    // we have received a WINDOW_UPDATE frame for a transport
    void RecvUpdate(uint32_t size) { tfc_->remote_window_ += size; }

    // Finish the update and check whether we became stalled or unstalled.
    StallEdge Finish() {
      bool is_stalled = tfc_->remote_window_ <= 0;
      if (is_stalled != was_stalled_) {
        return is_stalled ? StallEdge::kStalled : StallEdge::kUnstalled;
      } else {
        return StallEdge::kNoChange;
      }
    }

   private:
    TransportFlowControl* tfc_;
    const bool was_stalled_ = tfc_->remote_window_ <= 0;
  };

  // Call periodically (at a low-ish rate, 100ms - 10s makes sense)
  // to perform more complex flow control calculations and return an action
  // to let chttp2 change its parameters
  FlowControlAction PeriodicUpdate();

  int64_t target_window() const;
  int64_t target_frame_size() const { return target_frame_size_; }
  int64_t target_preferred_rx_crypto_frame_size() const {
    return target_preferred_rx_crypto_frame_size_;
  }

  BdpEstimator* bdp_estimator() { return &bdp_estimator_; }

  uint32_t acked_init_window() const { return acked_init_window_; }
  uint32_t queued_init_window() const { return target_initial_window_size_; }
  uint32_t sent_init_window() const { return sent_init_window_; }

  void FlushedSettings() { sent_init_window_ = queued_init_window(); }

  FlowControlAction SetAckedInitialWindow(uint32_t value);

  // Getters
  int64_t remote_window() const { return remote_window_; }
  int64_t announced_window() const { return announced_window_; }

  int64_t announced_stream_total_over_incoming_window() const {
    return announced_stream_total_over_incoming_window_;
  }

  void RemoveAnnouncedWindowDelta(int64_t delta) {
    if (delta > 0) {
      announced_stream_total_over_incoming_window_ -= delta;
    }
  }

  // A snapshot of the flow control stats to export.
  struct Stats {
    int64_t target_window;
    int64_t target_frame_size;
    int64_t target_preferred_rx_crypto_frame_size;
    uint32_t acked_init_window;
    uint32_t queued_init_window;
    uint32_t sent_init_window;
    int64_t remote_window;
    int64_t announced_window;
    int64_t announced_stream_total_over_incoming_window;
    // BDP estimator stats.
    int64_t bdp_accumulator;
    int64_t bdp_estimate;
    double bdp_bw_est;

    std::string ToString() const;
    Json::Object ToJsonObject() {
      Json::Object object;
      object["targetWindow"] = Json::FromNumber(target_window);
      object["targetFrameSize"] = Json::FromNumber(target_frame_size);
      object["targetPreferredRxCryptoFrameSize"] =
          Json::FromNumber(target_preferred_rx_crypto_frame_size);
      object["ackedInitWindow"] = Json::FromNumber(acked_init_window);
      object["queuedInitWindow"] = Json::FromNumber(queued_init_window);
      object["sentInitWindow"] = Json::FromNumber(sent_init_window);
      object["remoteWindow"] = Json::FromNumber(remote_window);
      object["announcedWindow"] = Json::FromNumber(announced_window);
      object["announcedStreamTotalOverIncomingWindow"] =
          Json::FromNumber(announced_stream_total_over_incoming_window);
      object["bdpAccumulator"] = Json::FromNumber(bdp_accumulator);
      object["bdpEstimate"] = Json::FromNumber(bdp_estimate);
      object["bdpBwEst"] = Json::FromNumber(bdp_bw_est);
      return object;
    }
  };

  Stats stats() const {
    Stats stats;
    stats.target_window = target_window();
    stats.target_frame_size = target_frame_size();
    stats.target_preferred_rx_crypto_frame_size =
        target_preferred_rx_crypto_frame_size();
    stats.acked_init_window = acked_init_window();
    stats.queued_init_window = queued_init_window();
    stats.sent_init_window = sent_init_window();
    stats.remote_window = remote_window();
    stats.announced_window = announced_window();
    stats.announced_stream_total_over_incoming_window =
        announced_stream_total_over_incoming_window();
    stats.bdp_accumulator = bdp_estimator_.accumulator();
    stats.bdp_estimate = bdp_estimator_.EstimateBdp();
    stats.bdp_bw_est = bdp_estimator_.EstimateBandwidth();
    return stats;
  }

 private:
  double TargetInitialWindowSizeBasedOnMemoryPressureAndBdp() const;
  static void UpdateSetting(absl::string_view name, int64_t* desired_value,
                            uint32_t new_desired_value,
                            FlowControlAction* action,
                            FlowControlAction& (FlowControlAction::*set)(
                                FlowControlAction::Urgency, uint32_t));

  FlowControlAction UpdateAction(FlowControlAction action);

  MemoryOwner* const memory_owner_;

  /// calculating what we should give for local window:
  /// we track the total amount of flow control over initial window size
  /// across all streams: this is data that we want to receive right now (it
  /// has an outstanding read)
  /// and the total amount of flow control under initial window size across all
  /// streams: this is data we've read early
  /// we want to adjust incoming_window such that:
  /// incoming_window = total_over - max(bdp - total_under, 0)
  int64_t announced_stream_total_over_incoming_window_ = 0;

  /// should we probe bdp?
  const bool enable_bdp_probe_;

  // bdp estimation
  BdpEstimator bdp_estimator_;

  int64_t remote_window_ = kDefaultWindow;
  int64_t target_initial_window_size_ = kDefaultWindow;
  int64_t target_frame_size_ = kDefaultFrameSize;
  int64_t target_preferred_rx_crypto_frame_size_ =
      kDefaultPreferredRxCryptoFrameSize;
  int64_t announced_window_ = kDefaultWindow;
  uint32_t acked_init_window_ = kDefaultWindow;
  uint32_t sent_init_window_ = kDefaultWindow;
};

// Implementation of flow control that abides to HTTP/2 spec and attempts
// to be as performant as possible.
class StreamFlowControl final {
 public:
  explicit StreamFlowControl(TransportFlowControl* tfc);
  ~StreamFlowControl() {
    tfc_->RemoveAnnouncedWindowDelta(announced_window_delta_);
  }

  // Track an update to the incoming flow control counters - that is how many
  // tokens we report to our peer that we're willing to accept.
  // Instantiators *must* call MakeAction before destruction of this value.
  class IncomingUpdateContext {
   public:
    explicit IncomingUpdateContext(StreamFlowControl* sfc)
        : tfc_upd_(sfc->tfc_), sfc_(sfc) {}

    FlowControlAction MakeAction() {
      return sfc_->UpdateAction(tfc_upd_.MakeAction());
    }

    // we have received data from the wire
    absl::Status RecvData(int64_t incoming_frame_size);

    // the application is asking for a certain amount of bytes
    void SetMinProgressSize(int64_t min_progress_size) {
      sfc_->min_progress_size_ = min_progress_size;
    }

    void SetPendingSize(int64_t pending_size);

   private:
    TransportFlowControl::IncomingUpdateContext tfc_upd_;
    StreamFlowControl* const sfc_;
  };

  // Track an update to the outgoing flow control counters - that is how many
  // tokens our peer has said we can send.
  class OutgoingUpdateContext {
   public:
    explicit OutgoingUpdateContext(StreamFlowControl* sfc)
        : tfc_upd_(sfc->tfc_), sfc_(sfc) {}
    // we have received a WINDOW_UPDATE frame for a stream
    void RecvUpdate(uint32_t size) { sfc_->remote_window_delta_ += size; }
    // we have sent data on the wire, we must track this in our bookkeeping for
    // the remote peer's flow control.
    void SentData(int64_t outgoing_frame_size) {
      tfc_upd_.StreamSentData(outgoing_frame_size);
      sfc_->remote_window_delta_ -= outgoing_frame_size;
    }

   private:
    TransportFlowControl::OutgoingUpdateContext tfc_upd_;
    StreamFlowControl* const sfc_;
  };

  // returns an announce if we should send a stream update to our peer, else
  // returns zero
  uint32_t DesiredAnnounceSize() const;
  // notify that we've actually sent a stream window update
  // (should be DesiredAnnounceSize())
  void SentUpdate(uint32_t announce);

  // Older API: combines getting the DesiredAnnounceSize() with SentUpdate()
  uint32_t MaybeSendUpdate() {
    uint32_t n = DesiredAnnounceSize();
    SentUpdate(n);
    return n;
  }

  int64_t remote_window_delta() const { return remote_window_delta_; }
  int64_t announced_window_delta() const { return announced_window_delta_; }
  int64_t min_progress_size() const { return min_progress_size_; }

  // A snapshot of the flow control stats to export.
  struct Stats {
    int64_t min_progress_size;
    int64_t remote_window_delta;
    int64_t announced_window_delta;
    std::optional<int64_t> pending_size;

    std::string ToString() const;
  };

  Stats stats() const {
    Stats stats;
    stats.min_progress_size = min_progress_size();
    stats.remote_window_delta = remote_window_delta();
    stats.announced_window_delta = announced_window_delta();
    stats.pending_size = pending_size_;
    return stats;
  }

 private:
  TransportFlowControl* const tfc_;
  int64_t min_progress_size_ = 0;
  int64_t remote_window_delta_ = 0;
  int64_t announced_window_delta_ = 0;
  std::optional<int64_t> pending_size_;

  FlowControlAction UpdateAction(FlowControlAction action);
};

class TestOnlyTransportTargetWindowEstimatesMocker {
 public:
  virtual ~TestOnlyTransportTargetWindowEstimatesMocker() {}
  virtual double ComputeNextTargetInitialWindowSizeFromPeriodicUpdate(
      double current_target) = 0;
};

extern TestOnlyTransportTargetWindowEstimatesMocker*
    g_test_only_transport_target_window_estimates_mocker;

}  // namespace chttp2
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_FLOW_CONTROL_H
