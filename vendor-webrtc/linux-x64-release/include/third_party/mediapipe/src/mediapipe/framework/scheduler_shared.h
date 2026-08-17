// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_SCHEDULER_SHARED_H_
#define MEDIAPIPE_FRAMEWORK_SCHEDULER_SHARED_H_

#include <atomic>
#include <cstdint>
#include <functional>
#include <memory>
#include <queue>
#include <utility>

#include "absl/base/macros.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/deps/clock.h"
#include "mediapipe/framework/deps/monotonic_clock.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {
namespace internal {

// This is meant for testing purposes only.
struct SchedulerTimes {
  // Total run time measured by the scheduler, in microseconds.
  int64_t total_time;
  // Total time spent running nodes, in microseconds.
  int64_t node_time;
  // The fraction of total time which was not spent running nodes. Only valid
  // when the graph is run on a single thread.
  double overhead() const {
    return 1.0 -
           (static_cast<double>(node_time) / static_cast<double>(total_time));
  }
};

// This service class is used to compute the scheduler overhead.
// It keeps track of the total runtime of the graph, and of the total time
// spent actually running nodes. The difference is considered overhead.
// This is a crude measure that only makes sense when running with a single
// worker thread, but it has the advantage of being simple and continuing to
// work across the scheduler changes we want to test.
class SchedulerTimer {
 public:
  SchedulerTimer() {
    clock_ = std::unique_ptr<mediapipe::Clock>(
        mediapipe::MonotonicClock::CreateSynchronizedMonotonicClock());
  }

  // Called when starting the scheduler.
  void StartRun() {
    start_time_ = absl::ToUnixMicros(clock_->TimeNow());
    total_node_time_ = 0;
  }
  // Called when terminating the scheduler.
  void EndRun() {
    total_run_time_ = absl::ToUnixMicros(clock_->TimeNow()) - start_time_;
  }

  // Called immediately before invoking ProcessNode or CloseNode.
  int64_t StartNode() { return absl::ToUnixMicros(clock_->TimeNow()); }
  // Called immediately after invoking ProcessNode or CloseNode.
  void EndNode(int64_t node_start_time) {
    total_node_time_.fetch_add(
        absl::ToUnixMicros(clock_->TimeNow()) - node_start_time,
        std::memory_order_relaxed);
  }

  SchedulerTimes GetSchedulerTimes() {
    internal::SchedulerTimes result;
    result.total_time = total_run_time_;
    result.node_time = total_node_time_;
    return result;
  }

 private:
  // Timer for measuring overhead.
  std::unique_ptr<mediapipe::Clock> clock_;

  // Time spent actually running nodes, in microseconds.
  std::atomic<int64_t> total_node_time_;

  // The start time of the graph, in microseconds.
  int64_t start_time_;
  // Total time spent running the graph, in microseconds.
  int64_t total_run_time_;
};

struct SchedulerShared {
  // When a non-source node returns StatusStop() or
  // CalculatorGraph::CloseAllPacketSources is called, the graph starts to
  // terminate: all source nodes are closed (at the next scheduling
  // opportunity), and the graph continues running until it is done. This
  // flag indicates that the graph is in that mode.
  std::atomic<bool> stopping;
  std::atomic<bool> has_error;
  std::function<void(const absl::Status& error)> error_callback;
  // Collects timing information for measuring overhead.
  internal::SchedulerTimer timer;
};

}  // namespace internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_SCHEDULER_SHARED_H_
