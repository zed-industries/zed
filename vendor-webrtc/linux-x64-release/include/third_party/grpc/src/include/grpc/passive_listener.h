// Copyright 2024 The gRPC Authors
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
#ifndef GRPC_PASSIVE_LISTENER_H
#define GRPC_PASSIVE_LISTENER_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>

#include <memory>
// #include <grpc/support/port_platform.h>

namespace grpc_core {
class Server;

namespace experimental {
class PassiveListenerImpl;

/// -- EXPERIMENTAL API --
/// Interface for used for Server Endpoint injection.
class PassiveListener {
 public:
  virtual ~PassiveListener() = default;
  /// -- EXPERIMENTAL API --
  ///
  /// Takes an Endpoint for an established connection, and treats it as if the
  /// connection had been accepted by the server.
  ///
  /// The server must be started before endpoints can be accepted.
  virtual absl::Status AcceptConnectedEndpoint(
      std::unique_ptr<grpc_event_engine::experimental::EventEngine::Endpoint>
          endpoint) = 0;

  /// -- EXPERIMENTAL API --
  ///
  /// Takes a connected file descriptor, and treats it as if the server had
  /// accepted the connection itself.
  ///
  /// Returns a failure status if the server's active EventEngine does not
  /// support Endpoint creation from fds.
  virtual absl::Status AcceptConnectedFd(int fd) = 0;
};

}  // namespace experimental
}  // namespace grpc_core

absl::Status grpc_server_add_passive_listener(
    grpc_core::Server* server, grpc_server_credentials* credentials,
    std::shared_ptr<grpc_core::experimental::PassiveListenerImpl>
        passive_listener);

#endif /* GRPC_PASSIVE_LISTENER_H */
