// Copyright 2014 The Crashpad Authors
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

#ifndef CRASHPAD_TEST_MAC_MACH_ERRORS_H_
#define CRASHPAD_TEST_MAC_MACH_ERRORS_H_

#include <mach/mach.h>

#include <string>

namespace crashpad {
namespace test {

// This function formats messages in a similar way to the Mach error logging
// macros in base/apple/mach_logging.h. It exists to interoperate with Google
// Test assertions, which don’t interoperate with logging but can be streamed
// to.
//
// Where non-test code could do:
//   MACH_CHECK(kr == KERN_SUCCESS, kr) << "vm_deallocate";
// Google Test-based test code can do:
//   EXPECT_EQ(kr, KERN_SUCCESS) << MachErrorMessage(kr, "vm_deallocate");

//! \brief Formats a Mach error message.
//!
//! The returned string will combine the \a base string, if supplied, with a
//! textual and numeric description of the error.
//!
//! \param[in] mach_err The Mach error code, which may be a `kern_return_t` or
//!     related type.
//! \param[in] base A string to prepend to the error description.
//!
//! \return A string of the format `"(os/kern) invalid address (1)"` if \a
//!     mach_err has the value `KERN_INVALID_ADDRESS` on a system where this is
//!     defined to be 1. If \a base is not empty, it will be prepended to this
//!     string, separated by a colon.
std::string MachErrorMessage(mach_error_t mach_err,
                             const std::string& base = std::string());

}  // namespace test
}  // namespace crashpad

#endif  // CRASHPAD_TEST_MAC_MACH_ERRORS_H_
