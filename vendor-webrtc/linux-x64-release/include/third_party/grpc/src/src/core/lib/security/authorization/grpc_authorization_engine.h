// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_GRPC_AUTHORIZATION_ENGINE_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_GRPC_AUTHORIZATION_ENGINE_H

#include <grpc/grpc_audit_logging.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <memory>
#include <string>
#include <vector>

#include "src/core/lib/security/authorization/authorization_engine.h"
#include "src/core/lib/security/authorization/evaluate_args.h"
#include "src/core/lib/security/authorization/matchers.h"
#include "src/core/lib/security/authorization/rbac_policy.h"

namespace grpc_core {

using experimental::AuditLogger;

// GrpcAuthorizationEngine can be either an Allow engine or Deny engine. This
// engine makes authorization decisions to Allow or Deny incoming RPC request
// based on permission and principal configs in the provided RBAC policy and the
// engine type. This engine ignores condition field in RBAC config. It is the
// caller's responsibility to provide RBAC policies that are compatible with
// this engine.
class GrpcAuthorizationEngine : public AuthorizationEngine {
 public:
  // Builds GrpcAuthorizationEngine without any policies.
  explicit GrpcAuthorizationEngine(Rbac::Action action)
      : action_(action), audit_condition_(Rbac::AuditCondition::kNone) {}
  // Builds GrpcAuthorizationEngine with allow/deny RBAC policy.
  explicit GrpcAuthorizationEngine(Rbac policy);

  GrpcAuthorizationEngine(GrpcAuthorizationEngine&& other) noexcept;
  GrpcAuthorizationEngine& operator=(GrpcAuthorizationEngine&& other) noexcept;

  Rbac::Action action() const { return action_; }

  // Required only for testing purpose.
  size_t num_policies() const { return policies_.size(); }

  // Required only for testing purpose.
  Rbac::AuditCondition audit_condition() const { return audit_condition_; }

  // Required only for testing purpose.
  const std::vector<std::unique_ptr<AuditLogger>>& audit_loggers() const {
    return audit_loggers_;
  }

  // Evaluates incoming request against RBAC policy and makes a decision to
  // whether allow/deny this request.
  Decision Evaluate(const EvaluateArgs& args) const override;

 private:
  struct Policy {
    std::string name;
    std::unique_ptr<AuthorizationMatcher> matcher;
  };

  std::string name_;
  Rbac::Action action_;
  std::vector<Policy> policies_;
  Rbac::AuditCondition audit_condition_;
  std::vector<std::unique_ptr<AuditLogger>> audit_loggers_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_GRPC_AUTHORIZATION_ENGINE_H
