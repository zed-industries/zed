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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_CEL_EXPRESSION_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_CEL_EXPRESSION_H

#include <grpc/support/port_platform.h>

#include <memory>
#include <vector>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "google/api/expr/v1alpha1/syntax.upb.h"
#include "src/core/lib/security/authorization/mock_cel/activation.h"
#include "src/core/lib/security/authorization/mock_cel/cel_value.h"

namespace grpc_core {
namespace mock_cel {

// This is a temporary stub implementation of CEL APIs.
// Once gRPC imports the CEL library, this file will be removed.

// Base interface for expression evaluating objects.
class CelExpression {
 public:
  virtual ~CelExpression() = default;

  // Evaluates expression and returns value.
  // activation contains bindings from parameter names to values
  virtual absl::StatusOr<CelValue> Evaluate(
      const BaseActivation& activation) const = 0;
};

// Base class for Expression Builder implementations
// Provides user with factory to register extension functions.
// ExpressionBuilder MUST NOT be destroyed before CelExpression objects
// it built.
class CelExpressionBuilder {
 public:
  virtual ~CelExpressionBuilder() = default;

  // Creates CelExpression object from AST tree.
  // expr specifies root of AST tree
  virtual absl::StatusOr<std::unique_ptr<CelExpression>> CreateExpression(
      const google_api_expr_v1alpha1_Expr* expr,
      const google_api_expr_v1alpha1_SourceInfo* source_info) const = 0;

  virtual absl::StatusOr<std::unique_ptr<CelExpression>> CreateExpression(
      const google_api_expr_v1alpha1_Expr* expr,
      const google_api_expr_v1alpha1_SourceInfo* source_info,
      std::vector<absl::Status>* warnings) const = 0;
};

}  // namespace mock_cel
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_CEL_EXPRESSION_H
