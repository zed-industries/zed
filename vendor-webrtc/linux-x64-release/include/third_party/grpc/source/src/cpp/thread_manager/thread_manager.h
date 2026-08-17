//
//
// Copyright 2016 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CPP_THREAD_MANAGER_THREAD_MANAGER_H
#define GRPC_SRC_CPP_THREAD_MANAGER_THREAD_MANAGER_H

#include <list>

#include "src/core/lib/resource_quota/api.h"
#include "src/core/lib/resource_quota/thread_quota.h"
#include "src/core/util/sync.h"
#include "src/core/util/thd.h"

namespace grpc {

class ThreadManager {
 public:
  explicit ThreadManager(const char* name, grpc_resource_quota* resource_quota,
                         int min_pollers, int max_pollers);
  virtual ~ThreadManager();

  // Initializes and Starts the Rpc Manager threads
  void Initialize();

  // The return type of PollForWork() function
  enum WorkStatus { WORK_FOUND, SHUTDOWN, TIMEOUT };

  // "Polls" for new work.
  // If the return value is WORK_FOUND:
  //  - The implementation of PollForWork() MAY set some opaque identifier to
  //    (identify the work item found) via the '*tag' parameter
  //  - The implementation MUST set the value of 'ok' to 'true' or 'false'. A
  //    value of 'false' indicates some implementation specific error (that is
  //    neither SHUTDOWN nor TIMEOUT)
  //  - ThreadManager does not interpret the values of 'tag' and 'ok'
  //  - ThreadManager WILL call DoWork() and pass '*tag' and 'ok' as input to
  //    DoWork()
  //
  // If the return value is SHUTDOWN:,
  //  - ThreadManager WILL NOT call DoWork() and terminates the thread
  //
  // If the return value is TIMEOUT:,
  //  - ThreadManager WILL NOT call DoWork()
  //  - ThreadManager MAY terminate the thread depending on the current number
  //    of active poller threads and mix_pollers/max_pollers settings
  //  - Also, the value of timeout is specific to the derived class
  //    implementation
  virtual WorkStatus PollForWork(void** tag, bool* ok) = 0;

  // The implementation of DoWork() is supposed to perform the work found by
  // PollForWork(). The tag and ok parameters are the same as returned by
  // PollForWork(). The resources parameter indicates that the call actually
  // has the resources available for performing the RPC's work. If it doesn't,
  // the implementation should fail it appropriately.
  //
  // The implementation of DoWork() should also do any setup needed to ensure
  // that the next call to PollForWork() (not necessarily by the current thread)
  // actually finds some work
  virtual void DoWork(void* tag, bool ok, bool resources) = 0;

  // Mark the ThreadManager as shutdown and begin draining the work. This is a
  // non-blocking call and the caller should call Wait(), a blocking call which
  // returns only once the shutdown is complete
  virtual void Shutdown();

  // Has Shutdown() been called
  bool IsShutdown();

  // A blocking call that returns only after the ThreadManager has shutdown and
  // all the threads have drained all the outstanding work
  virtual void Wait();

  // Max number of concurrent threads that were ever active in this thread
  // manager so far. This is useful for debugging purposes (and in unit tests)
  // to check if resource_quota is properly being enforced.
  int GetMaxActiveThreadsSoFar();

 private:
  // Helper wrapper class around grpc_core::Thread. Takes a ThreadManager object
  // and starts a new grpc_core::Thread to calls the Run() function.
  //
  // The Run() function calls ThreadManager::MainWorkLoop() function and once
  // that completes, it marks the WorkerThread completed by calling
  // ThreadManager::MarkAsCompleted()
  //
  // WHY IS THIS NEEDED?:
  // When a thread terminates, some other thread *must* call Join() on that
  // thread so that the resources are released. Having a WorkerThread wrapper
  // will make this easier. Once Run() completes, each thread calls the
  // following two functions:
  //    ThreadManager::CleanupCompletedThreads()
  //    ThreadManager::MarkAsCompleted()
  //
  //  - MarkAsCompleted() puts the WorkerThread object in the ThreadManger's
  //    completed_threads_ list
  //  - CleanupCompletedThreads() calls "Join()" on the threads that are already
  //    in the completed_threads_ list  (since a thread cannot call Join() on
  //    itself, it calls CleanupCompletedThreads() *before* calling
  //    MarkAsCompleted())
  //
  // TODO(sreek): Consider creating the threads 'detached' so that Join() need
  // not be called (and the need for this WorkerThread class is eliminated)
  class WorkerThread {
   public:
    explicit WorkerThread(ThreadManager* thd_mgr);
    ~WorkerThread();

    bool created() const { return created_; }
    void Start() { thd_.Start(); }

   private:
    // Calls thd_mgr_->MainWorkLoop() and once that completes, calls
    // thd_mgr_>MarkAsCompleted(this) to mark the thread as completed
    void Run();

    ThreadManager* const thd_mgr_;
    grpc_core::Thread thd_;
    bool created_;
  };

  // The main function in ThreadManager
  void MainWorkLoop();

  void MarkAsCompleted(WorkerThread* thd);
  void CleanupCompletedThreads();

  // Protects shutdown_, num_pollers_, num_threads_ and
  // max_active_threads_sofar_
  grpc_core::Mutex mu_;

  bool shutdown_;
  grpc_core::CondVar shutdown_cv_;

  // The resource user object to use when requesting quota to create threads
  //
  // Note: The user of this ThreadManager object must create grpc_resource_quota
  // object (that contains the actual max thread quota) and a grpc_resource_user
  // object through which quota is requested whenever new threads need to be
  // created
  grpc_core::ThreadQuotaPtr thread_quota_;

  // Number of threads doing polling
  int num_pollers_;

  // The minimum and maximum number of threads that should be doing polling
  int min_pollers_;
  int max_pollers_;

  // The total number of threads currently active (includes threads includes the
  // threads that are currently polling i.e num_pollers_)
  int num_threads_;

  // See GetMaxActiveThreadsSoFar()'s description.
  // To be more specific, this variable tracks the max value num_threads_ was
  // ever set so far
  int max_active_threads_sofar_;

  grpc_core::Mutex list_mu_;
  std::list<WorkerThread*> completed_threads_;
};

}  // namespace grpc

#endif  // GRPC_SRC_CPP_THREAD_MANAGER_THREAD_MANAGER_H
