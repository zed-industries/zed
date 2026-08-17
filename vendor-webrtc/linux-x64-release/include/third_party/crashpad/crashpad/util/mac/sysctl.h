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

#ifndef CRASHPAD_UTIL_MAC_SYSCTL_H_
#define CRASHPAD_UTIL_MAC_SYSCTL_H_

#include <string>

namespace crashpad {

//! \brief Calls `sysctlbyname` to read a string.
//!
//! \param[in] name The string name of the sysctl to raed.
//! \param[in] may_log_enoent If `true`, allows a warning to be logged if the
//!     sysctl is not found, indicated by `sysctlbyname` setting `errno` to
//!     `ENOENT`. If `false`, no warning will be logged if the sysctl is
//!     missing, and an empty string will be returned silently.
//!
//! \return The value of the sysctl read on success. On failure, an empty string
//!     with a warning logged, subject to \a may_log_enoent.
std::string ReadStringSysctlByName(const char* name, bool may_log_enoent);

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_MAC_SYSCTL_H_
