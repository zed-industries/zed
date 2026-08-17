/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_THREAD_H_

#include <memory>
#include <optional>

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "base/synchronization/waitable_event.h"
#include "base/task/single_thread_task_runner.h"
#include "base/thread_annotations.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "services/network/public/mojom/fetch_api.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-blink-forward.h"
#include "third_party/blink/public/platform/platform.h"
#include "third_party/blink/public/platform/web_policy_container.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/inspector/inspector_issue_storage.h"
#include "third_party/blink/renderer/core/workers/parent_execution_context_task_runners.h"
#include "third_party/blink/renderer/core/workers/worker_backing_thread_startup_data.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cross_thread_task.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_type.h"
#include "third_party/blink/renderer/platform/scheduler/public/worker_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_functional.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "v8/include/v8-inspector.h"
#include "v8/include/v8.h"

namespace blink {

class ConsoleMessageStorage;
class InspectorTaskRunner;
class InspectorIssueStorage;
class KURL;
class WorkerBackingThread;
class WorkerInspectorController;
class WorkerOrWorkletGlobalScope;
class WorkerReportingProxy;
class WorkerResourceTimingNotifier;
struct CrossThreadFetchClientSettingsObjectData;
struct GlobalScopeCreationParams;
struct WorkerDevToolsParams;
struct WorkerMainScriptLoadParameters;

// WorkerThread is a kind of WorkerBackingThread client. Each worker mechanism
// can access the lower thread infrastructure via an implementation of this
// abstract class. Multiple WorkerThreads may share one WorkerBackingThread for
// worklets.
//
// WorkerThread start and termination must be initiated on the parent thread and
// an actual task is executed on the worker thread.
//
// When termination starts, (debugger) tasks on WorkerThread are handled as
// follows:
//  - A running task may finish unless a forcible termination task interrupts.
//    If the running task is for debugger, it's guaranteed to finish without
//    any interruptions.
//  - Queued tasks never run.
class CORE_EXPORT WorkerThread : public Thread::TaskObserver {
 public:
  // Represents how this thread is terminated. Used for UMA. Append only.
  enum class ExitCode {
    kNotTerminated,
    kGracefullyTerminated,
    kSyncForciblyTerminated,
    kAsyncForciblyTerminated,
    kMaxValue = kAsyncForciblyTerminated,
  };

  ~WorkerThread() override;

  // Starts the underlying thread and creates the global scope. Called on the
  // parent thread.
  // Startup data for WorkerBackingThread is std::nullopt if |this| doesn't own
  // the underlying WorkerBackingThread.
  // TODO(nhiroki): We could separate WorkerBackingThread initialization from
  // GlobalScope initialization sequence, that is, InitializeOnWorkerThread().
  // After that, we could remove this startup data for WorkerBackingThread.
  // (https://crbug.com/710364)
  void Start(std::unique_ptr<GlobalScopeCreationParams>,
             const std::optional<WorkerBackingThreadStartupData>&,
             std::unique_ptr<WorkerDevToolsParams>);

  // Posts a task to evaluate a top-level classic script on the worker thread.
  // Called on the parent thread after Start().
  void EvaluateClassicScript(const KURL& script_url,
                             const String& source_code,
                             std::unique_ptr<Vector<uint8_t>> cached_meta_data,
                             const v8_inspector::V8StackTraceId& stack_id);

  // Posts a task to fetch and run a top-level classic script on the worker
  // thread. Called on the parent thread after Start().
  void FetchAndRunClassicScript(
      const KURL& script_url,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      std::unique_ptr<WebPolicyContainer> policy_container,
      std::unique_ptr<CrossThreadFetchClientSettingsObjectData>
          outside_settings_object_data,
      WorkerResourceTimingNotifier* outside_resource_timing_notifier,
      const v8_inspector::V8StackTraceId& stack_id);

  // Posts a task to fetch and run a top-level module script on the worker
  // thread. Called on the parent thread after Start().
  void FetchAndRunModuleScript(
      const KURL& script_url,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      std::unique_ptr<WebPolicyContainer> policy_container,
      std::unique_ptr<CrossThreadFetchClientSettingsObjectData>
          outside_settings_object_data,
      WorkerResourceTimingNotifier* outside_resource_timing_notifier,
      network::mojom::CredentialsMode);

  // Posts a task to the worker thread to close the global scope and terminate
  // the underlying thread. This task may be blocked by JavaScript execution on
  // the worker thread, so this function also forcibly terminates JavaScript
  // execution after a certain grace period.
  void Terminate() LOCKS_EXCLUDED(lock_);

  // Terminates the worker thread. Subclasses of WorkerThread can override this
  // to do cleanup. The default behavior is to call Terminate() and
  // synchronously call EnsureScriptExecutionTerminates() to ensure the thread
  // is quickly terminated. Called on the parent thread.
  virtual void TerminateForTesting();

  // Thread::TaskObserver.
  void WillProcessTask(const base::PendingTask&, bool) override;
  void DidProcessTask(const base::PendingTask&) override;

  virtual WorkerBackingThread& GetWorkerBackingThread() = 0;
  ConsoleMessageStorage* GetConsoleMessageStorage() const {
    return console_message_storage_.Get();
  }
  InspectorIssueStorage* GetInspectorIssueStorage() {
    return &inspector_issue_storage_;
  }
  v8::Isolate* GetIsolate();

  bool IsCurrentThread();

  WorkerReportingProxy& GetWorkerReportingProxy() const {
    return worker_reporting_proxy_;
  }

  // Only callable on the parent thread.
  void DebuggerTaskStarted();
  void DebuggerTaskFinished();

  // Callable on both the parent thread and the worker thread.
  const base::UnguessableToken& GetDevToolsWorkerToken() const {
    return devtools_worker_token_;
  }

  // Can be called only on the worker thread, WorkerOrWorkletGlobalScope
  // and WorkerInspectorController are not thread safe.
  WorkerOrWorkletGlobalScope* GlobalScope();
  WorkerInspectorController* GetWorkerInspectorController();

  // Number of active worker threads.
  static unsigned WorkerThreadCount();

  // Runs |function| with |parameters| on each worker thread, and
  // adds the current WorkerThread* as the first parameter |function|.
  // This only calls |function| for threads for which Start() was already
  // called.
  // Returns the number of workers that are scheduled to run the function.
  template <typename FunctionType, typename... Parameters>
  static unsigned CallOnAllWorkerThreads(FunctionType function,
                                         TaskType task_type,
                                         Parameters&&... parameters) {
    base::AutoLock locker(ThreadSetLock());
    unsigned called_worker_count = 0;
    for (WorkerThread* thread : WorkerThreads()) {
      PostCrossThreadTask(
          *thread->GetTaskRunner(task_type), FROM_HERE,
          CrossThreadBindOnce(function, WTF::CrossThreadUnretained(thread),
                              parameters...));
      ++called_worker_count;
    }
    return called_worker_count;
  }

  int GetWorkerThreadId() const { return worker_thread_id_; }

  bool IsForciblyTerminated() LOCKS_EXCLUDED(lock_);

  void WaitForShutdownForTesting();
  ExitCode GetExitCodeForTesting() LOCKS_EXCLUDED(lock_);
  scoped_refptr<base::SingleThreadTaskRunner> GetParentTaskRunnerForTesting() {
    return parent_thread_default_task_runner_;
  }

  scheduler::WorkerScheduler* GetScheduler();

  // Returns a task runner bound to this worker. Users of the task runner don't
  // have to care about the lifetime of the worker. When the worker global scope
  // is destroyed, the task runner starts failing PostTask calls and discards
  // queued tasks. This function can be called from any threads.
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(TaskType type);

  void ChildThreadStartedOnWorkerThread(WorkerThread*);
  void ChildThreadTerminatedOnWorkerThread(WorkerThread*);

  // Changes the lifecycle state of the associated execution context for
  // this worker to Paused and may enter a nested run loop. Only one nested
  // message loop will be entered but |pause_or_freeze_count_| will be
  // incremented on each call. Inspector can call pause when this thread is
  // first created. May be called multiple times and from any thread.
  void Pause();

  // Changes the lifecycle state of the associated execution context for
  // this worker to FrozenPaused and may enter a nested run loop. Only one
  // nested message loop will be entered but |pause_or_freeze_count_| will be
  // incremented on each call. May be called multiple times and from any thread.
  //
  // `is_in_back_forward_cache` represents whether the page goes to back/forward
  // cache.
  void Freeze(bool is_in_back_forward_cache);

  // Decrements |pause_or_freeze_count_| and if count is zero then
  // it will exit the entered nested run loop. Might be called from any thread.
  void Resume();

 protected:
  explicit WorkerThread(WorkerReportingProxy&);
  // For service workers. When service workers are started on the IO thread
  // Thread::Current() wouldn't be available so we need to pass the parent
  // thread default task runner explicitly.
  WorkerThread(WorkerReportingProxy&,
               scoped_refptr<base::SingleThreadTaskRunner>
                   parent_thread_default_task_runner);

  virtual ThreadType GetThreadType() const = 0;

  // Official moment of creation of worker: when the worker thread is created.
  // (https://w3c.github.io/hr-time/#time-origin)
  const base::TimeTicks time_origin_;

 private:
  friend class WorkerThreadTest;
  FRIEND_TEST_ALL_PREFIXES(WorkerThreadTest, ShouldTerminateScriptExecution);
  FRIEND_TEST_ALL_PREFIXES(
      WorkerThreadTest,
      Terminate_WhileDebuggerTaskIsRunningOnInitialization);
  FRIEND_TEST_ALL_PREFIXES(WorkerThreadTest,
                           Terminate_WhileDebuggerTaskIsRunning);

  static base::Lock& ThreadSetLock();
  // Contains threads which are created but haven't started.
  static HashSet<WorkerThread*>& InitializingWorkerThreads()
      EXCLUSIVE_LOCKS_REQUIRED(ThreadSetLock());
  // Contains threads which have started.
  static HashSet<WorkerThread*>& WorkerThreads()
      EXCLUSIVE_LOCKS_REQUIRED(ThreadSetLock());

  // Represents the state of this worker thread.
  enum class ThreadState {
    kNotStarted,
    kRunning,
    kReadyToShutdown,
  };

  // Factory method for creating a new worker context for the thread.
  // Called on the worker thread.
  virtual WorkerOrWorkletGlobalScope* CreateWorkerGlobalScope(
      std::unique_ptr<GlobalScopeCreationParams>) = 0;

  // Returns true when this WorkerThread owns the associated
  // WorkerBackingThread exclusively. If this function returns true, the
  // WorkerThread initializes / shutdowns the backing thread. Otherwise
  // the backing thread should be initialized / shutdown properly out of this
  // class.
  virtual bool IsOwningBackingThread() const { return true; }

  // Posts a delayed task to forcibly terminate script execution in case the
  // normal shutdown sequence does not start within a certain time period.
  void ScheduleToTerminateScriptExecution();

  enum class TerminationState {
    kTerminate,
    kPostponeTerminate,
    kTerminationUnnecessary,
  };

  // Returns true if we should synchronously terminate the script execution so
  // that a shutdown task can be handled by the thread event loop.
  TerminationState ShouldTerminateScriptExecution()
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Terminates worker script execution if the worker thread is running and not
  // already shutting down. Does not terminate if a debugger task is running,
  // because the debugger task is guaranteed to finish and it heavily uses V8
  // API calls which would crash after forcible script termination. Called on
  // the parent thread.
  void EnsureScriptExecutionTerminates(ExitCode) LOCKS_EXCLUDED(lock_);

  // Called during worker startup.
  void InitializeOnWorkerThread(
      std::unique_ptr<GlobalScopeCreationParams>,
      const std::optional<WorkerBackingThreadStartupData>&,
      std::unique_ptr<WorkerDevToolsParams>) LOCKS_EXCLUDED(lock_);

  // Barrier that ensures that task runners are initialized. After
  // initialization, immediately returns. The barrier guards a small window
  // during initialization where task runners are not set up yet and unusable
  // from other threads.
  void MakeSureTaskRunnersAreInitialized();

  void EvaluateClassicScriptOnWorkerThread(
      const KURL& script_url,
      String source_code,
      std::unique_ptr<Vector<uint8_t>> cached_meta_data,
      const v8_inspector::V8StackTraceId& stack_id);
  void FetchAndRunClassicScriptOnWorkerThread(
      const KURL& script_url,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      std::unique_ptr<WebPolicyContainer> policy_container,
      std::unique_ptr<CrossThreadFetchClientSettingsObjectData>
          outside_settings_object,
      WorkerResourceTimingNotifier* outside_resource_timing_notifier,
      const v8_inspector::V8StackTraceId& stack_id);
  void FetchAndRunModuleScriptOnWorkerThread(
      const KURL& script_url,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      std::unique_ptr<WebPolicyContainer> policy_container,
      std::unique_ptr<CrossThreadFetchClientSettingsObjectData>
          outside_settings_object,
      WorkerResourceTimingNotifier* outside_resource_timing_notifier,
      network::mojom::CredentialsMode);

  // PrepareForShutdownOnWorkerThread() notifies that the context will be
  // destroyed, discards queued tasks to prevent running further tasks, and
  // initiates termination of nested workers. It runs on the worker thread. It
  // can be called due to the parent thread posting a task to run it on the
  // worker thread, or the worker thread calling it itself synchronously.
  //
  // PerformShutdownOnWorkerThread() destroys the global scope, and notifies the
  // parent thread of completion of worker shutdown. A call of this function can
  // be postponed until all nested workers are terminated. It runs on the worker
  // thread. It can be called due to the parent thread posting a task to run it
  // on the worker thread, or the worker thread calling it itself synchronously
  // after all nested workers are terminated.
  //
  // These are called in this order during worker shutdown.
  //
  // The reason why worker shutdown is separated into these 2 functions:
  // Workers can simultaneously be requested to terminate for various reasons.
  // To serialize the termination requests, worker shutdown is supposed to be
  // initiated from the parent thread (i.e., Terminate()). On the other hand,
  // queued tasks etc must be discarded as soon as possible after shutdown is
  // requested to prevent running further tasks. To be specific, when close() is
  // called on the worker global scope, queued tasks must be discarded soon
  // before worker shutdown is formally requested via the parent thread. The
  // HTML spec defines this behavior (see spec comments in DidProcessTask()).
  // To achieve this, the worker thread runs PrepareForShutdownOnWorkerThread()
  // immediately after the task that called close() (see DidProcessTask()), and
  // then posts a task to the parent thread to request termination. In addition
  // to that, separate functions are useful for waiting until all nested workers
  // are terminated before the parent thread shut down.
  void PrepareForShutdownOnWorkerThread() LOCKS_EXCLUDED(lock_);
  void PerformShutdownOnWorkerThread() LOCKS_EXCLUDED(lock_);

  void SetThreadState(ThreadState) EXCLUSIVE_LOCKS_REQUIRED(lock_);
  void SetExitCode(ExitCode) EXCLUSIVE_LOCKS_REQUIRED(lock_);

  bool CheckRequestedToTerminate() LOCKS_EXCLUDED(lock_);

  class InterruptData;
  void PauseOrFreeze(mojom::blink::FrameLifecycleState state,
                     bool is_in_back_forward_cache);
  void PauseOrFreezeOnWorkerThread(mojom::blink::FrameLifecycleState state,
                                   bool is_in_back_forward_cache);
  void ResumeOnWorkerThread();
  void PauseOrFreezeWithInterruptDataOnWorkerThread(InterruptData*);
  static void PauseOrFreezeInsideV8InterruptOnWorkerThread(v8::Isolate*,
                                                           void* data);
  static void PauseOrFreezeInsidePostTaskOnWorkerThread(
      InterruptData* interrupt_data);

  // A unique identifier among all WorkerThreads.
  const int worker_thread_id_;

  // Set on the parent thread.
  bool requested_to_terminate_ GUARDED_BY(lock_) = false;

  ThreadState thread_state_ GUARDED_BY(lock_) = ThreadState::kNotStarted;
  ExitCode exit_code_ GUARDED_BY(lock_) = ExitCode::kNotTerminated;

  base::TimeDelta forcible_termination_delay_;

  scoped_refptr<InspectorTaskRunner> inspector_task_runner_;
  base::UnguessableToken devtools_worker_token_;
  InspectorIssueStorage inspector_issue_storage_;
  int debugger_task_counter_ GUARDED_BY(lock_) = 0;

  WorkerReportingProxy& worker_reporting_proxy_;

  // Task runner bound with the parent thread's default task queue. Be careful
  // that a task runner may run even after the parent execution context and
  // |this| are destroyed.
  // This is used only for scheduling a worker termination and for testing.
  scoped_refptr<base::SingleThreadTaskRunner>
      parent_thread_default_task_runner_;

  // Tasks managed by this scheduler are canceled when the global scope is
  // closed.
  std::unique_ptr<scheduler::WorkerScheduler> worker_scheduler_;

  // Task runners bound with |worker_scheduler_|. These are captured when the
  // worker scheduler is initialized.
  using TaskRunnerHashMap =
      HashMap<TaskType, scoped_refptr<base::SingleThreadTaskRunner>>;
  TaskRunnerHashMap worker_task_runners_;
  std::atomic<bool> worker_task_runners_initialized_{false};

  // This lock protects shared states between the parent thread and the worker
  // thread. See thread-safety annotations (e.g., GUARDED_BY) in this header
  // file.
  base::Lock lock_;

  // Whether the thread is paused in a nested message loop or not. Used
  // only on the worker thread.
  int pause_or_freeze_count_ = 0;

  // The `PauseHandle` needs to be destroyed before the scheduler is destroyed
  // otherwise we will hit a DCHECK.
  std::unique_ptr<scheduler::WorkerScheduler::PauseHandle> pause_handle_;

  // A nested message loop for handling pausing. Pointer is not owned. Used only
  // on the worker thread.
  Platform::NestedMessageLoopRunner* nested_runner_ = nullptr;

  CrossThreadPersistent<ConsoleMessageStorage> console_message_storage_;
  CrossThreadPersistent<WorkerOrWorkletGlobalScope> global_scope_;
  CrossThreadPersistent<WorkerInspectorController> worker_inspector_controller_;

  // Signaled when the thread completes termination on the worker thread. Only
  // the parent context thread should wait on this event after calling
  // Terminate().
  class RefCountedWaitableEvent;
  scoped_refptr<RefCountedWaitableEvent> shutdown_event_;

  // Used to cancel a scheduled forcible termination task. See
  // mayForciblyTerminateExecution() for details.
  TaskHandle forcible_termination_task_handle_;

  HashSet<WorkerThread*> child_threads_;

  // List of data to passed into the interrupt callbacks. The V8 API takes
  // a void* and we need to pass more data that just a ptr, so we pass
  // a pointer to a member in this list.
  HashSet<std::unique_ptr<InterruptData>> pending_interrupts_ GUARDED_BY(lock_);

  // Since the WorkerThread is allocated and deallocated on the parent thread,
  // we need a WeakPtrFactory that is allocated and cleared on the backing
  // thread.
  std::optional<base::WeakPtrFactory<WorkerThread>>
      backing_thread_weak_factory_;

  THREAD_CHECKER(parent_thread_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_THREAD_H_
