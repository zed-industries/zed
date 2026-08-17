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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_PENDING_CONNECTION_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_PENDING_CONNECTION_H

#include <string>

#include "absl/status/statusor.h"
#include "src/core/lib/promise/promise.h"
#include "src/core/lib/transport/promise_endpoint.h"
#include "src/core/util/dual_ref_counted.h"

namespace grpc_core {
namespace chaotic_good {

// Essentially this is the promise of one endpoint in the future, with the
// addition of an id used for handshaking so that can be communicated around as
// necessary.
class PendingConnection {
 public:
  explicit PendingConnection(absl::string_view id,
                             Promise<absl::StatusOr<PromiseEndpoint>> connector)
      : id_(id), connector_(std::move(connector)) {}

  PendingConnection(const PendingConnection&) = delete;
  PendingConnection& operator=(const PendingConnection&) = delete;
  PendingConnection(PendingConnection&&) = default;
  PendingConnection& operator=(PendingConnection&&) = default;

  absl::string_view id() const { return id_; }
  auto Await() { return std::move(connector_); }

 private:
  std::string id_;
  Promise<absl::StatusOr<PromiseEndpoint>> connector_;
};

class ServerConnectionFactory : public DualRefCounted<ServerConnectionFactory> {
 public:
  using DualRefCounted::DualRefCounted;
  virtual PendingConnection RequestDataConnection() = 0;
};

class ClientConnectionFactory : public DualRefCounted<ClientConnectionFactory> {
 public:
  using DualRefCounted::DualRefCounted;
  virtual PendingConnection Connect(absl::string_view id) = 0;
};

// Helper: convert an already existing endpoint into a pending connection
inline PendingConnection ImmediateConnection(absl::string_view id,
                                             PromiseEndpoint endpoint) {
  return PendingConnection(
      id,
      [endpoint = std::move(endpoint)]() mutable
          -> absl::StatusOr<PromiseEndpoint> { return std::move(endpoint); });
}

}  // namespace chaotic_good
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_PENDING_CONNECTION_H
