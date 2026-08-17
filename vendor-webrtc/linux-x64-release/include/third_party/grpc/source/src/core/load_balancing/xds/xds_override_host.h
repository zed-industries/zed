//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_XDS_XDS_OVERRIDE_HOST_H
#define GRPC_SRC_CORE_LOAD_BALANCING_XDS_XDS_OVERRIDE_HOST_H

#include <grpc/support/port_platform.h>

#include "absl/strings/string_view.h"
#include "src/core/load_balancing/lb_policy.h"
#include "src/core/util/json/json.h"
#include "src/core/util/json/json_args.h"
#include "src/core/util/json/json_object_loader.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/validation_errors.h"

namespace grpc_core {

// Config for stateful session LB policy.
class XdsOverrideHostLbConfig final : public LoadBalancingPolicy::Config {
 public:
  XdsOverrideHostLbConfig() = default;

  XdsOverrideHostLbConfig(const XdsOverrideHostLbConfig&) = delete;
  XdsOverrideHostLbConfig& operator=(const XdsOverrideHostLbConfig&) = delete;

  XdsOverrideHostLbConfig(XdsOverrideHostLbConfig&& other) = delete;
  XdsOverrideHostLbConfig& operator=(XdsOverrideHostLbConfig&& other) = delete;

  static absl::string_view Name() { return "xds_override_host_experimental"; }

  absl::string_view name() const override { return Name(); }

  const std::string& cluster_name() const { return cluster_name_; }
  RefCountedPtr<LoadBalancingPolicy::Config> child_config() const {
    return child_config_;
  }

  static const JsonLoaderInterface* JsonLoader(const JsonArgs&);
  void JsonPostLoad(const Json& json, const JsonArgs&,
                    ValidationErrors* errors);

 private:
  std::string cluster_name_;
  RefCountedPtr<LoadBalancingPolicy::Config> child_config_;
};

}  // namespace grpc_core
#endif  // GRPC_SRC_CORE_LOAD_BALANCING_XDS_XDS_OVERRIDE_HOST_H
