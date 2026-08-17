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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_PRIORITIZED_RACE_H
#define GRPC_SRC_CORE_LIB_PROMISE_PRIORITIZED_RACE_H

#include <grpc/support/port_platform.h>

#include <utility>

#include "src/core/lib/promise/detail/promise_like.h"

namespace grpc_core {

template <typename A, typename B>
class TwoPartyPrioritizedRace {
 public:
  using Result = decltype(std::declval<A>()());

  explicit TwoPartyPrioritizedRace(A a, B b)
      : a_(std::move(a)), b_(std::move(b)) {}

  Result operator()() {
    // Check the priority promise.
    auto p = a_();
    if (p.ready()) return p;
    // Check the other promise.
    auto q = b_();
    if (!q.ready()) return q;
    // re-poll a to see if it's also completed.
    auto r = a_();
    if (r.ready()) {
      // both are ready, but a is prioritized
      return r;
    }
    return q;
  }

 private:
  promise_detail::PromiseLike<A> a_;
  promise_detail::PromiseLike<B> b_;
};

/// Run all the promises until one is non-pending.
/// Once there's a non-pending promise, repoll all the promises before that.
/// Return the result from the lexically first non-pending promise.
template <typename... Promises>
class PrioritizedRace;

template <typename Promise, typename... Promises>
class PrioritizedRace<Promise, Promises...>
    : public TwoPartyPrioritizedRace<Promise, PrioritizedRace<Promises...>> {
 public:
  using Result = decltype(std::declval<Promise>()());
  explicit PrioritizedRace(Promise promise, Promises... promises)
      : TwoPartyPrioritizedRace<Promise, PrioritizedRace<Promises...>>(
            std::move(promise),
            PrioritizedRace<Promises...>(std::move(promises)...)) {}
};

template <typename Promise>
class PrioritizedRace<Promise> {
 public:
  using Result = decltype(std::declval<Promise>()());
  explicit PrioritizedRace(Promise promise) : promise_(std::move(promise)) {}
  Result operator()() { return promise_(); }

 private:
  Promise promise_;
};

template <typename... Promises>
PrioritizedRace(Promises...) -> PrioritizedRace<Promises...>;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_PRIORITIZED_RACE_H
