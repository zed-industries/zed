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

#ifndef GRPC_TEST_CORE_EVENT_ENGINE_MOCK_EVENT_ENGINE_H
#define GRPC_TEST_CORE_EVENT_ENGINE_MOCK_EVENT_ENGINE_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>

#include <memory>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "gmock/gmock.h"

namespace grpc_event_engine {
namespace experimental {

class MockEventEngine : public EventEngine {
 public:
  MOCK_METHOD(
      absl::StatusOr<std::unique_ptr<Listener>>, CreateListener,
      (Listener::AcceptCallback on_accept,
       absl::AnyInvocable<void(absl::Status)> on_shutdown,
       const EndpointConfig& config,
       std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory));
  MOCK_METHOD(ConnectionHandle, Connect,
              (OnConnectCallback on_connect, const ResolvedAddress& addr,
               const EndpointConfig& args, MemoryAllocator memory_allocator,
               Duration timeout));
  MOCK_METHOD(bool, CancelConnect, (ConnectionHandle handle));
  MOCK_METHOD(bool, IsWorkerThread, ());
  MOCK_METHOD(absl::StatusOr<std::unique_ptr<DNSResolver>>, GetDNSResolver,
              (const DNSResolver::ResolverOptions& options));
  MOCK_METHOD(void, Run, (Closure * closure));
  MOCK_METHOD(void, Run, (absl::AnyInvocable<void()> closure));
  MOCK_METHOD(TaskHandle, RunAfter, (Duration when, Closure* closure));
  MOCK_METHOD(TaskHandle, RunAfter,
              (Duration when, absl::AnyInvocable<void()> closure));
  MOCK_METHOD(bool, Cancel, (TaskHandle handle));
};

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_TEST_CORE_EVENT_ENGINE_MOCK_EVENT_ENGINE_H
