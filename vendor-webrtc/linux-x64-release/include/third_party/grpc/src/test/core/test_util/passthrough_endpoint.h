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

#ifndef GRPC_TEST_CORE_TEST_UTIL_PASSTHROUGH_ENDPOINT_H
#define GRPC_TEST_CORE_TEST_UTIL_PASSTHROUGH_ENDPOINT_H

#include <grpc/event_engine/event_engine.h>

#include <memory>

#include "src/core/lib/event_engine/default_event_engine.h"
#include "src/core/lib/event_engine/tcp_socket_utils.h"
#include "src/core/util/ref_counted.h"

namespace grpc_event_engine {
namespace experimental {

class PassthroughEndpoint final : public EventEngine::Endpoint {
 public:
  struct PassthroughEndpointPair {
    std::unique_ptr<PassthroughEndpoint> client;
    std::unique_ptr<PassthroughEndpoint> server;
  };
  // client_port, server_port are markers that are baked into the peer/local
  // addresses for debug information.
  // allow_inline_callbacks is a flag that allows the endpoint to call the
  // on_read/on_write callbacks inline (but outside any PassthroughEndpoint
  // locks)
  static PassthroughEndpointPair MakePassthroughEndpoint(
      int client_port, int server_port, bool allow_inline_callbacks);

  ~PassthroughEndpoint() override;

  bool Read(absl::AnyInvocable<void(absl::Status)> on_read, SliceBuffer* buffer,
            ReadArgs args) override;

  bool Write(absl::AnyInvocable<void(absl::Status)> on_write,
             SliceBuffer* buffer, WriteArgs args) override;

  std::vector<size_t> AllWriteMetrics() override { return {}; }
  std::optional<absl::string_view> GetMetricName(size_t) override {
    return std::nullopt;
  }
  std::optional<size_t> GetMetricKey(absl::string_view) override {
    return std::nullopt;
  }

  const EventEngine::ResolvedAddress& GetPeerAddress() const override {
    return recv_middle_->address;
  }
  const EventEngine::ResolvedAddress& GetLocalAddress() const override {
    return send_middle_->address;
  }

 private:
  class CallbackHelper;

  struct Middle : public grpc_core::RefCounted<Middle> {
    explicit Middle(int port)
        : address(URIToResolvedAddress(absl::StrCat("ipv4:127.0.0.1:", port))
                      .value()) {}

    void Close(CallbackHelper& callback_helper);

    grpc_core::Mutex mu;
    bool closed ABSL_GUARDED_BY(mu) = false;
    SliceBuffer* read_buffer ABSL_GUARDED_BY(mu) = nullptr;
    absl::AnyInvocable<void(absl::Status)> on_read ABSL_GUARDED_BY(mu) =
        nullptr;
    SliceBuffer* write_buffer ABSL_GUARDED_BY(mu) = nullptr;
    absl::AnyInvocable<void(absl::Status)> on_write ABSL_GUARDED_BY(mu) =
        nullptr;
    EventEngine::ResolvedAddress address;
  };

  PassthroughEndpoint(grpc_core::RefCountedPtr<Middle> send_middle,
                      grpc_core::RefCountedPtr<Middle> recv_middle,
                      bool allow_inline_callbacks)
      : send_middle_(std::move(send_middle)),
        recv_middle_(std::move(recv_middle)),
        allow_inline_callbacks_(allow_inline_callbacks) {}

  grpc_core::RefCountedPtr<Middle> send_middle_;
  grpc_core::RefCountedPtr<Middle> recv_middle_;
  std::shared_ptr<EventEngine> event_engine_ = GetDefaultEventEngine();
  bool allow_inline_callbacks_;
};

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_TEST_CORE_TEST_UTIL_PASSTHROUGH_ENDPOINT_H
