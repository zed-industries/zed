// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// ConditionVariable wraps pthreads condition variable synchronization or, on
// Windows, simulates it.  This functionality is very helpful for having
// several threads wait for an event, as is common with a thread pool managed
// by a master.  The meaning of such an event in the (worker) thread pool
// scenario is that additional tasks are now available for processing.  It is
// used in Chrome in the DNS prefetching system to notify worker threads that
// a queue now has items (tasks) which need to be tended to.  A related use
// would have a pool manager waiting on a ConditionVariable, waiting for a
// thread in the pool to announce (signal) that there is now more room in a
// (bounded size) communications queue for the manager to deposit tasks, or,
// as a second example, that the queue of tasks is completely empty and all
// workers are waiting.
//
// USAGE NOTE 1: spurious signal events are possible with this and
// most implementations of condition variables.  As a result, be
// *sure* to retest your condition before proceeding.  The following
// is a good example of doing this correctly:
//
// while (!work_to_be_done()) Wait(...);
//
// In contrast do NOT do the following:
//
// if (!work_to_be_done()) Wait(...);  // Don't do this.
//
// Especially avoid the above if you are relying on some other thread only
// issuing a signal up *if* there is work-to-do.  There can/will
// be spurious signals.  Recheck state on waiting thread before
// assuming the signal was intentional. Caveat caller ;-).
//
// USAGE NOTE 2: Broadcast() frees up all waiting threads at once,
// which leads to contention for the locks they all held when they
// called Wait().  This results in POOR performance.  A much better
// approach to getting a lot of threads out of Wait() is to have each
// thread (upon exiting Wait()) call Signal() to free up another
// Wait'ing thread.  Look at condition_variable_unittest.cc for
// both examples.
//
// Broadcast() can be used nicely during teardown, as it gets the job
// done, and leaves no sleeping threads... and performance is less
// critical at that point.
//
// The semantics of Broadcast() are carefully crafted so that *all*
// threads that were waiting when the request was made will indeed
// get signaled.  Some implementations mess up, and don't signal them
// all, while others allow the wait to be effectively turned off (for
// a while while waiting threads come around).  This implementation
// appears correct, as it will not "lose" any signals, and will guarantee
// that all threads get signaled by Broadcast().
//
// This implementation offers support for "performance" in its selection of
// which thread to revive.  Performance, in direct contrast with "fairness,"
// assures that the thread that most recently began to Wait() is selected by
// Signal to revive.  Fairness would (if publicly supported) assure that the
// thread that has Wait()ed the longest is selected. The default policy
// may improve performance, as the selected thread may have a greater chance of
// having some of its stack data in various CPU caches.

#ifndef BASE_SYNCHRONIZATION_CONDITION_VARIABLE_H_
#define BASE_SYNCHRONIZATION_CONDITION_VARIABLE_H_

#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
#include <pthread.h>
#endif

#include "base/base_export.h"
#include "base/synchronization/lock.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/windows_types.h"
#endif

namespace base {

class TimeDelta;

class BASE_EXPORT ConditionVariable {
 public:
  // Construct a cv for use with ONLY one user lock.
  explicit ConditionVariable(Lock* user_lock);

  ConditionVariable(const ConditionVariable&) = delete;
  ConditionVariable& operator=(const ConditionVariable&) = delete;

  ~ConditionVariable();

#if BUILDFLAG(IS_APPLE)
  // Initializes features for this class. See `base::features::Init()`.
  static void InitializeFeatures();
#endif

  // Wait() releases the caller's critical section atomically as it starts to
  // sleep, and the reacquires it when it is signaled. The wait functions are
  // susceptible to spurious wakeups. (See usage note 1 for more details.)
  NOT_TAIL_CALLED void Wait();
  NOT_TAIL_CALLED void TimedWait(const TimeDelta& max_time);

  // Broadcast() revives all waiting threads. (See usage note 2 for more
  // details.)
  void Broadcast();
  // Signal() revives one waiting thread.
  void Signal();

  // Declares that this ConditionVariable will only ever be used by a thread
  // that is idle at the bottom of its stack and waiting for work (in
  // particular, it is not synchronously waiting on this ConditionVariable
  // before resuming ongoing work). This is useful to avoid telling
  // base-internals that this thread is "blocked" when it's merely idle and
  // ready to do work. As such, this is only expected to be used by thread and
  // thread pool impls.
  void declare_only_used_while_idle() { waiting_is_blocking_ = false; }

 private:
#if BUILDFLAG(IS_WIN)
  CHROME_CONDITION_VARIABLE cv_;
  const raw_ptr<CHROME_SRWLOCK> srwlock_;
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
  pthread_cond_t condition_;
  raw_ptr<pthread_mutex_t> user_mutex_;
#endif

#if DCHECK_IS_ON()
  const raw_ptr<base::Lock>
      user_lock_;  // Needed to adjust shadow lock state on wait.
#endif

  // Whether a thread invoking Wait() on this ConditionalVariable should be
  // considered blocked as opposed to idle (and potentially replaced if part of
  // a pool).
  bool waiting_is_blocking_ = true;
};

}  // namespace base

#endif  // BASE_SYNCHRONIZATION_CONDITION_VARIABLE_H_
