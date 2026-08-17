/*
 * Copyright (C) 2006 Apple Computer, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TIMER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TIMER_H_

#include <optional>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/delay_policy.h"
#include "base/task/delayed_task_handle.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/sanitizers.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace base {
class TickClock;
}

namespace blink {

// Time intervals are all in seconds.

class PLATFORM_EXPORT TimerBase {
 public:
  explicit TimerBase(scoped_refptr<base::SingleThreadTaskRunner>);
  TimerBase(const TimerBase&) = delete;
  TimerBase& operator=(const TimerBase&) = delete;
  virtual ~TimerBase();

  // If |precise|, the task is scheduled with a precise delay policy to run
  // preferably as close as possible to the specified delay.
  void Start(base::TimeDelta next_fire_interval,
             std::optional<base::TimeDelta> repeat_interval,
             const base::Location&,
             bool precise = false);

  // If |precise|, the task is scheduled with a precise delay policy to run
  // preferably as close as possible to the specified delay.
  void StartRepeating(base::TimeDelta repeat_interval,
                      const base::Location& caller,
                      bool precise = false) {
    Start(repeat_interval, repeat_interval, caller, precise);
  }

  void StartOneShot(base::TimeDelta interval,
                    const base::Location& caller,
                    bool precise = false) {
    Start(interval, std::nullopt, caller, precise);
  }

  // Timer cancellation is fast enough that you shouldn't have to worry
  // about it unless you're canceling tens of thousands of tasks.
  virtual void Stop();
  bool IsActive() const;
  const base::Location& GetLocation() const { return location_; }

  base::TimeDelta NextFireInterval() const;
  std::optional<base::TimeDelta> RepeatInterval() const {
    return repeat_interval_;
  }

  void AugmentRepeatInterval(base::TimeDelta delta) {
    SetNextFireTime(next_fire_time_.is_null() ? TimerCurrentTimeTicks() + delta
                                              : next_fire_time_ + delta);
    DCHECK(repeat_interval_);
    *repeat_interval_ += delta;
  }

  void MoveToNewTaskRunner(scoped_refptr<base::SingleThreadTaskRunner>);

  void SetTaskRunnerForTesting(scoped_refptr<base::SingleThreadTaskRunner>,
                               const base::TickClock* tick_clock);

 protected:
  virtual void Fired() = 0;

  virtual base::OnceClosure BindTimerClosure() {
    return WTF::BindOnce(&TimerBase::RunInternal, WTF::Unretained(this));
  }

  void RunInternal();

 private:
  base::TimeTicks TimerCurrentTimeTicks() const;

  void SetNextFireTime(base::TimeTicks next_fire_time);

  base::TimeTicks next_fire_time_ =
      base::TimeTicks::Max();        // Max() if inactive
  std::optional<base::TimeDelta> repeat_interval_;
  base::Location location_;
  scoped_refptr<base::SingleThreadTaskRunner> web_task_runner_;
  // The tick clock used to calculate the run time for scheduled tasks.
  raw_ptr<const base::TickClock> tick_clock_ = nullptr;
  base::subtle::DelayPolicy delay_policy_;

#if DCHECK_IS_ON()
  base::PlatformThreadId thread_;
#endif
  // The handle to the posted delayed task.
  base::DelayedTaskHandle delayed_task_handle_;
};

template <typename TimerFiredClass>
class TaskRunnerTimer : public TimerBase {
 public:
  using TimerFiredFunction = void (TimerFiredClass::*)(TimerBase*);

  TaskRunnerTimer(scoped_refptr<base::SingleThreadTaskRunner> web_task_runner,
                  TimerFiredClass* o,
                  TimerFiredFunction f)
      : TimerBase(std::move(web_task_runner)), object_(o), function_(f) {
    static_assert(!WTF::IsGarbageCollectedType<TimerFiredClass>::value,
                  "Use HeapTaskRunnerTimer with garbage-collected types.");
  }

  ~TaskRunnerTimer() override = default;

 protected:
  void Fired() override { (object_->*function_)(this); }

 private:
  raw_ptr<TimerFiredClass> object_;
  TimerFiredFunction function_;
};

template <typename TimerFiredClass>
class HeapTaskRunnerTimer final : public TimerBase {
  DISALLOW_NEW();

 public:
  using TimerFiredFunction = void (TimerFiredClass::*)(TimerBase*);

  HeapTaskRunnerTimer(
      scoped_refptr<base::SingleThreadTaskRunner> web_task_runner,
      TimerFiredClass* object,
      TimerFiredFunction function)
      : TimerBase(std::move(web_task_runner)),
        object_(object),
        function_(function) {
    static_assert(
        WTF::IsGarbageCollectedType<TimerFiredClass>::value,
        "HeapTaskRunnerTimer can only be used with garbage-collected types.");
  }

  ~HeapTaskRunnerTimer() final = default;

  void Trace(Visitor* visitor) const { visitor->Trace(object_); }

 protected:
  void Fired() final { (object_->*function_)(this); }

  base::OnceClosure BindTimerClosure() final {
    return WTF::BindOnce(&HeapTaskRunnerTimer::RunInternalTrampoline,
                         WTF::Unretained(this),
                         WrapWeakPersistent(object_.Get()));
  }

 private:
  // Trampoline used for garbage-collected timer version also checks whether the
  // object has been deemed as dead by the GC but not yet reclaimed. Dead
  // objects that have not been reclaimed yet must not be touched (which is
  // enforced by ASAN poisoning).
  static void RunInternalTrampoline(HeapTaskRunnerTimer* timer,
                                    TimerFiredClass* object) {
    // |object| is null when the garbage collector deemed the timer as
    // unreachable.
    if (object)
      timer->RunInternal();
  }

  WeakMember<TimerFiredClass> object_;
  TimerFiredFunction function_;
};

NO_SANITIZE_ADDRESS
inline bool TimerBase::IsActive() const {
#if DCHECK_IS_ON()
  DCHECK_EQ(thread_, CurrentThread());
#endif
  return delayed_task_handle_.IsValid();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TIMER_H_
