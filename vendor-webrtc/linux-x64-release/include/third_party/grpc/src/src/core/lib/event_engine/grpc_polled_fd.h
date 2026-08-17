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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_GRPC_POLLED_FD_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_GRPC_POLLED_FD_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include <memory>

#if GRPC_ARES == 1

#include <ares.h>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "src/core/util/sync.h"

namespace grpc_event_engine::experimental {

// A wrapped fd that integrates with the EventEngine poller of the current
// platform. A GrpcPolledFd knows how to create grpc platform-specific poller
// handle from "ares_socket_t" sockets, and then sign up for
// readability/writeability with that poller handle, and do shutdown and
// destruction.
class GrpcPolledFd {
 public:
  virtual ~GrpcPolledFd() {}
  // Called when c-ares library is interested and there's no pending callback
  virtual void RegisterForOnReadableLocked(
      absl::AnyInvocable<void(absl::Status)> read_closure) = 0;
  // Called when c-ares library is interested and there's no pending callback
  virtual void RegisterForOnWriteableLocked(
      absl::AnyInvocable<void(absl::Status)> write_closure) = 0;
  // Indicates if there is data left even after just being read from
  virtual bool IsFdStillReadableLocked() = 0;
  // Called once and only once. Must cause cancellation of any pending
  // read/write callbacks. Return true when the Shutdown is confirmed, false
  // otherwise.
  //
  // TODO(yijiem): On Posix, ShutdownLocked will always succeed. On Windows,
  // ShutdownLocked only succeeds when error is Cancelled. We could remove these
  // requirements if we changed the FdNode lifetime model so that:
  //   1. FdNodes and their underlying socket handles remain alive for
  //      the lifetime of the resolver.
  //   2. Orphaning the resolver triggers shutdown and subsequent cleanup for
  //      all FdNodes and socket handles.
  GRPC_MUST_USE_RESULT virtual bool ShutdownLocked(absl::Status error) = 0;
  // Get the underlying ares_socket_t that this was created from
  virtual ares_socket_t GetWrappedAresSocketLocked() = 0;
  // A unique name, for logging
  virtual const char* GetName() const = 0;
};

// A GrpcPolledFdFactory is 1-to-1 with and owned by a GrpcAresRequest. It knows
// how to create GrpcPolledFd's for the current platform, and the
// GrpcAresRequest uses it for all of its fd's.
class GrpcPolledFdFactory {
 public:
  virtual ~GrpcPolledFdFactory() {}
  // Optionally initializes the GrpcPolledFdFactory with a grpc_core::Mutex*
  // for synchronization between the AresResolver and the GrpcPolledFds. The
  // Windows implementation overrides this.
  virtual void Initialize(grpc_core::Mutex* mutex,
                          EventEngine* event_engine) = 0;
  // Creates a new wrapped fd for the current platform
  virtual std::unique_ptr<GrpcPolledFd> NewGrpcPolledFdLocked(
      ares_socket_t as) = 0;
  // Optionally configures the ares channel after creation
  virtual void ConfigureAresChannelLocked(ares_channel channel) = 0;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_ARES == 1
#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_GRPC_POLLED_FD_H
