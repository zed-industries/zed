// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_IMPORTANT_FILE_WRITER_CLEANER_H_
#define BASE_FILES_IMPORTANT_FILE_WRITER_CLEANER_H_

#include <atomic>
#include <vector>

#include "base/base_export.h"
#include "base/containers/flat_set.h"
#include "base/files/file_path.h"
#include "base/memory/scoped_refptr.h"
#include "base/no_destructor.h"
#include "base/numerics/clamped_math.h"
#include "base/sequence_checker.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"

namespace base {

class SequencedTaskRunner;

// A cleaner for forgotten .tmp files left behind by ImportantFileWriter; see
// https://crbug.com/1075917.
//
// ImportantFileWriter has the potential to leak .tmp files in case of a crash
// or power failure during processing, or in case of interference by third-party
// software. This class implements a singleton that makes a single scan over
// given directories to delete any *.tmp files older than the current process.
// Processes that use ImportantFileWriter are expected to call the instance's
// Start method at some point during startup to enable the cleaner.
// ImportantFileWriter calls the AddDirectory method to provide the directory
// hosting an "important" file. Hosting processes are expected to call the Stop
// method at shutdown.
//
// The deletion scan takes place in a background task.
class BASE_EXPORT ImportantFileWriterCleaner {
 public:
  // Gets the process-wide single instance of the cleaner.
  static ImportantFileWriterCleaner& GetInstance();

  ImportantFileWriterCleaner(const ImportantFileWriterCleaner&) = delete;
  ImportantFileWriterCleaner& operator=(const ImportantFileWriterCleaner&) =
      delete;
  ~ImportantFileWriterCleaner() = delete;

  // Adds |directory| to the set to be cleaned if it has not already been
  // handled. If the Start method has already been called, the cleaner will
  // begin processing |directory| after all others that have previously been
  // added have been cleaned (immediately, if there are no others). Any calls to
  // this method prior to Initialize are ignored.
  static void AddDirectory(const FilePath& directory);

  // Initializes the instance on the hosting process's main sequence (the one on
  // which Start and Stop will ultimately be called). It is safe to call this
  // any number of times from the main sequence.
  void Initialize();

  // Starts the instance. If any directories have already been added, the
  // background task is posted immediately to begin processing them. Otherwise,
  // the next call to AddDirectory will begin processing.
  void Start();

  // Stops the instance. The background task, if it is active, is notified to
  // halt processing and return.
  void Stop();

  // Brings the instance back to the uninitialized state. This should be used in
  // tests that call Initialize so that the instance forgets about the test's
  // main thread task runner.
  void UninitializeForTesting();

  // Returns the upper-bound time. Files with modification times older than this
  // are assumed to have been orphaned by a previous instance of the process.
  base::Time GetUpperBoundTimeForTest() const;

 private:
  friend class NoDestructor<ImportantFileWriterCleaner>;

  ImportantFileWriterCleaner();

  // True once Start() has been called; false following Stop();
  bool is_started() const {
    DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
    return started_;
  }

  // True once the background task has been posted; false once it returns.
  bool is_running() const {
    DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
    return running_;
  }

  // The workhorse for AddDirectory.
  void AddDirectoryImpl(const FilePath& directory);

  // Schedules the background task to run, processing all directories that have
  // accumulated.
  void ScheduleTask();

  // Iterates over the contents of |directories|, deleting all *.tmp files older
  // than |upper_bound_time|. Checks |stop_flag| after each deletion to see if
  // the instance has been stopped by the host process. Returns false if
  // processing was interrupted by |stop_flag| having been set, or true
  // indicating that all directories were fully processed.
  static bool CleanInBackground(Time upper_bound_time,
                                std::vector<FilePath> directories,
                                std::atomic_bool& stop_flag);

  // Cleans up after completion of the background task. |processing_completed|
  // is true when all directories were fully processed, or false if the task
  // potentially exited early in response to Stop().
  void OnBackgroundTaskFinished(bool processing_completed);

  // Finalizes a request to stop after the background task returns.
  void DoStop();

  // Provides exclusive access to the instance's task runner.
  Lock task_runner_lock_;

  // The hosting process's main thread task runner.
  scoped_refptr<SequencedTaskRunner> task_runner_ GUARDED_BY(task_runner_lock_);

  // The time before which any discovered temporary file is presumed to be
  // unused, and therefore safe to delete.
  const Time upper_bound_time_;

  // The set of all directories hosting files written by an ImportantFileWriter.
  flat_set<FilePath> important_directories_
      GUARDED_BY_CONTEXT(sequence_checker_);

  // Directories added to the instance waiting either for a call to Start() or
  // waiting for an existing background task to complete.
  std::vector<FilePath> pending_directories_
      GUARDED_BY_CONTEXT(sequence_checker_);

  std::atomic_bool stop_flag_{false};

  bool started_ GUARDED_BY_CONTEXT(sequence_checker_) = false;
  bool running_ GUARDED_BY_CONTEXT(sequence_checker_) = false;

  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace base

#endif  // BASE_FILES_IMPORTANT_FILE_WRITER_CLEANER_H_
