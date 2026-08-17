//
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
//

#ifndef GRPC_SRC_CORE_LIB_SURFACE_LEGACY_CHANNEL_H
#define GRPC_SRC_CORE_LIB_SURFACE_LEGACY_CHANNEL_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include <optional>
#include <string>

#include "absl/status/statusor.h"
#include "src/core/call/call_arena_allocator.h"
#include "src/core/client_channel/client_channel_filter.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack.h"  // IWYU pragma: keep
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/surface/channel.h"
#include "src/core/lib/surface/channel_stack_type.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/telemetry/stats.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"

namespace grpc_core {

class LegacyChannel final : public Channel {
 public:
  static absl::StatusOr<RefCountedPtr<Channel>> Create(
      std::string target, ChannelArgs args,
      grpc_channel_stack_type channel_stack_type);

  // Do not instantiate directly -- use Create() instead.
  LegacyChannel(bool is_client, std::string target,
                const ChannelArgs& channel_args,
                RefCountedPtr<grpc_channel_stack> channel_stack);

  void Orphaned() override;

  bool IsLame() const override;

  grpc_call* CreateCall(grpc_call* parent_call, uint32_t propagation_mask,
                        grpc_completion_queue* cq,
                        grpc_pollset_set* pollset_set_alternative, Slice path,
                        std::optional<Slice> authority, Timestamp deadline,
                        bool registered_method) override;

  void StartCall(UnstartedCallHandler) override {
    Crash("StartCall() not supported on LegacyChannel");
  }

  grpc_event_engine::experimental::EventEngine* event_engine() const override {
    return channel_stack_->EventEngine();
  }

  bool SupportsConnectivityWatcher() const override;

  grpc_connectivity_state CheckConnectivityState(bool try_to_connect) override;

  void WatchConnectivityState(grpc_connectivity_state last_observed_state,
                              Timestamp deadline, grpc_completion_queue* cq,
                              void* tag) override;

  void AddConnectivityWatcher(
      grpc_connectivity_state initial_state,
      OrphanablePtr<AsyncConnectivityStateWatcherInterface> watcher) override;
  void RemoveConnectivityWatcher(
      AsyncConnectivityStateWatcherInterface* watcher) override;

  void GetInfo(const grpc_channel_info* channel_info) override;

  void ResetConnectionBackoff() override;

  void Ping(grpc_completion_queue* cq, void* tag) override;

  bool is_client() const override { return is_client_; }
  grpc_channel_stack* channel_stack() const override {
    return channel_stack_.get();
  }

 private:
  class StateWatcher;

  // Returns the client channel filter if this is a client channel,
  // otherwise null.
  ClientChannelFilter* GetClientChannelFilter() const;

  const bool is_client_;
  RefCountedPtr<grpc_channel_stack> channel_stack_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SURFACE_LEGACY_CHANNEL_H
