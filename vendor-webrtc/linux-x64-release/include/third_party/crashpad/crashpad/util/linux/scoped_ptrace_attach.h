// Copyright 2017 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_LINUX_SCOPED_PTRACE_ATTACH_H_
#define CRASHPAD_UTIL_LINUX_SCOPED_PTRACE_ATTACH_H_

#include <sys/types.h>

namespace crashpad {

//! \brief Attaches to the process with process ID \a pid and blocks until the
//!     target process has stopped by calling `waitpid()`.
//!
//! \param pid The process ID of the process to attach to.
//! \param can_log Whether this function may log messages on failure.
//! \return `true` on success. `false` on failure with a message logged if \a
//!     can_log is `true`.
bool PtraceAttach(pid_t pid, bool can_log = true);

//! \brief Detaches the process  with process ID \a pid. The process must
//!     already be ptrace attached.
//!
//! \param pid The process ID of the process to detach.
//! \param can_log Whether this function may log messages on failure.
//! \return `true` on success. `false` on failure with a message logged if \a
//!     can_log is `true`.
bool PtraceDetach(pid_t pid, bool can_log = true);

//! \brief Maintains a `ptrace()` attachment to a process.
//!
//! On destruction, the process will be detached.
class ScopedPtraceAttach {
 public:
  ScopedPtraceAttach();

  ScopedPtraceAttach(const ScopedPtraceAttach&) = delete;
  ScopedPtraceAttach& operator=(const ScopedPtraceAttach&) = delete;

  ~ScopedPtraceAttach();

  //! \brief Detaches from the process by calling `ptrace()`.
  //!
  //! \return `true` on success. `false` on failure, with a message logged.
  bool Reset();

  //! \brief Detaches from any previously attached process, attaches to the
  //!      process with process ID \a pid, and blocks until the target process
  //!      has stopped by calling `waitpid()`.
  //!
  //! \return `true` on success. `false` on failure, with a message logged.
  bool ResetAttach(pid_t pid);

 private:
  pid_t pid_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_LINUX_SCOPED_PTRACE_ATTACH_H_
