// Copyright 2019 The Crashpad Authors
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

#ifndef CRASHPAD_HANDLER_LINUX_CROS_CRASH_REPORT_EXCEPTION_HANDLER_H_
#define CRASHPAD_HANDLER_LINUX_CROS_CRASH_REPORT_EXCEPTION_HANDLER_H_

#include <sys/types.h>

#include <map>
#include <string>

#include "client/crash_report_database.h"
#include "handler/linux/exception_handler_server.h"
#include "handler/user_stream_data_source.h"
#include "util/linux/exception_handler_protocol.h"
#include "util/linux/ptrace_connection.h"
#include "util/misc/address_types.h"
#include "util/misc/uuid.h"

namespace crashpad {

//! \brief An exception handler that writes crash reports to the ChromeOS
//!     crash_reporter.
class CrosCrashReportExceptionHandler
    : public ExceptionHandlerServer::Delegate {
 public:
  //! \brief Creates a new object that will pass reports to
  //!     `/sbin/crash_reporter`.
  //!
  //! \param[in] database The database that supplies settings for this client.
  //!     This object does not write its reports to this database.
  //! \param[in] process_annotations A map of annotations to insert as
  //!     process-level annotations into each crash report that is written. Do
  //!     not confuse this with module-level annotations, which are under the
  //!     control of the crashing process, and are used to implement Chrome’s
  //!     “crash keys.” Process-level annotations are those that are beyond the
  //!     control of the crashing process, which must reliably be set even if
  //!     the process crashes before it’s able to establish its own annotations.
  //!     To interoperate with Breakpad servers, the recommended practice is to
  //!     specify values for the `"prod"` and `"ver"` keys as process
  //!     annotations.
  //! \param[in] user_stream_data_sources Data sources to be used to extend
  //!     crash reports. For each crash report that is written, the data sources
  //!     are called in turn. These data sources may contribute additional
  //!     minidump streams. `nullptr` if not required.
  CrosCrashReportExceptionHandler(
      CrashReportDatabase* database,
      const std::map<std::string, std::string>* process_annotations,
      const UserStreamDataSources* user_stream_data_sources);

  CrosCrashReportExceptionHandler(const CrosCrashReportExceptionHandler&) =
      delete;
  CrosCrashReportExceptionHandler& operator=(
      const CrosCrashReportExceptionHandler&) = delete;

  ~CrosCrashReportExceptionHandler() override;

  // ExceptionHandlerServer::Delegate:

  bool HandleException(pid_t client_process_id,
                       uid_t client_uid,
                       const ExceptionHandlerProtocol::ClientInformation& info,
                       VMAddress requesting_thread_stack_address = 0,
                       pid_t* requesting_thread_id = nullptr,
                       UUID* local_report_id = nullptr) override;

  bool HandleExceptionWithBroker(
      pid_t client_process_id,
      uid_t client_uid,
      const ExceptionHandlerProtocol::ClientInformation& info,
      int broker_sock,
      UUID* local_report_id = nullptr) override;

  void SetDumpDir(const base::FilePath& dump_dir) { dump_dir_ = dump_dir; }
  void SetAlwaysAllowFeedback() { always_allow_feedback_ = true; }
 private:
  bool HandleExceptionWithConnection(
      PtraceConnection* connection,
      const ExceptionHandlerProtocol::ClientInformation& info,
      uid_t client_uid,
      VMAddress requesting_thread_stack_address,
      pid_t* requesting_thread_id,
      UUID* local_report_id = nullptr);

  CrashReportDatabase* database_;  // weak
  const std::map<std::string, std::string>* process_annotations_;  // weak
  const UserStreamDataSources* user_stream_data_sources_;  // weak
  base::FilePath dump_dir_;
  bool always_allow_feedback_;
};

}  // namespace crashpad

#endif  // CRASHPAD_HANDLER_LINUX_CROS_CRASH_REPORT_EXCEPTION_HANDLER_H_
