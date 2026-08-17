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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_WINDOWS_WIN_SOCKET_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_WINDOWS_WIN_SOCKET_H

#include <grpc/support/port_platform.h>

#ifdef GPR_WINDOWS

#include <grpc/event_engine/event_engine.h>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "src/core/lib/event_engine/thread_pool/thread_pool.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/sync.h"

namespace grpc_event_engine::experimental {

class WinSocket {
 public:
  struct OverlappedResult {
    int wsa_error;
    DWORD bytes_transferred;
    absl::Status error_status;
  };

  // State related to a Read or Write socket operation
  class OpState {
   public:
    explicit OpState(WinSocket* win_socket) noexcept;
    // Signal a result has returned
    // If a callback is already primed for notification, it will be executed via
    // the WinSocket's ThreadPool. Otherwise, a "pending iocp" flag will
    // be set.
    void SetReady();
    // Set WSA result for a completed op.
    // If the error is non-zero, bytes will be overridden to 0.
    void SetResult(int wsa_error, DWORD bytes, absl::string_view context);
    // Set error results for a completed op.
    // This is a manual override, meant to ignore any WSA status code.
    void SetErrorStatus(absl::Status error_status);
    // Retrieve the results of an overlapped operation (via Winsock API) and
    // store them locally.
    void GetOverlappedResult();
    // Retrieve the results of an overlapped operation (via Winsock API) and
    // store them locally. This overload allows acceptance of connections on new
    // sockets.
    void GetOverlappedResult(SOCKET sock);
    // Retrieve the cached result from GetOverlappedResult
    const OverlappedResult& result() const { return result_; }
    // OVERLAPPED, needed for Winsock API calls
    LPOVERLAPPED overlapped() { return &overlapped_; }

   private:
    friend class WinSocket;

    OVERLAPPED overlapped_;
    WinSocket* win_socket_ = nullptr;
    EventEngine::Closure* closure_ = nullptr;
    OverlappedResult result_;
  };

  WinSocket(SOCKET socket, ThreadPool* thread_pool) noexcept;
  ~WinSocket();
  // Provide a closure that will be called when an IOCP completion has occurred.
  //
  // Notification callbacks *must be registered* before any WSASend or WSARecv
  // operations are started. Only one closure can be registered at a time for
  // each read or send operation.
  void NotifyOnRead(EventEngine::Closure* on_read);
  void NotifyOnWrite(EventEngine::Closure* on_write);
  // Remove the notification callback for read/write events.
  //
  // This method should only be called if no IOCP event is pending for the
  // socket. It is UB if an IOCP event comes through and a notification is not
  // registered.
  void UnregisterReadCallback();
  void UnregisterWriteCallback();

  bool IsShutdown();
  // Shutdown socket operations, but do not delete the WinSocket.
  // Connections will be disconnected, and the socket will be closed.
  // If the socket is managed by a shared_ptr (most should be), then the
  // WinSocket will be deleted when the last outstanding overlapped event comes
  // back.
  void Shutdown();
  void Shutdown(const grpc_core::DebugLocation& location,
                absl::string_view reason);

  // Return the appropriate OpState for a given OVERLAPPED
  // Returns nullptr if the overlapped does not match either read or write ops.
  OpState* GetOpInfoForOverlapped(OVERLAPPED* overlapped);
  // Getters for the operation state data.
  OpState* read_info() { return &read_info_; }
  OpState* write_info() { return &write_info_; }
  // Accessor method for underlying socket
  SOCKET raw_socket();

 private:
  void NotifyOnReady(OpState& info, EventEngine::Closure* closure);

  SOCKET socket_;
  std::atomic<bool> is_shutdown_{false};
  ThreadPool* thread_pool_;
  // These OpStates are effectively synchronized using their respective
  // OVERLAPPED structures and the Overlapped I/O APIs. For example, OpState
  // users should not attempt to read their bytes_transeferred until
  // GetOverlappedResult has returned, to ensure there are no two threads
  // reading and writing the same values concurrently.
  //
  // Callers must also ensure that at most one read and write operation are
  // occurring at a time. Attempting to do multiple concurrent reads/writes will
  // have undefined behavior.
  OpState read_info_;
  OpState write_info_;
};

// Attempt to configure default socket settings
absl::Status PrepareSocket(SOCKET sock);

// Set non block option for socket.
absl::Status SetSocketNonBlock(SOCKET sock);

// Get the local address of a socket.
absl::StatusOr<EventEngine::ResolvedAddress> SocketToAddress(SOCKET sock);

}  // namespace grpc_event_engine::experimental

#endif

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_WINDOWS_WIN_SOCKET_H
