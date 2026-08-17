// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_LIGHTWEIGHT_QUARANTINE_SUPPORT_H_
#define PARTITION_ALLOC_LIGHTWEIGHT_QUARANTINE_SUPPORT_H_

#include <optional>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/lightweight_quarantine.h"
#include "partition_alloc/thread_cache.h"

// Extra utilities for Lightweight Quarantine.
// This is a separate header to avoid cyclic reference between "thread_cache.h"
// and "lightweight_quarantine.h".

namespace partition_alloc {

// When this class is alive, Scheduler-Loop Quarantine for this thread is
// paused and freed allocations will be freed immediately.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC)
    ScopedSchedulerLoopQuarantineExclusion {
 public:
  ScopedSchedulerLoopQuarantineExclusion();
  ~ScopedSchedulerLoopQuarantineExclusion();

 private:
  std::optional<
      internal::SchedulerLoopQuarantineBranch::ScopedQuarantineExclusion>
      instance_;
};

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_LIGHTWEIGHT_QUARANTINE_SUPPORT_H_
