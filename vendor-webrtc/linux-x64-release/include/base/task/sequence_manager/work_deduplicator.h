// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_WORK_DEDUPLICATOR_H_
#define BASE_TASK_SEQUENCE_MANAGER_WORK_DEDUPLICATOR_H_

#include <atomic>

#include "base/base_export.h"
#include "base/task/sequence_manager/associated_thread_id.h"

namespace base {
namespace sequence_manager {
namespace internal {

// This class's job is to prevent redundant DoWorks being posted, which are
// expensive. The idea is a DoWork will (maybe) run a task before computing the
// delay till the next task. If the task run posts another task, we don't want
// it to schedule work because the DoWork will post a continuation as needed
// with the latest state taken into consideration (fences, enable / disable
// queue, task cancellation, etc...) Other threads can also post DoWork at any
// time, including while we're computing the delay till the next task. To
// account for that, we have split a DoWork up into two sections:
// [OnWorkStarted .. WillCheckForMoreWork] and
// [WillCheckForMoreWork .. DidCheckForMoreWork] where DidCheckForMoreWork
// detects if another thread called OnWorkRequested.
//
// Nesting is assumed to be dealt with by the ThreadController.
//
// Most methods are thread-affine except for On(Delayed)WorkRequested which are
// is thread-safe.
class BASE_EXPORT WorkDeduplicator {
 public:
  // Creates an unbound WorkDeduplicator. BindToCurrentThread must be called
  // before work can be scheduled.
  explicit WorkDeduplicator(
      scoped_refptr<const AssociatedThreadId> associated_thread);

  ~WorkDeduplicator();

  enum ShouldScheduleWork {
    kScheduleImmediate,
    kNotNeeded,
  };

  // Returns ShouldScheduleWork::kSchedule if OnWorkRequested was called while
  // unbound. Must be called on the associated thread.
  ShouldScheduleWork BindToCurrentThread();

  // Returns true if it's OK to schedule a DoWork without risk of task
  // duplication. Returns false if:
  // * We are unbound
  // * We are in a DoWork
  // * There is a pending DoWork
  //
  // Otherwise sets the pending DoWork flag and returns true.
  // Can be called on any thread.
  //
  //    DoWork
  //    ---------------------------------------------------------------------
  //    | <- OnWorkStarted                       |                          |
  //    |                WillCheckForMoreWork -> |                          |
  //    |                                        |   DidCheckForMoreWork -> |
  //    ---------------------------------------------------------------------
  // ^                            ^                           ^               ^
  // |                            |                           |               |
  // A                            B                           C               D
  //
  // Consider a DoWork and calls to OnWorkRequested at various times:
  // A: return ShouldScheduleWork::kNotNeeded because there's a pending DoWork.
  // B: return ShouldScheduleWork::kNotNeeded because we're in a DoWork.
  // C: return ShouldScheduleWork::kNotNeeded because we're in a DoWork, however
  //    DidCheckForMoreWork should subsequently return
  //    ShouldScheduleWork::kScheduleImmediate.
  // D: If DidCheckForMoreWork(NextTask::kIsImmediate) was called then it
  //    should ShouldScheduleWork::kNotNeeded because there's a pending DoWork.
  //    Otherwise it should return ShouldScheduleWork::kScheduleImmediate, but a
  //    subsequent call to OnWorkRequested should return
  //    ShouldScheduleWork::kNotNeeded because there's now a pending DoWork.
  ShouldScheduleWork OnWorkRequested();

  // Returns ShouldScheduleWork::kScheduleImmediate if it's OK to schedule a
  // DoDelayedWork without risk of redundancy. Deduplication of delayed work is
  // assumed to have been done by the caller, the purpose of this method it to
  // check if there's a pending DoWork which would schedule a delayed
  // continuation as needed.
  //
  // Returns ShouldScheduleWork::kNotNeeded if:
  // * We are unbound
  // * We are in a DoWork
  // * There is a pending DoWork
  //
  // Must be called on the associated thread.
  ShouldScheduleWork OnDelayedWorkRequested() const;

  // Marks us as having entered a DoWork, clearing the pending DoWork flag.
  // Must be called on the associated thread.
  void OnWorkStarted();

  // Marks us as being about to check if we have more work. This notification
  // helps prevent DoWork duplication in two scenarios:
  // * A cross-thread immediate task is posted while we are running a task. If
  //   the TaskQueue is disabled we can avoid a potentially spurious DoWork.
  // * A task is run which posts an immediate task but the ThreadControllerImpl
  //   work batch size is 2, and there's no further work. The immediate task ran
  //   in the work batch so we don't need another DoWork.
  void WillCheckForMoreWork();

  enum NextTask {
    kIsImmediate,
    kIsDelayed,
  };

  // Marks us as exiting DoWork. Returns ShouldScheduleWork::kScheduleImmediate
  // if an immediate DoWork continuation should be posted. This method
  // atomically takes into account any OnWorkRequested's called between
  // gathering information about |next_task| and this call. Must be called on
  // the associated thread.
  ShouldScheduleWork DidCheckForMoreWork(NextTask next_task);

 private:
  enum Flags {
    kInDoWorkFlag = 1 << 0,
    kPendingDoWorkFlag = 1 << 1,
    kBoundFlag = 1 << 2,
  };

  enum State {
    kUnbound = 0,
    kIdle = Flags::kBoundFlag,
    kDoWorkPending = Flags::kPendingDoWorkFlag | Flags::kBoundFlag,
    kInDoWork = Flags::kInDoWorkFlag | Flags::kBoundFlag,
  };

  std::atomic<int> state_{State::kUnbound};

  const scoped_refptr<const AssociatedThreadId> associated_thread_;
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_WORK_DEDUPLICATOR_H_
