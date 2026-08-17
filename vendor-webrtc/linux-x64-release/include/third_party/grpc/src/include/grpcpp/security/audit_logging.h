//
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
//

#ifndef GRPCPP_SECURITY_AUDIT_LOGGING_H
#define GRPCPP_SECURITY_AUDIT_LOGGING_H

#include <grpc/grpc_audit_logging.h>
#include <grpcpp/support/string_ref.h>

#include <memory>
#include <string>
#include <utility>

#include "absl/status/statusor.h"

namespace grpc {
namespace experimental {

using grpc_core::experimental::AuditContext;  // NOLINT(misc-unused-using-decls)
using grpc_core::experimental::AuditLogger;   // NOLINT(misc-unused-using-decls)
using grpc_core::experimental::
    AuditLoggerFactory;  // NOLINT(misc-unused-using-decls)
using grpc_core::experimental::
    RegisterAuditLoggerFactory;  // NOLINT(misc-unused-using-decls)

}  // namespace experimental
}  // namespace grpc

#endif  // GRPCPP_SECURITY_AUDIT_LOGGING_H
