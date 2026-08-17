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

#ifndef CRASHPAD_UTIL_LINUX_PROC_TASK_READER_H_
#define CRASHPAD_UTIL_LINUX_PROC_TASK_READER_H_

#include <sys/types.h>

#include <vector>

namespace crashpad {

//! \brief Enumerates the thread IDs of a process by reading
//!     <code>/proc/<i>pid</i>/task</code>.
//!
//! \param[in] pid The process ID for which to read thread IDs.
//! \param[out] tids The read thread IDs.
//! \return `true` if the task directory was successfully read. Format errors
//!     are logged, but won't cause this function to return `false`.
bool ReadThreadIDs(pid_t pid, std::vector<pid_t>* tids);

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_LINUX_PROC_TASK_READER_H_
