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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_CEL_EXPR_BUILDER_FACTORY_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_CEL_EXPR_BUILDER_FACTORY_H

#include <grpc/support/port_platform.h>

#include <memory>

#include "src/core/lib/security/authorization/mock_cel/cel_expression.h"
#include "src/core/lib/security/authorization/mock_cel/flat_expr_builder.h"

namespace grpc_core {
namespace mock_cel {

// This is a temporary stub implementation of CEL APIs.
// Once gRPC imports the CEL library, this file will be removed.

struct InterpreterOptions {
  bool short_circuiting = true;
};

inline std::unique_ptr<CelExpressionBuilder> CreateCelExpressionBuilder(
    const InterpreterOptions& options) {
  return std::make_unique<FlatExprBuilder>();
}

}  // namespace mock_cel
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_MOCK_CEL_CEL_EXPR_BUILDER_FACTORY_H
