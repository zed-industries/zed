// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_DOWN_CAST_H
#define GRPC_SRC_CORE_UTIL_DOWN_CAST_H

#include <grpc/support/port_platform.h>

#include <type_traits>

#include "absl/base/config.h"
#include "absl/log/check.h"

namespace grpc_core {

template <typename To, typename From>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline To DownCast(From* f) {
  static_assert(
      std::is_base_of<From, typename std::remove_pointer<To>::type>::value,
      "DownCast requires a base-to-derived relationship");
// If we have RTTI & we're in debug, assert that the cast is legal.
#if ABSL_INTERNAL_HAS_RTTI
#ifndef NDEBUG
  if (f != nullptr) CHECK_NE(dynamic_cast<To>(f), nullptr);
#endif
#endif
  return static_cast<To>(f);
}

template <typename To, typename From>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline To DownCast(From& f) {
  return *DownCast<typename std::remove_reference<To>::type*>(&f);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_DOWN_CAST_H
