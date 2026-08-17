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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_FLAT_EXPR_BUILDER_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_FLAT_EXPR_BUILDER_H

#include <grpc/support/port_platform.h>

#include <memory>
#include <set>
#include <string>
#include <vector>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "google/api/expr/v1alpha1/syntax.upb.h"
#include "src/core/lib/security/authorization/mock_cel/cel_expression.h"
#include "src/core/lib/security/authorization/mock_cel/evaluator_core.h"

namespace grpc_core {
namespace mock_cel {

// This is a temporary stub implementation of CEL APIs.
// Once gRPC imports the CEL library, this file will be removed.

// CelExpressionBuilder implementation.
// Builds instances of CelExpressionFlatImpl.
class FlatExprBuilder : public CelExpressionBuilder {
 public:
  FlatExprBuilder() = default;

  absl::StatusOr<std::unique_ptr<CelExpression>> CreateExpression(
      const google_api_expr_v1alpha1_Expr* expr,
      const google_api_expr_v1alpha1_SourceInfo* source_info) const override {
    ExecutionPath path;
    return std::make_unique<CelExpressionFlatImpl>(nullptr, path, 0,
                                                   std::set<std::string>{});
  }

  absl::StatusOr<std::unique_ptr<CelExpression>> CreateExpression(
      const google_api_expr_v1alpha1_Expr* expr,
      const google_api_expr_v1alpha1_SourceInfo* source_info,
      std::vector<absl::Status>* warnings) const override {
    ExecutionPath path;
    return std::make_unique<CelExpressionFlatImpl>(nullptr, path, 0,
                                                   std::set<std::string>{});
  }
};

}  // namespace mock_cel
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_FLAT_EXPR_BUILDER_H
