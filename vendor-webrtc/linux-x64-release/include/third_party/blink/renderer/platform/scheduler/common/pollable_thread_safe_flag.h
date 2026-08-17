// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_POLLABLE_THREAD_SAFE_FLAG_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_POLLABLE_THREAD_SAFE_FLAG_H_

#include <atomic>

#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

// A PollableThreadSafeFlag can be polled without requiring a lock, but can only
// be updated if a lock is held. This enables lock-free checking as to whether a
// condition has changed, while protecting operations which update the condition
// with a lock. You must ensure that the flag is only updated within the same
// lock-protected critical section as any other variables on which the condition
// depends.
class PollableThreadSafeFlag {
  DISALLOW_NEW();

 public:
  explicit PollableThreadSafeFlag(base::Lock* write_lock);
  PollableThreadSafeFlag(const PollableThreadSafeFlag&) = delete;
  PollableThreadSafeFlag& operator=(const PollableThreadSafeFlag&) = delete;

  // Set the flag. May only be called if |write_lock| is held.
  void SetWhileLocked(bool value);

  // Returns true iff the flag is set to true.
  bool IsSet() const;

 private:
  std::atomic<bool> flag_;
  raw_ptr<base::Lock> write_lock_;  // Not owned.
};

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_POLLABLE_THREAD_SAFE_FLAG_H_
