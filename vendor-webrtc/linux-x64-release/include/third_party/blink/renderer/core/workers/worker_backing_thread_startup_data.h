// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_BACKING_THREAD_STARTUP_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_BACKING_THREAD_STARTUP_DATA_H_

#include <optional>

#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace blink {

// WorkerBackingThreadStartupData contains parameters for starting
// WorkerBackingThread.
struct WorkerBackingThreadStartupData {
 public:
  enum class HeapLimitMode { kDefault, kIncreasedForDebugging };
  enum class AtomicsWaitMode { kDisallow, kAllow };

  WorkerBackingThreadStartupData(HeapLimitMode heap_limit_mode,
                                 AtomicsWaitMode atomics_wait_mode)
      : heap_limit_mode(heap_limit_mode),
        atomics_wait_mode(atomics_wait_mode) {}

  static WorkerBackingThreadStartupData CreateDefault() {
    return WorkerBackingThreadStartupData(HeapLimitMode::kDefault,
                                          AtomicsWaitMode::kDisallow);
  }

  HeapLimitMode heap_limit_mode;
  AtomicsWaitMode atomics_wait_mode;
};

}  // namespace blink

namespace WTF {

// This allows to pass std::optional<WorkerBackingThreadStartupData> across
// threads by PostTask().
template <>
struct CrossThreadCopier<std::optional<blink::WorkerBackingThreadStartupData>>
    : public CrossThreadCopierPassThrough<
          std::optional<blink::WorkerBackingThreadStartupData>> {};

// This allows to pass WorkerBackingThreadStartupData across threads by
// PostTask().
template <>
struct CrossThreadCopier<blink::WorkerBackingThreadStartupData>
    : public CrossThreadCopierPassThrough<
          blink::WorkerBackingThreadStartupData> {};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_BACKING_THREAD_STARTUP_DATA_H_
