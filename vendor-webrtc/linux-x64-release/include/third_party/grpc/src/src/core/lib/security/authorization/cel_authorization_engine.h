
// Copyright 2020 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_CEL_AUTHORIZATION_ENGINE_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_CEL_AUTHORIZATION_ENGINE_H

#include <grpc/support/port_platform.h>

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "absl/container/flat_hash_set.h"
#include "envoy/config/rbac/v3/rbac.upb.h"
#include "google/api/expr/v1alpha1/syntax.upb.h"
#include "src/core/lib/security/authorization/evaluate_args.h"
#include "src/core/lib/security/authorization/mock_cel/activation.h"
#include "src/core/lib/security/authorization/mock_cel/cel_value.h"
#include "upb/mem/arena.hpp"

namespace grpc_core {

// CelAuthorizationEngine makes an AuthorizationDecision to ALLOW or DENY the
// current action based on the condition fields in provided RBAC policies.
// The engine may be constructed with one or two policies. If two policies,
// the first policy is deny-if-matched and the second is allow-if-matched.
// The engine returns UNDECIDED decision if it fails to find a match in any
// policy. This engine ignores the principal and permission fields in RBAC
// policies. It is the caller's responsibility to provide RBAC policies that
// are compatible with this engine.
//
// Example:
// CelAuthorizationEngine* engine =
// CelAuthorizationEngine::CreateCelAuthorizationEngine(rbac_policies);
// engine->Evaluate(evaluate_args); // returns authorization decision.
class CelAuthorizationEngine {
 public:
  // rbac_policies must be a vector containing either a single policy of any
  // kind, or one deny policy and one allow policy, in that order.
  static std::unique_ptr<CelAuthorizationEngine> CreateCelAuthorizationEngine(
      const std::vector<envoy_config_rbac_v3_RBAC*>& rbac_policies);

  // Users should use the CreateCelAuthorizationEngine factory function
  // instead of calling the CelAuthorizationEngine constructor directly.
  explicit CelAuthorizationEngine(
      const std::vector<envoy_config_rbac_v3_RBAC*>& rbac_policies);
  // TODO(mywang@google.com): add an Evaluate member function.

 private:
  enum Action {
    kAllow,
    kDeny,
  };

  std::unique_ptr<mock_cel::Activation> CreateActivation(
      const EvaluateArgs& args);

  std::map<const std::string, const google_api_expr_v1alpha1_Expr*>
      deny_if_matched_;
  std::map<const std::string, const google_api_expr_v1alpha1_Expr*>
      allow_if_matched_;
  upb::Arena arena_;
  absl::flat_hash_set<std::string> envoy_attributes_;
  absl::flat_hash_set<std::string> header_keys_;
  std::unique_ptr<mock_cel::CelMap> headers_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_CEL_AUTHORIZATION_ENGINE_H
