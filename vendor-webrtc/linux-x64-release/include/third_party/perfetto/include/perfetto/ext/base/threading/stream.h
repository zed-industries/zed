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

#ifndef INCLUDE_PERFETTO_EXT_BASE_THREADING_STREAM_H_
#define INCLUDE_PERFETTO_EXT_BASE_THREADING_STREAM_H_

#include <functional>
#include <memory>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/ext/base/threading/future.h"
#include "perfetto/ext/base/threading/stream_combinators.h"

namespace perfetto {
namespace base {

// Creates a Stream<T> from P, a subclass of StreamPollable<T>.
//
// This function follows the same pattern of std::make_unique, std::make_shared
// etc.
template <typename P, typename... Args>
Stream<typename P::PollT> MakeStream(Args... args) {
  return Stream<typename P::PollT>(
      std::unique_ptr<StreamPollable<typename P::PollT>>(
          new P(std::forward<Args>(args)...)));
}

// An asynchronous iterator for values of type T.
//
// If Future<T> is an asynchronous version of T, Stream<T> is an asynchronous
// version of Iterator<T>. Long-running compute/IO operations which return
// multiple values can be represented with a Stream<T>.
//
// Note: Streams *must* be polled on the same thread on which they were
// created. The |SpawnResultStreams| can be used to move of the results of
// Streams between threads in a safe manner.
//
// Refer to the class documentation for Future<T> as most of the features and
// implementation of Future<T> also apply to Stream<T>.
template <typename T>
class Stream {
 public:
  using PollableItem = T;

  // Creates a Stream from a |StreamPollable<T>|. Prefer using |MakeStream|
  // instead of this function.
  explicit Stream(std::unique_ptr<StreamPollable<T>> pollable)
      : pollable_(std::move(pollable)) {}

  // Converts a Stream<T> to Stream<U>. This works by applying |map_fn| to each
  // element in T and then polling the returned Future<U> to completion.
  template <typename Function /* = Future<U>(T) */>
  Stream<FutureReturn<Function, T>> MapFuture(Function map_fn) && {
    return MakeStream<MapFutureStreamImpl<Function, T>>(std::move(*this),
                                                        std::move(map_fn));
  }

  // Creates a stream which fully polls |this| and then polls |concat| to
  // completion.
  Stream<T> Concat(Stream<T> concat) && {
    return MakeStream<ConcatStreamImpl<T>>(std::move(*this), std::move(concat));
  }

  // Converts a Stream<T> to Future<U> by collecting elements using |collector|.
  // See documentation on |Collector| for how to implement one.
  template <typename U>
  Future<U> Collect(std::unique_ptr<Collector<T, U>> collector) && {
    return MakeFuture<CollectImpl<T, U>>(std::move(*this),
                                         std::move(collector));
  }

  // Checks if the computation backing this Stream<T> has finished.
  //
  // Returns a StreamPollResult<T> which is a essentially a
  // variant<PendingPollResult, DonePollResult T>. If PendingPollResult is
  // returned, |ctx| will be used to register interest in the various fds which
  // are "blocking" this future from finishing. If DonePollResult is returned,
  // Poll *must not* be called again.
  StreamPollResult<T> PollNext(PollContext* ctx) {
    return pollable_->PollNext(ctx);
  }

 private:
  std::unique_ptr<StreamPollable<T>> pollable_;
};

// Alias to shorten type defintions for Stream<Status> which is common in
// the codebase.
using StatusStream = Stream<Status>;

// Alias to shorten type defintions for Stream<StatusOr<T>> which is common
// in the codebase.
template <typename T>
using StatusOrStream = Stream<StatusOr<T>>;

// Creates a Stream<T> which returns the next value inside |vector| every time
// Stream<T>::Poll is called.
template <typename T>
Stream<T> StreamFrom(std::vector<T> vector) {
  return MakeStream<ImmediateStreamImpl<T>>(std::move(vector));
}

// Creates a Stream<T> which immediately returns DonePollResult when polled.
template <typename T>
Stream<T> EmptyStream() {
  return StreamFrom(std::vector<T>());
}

// Creates a Stream<T> which returns |first| and each of |rest| in sequence when
// polled.
template <typename T, typename... Ts>
Stream<T> StreamOf(T first, Ts... rest) {
  std::vector<T> values;
  AddAllToVector(values, std::forward<T>(first), std::forward<Ts>(rest)...);
  return StreamFrom(std::move(values));
}

// Creates a Stream<T> which returns the value of |future| before completing.
template <typename T>
Stream<T> StreamFromFuture(Future<T> future) {
  return StreamOf(std::move(future)).MapFuture([](Future<T> value) {
    return value;
  });
}

// Creates a stream which returns no elements but calls |fn| in the destructor
// of the returned stream.
//
// This function can be used to do resource management for a stream by making
// the passed |fn| own the resources used by any "upstream" sources and then
// Concat-ing this stream with the upstream.
template <typename T, typename Function>
Stream<T> OnDestroyStream(Function fn) {
  return MakeStream<OnDestroyStreamImpl<T, Function>>(std::move(fn));
}

// Creates a Stream<T> returning values generated by each stream in |streams| as
// soon as they are produced without preserving ordering.
//
// The returned Stream<T> keeps the amount of Poll calls to the inner |streams|,
// to a minimum only calling Poll for the Streams which are marked are ready
// in the PollContext.
template <typename T>
Stream<T> FlattenStreams(std::vector<Stream<T>> streams) {
  return MakeStream<FlattenImpl<T>>(std::move(streams));
}

// Collector for Stream<Status>::Collect() which immediately resolves the
// returned Future when an error status is detected. Resolves with
// OkStatus once the entire stream finishes after returning all OkStatus().
inline std::unique_ptr<Collector<Status, Status>> AllOkCollector() {
  return std::make_unique<AllOkCollectorImpl>();
}

// Collector for Stream<T>::Collect() which ensures the stream returns *exactly*
// one T before completing. Crashes if either a) no values are produced by
// the Stream, b) more than one value is produced by the Stream.
template <typename T>
inline std::unique_ptr<Collector<T, T>> ToFutureCheckedCollector() {
  return std::make_unique<FutureCheckedCollectorImpl<T>>();
}

// Collector for Stream<StatusOr<T>>::Collect() which returns a vector
// containing all the successful results from the stream. If any element is an
// error, short-circuits the stream with the error.
template <typename T>
inline std::unique_ptr<Collector<StatusOr<T>, StatusOr<std::vector<T>>>>
StatusOrVectorCollector() {
  return std::make_unique<StatusOrVectorCollectorImpl<T>>();
}

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_THREADING_STREAM_H_
