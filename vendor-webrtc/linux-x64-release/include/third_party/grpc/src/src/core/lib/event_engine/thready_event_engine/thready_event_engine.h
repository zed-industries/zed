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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_THREADY_EVENT_ENGINE_THREADY_EVENT_ENGINE_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_THREADY_EVENT_ENGINE_THREADY_EVENT_ENGINE_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/support/port_platform.h>

#include <memory>
#include <utility>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace grpc_event_engine::experimental {

// An EventEngine that spawns a thread at every available opportunity:
// - Run() spawns a thread
// - RunAfter() schedules a timer that spawns a thread to run the callback
// - Endpoint operations spawn threads and then call the underlying event engine
//   functions
// Implemented as a decorator around a complete EventEngine so that it need not
// deal with OS details.
// This event engine is intended to be used for testing with TSAN to maximize
// its visibility into race conditions in the calling code.
class ThreadyEventEngine final : public EventEngine {
 public:
  explicit ThreadyEventEngine(std::shared_ptr<EventEngine> impl)
      : impl_(std::move(impl)) {}

  absl::StatusOr<std::unique_ptr<Listener>> CreateListener(
      Listener::AcceptCallback on_accept,
      absl::AnyInvocable<void(absl::Status)> on_shutdown,
      const EndpointConfig& config,
      std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory)
      override;

  ConnectionHandle Connect(OnConnectCallback on_connect,
                           const ResolvedAddress& addr,
                           const EndpointConfig& args,
                           MemoryAllocator memory_allocator,
                           Duration timeout) override;

  bool CancelConnect(ConnectionHandle handle) override;

  bool IsWorkerThread() override;

  absl::StatusOr<std::unique_ptr<DNSResolver>> GetDNSResolver(
      const DNSResolver::ResolverOptions& options) override;

  void Run(Closure* closure) override;
  void Run(absl::AnyInvocable<void()> closure) override;

  TaskHandle RunAfter(Duration when, Closure* closure) override;
  TaskHandle RunAfter(Duration when,
                      absl::AnyInvocable<void()> closure) override;

  bool Cancel(TaskHandle handle) override;

 private:
  class ThreadyDNSResolver final : public DNSResolver {
   public:
    ThreadyDNSResolver(std::unique_ptr<DNSResolver> impl,
                       std::shared_ptr<ThreadyEventEngine> engine)
        : impl_(std::move(impl)), engine_(std::move(engine)) {}
    void LookupHostname(LookupHostnameCallback on_resolve,
                        absl::string_view name,
                        absl::string_view default_port) override;
    void LookupSRV(LookupSRVCallback on_resolve,
                   absl::string_view name) override;
    void LookupTXT(LookupTXTCallback on_resolve,
                   absl::string_view name) override;

   private:
    std::unique_ptr<DNSResolver> impl_;
    std::shared_ptr<ThreadyEventEngine> engine_;
  };

  void Asynchronously(absl::AnyInvocable<void()> fn);

  std::shared_ptr<EventEngine> impl_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_THREADY_EVENT_ENGINE_THREADY_EVENT_ENGINE_H
