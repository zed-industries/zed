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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_LB_POLICY_REGISTRY_H
#define GRPC_SRC_CORE_LOAD_BALANCING_LB_POLICY_REGISTRY_H

#include <grpc/support/port_platform.h>

#include <map>
#include <memory>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/load_balancing/lb_policy.h"
#include "src/core/load_balancing/lb_policy_factory.h"
#include "src/core/util/json/json.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

class LoadBalancingPolicyRegistry final {
 public:
  /// Methods used to create and populate the LoadBalancingPolicyRegistry.
  /// NOT THREAD SAFE -- to be used only during global gRPC
  /// initialization and shutdown.
  class Builder final {
   public:
    /// Registers an LB policy factory.  The factory will be used to create an
    /// LB policy whose name matches that of the factory.
    void RegisterLoadBalancingPolicyFactory(
        std::unique_ptr<LoadBalancingPolicyFactory> factory);

    LoadBalancingPolicyRegistry Build();

   private:
    std::map<absl::string_view, std::unique_ptr<LoadBalancingPolicyFactory>>
        factories_;
  };

  /// Creates an LB policy of the type specified by \a name.
  OrphanablePtr<LoadBalancingPolicy> CreateLoadBalancingPolicy(
      absl::string_view name, LoadBalancingPolicy::Args args) const;

  /// Returns true if the LB policy factory specified by \a name exists in this
  /// registry. If the load balancing policy requires a config to be specified
  /// then sets \a requires_config to true.
  bool LoadBalancingPolicyExists(absl::string_view name,
                                 bool* requires_config) const;

  /// Returns a parsed object of the load balancing policy to be used from a
  /// LoadBalancingConfig array \a json.
  absl::StatusOr<RefCountedPtr<LoadBalancingPolicy::Config>>
  ParseLoadBalancingConfig(const Json& json) const;

 private:
  LoadBalancingPolicyFactory* GetLoadBalancingPolicyFactory(
      absl::string_view name) const;
  absl::StatusOr<Json::Object::const_iterator> ParseLoadBalancingConfigHelper(
      const Json& lb_config_array) const;

  std::map<absl::string_view, std::unique_ptr<LoadBalancingPolicyFactory>>
      factories_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_LB_POLICY_REGISTRY_H
