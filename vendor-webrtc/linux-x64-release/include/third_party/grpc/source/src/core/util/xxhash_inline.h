//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_XXHASH_INLINE_H
#define GRPC_SRC_CORE_UTIL_XXHASH_INLINE_H

#include <grpc/support/port_platform.h>

// This header is a simple wrapper around the third-party xxhash
// library, so that we don't need to define XXH_INLINE_ALL in every file
// that includes xxhash.h.  That definition confuses clang-format's
// ordering of includes.
#define XXH_INLINE_ALL
#include "xxhash.h"

#endif  // GRPC_SRC_CORE_UTIL_XXHASH_INLINE_H
