// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PROTOBUF_HPB_REQUIRES_H_
#define PROTOBUF_HPB_REQUIRES_H_

#include <type_traits>
namespace hpb::internal {
// Ports C++20 `requires` to C++17.
// C++20 ideal:
//  if constexpr (requires { t.foo(); }) { ... }
// Our C++17 stopgap solution:
//  if constexpr (Requires<T>([](auto x) -> decltype(x.foo()) {})) { ... }
template <typename... T, typename F>
constexpr bool Requires(F) {
  return std::is_invocable_v<F, T...>;
}
}  // namespace hpb::internal

#endif  // PROTOBUF_HPB_REQUIRES_H_
