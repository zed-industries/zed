//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_UNIX_SOCKETS_POSIX_H
#define GRPC_SRC_CORE_LIB_IOMGR_UNIX_SOCKETS_POSIX_H

#include <grpc/support/port_platform.h>
#include <grpc/support/string_util.h>

#include <string>

#include "absl/strings/string_view.h"
#include "src/core/lib/iomgr/port.h"
#include "src/core/lib/iomgr/resolve_address.h"

void grpc_create_socketpair_if_unix(int sv[2]);

absl::StatusOr<std::vector<grpc_resolved_address>>
grpc_resolve_unix_domain_address(absl::string_view name);

absl::StatusOr<std::vector<grpc_resolved_address>>
grpc_resolve_unix_abstract_domain_address(absl::string_view name);

int grpc_is_unix_socket(const grpc_resolved_address* resolved_addr);

void grpc_unlink_if_unix_domain_socket(
    const grpc_resolved_address* resolved_addr);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_UNIX_SOCKETS_POSIX_H
