/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_THREADING_FUTURE_COMBINATORS_H_
#define INCLUDE_PERFETTO_EXT_BASE_THREADING_FUTURE_COMBINATORS_H_

#include <optional>
#include <utility>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/threading/poll.h"

namespace perfetto {
namespace base {

template <typename T>
class Future;

// For a Function which Future<U>, returns the U.
template <typename Function, typename T>
using FutureReturn = typename std::invoke_result<Function, T>::type::PollT;

// Implementation of FuturePollable for creating a Future<T> from a T.
template <typename T>
class ImmediateImpl : public FuturePollable<T> {
 public:
  explicit ImmediateImpl(T value) : value_(std::move(value)) {}

  FuturePollResult<T> Poll(PollContext*) override { return std::move(value_); }

 private:
  T value_;
};

// Implementation of FuturePollable backing Future::ContinueWith.
template <typename Function, typename A, typename B = FutureReturn<Function, A>>
class ContinueWithImpl : public FuturePollable<B> {
 public:
  ContinueWithImpl(Future<A> first, Function second_fn)
      : first_(std::move(first)), second_fn_(std::move(second_fn)) {}

  FuturePollResult<B> Poll(PollContext* context) override {
    PERFETTO_CHECK((first_ && second_fn_) || second_);
    if (first_) {
      ASSIGN_OR_RETURN_IF_PENDING_FUTURE(res, first_->Poll(context));
      first_ = std::nullopt;
      second_ = (*second_fn_)(std::move(res));
      second_fn_ = std::nullopt;
    }
    return second_->Poll(context);
  }

 private:
  std::optional<Future<A>> first_;
  std::optional<Function> second_fn_;
  std::optional<Future<B>> second_;
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_THREADING_FUTURE_COMBINATORS_H_
