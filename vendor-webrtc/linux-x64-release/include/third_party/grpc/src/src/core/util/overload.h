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

#ifndef GRPC_SRC_CORE_UTIL_OVERLOAD_H
#define GRPC_SRC_CORE_UTIL_OVERLOAD_H

#include <grpc/support/port_platform.h>

#include <utility>

namespace grpc_core {

template <typename... Cases>
struct OverloadType;
// Compose one overload with N more -- use inheritance to leverage using and the
// empty base class optimization.
template <typename Case, typename... Cases>
struct OverloadType<Case, Cases...> : public Case,
                                      public OverloadType<Cases...> {
  explicit OverloadType(Case&& c, Cases&&... cases)
      : Case(std::forward<Case>(c)),
        OverloadType<Cases...>(std::forward<Cases>(cases)...) {}
  using Case::operator();
  using OverloadType<Cases...>::operator();
};
// Overload of a single case is just that case itself
template <typename Case>
struct OverloadType<Case> : public Case {
  explicit OverloadType(Case&& c) : Case(std::forward<Case>(c)) {}
  using Case::operator();
};

/// Compose callables into a single callable.
/// e.g. given [](int i) { puts("a"); } and [](double d) { puts("b"); },
/// return a callable object like:
/// struct {
///   void operator()(int i) { puts("a"); }
///   void operator()(double i) { puts("b"); }
/// };
/// Preserves all captures.
template <typename... Cases>
OverloadType<Cases...> Overload(Cases... cases) {
  return OverloadType<Cases...>(std::move(cases)...);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_OVERLOAD_H
