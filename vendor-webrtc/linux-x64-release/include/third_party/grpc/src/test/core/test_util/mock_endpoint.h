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

#ifndef GRPC_TEST_CORE_TEST_UTIL_MOCK_ENDPOINT_H
#define GRPC_TEST_CORE_TEST_UTIL_MOCK_ENDPOINT_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/slice.h>

#include <memory>

#include "src/core/lib/iomgr/endpoint.h"

namespace grpc_event_engine {
namespace experimental {

// Internal controller object for mock endpoint operations.
//
// This helps avoid shared ownership issues. The endpoint itself may destroyed
// while a fuzzer is still attempting to use it (e.g., the transport is closed,
// and a fuzzer still wants to schedule reads).
class MockEndpointController
    : public std::enable_shared_from_this<MockEndpointController> {
 public:
  // Factory method ensures this class is always a shared_ptr.
  static std::shared_ptr<MockEndpointController> Create(
      std::shared_ptr<EventEngine> engine);

  ~MockEndpointController();

  // ---- mock methods ----
  void TriggerReadEvent(Slice read_data);
  void NoMoreReads();
  void Read(absl::AnyInvocable<void(absl::Status)> on_read,
            SliceBuffer* buffer);
  // Takes ownership of the grpc_endpoint object from the controller.
  grpc_endpoint* TakeCEndpoint();

  // ---- accessors ----
  EventEngine* engine() { return engine_.get(); }

 private:
  explicit MockEndpointController(std::shared_ptr<EventEngine> engine);

  std::shared_ptr<EventEngine> engine_;
  grpc_core::Mutex mu_;
  bool reads_done_ ABSL_GUARDED_BY(mu_) = false;
  SliceBuffer read_buffer_ ABSL_GUARDED_BY(mu_);
  absl::AnyInvocable<void(absl::Status)> on_read_ ABSL_GUARDED_BY(mu_);
  SliceBuffer* on_read_slice_buffer_ ABSL_GUARDED_BY(mu_) = nullptr;
  grpc_endpoint* mock_grpc_endpoint_;
};

class MockEndpoint : public EventEngine::Endpoint {
 public:
  MockEndpoint();
  ~MockEndpoint() override = default;

  // ---- mock methods ----
  void SetController(std::shared_ptr<MockEndpointController> endpoint_control) {
    endpoint_control_ = std::move(endpoint_control);
  }

  // ---- overrides ----
  bool Read(absl::AnyInvocable<void(absl::Status)> on_read, SliceBuffer* buffer,
            ReadArgs args) override;
  bool Write(absl::AnyInvocable<void(absl::Status)> on_writable,
             SliceBuffer* data, WriteArgs args) override;
  const EventEngine::ResolvedAddress& GetPeerAddress() const override;
  const EventEngine::ResolvedAddress& GetLocalAddress() const override;

  std::vector<size_t> AllWriteMetrics() override { return {}; }

  std::optional<absl::string_view> GetMetricName(size_t) override {
    return std::nullopt;
  }
  std::optional<size_t> GetMetricKey(absl::string_view) override {
    return std::nullopt;
  }

 private:
  std::shared_ptr<MockEndpointController> endpoint_control_;
  EventEngine::ResolvedAddress peer_addr_;
  EventEngine::ResolvedAddress local_addr_;
};

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_TEST_CORE_TEST_UTIL_MOCK_ENDPOINT_H
