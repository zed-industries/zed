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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_ALPN_ALPN_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_ALPN_ALPN_H

#include <grpc/support/port_platform.h>
#include <string.h>

// Returns 1 if the version is supported, 0 otherwise.
int grpc_chttp2_is_alpn_version_supported(const char* version, size_t size);

// Returns the number of protocol versions to advertise
size_t grpc_chttp2_num_alpn_versions(void);

// Returns the protocol version at index i (0 <= i <
// grpc_chttp2_num_alpn_versions())
const char* grpc_chttp2_get_alpn_version_index(size_t i);

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_ALPN_ALPN_H
