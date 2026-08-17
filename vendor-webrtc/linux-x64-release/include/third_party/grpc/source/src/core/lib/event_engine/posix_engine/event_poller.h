// Copyright 2022 The gRPC Authors
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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_EVENT_POLLER_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_EVENT_POLLER_H
#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <string>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/event_engine/forkable.h"
#include "src/core/lib/event_engine/poller.h"
#include "src/core/lib/event_engine/posix_engine/posix_engine_closure.h"

namespace grpc_event_engine::experimental {

class Scheduler {
 public:
  virtual void Run(experimental::EventEngine::Closure* closure) = 0;
  virtual void Run(absl::AnyInvocable<void()>) = 0;
  virtual ~Scheduler() = default;
};

class PosixEventPoller;

class EventHandle {
 public:
  virtual int WrappedFd() = 0;
  // Delete the handle and optionally close the underlying file descriptor if
  // release_fd != nullptr. The on_done closure is scheduled to be invoked
  // after the operation is complete. After this operation, NotifyXXX and SetXXX
  // operations cannot be performed on the handle. In general, this method
  // should only be called after ShutdownHandle and after all existing NotifyXXX
  // closures have run and there is no waiting NotifyXXX closure.
  virtual void OrphanHandle(PosixEngineClosure* on_done, int* release_fd,
                            absl::string_view reason) = 0;
  // Shutdown a handle. If there is an attempt to call NotifyXXX operations
  // after Shutdown handle, those closures will be run immediately with the
  // absl::Status provided here being passed to the callbacks enclosed within
  // the PosixEngineClosure object.
  virtual void ShutdownHandle(absl::Status why) = 0;
  // Schedule on_read to be invoked when the underlying file descriptor
  // becomes readable. When the on_read closure is run, it may check
  // if the handle is shutdown using the IsHandleShutdown method and take
  // appropriate actions (for instance it should not try to invoke another
  // recursive NotifyOnRead if the handle is shutdown).
  virtual void NotifyOnRead(PosixEngineClosure* on_read) = 0;
  // Schedule on_write to be invoked when the underlying file descriptor
  // becomes writable. When the on_write closure is run, it may check
  // if the handle is shutdown using the IsHandleShutdown method and take
  // appropriate actions (for instance it should not try to invoke another
  // recursive NotifyOnWrite if the handle is shutdown).
  virtual void NotifyOnWrite(PosixEngineClosure* on_write) = 0;
  // Schedule on_error to be invoked when the underlying file descriptor
  // encounters errors. When the on_error closure is run, it may check
  // if the handle is shutdown using the IsHandleShutdown method and take
  // appropriate actions (for instance it should not try to invoke another
  // recursive NotifyOnError if the handle is shutdown).
  virtual void NotifyOnError(PosixEngineClosure* on_error) = 0;
  // Force set a readable event on the underlying file descriptor.
  virtual void SetReadable() = 0;
  // Force set a writable event on the underlying file descriptor.
  virtual void SetWritable() = 0;
  // Force set a error event on the underlying file descriptor.
  virtual void SetHasError() = 0;
  // Returns true if the handle has been shutdown.
  virtual bool IsHandleShutdown() = 0;
  // Returns the poller which was used to create this handle.
  virtual PosixEventPoller* Poller() = 0;
  virtual ~EventHandle() = default;
};

class PosixEventPoller : public grpc_event_engine::experimental::Poller,
                         public Forkable {
 public:
  // Return an opaque handle to perform actions on the provided file descriptor.
  virtual EventHandle* CreateHandle(int fd, absl::string_view name,
                                    bool track_err) = 0;
  virtual bool CanTrackErrors() const = 0;
  virtual std::string Name() = 0;
  // Shuts down and deletes the poller. It is legal to call this function
  // only when no other poller method is in progress. For instance, it is
  // not safe to call this method, while a thread is blocked on Work(...).
  // A graceful way to terminate the poller could be to:
  // 1. First orphan all created handles.
  // 2. Send a Kick() to the thread executing Work(...) and wait for the
  //    thread to return.
  // 3. Call Shutdown() on the poller.
  virtual void Shutdown() = 0;
  ~PosixEventPoller() override = default;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_EVENT_POLLER_H
