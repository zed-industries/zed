// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_WORK_TRACKER_H_
#define BASE_TASK_SEQUENCE_MANAGER_WORK_TRACKER_H_

#include <atomic>
#include <cstdint>

#include "base/base_export.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/synchronization/condition_variable.h"
#include "base/task/common/checked_lock.h"
#include "base/threading/thread_checker.h"

namespace base::sequence_manager::internal {

class WorkTracker;

// When `IsValid()`, this represents an authorization to execute work
// synchronously inside `RunOrPostTask`.
class BASE_EXPORT SyncWorkAuthorization {
 public:
  SyncWorkAuthorization(SyncWorkAuthorization&&);
  SyncWorkAuthorization& operator=(SyncWorkAuthorization&&);
  ~SyncWorkAuthorization();

  bool IsValid() const { return !!tracker_; }

 private:
  friend class WorkTracker;

  explicit SyncWorkAuthorization(WorkTracker* state);

  // RAW_PTR_EXCLUSION: Performance reasons (based on analysis of speedometer3).
  RAW_PTR_EXCLUSION WorkTracker* tracker_ = nullptr;
};

// Tracks queued and running work to support `RunOrPostTask`.
class BASE_EXPORT WorkTracker {
 public:
  WorkTracker();
  ~WorkTracker();

  // Controls whether `RunOrPostTask()` can run its callback synchronously when
  // no work is tracked by this. Don't allow this when work that is sequenced
  // with `RunOrPostTask()` may run without being tracked by methods below.
  void SetRunTaskSynchronouslyAllowed(bool can_run_tasks_synchronously);

  // Invoked before requesting to reload an empty immediate work queue. After
  // this, `RunOrPostTask()` can't run tasks synchronously until
  // `WillReloadImmediateWorkQueues()` and `OnIdle()` have been called in
  // sequence.
  void WillRequestReloadImmediateWorkQueue();

  // Invoked before reloading empty immediate work queues.
  void WillReloadImmediateWorkQueues();

  // Invoked before doing work. After this `RunOrPostTask()` can't run tasks
  // until `OnIdle()` is called. Work may begin even if immediate work queues
  // haven't be reloaded since the last `OnIdle()`, e.g. when a task queue is
  // enabled, when tasks are moved from the delayed incoming queue to the
  // delayed work queue or when the pump performs internal work.
  void OnBeginWork();

  // Invoked when the thread is out of work.
  void OnIdle();

  // Returns a valid `SyncWorkAuthorization` iff all these conditions are true:
  // - Explicitly allowed by `SetRunTaskSynchronouslyAllowed()`
  // - `WillReloadImmediateWorkQueues()` and `OnIdle()` were called in
  //   sequence after the last call to `WillRequestReloadImmediateWorkQueue()`
  // - `OnIdle()` was called after the last call to `OnBeginWork()`
  SyncWorkAuthorization TryAcquireSyncWorkAuthorization();

  // Asserts that there is work tracked by this, i.e.
  // `TryAcquireSyncWorkAuthorization()` would not grant a sync work
  // authorization even if allowed by `SetRunTaskSynchronouslyAllowed()`.
  void AssertHasWork();

 private:
  friend class SyncWorkAuthorization;

  void WaitNoSyncWork();

  // An atomic variable to track:
  // - Whether there is an unfulfilled request to reload immediate work queues.
  static constexpr uint32_t kImmediateWorkQueueNeedsReload = 1 << 0;
  // - Whether all work queues are empty and no work is running.
  static constexpr uint32_t kWorkQueuesEmptyAndNoWorkRunning = 1 << 1;
  // - Whether a valid `SyncWorkAuthorization` exists.
  static constexpr uint32_t kActiveSyncWork = 1 << 2;
  // - Whether a valid `SyncWorkAuthorization` can be granted when no work is
  //   tracked by `this`.
  static constexpr uint32_t kSyncWorkSupported = 1 << 3;
  std::atomic_uint32_t state_{kWorkQueuesEmptyAndNoWorkRunning};

  // Memory order for `state_`:
  //
  // Sync work must see all memory written before it was allowed. Similarly,
  // non-sync work must see all memory written by sync work. As a result:
  //
  // Operations that may allow sync work are std::memory_order_release:
  //    - Set `kWorkQueuesEmptyAndNoWorkRunning`
  //    - Set `kSyncWorkSupported`
  //
  // Operations that may allow non-sync work are `std::memory_order_release`:
  //    - Clear `kActiveSyncWork`
  //
  // Operations that precede sync work are `std::memory_order_acquire`:
  //    - Set `kActiveSyncWork`
  //
  // Operations that precede non-sync work are `std::memory_order_acquire`:
  //    - Check that `kActiveSyncWork` is not set.
  static constexpr std::memory_order kMemoryReleaseAllowWork =
      std::memory_order_release;
  static constexpr std::memory_order kMemoryAcquireBeforeWork =
      std::memory_order_acquire;
  static constexpr std::memory_order kMemoryRelaxedNotAllowOrBeforeWork =
      std::memory_order_relaxed;

  // Allows `OnBeginWork()` to wait until there is no more valid
  // `SyncWorkAuthorization`.
  base::internal::CheckedLock active_sync_work_lock_;
  ConditionVariable active_sync_work_cv_ =
      active_sync_work_lock_.CreateConditionVariable();

  THREAD_CHECKER(thread_checker_);
};

}  // namespace base::sequence_manager::internal

#endif  // BASE_TASK_SEQUENCE_MANAGER_WORK_TRACKER_H_
