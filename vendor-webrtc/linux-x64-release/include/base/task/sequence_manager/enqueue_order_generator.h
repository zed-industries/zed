// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_ENQUEUE_ORDER_GENERATOR_H_
#define BASE_TASK_SEQUENCE_MANAGER_ENQUEUE_ORDER_GENERATOR_H_

#include <stdint.h>

#include <atomic>

#include "base/base_export.h"
#include "base/task/sequence_manager/enqueue_order.h"

namespace base {
namespace sequence_manager {
namespace internal {

// EnqueueOrder can't be created from a raw number in non-test code.
// EnqueueOrderGenerator is used to create it with strictly monotonic guarantee.
class BASE_EXPORT EnqueueOrderGenerator {
 public:
  EnqueueOrderGenerator();
  EnqueueOrderGenerator(const EnqueueOrderGenerator&) = delete;
  EnqueueOrderGenerator& operator=(const EnqueueOrderGenerator&) = delete;
  ~EnqueueOrderGenerator();

  // Can be called from any thread.
  EnqueueOrder GenerateNext() {
    return EnqueueOrder(std::atomic_fetch_add_explicit(
        &counter_, uint64_t(1), std::memory_order_relaxed));
  }

 private:
  std::atomic<uint64_t> counter_;
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_ENQUEUE_ORDER_GENERATOR_H_
