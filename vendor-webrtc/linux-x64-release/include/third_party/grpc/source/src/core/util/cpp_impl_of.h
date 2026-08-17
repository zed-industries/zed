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

#ifndef GRPC_SRC_CORE_UTIL_CPP_IMPL_OF_H
#define GRPC_SRC_CORE_UTIL_CPP_IMPL_OF_H

namespace grpc_core {

// Declares CppType to be the backing implementation of CType.
// Use via the curiously recursive template:
// class Foo : public CppImplOf<Foo, grpc_foo> {};
// Provides casting methods each way.
// grpc_foo should be `typedef struct grpc_foo grpc_foo` and otherwise
// not defined.
template <typename CppType, typename CType>
class CppImplOf {
 public:
  // Convert the C struct to C++
  static CppType* FromC(CType* c_type) {
    return reinterpret_cast<CppType*>(c_type);
  }

  static const CppType* FromC(const CType* c_type) {
    return reinterpret_cast<const CppType*>(c_type);
  }

  // Retrieve a c pointer (of the same ownership as this)
  CType* c_ptr() {
    return reinterpret_cast<CType*>(static_cast<CppType*>(this));
  }

 protected:
  ~CppImplOf() = default;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_CPP_IMPL_OF_H
