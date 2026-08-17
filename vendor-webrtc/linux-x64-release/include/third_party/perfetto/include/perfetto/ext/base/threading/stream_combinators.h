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

#ifndef INCLUDE_PERFETTO_EXT_BASE_THREADING_STREAM_COMBINATORS_H_
#define INCLUDE_PERFETTO_EXT_BASE_THREADING_STREAM_COMBINATORS_H_

#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/ext/base/threading/future_combinators.h"
#include "perfetto/ext/base/threading/poll.h"

namespace perfetto {
namespace base {

template <typename T>
class Stream;

// Helper function for adding all the elements in parameter pack to a vector.
template <typename T, typename... Elements>
void AddAllToVector(std::vector<T>&) {}

template <typename T, typename... Elements>
void AddAllToVector(std::vector<T>& vec, T first, Elements... rest) {
  vec.emplace_back(std::forward<T>(first));
  AddAllToVector(vec, std::forward<Elements>(rest)...);
}

// For a Function which returns Stream<U>, returns the U.
template <typename Function, typename T>
using StreamReturn =
    typename std::invoke_result<Function, T>::type::PollableItem;

// Implementation of StreamPollable for creating a Stream<T> from a
// std::vector<T>.
template <typename T>
class ImmediateStreamImpl : public StreamPollable<T> {
 public:
  explicit ImmediateStreamImpl(std::vector<T> values)
      : values_(std::move(values)) {}

  StreamPollResult<T> PollNext(PollContext*) override {
    if (index_ >= values_.size()) {
      return DonePollResult();
    }
    return StreamPollResult<T>(std::move(values_[index_++]));
  }

 private:
  std::vector<T> values_;
  uint32_t index_ = 0;
};

// Implementation of a StreamPollable for creating a Stream<U> from a Stream<T>
// and a functor with prototype Future<U>(T).
template <typename Function, typename T>
class MapFutureStreamImpl : public StreamPollable<FutureReturn<Function, T>> {
 public:
  using U = FutureReturn<Function, T>;

  MapFutureStreamImpl(Stream<T> stream, Function map_fn)
      : stream_(std::move(stream)), map_fn_(std::move(map_fn)) {}

  StreamPollResult<U> PollNext(PollContext* context) override {
    if (!future_) {
      ASSIGN_OR_RETURN_IF_PENDING_STREAM(res, stream_.PollNext(context));
      if (res.IsDone()) {
        return DonePollResult();
      }
      future_ = map_fn_(std::move(res.item()));
    }
    ASSIGN_OR_RETURN_IF_PENDING_FUTURE(res, future_->Poll(context));
    future_ = std::nullopt;
    return res;
  }

 private:
  Stream<T> stream_;
  Function map_fn_;
  std::optional<Future<U>> future_;
};

// Implementation of a StreamPollable for creating a concatenating two streams
// together.
template <typename T>
class ConcatStreamImpl : public StreamPollable<T> {
 public:
  explicit ConcatStreamImpl(Stream<T> first, Stream<T> second)
      : first_(std::move(first)), second_(std::move(second)) {}

  StreamPollResult<T> PollNext(PollContext* context) override {
    if (first_) {
      ASSIGN_OR_RETURN_IF_PENDING_STREAM(res, first_->PollNext(context));
      if (!res.IsDone()) {
        return res.item();
      }
      first_ = std::nullopt;
    }
    return second_.PollNext(context);
  }

 private:
  std::optional<Stream<T>> first_;
  Stream<T> second_;
};

// Implementation of a StreamPollable for creating a Stream<T> from a
// std::vector<Stream<T>>. Values are returned from the inner streams as soon as
// they are available.
template <typename T>
class FlattenImpl : public StreamPollable<T> {
 public:
  explicit FlattenImpl(std::vector<Stream<T>> streams)
      : registered_handles_(static_cast<uint32_t>(streams.size())) {
    for (auto& stream : streams) {
      streams_.emplace_back(std::move(stream));
    }
  }

  StreamPollResult<T> PollNext(PollContext* upstream) override {
    for (uint32_t i = 0; i < streams_.size(); ++i) {
      auto& stream = streams_[i];
      if (!stream) {
        continue;
      }
      std::optional<PollContext> ctx = PollContextForStream(upstream, i);
      if (!ctx) {
        continue;
      }
      StreamPollResult<T> res = stream->PollNext(&*ctx);
      if (res.IsPending()) {
        PERFETTO_CHECK(!registered_handles_[i].empty());
        continue;
      }
      if (!res.IsDone()) {
        return res;
      }
      // StreamPollable has returned EOF. Clear it and the registered handles
      // out.
      stream = std::nullopt;
      ++eof_streams_;
      registered_handles_[i].clear();
    }

    // Every child stream being EOF means we have reached EOF as well.
    if (eof_streams_ == streams_.size()) {
      return DonePollResult();
    }
    // Every remaining stream must be pending so we can make no further
    // progress. Register all the child handles with the context and return.
    for (const FlatSet<PlatformHandle>& handles : registered_handles_) {
      upstream->RegisterAllInterested(handles);
    }
    return PendingPollResult();
  }

 private:
  std::optional<PollContext> PollContextForStream(PollContext* upstream,
                                                  uint32_t stream_idx) {
    FlatSet<PlatformHandle>& state = registered_handles_[stream_idx];
    if (state.empty()) {
      return PollContext(&state, &upstream->ready_handles());
    }
    for (PlatformHandle handle : upstream->ready_handles()) {
      if (state.count(handle)) {
        state.clear();
        return PollContext(&state, &upstream->ready_handles());
      }
    }
    return std::nullopt;
  }

  std::vector<std::optional<Stream<T>>> streams_;
  std::vector<FlatSet<PlatformHandle>> registered_handles_;
  uint32_t eof_streams_ = 0;
};

// Implementation of a Stream<T> which immediately completes and calls a
// function in the destructor.
template <typename T, typename Function>
class OnDestroyStreamImpl : public StreamPollable<T> {
 public:
  explicit OnDestroyStreamImpl(Function fn) : fn_(std::move(fn)) {}
  ~OnDestroyStreamImpl() override { fn_(); }

  StreamPollResult<T> PollNext(PollContext*) override {
    return DonePollResult();
  }

 private:
  Function fn_;
};

// Interface for converting a Stream<T> into a Future<U>.
//
// The goal of this interface is to allow a Stream to be converted to a Future,
// allowing short-circuiting (i.e. allowing the Future to complete before
// the stream finishes).
//
// The flexibility of this interface allows both supporting the traditional
// notion of collecting i.e. converting a Stream<T> to a Future<vector<T>> but
// also more advanced functionality like completing a Future<Status> early
// when errors are detected, racing Future<T> against each other and returning
// the first value produced etc.
template <typename T, typename U>
class Collector {
 public:
  virtual ~Collector() = default;

  // Receives the next item from a Stream<T>. If the wrapping Future<U> can be
  // completed, returns the a value U which completes that future. Otherwise,
  // returns std::nullopt.
  virtual std::optional<U> OnNext(T value) = 0;

  // Called when the stream has completed and returns the |U| which will be
  // used to complete the future. This method will only be called if OnNext
  // returned std::nullopt for every element in the stream.
  virtual U OnDone() = 0;
};

// Implementation of a StreamPollable which converts a Stream<T> to a Future<U>
// using an implementation of Collector<T, U>.
template <typename T, typename U>
class CollectImpl : public FuturePollable<U> {
 public:
  explicit CollectImpl(Stream<T> stream,
                       std::unique_ptr<Collector<T, U>> collector)
      : stream_(std::move(stream)), collector_(std::move(collector)) {}

  FuturePollResult<U> Poll(PollContext* context) override {
    for (;;) {
      ASSIGN_OR_RETURN_IF_PENDING_STREAM(res, stream_.PollNext(context));
      if (res.IsDone()) {
        return collector_->OnDone();
      }
      std::optional<U> collected = collector_->OnNext(std::move(res.item()));
      if (collected.has_value()) {
        return std::move(collected.value());
      }
    }
  }

 private:
  Stream<T> stream_;
  std::unique_ptr<Collector<T, U>> collector_;
};

// Implementation for |AllOkCollector|.
class AllOkCollectorImpl : public Collector<Status, Status> {
 public:
  ~AllOkCollectorImpl() override;

  std::optional<Status> OnNext(Status status) override {
    return status.ok() ? std::nullopt : std::make_optional(std::move(status));
  }
  Status OnDone() override { return OkStatus(); }
};

// Implementation for |ToFutureCheckedCollector|.
template <typename T>
class FutureCheckedCollectorImpl : public Collector<T, T> {
 public:
  std::optional<T> OnNext(T value) override {
    PERFETTO_CHECK(!prev_value_);
    prev_value_ = value;
    return std::nullopt;
  }
  T OnDone() override { return *prev_value_; }

 private:
  std::optional<T> prev_value_;
};

// Implementation for |StatusOrVectorCollector|.
template <typename T>
class StatusOrVectorCollectorImpl
    : public Collector<base::StatusOr<T>, base::StatusOr<std::vector<T>>> {
 public:
  std::optional<base::StatusOr<std::vector<T>>> OnNext(
      base::StatusOr<T> val_or) override {
    if (!val_or.ok()) {
      return std::make_optional(val_or.status());
    }
    values_.emplace_back(std::move(val_or.value()));
    return std::nullopt;
  }
  base::StatusOr<std::vector<T>> OnDone() override {
    return std::move(values_);
  }

 private:
  std::vector<T> values_;
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_THREADING_STREAM_COMBINATORS_H_
