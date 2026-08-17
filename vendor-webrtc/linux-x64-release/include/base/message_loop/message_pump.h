// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_MESSAGE_PUMP_H_
#define BASE_MESSAGE_LOOP_MESSAGE_PUMP_H_

#include <memory>
#include <utility>

#include "base/base_export.h"
#include "base/check.h"
#include "base/check_op.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/message_loop/message_pump_type.h"
#include "base/sequence_checker.h"
#include "base/time/time.h"
#include "build/build_config.h"

namespace base {

class IOWatcher;
class TimeTicks;

class BASE_EXPORT MessagePump {
 public:
  using MessagePumpFactory = std::unique_ptr<MessagePump>();
  // Uses the given base::MessagePumpFactory to override the default MessagePump
  // implementation for 'MessagePumpType::UI'. May only be called once.
  static void OverrideMessagePumpForUIFactory(MessagePumpFactory* factory);

  // Returns true if the MessagePumpForUI has been overidden.
  static bool IsMessagePumpForUIFactoryOveridden();

  static void InitializeFeatures();

  // Manage the state of |kAlignWakeUps| and the leeway of the process.
  static void OverrideAlignWakeUpsState(bool enabled, TimeDelta leeway);
  static void ResetAlignWakeUpsState();
  static bool GetAlignWakeUpsEnabled();
  static TimeDelta GetLeewayIgnoringThreadOverride();
  static TimeDelta GetLeewayForCurrentThread();

  // Creates the default MessagePump based on |type|. Caller owns return value.
  static std::unique_ptr<MessagePump> Create(MessagePumpType type);

  // Please see the comments above the Run method for an illustration of how
  // these delegate methods are used.
  class BASE_EXPORT Delegate {
   public:
    virtual ~Delegate() = default;

    struct NextWorkInfo {
      // Helper to extract a TimeDelta for pumps that need a
      // timeout-till-next-task.
      TimeDelta remaining_delay() const {
        DCHECK(!delayed_run_time.is_null() && !delayed_run_time.is_max());
        DCHECK_GE(TimeTicks::Now(), recent_now);
        return delayed_run_time - recent_now;
      }

      // Helper to verify if the next task is ready right away.
      bool is_immediate() const { return delayed_run_time.is_null(); }

      // The next PendingTask's |delayed_run_time|. is_null() if there's extra
      // work to run immediately. is_max() if there are no more immediate nor
      // delayed tasks.
      TimeTicks delayed_run_time;

      // |leeway| determines the preferred time range for scheduling
      // work. A larger leeway provides more freedom to schedule work at
      // an optimal time for power consumption. This field is ignored
      // for immediate work.
      TimeDelta leeway;

      // A recent view of TimeTicks::Now(). Only valid if |delayed_run_time|
      // isn't null nor max. MessagePump impls should use remaining_delay()
      // instead of resampling Now() if they wish to sleep for a TimeDelta.
      TimeTicks recent_now;
    };

    // Executes an immediate task or a ripe delayed task. Returns information
    // about when DoWork() should be called again. If the returned NextWorkInfo
    // is_immediate(), DoWork() must be invoked again shortly. Else, DoWork()
    // must be invoked at |NextWorkInfo::delayed_run_time| or when
    // ScheduleWork() is invoked, whichever comes first. Redundant/spurious
    // invocations of DoWork() outside of those requirements are tolerated.
    // DoIdleWork() will not be called so long as this returns a NextWorkInfo
    // which is_immediate().
    virtual NextWorkInfo DoWork() = 0;

    // Called from within Run just before the message pump goes to sleep.
    virtual void DoIdleWork() = 0;

    class ScopedDoWorkItem {
     public:
      ScopedDoWorkItem() : outer_(nullptr), work_item_depth_(0) {}

      ~ScopedDoWorkItem() {
        if (outer_) {
          outer_->OnEndWorkItem(work_item_depth_);
        }
      }

      ScopedDoWorkItem(ScopedDoWorkItem&& rhs)
          : outer_(std::exchange(rhs.outer_, nullptr)),
            work_item_depth_(rhs.work_item_depth_) {}
      ScopedDoWorkItem& operator=(ScopedDoWorkItem&& rhs) {
        // We should only ever go from an empty ScopedDoWorkItem to an
        // initialized one, or from an initialized one to an empty one.
        CHECK_NE(IsNull(), rhs.IsNull());
        // Since we're overwriting this ScopedDoWorkItem, we need to record its
        // destruction.
        if (outer_) {
          outer_->OnEndWorkItem(work_item_depth_);
        }

        work_item_depth_ = rhs.work_item_depth_;
        outer_ = std::exchange(rhs.outer_, nullptr);
        return *this;
      }

      bool IsNull() { return !outer_; }

     private:
      friend Delegate;

      explicit ScopedDoWorkItem(Delegate* outer) : outer_(outer) {
        outer_->OnBeginWorkItem();
        work_item_depth_ = outer_->RunDepth();
      }

      // `outer_` is not a raw_ptr<...> for performance reasons (based on
      // analysis of sampling profiler data and tab_search:top100:2020).
      RAW_PTR_EXCLUSION Delegate* outer_;

      // Records the run level at which this DoWorkItem was created to allow
      // detection of exits of nested loops.
      int work_item_depth_;
    };

    // Called before a unit of work is executed. This allows reports
    // about individual units of work to be produced. The unit of work ends when
    // the returned ScopedDoWorkItem goes out of scope.
    // TODO(crbug.com/40580088): Place calls for all platforms. Without this,
    // some state like the top-level "ThreadController active" trace event will
    // not be correct when work is performed.
    [[nodiscard]] ScopedDoWorkItem BeginWorkItem() {
      return ScopedDoWorkItem(this);
    }

    // Called before the message pump starts waiting for work. This indicates
    // that the message pump is idle (out of application work and ideally out of
    // native work -- if it can tell).
    virtual void BeforeWait() = 0;

    // May be called when starting to process native work and it is guaranteed
    // that DoWork() will be called again before sleeping. Allows the delegate
    // to skip unnecessary ScheduleWork() calls.
    virtual void BeginNativeWorkBeforeDoWork() = 0;

    // Returns the nesting level at which the Delegate is currently running.
    virtual int RunDepth() = 0;

   private:
    // Called upon entering/exiting a ScopedDoWorkItem.
    virtual void OnBeginWorkItem() = 0;
    virtual void OnEndWorkItem(int work_item_depth) = 0;
  };

  MessagePump();
  virtual ~MessagePump();

  // The Run method is called to enter the message pump's run loop.
  //
  // Within the method, the message pump is responsible for processing native
  // messages as well as for giving cycles to the delegate periodically. The
  // message pump should take care to mix delegate callbacks with native message
  // processing so neither type of event starves the other of cycles. Each call
  // to a delegate function is considered the beginning of a new "unit of work".
  //
  // The anatomy of a typical run loop:
  //
  //   for (;;) {
  //     bool did_native_work = false;
  //     {
  //       auto scoped_do_work_item = state_->delegate->BeginWorkItem();
  //       did_native_work = DoNativeWork();
  //     }
  //     if (should_quit_)
  //       break;
  //
  //     Delegate::NextWorkInfo next_work_info = delegate->DoWork();
  //     if (should_quit_)
  //       break;
  //
  //     if (did_native_work || next_work_info.is_immediate())
  //       continue;
  //
  //     delegate_->DoIdleWork();
  //     if (should_quit_)
  //       break;
  //
  //     if (did_idle_work)
  //       continue;
  //
  //     WaitForWork();
  //   }
  //

  // Here, DoNativeWork is some private method of the message pump that is
  // responsible for dispatching the next UI message or notifying the next IO
  // completion (for example).  WaitForWork is a private method that simply
  // blocks until there is more work of any type to do.
  //
  // Notice that the run loop cycles between calling DoNativeWork and DoWork
  // methods. This helps ensure that none of these work queues starve the
  // others. This is important for message pumps that are used to drive
  // animations, for example.
  //
  // Notice also that after each callout to foreign code, the run loop checks to
  // see if it should quit.  The Quit method is responsible for setting this
  // flag.  No further work is done once the quit flag is set.
  //
  // NOTE 1: Run may be called reentrantly from any of the callouts to foreign
  // code (internal work, DoWork, DoIdleWork). As a result, DoWork and
  // DoIdleWork must be reentrant.
  //
  // NOTE 2: Run implementations must arrange for DoWork to be invoked as
  // expected if a callout to foreign code enters a message pump outside their
  // control. For example, the MessageBox API on Windows pumps UI messages. If
  // the MessageBox API is called (indirectly) from within Run, it is expected
  // that DoWork will be invoked from within that call in response to
  // ScheduleWork or as requested by the last NextWorkInfo returned by DoWork.
  // The MessagePump::Delegate may then elect to do nested work or not depending
  // on its policy in that context. Regardless of that decision (and return
  // value of the nested DoWork() call), DoWork() will be invoked again when the
  // nested loop unwinds.
  virtual void Run(Delegate* delegate) = 0;

  // Quit immediately from the most recently entered run loop.  This method may
  // only be used on the thread that called Run.
  virtual void Quit() = 0;

  // Schedule a DoWork callback to happen reasonably soon.  Does nothing if a
  // DoWork callback is already scheduled. Once this call is made, DoWork is
  // guaranteed to be called repeatedly at least until it returns a
  // non-immediate NextWorkInfo. This call can be expensive and callers should
  // attempt not to invoke it again before a non-immediate NextWorkInfo was
  // returned from DoWork(). Thread-safe (and callers should avoid holding a
  // Lock at all cost while making this call as some platforms' priority
  // boosting features have been observed to cause the caller to get descheduled
  // : https://crbug.com/890978).
  virtual void ScheduleWork() = 0;

  // Schedule a DoWork callback to happen at the specified time, cancelling any
  // pending callback scheduled by this method. This method may only be used on
  // the thread that called Run.
  //
  // It isn't necessary to call this during normal execution, as the pump wakes
  // up as requested by the return value of DoWork().
  // TODO(crbug.com/40594269): Determine if this must be called to ensure that
  // delayed tasks run when a message pump outside the control of Run is
  // entered.
  virtual void ScheduleDelayedWork(
      const Delegate::NextWorkInfo& next_work_info) = 0;

  // Returns an adjusted |run_time| based on alignment policies of the pump.
  virtual TimeTicks AdjustDelayedRunTime(TimeTicks earliest_time,
                                         TimeTicks run_time,
                                         TimeTicks latest_time);

  // Requests the pump to handle either the likely imminent creation (`true`) or
  // destruction (`false`) of a native nested loop in which application tasks
  // are desired to be run. The pump should override and return `true` if it
  // supports this call and has scheduled work in response. The default
  // implementation returns `false` and does nothing.
  virtual bool HandleNestedNativeLoopWithApplicationTasks(
      bool application_tasks_desired);

  // If the MessagePump implementation supports async IO event handling, this
  // returns a valid IOWatcher implementation to use. Otherwise returns null.
  virtual IOWatcher* GetIOWatcher();

  // May cause the message pump to busy loop for the specified duration. May not
  // work for all message pump types, and is only an upper bound of busy looping
  // time. This may be used to avoid sleeping when a short wait is
  // expected. This is exposed here rather than being internal to allow setting
  // it depending on the context (e.g. on some threads only, or only when the
  // thread is expected to benefit from it).
  void SetBusyLoop(base::TimeDelta max_busy_loop_time) {
    max_busy_loop_time_ = max_busy_loop_time;
  }

 protected:
  base::TimeDelta max_busy_loop_time_;

 private:
  // TODO(crbug.com/379190028): Individual MessagePump subclasses should own and
  // initialize their own IOWatcher.
  std::unique_ptr<IOWatcher> io_watcher_;
};

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_MESSAGE_PUMP_H_
