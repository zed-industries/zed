//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_TRANSPORT_BDP_ESTIMATOR_H
#define GRPC_SRC_CORE_LIB_TRANSPORT_BDP_ESTIMATOR_H

#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <inttypes.h>

#include <string>

#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/util/time.h"

namespace grpc_core {

class BdpEstimator {
 public:
  explicit BdpEstimator(absl::string_view name);
  ~BdpEstimator() {}

  int64_t EstimateBdp() const { return estimate_; }
  double EstimateBandwidth() const { return bw_est_; }

  void AddIncomingBytes(int64_t num_bytes) { accumulator_ += num_bytes; }

  // Schedule a ping: call in response to receiving a true from
  // grpc_bdp_estimator_add_incoming_bytes once a ping has been scheduled by a
  // transport (but not necessarily started)
  void SchedulePing() {
    GRPC_TRACE_LOG(bdp_estimator, INFO)
        << "bdp[" << name_ << "]:sched acc=" << accumulator_
        << " est=" << estimate_;
    CHECK(ping_state_ == PingState::UNSCHEDULED);
    ping_state_ = PingState::SCHEDULED;
    accumulator_ = 0;
  }

  // Start a ping: call after calling grpc_bdp_estimator_schedule_ping and
  // once
  // the ping is on the wire
  void StartPing() {
    GRPC_TRACE_LOG(bdp_estimator, INFO)
        << "bdp[" << name_ << "]:start acc=" << accumulator_
        << " est=" << estimate_;
    CHECK(ping_state_ == PingState::SCHEDULED);
    ping_state_ = PingState::STARTED;
    ping_start_time_ = gpr_now(GPR_CLOCK_MONOTONIC);
  }

  // Completes a previously started ping, returns when to schedule the next one
  Timestamp CompletePing();

  int64_t accumulator() const { return accumulator_; }

 private:
  enum class PingState { UNSCHEDULED, SCHEDULED, STARTED };

  int64_t accumulator_;
  int64_t estimate_;
  // when was the current ping started?
  gpr_timespec ping_start_time_;
  Duration inter_ping_delay_;
  int stable_estimate_count_;
  PingState ping_state_;
  double bw_est_;
  absl::string_view name_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_TRANSPORT_BDP_ESTIMATOR_H
