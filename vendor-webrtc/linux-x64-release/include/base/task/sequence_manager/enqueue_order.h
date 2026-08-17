// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_ENQUEUE_ORDER_H_
#define BASE_TASK_SEQUENCE_MANAGER_ENQUEUE_ORDER_H_

#include <stdint.h>

#include <limits>

namespace base {
namespace sequence_manager {

namespace internal {
class EnqueueOrderGenerator;
}

// 64-bit number which is used to order tasks.
// SequenceManager assumes this number will never overflow.
class EnqueueOrder {
 public:
  EnqueueOrder() : value_(kNone) {}
  ~EnqueueOrder() = default;

  static EnqueueOrder none() { return EnqueueOrder(kNone); }
  static EnqueueOrder blocking_fence() { return EnqueueOrder(kBlockingFence); }

  // Returns an EnqueueOrder that compares greater than any other EnqueueOrder.
  static EnqueueOrder max() {
    return EnqueueOrder(std::numeric_limits<uint64_t>::max());
  }

  // It's okay to use EnqueueOrder in boolean expressions keeping in mind
  // that some non-zero values have a special meaning.
  operator uint64_t() const { return value_; }

  static EnqueueOrder FromIntForTesting(uint64_t value) {
    return EnqueueOrder(value);
  }

 private:
  // EnqueueOrderGenerator is the only class allowed to create an EnqueueOrder
  // with a non-default constructor.
  friend class internal::EnqueueOrderGenerator;

  explicit EnqueueOrder(uint64_t value) : value_(value) {}

  enum SpecialValues : uint64_t {
    kNone = 0,
    kBlockingFence = 1,
    kFirst = 2,
  };

  uint64_t value_;
};

}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_ENQUEUE_ORDER_H_
