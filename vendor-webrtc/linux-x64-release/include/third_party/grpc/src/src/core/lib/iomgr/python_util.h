//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_PYTHON_UTIL_H
#define GRPC_SRC_CORE_LIB_IOMGR_PYTHON_UTIL_H

#include <grpc/slice.h>
#include <grpc/status.h>
#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/error.h"

// These are only used by the gRPC Python extensions.
// They are easier to define here (rather than in Cython)
// because Cython doesn't handle #defines well.

inline grpc_error_handle grpc_socket_error(char* error) {
  return grpc_error_set_int(GRPC_ERROR_CREATE(error),
                            grpc_core::StatusIntProperty::kRpcStatus,
                            GRPC_STATUS_UNAVAILABLE);
}

inline char* grpc_slice_buffer_start(grpc_slice_buffer* buffer, int i) {
  return reinterpret_cast<char*>(GRPC_SLICE_START_PTR(buffer->slices[i]));
}

inline int grpc_slice_buffer_length(grpc_slice_buffer* buffer, int i) {
  return GRPC_SLICE_LENGTH(buffer->slices[i]);
}

#endif  // GRPC_SRC_CORE_LIB_IOMGR_PYTHON_UTIL_H
