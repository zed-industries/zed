// Copyright 2022 gRPC Authors
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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENGINE_LISTENER_UTILS_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENGINE_LISTENER_UTILS_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include "absl/status/statusor.h"
#include "src/core/lib/event_engine/posix_engine/tcp_socket_utils.h"

namespace grpc_event_engine::experimental {

// This interface exists to allow different EventEngines to implement different
// custom interception operations while a socket is Appended. The
// listener util functions are defined over this interface and thus can be
// shared across multiple EventEngines.
class ListenerSocketsContainer {
 public:
  struct ListenerSocket {
    // Listener socket fd
    PosixSocketWrapper sock;
    // Assigned/chosen listening port
    int port;
    // Socket configuration
    bool zero_copy_enabled;
    // Address at which the socket is listening for connections
    grpc_event_engine::experimental::EventEngine::ResolvedAddress addr;
    // Dual stack mode.
    PosixSocketWrapper::DSMode dsmode;
  };
  // Adds a socket to the internal db of sockets associated with a listener.
  virtual void Append(ListenerSocket socket) = 0;

  // Returns a non-OK status if the socket cannot be found. Otherwise, returns
  // the socket.
  virtual absl::StatusOr<ListenerSocket> Find(
      const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
          addr) = 0;

  virtual ~ListenerSocketsContainer() = default;
};

// Creates and configures a socket to be used by the EventEngine Listener. The
// type of the socket to create is determined by the by the passed address. The
// socket configuration is specified by passed tcp options. If successful, it
// returns a ListenerSocketsContainer::ListenerSocket type which holds the
// socket fd and its dsmode. If unsuccessful, it returns a Not-OK status.
absl::StatusOr<ListenerSocketsContainer::ListenerSocket>
CreateAndPrepareListenerSocket(
    const PosixTcpOptions& options,
    const grpc_event_engine::experimental::EventEngine::ResolvedAddress& addr);

// Instead of creating and adding a socket bound to specific address, this
// function creates and adds a socket bound to the wildcard address on the
// server. The newly created socket is configured according to the passed
// options and added to the passed ListenerSocketsContainer object. The function
// returns the port at which the created socket listens for incoming
// connections.
absl::StatusOr<int> ListenerContainerAddWildcardAddresses(
    ListenerSocketsContainer& listener_sockets, const PosixTcpOptions& options,
    int requested_port);

// Get all addresses assigned to network interfaces on the machine and create
// and add a socket for each local address. Each newly created socket is
// configured according to the passed options and added to the passed
// ListenerSocketsContainer object. The requested_port is the port to use for
// every socket. If set to 0, a random port will be used for every socket.
// The function returns the chosen port number for all created sockets.
absl::StatusOr<int> ListenerContainerAddAllLocalAddresses(
    ListenerSocketsContainer& listener_sockets, const PosixTcpOptions& options,
    int requested_port);

// Returns true if addr is link-local (i.e. within the range 169.254.0.0/16 or
// fe80::/10).
bool IsSockAddrLinkLocal(const EventEngine::ResolvedAddress* resolved_addr);

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENGINE_LISTENER_UTILS_H
