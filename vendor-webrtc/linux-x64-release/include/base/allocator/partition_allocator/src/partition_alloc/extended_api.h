// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_EXTENDED_API_H_
#define PARTITION_ALLOC_EXTENDED_API_H_

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_root.h"
#include "partition_alloc/partition_stats.h"
#include "partition_alloc/thread_cache.h"

namespace partition_alloc::internal {
// Get allocation stats for the thread cache partition on the current
// thread. See the documentation of ThreadAllocStats for details.
ThreadAllocStats GetAllocStatsForCurrentThread();

// Creates a scope for testing which:
// - if the given |root| is a default malloc root for the entire process,
//   enables the thread cache for the entire process.
//   (This may happen if UsePartitionAllocAsMalloc is enabled.)
// - otherwise, disables the thread cache for the entire process, and
//   replaces it with a thread cache for |root|.
// This class is unsafe to run if there are multiple threads running
// in the process.
class ThreadCacheProcessScopeForTesting {
 public:
  explicit ThreadCacheProcessScopeForTesting(PartitionRoot* root);
  ~ThreadCacheProcessScopeForTesting();

  ThreadCacheProcessScopeForTesting() = delete;

 private:
  PartitionRoot* root_ = nullptr;
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  bool regular_was_enabled_ = false;
#endif
};

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_EXTENDED_API_H_
