// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_POST_DELAYED_MEMORY_REDUCTION_TASK_H_
#define BASE_MEMORY_POST_DELAYED_MEMORY_REDUCTION_TASK_H_

#include "base/task/sequenced_task_runner.h"

namespace base {

// Context in which a memory reduction task is invoked.
enum class MemoryReductionTaskContext {
  // After the expiration of its delay.
  kDelayExpired,
  // Before the expiration of its delay, to proactively reduce memory.
  kProactive,
};

// This API should be used for posting delayed tasks that reduce memory usage
// while Chrome is backgrounded. On Android 14+, tasks posted this way may be
// run before the delay is elapsed, in the case where Chrome is about to be
// frozen by Android. On other platforms, this is equivalent to directly posting
// the delayed task, using the task runner.
void BASE_EXPORT
PostDelayedMemoryReductionTask(scoped_refptr<SequencedTaskRunner> task_runner,
                               const Location& from_here,
                               OnceClosure task,
                               base::TimeDelta delay);

// Same as above, but passes a parameter to the task, depending on how it was
// run. On non-Android platforms, will always pass |kDelayExpired|.
void BASE_EXPORT PostDelayedMemoryReductionTask(
    scoped_refptr<SequencedTaskRunner> task_runner,
    const Location& from_here,
    OnceCallback<void(MemoryReductionTaskContext)> task,
    base::TimeDelta delay);

// Replacement for |OneShotTimer|, that allows the tasks to be run by
// |OnPreFreeze| (see |PreFreezeBackgroundMemoryTrimmer| above).
class BASE_EXPORT OneShotDelayedBackgroundTimer final {
 public:
  OneShotDelayedBackgroundTimer();
  ~OneShotDelayedBackgroundTimer();

  void Stop();

  void Start(const Location& posted_from, TimeDelta delay, OnceClosure task) {
    Start(posted_from, delay,
          BindOnce(
              [](OnceClosure task,
                 MemoryReductionTaskContext called_from_prefreeze) {
                std::move(task).Run();
              },
              std::move(task)));
  }
  void Start(const Location& posted_from,
             TimeDelta delay,
             OnceCallback<void(MemoryReductionTaskContext)> task);

  bool IsRunning() const;

  template <class Receiver>
  void Start(const Location& posted_from,
             TimeDelta delay,
             Receiver* receiver,
             void (Receiver::*method)()) {
    Start(posted_from, delay, BindOnce(method, Unretained(receiver)));
  }

  void SetTaskRunner(scoped_refptr<SequencedTaskRunner> task_runner);

 private:
  class OneShotDelayedBackgroundTimerImpl {
   public:
    virtual ~OneShotDelayedBackgroundTimerImpl() = default;
    virtual void Stop() = 0;
    virtual void Start(const Location& posted_from,
                       TimeDelta delay,
                       OnceCallback<void(MemoryReductionTaskContext)> task) = 0;
    virtual bool IsRunning() const = 0;
    virtual void SetTaskRunner(
        scoped_refptr<SequencedTaskRunner> task_runner) = 0;
  };

#if BUILDFLAG(IS_ANDROID)
  friend class android::PreFreezeBackgroundMemoryTrimmer;
#endif
  class TimerImpl;
  class TaskImpl;

  std::unique_ptr<OneShotDelayedBackgroundTimerImpl> impl_;
};

}  // namespace base

#endif  // BASE_MEMORY_POST_DELAYED_MEMORY_REDUCTION_TASK_H_
