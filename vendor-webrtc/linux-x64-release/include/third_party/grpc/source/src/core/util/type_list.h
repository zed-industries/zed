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

#ifndef GRPC_SRC_CORE_UTIL_TYPE_LIST_H
#define GRPC_SRC_CORE_UTIL_TYPE_LIST_H

namespace grpc_core {

// A list of types
template <typename... A>
struct Typelist {
  template <template <typename...> class T>
  using Instantiate = T<A...>;

  template <typename C>
  using PushFront = Typelist<C, A...>;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_TYPE_LIST_H
