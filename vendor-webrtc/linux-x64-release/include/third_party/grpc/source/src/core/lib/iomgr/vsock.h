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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_VSOCK_H
#define GRPC_SRC_CORE_LIB_IOMGR_VSOCK_H

#include <grpc/support/port_platform.h>
#include <grpc/support/string_util.h>

#include <string>

#include "absl/strings/string_view.h"
#include "src/core/lib/iomgr/port.h"
#include "src/core/lib/iomgr/resolve_address.h"

absl::StatusOr<std::vector<grpc_resolved_address>> grpc_resolve_vsock_address(
    absl::string_view name);

int grpc_is_vsock(const grpc_resolved_address* resolved_addr);

#endif /* GRPC_SRC_CORE_LIB_IOMGR_VSOCK_H */
