//
//
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
//
//

#ifndef GRPC_TEST_CORE_END2END_FUZZERS_FUZZING_COMMON_H
#define GRPC_TEST_CORE_END2END_FUZZERS_FUZZING_COMMON_H

#include <grpc/grpc.h>
#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <functional>
#include <memory>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "absl/types/span.h"
#include "src/core/lib/event_engine/default_event_engine.h"
#include "src/core/lib/resource_quota/resource_quota.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"
#include "test/core/end2end/fuzzers/api_fuzzer.pb.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.pb.h"

namespace grpc_core {
namespace testing {

class Validator {
 public:
  explicit Validator(std::function<void(bool)> impl) : impl_(std::move(impl)) {}

  virtual ~Validator() {}
  void Run(bool success) {
    impl_(success);
    delete this;
  }

 private:
  std::function<void(bool)> impl_;
};

inline Validator* MakeValidator(std::function<void(bool)> impl) {
  return new Validator(std::move(impl));
}

inline Validator* AssertSuccessAndDecrement(int* counter) {
  return MakeValidator([counter](bool success) {
    CHECK(success);
    --*counter;
  });
}

inline Validator* Decrement(int* counter) {
  return MakeValidator([counter](bool) { --*counter; });
}

class Call;

class BasicFuzzer {
 public:
  explicit BasicFuzzer(const fuzzing_event_engine::Actions& actions);

  enum Result { kPending = 0, kComplete = 1, kFailed = 2, kNotSupported = 3 };
  virtual Result ExecuteAction(const api_fuzzer::Action& action);
  Call* ActiveCall();

  bool Continue();
  virtual void Tick();

  void Run(absl::Span<const api_fuzzer::Action* const> actions);

 protected:
  ~BasicFuzzer();

  bool server_finished_shutting_down() {
    return server() != nullptr && server_shutdown_ &&
           pending_server_shutdowns_ == 0;
  }
  bool server_shutdown_called() { return server_shutdown_; }

  void ShutdownCalls();
  void ResetServerState() {
    server_shutdown_ = false;
    CHECK_EQ(pending_server_shutdowns_, 0);
  }

  // Poll any created completion queue to drive the RPC forward.
  Result PollCq();

  // Shutdown the active server.
  Result ShutdownServer();

  RefCountedPtr<ResourceQuota> resource_quota() { return resource_quota_; }

  std::shared_ptr<grpc_event_engine::experimental::FuzzingEventEngine>
  engine() {
    return engine_;
  }

  grpc_completion_queue* cq() { return cq_; }

  void UpdateMinimumRunTime(Duration minimum_run_time) {
    minimum_run_time_ = std::max(minimum_run_time, minimum_run_time_);
  }

 private:
  // Channel specific actions.
  // Create an active channel with the specified parameters.
  virtual Result CreateChannel(
      const api_fuzzer::CreateChannel& create_channel) = 0;

  // Close the active channel.
  Result CloseChannel();
  // Check whether the channel is connected and optionally try to connect if it
  // is not connected.
  Result CheckConnectivity(bool try_to_connect);
  // Watch whether the channel connects within the specified duration.
  Result WatchConnectivity(uint32_t duration_us);
  // Verify that the channel target can be reliably queried.
  Result ValidateChannelTarget();

  // Server specific actions
  // Create an active server.
  virtual Result CreateServer(
      const api_fuzzer::CreateServer& create_server) = 0;
  // Destroy the active server.
  Result DestroyServerIfReady();

  // Request to be notified of a new RPC on the active server.
  Result ServerRequestCall();
  // Cancel all server calls.
  Result CancelAllCallsIfShutdown();

  // Call specific actions.
  // Create a call on the active channel with the specified parameters. Also add
  // it the list of managed calls.
  Result CreateCall(const api_fuzzer::CreateCall& create_call);
  // Choose a different active call from the list of managed calls.
  Result ChangeActiveCall();
  // Queue a batch of operations to be executed on the active call.
  Result QueueBatchForActiveCall(const api_fuzzer::Batch& queue_batch);
  // Cancel the active call.
  Result CancelActiveCall();
  // Validate that the peer can be reliably queried for the active call.
  Result ValidatePeerForActiveCall();
  // Cancel and destroy the active call.
  Result DestroyActiveCall();
  // Pause the run loop for some time
  Result Pause(Duration duration);

  // Other actions.
  // Change the resource quota limits.
  Result ResizeResourceQuota(uint32_t resize_resource_quota);

  void TryShutdown();

  virtual grpc_server* server() = 0;
  virtual grpc_channel* channel() = 0;
  virtual void DestroyServer() = 0;
  virtual void DestroyChannel() = 0;

  std::shared_ptr<grpc_event_engine::experimental::FuzzingEventEngine> engine_;
  grpc_completion_queue* cq_;
  bool server_shutdown_ = false;
  int pending_server_shutdowns_ = 0;
  int pending_channel_watches_ = 0;
  int paused_ = 0;
  std::vector<std::shared_ptr<Call>> calls_;
  RefCountedPtr<ResourceQuota> resource_quota_;
  size_t active_call_ = 0;
  Duration minimum_run_time_ = Duration::Zero();
};

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_END2END_FUZZERS_FUZZING_COMMON_H
