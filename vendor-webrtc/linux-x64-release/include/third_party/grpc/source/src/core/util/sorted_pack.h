// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_SORTED_PACK_H
#define GRPC_SRC_CORE_UTIL_SORTED_PACK_H

#include <grpc/support/port_platform.h>

#include <type_traits>

#include "src/core/util/type_list.h"

namespace grpc_core {

namespace sorted_pack_detail {

// Find the smallest element of Args, and the rest of the elements
template <template <typename, typename> class Cmp, typename Args>
struct Smallest;

template <template <typename, typename> class Cmp, typename Arg,
          typename... Args>
struct Smallest<Cmp, Typelist<Arg, Args...>> {
  using SmallestRest = Smallest<Cmp, Typelist<Args...>>;
  using PrevSmallest = typename SmallestRest::Result;
  using PrevRest = typename SmallestRest::Rest;
  static constexpr bool kCmpResult = Cmp<Arg, PrevSmallest>::kValue;
  using Result = typename std::conditional<kCmpResult, Arg, PrevSmallest>::type;
  using Prefix = typename std::conditional<kCmpResult, PrevSmallest, Arg>::type;
  using Rest = typename PrevRest::template PushFront<Prefix>;
};

template <template <typename, typename> class Cmp, typename Arg>
struct Smallest<Cmp, Typelist<Arg>> {
  using Result = Arg;
  using Rest = Typelist<>;
};

// Sort a list of types into a typelist
template <template <typename, typename> class Cmp, typename Args>
struct Sorted;

template <template <typename, typename> class Cmp, typename... Args>
struct Sorted<Cmp, Typelist<Args...>> {
  using SmallestResult = Smallest<Cmp, Typelist<Args...>>;
  using SmallestType = typename SmallestResult::Result;
  using RestOfTypes = typename SmallestResult::Rest;
  using SortedRestOfTypes = typename Sorted<Cmp, RestOfTypes>::Result;
  using Result = typename SortedRestOfTypes::template PushFront<SmallestType>;
};

template <template <typename, typename> class Cmp, typename Arg>
struct Sorted<Cmp, Typelist<Arg>> {
  using Result = Typelist<Arg>;
};

template <template <typename, typename> class Cmp>
struct Sorted<Cmp, Typelist<>> {
  using Result = Typelist<>;
};

}  // namespace sorted_pack_detail

// Given a type T<A...>, and a type comparator Cmp<P,Q>, and some set of types
// Args...:
// Sort Args... using Cmp into SortedArgs..., then instantiate T<SortedArgs...>
// as Type.
// Cmp<P,Q> should have a single constant `kValue` that is true if P < Q.
template <template <typename...> class T,
          template <typename, typename> class Cmp, typename... Args>
struct WithSortedPack {
  using Type = typename sorted_pack_detail::Sorted<
      Cmp, Typelist<Args...>>::Result::template Instantiate<T>;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_SORTED_PACK_H
