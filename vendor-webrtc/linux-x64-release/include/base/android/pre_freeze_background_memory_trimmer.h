// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_PRE_FREEZE_BACKGROUND_MEMORY_TRIMMER_H_
#define BASE_ANDROID_PRE_FREEZE_BACKGROUND_MEMORY_TRIMMER_H_

#include <deque>

#include "base/compiler_specific.h"
#include "base/debug/proc_maps_linux.h"
#include "base/feature_list.h"
#include "base/functional/callback.h"
#include "base/memory/post_delayed_memory_reduction_task.h"
#include "base/no_destructor.h"
#include "base/profiler/sample_metadata.h"
#include "base/task/delayed_task_handle.h"
#include "base/task/sequenced_task_runner.h"
#include "base/timer/timer.h"

namespace base::android {
class MemoryPurgeManagerAndroid;

BASE_EXPORT BASE_DECLARE_FEATURE(kShouldFreezeSelf);
BASE_EXPORT BASE_DECLARE_FEATURE(kUseRunningCompact);

// Starting from Android U, apps are frozen shortly after being backgrounded
// (with some exceptions). This causes some background tasks for reclaiming
// resources in Chrome to not be run until Chrome is foregrounded again (which
// completely defeats their purpose).
//
// To try to avoid this problem, we use the |PostDelayedBackgroundTask| found
// below. Prior to Android U, this will simply post the task in the background
// with the given delay. From Android U onwards, this will post the task in the
// background with the given delay, but will run it sooner if we are about to
// be frozen.
class BASE_EXPORT PreFreezeBackgroundMemoryTrimmer {
 public:
  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class CompactCancellationReason {
    kAppFreezer,
    kPageResumed,
    kMaxValue = kPageResumed
  };

  static PreFreezeBackgroundMemoryTrimmer& Instance();
  ~PreFreezeBackgroundMemoryTrimmer() = delete;

  // Posts a delayed task. On versions of Android starting from U, may run the
  // task sooner if we are backgrounded. In this case, we run the task on the
  // correct task runner, ignoring the given delay.
  static void PostDelayedBackgroundTask(
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      const base::Location& from_here,
      OnceCallback<void(void)> task,
      base::TimeDelta delay) LOCKS_EXCLUDED(lock()) {
    PostDelayedBackgroundTask(
        task_runner, from_here,
        BindOnce(
            [](OnceClosure task,
               MemoryReductionTaskContext called_from_prefreeze) {
              std::move(task).Run();
            },
            std::move(task)),
        delay);
  }
  static void PostDelayedBackgroundTask(
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      const base::Location& from_here,
      OnceCallback<void(MemoryReductionTaskContext)> task,
      base::TimeDelta delay) LOCKS_EXCLUDED(lock());

  class PreFreezeMetric {
   public:
    virtual ~PreFreezeMetric();

    // |Measure| should return an amount of memory in bytes, or nullopt if
    // unable to record the metric for any reason. It is called underneath a
    // lock, so it should be fast enough to avoid delays (the same lock is held
    // when unregistering metrics).
    virtual std::optional<uint64_t> Measure() const = 0;

    const std::string& name() const LIFETIME_BOUND { return name_; }

   protected:
    friend class PreFreezeBackgroundMemoryTrimmer;
    explicit PreFreezeMetric(const std::string& name);

   private:
    const std::string name_;
  };

  // Registers a new metric to record before and after PreFreeze. Callers are
  // responsible for making sure that the same metric is not registered
  // multiple times.
  //
  // See |PreFreezeMetric| for details on the metric itself.
  //
  // Each time |OnPreFreeze| is run, |metric->Measure()| will be called twice:
  // - Once directly before any tasks are run; and
  // - Once two seconds after the first time it was called.
  //
  // As an example, calling RegisterMemoryMetric(PrivateMemoryFootprintMetric)
  // in the Browser process would cause the following metrics to be recorded 2
  // seconds after the next time |OnPreFreeze| is run:
  // - "Memory.PreFreeze2.Browser.PrivateMemoryFootprint.Before"
  // - "Memory.PreFreeze2.Browser.PrivateMemoryFootprint.After"
  // - "Memory.PreFreeze2.Browser.PrivateMemoryFootprint.Diff"
  //
  // See "Memory.PreFreeze2.{process_type}.{name}.{suffix}" for details on the
  // exact metrics.
  static void RegisterMemoryMetric(const PreFreezeMetric* metric)
      LOCKS_EXCLUDED(lock());

  static void UnregisterMemoryMetric(const PreFreezeMetric* metric)
      LOCKS_EXCLUDED(lock());

  // The callback runs in the thread pool. The caller cannot make any thread
  // safety assumptions for the callback execution (e.g. it could run
  // concurrently with the thread that registered it).
  static void SetOnStartSelfCompactionCallback(base::RepeatingClosure callback)
      LOCKS_EXCLUDED(lock());

  static bool CompactionIsSupported();

  // If we are currently running self compaction, cancel it. If it was running,
  // record a metric with the reason for the cancellation.
  static void MaybeCancelCompaction(
      CompactCancellationReason cancellation_reason);

  static void SetSupportsModernTrimForTesting(bool is_supported);
  static void ClearMetricsForTesting() LOCKS_EXCLUDED(lock());
  size_t GetNumberOfPendingBackgroundTasksForTesting() const
      LOCKS_EXCLUDED(lock());
  size_t GetNumberOfKnownMetricsForTesting() const LOCKS_EXCLUDED(lock());
  size_t GetNumberOfValuesBeforeForTesting() const LOCKS_EXCLUDED(lock());
  bool DidRegisterTasksForTesting() const;

  static void OnPreFreezeForTesting() LOCKS_EXCLUDED(lock()) { OnPreFreeze(); }
  static void ResetCompactionForTesting();

  static std::optional<uint64_t> CompactRegion(
      debug::MappedMemoryRegion region);

  // Called when Chrome is about to be frozen. Runs as many delayed tasks as
  // possible immediately, before we are frozen.
  static void OnPreFreeze() LOCKS_EXCLUDED(lock());

  static void OnSelfFreeze() LOCKS_EXCLUDED(lock());

  static void OnRunningCompact() LOCKS_EXCLUDED(lock());

  static bool SupportsModernTrim();
  static bool ShouldUseModernTrim();
  static bool IsTrimMemoryBackgroundCritical();

 private:
  friend class base::NoDestructor<PreFreezeBackgroundMemoryTrimmer>;
  friend jboolean JNI_MemoryPurgeManager_IsOnPreFreezeMemoryTrimEnabled(
      JNIEnv* env);
  friend class base::android::MemoryPurgeManagerAndroid;
  friend class base::OneShotDelayedBackgroundTimer;
  friend class PreFreezeBackgroundMemoryTrimmerTest;
  friend class PreFreezeSelfCompactionTest;
  friend class PreFreezeSelfCompactionTestWithParam;
  FRIEND_TEST_ALL_PREFIXES(PreFreezeSelfCompactionTestWithParam, Disabled);
  FRIEND_TEST_ALL_PREFIXES(PreFreezeSelfCompactionTestWithParam, Cancel);
  FRIEND_TEST_ALL_PREFIXES(PreFreezeSelfCompactionTest, NotCanceled);
  FRIEND_TEST_ALL_PREFIXES(PreFreezeSelfCompactionTest, OnSelfFreezeCancel);

  // We use our own implementation here, based on |PostCancelableDelayedTask|,
  // rather than relying on something like |base::OneShotTimer|, since
  // |base::OneShotTimer| doesn't support things like immediately running our
  // task from a different sequence, and some |base::OneShotTimer|
  // functionality (e.g. |FireNow|) only works with the default task runner.
  class BackgroundTask final {
   public:
    static std::unique_ptr<BackgroundTask> Create(
        scoped_refptr<base::SequencedTaskRunner> task_runner,
        const base::Location& from_here,
        OnceCallback<void(MemoryReductionTaskContext)> task,
        base::TimeDelta delay);

    explicit BackgroundTask(
        scoped_refptr<base::SequencedTaskRunner> task_runner);
    ~BackgroundTask();

    static void RunNow(std::unique_ptr<BackgroundTask> background_task);

    void Run(MemoryReductionTaskContext from_pre_freeze);

    void CancelTask();

   private:
    friend class PreFreezeBackgroundMemoryTrimmer;
    void Start(const Location& from_here,
               TimeDelta delay,
               OnceCallback<void(MemoryReductionTaskContext)> task);
    void StartInternal(const Location& from_here,
                       TimeDelta delay,
                       OnceClosure task);
    scoped_refptr<base::SequencedTaskRunner> task_runner_;
    base::DelayedTaskHandle task_handle_;

    OnceCallback<void(MemoryReductionTaskContext)> task_;
  };

 private:
  class CompactionMetric final : public RefCountedThreadSafe<CompactionMetric> {
   public:
    CompactionMetric(const std::string& name,
                     base::TimeTicks triggered_at,
                     base::TimeTicks started_at);

    void RecordDelayedMetrics();
    void RecordTimeMetrics(base::TimeTicks compaction_last_finished,
                           base::TimeTicks compaction_last_cancelled);

    void RecordBeforeMetrics();
    void MaybeRecordCompactionMetrics() LOCKS_EXCLUDED(lock());

   private:
    friend class RefCountedThreadSafe<CompactionMetric>;
    ~CompactionMetric();
    void RecordSmapsRollup(std::optional<debug::SmapsRollup>* target)
        LOCKS_EXCLUDED(lock());
    void RecordSmapsRollupWithDelay(std::optional<debug::SmapsRollup>* target,
                                    base::TimeDelta delay);
    std::string GetMetricName(std::string_view name) const;
    std::string GetMetricName(std::string_view name,
                              std::string_view suffix) const;
    void RecordCompactionMetrics(const debug::SmapsRollup& value,
                                 std::string_view suffix);
    void RecordCompactionMetric(size_t value_bytes,
                                std::string_view metric_name,
                                std::string_view suffix);
    void RecordCompactionDiffMetrics(const debug::SmapsRollup& before,
                                     const debug::SmapsRollup& after,
                                     std::string_view suffix);
    void RecordCompactionDiffMetric(size_t before_value_bytes,
                                    size_t after_value_bytes,
                                    std::string_view name,
                                    std::string_view suffix);

    const std::string name_;
    // When the self compaction was first triggered. There is a delay between
    // this time and when we actually begin the compaction.
    const base::TimeTicks compaction_triggered_at_;
    // When the self compaction first started. This should generally be
    // |compaction_triggered_at_ +
    // kShouldFreezeSelfDelayAfterPreFreezeTasks.Get()|, but may be longer if
    // the task was delayed.
    const base::TimeTicks compaction_started_at_;
    // We use std::optional here because:
    // - We record these incrementally.
    // - We may stop recording at some point.
    // - We only want to emit histograms if all values were recorded.
    std::optional<debug::SmapsRollup> smaps_before_;
    std::optional<debug::SmapsRollup> smaps_after_;
    std::optional<debug::SmapsRollup> smaps_after_1s_;
    std::optional<debug::SmapsRollup> smaps_after_10s_;
    std::optional<debug::SmapsRollup> smaps_after_60s_;
  };

  class CompactionState {
   public:
    CompactionState(scoped_refptr<SequencedTaskRunner> task_runner,
                    base::TimeTicks triggered_at,
                    uint64_t max_bytes);
    virtual ~CompactionState();

    virtual bool IsFeatureEnabled() const = 0;
    virtual std::string GetMetricName(std::string_view name) const = 0;
    void MaybeReadProcMaps();
    virtual scoped_refptr<CompactionMetric> MakeCompactionMetric() const = 0;
    virtual base::TimeDelta GetDelayAfterPreFreezeTasks() const = 0;

    scoped_refptr<SequencedTaskRunner> task_runner_;
    std::vector<debug::MappedMemoryRegion> regions_;
    const base::TimeTicks triggered_at_;
    const uint64_t max_bytes_;
  };

  class SelfCompactionState final : public CompactionState {
   public:
    SelfCompactionState(scoped_refptr<SequencedTaskRunner> task_runner,
                        base::TimeTicks triggered_at);
    SelfCompactionState(scoped_refptr<SequencedTaskRunner> task_runner,
                        base::TimeTicks triggered_at,
                        uint64_t max_bytes);
    bool IsFeatureEnabled() const override;
    base::TimeDelta GetDelayAfterPreFreezeTasks() const override;
    std::string GetMetricName(std::string_view name) const override;
    scoped_refptr<CompactionMetric> MakeCompactionMetric() const override;
  };

  class RunningCompactionState final : public CompactionState {
   public:
    RunningCompactionState(scoped_refptr<SequencedTaskRunner> task_runner,
                           base::TimeTicks triggered_at);
    RunningCompactionState(scoped_refptr<SequencedTaskRunner> task_runner,
                           base::TimeTicks triggered_at,
                           uint64_t max_bytes);
    bool IsFeatureEnabled() const override;
    base::TimeDelta GetDelayAfterPreFreezeTasks() const override;
    std::string GetMetricName(std::string_view name) const override;
    scoped_refptr<CompactionMetric> MakeCompactionMetric() const override;
  };

  PreFreezeBackgroundMemoryTrimmer();

  static base::Lock& lock() { return Instance().lock_; }

  // Compacts the memory for the process.
  void CompactSelf(std::unique_ptr<CompactionState> state);

  template <class State>
  void OnTriggerCompact(scoped_refptr<SequencedTaskRunner> task_runner);

  void StartCompaction(std::unique_ptr<CompactionState> state)
      LOCKS_EXCLUDED(lock());
  static base::TimeDelta GetDelayBetweenCompaction();
  void MaybePostCompactionTask(std::unique_ptr<CompactionState> state,
                               scoped_refptr<CompactionMetric> metric)
      LOCKS_EXCLUDED(lock());
  void CompactionTask(std::unique_ptr<CompactionState> state,
                      scoped_refptr<CompactionMetric> metric)
      LOCKS_EXCLUDED(lock());
  void FinishCompaction(std::unique_ptr<CompactionState> state,
                        scoped_refptr<CompactionMetric> metric)
      LOCKS_EXCLUDED(lock());

  static bool ShouldContinueCompaction(
      const PreFreezeBackgroundMemoryTrimmer::CompactionState& state)
      LOCKS_EXCLUDED(lock());
  static bool ShouldContinueCompaction(base::TimeTicks compaction_triggered_at)
      LOCKS_EXCLUDED(lock());

  static std::optional<uint64_t> CompactMemory(
      std::vector<debug::MappedMemoryRegion>* regions,
      const uint64_t max_bytes);

  void RegisterMemoryMetricInternal(const PreFreezeMetric* metric)
      EXCLUSIVE_LOCKS_REQUIRED(lock());

  void UnregisterMemoryMetricInternal(const PreFreezeMetric* metric)
      EXCLUSIVE_LOCKS_REQUIRED(lock());
  static void UnregisterBackgroundTask(BackgroundTask*) LOCKS_EXCLUDED(lock());

  void UnregisterBackgroundTaskInternal(BackgroundTask*) LOCKS_EXCLUDED(lock());

  static void RegisterPrivateMemoryFootprintMetric() LOCKS_EXCLUDED(lock());
  void RegisterPrivateMemoryFootprintMetricInternal() LOCKS_EXCLUDED(lock());

  void PostDelayedBackgroundTaskInternal(
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      const base::Location& from_here,
      OnceCallback<void(MemoryReductionTaskContext)> task,
      base::TimeDelta delay) LOCKS_EXCLUDED(lock());
  void PostDelayedBackgroundTaskModern(
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      const base::Location& from_here,
      OnceCallback<void(MemoryReductionTaskContext)> task,
      base::TimeDelta delay) LOCKS_EXCLUDED(lock());
  BackgroundTask* PostDelayedBackgroundTaskModernHelper(
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      const base::Location& from_here,
      OnceCallback<void(MemoryReductionTaskContext)> task,
      base::TimeDelta delay) EXCLUSIVE_LOCKS_REQUIRED(lock());

  void OnPreFreezeInternal() LOCKS_EXCLUDED(lock());
  void RunPreFreezeTasks() EXCLUSIVE_LOCKS_REQUIRED(lock());

  void MaybeCancelCompactionInternal(
      CompactCancellationReason cancellation_reason) LOCKS_EXCLUDED(lock());

  void PostMetricsTasksIfModern() EXCLUSIVE_LOCKS_REQUIRED(lock());
  void PostMetricsTask() EXCLUSIVE_LOCKS_REQUIRED(lock());
  void RecordMetrics() LOCKS_EXCLUDED(lock());

  mutable base::Lock lock_;
  std::deque<std::unique_ptr<BackgroundTask>> background_tasks_
      GUARDED_BY(lock());
  std::vector<const PreFreezeMetric*> metrics_ GUARDED_BY(lock());
  // When a metrics task is posted (see |RecordMetrics|), the values of each
  // metric before any tasks are run are saved here. The "i"th entry corresponds
  // to the "i"th entry in |metrics_|. When there is no pending metrics task,
  // |values_before_| should be empty.
  std::vector<std::optional<uint64_t>> values_before_ GUARDED_BY(lock());
  // Whether or not we should continue self compaction. There are two reasons
  // why we would cancel:
  // (1) We have resumed, meaning we are likely to touch much of the process
  //     memory soon, and we do not want to waste CPU time with compaction,
  //     since it can block other work that needs to be done.
  // (2) We are going to be frozen by App Freezer, which will do the compaction
  //     work for us. This situation should be relatively rare, because we
  //     attempt to not do self compaction if we know that we are going to
  //     frozen by App Freezer.
  base::TimeTicks compaction_last_cancelled_ GUARDED_BY(lock()) =
      base::TimeTicks::Min();
  // When we last triggered self compaction. Used to record metrics.
  base::TimeTicks compaction_last_triggered_ GUARDED_BY(lock()) =
      base::TimeTicks::Min();
  // When we last finished self compaction (either successfully, or from
  // being cancelled). Used to record metrics.
  base::TimeTicks compaction_last_finished_ GUARDED_BY(lock()) =
      base::TimeTicks::Min();
  std::optional<base::ScopedSampleMetadata> process_compacted_metadata_
      GUARDED_BY(lock());
  base::RepeatingClosure on_self_compact_callback_ GUARDED_BY(lock());
  bool supports_modern_trim_;
};

}  // namespace base::android

#endif  // BASE_ANDROID_PRE_FREEZE_BACKGROUND_MEMORY_TRIMMER_H_
