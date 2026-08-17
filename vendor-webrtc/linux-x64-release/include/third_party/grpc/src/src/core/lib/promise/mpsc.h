// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_MPSC_H
#define GRPC_SRC_CORE_LIB_PROMISE_MPSC_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <algorithm>
#include <cstdint>
#include <limits>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/log/check.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/promise/status_flag.h"
#include "src/core/lib/promise/wait_set.h"
#include "src/core/util/dump_args.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {

namespace mpscpipe_detail {

// Multi Producer Single Consumer (MPSC) inter-activity communications.
// MPSC is used to communicate in between two or more Activities or Promise
// Parties in a thread safe way.
// The communication consists of one or more MpscSender objects and one
// MpscReceiver.

// "Center" of the communication pipe.
// Contains sent but not received messages, and open/close state.
template <typename T>
class Center : public RefCounted<Center<T>> {
 public:
  // Construct the center with a maximum queue size.
  explicit Center(size_t max_queued) : max_queued_(max_queued) {}

  static constexpr const uint64_t kClosedBatch =
      std::numeric_limits<uint64_t>::max();

  // Poll for new items.
  // - Returns true if new items were obtained, in which case they are contained
  //   in dest in the order they were added. Wakes up all pending senders since
  //   there will now be space to send.
  // - If receives have been closed, returns false.
  // - If no new items are available, returns
  //   Pending and sets up a waker to be awoken when more items are available.
  // TODO(ctiller): consider the problem of thundering herds here. There may be
  // more senders than there are queue spots, and so the strategy of waking up
  // all senders is ill-advised.
  // That said, some senders may have been cancelled by the time we wake them,
  // and so waking a subset could cause starvation.
  Poll<bool> PollReceiveBatch(std::vector<T>& dest) {
    ReleasableMutexLock lock(&mu_);
    GRPC_TRACE_LOG(promise_primitives, INFO)
        << "MPSC::PollReceiveBatch: "
        << GRPC_DUMP_ARGS(this, batch_, queue_.size());
    if (queue_.empty()) {
      if (batch_ == kClosedBatch) return false;
      receive_waker_ = GetContext<Activity>()->MakeNonOwningWaker();
      return Pending{};
    }
    dest.swap(queue_);
    queue_.clear();
    if (batch_ != kClosedBatch) ++batch_;
    auto wakeups = send_wakers_.TakeWakeupSet();
    lock.Release();
    wakeups.Wakeup();
    return true;
  }

  // Return value:
  //  - if the pipe is closed, returns kClosedBatch
  //  - if await_receipt is false, returns the batch number the item was sent
  //  in.
  //  - if await_receipt is true, returns the first sending batch number that
  //  guarantees the item has been received.
  template <bool kAwaitReceipt>
  uint64_t Send(T t) {
    ReleasableMutexLock lock(&mu_);
    if (batch_ == kClosedBatch) return kClosedBatch;
    queue_.push_back(std::move(t));
    auto receive_waker = std::move(receive_waker_);
    const uint64_t batch =
        (!kAwaitReceipt && queue_.size() <= max_queued_) ? batch_ : batch_ + 1;
    lock.Release();
    receive_waker.Wakeup();
    return batch;
  }

  // Poll until a particular batch number is received.
  Poll<Empty> PollReceiveBatch(uint64_t batch) {
    ReleasableMutexLock lock(&mu_);
    GRPC_TRACE_LOG(promise_primitives, INFO)
        << "MPSC::PollReceiveBatch: " << GRPC_DUMP_ARGS(this, batch_, batch);
    if (batch_ >= batch) return Empty{};
    send_wakers_.AddPending(GetContext<Activity>()->MakeNonOwningWaker());
    return Pending{};
  }

  // Mark that the receiver is closed.
  void ReceiverClosed(bool wake_receiver) {
    ReleasableMutexLock lock(&mu_);
    GRPC_TRACE_LOG(promise_primitives, INFO)
        << "MPSC::ReceiverClosed: " << GRPC_DUMP_ARGS(this, batch_);
    if (batch_ == kClosedBatch) return;
    batch_ = kClosedBatch;
    auto wakeups = send_wakers_.TakeWakeupSet();
    auto receive_waker = std::move(receive_waker_);
    lock.Release();
    if (wake_receiver) receive_waker.Wakeup();
    wakeups.Wakeup();
  }

 private:
  Mutex mu_;
  const size_t max_queued_;
  std::vector<T> queue_ ABSL_GUARDED_BY(mu_);
  // Every time we give queue_ to the receiver, we increment batch_.
  // When the receiver is closed we set batch_ to kClosedBatch.
  uint64_t batch_ ABSL_GUARDED_BY(mu_) = 1;
  Waker receive_waker_ ABSL_GUARDED_BY(mu_);
  WaitSet send_wakers_ ABSL_GUARDED_BY(mu_);
};

}  // namespace mpscpipe_detail

template <typename T>
class MpscReceiver;

// Send half of an mpsc pipe.
template <typename T>
class MpscSender {
 public:
  MpscSender() = default;
  MpscSender(const MpscSender&) = default;
  MpscSender& operator=(const MpscSender&) = default;
  MpscSender(MpscSender&&) noexcept = default;
  MpscSender& operator=(MpscSender&&) noexcept = default;

  // Input: Input is the object that you want to send. The promise that is
  // returned by Send will take ownership of the object.
  // Return: Returns a promise that will send one item.
  // This promise can either return
  // 1. Pending{} if the sending is still pending
  // 2. Resolves to true if sending is successful
  // 3. Resolves to false if the receiver was closed and the value
  //    will never be successfully sent.
  // The promise returned is thread safe. We can use multiple send calls
  // in parallel to generate multiple such send promises and these promises can
  // be run in parallel in a thread safe way.
  auto Send(T t) { return SendGeneric<false>(std::move(t)); }

  // Similar to send, but the promise returned by SendAcked will not resolve
  // until the item has been received by the receiver.
  auto SendAcked(T t) { return SendGeneric<true>(std::move(t)); }

  StatusFlag UnbufferedImmediateSend(T t) {
    return StatusFlag(center_->template Send<false>(std::move(t)) !=
                      mpscpipe_detail::Center<T>::kClosedBatch);
  }

 private:
  template <bool kAwaitReceipt>
  auto SendGeneric(T t) {
    return [center = center_, t = std::move(t),
            batch = uint64_t(0)]() mutable -> Poll<StatusFlag> {
      if (center == nullptr) return Failure{};
      if (batch == 0) {
        batch = center->template Send<kAwaitReceipt>(std::move(t));
        DCHECK_NE(batch, 0u);
        if (batch == mpscpipe_detail::Center<T>::kClosedBatch) return Failure{};
      }
      auto p = center->PollReceiveBatch(batch);
      if (p.pending()) return Pending{};
      return Success{};
    };
  }

  friend class MpscReceiver<T>;
  explicit MpscSender(RefCountedPtr<mpscpipe_detail::Center<T>> center)
      : center_(std::move(center)) {}
  RefCountedPtr<mpscpipe_detail::Center<T>> center_;
};

// Receive half of an mpsc pipe.
template <typename T>
class MpscReceiver {
 public:
  // max_buffer_hint is the maximum number of elements we'd like to buffer.
  // We half this before passing to Center so that the number there is the
  // maximum number of elements that can be queued in the center of the pipe.
  // The receiver also holds some of the buffered elements (up to half of them!)
  // so the total outstanding is equal to max_buffer_hint (unless it's 1 in
  // which case instantaneosly we may have two elements buffered).
  explicit MpscReceiver(size_t max_buffer_hint)
      : center_(MakeRefCounted<mpscpipe_detail::Center<T>>(
            std::max(static_cast<size_t>(1), max_buffer_hint / 2))) {}
  ~MpscReceiver() {
    if (center_ != nullptr) center_->ReceiverClosed(false);
  }
  // Marking the receiver closed will make sure it will not receive any
  // messages. If a sender tries to Send a message to a closed receiver,
  // sending will fail.
  void MarkClosed() {
    if (center_ != nullptr) center_->ReceiverClosed(true);
  }
  MpscReceiver(const MpscReceiver&) = delete;
  MpscReceiver& operator=(const MpscReceiver&) = delete;
  // Only movable until it's first polled, and so we don't need to contend with
  // a non-empty buffer during a legal move!
  MpscReceiver(MpscReceiver&& other) noexcept
      : center_(std::move(other.center_)) {
    DCHECK(other.buffer_.empty());
  }
  MpscReceiver& operator=(MpscReceiver&& other) noexcept {
    DCHECK(other.buffer_.empty());
    center_ = std::move(other.center_);
    return *this;
  }

  // Construct a new sender for this receiver. One receiver can have multiple
  // senders.
  MpscSender<T> MakeSender() { return MpscSender<T>(center_); }

  // Returns a promise that will resolve to ValueOrFailure<T>.
  // If receiving is closed, the promise will resolve to failure.
  // Otherwise, the promise resolves to the next item and removes
  // said item from the queue.
  auto Next() {
    return [this]() -> Poll<ValueOrFailure<T>> {
      if (buffer_it_ != buffer_.end()) {
        return Poll<ValueOrFailure<T>>(std::move(*buffer_it_++));
      }
      auto p = center_->PollReceiveBatch(buffer_);
      if (bool* r = p.value_if_ready()) {
        if (!*r) return Failure{};
        buffer_it_ = buffer_.begin();
        return Poll<ValueOrFailure<T>>(std::move(*buffer_it_++));
      }
      return Pending{};
    };
  }
  // Returns a promise that will resolve to ValueOrFailure<std::vector<T>>.
  // If receiving is closed, the promise will resolve to failure.
  // Otherwise, the promise returns all the items enqueued till now and removes
  // said items from the queue.
  auto NextBatch() {
    return [this]() -> Poll<ValueOrFailure<std::vector<T>>> {
      if (buffer_it_ != buffer_.end()) {
        std::vector<T> tmp_buffer;
        std::move(buffer_it_, buffer_.end(), std::back_inserter(tmp_buffer));
        buffer_.clear();
        buffer_it_ = buffer_.end();
        return ValueOrFailure<std::vector<T>>(std::move(tmp_buffer));
      }

      auto p = center_->PollReceiveBatch(buffer_);
      if (bool* r = p.value_if_ready()) {
        if (!*r) return Failure{};
        auto tmp_buffer(std::move(buffer_));
        buffer_.clear();
        buffer_it_ = buffer_.end();
        return ValueOrFailure<std::vector<T>>(std::move(tmp_buffer));
      }
      return Pending{};
    };
  }

 private:
  // Received items. We move out of here one by one, but don't resize the
  // vector. Instead, when we run out of items, we poll the center for more -
  // which swaps this buffer in for the new receive queue and clears it.
  // In this way, upon hitting a steady state the queue ought to be allocation
  // free.
  std::vector<T> buffer_;
  typename std::vector<T>::iterator buffer_it_ = buffer_.end();
  RefCountedPtr<mpscpipe_detail::Center<T>> center_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_MPSC_H
