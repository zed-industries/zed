//
//
// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_SYSTEMD_UTILS_H
#define GRPC_SRC_CORE_LIB_IOMGR_SYSTEMD_UTILS_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/tcp_server_utils_posix.h"

// Check whether systemd has pre-allocated FDs. If so, check whether any
// pre-allocated FD is valid, i.e. matches addr and its family. If there is
// any valid FD, set its value to s->pre_allocated_fd
//
void set_matching_sd_fds(grpc_tcp_server* s, const grpc_resolved_address* addr,
                         int requested_port);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_SYSTEMD_UTILS_H
