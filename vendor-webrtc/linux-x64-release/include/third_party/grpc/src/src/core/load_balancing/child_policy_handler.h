//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_CHILD_POLICY_HANDLER_H
#define GRPC_SRC_CORE_LOAD_BALANCING_CHILD_POLICY_HANDLER_H
#include <grpc/support/port_platform.h>

#include <utility>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/load_balancing/lb_policy.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

// A class that makes it easy to gracefully switch child policies.
//
// Callers should instantiate this instead of using
// CoreConfiguration::Get().lb_policy_registry().CreateLoadBalancingPolicy().
// Once instantiated, this object will automatically take care of constructing
// the child policy as needed upon receiving an update.
class ChildPolicyHandler : public LoadBalancingPolicy {
 public:
  ChildPolicyHandler(Args args, TraceFlag* tracer)
      : LoadBalancingPolicy(std::move(args)), tracer_(tracer) {}

  absl::string_view name() const override { return "child_policy_handler"; }

  absl::Status UpdateLocked(UpdateArgs args) override;
  void ExitIdleLocked() override;
  void ResetBackoffLocked() override;

  // Returns true if transitioning from the old config to the new config
  // requires instantiating a new policy object.
  virtual bool ConfigChangeRequiresNewPolicyInstance(
      LoadBalancingPolicy::Config* old_config,
      LoadBalancingPolicy::Config* new_config) const;

  // Instantiates a new policy of the specified name.
  // May be overridden by subclasses to avoid recursion when an LB
  // policy factory returns a ChildPolicyHandler.
  virtual OrphanablePtr<LoadBalancingPolicy> CreateLoadBalancingPolicy(
      absl::string_view name, LoadBalancingPolicy::Args args) const;

 private:
  class Helper;

  void ShutdownLocked() override;

  OrphanablePtr<LoadBalancingPolicy> CreateChildPolicy(
      absl::string_view child_policy_name, const ChannelArgs& args);

  // Passed in from caller at construction time.
  TraceFlag* tracer_;

  bool shutting_down_ = false;

  // The most recent config passed to UpdateLocked().
  // If pending_child_policy_ is non-null, this is the config passed to
  // pending_child_policy_; otherwise, it's the config passed to child_policy_.
  RefCountedPtr<LoadBalancingPolicy::Config> current_config_;

  // Child LB policy.
  OrphanablePtr<LoadBalancingPolicy> child_policy_;
  OrphanablePtr<LoadBalancingPolicy> pending_child_policy_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_CHILD_POLICY_HANDLER_H
