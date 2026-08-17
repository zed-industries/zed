// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_H_
#define BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_H_

#include <optional>
#include <stack>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/features.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ref.h"
#include "base/memory/scoped_refptr.h"
#include "base/message_loop/message_pump.h"
#include "base/profiler/sample_metadata.h"
#include "base/rand_util.h"
#include "base/run_loop.h"
#include "base/task/common/lazy_now.h"
#include "base/task/sequence_manager/associated_thread_id.h"
#include "base/task/sequence_manager/tasks.h"
#include "base/task/single_thread_task_runner.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "base/trace_event/base_tracing.h"
#include "base/tracing_buildflags.h"
#include "build/build_config.h"

namespace base {

class HistogramBase;
class MessageLoopBase;
class TickClock;
struct PendingTask;

namespace sequence_manager {
namespace internal {

class SequencedTaskSource;

// Implementation of this interface is used by SequenceManager to schedule
// actual work to be run. Hopefully we can stop using MessageLoop and this
// interface will become more concise.
class BASE_EXPORT ThreadController {
 public:
  // Phases the top-RunLevel can go through. While these are more precise than
  // RunLevelTracker::State, unlike it: phases are determined retrospectively
  // as we often only find out the type of work that was just performed at the
  // end of a phase. Or even find out about past phases later in the timeline
  // (i.e. kScheduled is only known after the first kSelectingApplicationTask
  // phase out-of-idle).
  // Public for unit tests.
  // These values are logged to UMA. Entries should not be renumbered and
  // numeric values should never be reused. Please keep in sync
  // with "MessagePumpPhases" in src/tools/metrics/histograms/enums.xml.
  enum Phase {
    kScheduled = 1,
    kPumpOverhead = 2,
    // Any work item, in practice application tasks are mapped to
    // kApplicationTask so this only accounts for native work.
    kWorkItem = 3,
    kNativeWork = kWorkItem,
    kSelectingApplicationTask = 4,
    kApplicationTask = 5,
    kIdleWork = 6,
    kNested = 7,
    kLastPhase = kNested,
    // Reported as a kWorkItem but doesn't clear state relevant to the ongoing
    // work item as it isn't finished (will resume after nesting).
    kWorkItemSuspendedOnNested,
  };

  explicit ThreadController(const TickClock* time_source);
  virtual ~ThreadController();

  // Sets the number of tasks executed in a single invocation of DoWork.
  // Increasing the batch size can reduce the overhead of yielding back to the
  // main message loop.
  virtual void SetWorkBatchSize(int work_batch_size = 1) = 0;

  // Notifies that |pending_task| is about to be enqueued. Needed for tracing
  // purposes. The impl may use this opportunity add metadata to |pending_task|
  // before it is moved into the queue.
  virtual void WillQueueTask(PendingTask* pending_task) = 0;

  // Notify the controller that its associated sequence has immediate work
  // to run. Shortly after this is called, the thread associated with this
  // controller will run a task returned by sequence->TakeTask(). Can be called
  // from any sequence.
  //
  // TODO(altimin): Change this to "the thread associated with this
  // controller will run tasks returned by sequence->TakeTask() until it
  // returns null or sequence->DidRunTask() returns false" once the
  // code is changed to work that way.
  virtual void ScheduleWork() = 0;

  // Notify the controller that SequencedTaskSource will have a delayed work
  // ready to be run at |wake_up|. This call cancels any previously
  // scheduled delayed work. Can only be called from the main sequence.
  // NOTE: GetPendingWakeUp might return a different value as it also takes
  // immediate work into account.
  // TODO(kraynov): Remove |lazy_now| parameter.
  virtual void SetNextDelayedDoWork(LazyNow* lazy_now,
                                    std::optional<WakeUp> wake_up) = 0;

  // Sets the sequenced task source from which to take tasks after
  // a Schedule*Work() call is made.
  // Must be called before the first call to Schedule*Work().
  virtual void SetSequencedTaskSource(SequencedTaskSource*) = 0;

  // Completes delayed initialization of unbound ThreadControllers.
  // BindToCurrentThread(MessageLoopBase*) or BindToCurrentThread(MessagePump*)
  // may only be called once.
  virtual void BindToCurrentThread(
      std::unique_ptr<MessagePump> message_pump) = 0;

  // Explicitly allow or disallow task execution. Implicitly disallowed when
  // entering a nested runloop.
  virtual void SetTaskExecutionAllowedInNativeNestedLoop(bool allowed) = 0;

  // Whether task execution is allowed or not.
  virtual bool IsTaskExecutionAllowed() const = 0;

  // Returns the MessagePump we're bound to if any.
  virtual MessagePump* GetBoundMessagePump() const = 0;

  // Returns true if the current run loop should quit when idle.
  virtual bool ShouldQuitRunLoopWhenIdle() = 0;

#if BUILDFLAG(IS_IOS) || BUILDFLAG(IS_ANDROID)
  // On iOS, the main message loop cannot be Run().  Instead call
  // AttachToMessagePump(), which connects this ThreadController to the
  // UI thread's CFRunLoop and allows PostTask() to work.
  virtual void AttachToMessagePump() = 0;
#endif

#if BUILDFLAG(IS_IOS)
  // Detaches this ThreadController from the message pump, allowing the
  // controller to be shut down cleanly.
  virtual void DetachFromMessagePump() = 0;
#endif

  // Initializes features for this class. See `base::features::Init()`.
  static void InitializeFeatures(
      features::EmitThreadControllerProfilerMetadata emit_profiler_metadata);

  // Enables TimeKeeper metrics. `thread_name` will be used as a suffix.
  // Setting `wall_time_based_metrics_enabled_for_testing` adds wall-time
  // based metrics for this thread. It also also disables subsampling.
  void EnableMessagePumpTimeKeeperMetrics(
      const char* thread_name,
      bool wall_time_based_metrics_enabled_for_testing);

  // Sets the SingleThreadTaskRunner that will be returned by
  // SingleThreadTaskRunner::GetCurrentDefault on the thread controlled by this
  // ThreadController.
  virtual void SetDefaultTaskRunner(scoped_refptr<SingleThreadTaskRunner>) = 0;

  // TODO(altimin): Get rid of the methods below.
  // These methods exist due to current integration of SequenceManager
  // with MessageLoop.

  virtual bool RunsTasksInCurrentSequence() = 0;
  void SetTickClock(const TickClock* clock);
  virtual scoped_refptr<SingleThreadTaskRunner> GetDefaultTaskRunner() = 0;
  virtual void RestoreDefaultTaskRunner() = 0;
  virtual void AddNestingObserver(RunLoop::NestingObserver* observer) = 0;
  virtual void RemoveNestingObserver(RunLoop::NestingObserver* observer) = 0;

  const scoped_refptr<AssociatedThreadId>& GetAssociatedThread() const {
    return associated_thread_;
  }

 protected:
  const scoped_refptr<AssociatedThreadId> associated_thread_;

  // The source of TimeTicks for this ThreadController.
  // Must only be accessed from the `associated_thread_`.
  // TODO(scheduler-dev): This could be made
  // `GUARDED_BY_CONTEXT(associated_thread_->thread_checker)` when
  // switching MainThreadOnly to thread annotations and annotating all
  // thread-affine ThreadController methods. Without that, this lone annotation
  // would result in an inconsistent set of DCHECKs...
  raw_ptr<const TickClock> time_source_;  // Not owned.

  // Whether or not wall-time based metrics are enabled.
  bool wall_time_based_metrics_enabled_for_testing_;

  // Tracks the state of each run-level (main and nested ones) in its associated
  // ThreadController. It does so using two high-level principles:
  //  1) #work-in-work-implies-nested :
  //     If the |state_| is kRunningWorkItem and another work item starts
  //     (OnWorkStarted()), it implies this inner-work-item is running from a
  //  2) #done-work-at-lower-runlevel-implies-done-nested
  //     WorkItems are required to pass in the nesting depth at which they were
  //     created in OnWorkEnded(). Then, if |rundepth| is lower than the current
  //     RunDepth(), we know the top RunLevel was an (already exited) nested
  //     loop and will be popped off |run_levels_|.
  // We need this logic because native nested loops can run from any work item
  // without a RunLoop being involved, see
  // ThreadControllerWithMessagePumpTest.ThreadControllerActive* tests for
  // examples. Using these two heuristics is the simplest way, trying to
  // capture all the ways in which work items can nest is harder than reacting
  // as it happens.
  //
  // Note 1: "native work" is only captured if the MessagePump is
  // instrumented to see them and shares them with ThreadController (via
  // MessagePump::Delegate::OnBeginWorkItem). As such it is still possible to
  // view trace events emanating from native work without "ThreadController
  // active" being active.
  // Note 2: Non-instrumented native work does not break the two high-level
  // principles above because:
  //  A) If a non-instrumented work item enters a nested loop, either:
  //     i) No instrumented work run within the loop so it's invisible.
  //     ii) Instrumented work runs *and* current state is kRunningWorkItem
  //         ((A) is a work item within an instrumented work item):
  //         #work-in-work-implies-nested triggers and the nested loop is
  //         visible.
  //     iii) Instrumented work runs *and* current state is kIdle or
  //          kInBetweenWorkItems ((A) is a work item run by a native loop):
  //          #work-in-work-implies-nested doesn't trigger and this instrumented
  //          work (iii) looks like a non-nested continuation of work at the
  //          current RunLevel.
  //  B) When work item (A) exits its nested loop and completes, respectively:
  //     i) The loop was invisible so no RunLevel was created for it and
  //        #done-work-at-lower-runlevel-implies-done-nested doesn't trigger so
  //        it balances out.
  //     ii) Instrumented work did run, and so RunLevels() increased. However,
  //         since instrumented work (the work which called the nested loop)
  //         keeps track of its own run depth, on its exit, we know to pop the
  //         RunLevel corresponding to the nested work.
  //     iii) Nested instrumented work was visible but didn't appear nested,
  //          state is now back to kInBetweenWorkItems or kIdle as before (A).
  class BASE_EXPORT RunLevelTracker {
   public:
    // States each RunLevel can be in.
    enum State {
      // Waiting for work (pending wakeup).
      kIdle,
      // Between two work items but not idle.
      kInBetweenWorkItems,
      // Running and currently processing a work items (includes selecting the
      // next work item, i.e. either peeking the native work queue or selecting
      // the next application task).
      kRunningWorkItem,
    };

    explicit RunLevelTracker(const ThreadController& outer);
    ~RunLevelTracker();

    void OnRunLoopStarted(State initial_state, LazyNow& lazy_now);
    void OnRunLoopEnded();
    void OnWorkStarted(LazyNow& lazy_now);
    void OnApplicationTaskSelected(TimeTicks queue_time, LazyNow& lazy_now);
    void OnWorkEnded(LazyNow& lazy_now, int run_level_depth);
    void OnIdle(LazyNow& lazy_now);

    size_t num_run_levels() const {
      DCHECK_CALLED_ON_VALID_THREAD(outer_->associated_thread_->thread_checker);
      return run_levels_.size();
    }

    // Emits a perfetto::Flow (wakeup.flow) event associated with this
    // RunLevelTracker.
    void RecordScheduleWork();

    void EnableTimeKeeperMetrics(
        const char* thread_name,
        bool wall_time_based_metrics_enabled_for_testing);

    // Observes changes of state sent as trace-events so they can be tested.
    class TraceObserverForTesting {
     public:
      virtual ~TraceObserverForTesting() = default;

      virtual void OnThreadControllerActiveBegin() = 0;
      virtual void OnThreadControllerActiveEnd() = 0;
      virtual void OnPhaseRecorded(Phase phase) = 0;
    };

    static void SetTraceObserverForTesting(
        TraceObserverForTesting* trace_observer_for_testing);

   private:
    // Keeps track of the time spent in various Phases (ignores idle), reports
    // via UMA to the corresponding phase every time one reaches >= 100ms of
    // cumulative time, resulting in a metric of relative time spent in each
    // non-idle phase. Also emits each phase as a trace event on its own
    // MessagePumpPhases track when the disabled-by-default-base tracing
    // category is enabled.
    class TimeKeeper {
     public:
      explicit TimeKeeper(const RunLevelTracker& outer);

      void EnableRecording(const char* thread_name,
                           bool wall_time_based_metrics_enabled_for_testing);

      // Records the start time of the first phase out-of-idle. The kScheduled
      // phase will be attributed the time before this point once its
      // `queue_time` is known.
      void RecordWakeUp(LazyNow& lazy_now);

      // Accounts the time since OnWorkStarted() towards
      // kSelectingApplicationTask. Accounts `queue_time - last_wakeup_` towards
      // kScheduled (iff `queue_time` is not null nor later than
      // `last_wakeup_`). And flags the current kWorkItem as a kApplicationTask,
      // to be accounted from OnWorkEnded(). Emits a trace event for the
      // kScheduled phase if applicable.
      void OnApplicationTaskSelected(TimeTicks queue_time, LazyNow& lazy_now);

      // If recording is enabled: Records the end of a phase, attributing it the
      // delta between `lazy_now` and `last_phase_end` and emit a trace event
      // for it.
      void RecordEndOfPhase(Phase phase, LazyNow& lazy_now);

      // If recording is enabled: If the `wakeup.flow` category is enabled,
      // record a TerminatingFlow into the current "ThreadController Active"
      // track event.
      void MaybeEmitIncomingWakeupFlow(perfetto::EventContext& ctx);

      const std::string& thread_name() const LIFETIME_BOUND {
        return thread_name_;
      }

      bool wall_time_based_metrics_enabled_for_testing() const {
        return wall_time_based_metrics_enabled_for_testing_;
      }

     private:
      enum class ShouldRecordReqs {
        // Regular should-record requirements.
        kRegular,
        // On wakeup there's an exception to the requirement that `last_wakeup_`
        // be set.
        kOnWakeUp,
        // On end-nested there's an exception to the requirement that there's no
        // ongoing nesting (as the kNested phase ends from ~RunLevel, before
        // run_levels.pop() completes).
        kOnEndNested,
      };
      bool ShouldRecordNow(ShouldRecordReqs reqs = ShouldRecordReqs::kRegular);

      // Common helper to actually record time in a phase and emitt histograms
      // as needed.
      void RecordTimeInPhase(Phase phase,
                             TimeTicks phase_begin,
                             TimeTicks phase_end);

      static const char* PhaseToEventName(Phase phase);

      std::string thread_name_;
      // Whether or not wall-time based metrics are reported.
      bool wall_time_based_metrics_enabled_for_testing_ = false;
      // Cumulative time deltas for each phase, reported and reset when >=100ms.
      std::array<TimeDelta, Phase::kLastPhase + 1> deltas_ = {};
      // Set at the start of the first work item out-of-idle. Consumed from the
      // first application task found in that work cycle
      // (in OnApplicationTaskSelected).
      TimeTicks last_wakeup_;
      // The end of the last phase (used as the beginning of the next one).
      TimeTicks last_phase_end_;
      // The end of the last kIdleWork phase. Used as a minimum for the next
      // kScheduled phase's begin (as it's possible that the next wake-up is
      // scheduled during DoIdleWork and we don't want overlapping phases).
      TimeTicks last_sleep_;
      // Assumes each kWorkItem is native unless OnApplicationTaskSelected() is
      // invoked in a given [OnWorkStarted, OnWorkEnded].
      bool current_work_item_is_native_ = true;

      // non-null when recording is enabled.
      raw_ptr<HistogramBase> histogram_ = nullptr;
#if BUILDFLAG(ENABLE_BASE_TRACING)
      std::optional<perfetto::Track> perfetto_track_;

      // True if tracing was enabled during the last pass of RecordTimeInPhase.
      bool was_tracing_enabled_ = false;
#endif
      const raw_ref<const RunLevelTracker> outer_;
    } time_keeper_{*this};

    class RunLevel {
     public:
      RunLevel(State initial_state,
               bool is_nested,
               TimeKeeper& time_keeper,
               LazyNow& lazy_now);
      ~RunLevel();

      // Move-constructible for STL compat. Flags `other.was_moved_` so it noops
      // on destruction after handing off its responsibility. Move-assignment
      // is not necessary nor possible as not all members are assignable.
      RunLevel(RunLevel&& other);
      RunLevel& operator=(RunLevel&&) = delete;

      void UpdateState(State new_state, LazyNow& lazy_now);

      State state() const { return state_; }

      void set_exit_lazy_now(LazyNow* exit_lazy_now) {
        DCHECK(exit_lazy_now);
        DCHECK(!exit_lazy_now_);
        exit_lazy_now_ = exit_lazy_now;
      }

     private:
      void LogPercentageMetric(const char* name, int value);
      void LogPercentageMetric(const char* name,
                               int value,
                               base::TimeDelta interval_duration);
      void LogIntervalMetric(const char* name,
                             base::TimeDelta value,
                             base::TimeDelta interval_duration);
      void LogOnActiveMetrics(LazyNow& lazy_now);
      void LogOnIdleMetrics(LazyNow& lazy_now);

      base::TimeTicks last_active_end_;
      base::TimeTicks last_active_start_;
      base::ThreadTicks last_active_threadtick_start_;
      base::TimeDelta accumulated_idle_time_;
      base::TimeDelta accumulated_active_time_;
      base::TimeDelta accumulated_active_on_cpu_time_;
      base::TimeDelta accumulated_active_off_cpu_time_;
      MetricsSubSampler metrics_sub_sampler_;

      State state_ = kIdle;
      bool is_nested_;

      bool ShouldRecordSampleMetadata();

      // Get full suffix for histogram logging purposes. |duration| should equal
      // TimeDelta() when not applicable.
      std::string GetSuffixForHistogram(TimeDelta duration);

      std::string GetSuffixForCatchAllHistogram();
      std::string_view GetThreadName();

      const raw_ref<TimeKeeper> time_keeper_;
      // Must be set shortly before ~RunLevel.
      raw_ptr<LazyNow> exit_lazy_now_ = nullptr;

      SampleMetadata thread_controller_sample_metadata_;
      size_t thread_controller_active_id_ = 0;

      // Toggles to true when used as RunLevel&& input to construct another
      // RunLevel. This RunLevel's destructor will then no-op.
      class TruePostMove {
       public:
        TruePostMove() = default;
        TruePostMove(TruePostMove&& other) { other.was_moved_ = true; }
        // Not necessary for now.
        TruePostMove& operator=(TruePostMove&&) = delete;

        explicit operator bool() { return was_moved_; }

       private:
        bool was_moved_ = false;
      };
      TruePostMove was_moved_;
    };

    [[maybe_unused]] const raw_ref<const ThreadController> outer_;

    std::stack<RunLevel, std::vector<RunLevel>> run_levels_
        GUARDED_BY_CONTEXT(outer_->associated_thread_->thread_checker);

    static TraceObserverForTesting* trace_observer_for_testing_;
  } run_level_tracker_{*this};
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_THREAD_CONTROLLER_H_
