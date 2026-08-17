//
//
// Copyright 2016 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_RESOLVER_DNS_C_ARES_GRPC_ARES_EV_DRIVER_H
#define GRPC_SRC_CORE_RESOLVER_DNS_C_ARES_GRPC_ARES_EV_DRIVER_H

#include <ares.h>
#include <grpc/support/port_platform.h>

#include <memory>

#include "absl/base/thread_annotations.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/resolver/dns/c_ares/grpc_ares_wrapper.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// A wrapped fd that integrates with the grpc iomgr of the current platform.
// A GrpcPolledFd knows how to create grpc platform-specific iomgr endpoints
// from "ares_socket_t" sockets, and then sign up for readability/writeability
// with that poller, and do shutdown and destruction.
class GrpcPolledFd {
 public:
  virtual ~GrpcPolledFd() {}
  // Called when c-ares library is interested and there's no pending callback
  virtual void RegisterForOnReadableLocked(grpc_closure* read_closure)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&grpc_ares_request::mu) = 0;
  // Called when c-ares library is interested and there's no pending callback
  virtual void RegisterForOnWriteableLocked(grpc_closure* write_closure)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&grpc_ares_request::mu) = 0;
  // Indicates if there is data left even after just being read from
  virtual bool IsFdStillReadableLocked()
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&grpc_ares_request::mu) = 0;
  // Called once and only once. Must cause cancellation of any pending
  // read/write callbacks.
  virtual void ShutdownLocked(grpc_error_handle error)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&grpc_ares_request::mu) = 0;
  // Get the underlying ares_socket_t that this was created from
  virtual ares_socket_t GetWrappedAresSocketLocked()
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&grpc_ares_request::mu) = 0;
  // A unique name, for logging
  virtual const char* GetName() const = 0;
};

// A GrpcPolledFdFactory is 1-to-1 with and owned by the
// ares event driver. It knows how to create GrpcPolledFd's
// for the current platform, and the ares driver uses it for all of
// its fd's.
class GrpcPolledFdFactory {
 public:
  virtual ~GrpcPolledFdFactory() {}
  // Creates a new wrapped fd for the current platform
  virtual GrpcPolledFd* NewGrpcPolledFdLocked(
      ares_socket_t as, grpc_pollset_set* driver_pollset_set)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&grpc_ares_request::mu) = 0;
  // Optionally configures the ares channel after creation
  virtual void ConfigureAresChannelLocked(ares_channel channel)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(&grpc_ares_request::mu) = 0;
};

// Creates a new polled fd factory.
// Note that even though ownership of mu is not transferred, the mu
// parameter is guaranteed to be alive for the the whole lifetime of
// the resulting GrpcPolledFdFactory as well as any GrpcPolledFd
// returned by the factory.
std::unique_ptr<GrpcPolledFdFactory> NewGrpcPolledFdFactory(Mutex* mu);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_RESOLVER_DNS_C_ARES_GRPC_ARES_EV_DRIVER_H
