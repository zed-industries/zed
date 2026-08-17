//
//
// Copyright 2025 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_CHECK_CLASS_SIZE_H
#define GRPC_SRC_CORE_UTIL_CHECK_CLASS_SIZE_H

#include <grpc/support/port_platform.h>

#if defined(__has_feature)
#if __has_feature(memory_sanitizer)
#define GRPC_MSAN_ENABLED 1
#else
#define GRPC_MSAN_ENABLED 0
#endif
#else
#ifdef MEMORY_SANITIZER
#define GRPC_MSAN_ENABLED 1
#else
#define GRPC_MSAN_ENABLED 0
#endif
#endif

#if defined(GPR_LINUX) && !defined(NDEBUG) && !defined(GRPC_ASAN_ENABLED) && \
    !defined(GRPC_MSAN_ENABLED)
// Since class size varies based on platform and compiler, we limit our
// guardrail to only one platform.
#define GRPC_CHECK_CLASS_SIZE(class_name, class_size) \
  static_assert(sizeof(class_name) <= (class_size), "Class size too large");
#else
#define GRPC_CHECK_CLASS_SIZE(class_name, class_size)
#endif

#endif  // GRPC_SRC_CORE_UTIL_CHECK_CLASS_SIZE_H
