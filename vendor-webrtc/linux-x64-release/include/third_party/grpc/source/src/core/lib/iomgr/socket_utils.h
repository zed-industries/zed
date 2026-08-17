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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_SOCKET_UTILS_H
#define GRPC_SRC_CORE_LIB_IOMGR_SOCKET_UTILS_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

// TODO(juanlishen): The following functions might be simple enough to implement
// ourselves, so that they don't cause any portability hassle.

// A wrapper for htons on POSIX and Windows
uint16_t grpc_htons(uint16_t hostshort);

// A wrapper for ntohs on POSIX and WINDOWS
uint16_t grpc_ntohs(uint16_t netshort);

// A wrapper for htonl on POSIX and Windows
uint32_t grpc_htonl(uint32_t hostlong);

// A wrapper for ntohl on POSIX and WINDOWS
uint32_t grpc_ntohl(uint32_t netlong);

// A wrapper for inet_pton on POSIX and WINDOWS
int grpc_inet_pton(int af, const char* src, void* dst);

// A wrapper for inet_ntop on POSIX systems and InetNtop on Windows systems
const char* grpc_inet_ntop(int af, const void* src, char* dst, size_t size);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_SOCKET_UTILS_H
