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

#ifndef CRASHPAD_UTIL_LINUX_PTRACE_CONNECTION_H_
#define CRASHPAD_UTIL_LINUX_PTRACE_CONNECTION_H_

#include <sys/types.h>

#include <string>
#include <vector>

#include "base/files/file_path.h"
#include "util/linux/thread_info.h"
#include "util/process/process_memory_linux.h"

namespace crashpad {

//! \brief Provides an interface for making `ptrace` requests against a process
//!     and its threads.
class PtraceConnection {
 public:
  virtual ~PtraceConnection() {}

  //! \brief Returns the process ID of the connected process.
  virtual pid_t GetProcessID() = 0;

  //! \brief Adds a new thread to this connection.
  //!
  //! \param[in] tid The thread ID of the thread to attach.
  //! \return `true` on success. `false` on failure with a message logged.
  virtual bool Attach(pid_t tid) = 0;

  //! \brief Returns `true` if connected to a 64-bit process.
  virtual bool Is64Bit() = 0;

  //! \brief Retrieves a ThreadInfo for a target thread.
  //!
  //! \param[in] tid The thread ID of the target thread.
  //! \param[out] info Information about the thread.
  //! \return `true` on success. `false` on failure with a message logged.
  virtual bool GetThreadInfo(pid_t tid, ThreadInfo* info) = 0;

  //! \brief Reads the entire contents of a file.
  //!
  //! \param[in] path The path of the file to read.
  //! \param[out] contents The file contents, valid if this method returns
  //!     `true`.
  //! \return `true` on success. `false` on failure with a message logged.
  virtual bool ReadFileContents(const base::FilePath& path,
                                std::string* contents) = 0;

  //! \brief Returns a memory reader for the connected process.
  //!
  //! The caller does not take ownership of the reader. The reader is valid for
  //! the lifetime of the PtraceConnection that created it.
  virtual ProcessMemoryLinux* Memory() = 0;

  //! \brief Determines the thread IDs of the threads in the connected process.
  //!
  //! \param[out] threads The list of thread IDs.
  //! \return `true` on success, `false` on failure with a message logged. If
  //!     this method returns `false`, \a threads may contain a partial list of
  //!     thread IDs.
  virtual bool Threads(std::vector<pid_t>* threads) = 0;

  //! \brief Copies memory from the connected process into a caller-provided
  //!     buffer in the current process, up to a maximum number of bytes.
  //!
  //! \param[in] address The address, in the connected process' address space,
  //!     of the memory region to copy.
  //! \param[in] size The maximum size, in bytes, of the memory region to copy.
  //!     \a buffer must be at least this size.
  //! \param[out] buffer The buffer into which the contents of the other
  //!     process' memory will be copied.
  //!
  //! \return the number of bytes copied, 0 if there is no more data to read, or
  //!     -1 on failure with a message logged.
  virtual ssize_t ReadUpTo(VMAddress address, size_t size, void* buffer) = 0;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_LINUX_PTRACE_CONNECTION_H_
