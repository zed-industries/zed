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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_ALL_OK_H
#define GRPC_SRC_CORE_LIB_PROMISE_ALL_OK_H

#include <grpc/support/port_platform.h>

#include <tuple>
#include <variant>

#include "absl/meta/type_traits.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/lib/promise/detail/join_state.h"
#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/map.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/promise/status_flag.h"

namespace grpc_core {
// AllOk promise combinator.
//
// Input:
// 1. Two or more promises.
// 2. All promises MUST return either a Poll<StatusFlag> or Poll<absl::Status>.
//
// Return:
// 1. The return type of AllOk<Result, Promises...> is Poll<Result>, where
// Result is either StatusFlag or absl::Status.
// 2. If Result is StatusFlag, then all the promises MUST return
// Poll<StatusFlag>.
// 3. If Result is absl::Status, then the promises may return either
// Poll<StatusFlag> or Poll<absl::Status>.
// For further understanding of points 2 and 3, refer the WithMixedTypes* tests
// in all_ok_test.cc.
//
// Polling the AllOk combinator works in the following way :
// Polling this AllOk combinator will make the pending promises run
// serially, in order and on the same thread.
// Each promise being executed either returns a status (StatusFlag or
// absl::Status) or Pending{}. Each subsequent execution of the AllOk will only
// execute the input promises which are still pending. This mechanism ensures
// that no promise is executed after it resolves, which is an essential
// requirement. If all the promises have finished running successfully, the
// AllOk combinator will return a success status. If there is at least one
// promise that has returned Pending{}, the AllOk combinator will return
// Pending{}.
//
// The execution of promises in the AllOk combinator will stop if any one
// promise returns a failure status. This failure status is returned by the
// AllOk combinator.

namespace promise_detail {

// Traits object to pass to JoinState
template <typename Result>
struct AllOkTraits {
  template <typename T>
  using ResultType = Result;
  template <typename T>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsOk(const T& x) {
    return IsStatusOk(x);
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static Empty Unwrapped(StatusFlag) {
    return Empty{};
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static Empty Unwrapped(absl::Status) {
    return Empty{};
  }
  template <typename R, typename T>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static R EarlyReturn(T&& x) {
    return StatusCast<R>(std::forward<T>(x));
  }
  template <typename... A>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static Result FinalReturn(A&&...) {
    return Result{};
  }
};

// Implementation of AllOk combinator.
template <typename Result, typename... Promises>
class AllOk {
 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION explicit AllOk(Promises... promises)
      : state_(std::move(promises)...) {}
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION auto operator()() {
    return state_.PollOnce();
  }

 private:
  JoinState<AllOkTraits<Result>, Promises...> state_;
};

}  // namespace promise_detail

// Run all promises.
// If any fail, cancel the rest and return the failure.
// If all succeed, return Ok.
template <typename Result, typename... Promises>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline auto AllOk(Promises... promises) {
  return promise_detail::AllOk<Result, Promises...>(std::move(promises)...);
}

// Construct a promise for each element of the set, then run them all.
// If any fail, cancel the rest and return the failure.
// If all succeed, return Ok.
template <typename Result, typename Iter, typename FactoryFn>
inline auto AllOkIter(Iter begin, Iter end, FactoryFn factory_fn) {
  using Factory =
      promise_detail::RepeatedPromiseFactory<decltype(*begin), FactoryFn>;
  Factory factory(std::move(factory_fn));
  using Promise = typename Factory::Promise;
  std::vector<Promise> promises;
  std::vector<bool> done;
  for (auto it = begin; it != end; ++it) {
    promises.emplace_back(factory.Make(*it));
    done.push_back(false);
  }
  return [promises = std::move(promises),
          done = std::move(done)]() mutable -> Poll<Result> {
    using Traits = promise_detail::AllOkTraits<Result>;
    bool still_working = false;
    for (size_t i = 0; i < promises.size(); ++i) {
      if (done[i]) continue;
      auto p = promises[i]();
      if (auto* r = p.value_if_ready()) {
        if (!Traits::IsOk(*r)) {
          return Traits::template EarlyReturn<Result>(std::move(*r));
        }
        done[i] = true;
      } else {
        still_working = true;
      }
    }
    if (still_working) return Pending{};
    return Traits::FinalReturn();
  };
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_ALL_OK_H
