// Copyright 2021 The Crashpad Authors
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

#ifndef CRASHPAD_HANDLER_PRUNE_CRASH_REPORTS_THREAD_H_
#define CRASHPAD_HANDLER_PRUNE_CRASH_REPORTS_THREAD_H_

#include <memory>

#include "base/files/file_path.h"
#include "util/thread/stoppable.h"
#include "util/thread/worker_thread.h"

namespace crashpad {

class CrashReportDatabase;
class PruneCondition;

//! \brief A thread that periodically prunes crash reports from the database
//!     using the specified condition, and any leftover locked intermediate
//!     dumps.
//!
//! After the thread is started, the database is pruned using the condition
//! every 24 hours. Upon calling Start(), the thread waits before performing
//! the initial prune operation.
//!
//! Locked intermediate dump files are unlocked only once, not periodically.
//! Locked dumps that match this bundle id can be unlocked if they are over a
//! day old. Otherwise, unlock dumps that are over 60 days old.
class PruneIntermediateDumpsAndCrashReportsThread
    : public WorkerThread::Delegate,
      public Stoppable {
 public:
  //! \brief Constructs a new object.
  //!
  //! \param[in] database The database to prune crash reports from.
  //! \param[in] condition The condition used to evaluate crash reports for
  //!     pruning.
  //! \param[in] pending_path The path to any locked intermediate dump files.
  //! \param[in] bundle_identifier_and_seperator The identifier for this client,
  //!     used to determine when locked files are considered stale, with a
  //!     seperator at the end to allow for substring searches.
  //! \param[in] is_extension Whether the process is an app extension. If
  //!     `true`, the inital prune will occur after a 5-second delay. If
  //!     `false`, the initial prune will occur after a 60-second delay.
  PruneIntermediateDumpsAndCrashReportsThread(
      CrashReportDatabase* database,
      std::unique_ptr<PruneCondition> condition,
      base::FilePath pending_path,
      std::string bundle_identifier_and_seperator,
      bool is_extension);

  PruneIntermediateDumpsAndCrashReportsThread(
      const PruneIntermediateDumpsAndCrashReportsThread&) = delete;
  PruneIntermediateDumpsAndCrashReportsThread& operator=(
      const PruneIntermediateDumpsAndCrashReportsThread&) = delete;

  ~PruneIntermediateDumpsAndCrashReportsThread();

  // Stoppable:

  //! \brief Starts a dedicated pruning thread.
  //!
  //! The thread waits before running the initial prune, so as to not interfere
  //! with any startup-related IO performed by the client.
  //!
  //! This method may only be be called on a newly-constructed object or after
  //! a call to Stop().
  void Start() override;

  //! \brief Stops the pruning thread.
  //!
  //! This method must only be called after Start(). If Start() has been called,
  //! this method must be called before destroying an object of this class.
  //!
  //! This method may be called from any thread other than the pruning thread.
  //! It is expected to only be called from the same thread that called Start().
  void Stop() override;

  //! \return `true` if the thread is running, `false` if it is not.
  bool is_running() const { return thread_.is_running(); }

 private:
  // WorkerThread::Delegate:
  void DoWork(const WorkerThread* thread) override;

  WorkerThread thread_;
  std::unique_ptr<PruneCondition> condition_;
  base::FilePath pending_path_;
  std::string bundle_identifier_and_seperator_;
  bool clean_old_intermediate_dumps_;
  double initial_work_delay_;
  time_t last_start_time_;
  CrashReportDatabase* database_;  // weak
};

}  // namespace crashpad

#endif  // CRASHPAD_HANDLER_PRUNE_CRASH_REPORTS_THREAD_H_
