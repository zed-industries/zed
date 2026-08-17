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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_LOOP_H
#define GRPC_SRC_CORE_LIB_PROMISE_LOOP_H

#include <grpc/support/port_platform.h>

#include <utility>
#include <variant>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/util/construct_destruct.h"

namespace grpc_core {

// A Loop combinator
//
// Input:
//
// 1. A Loop combinator takes as input only one promise factory.
// 2. This input promise factory should have a return type of either
//    a.  LoopCtl<T> which is an alias for std::variant<Continue, T>
//    b.  Or Poll<LoopCtl<T>>
//
// Running of the Loop combinator:
//
// 1. The input promise is guranteed to run at least once when the combinator
//    is invoked.
// 2. The Loop combinators execution will keep running input promise for
//    as long as the input promise returns the Continue() object.
// 3. The Loop breaks if
//    a.  the input promise returns T.
//    b.  the input promise returns Pending{}.
//
// The execution of multiple iterations of the input
// promise happen on the same thread.
//
// Return:
//
// The Loop combinator when executed will return Poll<T>.
//
// Example:
//
// {
//   std::string execution_order;
//   int i = 0;
//   Poll<int> retval = Loop([&execution_order, &i]() {
//       return [&execution_order, &i]() -> LoopCtl<int> {
//           absl::StrAppend(&execution_order, i);
//           i++;
//           if (i < 5) return Continue();
//           return i;
//       };
//   })();
//   EXPECT_TRUE(retval.ready());
//   EXPECT_EQ(retval.value(), 5);
//   EXPECT_EQ(i, 5);
//   EXPECT_STREQ(execution_order.c_str(), "01234");
// }

// Special type - signals to loop to take another iteration, instead of
// finishing
struct Continue {};

// Result of polling a loop promise - either Continue looping, or return a value
// T
template <typename T>
using LoopCtl = std::variant<Continue, T>;

namespace promise_detail {

template <typename T>
struct LoopTraits;

template <typename T>
struct LoopTraits<LoopCtl<T>> {
  using Result = T;
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static LoopCtl<T> ToLoopCtl(
      LoopCtl<T> value) {
    return value;
  }
};

template <typename T>
struct LoopTraits<absl::StatusOr<LoopCtl<T>>> {
  using Result = absl::StatusOr<T>;
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static LoopCtl<Result> ToLoopCtl(
      absl::StatusOr<LoopCtl<T>> value) {
    if (!value.ok()) return value.status();
    auto& inner = *value;
    if (std::holds_alternative<Continue>(inner)) return Continue{};
    return std::get<T>(std::move(inner));
  }
};

template <>
struct LoopTraits<absl::StatusOr<LoopCtl<absl::Status>>> {
  using Result = absl::Status;
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static LoopCtl<Result> ToLoopCtl(
      absl::StatusOr<LoopCtl<absl::Status>> value) {
    if (!value.ok()) return value.status();
    const auto& inner = *value;
    if (std::holds_alternative<Continue>(inner)) return Continue{};
    return std::get<absl::Status>(inner);
  }
};

}  // namespace promise_detail

template <typename F>
class Loop {
 private:
  static_assert(promise_detail::kIsRepeatedPromiseFactory<void, F>);

  using Factory = promise_detail::RepeatedPromiseFactory<void, F>;
  using PromiseType = decltype(std::declval<Factory>().Make());
  using PromiseResult = typename PromiseType::Result;

 public:
  using Result = typename promise_detail::LoopTraits<PromiseResult>::Result;

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION explicit Loop(F f)
      : factory_(std::move(f)) {}
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION ~Loop() {
    if (started_) Destruct(&promise_);
  }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION Loop(Loop&& loop) noexcept
      : factory_(std::move(loop.factory_)), started_(loop.started_) {
    if (started_) Construct(&promise_, std::move(loop.promise_));
  }

  Loop(const Loop& loop) = delete;
  Loop& operator=(const Loop& loop) = delete;

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION Poll<Result> operator()() {
    GRPC_TRACE_LOG(promise_primitives, INFO)
        << "loop[" << this << "] begin poll started=" << started_;
    if (!started_) {
      started_ = true;
      Construct(&promise_, factory_.Make());
    }
    while (true) {
      // Poll the inner promise.
      auto promise_result = promise_();
      // If it returns a value:
      if (auto* p = promise_result.value_if_ready()) {
        //  - then if it's Continue, destroy the promise and recreate a new one
        //  from our factory.
        auto lc =
            promise_detail::LoopTraits<PromiseResult>::ToLoopCtl(std::move(*p));
        if (std::holds_alternative<Continue>(lc)) {
          GRPC_TRACE_LOG(promise_primitives, INFO)
              << "loop[" << this << "] iteration complete, continue";
          Destruct(&promise_);
          Construct(&promise_, factory_.Make());
          continue;
        }
        GRPC_TRACE_LOG(promise_primitives, INFO)
            << "loop[" << this << "] iteration complete, return";
        //  - otherwise there's our result... return it out.
        return std::get<Result>(std::move(lc));
      } else {
        // Otherwise the inner promise was pending, so we are pending.
        GRPC_TRACE_LOG(promise_primitives, INFO)
            << "loop[" << this << "] pending";
        return Pending();
      }
    }
  }

 private:
  GPR_NO_UNIQUE_ADDRESS Factory factory_;
  GPR_NO_UNIQUE_ADDRESS union {
    GPR_NO_UNIQUE_ADDRESS PromiseType promise_;
  };
  bool started_ = false;
};

template <typename F>
Loop(F) -> Loop<F>;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_LOOP_H
