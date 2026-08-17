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

#ifndef GRPC_SRC_CORE_UTIL_MATCH_H
#define GRPC_SRC_CORE_UTIL_MATCH_H

#include <grpc/support/port_platform.h>

#include <utility>
#include <variant>

#include "src/core/util/overload.h"

namespace grpc_core {

namespace detail {

template <typename... Cases>
struct MatchPointerExtractor {
  OverloadType<Cases...> cases;
  template <typename T>
  auto operator()(T& value) -> decltype(cases(&value)) {
    return cases(&value);
  }
};

}  // namespace detail

/// Match on a variant.
/// Given variant \a value, and a set of callables \a fs, call the appropriate
/// callable based on the type contained in \a value.
///
/// Example (prints "hoorah"):
///   variant<int, string> v = 42;
///   Match(v,
///         [](int i) { puts("hoorah"); },
///         [](string s) { puts("boo"); });
template <typename... Fs, typename T0, typename... Ts>
auto Match(const std::variant<T0, Ts...>& value, Fs... fs)
    -> decltype(std::declval<OverloadType<Fs...>>()(std::declval<T0>())) {
  return std::visit(Overload(std::move(fs)...), value);
}

/// A version of Match that takes a mutable pointer to a variant and calls its
/// overload callables with a mutable pointer to the current variant value.
///
/// Example:
///   variant<int, string> v = 42;
///   MatchMutable(&v,
///                [](int* i) { *i = 1; },
///                [](string* s) { *s = "foo"; });
///   // v now contains 1.
template <typename... Fs, typename T0, typename... Ts>
auto MatchMutable(std::variant<T0, Ts...>* value, Fs... fs)
    -> decltype(std::declval<OverloadType<Fs...>>()(std::declval<T0*>())) {
  return std::visit(detail::MatchPointerExtractor<Fs...>{OverloadType<Fs...>(
                        std::move(fs)...)},
                    *value);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_MATCH_H
