// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_FILTERS_CHANNEL_IDLE_LEGACY_CHANNEL_IDLE_FILTER_H
#define GRPC_SRC_CORE_EXT_FILTERS_CHANNEL_IDLE_LEGACY_CHANNEL_IDLE_FILTER_H

#include <grpc/impl/connectivity_state.h>
#include <grpc/support/port_platform.h>

#include <memory>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/ext/filters/channel_idle/idle_filter_state.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/connectivity_state.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/single_set_ptr.h"
#include "src/core/util/time.h"

namespace grpc_core {

Duration GetClientIdleTimeout(const ChannelArgs& args);

class LegacyChannelIdleFilter : public ChannelFilter {
 public:
  LegacyChannelIdleFilter(grpc_channel_stack* channel_stack,
                          Duration client_idle_timeout)
      : channel_stack_(channel_stack),
        client_idle_timeout_(client_idle_timeout) {}

  ~LegacyChannelIdleFilter() override = default;

  LegacyChannelIdleFilter(const LegacyChannelIdleFilter&) = delete;
  LegacyChannelIdleFilter& operator=(const LegacyChannelIdleFilter&) = delete;
  LegacyChannelIdleFilter(LegacyChannelIdleFilter&&) = default;
  LegacyChannelIdleFilter& operator=(LegacyChannelIdleFilter&&) = default;

  // Construct a promise for one call.
  ArenaPromise<ServerMetadataHandle> MakeCallPromise(
      CallArgs call_args, NextPromiseFactory next_promise_factory) override;

  bool StartTransportOp(grpc_transport_op* op) override;

 protected:
  using SingleSetActivityPtr =
      SingleSetPtr<Activity, typename ActivityPtr::deleter_type>;

  grpc_channel_stack* channel_stack() { return channel_stack_; };

  virtual void Shutdown();
  void CloseChannel(absl::string_view reason);

  void IncreaseCallCount();
  void DecreaseCallCount();

 private:
  void StartIdleTimer();

  struct CallCountDecreaser {
    void operator()(LegacyChannelIdleFilter* filter) const {
      filter->DecreaseCallCount();
    }
  };

  // The channel stack to which we take refs for pending callbacks.
  grpc_channel_stack* channel_stack_;
  Duration client_idle_timeout_;
  std::shared_ptr<IdleFilterState> idle_filter_state_{
      std::make_shared<IdleFilterState>(false)};

  SingleSetActivityPtr activity_;
};

class LegacyClientIdleFilter final : public LegacyChannelIdleFilter {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "client_idle"; }

  static absl::StatusOr<std::unique_ptr<LegacyClientIdleFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args filter_args);

  using LegacyChannelIdleFilter::LegacyChannelIdleFilter;
};

class LegacyMaxAgeFilter final : public LegacyChannelIdleFilter {
 public:
  static const grpc_channel_filter kFilter;
  struct Config;

  static absl::string_view TypeName() { return "max_age"; }

  static absl::StatusOr<std::unique_ptr<LegacyMaxAgeFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args filter_args);

  LegacyMaxAgeFilter(grpc_channel_stack* channel_stack,
                     const Config& max_age_config);

  void PostInit() override;

 private:
  class ConnectivityWatcher : public AsyncConnectivityStateWatcherInterface {
   public:
    explicit ConnectivityWatcher(LegacyMaxAgeFilter* filter)
        : channel_stack_(filter->channel_stack()->Ref()), filter_(filter) {}
    ~ConnectivityWatcher() override = default;

    void OnConnectivityStateChange(grpc_connectivity_state new_state,
                                   const absl::Status&) override {
      if (new_state == GRPC_CHANNEL_SHUTDOWN) filter_->Shutdown();
    }

   private:
    RefCountedPtr<grpc_channel_stack> channel_stack_;
    LegacyMaxAgeFilter* filter_;
  };

  void Shutdown() override;

  SingleSetActivityPtr max_age_activity_;
  Duration max_connection_age_;
  Duration max_connection_age_grace_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_FILTERS_CHANNEL_IDLE_LEGACY_CHANNEL_IDLE_FILTER_H
