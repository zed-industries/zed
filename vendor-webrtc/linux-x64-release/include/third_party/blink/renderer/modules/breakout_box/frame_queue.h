// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_FRAME_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_FRAME_QUEUE_H_

#include <optional>

#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

// Implements a thread-safe circular queue.
template <typename NativeFrameType>
class FrameQueue
    : public WTF::ThreadSafeRefCounted<FrameQueue<NativeFrameType>> {
 public:
  explicit FrameQueue(wtf_size_t max_size)
      : max_size_(std::max(1u, max_size)) {}

  base::Lock& GetLock() { return lock_; }

  std::optional<NativeFrameType> Push(NativeFrameType frame) {
    base::AutoLock locker_(GetLock());
    return PushLocked(std::move(frame));
  }

  std::optional<NativeFrameType> PushLocked(NativeFrameType frame)
      EXCLUSIVE_LOCKS_REQUIRED(GetLock()) {
    std::optional<NativeFrameType> ret;
    if (queue_.size() == max_size_)
      ret = queue_.TakeFirst();
    queue_.push_back(std::move(frame));
    return ret;
  }

  std::optional<NativeFrameType> Pop() {
    base::AutoLock locker_(GetLock());
    return PopLocked();
  }

  std::optional<NativeFrameType> PopLocked()
      EXCLUSIVE_LOCKS_REQUIRED(GetLock()) {
    if (queue_.empty())
      return std::nullopt;
    return queue_.TakeFirst();
  }

  std::optional<NativeFrameType> PeekLocked()
      EXCLUSIVE_LOCKS_REQUIRED(GetLock()) {
    if (queue_.empty())
      return std::nullopt;
    return queue_.front();
  }

  bool IsEmpty() {
    base::AutoLock locker_(GetLock());
    return IsEmptyLocked();
  }

  bool IsEmptyLocked() EXCLUSIVE_LOCKS_REQUIRED(GetLock()) {
    return queue_.empty();
  }

  wtf_size_t MaxSize() const { return max_size_; }

  void Clear() {
    base::AutoLock locker_(GetLock());
    queue_.clear();
  }

 private:
  base::Lock lock_;
  Deque<NativeFrameType> queue_ GUARDED_BY(GetLock());
  const wtf_size_t max_size_;
};

// Wrapper that allows sharing a single FrameQueue reference across multiple
// threads.
template <typename NativeFrameType>
class FrameQueueHandle {
 public:
  explicit FrameQueueHandle(
      scoped_refptr<FrameQueue<NativeFrameType>> frame_queue)
      : queue_(std::move(frame_queue)) {
    DCHECK(queue_);
  }
  FrameQueueHandle(const FrameQueueHandle&) = delete;
  FrameQueueHandle& operator=(const FrameQueueHandle&) = delete;

  ~FrameQueueHandle() { Invalidate(); }

  // Returns a copy of |queue_|, which should be checked for nullity and
  // re-used throughout the scope of a function call, instead of calling
  // Queue() multiple times.  Otherwise the queue could be destroyed
  // between calls.
  scoped_refptr<FrameQueue<NativeFrameType>> Queue() const {
    base::AutoLock locker(lock_);
    return queue_;
  }

  // Releases the internal FrameQueue reference.
  void Invalidate() {
    base::AutoLock locker(lock_);
    queue_.reset();
  }

 private:
  mutable base::Lock lock_;
  scoped_refptr<FrameQueue<NativeFrameType>> queue_ GUARDED_BY(lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_FRAME_QUEUE_H_
