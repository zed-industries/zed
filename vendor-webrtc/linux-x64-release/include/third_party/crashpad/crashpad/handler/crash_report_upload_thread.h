// Copyright 2015 The Crashpad Authors
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

#ifndef CRASHPAD_HANDLER_CRASH_REPORT_UPLOAD_THREAD_H_
#define CRASHPAD_HANDLER_CRASH_REPORT_UPLOAD_THREAD_H_

#include <functional>
#include <memory>
#include <string>
#include <unordered_map>

#include "build/build_config.h"
#include "client/crash_report_database.h"
#include "util/misc/uuid.h"
#include "util/stdlib/thread_safe_vector.h"
#include "util/thread/stoppable.h"
#include "util/thread/worker_thread.h"

namespace crashpad {

//! \brief A thread that processes pending crash reports in a
//!     CrashReportDatabase by uploading them or marking them as completed
//!     without upload, as desired.
//!
//! A producer of crash reports should notify an object of this class that a new
//! report has been added to the database by calling ReportPending().
//!
//! Independently of being triggered by ReportPending(), objects of this class
//! can periodically examine the database for pending reports. This allows
//! failed upload attempts for reports left in the pending state to be retried.
//! It also catches reports that are added without a ReportPending() signal
//! being caught. This may happen if crash reports are added to the database by
//! other processes.
class CrashReportUploadThread : public WorkerThread::Delegate,
                                public Stoppable {
 public:
   //! \brief Options to be passed to the CrashReportUploadThread constructor.
   struct Options {
    //! Whether client identifying parameters like product name or version
    //! should be added to the URL.
    bool identify_client_via_url;

    //! Whether uploads should be throttled to a (currently hardcoded) rate.
    bool rate_limit;

    //! Whether uploads should use `gzip` compression.
    bool upload_gzip;

    //! Whether to periodically check for new pending reports not already known
    //! to exist. When `false`, only an initial upload attempt will be made for
    //! reports known to exist by having been added by the ReportPending()
    //! method. No scans for new pending reports will be conducted.
    bool watch_pending_reports;
  };

  //! \brief Observation callback invoked each time the in-process handler
  //!     finishes processing and attempting to upload on-disk crash reports
  //!     (whether or not the uploads succeeded).
  //!
  //! This callback is copied into this object. Any references or pointers
  //! inside must outlive this object.
  //!
  //! The callback might be invoked on a background thread, so clients must
  //! synchronize appropriately.
  using ProcessPendingReportsObservationCallback = std::function<void()>;

  //! \brief Constructs a new object.
  //!
  //! \param[in] database The database to upload crash reports from.
  //! \param[in] url The URL of the server to upload crash reports to.
  //! \param[in] options Options for the report uploads.
  //! \param[in] callback Optional callback invoked zero or more times
  //!     on a background thread each time the this object finishes
  //!     processing and attempting to upload on-disk crash reports.
  //!     If this callback is empty, it is not invoked.
  CrashReportUploadThread(CrashReportDatabase* database,
                          const std::string& url,
                          const Options& options,
                          ProcessPendingReportsObservationCallback callback);

  CrashReportUploadThread(const CrashReportUploadThread&) = delete;
  CrashReportUploadThread& operator=(const CrashReportUploadThread&) = delete;

  ~CrashReportUploadThread();

  //! \brief Informs the upload thread that a new pending report has been added
  //!     to the database.
  //!
  //! \param[in] report_uuid The unique identifier of the newly added pending
  //!     report.
  //!
  //! This method may be called from any thread.
  void ReportPending(const UUID& report_uuid);

  // Stoppable:

  //! \brief Starts a dedicated upload thread, which executes ThreadMain().
  //!
  //! This method may only be be called on a newly-constructed object or after
  //! a call to Stop().
  void Start() override;

  //! \brief Stops the upload thread.
  //!
  //! The upload thread will terminate after completing whatever task it is
  //! performing. If it is not performing any task, it will terminate
  //! immediately. This method blocks while waiting for the upload thread to
  //! terminate.
  //!
  //! This method must only be called after Start(). If Start() has been called,
  //! this method must be called before destroying an object of this class.
  //!
  //! This method may be called from any thread other than the upload thread.
  //! It is expected to only be called from the same thread that called Start().
  void Stop() override;

  //! \return `true` if the thread is running, `false` if it is not.
  bool is_running() const { return thread_.is_running(); }

 private:
  //! \brief The result code from UploadReport().
  enum class UploadResult {
    //! \brief The crash report was uploaded successfully.
    kSuccess,

    //! \brief The crash report upload failed in such a way that recovery is
    //!     impossible.
    //!
    //! No further upload attempts should be made for the report.
    kPermanentFailure,

    //! \brief The crash report upload failed, but it might succeed again if
    //!     retried in the future.
    //!
    //! If the report has not already been retried too many times, the caller
    //! may arrange to call UploadReport() for the report again in the future,
    //! after a suitable delay.
    kRetry,
  };

  //! \brief Calls ProcessPendingReport() on pending reports.
  //!
  //! Assuming Stop() has not been called, this will process reports that the
  //! object has been made aware of in ReportPending(). Additionally, if the
  //! object was constructed with \a watch_pending_reports, it will also scan
  //! the crash report database for other pending reports, and process those as
  //! well.
  void ProcessPendingReports();

  //! \brief Processes a single pending report from the database.
  //!
  //! \param[in] report The crash report to process.
  //!
  //! If report upload is enabled, this method attempts to upload \a report by
  //! calling UplaodReport(). If the upload is successful, the report will be
  //! marked as “completed” in the database. If the upload fails and more
  //! retries are desired, the report’s upload-attempt count and
  //! last-upload-attempt time will be updated in the database and it will
  //! remain in the “pending” state. If the upload fails and no more retries are
  //! desired, or report upload is disabled, it will be marked as “completed” in
  //! the database without ever having been uploaded.
  void ProcessPendingReport(const CrashReportDatabase::Report& report);

  //! \brief Attempts to upload a crash report.
  //!
  //! \param[in] report The report to upload. The caller is responsible for
  //!     calling CrashReportDatabase::GetReportForUploading() before calling
  //!     this method, and for calling
  //!     CrashReportDatabase::RecordUploadComplete() after calling this method.
  //! \param[out] response_body If the upload attempt is successful, this will
  //!     be set to the response body sent by the server. Breakpad-type servers
  //!     provide the crash ID assigned by the server in the response body.
  //!
  //! \return A member of UploadResult indicating the result of the upload
  //!    attempt.
  UploadResult UploadReport(const CrashReportDatabase::UploadReport* report,
                            std::string* response_body);

  // WorkerThread::Delegate:
  //! \brief Calls ProcessPendingReports() in response to ReportPending() having
  //!     been called on any thread, as well as periodically on a timer.
  void DoWork(const WorkerThread* thread) override;

  //! \brief Rate-limit uploads.
  //!
  //! \param[in] report The crash report to process.
  //!
  //! This currently implements very simplistic rate-limiting, compatible with
  //! the Breakpad client, where the strategy is to permit one upload attempt
  //! per hour, and retire reports that would exceed this limit or for which the
  //! upload fails on the first attempt.
  //! If upload was requested explicitly (i.e. by user action), do not throttle
  //! the upload.
  //!
  //! TODO(mark): Provide a proper rate-limiting strategy and allow for failed
  //! upload attempts to be retried.
  bool ShouldRateLimitUpload(const CrashReportDatabase::Report& report);

#if BUILDFLAG(IS_IOS)
  //! \brief Rate-limit report retries.
  //!
  //! \param[in] report The crash report to process.
  //!
  //! This implements a per-report retry rate limit (as opposed to per upload
  //! rate limit in ShouldRateLimitUpload). When a report upload ends in a retry
  //! state, an in-memory only timestamp is stored in |retry_uuid_time_map_|
  //! with the next possible retry time. This timestamp is a backoff from the
  //! main thread work interval, doubling on each attemt. Because this is only
  //! stored in memory, on restart reports in the retry state will always be
  //! tried once, and then fall back into the next backoff. This continues until
  //! kRetryAttempts is reached.
  bool ShouldRateLimitRetry(const CrashReportDatabase::Report& report);
#endif

  const Options options_;
  const ProcessPendingReportsObservationCallback callback_;
  const std::string url_;
  WorkerThread thread_;
  ThreadSafeVector<UUID> known_pending_report_uuids_;
#if BUILDFLAG(IS_IOS)
  // This is not thread-safe, and only used by the worker thread.
  std::map<UUID, time_t> retry_uuid_time_map_;
#endif
  CrashReportDatabase* database_;  // weak
};

}  // namespace crashpad

#endif  // CRASHPAD_HANDLER_CRASH_REPORT_UPLOAD_THREAD_H_
