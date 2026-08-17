// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_THREAD_GROUP_PROFILER_H_
#define BASE_PROFILER_THREAD_GROUP_PROFILER_H_

#include <memory>

#include "base/cancelable_callback.h"
#include "base/containers/flat_map.h"
#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/profiler/periodic_sampling_scheduler.h"
#include "base/profiler/sampling_profiler_thread_token.h"
#include "base/profiler/stack_sampling_profiler.h"
#include "base/task/sequenced_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"

namespace base {
namespace internal {
class WorkerThread;
}  // namespace internal

class ThreadGroupProfilerClient;

// ThreadGroupProfiler manages sampling of active worker threads and
// schedules periodic sampling.
// This class will be accessed on
//   - Main thread: Construction, shutdown and destruction.
//   - Worker thread: Invokes the OnWorkerThread* functions to inform the class
//   of their lifetime events.
//   - Sequenced task runner: Internal operations of this class are scheduled on
//   the task runner.
// Once created, ThreadGroupProfiler will periodically profile active worker
// threads by creating a StackSamplingProfiler for each thread. At the beginning
// of a session, all active worker threads are sampled. During the session, if a
// worker thread becomes active (via OnWorkerThreadActive) it will be sampled
// for the remainder of this session. Once the sampling starts for a thread it
// will continue until either the thread is exiting (via OnWorkerThreadExit) or
// the profile is completed. When a profile completes the associated
// StackSamplingProfiler is destroyed. Worker threads being sampled will be
// blocked on exit until the profiling is stopped.
//
// When shutting down, the class requires that the provided SequencedTaskRunner
// is shutdown prior to invoking Shutdown().
class BASE_EXPORT ThreadGroupProfiler {
 public:
  // Interface for profiling stack samples from a specific thread.
  // This provides an abstraction over StackSamplingProfiler to enable testing
  // of ThreadGroupProfiler without depending on actual profiler implementation.
  class Profiler {
   public:
    virtual ~Profiler() = default;
    virtual void Start() = 0;

   protected:
    Profiler() = default;
  };

  // Sets the instance of ThreadProfilerClient to provide embedder-specific
  // implementation logic. This instance must be set early, before
  // CreateThreadGroupProfiler() and IsProfilingEnabled() are called.
  static void SetClient(std::unique_ptr<ThreadGroupProfilerClient> client);

  // Must be called after SetClient().
  static bool IsProfilingEnabled();

  using ProfilerFactory = RepeatingCallback<std::unique_ptr<Profiler>(
      SamplingProfilerThreadToken thread_token,
      const StackSamplingProfiler::SamplingParams& params,
      std::unique_ptr<ProfileBuilder> profile_builder,
      StackSamplingProfiler::UnwindersFactory unwinder_factory)>;

  using GetTimeToNextCollectionCallback = RepeatingCallback<TimeDelta()>;

  // ThreadGroupProfiler constructor. |task_runner| will be used to schedule the
  // profile collection. |thread_group_type| will used to tag the metadata for
  // all samples collected in this profiler. |profiler_factory| is a repeating
  // callback that will be used to make Profiler, intended to be used for
  // dependency injection for testing. |time_to_next_collection_callback| is a
  // repeating callback that will be used to get the next collection time,
  // intended to be used for dependency injection for testing.
  explicit ThreadGroupProfiler(
      scoped_refptr<SequencedTaskRunner> task_runner,
      int64_t thread_group_type,
      std::unique_ptr<PeriodicSamplingScheduler> periodic_sampling_scheduler =
          nullptr,
      ProfilerFactory profiler_factory = GetDefaultProfilerFactory());
  ThreadGroupProfiler(const ThreadGroupProfiler&) = delete;
  ThreadGroupProfiler& operator=(const ThreadGroupProfiler&) = delete;

  ~ThreadGroupProfiler();

  // Shuts down ThreadGroupProfiler instance and stops all current profiling.
  // This should only be called after task runner is stopped as it expects
  // exclusive access on this instance. No more sampling will happen and worker
  // threads are freed to exit after shutdown finishes.
  void Shutdown();

  // Register new worker thread on starting. Must be called on worker
  // thread.
  void OnWorkerThreadStarted(internal::WorkerThread* worker_thread);

  // Starts profilng on worker that has become active during a sampling
  // session. Must be called on worker thread.
  void OnWorkerThreadActive(internal::WorkerThread* worker_thread);

  // Must be called on worker thread when it becomes idle, i.e. no more work is
  // scheduled to run on this thread.
  void OnWorkerThreadIdle(internal::WorkerThread* worker_thread);

  // Clean up on worker thread exiting. Must be called on worker thread.
  void OnWorkerThreadExiting(internal::WorkerThread* worker_thread);

 private:
  struct WorkerThreadContext {
    SamplingProfilerThreadToken token;
    bool is_idle;
  };
  class ProfilerImpl;

  // Represents an active sample collection phase and is responsible for
  // creating profilers for active threads both at the beginning as well as
  // during the sampling duration.
  class ActiveCollection {
   public:
    explicit ActiveCollection(
        const flat_map<internal::WorkerThread*, WorkerThreadContext>&
            worker_thread_context_set,
        int64_t thread_group_type,
        const TimeDelta& sampling_duration,
        SequencedTaskRunner* task_runner,
        ProfilerFactory stack_sampling_profiler_factory,
        OnceClosure collection_completed_callback);
    ~ActiveCollection();
    ActiveCollection(const ActiveCollection&) = delete;
    ActiveCollection& operator=(const ActiveCollection&) = delete;

    // Maybe create a new profiler for worker_thread depending on how close
    // the collection is to being complete.
    void MaybeAddWorkerThread(internal::WorkerThread* worker_thread,
                              const SamplingProfilerThreadToken& token);

    // Destroy the profiler for worker_thread if it exists.
    void RemoveWorkerThread(internal::WorkerThread* worker_thread);

   private:
    // Helper function for creating the StackSamplingProfiler.
    std::unique_ptr<Profiler> CreateSamplingProfilerForThread(
        internal::WorkerThread* worker_thread,
        const SamplingProfilerThreadToken& token,
        const StackSamplingProfiler::SamplingParams& sampling_params);

    // Remove completed profiler from collection. If this is the last profiler,
    // invokes the collection completed callback.
    void OnProfilerCollectionCompleted(internal::WorkerThread* worker_thread);
    // Invokes collection completed callback to end an empty collection.
    void OnEmptyCollectionCompleted();

    const int64_t thread_group_type_;

    // A map that stores the active `StackSamplingProfiler` instances
    // for each worker thread.
    flat_map<internal::WorkerThread*, std::unique_ptr<Profiler>> profilers_;

    scoped_refptr<SequencedTaskRunner> task_runner_;

    ProfilerFactory stack_sampling_profiler_factory_;

    // Callback to notify on collection complete. After this callback is run
    // there's no guarantee that the instance is still alive.
    OnceClosure collection_complete_callback_;

    const TimeDelta sampling_duration_;

    // Tracks the end time (an estimate calculated at start of sampling by
    // adding the sampling duration) of the current sampling session.
    const TimeTicks collection_end_time_;

    // Used to trigger collection completed when the collection is empty at the
    // end of a session. This callback is only alive when there are no profilers
    // in this collection and is cancelled immediately when there are active
    // profilers.
    CancelableOnceClosure empty_collection_closure_;
  };

  // Retrieve the ThreadProfilerClient instance provided via SetClient().
  static ThreadGroupProfilerClient* GetClient();

  static ProfilerFactory GetDefaultProfilerFactory();

  static GetTimeToNextCollectionCallback
  GetDefaultTimeToNextCollectionCallback();

  static TimeDelta GetSamplingDuration();

  // All the private functions below are executed on the task runner to
  // ensure proper synchronization. This is enforced through
  // DCHECK_CALLED_ON_VALID_SEQUENCE(task_runner_sequence_checker_) for
  // functions that are called as PostTask task and
  // VALID_CONTEXT_REQUIRED(task_runner_sequence_checker_) for functions that
  // are called directly.

  void StartTask();

  void OnWorkerThreadStartedTask(internal::WorkerThread* worker_thread,
                                 SamplingProfilerThreadToken token);
  void OnWorkerThreadActiveTask(internal::WorkerThread* worker_thread);
  void OnWorkerThreadIdleTask(internal::WorkerThread* worker_thread);
  void OnWorkerThreadExitingTask(internal::WorkerThread* worker_thread,
                                 WaitableEvent* profiling_has_stopped);

  // Starts the thread group profiler collection. This will create stack
  // sampling profilers for all active worker threads in the thread group,
  // monitor new active worker threads (these include both new worker threads
  // that are spawned and idle worker threads becoming active) during sampling
  // duration and schedules the next sampling session.
  void CollectProfilesTask();

  void EndActiveCollectionTask();

  // A map that stores the worker threads, their corresponding profiler
  // token and their idle states.
  flat_map<internal::WorkerThread*, WorkerThreadContext>
      worker_thread_context_set_
          GUARDED_BY_CONTEXT(task_runner_sequence_checker_);

  // This has no value if not in an active collection phase.
  std::optional<ActiveCollection> active_collection_
      GUARDED_BY_CONTEXT(task_runner_sequence_checker_);

  // Value to use as metadata for specifying which type of thread group is being
  // profiled.
  const int64_t thread_group_type_;

  // Used to block worker threads from exiting during ThreadGroupProfiler
  // shutdown.
  WaitableEvent thread_group_profiler_shutdown_{
      base::WaitableEvent::ResetPolicy::MANUAL,
      base::WaitableEvent::InitialState::NOT_SIGNALED};

  std::unique_ptr<PeriodicSamplingScheduler> periodic_sampling_scheduler_
      GUARDED_BY_CONTEXT(task_runner_sequence_checker_);

  scoped_refptr<SequencedTaskRunner> task_runner_;

  ProfilerFactory stack_sampling_profiler_factory_;

  SEQUENCE_CHECKER(task_runner_sequence_checker_);
  SEQUENCE_CHECKER(construction_sequence_checker_);
};

}  // namespace base

#endif  // BASE_PROFILER_THREAD_GROUP_PROFILER_H_
