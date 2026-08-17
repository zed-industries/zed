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

  template <typename C>
  using PushBack = Typelist<A..., C>;
};

namespace typelist_detail {
template <typename T>
struct ReverseTpl;

template <typename A>
struct ReverseTpl<Typelist<A>> {
  using Result = Typelist<A>;
};

template <typename A, typename... Rest>
struct ReverseTpl<Typelist<A, Rest...>> {
  using Result =
      typename ReverseTpl<Typelist<Rest...>>::Result::template PushBack<A>;
};
}  // namespace typelist_detail

template <typename... A>
using Reverse = typename typelist_detail::ReverseTpl<Typelist<A...>>::Result;

// A compile time list of values
template <auto... A>
struct Valuelist {
  template <template <auto...> class T>
  using Instantiate = T<A...>;

  template <auto C>
  using PushFront = Valuelist<C, A...>;

  template <auto C>
  using PushBack = Valuelist<A..., C>;
};

namespace valuelist_detail {
template <typename T>
struct ReverseTpl;

template <auto A>
struct ReverseTpl<Valuelist<A>> {
  using Result = Valuelist<A>;
};

template <auto A, auto... Rest>
struct ReverseTpl<Valuelist<A, Rest...>> {
  using Result =
      typename ReverseTpl<Valuelist<Rest...>>::Result::template PushBack<A>;
};

}  // namespace valuelist_detail

template <auto... A>
using ReverseValues =
    typename valuelist_detail::ReverseTpl<Valuelist<A...>>::Result;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_TYPE_LIST_H
