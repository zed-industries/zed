//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_RESOLVER_POLLING_RESOLVER_H
#define GRPC_SRC_CORE_RESOLVER_POLLING_RESOLVER_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <memory>
#include <optional>
#include <string>

#include "absl/status/status.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/resolver/resolver.h"
#include "src/core/resolver/resolver_factory.h"
#include "src/core/util/backoff.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/time.h"
#include "src/core/util/work_serializer.h"

namespace grpc_core {

// A base class for polling-based resolvers.
// Handles cooldown and backoff timers.
// Implementations need only to implement StartRequest().
class PollingResolver : public Resolver {
 public:
  PollingResolver(ResolverArgs args, Duration min_time_between_resolutions,
                  BackOff::Options backoff_options, TraceFlag* tracer);
  ~PollingResolver() override;

  void StartLocked() override;
  void RequestReresolutionLocked() override;
  void ResetBackoffLocked() override;
  void ShutdownLocked() override;

 protected:
  // Implemented by subclass.
  // Starts a request, returning an object representing the pending
  // request.  Orphaning that object should cancel the request.
  // When the request is complete, the implementation must call
  // OnRequestComplete() with the result.
  virtual OrphanablePtr<Orphanable> StartRequest() = 0;

  // To be invoked by the subclass when a request is complete.
  void OnRequestComplete(Result result);

  // Convenient accessor methods for subclasses.
  const std::string& authority() const { return authority_; }
  const std::string& name_to_resolve() const { return name_to_resolve_; }
  grpc_pollset_set* interested_parties() const { return interested_parties_; }
  const ChannelArgs& channel_args() const { return channel_args_; }
  WorkSerializer* work_serializer() { return work_serializer_.get(); }

 private:
  void MaybeStartResolvingLocked();
  void StartResolvingLocked();

  void OnRequestCompleteLocked(Result result);
  void GetResultStatus(absl::Status status);

  void ScheduleNextResolutionTimer(Duration delay);
  void OnNextResolutionLocked();
  void MaybeCancelNextResolutionTimer();

  /// authority
  std::string authority_;
  /// name to resolve
  std::string name_to_resolve_;
  /// channel args
  ChannelArgs channel_args_;
  std::shared_ptr<WorkSerializer> work_serializer_;
  std::unique_ptr<ResultHandler> result_handler_;
  TraceFlag* tracer_;
  /// pollset_set to drive the name resolution process
  grpc_pollset_set* interested_parties_ = nullptr;
  /// are we shutting down?
  bool shutdown_ = false;
  /// are we currently resolving?
  OrphanablePtr<Orphanable> request_;
  /// min time between DNS requests
  Duration min_time_between_resolutions_;
  /// timestamp of last DNS request
  std::optional<Timestamp> last_resolution_timestamp_;
  /// retry backoff state
  BackOff backoff_;
  /// state for handling interactions between re-resolution requests and
  /// result health callbacks
  enum class ResultStatusState {
    kNone,
    kResultHealthCallbackPending,
    kReresolutionRequestedWhileCallbackWasPending,
  };
  ResultStatusState result_status_state_ = ResultStatusState::kNone;
  /// next resolution timer
  std::optional<grpc_event_engine::experimental::EventEngine::TaskHandle>
      next_resolution_timer_handle_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_RESOLVER_POLLING_RESOLVER_H
