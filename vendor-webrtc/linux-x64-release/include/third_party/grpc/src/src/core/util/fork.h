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

#ifndef GRPC_SRC_CORE_UTIL_FORK_H
#define GRPC_SRC_CORE_UTIL_FORK_H

#include <grpc/support/port_platform.h>

#include <atomic>
#include <set>

//
// NOTE: FORKING IS NOT GENERALLY SUPPORTED, THIS IS ONLY INTENDED TO WORK
//       AROUND VERY SPECIFIC USE CASES.
//

namespace grpc_core {

class GPR_DLL Fork {
 public:
  typedef void (*child_postfork_func)(void);

  static void GlobalInit();

  // Returns true if fork support is enabled, false otherwise
  static bool Enabled();

  // Increment the count of active ExecCtxs.
  // Will block until a pending fork is complete if one is in progress.
  static void IncExecCtxCount() {
    if (GPR_UNLIKELY(support_enabled_.load(std::memory_order_relaxed))) {
      DoIncExecCtxCount();
    }
  }

  // Decrement the count of active ExecCtxs
  static void DecExecCtxCount() {
    if (GPR_UNLIKELY(support_enabled_.load(std::memory_order_relaxed))) {
      DoDecExecCtxCount();
    }
  }

  // Provide a function that will be invoked in the child's postfork handler to
  // reset the polling engine's internal state.
  // Returns true if reset_child_polling_engine was not previously registered,
  // otherwise returns false and does nothing.
  static bool RegisterResetChildPollingEngineFunc(
      child_postfork_func reset_child_polling_engine);
  static const std::set<child_postfork_func>& GetResetChildPollingEngineFunc();

  // Check if there is a single active ExecCtx
  // (the one used to invoke this function).  If there are more,
  // return false.  Otherwise, return true and block creation of
  // more ExecCtx s until AllowExecCtx() is called
  //
  static bool BlockExecCtx();
  static void AllowExecCtx();

  // Increment the count of active threads.
  static void IncThreadCount();

  // Decrement the count of active threads.
  static void DecThreadCount();

  // Await all core threads to be joined.
  static void AwaitThreads();

  // Test only: overrides environment variables/compile flags
  // Must be called before grpc_init()
  static void Enable(bool enable);

 private:
  static void DoIncExecCtxCount();
  static void DoDecExecCtxCount();

  static std::atomic<bool> support_enabled_;
  static bool override_enabled_;
  static std::set<child_postfork_func>* reset_child_polling_engine_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_FORK_H
