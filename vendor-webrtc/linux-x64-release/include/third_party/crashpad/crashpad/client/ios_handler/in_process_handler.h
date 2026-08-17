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

#ifndef CRASHPAD_CLIENT_IOS_HANDLER_IN_PROCESS_IN_PROCESS_HANDLER_H_
#define CRASHPAD_CLIENT_IOS_HANDLER_IN_PROCESS_IN_PROCESS_HANDLER_H_

#include <mach/mach.h>
#include <stdint.h>

#include <atomic>
#include <functional>
#include <map>
#include <string>
#include <vector>

#include "base/files/file_path.h"
#include "base/synchronization/lock.h"
#include "client/ios_handler/prune_intermediate_dumps_and_crash_reports_thread.h"
#include "client/upload_behavior_ios.h"
#include "handler/crash_report_upload_thread.h"
#include "handler/user_stream_data_source.h"
#include "snapshot/ios/process_snapshot_ios_intermediate_dump.h"
#include "util/ios/ios_intermediate_dump_writer.h"
#include "util/ios/ios_system_data_collector.h"
#include "util/mach/mach_extensions.h"
#include "util/misc/capture_context.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {
namespace internal {

//! \brief Manage intermediate minidump generation, and own the crash report
//!     upload thread and database.
class InProcessHandler {
 public:
  InProcessHandler();
  ~InProcessHandler();
  InProcessHandler(const InProcessHandler&) = delete;
  InProcessHandler& operator=(const InProcessHandler&) = delete;

  //! \brief Observation callback invoked each time this object finishes
  //!     processing and attempting to upload on-disk crash reports (whether or
  //!     not the uploads succeeded).
  //!
  //! This callback is copied into this object. Any references or pointers
  //! inside must outlive this object.
  //!
  //! The callback might be invoked on a background thread, so clients must
  //! synchronize appropriately.
  using ProcessPendingReportsObservationCallback = std::function<void()>;

  //! \brief Initializes the in-process handler.
  //!
  //! This method must be called only once, and must be successfully called
  //! before any other method in this class may be called.
  //!
  //! \param[in] database The path to a Crashpad database.
  //! \param[in] url The URL of an upload server.
  //! \param[in] annotations Process annotations to set in each crash report.
  //! \param[in] callback Optional callback invoked zero or more times
  //!     on a background thread each time this object finishes
  //!     processing and attempting to upload on-disk crash reports.
  //! \return `true` if a handler to a pending intermediate dump could be
  //!     opened.
  bool Initialize(const base::FilePath& database,
                  const std::string& url,
                  const std::map<std::string, std::string>& annotations,
                  ProcessPendingReportsObservationCallback callback =
                      ProcessPendingReportsObservationCallback());

  //! \brief Generate an intermediate dump from a signal handler exception.
  //!      Writes the dump with the cached writer does not allow concurrent
  //!      exceptions to be written. It is expected the system will terminate
  //!      the application after this call.
  //!
  //! \param[in] siginfo A pointer to a `siginfo_t` object received by a signal
  //!     handler.
  //! \param[in] context A pointer to a `ucontext_t` object received by a
  //!     signal.
  void DumpExceptionFromSignal(siginfo_t* siginfo, ucontext_t* context);

  //! \brief Generate an intermediate dump from a mach exception. Writes the
  //!     dump with the cached writer does not allow concurrent exceptions to be
  //!     written. It is expected the system will terminate the application
  //!     after this call.
  //!
  //! \param[in] behavior
  //! \param[in] thread
  //! \param[in] exception
  //! \param[in] code
  //! \param[in] code_count
  //! \param[in,out] flavor
  //! \param[in] old_state
  //! \param[in] old_state_count
  void DumpExceptionFromMachException(exception_behavior_t behavior,
                                      thread_t thread,
                                      exception_type_t exception,
                                      const mach_exception_data_type_t* code,
                                      mach_msg_type_number_t code_count,
                                      thread_state_flavor_t flavor,
                                      ConstThreadState old_state,
                                      mach_msg_type_number_t old_state_count);

  //! \brief Generate an intermediate dump from an uncaught NSException.
  //!
  //! When the ObjcExceptionPreprocessor does not detect an NSException as it is
  //! thrown, the last-chance uncaught exception handler passes a list of call
  //! stack frame addresses.  Record them in the intermediate dump so a minidump
  //! with a 'fake' call stack is generated.  Writes the dump with the cached
  //! writer does not allow concurrent exceptions to be written. It is expected
  //! the system will terminate the application after this call.

  //!
  //! \param[in] frames An array of call stack frame addresses.
  //! \param[in] num_frames The number of frames in |frames|.
  void DumpExceptionFromNSExceptionWithFrames(const uint64_t* frames,
                                              const size_t num_frames);

  //! \brief Generate a simulated intermediate dump similar to a Mach exception
  //!     in the same base directory as other exceptions. Does not use the
  //!     cached writer.
  //!
  //! \param[in] context A pointer to a NativeCPUContext object for this
  //!     simulated exception.
  //! \param[in] exception
  //! \param[out] path The path of the intermediate dump generated.
  //! \return `true` if the pending intermediate dump could be written.
  bool DumpExceptionFromSimulatedMachException(const NativeCPUContext* context,
                                               exception_type_t exception,
                                               base::FilePath* path);

  //! \brief Generate a simulated intermediate dump similar to a Mach exception
  //!     at a specific path. Does not use the cached writer.
  //!
  //! \param[in] context A pointer to a NativeCPUContext object for this
  //!     simulated exception.
  //! \param[in] exception
  //! \param[in] path Path to where the intermediate dump should be written.
  //! \return `true` if the pending intermediate dump could be written.
  bool DumpExceptionFromSimulatedMachExceptionAtPath(
      const NativeCPUContext* context,
      exception_type_t exception,
      const base::FilePath& path);

  //! \brief Moves an intermediate dump to the pending directory. This is meant
  //!     to be used by the UncaughtExceptionHandler, when NSException caught
  //!     by the preprocessor matches the UncaughtExceptionHandler.
  //!
  //! \param[in] path Path to the specific intermediate dump.
  bool MoveIntermediateDumpAtPathToPending(const base::FilePath& path);

  //! \brief Requests that the handler convert all intermediate dumps into
  //!     minidumps and trigger an upload if possible.
  //!
  //! \param[in] annotations Process annotations to set in each crash report.
  //! \param[in] user_stream_sources An optional vector containing the
  //!     extensibility data sources to call on crash. Each time a minidump is
  //!     created, the sources are called in turn. Any streams returned are
  //!     added to the minidump.
  void ProcessIntermediateDumps(
      const std::map<std::string, std::string>& annotations,
      const UserStreamDataSources* user_stream_sources);

  //! \brief Requests that the handler convert a specific intermediate dump into
  //!     a minidump and trigger an upload if possible.
  //!
  //! \param[in] path Path to the specific intermediate dump.
  //! \param[in] annotations Process annotations to set in each crash report.
  //! \param[in] user_stream_sources An optional vector containing the
  //!     extensibility data sources to call on crash. Each time a minidump is
  //!     created, the sources are called in turn. Any streams returned are
  //!     added to the minidump.
  void ProcessIntermediateDump(
      const base::FilePath& path,
      const std::map<std::string, std::string>& annotations = {},
      const UserStreamDataSources* user_stream_sources = {});

  //! \brief Requests that the handler begin in-process uploading of any
  //!     pending reports.
  //!
  //! \param[in] upload_behavior Controls when the upload thread will run and
  //!     process pending reports. By default, only uploads pending reports
  //!     when the application is active.
  void StartProcessingPendingReports(
      UploadBehavior upload_behavior = UploadBehavior::kUploadWhenAppIsActive);

  //! \brief Inject a callback into the Mach exception and signal handling
  //!     mechanisms. Intended to be used by tests to trigger a reentrant
  //      exception.
  void SetExceptionCallbackForTesting(void (*callback)()) {
    exception_callback_for_testing_ = callback;
  }

 private:
  //! \brief Helper to start and end intermediate reports.
  class ScopedReport {
   public:
    ScopedReport(IOSIntermediateDumpWriter* writer,
                 const IOSSystemDataCollector& system_data,
                 const std::map<std::string, std::string>& annotations,
                 const uint64_t* frames = nullptr,
                 const size_t num_frames = 0);
    ~ScopedReport();
    ScopedReport(const ScopedReport&) = delete;
    ScopedReport& operator=(const ScopedReport&) = delete;

   private:
    IOSIntermediateDumpWriter* writer_;
    const uint64_t* frames_;
    const size_t num_frames_;
    IOSIntermediateDumpWriter::ScopedRootMap rootMap_;
  };

  //! \brief Helper to manage closing the intermediate dump writer and unlocking
  //!     the dump file (renaming the file) after the report is written.
  class ScopedLockedWriter {
   public:
    ScopedLockedWriter(IOSIntermediateDumpWriter* writer,
                       const char* writer_path,
                       const char* writer_unlocked_path);

    //! \brief Close the writer_ and rename to the file with path without the
    //!     .locked extension.
    ~ScopedLockedWriter();

    ScopedLockedWriter(const ScopedLockedWriter&) = delete;
    ScopedLockedWriter& operator=(const ScopedLockedWriter&) = delete;

    IOSIntermediateDumpWriter* GetWriter() { return writer_; }

   private:
    const char* writer_path_;
    const char* writer_unlocked_path_;
    IOSIntermediateDumpWriter* writer_;
  };

  //! \brief Manage the prune and upload thread when the active state changes.
  //!
  //! \param[in] active `true` if the application is actively running in the
  //!     foreground, `false` otherwise.
  //! \param[in] upload_behavior Controls when the upload thread will run and
  //!     process pending reports.
  void UpdatePruneAndUploadThreads(bool active, UploadBehavior upload_behavior);

  //! \brief Writes a minidump to the Crashpad database from the
  //!     \a process_snapshot, and triggers the upload_thread_ if started.
  //! \param[in] user_stream_sources An optional vector containing the
  //!     extensibility data sources to call on crash. Each time a minidump is
  //!     created, the sources are called in turn. Any streams returned are
  //!     added to the minidump.
  void SaveSnapshot(ProcessSnapshotIOSIntermediateDump& process_snapshot,
                    const UserStreamDataSources* user_stream_sources = {});

  //! \brief Process a maximum of 20 pending intermediate dumps. Dumps named
  //!     with our bundle id get first priority to prevent spamming.
  std::vector<base::FilePath> PendingFiles();

  //! \brief Lock access to the cached intermediate dump writer from
  //!     concurrent signal, Mach exception and uncaught NSExceptions so that
  //!     the first exception wins. If the same thread triggers another
  //!     reentrant exception, ignore it. If a different thread triggers a
  //!     concurrent exception, sleep indefinitely.
  IOSIntermediateDumpWriter* GetCachedWriter();

  //! \brief Open a new intermediate dump writer from \a writer_path.
  std::unique_ptr<IOSIntermediateDumpWriter> CreateWriterWithPath(
      const base::FilePath& writer_path);

  //! \brief Generates a new file path to be used by an intermediate dump
  //! writer built from base_dir_,, bundle_identifier_and_seperator_, a new
  //! UUID, with a .locked extension.
  const base::FilePath NewLockedFilePath();

  // Intended to be used by tests triggering a reentrant exception. Called
  // in DumpExceptionFromMachException and DumpExceptionFromSignal after
  // acquiring the cached_writer_.
  void (*exception_callback_for_testing_)() = nullptr;

  // Used to synchronize access to UpdatePruneAndUploadThreads().
  base::Lock prune_and_upload_lock_;
  std::atomic_bool upload_thread_enabled_ = false;
  std::map<std::string, std::string> annotations_;
  base::FilePath base_dir_;
  std::string cached_writer_path_;
  std::string cached_writer_unlocked_path_;
  std::unique_ptr<IOSIntermediateDumpWriter> cached_writer_;
  std::atomic<uint64_t> exception_thread_id_ = 0;
  std::unique_ptr<CrashReportUploadThread> upload_thread_;
  std::unique_ptr<PruneIntermediateDumpsAndCrashReportsThread> prune_thread_;
  std::unique_ptr<CrashReportDatabase> database_;
  std::string bundle_identifier_and_seperator_;
  IOSSystemDataCollector system_data_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_CLIENT_IOS_HANDLER_IN_PROCESS_IN_PROCESS_HANDLER_H_
