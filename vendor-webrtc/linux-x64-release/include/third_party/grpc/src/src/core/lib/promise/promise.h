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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_PROMISE_H
#define GRPC_SRC_CORE_LIB_PROMISE_PROMISE_H

#include <grpc/support/port_platform.h>

#include <optional>
#include <type_traits>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "src/core/lib/promise/detail/promise_like.h"
#include "src/core/lib/promise/poll.h"

namespace grpc_core {

// A Promise is any functor that takes no arguments and returns Poll<T>.
// Most of the time we just pass around the functor, but occasionally
// it pays to have a type erased variant, which we define here.
template <typename T>
using Promise = absl::AnyInvocable<Poll<T>()>;

// Helper to execute a promise immediately and return either the result or
// nothing.
template <typename Promise>
auto NowOrNever(Promise promise)
    -> std::optional<typename promise_detail::PromiseLike<Promise>::Result> {
  auto r = promise_detail::PromiseLike<Promise>(std::move(promise))();
  if (auto* p = r.value_if_ready()) {
    return std::move(*p);
  }
  return {};
}

// A promise that never completes.
template <typename T>
struct Never {
  Poll<T> operator()() { return Pending(); }
};

namespace promise_detail {
// A promise that immediately completes.
template <typename T>
class Immediate {
 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION explicit Immediate(T value)
      : value_(std::move(value)) {}

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION Poll<T> operator()() {
    return std::move(value_);
  }

 private:
  T value_;
};
}  // namespace promise_detail

// Return \a value immediately
template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline promise_detail::Immediate<T>
Immediate(T value) {
  return promise_detail::Immediate<T>(std::move(value));
}

// Return status Ok immediately
struct ImmediateOkStatus {
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION Poll<absl::Status> operator()() {
    return absl::OkStatus();
  }
};

// Typecheck that a promise returns the expected return type.
// usage: auto promise = AssertResultType<int>([]() { return 3; });
// NOTE: there are tests in promise_test.cc that are commented out because they
// should fail to compile. When modifying this code these should be uncommented
// and their miscompilation verified.
template <typename T, typename F>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline auto AssertResultType(F f) ->
    typename std::enable_if<std::is_same<decltype(f()), Poll<T>>::value,
                            F>::type {
  return f;
}

template <typename Promise>
using PromiseResult = typename PollTraits<
    typename promise_detail::PromiseLike<Promise>::Result>::Type;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_PROMISE_H
