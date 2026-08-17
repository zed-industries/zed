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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_POLLSET_WINDOWS_H
#define GRPC_SRC_CORE_LIB_IOMGR_POLLSET_WINDOWS_H

#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_WINSOCK_SOCKET
#include "src/core/lib/iomgr/socket_windows.h"

// There isn't really any such thing as a pollset under Windows, due to the
// nature of the IO completion ports. A Windows "pollset" is merely a mutex
// used to synchronize with the IOCP, and workers are condition variables
// used to block threads until work is ready.

typedef enum {
  GRPC_POLLSET_WORKER_LINK_POLLSET = 0,
  GRPC_POLLSET_WORKER_LINK_GLOBAL,
  GRPC_POLLSET_WORKER_LINK_TYPES
} grpc_pollset_worker_link_type;

typedef struct grpc_pollset_worker_link {
  struct grpc_pollset_worker* next;
  struct grpc_pollset_worker* prev;
} grpc_pollset_worker_link;

struct grpc_pollset;
typedef struct grpc_pollset grpc_pollset;

typedef struct grpc_pollset_worker {
  gpr_cv cv;
  int kicked;
  struct grpc_pollset* pollset;
  grpc_pollset_worker_link links[GRPC_POLLSET_WORKER_LINK_TYPES];
} grpc_pollset_worker;

struct grpc_pollset {
  int shutting_down;
  int kicked_without_pollers;
  int is_iocp_worker;
  grpc_pollset_worker root_worker;
  grpc_closure* on_shutdown;
};

void grpc_pollset_global_init(void);
void grpc_pollset_global_shutdown(void);

#endif

#endif  // GRPC_SRC_CORE_LIB_IOMGR_POLLSET_WINDOWS_H
