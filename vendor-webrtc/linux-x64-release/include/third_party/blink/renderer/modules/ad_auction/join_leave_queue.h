// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_JOIN_LEAVE_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_JOIN_LEAVE_QUEUE_H_

#include "base/check.h"
#include "base/check_op.h"
#include "base/functional/callback.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace blink {

// A FIFO queue for interest group joins and leaves. It ensures there are never
// more than `max_active` started requests, adding requests to a queue if they
// can't be immediately started. Once the consumer informs it when a previously
// start request completes, it will start another request, if one is queued.
//
// This is a separate class so it can be unit tested.
template <typename T>
class JoinLeaveQueue {
 public:
  using StartCallback = base::RepeatingCallback<void(T&& operation)>;

  // `max_active` is the maximum number of active operations at a time. `start`
  // is invoked to start an operation. `this` may not called into or deleted
  // while `start` is being invoked.
  JoinLeaveQueue(int max_active, StartCallback start)
      : max_active_(max_active), start_(start) {}

  JoinLeaveQueue(JoinLeaveQueue&) = delete;
  JoinLeaveQueue& operator=(JoinLeaveQueue&) = delete;

  ~JoinLeaveQueue() = default;

  // If there are fewer than `max_active` operations, immediately invokes
  // `start` with operation. Otherwise enqueues `operation`.
  void Enqueue(T&& operation) {
    if (num_active_ < max_active_) {
      ++num_active_;
      start_.Run(std::move(operation));
      return;
    }

    queue_.push_back(std::move(operation));
  }

  // Called when a previously started operation completes. Starts the next
  // queued operation, if there is one.
  void OnComplete() {
    DCHECK_GT(num_active_, 0);

    if (!queue_.empty()) {
      DCHECK_EQ(num_active_, max_active_);
      start_.Run(queue_.TakeFirst());
      return;
    }

    --num_active_;
  }

  int num_active_for_testing() const { return num_active_; }

 private:
  // Maximum number of active operations.
  const int max_active_;

  // Callback to start the input operation.
  const StartCallback start_;

  // Current number of active operations. Active operations are not included in
  // `queue_`.
  int num_active_ = 0;

  // FIFO queue of operations that have not yet started.
  Deque<T> queue_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_JOIN_LEAVE_QUEUE_H_
