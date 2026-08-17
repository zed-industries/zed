//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_CLIENT_CHANNEL_INTERNAL_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_CLIENT_CHANNEL_INTERNAL_H

#include <grpc/support/port_platform.h>

#include <utility>

#include "absl/functional/any_invocable.h"
#include "absl/log/check.h"
#include "src/core/call/call_destination.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/load_balancing/lb_policy.h"
#include "src/core/service_config/service_config_call_data.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/util/down_cast.h"
#include "src/core/util/unique_type_name.h"

//
// This file contains internal interfaces used to allow various plugins
// (filters, LB policies, etc) to access internal data provided by the
// ClientChannelFilter that is not normally accessible via external APIs.
//

// Channel arg key for health check service name.
#define GRPC_ARG_HEALTH_CHECK_SERVICE_NAME \
  "grpc.internal.health_check_service_name"

namespace grpc_core {

// Internal type for LB call state interface.  Provides an interface for
// LB policies to access internal call attributes.
class ClientChannelLbCallState : public LoadBalancingPolicy::CallState {
 public:
  template <typename A>
  A* GetCallAttribute() const {
    return DownCast<A*>(GetCallAttribute(A::TypeName()));
  }

  virtual ServiceConfigCallData::CallAttributeInterface* GetCallAttribute(
      UniqueTypeName type) const = 0;
  virtual ClientCallTracer::CallAttemptTracer* GetCallAttemptTracer() const = 0;
};

// Internal type for ServiceConfigCallData.  Handles call commits.
class ClientChannelServiceConfigCallData final : public ServiceConfigCallData {
 public:
  explicit ClientChannelServiceConfigCallData(Arena* arena)
      : ServiceConfigCallData(arena) {}

  void SetOnCommit(absl::AnyInvocable<void()> on_commit) {
    CHECK(on_commit_ == nullptr);
    on_commit_ = std::move(on_commit);
  }

  void Commit() {
    auto on_commit = std::move(on_commit_);
    if (on_commit != nullptr) on_commit();
  }

 private:
  absl::AnyInvocable<void()> on_commit_;
};

template <>
struct ContextSubclass<ClientChannelServiceConfigCallData> {
  using Base = ServiceConfigCallData;
};

class SubchannelInterfaceWithCallDestination : public SubchannelInterface {
 public:
  using SubchannelInterface::SubchannelInterface;
  // Obtain the call destination for this subchannel.
  virtual RefCountedPtr<UnstartedCallDestination> call_destination() = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_CLIENT_CHANNEL_INTERNAL_H
