//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_TIMER_MANAGER_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_TIMER_MANAGER_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "src/core/lib/event_engine/forkable.h"
#include "src/core/lib/event_engine/posix_engine/timer.h"
#include "src/core/lib/event_engine/thread_pool/thread_pool.h"
#include "src/core/util/notification.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"

namespace grpc_event_engine::experimental {

// Timer Manager tries to keep only one thread waiting for the next timeout at
// all times, and thus effectively preventing the thundering herd problem.
// TODO(ctiller): consider unifying this thread pool and the one in
// thread_pool.{h,cc}.
class TimerManager final : public grpc_event_engine::experimental::Forkable {
 public:
  explicit TimerManager(
      std::shared_ptr<grpc_event_engine::experimental::ThreadPool> thread_pool);
  ~TimerManager() override;

  grpc_core::Timestamp Now() { return host_.Now(); }

  void TimerInit(Timer* timer, grpc_core::Timestamp deadline,
                 experimental::EventEngine::Closure* closure);
  bool TimerCancel(Timer* timer);

  static bool IsTimerManagerThread();

  // Called on destruction, prefork, and manually when needed.
  void Shutdown();

  void PrepareFork() override;
  void PostforkParent() override;
  void PostforkChild() override;

 private:
  class Host final : public TimerListHost {
   public:
    explicit Host(TimerManager* timer_manager)
        : timer_manager_(timer_manager) {}

    void Kick() override;
    grpc_core::Timestamp Now() override;

   private:
    TimerManager* const timer_manager_;
  };

  void RestartPostFork();
  void MainLoop();
  void RunSomeTimers(std::vector<experimental::EventEngine::Closure*> timers);
  bool WaitUntil(grpc_core::Timestamp next);
  void Kick();

  grpc_core::Mutex mu_;
  // Condvar associated with the main thread waiting to wakeup and work.
  // Threads wait on this until either a timeout is reached or the timer manager
  // is kicked. On shutdown we Signal against this to wake up all threads and
  // have them finish. On kick we Signal against this to wake up the main
  // thread.
  grpc_core::CondVar cv_wait_;
  Host host_;
  // are we shutting down?
  bool shutdown_ ABSL_GUARDED_BY(mu_) = false;
  // are we shutting down?
  bool kicked_ ABSL_GUARDED_BY(mu_) = false;
  // number of timer wakeups
  uint64_t wakeups_ ABSL_GUARDED_BY(mu_) = false;
  // actual timer implementation
  std::unique_ptr<TimerList> timer_list_;
  std::shared_ptr<grpc_event_engine::experimental::ThreadPool> thread_pool_;
  std::optional<grpc_core::Notification> main_loop_exit_signal_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_TIMER_MANAGER_H
