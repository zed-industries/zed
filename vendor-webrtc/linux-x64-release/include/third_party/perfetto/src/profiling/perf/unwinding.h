/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_PROFILING_PERF_UNWINDING_H_
#define SRC_PROFILING_PERF_UNWINDING_H_

#include <stdint.h>
#include <condition_variable>
#include <map>
#include <optional>
#include <thread>

#include <linux/perf_event.h>
#include <unwindstack/Error.h>

#include "perfetto/base/flat_set.h"
#include "perfetto/base/logging.h"
#include "perfetto/ext/base/thread_checker.h"
#include "perfetto/ext/base/unix_task_runner.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "src/kallsyms/kernel_symbol_map.h"
#include "src/kallsyms/lazy_kernel_symbolizer.h"
#include "src/profiling/common/unwind_support.h"
#include "src/profiling/perf/common_types.h"
#include "src/profiling/perf/unwind_queue.h"

namespace perfetto {
namespace profiling {

constexpr static uint32_t kUnwindQueueCapacity = 1024;

// Unwinds and symbolises callstacks. For userspace this uses the sampled stack
// and register state (see |ParsedSample|). For kernelspace, the kernel itself
// unwinds the stack (recording a list of instruction pointers), so only
// symbolisation using /proc/kallsyms is necessary. Has a single unwinding ring
// queue, shared across all data sources.
//
// Userspace samples cannot be unwound without having /proc/<pid>/{maps,mem}
// file descriptors for that process. This lookup can be asynchronous (e.g. on
// Android), so the unwinder might have to wait before it can process (or
// discard) some of the enqueued samples. To avoid blocking the entire queue,
// the unwinder is allowed to process the entries out of order.
//
// Besides the queue, all interactions between the unwinder and the rest of the
// producer logic are through posted tasks.
//
// As unwinding times are long-tailed (example measurements: median <1ms,
// worst-case ~1000ms), the unwinder runs on a dedicated thread to avoid
// starving the rest of the producer's work (including IPC and consumption of
// records from the kernel ring buffers).
//
// This class should not be instantiated directly, use the |UnwinderHandle|
// below instead.
//
// TODO(rsavitski): while the inputs to the unwinder are batched as a result of
// the reader posting a wakeup only after consuming a batch of kernel samples,
// the Unwinder might be staggering wakeups for the producer thread by posting a
// task every time a sample has been unwound. Evaluate how bad these wakeups are
// in practice, and consider also implementing a batching strategy for the
// unwinder->serialization handoff (which isn't very latency-sensitive).
class Unwinder {
 public:
  friend class UnwinderHandle;

  enum class UnwindMode { kUnwindStack, kFramePointer };

  // Callbacks from the unwinder to the primary producer thread.
  class Delegate {
   public:
    virtual void PostEmitSample(DataSourceInstanceID ds_id,
                                CompletedSample sample) = 0;
    virtual void PostEmitUnwinderSkippedSample(DataSourceInstanceID ds_id,
                                               ParsedSample sample) = 0;
    virtual void PostFinishDataSourceStop(DataSourceInstanceID ds_id) = 0;

    virtual ~Delegate();
  };

  ~Unwinder() { PERFETTO_DCHECK_THREAD(thread_checker_); }

  void PostStartDataSource(DataSourceInstanceID ds_id,
                           bool kernel_frames,
                           UnwindMode unwind_mode);
  void PostAdoptProcDescriptors(DataSourceInstanceID ds_id,
                                pid_t pid,
                                base::ScopedFile maps_fd,
                                base::ScopedFile mem_fd);
  void PostRecordTimedOutProcDescriptors(DataSourceInstanceID ds_id, pid_t pid);
  void PostRecordNoUserspaceProcess(DataSourceInstanceID ds_id, pid_t pid);
  void PostProcessQueue();
  void PostInitiateDataSourceStop(DataSourceInstanceID ds_id);
  void PostPurgeDataSource(DataSourceInstanceID ds_id);

  void PostClearCachedStatePeriodic(DataSourceInstanceID ds_id,
                                    uint32_t period_ms);

  UnwindQueue<UnwindEntry, kUnwindQueueCapacity>& unwind_queue() {
    return unwind_queue_;
  }

  uint64_t GetEnqueuedFootprint() {
    uint64_t freed =
        footprint_tracker_.stack_bytes_freed.load(std::memory_order_acquire);
    uint64_t allocated = footprint_tracker_.stack_bytes_allocated.load(
        std::memory_order_relaxed);

    // overflow not a concern in practice
    PERFETTO_DCHECK(allocated >= freed);
    return allocated - freed;
  }

  void IncrementEnqueuedFootprint(uint64_t increment) {
    footprint_tracker_.stack_bytes_allocated.fetch_add(
        increment, std::memory_order_relaxed);
  }

 private:
  struct ProcessState {
    // kInitial: unwinder waiting for more info on the process (proc-fds, their
    //           lookup expiration, or that there is no need for them).
    // kFdsResolved: proc-fds available, can unwind samples.
    // kFdsTimedOut: proc-fd lookup timed out, will discard samples. Can still
    //               transition to kFdsResolved if the fds are received later.
    // kNoUserspace: only handling kernel callchains (the sample might
    //               still be for a userspace process), can process samples.
    enum class Status { kInitial, kFdsResolved, kFdsTimedOut, kNoUserspace };

    Status status = Status::kInitial;
    // Present iff status == kFdsResolved.
    std::optional<UnwindingMetadata> unwind_state;
    // Used to distinguish first-time unwinding attempts for a process, for
    // logging purposes.
    bool attempted_unwinding = false;
  };

  struct DataSourceState {
    enum class Status { kActive, kShuttingDown };
    explicit DataSourceState(UnwindMode _unwind_mode)
        : unwind_mode(_unwind_mode) {}

    Status status = Status::kActive;
    const UnwindMode unwind_mode;
    std::map<pid_t, ProcessState> process_states;
  };

  // Accounting for how much heap memory is attached to the enqueued samples at
  // a given time. Read by the main thread, mutated by both threads.
  // We track just the heap allocated for the sampled stacks, as it dominates
  // the per-sample heap use.
  struct QueueFootprintTracker {
    std::atomic<uint64_t> stack_bytes_allocated;
    std::atomic<uint64_t> stack_bytes_freed;
  };

  // Must be instantiated via the |UnwinderHandle|.
  Unwinder(Delegate* delegate, base::UnixTaskRunner* task_runner);

  // Marks the data source as valid and active at the unwinding stage.
  // Initializes kernel address symbolization if needed.
  void StartDataSource(DataSourceInstanceID ds_id,
                       bool kernel_frames,
                       UnwindMode unwind_mode);

  void AdoptProcDescriptors(DataSourceInstanceID ds_id,
                            pid_t pid,
                            base::ScopedFile maps_fd,
                            base::ScopedFile mem_fd);
  void UpdateProcessStateStatus(DataSourceInstanceID ds_id,
                                pid_t pid,
                                ProcessState::Status new_status);

  // Primary task. Processes the enqueued samples using
  // |ConsumeAndUnwindReadySamples|, and re-evaluates data source state.
  void ProcessQueue();

  // Processes the enqueued samples for which all unwinding inputs are ready.
  // Returns the set of data source instances which still have samples pending
  // (i.e. waiting on the proc-fds).
  base::FlatSet<DataSourceInstanceID> ConsumeAndUnwindReadySamples();

  CompletedSample UnwindSample(const ParsedSample& sample,
                               UnwindingMetadata* opt_user_state,
                               bool pid_unwound_before,
                               UnwindMode unwind_mode);

  // Returns a list of symbolized kernel frames in the sample (if any).
  std::vector<unwindstack::FrameData> SymbolizeKernelCallchain(
      const ParsedSample& sample);

  // Marks the data source as shutting down at the unwinding stage. It is known
  // that no new samples for this source will be pushed into the queue, but we
  // need to delay the unwinder state teardown until all previously-enqueued
  // samples for this source are processed.
  void InitiateDataSourceStop(DataSourceInstanceID ds_id);

  // Tears down unwinding state for the data source without any outstanding
  // samples, and informs the service that it can continue the shutdown
  // sequence.
  void FinishDataSourceStop(DataSourceInstanceID ds_id);

  // Immediately destroys the data source state, used for abrupt stops.
  void PurgeDataSource(DataSourceInstanceID ds_id);

  void DecrementEnqueuedFootprint(uint64_t decrement) {
    footprint_tracker_.stack_bytes_freed.fetch_add(decrement,
                                                   std::memory_order_relaxed);
  }

  // Clears the parsed maps for all previously-sampled processes, and resets the
  // libunwindstack cache. This has the effect of deallocating the cached Elf
  // objects within libunwindstack, which take up non-trivial amounts of memory.
  //
  // There are two reasons for having this operation:
  // * over a longer trace, it's desirable to drop heavy state for processes
  //   that haven't been sampled recently.
  // * since libunwindstack's cache is not bounded, it'll tend towards having
  //   state for all processes that are targeted by the profiling config.
  //   Clearing the cache periodically helps keep its footprint closer to the
  //   actual working set (NB: which might still be arbitrarily big, depending
  //   on the profiling config).
  //
  // After this function completes, the next unwind for each process will
  // therefore incur a guaranteed maps reparse.
  //
  // Unwinding for concurrent data sources will *not* be directly affected at
  // the time of writing, as the non-cleared parsed maps will keep the cached
  // Elf objects alive through shared_ptrs.
  //
  // Note that this operation is heavy in terms of cpu%, and should therefore
  // be called only for profiling configs that require it.
  //
  // TODO(rsavitski): dropping the full parsed maps is somewhat excessive, could
  // instead clear just the |MapInfo.elf| shared_ptr, but that's considered too
  // brittle as it's an implementation detail of libunwindstack.
  // TODO(rsavitski): improve libunwindstack cache's architecture (it is still
  // worth having at the moment to speed up unwinds across map reparses).
  void ClearCachedStatePeriodic(DataSourceInstanceID ds_id, uint32_t period_ms);

  void ResetAndEnableUnwindstackCache();

  base::UnixTaskRunner* const task_runner_;
  Delegate* const delegate_;
  UnwindQueue<UnwindEntry, kUnwindQueueCapacity> unwind_queue_;
  QueueFootprintTracker footprint_tracker_;
  std::map<DataSourceInstanceID, DataSourceState> data_sources_;
  LazyKernelSymbolizer kernel_symbolizer_;

  PERFETTO_THREAD_CHECKER(thread_checker_)
};

// Owning resource handle for an |Unwinder| with a dedicated task thread.
// Ensures that the |Unwinder| is constructed and destructed on the task thread.
// TODO(rsavitski): update base::ThreadTaskRunner to allow for this pattern of
// owned state, and consolidate.
class UnwinderHandle {
 public:
  explicit UnwinderHandle(Unwinder::Delegate* delegate) {
    std::mutex init_lock;
    std::condition_variable init_cv;

    std::function<void(base::UnixTaskRunner*, Unwinder*)> initializer =
        [this, &init_lock, &init_cv](base::UnixTaskRunner* task_runner,
                                     Unwinder* unwinder) {
          std::lock_guard<std::mutex> lock(init_lock);
          task_runner_ = task_runner;
          unwinder_ = unwinder;
          // Notify while still holding the lock, as init_cv ceases to exist as
          // soon as the main thread observes a non-null task_runner_, and it
          // can wake up spuriously (i.e. before the notify if we had unlocked
          // before notifying).
          init_cv.notify_one();
        };

    thread_ = std::thread(&UnwinderHandle::RunTaskThread, this,
                          std::move(initializer), delegate);

    std::unique_lock<std::mutex> lock(init_lock);
    init_cv.wait(lock, [this] { return !!task_runner_ && !!unwinder_; });
  }

  ~UnwinderHandle() {
    if (task_runner_) {
      PERFETTO_CHECK(!task_runner_->QuitCalled());
      task_runner_->Quit();

      PERFETTO_DCHECK(thread_.joinable());
    }
    if (thread_.joinable())
      thread_.join();
  }

  Unwinder* operator->() { return unwinder_; }

 private:
  void RunTaskThread(
      std::function<void(base::UnixTaskRunner*, Unwinder*)> initializer,
      Unwinder::Delegate* delegate) {
    base::UnixTaskRunner task_runner;
    Unwinder unwinder(delegate, &task_runner);
    task_runner.PostTask(
        std::bind(std::move(initializer), &task_runner, &unwinder));
    task_runner.Run();
  }

  std::thread thread_;
  base::UnixTaskRunner* task_runner_ = nullptr;
  Unwinder* unwinder_ = nullptr;
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_PERF_UNWINDING_H_
