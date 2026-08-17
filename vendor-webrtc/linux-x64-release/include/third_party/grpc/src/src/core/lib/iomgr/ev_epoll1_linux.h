//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_EV_EPOLL1_LINUX_H
#define GRPC_SRC_CORE_LIB_IOMGR_EV_EPOLL1_LINUX_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/ev_posix.h"
#include "src/core/lib/iomgr/port.h"

// a polling engine that utilizes a singleton epoll set and turnstile polling

extern const grpc_event_engine_vtable grpc_ev_epoll1_posix;

#endif  // GRPC_SRC_CORE_LIB_IOMGR_EV_EPOLL1_LINUX_H
