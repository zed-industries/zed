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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_IF_H
#define GRPC_SRC_CORE_LIB_PROMISE_IF_H

#include <grpc/support/port_platform.h>

#include <memory>
#include <utility>
#include <variant>

#include "absl/status/statusor.h"
#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/detail/promise_like.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/util/construct_destruct.h"

namespace grpc_core {

namespace promise_detail {

// If promise combinator.
//
// If(C condition, T if_true, F if_false)
//
// Takes exactly 3 inputs
//
// The first input C can be one of the following
// 1. A bool variable or constant.
// 2. A promise that returns Poll<bool>
// 3. A promise factory that returns a promise that returns Poll<bool>
// 4. A promise that returns Poll<absl::StatusOr<bool>>
// 5. A promise factory that returns a promise that returns
//    Poll<absl::StatusOr<bool>>
//
// The second and third inputs can be one of the following
// 1. Promise
// 2. Promise Factory
// The second and third promises must have the same return type.
//
// The If combinator works in the following way
// 1. It processes the first input first. If the first input is a promise or a
// promise factory, the promise is executed. The return value of this execution
// could either be Poll<bool> or Poll<absl::StatusOr<bool>> . If the first input
// is a bool, it is taken as it is.
// 2. If the first promise returns Pending{} , the second and third promises are
// not executed.
// 3. If the promise returns any failure status , the second and third promises
// are not executed.
// 4. If the return value of the first promise is equivalent to true, the
// combinator executes the second promise (if_true).
// 5. If the return value of the first promise is equivalent to false, the
// combinator executes the third promise (if_false).
//
// Both the condition and the if_true/if_false promises will be executed
// serially on the same thread.
//
// If first input is a constant, it's guaranteed that one of the promise
// factories if_true or if_false will be evaluated before returning from this
// function. This makes it safe to capture lambda arguments in the promise
// factory by reference.

template <typename CallPoll, typename T, typename F>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline typename CallPoll::PollResult
ChooseIf(CallPoll call_poll, bool result, T* if_true, F* if_false) {
  if (result) {
    auto promise = if_true->Make();
    return call_poll(promise);
  } else {
    auto promise = if_false->Make();
    return call_poll(promise);
  }
}

template <typename CallPoll, typename T, typename F>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline typename CallPoll::PollResult
ChooseIf(CallPoll call_poll, absl::StatusOr<bool> result, T* if_true,
         F* if_false) {
  if (!result.ok()) {
    return typename CallPoll::PollResult(result.status());
  } else if (*result) {
    auto promise = if_true->Make();
    return call_poll(promise);
  } else {
    auto promise = if_false->Make();
    return call_poll(promise);
  }
}

}  // namespace promise_detail

template <typename C, typename T, typename F>
class If {
 private:
  using TrueFactory = promise_detail::OncePromiseFactory<void, T>;
  using FalseFactory = promise_detail::OncePromiseFactory<void, F>;
  using ConditionPromise = promise_detail::PromiseLike<C>;
  using TruePromise = typename TrueFactory::Promise;
  using FalsePromise = typename FalseFactory::Promise;
  using Result =
      typename PollTraits<decltype(std::declval<TruePromise>()())>::Type;

 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION If(C condition, T if_true, F if_false)
      : state_(Evaluating{ConditionPromise(std::move(condition)),
                          TrueFactory(std::move(if_true)),
                          FalseFactory(std::move(if_false))}) {}

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION Poll<Result> operator()() {
    return std::visit(CallPoll<false>{this}, state_);
  }

 private:
  struct Evaluating {
    ConditionPromise condition;
    TrueFactory if_true;
    FalseFactory if_false;
  };
  using State = std::variant<Evaluating, TruePromise, FalsePromise>;
  State state_;

  template <bool kSetState>
  struct CallPoll {
    using PollResult = Poll<Result>;

    If* const self;

    GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION PollResult
    operator()(Evaluating& evaluating) const {
      static_assert(
          !kSetState,
          "shouldn't need to set state coming through the initial branch");
      auto r = evaluating.condition();
      if (auto* p = r.value_if_ready()) {
        return ChooseIf(CallPoll<true>{self}, std::move(*p),
                        &evaluating.if_true, &evaluating.if_false);
      }
      return Pending();
    }

    template <class Promise>
    GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION PollResult
    operator()(Promise& promise) const {
      auto r = promise();
      if (kSetState && r.pending()) {
        self->state_.template emplace<Promise>(std::move(promise));
      }
      return r;
    }
  };
};

template <typename T, typename F>
class If<bool, T, F> {
 private:
  using TrueFactory = promise_detail::OncePromiseFactory<void, T>;
  using FalseFactory = promise_detail::OncePromiseFactory<void, F>;
  using TruePromise = typename TrueFactory::Promise;
  using FalsePromise = typename FalseFactory::Promise;
  using Result =
      typename PollTraits<decltype(std::declval<TruePromise>()())>::Type;

 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION If(bool condition, T if_true, F if_false)
      : condition_(condition) {
    TrueFactory true_factory(std::move(if_true));
    FalseFactory false_factory(std::move(if_false));
    if (condition_) {
      Construct(&if_true_, true_factory.Make());
    } else {
      Construct(&if_false_, false_factory.Make());
    }
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION ~If() {
    if (condition_) {
      Destruct(&if_true_);
    } else {
      Destruct(&if_false_);
    }
  }

  If(const If&) = delete;
  If& operator=(const If&) = delete;
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION If(If&& other) noexcept
      : condition_(other.condition_) {
    if (condition_) {
      Construct(&if_true_, std::move(other.if_true_));
    } else {
      Construct(&if_false_, std::move(other.if_false_));
    }
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION If& operator=(If&& other) noexcept {
    if (&other == this) return *this;
    Destruct(this);
    Construct(this, std::move(other));
    return *this;
  }

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION Poll<Result> operator()() {
#ifndef NDEBUG
    asan_canary_ = std::make_unique<int>(1 + *asan_canary_);
#endif
    if (condition_) {
      return if_true_();
    } else {
      return if_false_();
    }
  }

 private:
  bool condition_;
  union {
    TruePromise if_true_;
    FalsePromise if_false_;
  };
  // Make failure to destruct show up in ASAN builds.
#ifndef NDEBUG
  std::unique_ptr<int> asan_canary_ = std::make_unique<int>(0);
#endif
};

template <typename C, typename T, typename F>
If(C, T, F) -> If<C, T, F>;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_IF_H
