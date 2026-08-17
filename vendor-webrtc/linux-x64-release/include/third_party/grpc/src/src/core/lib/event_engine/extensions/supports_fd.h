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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_EXTENSIONS_SUPPORTS_FD_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_EXTENSIONS_SUPPORTS_FD_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include "absl/functional/any_invocable.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace grpc_event_engine::experimental {

class EndpointSupportsFdExtension {
 public:
  virtual ~EndpointSupportsFdExtension() = default;
  static absl::string_view EndpointExtensionName() {
    return "io.grpc.event_engine.extension.endpoint_supports_fd";
  }
  /// Returns the file descriptor associated with the posix endpoint.
  virtual int GetWrappedFd() = 0;

  /// Shutdown the endpoint. This function call should trigger execution of
  /// any pending endpoint Read/Write callbacks with appropriate error
  /// absl::Status. After this function call any subsequent endpoint
  /// Read/Write operations until endpoint deletion should fail with an
  /// appropriate absl::Status.
  ///
  /// \a on_release_fd - If specified, the callback is invoked when the
  /// endpoint is destroyed/deleted. The underlying file descriptor is
  /// released instead of being closed. The callback will get the released
  /// file descriptor as its argument if the release operation is successful.
  /// Otherwise it would get an appropriate error status as its argument.
  virtual void Shutdown(absl::AnyInvocable<void(absl::StatusOr<int> release_fd)>
                            on_release_fd) = 0;
};

class ListenerSupportsFdExtension {
 public:
  virtual ~ListenerSupportsFdExtension() = default;
  static absl::string_view EndpointExtensionName() {
    return "io.grpc.event_engine.extension.listener_supports_fd";
  }
  /// Called when a posix listener bind operation completes. A single bind
  /// operation may trigger creation of multiple listener fds. This callback
  /// should be invoked once on each newly created and bound fd. If the
  /// corresponding bind operation fails for a particular fd, this callback
  /// must be invoked with a absl::FailedPreConditionError status.
  ///
  /// \a listener_fd - The listening socket fd that was bound to the specified
  /// address.
  using OnPosixBindNewFdCallback =
      absl::AnyInvocable<void(absl::StatusOr<int> listener_fd)>;
  /// Bind an address/port to this Listener.
  ///
  /// It is expected that multiple addresses/ports can be bound to this
  /// Listener before Listener::Start has been called. Returns either the
  /// bound port or an appropriate error status.
  /// \a addr - The address to listen for incoming connections.
  /// \a on_bind_new_fd The callback is invoked once for each newly bound
  /// listener fd that may be created by this Bind operation.
  virtual absl::StatusOr<int> BindWithFd(
      const EventEngine::ResolvedAddress& addr,
      OnPosixBindNewFdCallback on_bind_new_fd) = 0;

  /// Handle an externally accepted client connection. It must return an
  /// appropriate error status in case of failure.
  ///
  /// This may be invoked to process a new client connection accepted by an
  /// external listening fd.
  /// \a listener_fd - The external listening socket fd that accepted the new
  /// client connection.
  /// \a fd - The socket file descriptor representing the new client
  /// connection.
  /// \a pending_data - If specified, it holds any pending data that may have
  /// already been read over the externally accepted client connection.
  /// Otherwise, it is assumed that no data has been read over the new client
  /// connection.
  virtual absl::Status HandleExternalConnection(int listener_fd, int fd,
                                                SliceBuffer* pending_data) = 0;

  /// Shutdown/stop listening on all bind Fds.
  virtual void ShutdownListeningFds() = 0;
};

class EventEngineSupportsFdExtension {
 public:
  virtual ~EventEngineSupportsFdExtension() = default;
  static absl::string_view EndpointExtensionName() {
    return "io.grpc.event_engine.extension.event_engine_supports_fd";
  }
  /// Creates a posix specific EventEngine::Endpoint from an fd which is already
  /// assumed to be connected to a remote peer. \a fd - The connected socket
  /// file descriptor. \a config - Additional configuration to applied to the
  /// endpoint. \a memory_allocator - The endpoint may use the provided memory
  /// allocator to track memory allocations.
  virtual std::unique_ptr<EventEngine::Endpoint> CreatePosixEndpointFromFd(
      int fd, const EndpointConfig& config,
      MemoryAllocator memory_allocator) = 0;

  /// Creates an EventEngine::Endpoint from an fd which is already assumed to be
  /// connected to a remote peer. See \a CreatePosixEndpointFromFd for details.
  /// This has the same behavior, but the \a memory_allocator is taken from the
  /// EndpointConfig's resource quota.
  virtual std::unique_ptr<EventEngine::Endpoint> CreateEndpointFromFd(
      int fd, const EndpointConfig& config) = 0;

  /// Creates an EventEngine::Endpoint from a file descriptor that is configured
  /// and bound locally but not yet connected to a remote peer. Returns a
  /// connection handle to cancel the connection attempt if needed. Created
  /// endpoint will be returned through `on_connect` callback.
  /// \a fd - The socket file descriptor.
  /// \a on_connect - The callback to invoke once fd is connected to peer.
  /// \a addr - The remote peer to connect to. This should be the mapped peer
  /// address returned when creating a new socket.
  /// \a config - Additional configuration to be applied to the endpoint.
  /// \a memory_allocator - The endpoint may use the provided memory allocator
  /// to track memory allocations.
  /// \a timeout - The timeout to use for the connection attempt.
  virtual EventEngine::ConnectionHandle CreateEndpointFromUnconnectedFd(
      int fd, EventEngine::OnConnectCallback on_connect,
      const EventEngine::ResolvedAddress& addr, const EndpointConfig& config,
      MemoryAllocator memory_allocator, EventEngine::Duration timeout) = 0;

  /// Called when the posix listener has accepted a new client connection.
  /// \a listener_fd - The listening socket fd that accepted the new client
  /// connection.
  /// \a endpoint - The EventEngine endpoint to handle data exchange over the
  /// new client connection.
  /// \a is_external - A boolean indicating whether the new client connection
  /// is accepted by an external listener_fd or by a listener_fd that is
  /// managed by the EventEngine listener.
  /// \a memory_allocator - The callback may use the provided memory
  /// allocator to handle memory allocation operations.
  /// \a pending_data - If specified, it holds any pending data that may have
  /// already been read over the new client connection. Otherwise, it is
  /// assumed that no data has been read over the new client connection.
  using PosixAcceptCallback = absl::AnyInvocable<void(
      int listener_fd, std::unique_ptr<EventEngine::Endpoint> endpoint,
      bool is_external, MemoryAllocator memory_allocator,
      SliceBuffer* pending_data)>;

  /// Factory method to create a posix specific network listener / server with
  /// fd support.
  ///
  /// Once a \a Listener is created and started, the \a on_accept callback will
  /// be called once asynchronously for each established connection. This method
  /// may return a non-OK status immediately if an error was encountered in any
  /// synchronous steps required to create the Listener. In this case,
  /// \a on_shutdown will never be called.
  ///
  /// If this method returns a Listener, then \a on_shutdown will be invoked
  /// exactly once, when the Listener is shut down. The status passed to it will
  /// indicate if there was a problem during shutdown.
  ///
  /// The provided \a MemoryAllocatorFactory is used to create \a
  /// MemoryAllocators for Endpoint construction.
  virtual absl::StatusOr<std::unique_ptr<EventEngine::Listener>>
  CreatePosixListener(
      PosixAcceptCallback on_accept,
      absl::AnyInvocable<void(absl::Status)> on_shutdown,
      const EndpointConfig& config,
      std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory) = 0;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_EXTENSIONS_SUPPORTS_FD_H
