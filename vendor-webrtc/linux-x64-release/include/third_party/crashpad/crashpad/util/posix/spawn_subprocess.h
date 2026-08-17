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

#ifndef CRASHPAD_UTIL_POSIX_SPAWN_SUBPROCESS_H_
#define CRASHPAD_UTIL_POSIX_SPAWN_SUBPROCESS_H_

#include <string>
#include <vector>

namespace crashpad {

//! \brief Spawns a subprocess.
//!
//! A grandchild process will be started through the
//! `fork()`-and-`posix_spawn()` pattern where supported, and
//! double-`fork()`-and-`execv()` pattern elsewhere. This allows the grandchild
//! to fully disassociate from the parent. The grandchild will not be a member
//! of the parent’s process group or session and will not have a controlling
//! terminal, providing isolation from signals not intended for it. The
//! grandchild’s parent process, in terms of the process tree hierarchy, will be
//! the process with process ID 1, relieving any other process of the
//! responsibility to reap it via `waitpid()`. Aside from the three file
//! descriptors associated with the standard input/output streams and any file
//! descriptor passed in \a preserve_fd, the grandchild will not inherit any
//! file descriptors from the parent process.
//!
//! \param[in] argv The argument vector to start the grandchild process with.
//!     `argv[0]` is used as the path to the executable.
//! \param[in] envp A vector of environment variables of the form `var=value` to
//!     be passed to the spawned process. If this value is `nullptr`, the
//!     current environment is used.
//! \param[in] preserve_fd A file descriptor to be inherited by the grandchild
//!     process. This file descriptor is inherited in addition to the three file
//!     descriptors associated with the standard input/output streams. Use `-1`
//!     if no additional file descriptors are to be inherited.
//! \param[in] use_path Whether to consult the `PATH` environment variable when
//!     requested to start an executable at a non-absolute path.
//! \param[in] child_function If not `nullptr`, this function will be called in
//!     the intermediate child process. Take note that this function will run in
//!     the context of a forked process, and must be safe for that purpose.
//!
//! \return `true` on success, and `false` on failure with a message logged.
//!     Only failures that occur in the parent process that indicate a definite
//!     failure to start the the grandchild are reported in the return value.
//!     Failures in the intermediate child or grandchild processes cannot be
//!     reported in the return value, and are addressed by logging a message and
//!     terminating. The caller assumes the responsibility for detecting such
//!     failures, for example, by observing a failure to perform a successful
//!     handshake with the grandchild process.
bool SpawnSubprocess(const std::vector<std::string>& argv,
                     const std::vector<std::string>* envp,
                     int preserve_fd,
                     bool use_path,
                     void (*child_function)());

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_POSIX_SPAWN_SUBPROCESS_H_
