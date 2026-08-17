//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_MEMORY_H
#define GRPC_SRC_CORE_UTIL_MEMORY_H

#include <grpc/support/alloc.h>
#include <grpc/support/port_platform.h>

#include <memory>
#include <type_traits>

namespace grpc_core {

class DefaultDeleteChar {
 public:
  void operator()(char* p) {
    if (p == nullptr) return;
    gpr_free(p);
  }
};

// UniquePtr<T> is only allowed for char and UniquePtr<char> is deprecated
// in favor of std::string. UniquePtr<char> is equivalent std::unique_ptr
// except that it uses gpr_free for deleter.
template <typename T>
using UniquePtr = std::unique_ptr<T, DefaultDeleteChar>;

template <class T>
T* Zalloc() {
  static_assert(std::is_trivial<T>::value, "Type should be trivial");
  return static_cast<T*>(gpr_zalloc(sizeof(T)));
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_MEMORY_H
