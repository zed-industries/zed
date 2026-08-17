//
//
// Copyright 2015 gRPC authors.
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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_THREAD_POOL_WORK_STEALING_THREAD_POOL_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_THREAD_POOL_WORK_STEALING_THREAD_POOL_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/thd_id.h>
#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <memory>

#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_set.h"
#include "absl/functional/any_invocable.h"
#include "src/core/lib/event_engine/thread_pool/thread_count.h"
#include "src/core/lib/event_engine/thread_pool/thread_pool.h"
#include "src/core/lib/event_engine/work_queue/basic_work_queue.h"
#include "src/core/lib/event_engine/work_queue/work_queue.h"
#include "src/core/util/backoff.h"
#include "src/core/util/notification.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"

namespace grpc_event_engine::experimental {

class WorkStealingThreadPool final : public ThreadPool {
 public:
  explicit WorkStealingThreadPool(size_t reserve_threads);
  // Asserts Quiesce was called.
  ~WorkStealingThreadPool() override;
  // Shut down the pool, and wait for all threads to exit.
  // This method is safe to call from within a ThreadPool thread.
  void Quiesce() override;
  // Run must not be called after Quiesce completes
  void Run(absl::AnyInvocable<void()> callback) override;
  void Run(EventEngine::Closure* closure) override;

  // Forkable
  // These methods are exposed on the public object to allow for testing.
  void PrepareFork() override;
  void PostforkParent() override;
  void PostforkChild() override;

 private:
  // A basic communication mechanism to signal waiting threads that work is
  // available.
  class WorkSignal {
   public:
    void Signal();
    void SignalAll();
    // Returns whether a timeout occurred.
    bool WaitWithTimeout(grpc_core::Duration time);

   private:
    grpc_core::Mutex mu_;
    grpc_core::CondVar cv_ ABSL_GUARDED_BY(mu_);
  };

  // A pool of WorkQueues that participate in work stealing.
  //
  // Every worker thread registers and unregisters its thread-local thread pool
  // here, and steals closures from other threads when work is otherwise
  // unavailable.
  class TheftRegistry {
   public:
    // Allow any member of the registry to steal from the provided queue.
    void Enroll(WorkQueue* queue) ABSL_LOCKS_EXCLUDED(mu_);
    // Disallow work stealing from the provided queue.
    void Unenroll(WorkQueue* queue) ABSL_LOCKS_EXCLUDED(mu_);
    // Returns one closure from another thread, or nullptr if none are
    // available.
    EventEngine::Closure* StealOne() ABSL_LOCKS_EXCLUDED(mu_);

   private:
    grpc_core::Mutex mu_;
    absl::flat_hash_set<WorkQueue*> queues_ ABSL_GUARDED_BY(mu_);
  };

  // An implementation of the ThreadPool
  // This object is held as a shared_ptr between the owning ThreadPool and each
  // worker thread. This design allows a ThreadPool worker thread to be the last
  // owner of the ThreadPool itself.
  class WorkStealingThreadPoolImpl
      : public std::enable_shared_from_this<WorkStealingThreadPoolImpl> {
   public:
    explicit WorkStealingThreadPoolImpl(size_t reserve_threads);
    // Start all threads.
    void Start();
    // Add a closure to a work queue, preferably a thread-local queue if
    // available, otherwise the global queue.
    void Run(EventEngine::Closure* closure);
    // Start a new thread.
    // The reason argument determines whether thread creation is rate-limited;
    // threads created to populate the initial pool are not rate-limited, but
    // all others thread creation scenarios are rate-limited.
    void StartThread();
    // Shut down the pool, and wait for all threads to exit.
    // This method is safe to call from within a ThreadPool thread.
    void Quiesce();
    // Sets a throttled state.
    // After the initial pool has been created, if the pool is backlogged when
    // a new thread has started, it is rate limited.
    // Returns the previous throttling state.
    bool SetThrottled(bool throttle);
    // Set the shutdown flag.
    void SetShutdown(bool is_shutdown);
    // Set the forking flag.
    void SetForking(bool is_forking);
    // Forkable
    // Ensures that the thread pool is empty before forking.
    // Postfork parent and child have the same behavior.
    void PrepareFork();
    void Postfork();
    // Thread ID tracking
    void TrackThread(gpr_thd_id tid);
    void UntrackThread(gpr_thd_id tid);
    // Accessor methods
    bool IsShutdown();
    bool IsForking();
    bool IsQuiesced();
    size_t reserve_threads() { return reserve_threads_; }
    BusyThreadCount* busy_thread_count() { return &busy_thread_count_; }
    LivingThreadCount* living_thread_count() { return &living_thread_count_; }
    TheftRegistry* theft_registry() { return &theft_registry_; }
    WorkQueue* queue() { return &queue_; }
    WorkSignal* work_signal() { return &work_signal_; }

   private:
    // Lifeguard monitors the pool and keeps it healthy.
    // It has two main responsibilities:
    //  * scale the pool to match demand.
    //  * distribute work to worker threads if the global queue is backing up
    //    and there are threads that can accept work.
    class Lifeguard {
     public:
      explicit Lifeguard(WorkStealingThreadPoolImpl* pool);
      ~Lifeguard();

     private:
      // The main body of the lifeguard thread.
      void LifeguardMain();
      // Starts a new thread if the pool is backlogged
      // Return true if a new thread was started.
      bool MaybeStartNewThread();

      WorkStealingThreadPoolImpl* pool_;
      grpc_core::BackOff backoff_;
      // Used for signaling that the lifeguard thread has stopped running.
      std::unique_ptr<grpc_core::Notification> lifeguard_should_shut_down_;
      std::unique_ptr<grpc_core::Notification> lifeguard_is_shut_down_;
      std::atomic<bool> lifeguard_running_{false};
    };

    void DumpStacksAndCrash();

    const size_t reserve_threads_;
    BusyThreadCount busy_thread_count_;
    LivingThreadCount living_thread_count_;
    TheftRegistry theft_registry_;
    BasicWorkQueue queue_;
    // Track shutdown and fork bits separately.
    // It's possible for a ThreadPool to initiate shut down while fork handlers
    // are running, and similarly possible for a fork event to occur during
    // shutdown.
    std::atomic<bool> shutdown_{false};
    std::atomic<bool> forking_{false};
    std::atomic<bool> quiesced_{false};
    std::atomic<uint64_t> last_started_thread_{0};
    // After pool creation we use this to rate limit creation of threads to one
    // at a time.
    std::atomic<bool> throttled_{false};
    WorkSignal work_signal_;
    grpc_core::Mutex lifeguard_ptr_mu_;
    std::unique_ptr<Lifeguard> lifeguard_ ABSL_GUARDED_BY(lifeguard_ptr_mu_);
    // Set of threads for verbose failure debugging
    grpc_core::Mutex thd_set_mu_;
    absl::flat_hash_set<gpr_thd_id> thds_ ABSL_GUARDED_BY(thd_set_mu_);
  };

  class ThreadState {
   public:
    explicit ThreadState(std::shared_ptr<WorkStealingThreadPoolImpl> pool);
    void ThreadBody();
    void SleepIfRunning();
    bool Step();
    // After the pool is shut down, ensure all local and global callbacks are
    // executed before quitting the thread.
    void FinishDraining();

   private:
    // pool_ must be the first member so that it is alive when the thread count
    // is decremented at time of destruction. This is necessary when this thread
    // state holds the last shared_ptr keeping the pool alive.
    std::shared_ptr<WorkStealingThreadPoolImpl> pool_;
    // auto_thread_counter_ must be declared after pool_, so that the thread
    // count is decremented after all other pool state is cleaned up.
    LivingThreadCount::AutoThreadCounter auto_thread_counter_;
    grpc_core::BackOff backoff_;
    size_t busy_count_idx_;
  };

  const std::shared_ptr<WorkStealingThreadPoolImpl> pool_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_THREAD_POOL_WORK_STEALING_THREAD_POOL_H
