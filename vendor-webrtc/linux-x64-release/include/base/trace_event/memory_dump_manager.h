// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_MEMORY_DUMP_MANAGER_H_
#define BASE_TRACE_EVENT_MEMORY_DUMP_MANAGER_H_

#include <stdint.h>

#include <map>
#include <memory>
#include <unordered_set>
#include <vector>

#include "base/base_export.h"
#include "base/gtest_prod_util.h"
#include "base/memory/singleton.h"
#include "base/synchronization/lock.h"
#include "base/trace_event/memory_allocator_dump.h"
#include "base/trace_event/memory_dump_provider_info.h"
#include "base/trace_event/memory_dump_request_args.h"
#include "base/trace_event/process_memory_dump.h"
#include "base/trace_event/trace_event.h"

namespace base {

class SequencedTaskRunner;
class SingleThreadTaskRunner;
class Thread;

namespace trace_event {

class MemoryDumpProvider;

// This is the interface exposed to the rest of the codebase to deal with
// memory tracing. The main entry point for clients is represented by
// RequestDumpPoint(). The extension by Un(RegisterDumpProvider).
class BASE_EXPORT MemoryDumpManager {
 public:
  using RequestGlobalDumpFunction =
      RepeatingCallback<void(MemoryDumpType, MemoryDumpLevelOfDetail)>;

  static constexpr const char* const kTraceCategory =
      TRACE_DISABLED_BY_DEFAULT("memory-infra");

  // This value is returned as the tracing id of the child processes by
  // GetTracingProcessId() when tracing is not enabled.
  static const uint64_t kInvalidTracingProcessId;

  static MemoryDumpManager* GetInstance();
  static std::unique_ptr<MemoryDumpManager> CreateInstanceForTesting();

  // Resets the initialization. When destroying and recreating the manager in
  // multi threaded environment is hard, this method can be used to reset the
  // state to begin new initialization.
  void ResetForTesting();

  MemoryDumpManager(const MemoryDumpManager&) = delete;
  MemoryDumpManager& operator=(const MemoryDumpManager&) = delete;

  // Invoked once per process to listen to trace begin / end events.
  // Initialization can happen after (Un)RegisterMemoryDumpProvider() calls
  // and the MemoryDumpManager guarantees to support this.
  // On the other side, the MemoryDumpManager will not be fully operational
  // (any CreateProcessDump() will return a failure) until initialized.
  // Arguments:
  //  is_coordinator: True when current process coordinates the periodic dump
  //      triggering.
  //  request_dump_function: Function to invoke a global dump. Global dump
  //      involves embedder-specific behaviors like multiprocess handshaking.
  //      TODO(primiano): this is only required to trigger global dumps from
  //      the scheduler. Should be removed once they are both moved out of base.
  void Initialize(RequestGlobalDumpFunction request_dump_function,
                  bool is_coordinator);

  // (Un)Registers a MemoryDumpProvider instance.
  // Args:
  //  - mdp: the MemoryDumpProvider instance to be registered. MemoryDumpManager
  //      does NOT take memory ownership of |mdp|, which is expected to either
  //      be a singleton or unregister itself.
  //  - name: a friendly name (duplicates allowed). Used for debugging and
  //      run-time profiling of memory-infra internals. Must be a long-lived
  //      C string.
  //  - task_runner: either a SingleThreadTaskRunner or SequencedTaskRunner. All
  //      the calls to |mdp| will be run on the given |task_runner|. If passed
  //      null |mdp| should be able to handle calls on arbitrary threads.
  //  - options: extra optional arguments. See memory_dump_provider.h.
  void RegisterDumpProvider(MemoryDumpProvider* mdp,
                            const char* name,
                            scoped_refptr<SingleThreadTaskRunner> task_runner);
  void RegisterDumpProvider(MemoryDumpProvider* mdp,
                            const char* name,
                            scoped_refptr<SingleThreadTaskRunner> task_runner,
                            MemoryDumpProvider::Options options);
  void RegisterDumpProviderWithSequencedTaskRunner(
      MemoryDumpProvider* mdp,
      const char* name,
      scoped_refptr<SequencedTaskRunner> task_runner,
      MemoryDumpProvider::Options options);
  void UnregisterDumpProvider(MemoryDumpProvider* mdp);

  // Unregisters an unbound dump provider and takes care about its deletion
  // asynchronously. Can be used only for for dump providers with no
  // task-runner affinity.
  // This method takes ownership of the dump provider and guarantees that:
  //  - The |mdp| will be deleted at some point in the near future.
  //  - Its deletion will not happen concurrently with the OnMemoryDump() call.
  // Note that OnMemoryDump() calls can still happen after this method returns.
  void UnregisterAndDeleteDumpProviderSoon(
      std::unique_ptr<MemoryDumpProvider> mdp);

  // Prepare MemoryDumpManager for CreateProcessDump() calls for tracing-related
  // modes (i.e. |level_of_detail| != SUMMARY_ONLY).
  // Also initializes the scheduler with the given config.
  void SetupForTracing(const TraceConfig::MemoryDumpConfig&);

  // Tear-down tracing related state.
  // Non-tracing modes (e.g. SUMMARY_ONLY) will continue to work.
  void TeardownForTracing();

  // Creates a memory dump for the current process and appends it to the trace.
  // |callback| will be invoked asynchronously upon completion on the same
  // thread on which CreateProcessDump() was called. This method should only be
  // used by the memory-infra service while creating a global memory dump.
  void CreateProcessDump(const MemoryDumpRequestArgs& args,
                         ProcessMemoryDumpCallback callback);

  // Lets tests see if a dump provider is registered.
  bool IsDumpProviderRegisteredForTesting(MemoryDumpProvider*);

  // Returns a unique id for identifying the processes. The id can be
  // retrieved by child processes only when tracing is enabled. This is
  // intended to express cross-process sharing of memory dumps on the
  // child-process side, without having to know its own child process id.
  uint64_t GetTracingProcessId() const { return tracing_process_id_; }
  void set_tracing_process_id(uint64_t tracing_process_id) {
    tracing_process_id_ = tracing_process_id;
  }

  // Returns the name for a the allocated_objects dump. Use this to declare
  // suballocator dumps from other dump providers.
  // It will return nullptr if there is no dump provider for the system
  // allocator registered (which is currently the case for Mac OS).
  const char* system_allocator_pool_name() const {
    return kSystemAllocatorPoolName;
  }

  // When set to true, calling |RegisterMemoryDumpProvider| is a no-op.
  void set_dumper_registrations_ignored_for_testing(bool ignored) {
    dumper_registrations_ignored_for_testing_ = ignored;
  }

  bool IsInitialized() {
    AutoLock lock(lock_);
    return can_request_global_dumps();
  }

  scoped_refptr<SequencedTaskRunner> GetDumpThreadTaskRunner();

 private:
  friend std::default_delete<MemoryDumpManager>;  // For the testing instance.
  friend struct DefaultSingletonTraits<MemoryDumpManager>;
  friend class MemoryDumpManagerTest;
  FRIEND_TEST_ALL_PREFIXES(MemoryDumpManagerTest,
                           NoStackOverflowWithTooManyMDPs);

  // Holds the state of a process memory dump that needs to be carried over
  // across task runners in order to fulfill an asynchronous CreateProcessDump()
  // request. At any time exactly one task runner owns a
  // ProcessMemoryDumpAsyncState.
  struct ProcessMemoryDumpAsyncState {
    ProcessMemoryDumpAsyncState(
        MemoryDumpRequestArgs req_args,
        const MemoryDumpProviderInfo::OrderedSet& dump_providers,
        ProcessMemoryDumpCallback callback,
        scoped_refptr<SequencedTaskRunner> dump_thread_task_runner);
    ProcessMemoryDumpAsyncState(const ProcessMemoryDumpAsyncState&) = delete;
    ProcessMemoryDumpAsyncState& operator=(const ProcessMemoryDumpAsyncState&) =
        delete;
    ~ProcessMemoryDumpAsyncState();

    // A ProcessMemoryDump to collect data from MemoryDumpProviders.
    std::unique_ptr<ProcessMemoryDump> process_memory_dump;

    // The arguments passed to the initial CreateProcessDump() request.
    const MemoryDumpRequestArgs req_args;

    // An ordered sequence of dump providers that have to be invoked to complete
    // the dump. This is a copy of |dump_providers_| at the beginning of a dump
    // and becomes empty at the end, when all dump providers have been invoked.
    std::vector<scoped_refptr<MemoryDumpProviderInfo>> pending_dump_providers;

    // Callback passed to the initial call to CreateProcessDump().
    ProcessMemoryDumpCallback callback;

    // The thread on which FinishAsyncProcessDump() (and hence |callback|)
    // should be invoked. This is the thread on which the initial
    // CreateProcessDump() request was called.
    const scoped_refptr<SingleThreadTaskRunner> callback_task_runner;

    // The thread on which unbound dump providers should be invoked.
    // This is essentially |dump_thread_|.task_runner() but needs to be kept
    // as a separate variable as it needs to be accessed by arbitrary dumpers'
    // threads outside of the lock_ to avoid races when disabling tracing.
    // It is immutable for all the duration of a tracing session.
    const scoped_refptr<SequencedTaskRunner> dump_thread_task_runner;
  };

  static const int kMaxConsecutiveFailuresCount;
  static const char* const kSystemAllocatorPoolName;

  MemoryDumpManager();
  virtual ~MemoryDumpManager();

  static void SetInstanceForTesting(MemoryDumpManager* instance);

  // Lazily initializes dump_thread_ and returns its TaskRunner.
  scoped_refptr<base::SequencedTaskRunner> GetOrCreateBgTaskRunnerLocked()
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Calls InvokeOnMemoryDump() for the each MDP that belongs to the current
  // task runner and switches to the task runner of the next MDP. Handles
  // failures in MDP and thread hops, and always calls FinishAsyncProcessDump()
  // at the end.
  void ContinueAsyncProcessDump(
      ProcessMemoryDumpAsyncState* owned_pmd_async_state);

  // Invokes OnMemoryDump() of the given MDP. Should be called on the MDP task
  // runner.
  void InvokeOnMemoryDump(MemoryDumpProviderInfo* mdpinfo,
                          ProcessMemoryDump* pmd);

  void FinishAsyncProcessDump(
      std::unique_ptr<ProcessMemoryDumpAsyncState> pmd_async_state);

  // Helper for RegierDumpProvider* functions.
  void RegisterDumpProviderInternal(
      MemoryDumpProvider* mdp,
      const char* name,
      scoped_refptr<SequencedTaskRunner> task_runner,
      const MemoryDumpProvider::Options& options);

  // Helper for the public UnregisterDumpProvider* functions.
  void UnregisterDumpProviderInternal(MemoryDumpProvider* mdp,
                                      bool take_mdp_ownership_and_delete_async);

  bool can_request_global_dumps() const {
    return !request_dump_function_.is_null();
  }

  // An ordered set of registered MemoryDumpProviderInfo(s), sorted by task
  // runner affinity (MDPs belonging to the same task runners are adjacent).
  MemoryDumpProviderInfo::OrderedSet dump_providers_ GUARDED_BY(lock_);

  // Function provided by the embedder to handle global dump requests.
  RequestGlobalDumpFunction request_dump_function_;

  // True when current process coordinates the periodic dump triggering.
  bool is_coordinator_ GUARDED_BY(lock_) = false;

  // Protects from concurrent accesses to the local state, eg: to guard against
  // disabling logging while dumping on another thread.
  Lock lock_;

  // Thread used for MemoryDumpProviders which don't specify a task runner
  // affinity.
  std::unique_ptr<Thread> dump_thread_ GUARDED_BY(lock_);

  // The unique id of the child process. This is created only for tracing and is
  // expected to be valid only when tracing is enabled.
  uint64_t tracing_process_id_ = kInvalidTracingProcessId;

  // When true, calling |RegisterMemoryDumpProvider| is a no-op.
  bool dumper_registrations_ignored_for_testing_ = false;
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_MEMORY_DUMP_MANAGER_H_
