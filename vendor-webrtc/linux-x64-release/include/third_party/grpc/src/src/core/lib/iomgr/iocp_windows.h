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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_IOCP_WINDOWS_H
#define GRPC_SRC_CORE_LIB_IOMGR_IOCP_WINDOWS_H

#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_WINSOCK_SOCKET

#include "src/core/lib/iomgr/exec_ctx.h"
#include "src/core/lib/iomgr/socket_windows.h"

typedef enum {
  GRPC_IOCP_WORK_WORK,
  GRPC_IOCP_WORK_TIMEOUT,
  GRPC_IOCP_WORK_KICK
} grpc_iocp_work_status;

grpc_iocp_work_status grpc_iocp_work(grpc_core::Timestamp deadline);
void grpc_iocp_init(void);
void grpc_iocp_kick(void);
void grpc_iocp_flush(void);
void grpc_iocp_shutdown(void);
void grpc_iocp_add_socket(grpc_winsocket*);

// Register that this socket has started shutting down.
// This prevents gRPC from completing its own shutdown until this socket's
// shutdown is finished. IOCP must continue doing work until all such sockets
// have finished shutting down. The socket's state_mu must be locked.
void grpc_iocp_register_socket_shutdown_socket_locked(grpc_winsocket* socket);

// Mark that this socket has finished shutting down.
// The socket's state lock does not need to be held since this function is only
// called once the socket is ready to be destroyed.
void grpc_iocp_finish_socket_shutdown(grpc_winsocket* socket);

#endif

#endif  // GRPC_SRC_CORE_LIB_IOMGR_IOCP_WINDOWS_H
