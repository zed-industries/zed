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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_EXECUTOR_H
#define GRPC_SRC_CORE_LIB_IOMGR_EXECUTOR_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/closure.h"
#include "src/core/util/spinlock.h"
#include "src/core/util/thd.h"

namespace grpc_core {

struct ThreadState {
  gpr_mu mu;
  size_t id;         // For debugging purposes
  const char* name;  // Thread state name
  gpr_cv cv;
  grpc_closure_list elems;
  size_t depth;  // Number of closures in the closure list
  bool shutdown;
  bool queued_long_job;
  Thread thd;
};

enum class ExecutorType {
  DEFAULT = 0,
  RESOLVER,

  NUM_EXECUTORS  // Add new values above this
};

enum class ExecutorJobType {
  SHORT = 0,
  LONG,
  NUM_JOB_TYPES  // Add new values above this
};

class Executor {
 public:
  explicit Executor(const char* executor_name);

  void Init();

  /// Is the executor multi-threaded?
  bool IsThreaded() const;

  // Enable/disable threading - must be called after Init and Shutdown(). Never
  // call SetThreading(false) in the middle of an application
  void SetThreading(bool threading);

  /// Shutdown the executor, running all pending work as part of the call
  void Shutdown();

  /// Enqueue the closure onto the executor. is_short is true if the closure is
  /// a short job (i.e expected to not block and complete quickly)
  void Enqueue(grpc_closure* closure, grpc_error_handle error, bool is_short);

  // TODO(sreek): Currently we have two executors (available globally): The
  // default executor and the resolver executor.
  //
  // Some of the functions below operate on the DEFAULT executor only while some
  // operate of ALL the executors. This is a bit confusing and should be cleaned
  // up in future (where we make all the following functions take ExecutorType
  // and/or JobType)

  // Initialize ALL the executors
  static void InitAll();

  static void Run(grpc_closure* closure, grpc_error_handle error,
                  ExecutorType executor_type = ExecutorType::DEFAULT,
                  ExecutorJobType job_type = ExecutorJobType::SHORT);

  // Shutdown ALL the executors
  static void ShutdownAll();

  // Set the threading mode for ALL the executors
  static void SetThreadingAll(bool enable);

  // Set the threading mode for ALL the executors
  static void SetThreadingDefault(bool enable);

  // Return if a given executor is running in threaded mode (i.e if
  // SetThreading(true) was called previously on that executor)
  static bool IsThreaded(ExecutorType executor_type);

  // Return if the DEFAULT executor is threaded
  static bool IsThreadedDefault();

 private:
  static size_t RunClosures(const char* executor_name, grpc_closure_list list);
  static void ThreadMain(void* arg);

  const char* name_;
  ThreadState* thd_state_;
  size_t max_threads_;
  gpr_atm num_threads_;
  gpr_spinlock adding_thread_lock_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_IOMGR_EXECUTOR_H
