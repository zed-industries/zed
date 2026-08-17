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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENGINE_LISTENER_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENGINE_LISTENER_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/event_engine/slice_buffer.h>
#include <grpc/support/port_platform.h>
#include <string.h>

#include <atomic>
#include <list>
#include <memory>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/lib/event_engine/posix.h"
#include "src/core/lib/iomgr/port.h"
#include "src/core/util/sync.h"

#ifdef GRPC_POSIX_SOCKET_TCP
#include "src/core/lib/event_engine/posix_engine/event_poller.h"
#include "src/core/lib/event_engine/posix_engine/posix_engine_closure.h"
#include "src/core/lib/event_engine/posix_engine/posix_engine_listener_utils.h"
#include "src/core/lib/event_engine/posix_engine/tcp_socket_utils.h"
#include "src/core/lib/event_engine/tcp_socket_utils.h"
#endif

namespace grpc_event_engine::experimental {

#ifdef GRPC_POSIX_SOCKET_TCP
class PosixEngineListenerImpl
    : public std::enable_shared_from_this<PosixEngineListenerImpl> {
 public:
  PosixEngineListenerImpl(
      PosixEventEngineWithFdSupport::PosixAcceptCallback on_accept,
      absl::AnyInvocable<void(absl::Status)> on_shutdown,
      const grpc_event_engine::experimental::EndpointConfig& config,
      std::unique_ptr<grpc_event_engine::experimental::MemoryAllocatorFactory>
          memory_allocator_factory,
      PosixEventPoller* poller, std::shared_ptr<EventEngine> engine);
  // Binds an address to the listener. This creates a ListenerSocket
  // and sets its fields appropriately.
  absl::StatusOr<int> Bind(
      const EventEngine::ResolvedAddress& addr,
      PosixListenerWithFdSupport::OnPosixBindNewFdCallback on_bind_new_fd);
  // Signals event manager to listen for connections on all created sockets.
  absl::Status Start();
  // Trigger graceful shutdown of all asynchronous accept operations.
  void TriggerShutdown();

  absl::Status HandleExternalConnection(int listener_fd, int fd,
                                        SliceBuffer* pending_data);

  ~PosixEngineListenerImpl();

 private:
  // This class represents accepting for one bind fd belonging to the listener.
  // Each AsyncConnectionAcceptor takes a ref to the parent
  // PosixEngineListenerImpl object. So the PosixEngineListenerImpl can be
  // deleted only after all AsyncConnectionAcceptors get destroyed.
  class AsyncConnectionAcceptor {
   public:
    AsyncConnectionAcceptor(std::shared_ptr<EventEngine> engine,
                            std::shared_ptr<PosixEngineListenerImpl> listener,
                            ListenerSocketsContainer::ListenerSocket socket)
        : engine_(std::move(engine)),
          listener_(std::move(listener)),
          socket_(socket),
          handle_(listener_->poller_->CreateHandle(
              socket_.sock.Fd(),
              *grpc_event_engine::experimental::
                  ResolvedAddressToNormalizedString(socket_.addr),
              listener_->poller_->CanTrackErrors())),
          notify_on_accept_(PosixEngineClosure::ToPermanentClosure(
              [this](absl::Status status) { NotifyOnAccept(status); })) {};
    // Start listening for incoming connections on the socket.
    void Start();
    // Internal callback invoked when the socket has incoming connections to
    // process.
    void NotifyOnAccept(absl::Status status);
    // Shutdown the poller handle associated with this socket.
    void Shutdown();
    void Ref() { ref_count_.fetch_add(1, std::memory_order_relaxed); }
    void Unref() {
      if (ref_count_.fetch_sub(1, std::memory_order_acq_rel) == 1) {
        delete this;
      }
    }
    ListenerSocketsContainer::ListenerSocket& Socket() { return socket_; }
    ~AsyncConnectionAcceptor() {
      auto address = socket_.sock.LocalAddress();
      if (address.ok()) {
        // If uds socket, unlink it so that the corresponding file is deleted.
        UnlinkIfUnixDomainSocket(*address);
      }
      handle_->OrphanHandle(nullptr, nullptr, "");
      delete notify_on_accept_;
    }

   private:
    std::atomic<int> ref_count_{1};
    std::shared_ptr<EventEngine> engine_;
    std::shared_ptr<PosixEngineListenerImpl> listener_;
    ListenerSocketsContainer::ListenerSocket socket_;
    EventHandle* handle_;
    PosixEngineClosure* notify_on_accept_;
    // Tracks the status of a backup timer to retry accept4 calls after file
    // descriptor exhaustion.
    std::atomic<bool> retry_timer_armed_{false};
  };
  class ListenerAsyncAcceptors : public ListenerSocketsContainer {
   public:
    explicit ListenerAsyncAcceptors(PosixEngineListenerImpl* listener)
        : listener_(listener) {};

    void UpdateOnAppendCallback(
        PosixListenerWithFdSupport::OnPosixBindNewFdCallback on_append) {
      on_append_ = std::move(on_append);
    }

    void Append(ListenerSocket socket) override {
      acceptors_.push_back(new AsyncConnectionAcceptor(
          listener_->engine_, listener_->shared_from_this(), socket));
      if (on_append_) {
        on_append_(socket.sock.Fd());
      }
    }

    absl::StatusOr<ListenerSocket> Find(
        const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
            addr) override {
      for (auto* acceptor : acceptors_) {
        if (acceptor->Socket().addr.size() == addr.size() &&
            memcmp(acceptor->Socket().addr.address(), addr.address(),
                   addr.size()) == 0) {
          return acceptor->Socket();
        }
      }
      return absl::NotFoundError("Socket not found!");
    }

    int Size() { return static_cast<int>(acceptors_.size()); }

    std::list<AsyncConnectionAcceptor*>::const_iterator begin() {
      return acceptors_.begin();
    }
    std::list<AsyncConnectionAcceptor*>::const_iterator end() {
      return acceptors_.end();
    }

   private:
    PosixListenerWithFdSupport::OnPosixBindNewFdCallback on_append_;
    std::list<AsyncConnectionAcceptor*> acceptors_;
    PosixEngineListenerImpl* listener_;
  };
  friend class ListenerAsyncAcceptors;
  friend class AsyncConnectionAcceptor;
  // The mutex ensures thread safety when multiple threads try to call Bind
  // and Start in parallel.
  grpc_core::Mutex mu_;
  PosixEventPoller* poller_;
  PosixTcpOptions options_;
  std::shared_ptr<EventEngine> engine_;
  // Linked list of sockets. One is created upon each successful bind
  // operation.
  ListenerAsyncAcceptors acceptors_ ABSL_GUARDED_BY(mu_);
  // Callback to be invoked upon accepting a connection.
  PosixEventEngineWithFdSupport::PosixAcceptCallback on_accept_;
  // Callback to be invoked upon shutdown of listener.
  absl::AnyInvocable<void(absl::Status)> on_shutdown_;
  // Set to true when the listener has started listening for new connections.
  // Any further bind operations would fail.
  bool started_ ABSL_GUARDED_BY(mu_) = false;
  // Pointer to a slice allocator factory object which can generate
  // unique slice allocators for each new incoming connection.
  std::unique_ptr<grpc_event_engine::experimental::MemoryAllocatorFactory>
      memory_allocator_factory_;
};

class PosixEngineListener : public PosixListenerWithFdSupport {
 public:
  PosixEngineListener(
      PosixEventEngineWithFdSupport::PosixAcceptCallback on_accept,
      absl::AnyInvocable<void(absl::Status)> on_shutdown,
      const grpc_event_engine::experimental::EndpointConfig& config,
      std::unique_ptr<grpc_event_engine::experimental::MemoryAllocatorFactory>
          memory_allocator_factory,
      PosixEventPoller* poller, std::shared_ptr<EventEngine> engine)
      : impl_(std::make_shared<PosixEngineListenerImpl>(
            std::move(on_accept), std::move(on_shutdown), config,
            std::move(memory_allocator_factory), poller, std::move(engine))) {}
  ~PosixEngineListener() override { ShutdownListeningFds(); };
  absl::StatusOr<int> Bind(
      const grpc_event_engine::experimental::EventEngine::ResolvedAddress& addr)
      override {
    return impl_->Bind(addr, nullptr);
  }
  absl::StatusOr<int> BindWithFd(
      const EventEngine::ResolvedAddress& addr,
      PosixListenerWithFdSupport::OnPosixBindNewFdCallback on_bind_new_fd)
      override {
    return impl_->Bind(addr, std::move(on_bind_new_fd));
  }
  absl::Status HandleExternalConnection(int listener_fd, int fd,
                                        SliceBuffer* pending_data) override {
    return impl_->HandleExternalConnection(listener_fd, fd, pending_data);
  }
  absl::Status Start() override { return impl_->Start(); }

  void ShutdownListeningFds() override {
    if (!shutdown_.exchange(true, std::memory_order_acq_rel)) {
      impl_->TriggerShutdown();
    }
  }

 private:
  std::shared_ptr<PosixEngineListenerImpl> impl_;
  // Set to true when the listener had been explicitly shutdown.
  std::atomic<bool> shutdown_{false};
};

#else  // GRPC_POSIX_SOCKET_TCP

#include "src/core/util/crash.h"

class PosixEngineListener : public PosixListenerWithFdSupport {
 public:
  PosixEngineListener() = default;
  ~PosixEngineListener() override = default;
  absl::StatusOr<int> Bind(const grpc_event_engine::experimental::EventEngine::
                               ResolvedAddress& /*addr*/) override {
    grpc_core::Crash(
        "EventEngine::Listener::Bind not supported on this platform");
  }
  absl::Status Start() override {
    grpc_core::Crash(
        "EventEngine::Listener::Start not supported on this platform");
  }
  absl::StatusOr<int> BindWithFd(
      const EventEngine::ResolvedAddress& /*addr*/,
      PosixListenerWithFdSupport::OnPosixBindNewFdCallback
      /*on_bind_new_fd*/) override {
    grpc_core::Crash(
        "PosixEngineListener::BindWithFd not supported on this "
        "platform");
  }
  absl::Status HandleExternalConnection(
      int /*listener_fd*/, int /*fd*/, SliceBuffer* /*pending_data*/) override {
    grpc_core::Crash(
        "PosixEngineListener::HandleExternalConnection not "
        "supported on this platform");
  }
  void ShutdownListeningFds() override {
    grpc_core::Crash(
        "PosixEngineListener::ShutdownListeningFds not supported on "
        "this platform");
  }
};

#endif

}  // namespace grpc_event_engine::experimental
#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENGINE_LISTENER_H
