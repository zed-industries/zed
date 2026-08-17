/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_BASE_REF_COUNTER_H_
#define RTC_BASE_REF_COUNTER_H_

#include <atomic>

#include "rtc_base/ref_count.h"

namespace webrtc {
namespace webrtc_impl {

class RefCounter {
 public:
  explicit RefCounter(int ref_count) : ref_count_(ref_count) {}
  RefCounter() = delete;

  void IncRef() {
    // Relaxed memory order: The current thread is allowed to act on the
    // resource protected by the reference counter both before and after the
    // atomic op, so this function doesn't prevent memory access reordering.
    ref_count_.fetch_add(1, std::memory_order_relaxed);
  }

  // Returns kDroppedLastRef if this call dropped the last reference; the caller
  // should therefore free the resource protected by the reference counter.
  // Otherwise, returns kOtherRefsRemained (note that in case of multithreading,
  // some other caller may have dropped the last reference by the time this call
  // returns; all we know is that we didn't do it).
  RefCountReleaseStatus DecRef() {
    // Use release-acquire barrier to ensure all actions on the protected
    // resource are finished before the resource can be freed.
    // When ref_count_after_subtract > 0, this function require
    // std::memory_order_release part of the barrier.
    // When ref_count_after_subtract == 0, this function require
    // std::memory_order_acquire part of the barrier.
    // In addition std::memory_order_release is used for synchronization with
    // the HasOneRef function to make sure all actions on the protected resource
    // are finished before the resource is assumed to have exclusive access.
    int ref_count_after_subtract =
        ref_count_.fetch_sub(1, std::memory_order_acq_rel) - 1;
    return ref_count_after_subtract == 0
               ? RefCountReleaseStatus::kDroppedLastRef
               : RefCountReleaseStatus::kOtherRefsRemained;
  }

  // Return whether the reference count is one. If the reference count is used
  // in the conventional way, a reference count of 1 implies that the current
  // thread owns the reference and no other thread shares it. This call performs
  // the test for a reference count of one, and performs the memory barrier
  // needed for the owning thread to act on the resource protected by the
  // reference counter, knowing that it has exclusive access.
  bool HasOneRef() const {
    // To ensure resource protected by the reference counter has exclusive
    // access, all changes to the resource before it was released by other
    // threads must be visible by current thread. That is provided by release
    // (in DecRef) and acquire (in this function) ordering.
    return ref_count_.load(std::memory_order_acquire) == 1;
  }

 private:
  std::atomic<int> ref_count_;
};

}  // namespace webrtc_impl
}  // namespace webrtc

#endif  // RTC_BASE_REF_COUNTER_H_
