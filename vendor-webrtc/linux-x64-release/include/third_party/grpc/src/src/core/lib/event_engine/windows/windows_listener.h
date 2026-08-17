// Copyright 2023 The gRPC Authors
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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_WINDOWS_WINDOWS_LISTENER_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_WINDOWS_WINDOWS_LISTENER_H

#include <grpc/support/port_platform.h>

#ifdef GPR_WINDOWS

#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>

#include <list>

#include "absl/base/thread_annotations.h"
#include "absl/status/statusor.h"
#include "src/core/lib/event_engine/common_closures.h"
#include "src/core/lib/event_engine/extensions/iomgr_compatible.h"
#include "src/core/lib/event_engine/query_extensions.h"
#include "src/core/lib/event_engine/thread_pool/thread_pool.h"
#include "src/core/lib/event_engine/windows/iocp.h"
#include "src/core/lib/iomgr/port.h"
#include "src/core/util/sync.h"

#ifdef GRPC_HAVE_UNIX_SOCKET
// clang-format off
#include <ws2def.h>
#include <afunix.h>
// clang-format on
#endif

namespace grpc_event_engine::experimental {

class WindowsEventEngineListener
    : public ExtendedType<
          EventEngine::Listener,
          grpc_event_engine::experimental::IomgrCompatibleListener> {
 public:
  WindowsEventEngineListener(
      IOCP* iocp, AcceptCallback accept_cb,
      absl::AnyInvocable<void(absl::Status)> on_shutdown,
      std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory,
      std::shared_ptr<EventEngine> engine, ThreadPool* thread_pool_,
      const EndpointConfig& config);
  ~WindowsEventEngineListener() override;
  absl::StatusOr<int> Bind(const EventEngine::ResolvedAddress& addr) override;
  absl::Status Start() override;

  // Part of IomgrCompatibleListener
  void Shutdown() override;

 private:
  /// Responsible for listening on a single port.
  class SinglePortSocketListener {
   public:
    ~SinglePortSocketListener();
    // This factory will create a bound, listening WinSocket, registered with
    // the listener's IOCP poller.
    static absl::StatusOr<std::unique_ptr<SinglePortSocketListener>> Create(
        WindowsEventEngineListener* listener, SOCKET sock,
        EventEngine::ResolvedAddress addr);

    // Two-stage initialization, allows creation of all bound sockets before the
    // listener is started.
    absl::Status Start();
    absl::Status StartLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(io_state_->mu);

    // Accessor methods
    EventEngine::ResolvedAddress listener_sockname() {
      return listener_sockname_;
    };
    int port() { return port_; }

   private:
    struct AsyncIOState;

    class OnAcceptCallbackWrapper : public EventEngine::Closure {
     public:
      void Run() override;
      void Prime(std::shared_ptr<AsyncIOState> io_state);

     private:
      std::shared_ptr<AsyncIOState> io_state_;
    };

    // A class to manage the data that must outlive the Endpoint.
    //
    // Once a listener is done and destroyed, there still may be overlapped
    // operations pending. To clean up safely, this data must outlive the
    // Listener, and be destroyed asynchronously when all pending overlapped
    // events are complete.
    struct AsyncIOState {
      AsyncIOState(SinglePortSocketListener* port_listener,
                   std::unique_ptr<WinSocket> listener_socket);
      ~AsyncIOState();
      SinglePortSocketListener* const port_listener;
      OnAcceptCallbackWrapper on_accept_cb;
      // Synchronize accept handling on the same socket.
      grpc_core::Mutex mu;
      // This will hold the socket for the next accept.
      SOCKET accept_socket ABSL_GUARDED_BY(mu) = INVALID_SOCKET;
      // The listener winsocket.
      std::unique_ptr<WinSocket> listener_socket ABSL_GUARDED_BY(mu);
    };

    SinglePortSocketListener(WindowsEventEngineListener* listener,
                             LPFN_ACCEPTEX AcceptEx,
                             std::unique_ptr<WinSocket> listener_socket,
                             int port, EventEngine::ResolvedAddress hostbyname);

    // Bind a recently-created socket for listening
    struct PrepareListenerSocketResult {
      int port;
      EventEngine::ResolvedAddress hostbyname;
    };
    static absl::StatusOr<PrepareListenerSocketResult> PrepareListenerSocket(
        SOCKET sock, const EventEngine::ResolvedAddress& addr);

    void OnAcceptCallbackLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(io_state_->mu);

    // The cached AcceptEx for that port.
    LPFN_ACCEPTEX AcceptEx;
    // Buffer to hold the local and remote address.
    // This seemingly magic number comes from AcceptEx's documentation. each
    // address buffer needs to have at least 16 more bytes at their end.
#ifdef GRPC_HAVE_UNIX_SOCKET
    // unix addr is larger than ip addr.
    uint8_t addresses_[(sizeof(sockaddr_un) + 16) * 2] = {};
#else
    uint8_t addresses_[(sizeof(sockaddr_in6) + 16) * 2] = {};
#endif
    // The parent listener
    WindowsEventEngineListener* listener_;
    // shared state for asynchronous cleanup of overlapped operations
    std::shared_ptr<AsyncIOState> io_state_;
    // The actual TCP port number.
    int port_;
    EventEngine::ResolvedAddress listener_sockname_;
  };
  absl::StatusOr<SinglePortSocketListener*> AddSinglePortSocketListener(
      SOCKET sock, EventEngine::ResolvedAddress addr);

  IOCP* const iocp_;
  const EndpointConfig& config_;
  std::shared_ptr<EventEngine> engine_;
  ThreadPool* thread_pool_;
  const std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory_;
  AcceptCallback accept_cb_;
  absl::AnyInvocable<void(absl::Status)> on_shutdown_;
  std::atomic<bool> started_{false};
  grpc_core::Mutex port_listeners_mu_;
  std::list<std::unique_ptr<SinglePortSocketListener>> port_listeners_
      ABSL_GUARDED_BY(port_listeners_mu_);
  bool listeners_shutdown_ ABSL_GUARDED_BY(port_listeners_mu_) = false;
};

}  // namespace grpc_event_engine::experimental

#endif

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_WINDOWS_WINDOWS_LISTENER_H
