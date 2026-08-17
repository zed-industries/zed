// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ONE_SHOT_EVENT_H_
#define BASE_ONE_SHOT_EVENT_H_

#include <vector>

#include "base/base_export.h"
#include "base/check.h"
#include "base/functional/callback_forward.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"

namespace base {

class Location;
class TimeDelta;

// This class represents an event that's expected to happen once. It allows
// clients to guarantee that code is run after the `OneShotEvent` is signaled.
// If the `OneShotEvent` is destroyed before it's signaled, the closures are
// destroyed without being run.
//
// This class is similar to a `WaitableEvent` combined with several
// `WaitableEventWatcher`s, but using it is simpler.
//
// This class' methods must be used from a single sequence (although not
// necessarily the one in which it has been constructed).
// However, there are no restrictions on the `TaskRunner`s used - and hence, the
// sequence/thread on which the posted tasks will run. By default they will
// be posted to the current sequence's default task runner.
class BASE_EXPORT OneShotEvent {
 public:
  OneShotEvent();
  // Use the following constructor to create an already signaled event. This is
  // useful if you construct the event on a different thread from where it is
  // used, in which case it is not possible to call `Signal` just after
  // construction.
  explicit OneShotEvent(bool signaled);
  ~OneShotEvent();

  // True if `Signal` has been called. This function is mostly for migrating old
  // code; usually calling `Post` unconditionally will result in more readable
  // code.
  bool is_signaled() const {
    DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
    return signaled_;
  }

  // Causes `is_signaled` to return true and all tasks to be posted to their
  // corresponding task runners in the FIFO order. Note that tasks posted to
  // different `TaskRunner`s may still execute in arbitrary order. This
  // method must only be called once.
  void Signal();

  // Schedules `task` to be called on `runner` after `is_signaled` becomes
  // `true`. If called with `delay`, then the task will happen (roughly) `delay`
  // after `is_signaled`, *not* `delay` after the post. Inside `task`, if this
  // `OneShotEvent` is still alive, `CHECK(is_signaled())` will never fail
  // (which implies that `OneShotEvent::Reset` doesn't exist).
  //
  // If `*this` is destroyed before being released, none of these tasks will be
  // executed.
  //
  // Tasks are posted in FIFO order, however, tasks may still execute in an
  // arbitrary order (specified by the combination and type of `TaskRunner`s
  // used). Tasks will never be called on the current sequence before this
  // function returns.
  // Beware that there's no simple way to wait for all tasks on a `OneShotEvent`
  // to complete, so it's almost never safe to use `base::Unretained` when
  // creating one.
  void Post(const Location& from_here,
            OnceClosure task,
            scoped_refptr<TaskRunner> runner =
                SequencedTaskRunner::GetCurrentDefault()) const;
  void PostDelayed(const Location& from_here,
                   OnceClosure task,
                   const TimeDelta& delay,
                   scoped_refptr<TaskRunner> runner =
                       SequencedTaskRunner::GetCurrentDefault()) const;

 private:
  struct TaskInfo;
  SEQUENCE_CHECKER(sequence_checker_);

  bool signaled_ = false;

  // The task list is mutable because it's not part of the logical state of the
  // object. This lets us return const references to the `OneShotEvent` to
  // clients that just want to run tasks through it without worrying that
  // they'll signal the event.
  //
  // Optimization note: We could reduce the size of this class to a single
  // pointer by storing `signaled_` in the low bit of a pointer, and storing the
  // size and capacity of the array (if any) on the far end of the pointer.
  mutable std::vector<TaskInfo> tasks_;
};

}  // namespace base

#endif  // BASE_ONE_SHOT_EVENT_H_
