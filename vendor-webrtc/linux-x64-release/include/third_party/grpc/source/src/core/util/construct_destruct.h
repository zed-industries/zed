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

#ifndef GRPC_SRC_CORE_UTIL_CONSTRUCT_DESTRUCT_H
#define GRPC_SRC_CORE_UTIL_CONSTRUCT_DESTRUCT_H

#include <grpc/support/port_platform.h>

#include <new>
#include <utility>

namespace grpc_core {

// Call the destructor of p without having to name the type of p.
template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline void Destruct(T* p) {
  p->~T();
}

// Call the constructor of p without having to name the type of p and forward
// any arguments
template <typename T, typename... Args>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline void Construct(T* p,
                                                           Args&&... args) {
  new (p) T(std::forward<Args>(args)...);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_CONSTRUCT_DESTRUCT_H
