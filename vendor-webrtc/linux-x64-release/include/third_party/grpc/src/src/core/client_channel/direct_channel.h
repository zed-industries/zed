// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_DIRECT_CHANNEL_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_DIRECT_CHANNEL_H

#include <memory>

#include "src/core/lib/surface/channel.h"
#include "src/core/lib/transport/transport.h"

namespace grpc_core {

class DirectChannel final : public Channel {
 public:
  class TransportCallDestination final : public CallDestination {
   public:
    explicit TransportCallDestination(OrphanablePtr<ClientTransport> transport)
        : transport_(std::move(transport)) {}

    ClientTransport* transport() { return transport_.get(); }

    void HandleCall(CallHandler handler) override {
      transport_->StartCall(std::move(handler));
    }

    void Orphaned() override { transport_.reset(); }

   private:
    OrphanablePtr<ClientTransport> transport_;
  };

  static absl::StatusOr<RefCountedPtr<DirectChannel>> Create(
      std::string target, const ChannelArgs& args);

  DirectChannel(
      std::string target, const ChannelArgs& args,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>
          event_engine,
      RefCountedPtr<TransportCallDestination> transport_call_destination,
      RefCountedPtr<UnstartedCallDestination> interception_chain)
      : Channel(std::move(target), args),
        transport_call_destination_(std::move(transport_call_destination)),
        interception_chain_(std::move(interception_chain)),
        event_engine_(std::move(event_engine)) {}

  void Orphaned() override;
  void StartCall(UnstartedCallHandler unstarted_handler) override;
  bool IsLame() const override { return false; }
  grpc_call* CreateCall(grpc_call* parent_call, uint32_t propagation_mask,
                        grpc_completion_queue* cq,
                        grpc_pollset_set* pollset_set_alternative, Slice path,
                        std::optional<Slice> authority, Timestamp deadline,
                        bool registered_method) override;
  grpc_event_engine::experimental::EventEngine* event_engine() const override {
    return event_engine_.get();
  }
  bool SupportsConnectivityWatcher() const override { return false; }
  grpc_connectivity_state CheckConnectivityState(bool) override {
    Crash("CheckConnectivityState not supported");
  }
  void WatchConnectivityState(grpc_connectivity_state, Timestamp,
                              grpc_completion_queue*, void*) override {
    Crash("WatchConnectivityState not supported");
  }
  void AddConnectivityWatcher(
      grpc_connectivity_state,
      OrphanablePtr<AsyncConnectivityStateWatcherInterface>) override {
    Crash("AddConnectivityWatcher not supported");
  }
  void RemoveConnectivityWatcher(
      AsyncConnectivityStateWatcherInterface*) override {
    Crash("RemoveConnectivityWatcher not supported");
  }
  void GetInfo(const grpc_channel_info* channel_info) override;
  void ResetConnectionBackoff() override {}
  void Ping(grpc_completion_queue*, void*) override {
    Crash("Ping not supported");
  }

 private:
  RefCountedPtr<TransportCallDestination> transport_call_destination_;
  RefCountedPtr<UnstartedCallDestination> interception_chain_;
  const std::shared_ptr<grpc_event_engine::experimental::EventEngine>
      event_engine_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_DIRECT_CHANNEL_H
