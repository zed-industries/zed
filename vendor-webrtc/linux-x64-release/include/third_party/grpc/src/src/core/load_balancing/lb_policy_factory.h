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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_LB_POLICY_FACTORY_H
#define GRPC_SRC_CORE_LOAD_BALANCING_LB_POLICY_FACTORY_H

#include <grpc/support/port_platform.h>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/load_balancing/lb_policy.h"
#include "src/core/util/json/json.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

class LoadBalancingPolicyFactory {
 public:
  virtual ~LoadBalancingPolicyFactory() {}

  /// Returns a new LB policy instance.
  virtual OrphanablePtr<LoadBalancingPolicy> CreateLoadBalancingPolicy(
      LoadBalancingPolicy::Args) const = 0;

  /// Returns the LB policy name that this factory provides.
  virtual absl::string_view name() const = 0;

  virtual absl::StatusOr<RefCountedPtr<LoadBalancingPolicy::Config>>
  ParseLoadBalancingConfig(const Json& json) const = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_LB_POLICY_FACTORY_H
