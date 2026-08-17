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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_TRY_SEQ_H
#define GRPC_SRC_CORE_LIB_PROMISE_TRY_SEQ_H

#include <grpc/support/port_platform.h>
#include <stdlib.h>

#include <type_traits>
#include <utility>

#include "absl/log/check.h"
#include "absl/meta/type_traits.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/lib/promise/detail/basic_seq.h"
#include "src/core/lib/promise/detail/promise_like.h"
#include "src/core/lib/promise/detail/seq_state.h"
#include "src/core/lib/promise/detail/status.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/promise/status_flag.h"

namespace grpc_core {

namespace promise_detail {

// TrySeq Promise combinator.
//
// Input :
// 1. The TrySeq combinator needs minimum one promise as input.
// 2. The first input to TrySeq combinator is a promise.
// 3. The remaining inputs to TrySeq combinator are Promise Factories (functors
// that return a promise). The input type of the Nth functor should be the
// return value of the (N-1)th promise.
// 4. Functors can return promises with return type any of the following :
//    1. StatusOr<> to signal that a value is fed forward, or Status to
//       indicate only success/failure. In the case of returning Status, the
//       next functor in the chain takes no arguments.
//    2. StatusFlag and ValueOrStatus can be return types if rich error
//       information is not necessar. In this case the next functor in the chain
//       takes no arguments.
//
// Return :
// Polling the TrySeq Promise combinator returns Poll<StatusOr<T>> where T is
// the type returned by the last promise in the list of input promises.
//
// Polling the TrySeq combinator works in the following way :
// Run the first promise. If it returns Pending{}, nothing else is executed.
// If the first promise returns a value, pass this result to the second functor,
// and run the returned promise. If it returns Pending{}, nothing else is
// executed. If it returns a value, pass this result to the third functor, and
// run the returned promise. etc. Return the final value.
//
// If any of the promises in the TrySeq chain returns a failure status, TrySeq
// will NOT proceed with the execution of the remaining promises. If you want
// the execution to continue when a failure status is received, use the Seq
// combinator instead.
//
// Promises in the TrySeq combinator are run in order, serially and on the same
// thread.
//
// Example :
// The unit tests (esp ThreeTypedPendingThens) in try_seq_test.cc provide all
// possible permutations of how TrySeq combinator can be used.

template <typename T, typename Ignored = void>
struct TrySeqTraitsWithSfinae {
  using UnwrappedType = T;
  using WrappedType = absl::StatusOr<T>;
  template <typename Next>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static auto CallFactory(Next* next,
                                                               T&& value) {
    return next->Make(std::forward<T>(value));
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsOk(const T&) {
    return true;
  }
  static const char* ErrorString(const T&) { abort(); }
  template <typename R>
  static R ReturnValue(T&&) {
    abort();
  }
  template <typename Result, typename RunNext>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static Poll<Result>
  CheckResultAndRunNext(T prior, RunNext run_next) {
    return run_next(std::move(prior));
  }
};

template <typename T>
struct TrySeqTraitsWithSfinae<absl::StatusOr<T>> {
  using UnwrappedType = T;
  using WrappedType = absl::StatusOr<T>;
  template <typename Next>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static auto CallFactory(
      Next* next, absl::StatusOr<T>&& status) {
    return next->Make(std::move(*status));
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsOk(
      const absl::StatusOr<T>& status) {
    return status.ok();
  }
  static std::string ErrorString(const absl::StatusOr<T>& status) {
    return status.status().ToString();
  }
  template <typename R>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static R ReturnValue(
      absl::StatusOr<T>&& status) {
    return FailureStatusCast<R>(status.status());
  }
  template <typename Result, typename RunNext>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static Poll<Result>
  CheckResultAndRunNext(absl::StatusOr<T> prior, RunNext run_next) {
    if (!prior.ok()) return FailureStatusCast<Result>(prior.status());
    return run_next(std::move(prior));
  }
};

template <typename T>
struct AllowGenericTrySeqTraits {
  static constexpr bool value = true;
};

template <>
struct AllowGenericTrySeqTraits<absl::Status> {
  static constexpr bool value = false;
};

template <typename T>
struct AllowGenericTrySeqTraits<absl::StatusOr<T>> {
  static constexpr bool value = false;
};

template <typename T, typename AnyType = void>
struct TakeValueExists {
  static constexpr bool value = false;
};
template <typename T>
struct TakeValueExists<T,
                       absl::void_t<decltype(TakeValue(std::declval<T>()))>> {
  static constexpr bool value = true;
};
// If there exists a function 'IsStatusOk(const T&) -> bool' then we assume that
// T is a status type for the purposes of promise sequences, and a non-OK T
// should terminate the sequence and return.
template <typename T>
struct TrySeqTraitsWithSfinae<
    T, absl::enable_if_t<
           std::is_same<decltype(IsStatusOk(std::declval<T>())), bool>::value &&
               !TakeValueExists<T>::value && AllowGenericTrySeqTraits<T>::value,
           void>> {
  using UnwrappedType = void;
  using WrappedType = T;
  template <typename Next>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static auto CallFactory(Next* next,
                                                               T&&) {
    return next->Make();
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsOk(const T& status) {
    return IsStatusOk(status);
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static std::string ErrorString(
      const T& status) {
    return IsStatusOk(status) ? "OK" : "FAILED";
  }
  template <typename R>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static R ReturnValue(T&& status) {
    return FailureStatusCast<R>(std::move(status));
  }
  template <typename Result, typename RunNext>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static Poll<Result>
  CheckResultAndRunNext(T prior, RunNext run_next) {
    if (!IsStatusOk(prior)) return Result(std::move(prior));
    return run_next(std::move(prior));
  }
};
template <typename T>
struct TrySeqTraitsWithSfinae<
    T, absl::enable_if_t<
           std::is_same<decltype(IsStatusOk(std::declval<T>())), bool>::value &&
               TakeValueExists<T>::value && AllowGenericTrySeqTraits<T>::value,
           void>> {
  using UnwrappedType = decltype(TakeValue(std::declval<T>()));
  using WrappedType = T;
  template <typename Next>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static auto CallFactory(Next* next,
                                                               T&& status) {
    return next->Make(TakeValue(std::forward<T>(status)));
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsOk(const T& status) {
    return IsStatusOk(status);
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static std::string ErrorString(
      const T& status) {
    return IsStatusOk(status) ? "OK" : "FAILED";
  }
  template <typename R>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static R ReturnValue(T&& status) {
    DCHECK(!IsStatusOk(status));
    return FailureStatusCast<R>(status.status());
  }
  template <typename Result, typename RunNext>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static Poll<Result>
  CheckResultAndRunNext(T prior, RunNext run_next) {
    if (!IsStatusOk(prior)) return Result(std::move(prior));
    return run_next(std::move(prior));
  }
};
template <>
struct TrySeqTraitsWithSfinae<absl::Status> {
  using UnwrappedType = void;
  using WrappedType = absl::Status;
  template <typename Next>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static auto CallFactory(Next* next,
                                                               absl::Status&&) {
    return next->Make();
  }
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static bool IsOk(
      const absl::Status& status) {
    return status.ok();
  }
  static std::string ErrorString(const absl::Status& status) {
    return status.ToString();
  }
  template <typename R>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static R ReturnValue(
      absl::Status&& status) {
    return FailureStatusCast<R>(std::move(status));
  }
  template <typename Result, typename RunNext>
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static Poll<Result>
  CheckResultAndRunNext(absl::Status prior, RunNext run_next) {
    if (!prior.ok()) return StatusCast<Result>(std::move(prior));
    return run_next(std::move(prior));
  }
};

template <typename T>
using TrySeqTraits = TrySeqTraitsWithSfinae<T>;

template <typename P, typename... Fs>
class TrySeq {
 public:
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION explicit TrySeq(P&& promise,
                                                       Fs&&... factories,
                                                       DebugLocation whence)
      : state_(std::forward<P>(promise), std::forward<Fs>(factories)...,
               whence) {}

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION auto operator()() {
    return state_.PollOnce();
  }

 private:
  SeqState<TrySeqTraits, P, Fs...> state_;
};

template <typename Iter, typename Factory, typename Argument>
using TrySeqIter = BasicSeqIter<TrySeqTraits, Iter, Factory, Argument>;

template <typename Container, typename Factory, typename Argument>
struct TrySeqContainerResultTraits {
  using BaseResult =
      TrySeqIter<typename Container::iterator, Factory, Argument>;
  class Result {
   public:
    Result(Container container, Factory factory, Argument argument)
        : container_(std::move(container)),
          base_result_(container_.begin(), container_.end(), std::move(factory),
                       std::move(argument)) {}
    Result(const Result&) = delete;
    Result& operator=(const Result&) = delete;
    Result(Result&&) = default;
    Result& operator=(Result&&) = default;

    auto operator()() { return base_result_(); }

   private:
    Container container_;
    BaseResult base_result_;
  };
};

}  // namespace promise_detail

template <typename F>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline F TrySeq(F functor) {
  return functor;
}

template <typename F0, typename F1>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline promise_detail::TrySeq<F0, F1>
TrySeq(F0 f0, F1 f1, DebugLocation whence = {}) {
  return promise_detail::TrySeq<F0, F1>(std::move(f0), std::move(f1), whence);
}

template <typename F0, typename F1, typename F2>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline promise_detail::TrySeq<F0, F1, F2>
TrySeq(F0 f0, F1 f1, F2 f2, DebugLocation whence = {}) {
  return promise_detail::TrySeq<F0, F1, F2>(std::move(f0), std::move(f1),
                                            std::move(f2), whence);
}

template <typename F0, typename F1, typename F2, typename F3>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline promise_detail::TrySeq<F0, F1, F2,
                                                                   F3>
TrySeq(F0 f0, F1 f1, F2 f2, F3 f3, DebugLocation whence = {}) {
  return promise_detail::TrySeq<F0, F1, F2, F3>(
      std::move(f0), std::move(f1), std::move(f2), std::move(f3), whence);
}

template <typename F0, typename F1, typename F2, typename F3, typename F4>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline promise_detail::TrySeq<F0, F1, F2,
                                                                   F3, F4>
TrySeq(F0 f0, F1 f1, F2 f2, F3 f3, F4 f4, DebugLocation whence = {}) {
  return promise_detail::TrySeq<F0, F1, F2, F3, F4>(
      std::move(f0), std::move(f1), std::move(f2), std::move(f3), std::move(f4),
      whence);
}

template <typename F0, typename F1, typename F2, typename F3, typename F4,
          typename F5>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION
    promise_detail::TrySeq<F0, F1, F2, F3, F4, F5>
    TrySeq(F0 f0, F1 f1, F2 f2, F3 f3, F4 f4, F5 f5,
           DebugLocation whence = {}) {
  return promise_detail::TrySeq<F0, F1, F2, F3, F4, F5>(
      std::move(f0), std::move(f1), std::move(f2), std::move(f3), std::move(f4),
      std::move(f5), whence);
}

template <typename F0, typename F1, typename F2, typename F3, typename F4,
          typename F5, typename F6>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION
    promise_detail::TrySeq<F0, F1, F2, F3, F4, F5, F6>
    TrySeq(F0 f0, F1 f1, F2 f2, F3 f3, F4 f4, F5 f5, F6 f6,
           DebugLocation whence = {}) {
  return promise_detail::TrySeq<F0, F1, F2, F3, F4, F5, F6>(
      std::move(f0), std::move(f1), std::move(f2), std::move(f3), std::move(f4),
      std::move(f5), std::move(f6), whence);
}

template <typename F0, typename F1, typename F2, typename F3, typename F4,
          typename F5, typename F6, typename F7>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION
    promise_detail::TrySeq<F0, F1, F2, F3, F4, F5, F6, F7>
    TrySeq(F0 f0, F1 f1, F2 f2, F3 f3, F4 f4, F5 f5, F6 f6, F7 f7,
           DebugLocation whence = {}) {
  return promise_detail::TrySeq<F0, F1, F2, F3, F4, F5, F6, F7>(
      std::move(f0), std::move(f1), std::move(f2), std::move(f3), std::move(f4),
      std::move(f5), std::move(f6), std::move(f7), whence);
}

// Try a sequence of operations of unknown length.
// Asynchronously:
//   for (element in (begin, end)) {
//     auto r = wait_for factory(element, argument);
//     if (!r.ok()) return r;
//     argument = *r;
//   }
//   return argument;
template <typename Iter, typename Factory, typename Argument>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION auto TrySeqIter(Iter begin, Iter end,
                                                     Argument argument,
                                                     Factory factory) {
  return promise_detail::TrySeqIter<Iter, Factory, Argument>(
      begin, end, std::move(factory), std::move(argument));
}

template <typename Container, typename Factory, typename Argument>
auto TrySeqContainer(Container container, Argument argument, Factory factory) {
  using Result =
      typename promise_detail::TrySeqContainerResultTraits<Container, Factory,
                                                           Argument>::Result;
  return Result(std::move(container), std::move(factory), std::move(argument));
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_TRY_SEQ_H
