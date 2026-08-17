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

#ifndef INCLUDE_PERFETTO_EXT_BASE_THREADING_FUTURE_H_
#define INCLUDE_PERFETTO_EXT_BASE_THREADING_FUTURE_H_

#include <memory>
#include <type_traits>

#include "perfetto/ext/base/status_or.h"
#include "perfetto/ext/base/threading/future_combinators.h"
#include "perfetto/ext/base/threading/poll.h"

namespace perfetto {
namespace base {

// Creates a Future<T> from P, a subclass of FuturePollable<T>.
//
// T generally is a primitive (e.g. int, string, double) or structs of
// primitives but any can also be any moveable type.
//
// This function follows the same pattern of std::make_unique, std::make_shared
// etc.
template <typename P, typename... Args, typename T = typename P::PollT>
Future<T> MakeFuture(Args... args) {
  return Future<T>(
      std::unique_ptr<FuturePollable<T>>(new P(std::forward<Args>(args)...)));
}

// A value of type T which is computed asynchronously.
//
// The result of long running compute/IO operations may not be available
// immediately. This class acts as a representation of the value which will be
// produced at some point in the future. Callers can then be notified of the
// result once it's available to be processed.
//
// This class takes heavy inspiration from the implementation of Futures in
// Rust. Specifically, this implementation is:
//  - pull-based/lazy: Futures do nothing until "polled" i.e. driven to
//    completion by a base::TaskRunner. The implementation of this is provided
//    by base::TaskRunnerPoller.
//  - backpressured: because futures are "polled", the result is only
//    requested when it can be processed on the base::TaskRunner thread.
//  - cancellable: by just destroying the future the computation can be
//    cancelled. Note, that the implementation of the source future still needs
//    to propogate cancellation across thread/socket/pipe boundary.
//
// Note: Futures *must* be polled on the same thread on which they were created.
// The |SpawnResultFuture| can be used to move the results of Futures between
// threads in a safe manner.
//
// Implementation note:
// An important point to note is that Future<T> is a final class. Implementation
// of Future<T>::Poll happens through an indirection layer by implementing the
// FuturePollable<T> interface. This allows for the
// unique_ptr<FuturePollable<T>> to be hidden, making callsites nicer while
// also allowing useful "helper" functions like |ContinueWith| to live on the
// class rather than as free functions.
template <typename T>
class Future final {
 public:
  using PollT = T;

  // Creates a Future from a |FuturePollable<T>|. Prefer using |MakeFuture|
  // instead of this function.
  explicit Future(std::unique_ptr<FuturePollable<T>> pollable)
      : pollable_(std::move(pollable)) {}

  // Intentionally implicit to allow for ergonomic definition of functions
  // returning Future<T> for any T.
  Future(T item) : pollable_(new ImmediateImpl<T>(std::move(item))) {}

  // Intentionally implicit to allow for egonomic definition of functions
  // returning Future<StatusOr<T>> by simply returning ErrStatus.
  // The enable_if is necessary because this definition is the same as the above
  // constructor in cases where T = base::Status.
  template <typename U = T,
            typename = std::enable_if_t<!std::is_same_v<Status, U>>>
  Future(Status status) : Future(T(std::move(status))) {}

  // Intentionally implicit to allow for egonomic definition of functions
  // returning Future<StatusOr<T>> by simply returning T.
  template <typename U = T, typename = typename U::value_type>
  Future(typename U::value_type val) : Future(T(std::move(val))) {}

  // Operator used to chain operations on Futures. The result T produced by
  // |this| is passed to |fn| which itself returns a Future<U>. The return value
  // of this function is a Future<U> which encapsulates both the operation done
  // by |this| as well as by the Future<U> returned by |fn|.
  //
  // Usage:
  // ```
  // Future<int> MySpecialFutureFn();
  // Future<std::string> IntToStringInBackground(int);
  //
  // MySpecialFutureFn().ContinueWith([](int x) -> Future<std::string> {
  //   return IntToStringInBackground(x);
  // });
  // ```
  template <typename Function /* Future<U>(T) */,
            typename U = FutureReturn<Function, T>>
  Future<U> ContinueWith(Function fn) && {
    return MakeFuture<ContinueWithImpl<Function, T>>(std::move(*this),
                                                     std::move(fn));
  }

  // Checks if the computation backing this Future<T> has finished.
  //
  // Returns a FuturePollResult<T> which is a essentially a
  // variant<PendingPollResult, T>. If PendingPollResult is returned, |ctx| will
  // be used to register interest in the various fds which are "blocking" this
  // future from finishing. If T is returned, Poll *must not* be called again.
  FuturePollResult<T> Poll(PollContext* ctx) { return pollable_->Poll(ctx); }

 private:
  // TOOD(lalitm): if performance becomes a problem, this can be changed to
  // something more efficient e.g. either storage in a stack allocated buffer
  // or with bump-pointer allocation. In the current usage this is not a
  // performance bottleneck and so this is not important enough to invest time
  // into fixing.
  std::unique_ptr<FuturePollable<T>> pollable_;
};

// Alias to shorten type defintions for Future<Status> which is common in
// the codebase.
using StatusFuture = Future<Status>;

// Alias to shorten type defintions for Future<StatusOr<T>> which is common
// in the codebase.
template <typename T>
using StatusOrFuture = Future<StatusOr<T>>;

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_THREADING_FUTURE_H_
