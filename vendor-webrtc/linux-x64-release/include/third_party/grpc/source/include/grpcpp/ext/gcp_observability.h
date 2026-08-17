//
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
//

#ifndef GRPCPP_EXT_GCP_OBSERVABILITY_H
#define GRPCPP_EXT_GCP_OBSERVABILITY_H

#include <grpc/support/port_platform.h>
#include <grpcpp/impl/grpc_library.h>

#include "absl/status/status.h"
#include "absl/status/statusor.h"

namespace grpc {

// GcpObservability objects follow the RAII idiom and help manage the lifetime
// of gRPC Observability data exporting to GCP. `GcpObservability::Init()`
// should be invoked instead to return an `GcpObservability` instance.
// Observability data is flushed at regular intervals, and also when this
// instance goes out of scope and its destructor is invoked.
class GcpObservability {
 public:
  // Initialize GCP Observability for gRPC.
  // This should be called before any other gRPC operations like creating a
  // channel, server, credentials etc.
  // The return value helps determine whether observability was
  // successfully enabled or not. On success, an object of class `Observability`
  // is returned. When this object goes out of scope, GCP Observability stats,
  // tracing and logging data is flushed. On failure, the status message can be
  // used to determine the cause of failure. It is up to the applications to
  // either crash on failure, or continue without GCP observability being
  // enabled. The status codes do not have any special meaning at present, and
  // users should not make any assumptions based on the status code, other than
  // a non-OK status code meaning that observability initialization failed.
  //
  // The expected usage is to call this at the top (or near the top) in
  // main(), and let it go out of scope after all RPCs and activities that we
  // want to observe are done. Please look at
  // https://github.com/grpc/grpc/blob/master/examples/cpp/gcp_observability/helloworld/greeter_client.cc
  // and
  // https://github.com/grpc/grpc/blob/master/examples/cpp/gcp_observability/helloworld/greeter_server.cc
  // for sample usage.
  //
  // It is possible for an initialized GcpObservability object to go out of
  // scope while RPCs and other gRPC operations are still ongoing. In this case,
  // GCP Observability tries to flush all observability data collected till that
  // point.
  //
  // Note that this is a blocking call which properly sets up gRPC Observability
  // to work with GCP and might take a few seconds to return.  Similarly, the
  // destruction of a non-moved-from `Observability` object is also blocking
  // since it flushes the observability data to GCP.
  //
  // As an implementation detail, this properly initializes the OpenCensus stats
  // and tracing plugin, so applications do not need to perform any additional
  // gRPC C++ OpenCensus setup/registration to get GCP Observability for gRPC.
  static absl::StatusOr<GcpObservability> Init();

  GcpObservability() = default;
  // Move constructor and Move-assignment operator.
  // The moved-from object will no longer be valid and will not cause GCP
  // Observability stats, tracing and logging data to flush.
  GcpObservability(GcpObservability&& other) noexcept;
  GcpObservability& operator=(GcpObservability&& other) noexcept;

  // Delete copy and copy-assignment operator
  GcpObservability(const GcpObservability&) = delete;
  GcpObservability& operator=(const GcpObservability&) = delete;

 private:
  // Helper class that aids in implementing GCP Observability.
  // Inheriting from GrpcLibrary makes sure that gRPC is initialized and remains
  // initialized for the lifetime of GCP Observability. In the future, when gRPC
  // initialization goes away, we might still want to keep gRPC Event Engine
  // initialized, just in case, we need to perform some IO operations during
  // observability close.
  // Note that the lifetime guarantees are only one way, i.e., GcpObservability
  // object guarantees that gRPC will not shutdown while the object is still in
  // scope, but the other way around does not hold true. Even though that is not
  // the expected usage, GCP Observability can shutdown before gRPC shuts down.
  // It follows that gRPC should not hold any callbacks from GcpObservability. A
  // change in this restriction should go through a design review.
  class GcpObservabilityImpl : private internal::GrpcLibrary {
   public:
    ~GcpObservabilityImpl() override;
  };
  std::unique_ptr<GcpObservabilityImpl> impl_;
};

namespace experimental {
// TODO(yashykt): Delete this after the 1.55 release.
GRPC_DEPRECATED("Use grpc::GcpObservability::Init() instead.")
absl::Status GcpObservabilityInit();
GRPC_DEPRECATED("Use grpc::GcpObservability::Init() instead.")
void GcpObservabilityClose();
}  // namespace experimental

}  // namespace grpc

#endif  // GRPCPP_EXT_GCP_OBSERVABILITY_H
