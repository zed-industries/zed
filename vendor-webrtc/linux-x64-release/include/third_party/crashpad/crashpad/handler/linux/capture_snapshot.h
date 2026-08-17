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

#ifndef CRASHPAD_HANDLER_LINUX_CAPTURE_SNAPSHOT_H_
#define CRASHPAD_HANDLER_LINUX_CAPTURE_SNAPSHOT_H_

#include <sys/types.h>

#include <map>
#include <memory>
#include <string>

#include "snapshot/linux/process_snapshot_linux.h"
#include "snapshot/sanitized/process_snapshot_sanitized.h"
#include "util/linux/exception_handler_protocol.h"
#include "util/linux/ptrace_connection.h"
#include "util/misc/address_types.h"

namespace crashpad {

//! \brief Captures a snapshot of a client over \a connection.
//!
//! \param[in] connection A PtraceConnection to the client to snapshot.
//! \param[in] info Information about the client configuring the snapshot.
//! \param[in] process_annotations A map of annotations to insert as
//!     process-level annotations into the snapshot.
//! \param[in] client_uid The client's user ID.
//! \param[in] requesting_thread_stack_address An address on the stack of the
//!     thread requesting the snapshot. If \a info includes an exception
//!     address, the exception will be assigned to the thread whose stack
//!     address range contains this address. If 0, \a requesting_thread_id will
//!     be -1.
//! \param[out] requesting_thread_id The thread ID of the thread corresponding
//!     to \a requesting_thread_stack_address. Set to -1 if the thread ID could
//!     not be determined. Optional.
//! \param[out] process_snapshot A snapshot of the client process, valid if this
//!     function returns `true`.
//! \param[out] sanitized_snapshot A sanitized snapshot of the client process,
//!     valid if this function returns `true` and sanitization was requested in
//!     \a info.
//! \return `true` if \a process_snapshot was successfully created. A message
//!     will be logged on failure, but not if the snapshot was skipped because
//!     handling was disabled by CrashpadInfoClientOptions.
bool CaptureSnapshot(
    PtraceConnection* connection,
    const ExceptionHandlerProtocol::ClientInformation& info,
    const std::map<std::string, std::string>& process_annotations,
    uid_t client_uid,
    VMAddress requesting_thread_stack_address,
    pid_t* requesting_thread_id,
    std::unique_ptr<ProcessSnapshotLinux>* process_snapshot,
    std::unique_ptr<ProcessSnapshotSanitized>* sanitized_snapshot);

}  // namespace crashpad

#endif  // CRASHPAD_HANDLER_LINUX_CAPTURE_SNAPSHOT_H_
