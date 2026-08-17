// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_RESOLVED_ADDRESS_H
#define GRPC_SRC_CORE_LIB_IOMGR_RESOLVED_ADDRESS_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_WINSOCK_SOCKET
#include <ws2tcpip.h>
#endif

#if defined(GRPC_POSIX_SOCKET) || defined(GRPC_CFSTREAM)
#include <sys/socket.h>
#endif

#define GRPC_MAX_SOCKADDR_SIZE 128

struct grpc_resolved_address {
  char addr[GRPC_MAX_SOCKADDR_SIZE];
  socklen_t len;
};

#endif  // GRPC_SRC_CORE_LIB_IOMGR_RESOLVED_ADDRESS_H
