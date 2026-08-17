// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_SINGLE_THREAD_IDLE_TASK_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_SINGLE_THREAD_IDLE_TASK_RUNNER_H_

#include <map>
#include <utility>

#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {
namespace scheduler {
class IdleHelper;

// A SingleThreadIdleTaskRunner is a task runner for running idle tasks. Idle
// tasks have an unbound argument which is bound to a deadline
// (in base::TimeTicks) when they are run. The idle task is expected to
// complete by this deadline.
class SingleThreadIdleTaskRunner
    : public WTF::ThreadSafeRefCounted<SingleThreadIdleTaskRunner> {
 public:
  using IdleTask = base::OnceCallback<void(base::TimeTicks)>;

  // Used to request idle task deadlines and signal posting of idle tasks.
  class PLATFORM_EXPORT Delegate {
   public:
    Delegate();
    Delegate(const Delegate&) = delete;
    Delegate& operator=(const Delegate&) = delete;
    virtual ~Delegate();

    // Signals that an idle task has been posted. This will be called on the
    // posting thread, which may not be the same thread as the
    // SingleThreadIdleTaskRunner runs on.
    virtual void OnIdleTaskPosted() = 0;

    // Signals that a new idle task is about to be run and returns the deadline
    // for this idle task.
    virtual base::TimeTicks WillProcessIdleTask() = 0;

    // Signals that an idle task has finished being run.
    virtual void DidProcessIdleTask() = 0;

    // Returns the current time.
    virtual base::TimeTicks NowTicks() = 0;
  };

  // NOTE Category strings must have application lifetime (statics or
  // literals). They may not include " chars.
  SingleThreadIdleTaskRunner(
      scoped_refptr<base::SingleThreadTaskRunner> idle_priority_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> control_task_runner,
      Delegate* delegate);
  SingleThreadIdleTaskRunner(const SingleThreadIdleTaskRunner&) = delete;
  SingleThreadIdleTaskRunner& operator=(const SingleThreadIdleTaskRunner&) =
      delete;

  virtual void PostIdleTask(const base::Location& from_here,
                            IdleTask idle_task);

  // |idle_task| is eligible to run after the next time an idle period starts
  // after |delay|.  Note this has after wake-up semantics, i.e. unless
  // something else wakes the CPU up, this won't run.
  virtual void PostDelayedIdleTask(const base::Location& from_here,
                                   const base::TimeDelta delay,
                                   IdleTask idle_task);

  bool RunsTasksInCurrentSequence() const;

 protected:
  virtual ~SingleThreadIdleTaskRunner();

 private:
  friend class WTF::ThreadSafeRefCounted<SingleThreadIdleTaskRunner>;
  friend class IdleHelper;

  void RunTask(IdleTask idle_task);

  void EnqueueReadyDelayedIdleTasks();

  void PostDelayedIdleTaskOnAssociatedThread(
      const base::Location& from_here,
      const base::TimeTicks delayed_run_time,
      IdleTask idle_task);

  using DelayedIdleTask = std::pair<const base::Location, base::OnceClosure>;

  scoped_refptr<base::SingleThreadTaskRunner> idle_priority_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> control_task_runner_;
  std::multimap<base::TimeTicks, DelayedIdleTask> delayed_idle_tasks_
      ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327)");
  raw_ptr<Delegate, DanglingUntriaged> delegate_;  // NOT OWNED
  base::WeakPtr<SingleThreadIdleTaskRunner> weak_scheduler_ptr_;
  base::WeakPtrFactory<SingleThreadIdleTaskRunner> weak_factory_{this};

 public:
  using RunTaskDecltype = decltype(&SingleThreadIdleTaskRunner::RunTask);
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_SINGLE_THREAD_IDLE_TASK_RUNNER_H_
