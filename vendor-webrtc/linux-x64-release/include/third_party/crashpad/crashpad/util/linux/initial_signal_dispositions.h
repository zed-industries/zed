// Copyright 2020 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_LINUX_INITIAL_SIGNAL_DISPOSITIONS_H
#define CRASHPAD_UTIL_LINUX_INITIAL_SIGNAL_DISPOSITIONS_H

namespace crashpad {

//! \brief Establishes signal dispositions for a process based on the platform.
//!
//! Default signal dispositions are normally configured by the kernel, but
//! additional signal handlers might be installed by dependent or preloaded
//! libraries, e.g. Bionic normally installs signal handlers which log stack
//! traces to Android's logcat.
//!
//! This function initializes signal dispositions when the default dispositions
//! provided by the platform are broken. This function must be called before any
//! application level signal handlers have been installed and should be called
//! early in the process lifetime to reduce the chance of any broken signal
//! handlers being triggered.
//!
//! When running on Android M (API 23), this function installs `SIG_DFL` for
//! signals: `SIGABRT`, `SIGFPE`, `SIGPIPE`, `SIGSTKFLT`, and `SIGTRAP`.
//!
//! \return `true` on success. Otherwise `false` with a message logged.
bool InitializeSignalDispositions();

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_LINUX_INITIAL_SIGNAL_DISPOSITIONS_H
