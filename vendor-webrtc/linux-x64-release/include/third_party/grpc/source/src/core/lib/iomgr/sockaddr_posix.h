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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_SOCKADDR_POSIX_H
#define GRPC_SRC_CORE_LIB_IOMGR_SOCKADDR_POSIX_H

// IWYU pragma: private, include "src/core/lib/iomgr/sockaddr.h"

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_POSIX_SOCKET_SOCKADDR
#include <arpa/inet.h>
#include <netdb.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#ifdef GRPC_HAVE_VSOCK
#include <linux/vm_sockets.h>
#endif

typedef struct sockaddr grpc_sockaddr;
typedef struct sockaddr_in grpc_sockaddr_in;
typedef struct in_addr grpc_in_addr;
typedef struct sockaddr_in6 grpc_sockaddr_in6;
typedef struct in6_addr grpc_in6_addr;

#define GRPC_INET_ADDRSTRLEN INET_ADDRSTRLEN
#define GRPC_INET6_ADDRSTRLEN INET6_ADDRSTRLEN

#define GRPC_SOCK_STREAM SOCK_STREAM
#define GRPC_SOCK_DGRAM SOCK_DGRAM

#define GRPC_AF_UNSPEC AF_UNSPEC
#define GRPC_AF_UNIX AF_UNIX
#ifdef GRPC_HAVE_VSOCK
#define GRPC_AF_VSOCK AF_VSOCK
#endif
#define GRPC_AF_INET AF_INET
#define GRPC_AF_INET6 AF_INET6

#define GRPC_AI_PASSIVE AI_PASSIVE

#endif

#endif  // GRPC_SRC_CORE_LIB_IOMGR_SOCKADDR_POSIX_H
