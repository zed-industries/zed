//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_WAKEUP_FD_PIPE_H
#define GRPC_SRC_CORE_LIB_IOMGR_WAKEUP_FD_PIPE_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/wakeup_fd_posix.h"

extern const grpc_wakeup_fd_vtable grpc_pipe_wakeup_fd_vtable;

#endif  // GRPC_SRC_CORE_LIB_IOMGR_WAKEUP_FD_PIPE_H
