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

#ifndef CRASHPAD_UTIL_MACH_BOOTSTRAP_H_
#define CRASHPAD_UTIL_MACH_BOOTSTRAP_H_

#include <string>

#include "base/apple/scoped_mach_port.h"

namespace crashpad {

//! \brief Makes a `boostrap_check_in()` call to the process’ bootstrap server.
//!
//! This function is provided to make it easier to call `bootstrap_check_in()`
//! while avoiding accidental leaks of the returned receive right.
//!
//! \param[in] service_name The service name to check in.
//!
//! \return On success, a receive right to the service port. On failure,
//!     `MACH_PORT_NULL` with a message logged.
base::apple::ScopedMachReceiveRight BootstrapCheckIn(
    const std::string& service_name);

//! \brief Makes a `boostrap_look_up()` call to the process’ bootstrap server.
//!
//! This function is provided to make it easier to call `bootstrap_look_up()`
//! while avoiding accidental leaks of the returned send right.
//!
//! \param[in] service_name The service name to look up.
//!
//! \return On success, a send right to the service port. On failure,
//!     `MACH_PORT_NULL` with a message logged.
base::apple::ScopedMachSendRight BootstrapLookUp(
    const std::string& service_name);

//! \brief Obtains the system’s default Mach exception handler for crash-type
//!     exceptions.
//!
//! This is obtained by looking up `"com.apple.ReportCrash"` with the bootstrap
//! server. The service name comes from the first launch agent loaded by
//! `launchd` with a `MachServices` entry having `ExceptionServer` set. This
//! launch agent is normally loaded from
//! `/System/Library/LaunchAgents/com.apple.ReportCrash.plist`.
//!
//! \return On success, a send right to an `exception_handler_t` corresponding
//!     to the system’s default crash reporter. On failure, `MACH_PORT_NULL`,
//!     with a message logged.
base::apple::ScopedMachSendRight SystemCrashReporterHandler();

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_MACH_BOOTSTRAP_H_
