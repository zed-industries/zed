// Copyright 2022 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_WIN_EXCEPTION_CODES_H_
#define CRASHPAD_UTIL_WIN_EXCEPTION_CODES_H_

#include <stdint.h>

namespace crashpad {

//! \brief Crashpad-specific exception codes that are used as arguments to
//!     `RaiseException()` in unusual circumstances.
enum ExceptionCodes : uint32_t {
  //! \brief The exception code (roughly "Client called") used when
  //!     DumpAndCrashTargetProcess() triggers an exception in a target
  //!     process.
  //!
  //! \note This value does not have any bits of the top nibble set, to avoid
  //!     confusion with real exception codes which tend to have those bits
  //!     set.
  kTriggeredExceptionCode = 0xcca11ed,
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_WIN_EXCEPTION_CODES_H_
