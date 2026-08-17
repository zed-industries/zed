// Copyright 2021 The Crashpad Authors
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

#ifndef CRASHPAD_CLIENT_SIMULATE_CRASH_IOS_H_
#define CRASHPAD_CLIENT_SIMULATE_CRASH_IOS_H_

#include "client/crashpad_client.h"
#include "util/misc/capture_context.h"

//! \file

//! \brief Captures the CPU context and creates a minidump dump without an
//!     exception. The minidump will immediately become eligible for further
//!     processing, including upload.
//!
//! \sa CRASHPAD_SIMULATE_CRASH_AND_DEFER_PROCESSING
#define CRASHPAD_SIMULATE_CRASH()                             \
  do {                                                        \
    crashpad::NativeCPUContext cpu_context;                   \
    crashpad::CaptureContext(&cpu_context);                   \
    crashpad::CrashpadClient::DumpWithoutCrash(&cpu_context); \
  } while (false)

//! \brief Captures the CPU context and captures an intermediate dump without an
//!     exception. Does not convert the intermediate dump into a minidump.
//!
//! Deferring processing is useful when the application may be in an unstable
//! state, such as during a hang.
//!
//! \sa CRASHPAD_SIMULATE_CRASH
#define CRASHPAD_SIMULATE_CRASH_AND_DEFER_PROCESSING()            \
  do {                                                            \
    crashpad::NativeCPUContext cpu_context;                       \
    crashpad::CaptureContext(&cpu_context);                       \
    crashpad::CrashpadClient::DumpWithoutCrashAndDeferProcessing( \
        &cpu_context);                                            \
  } while (false)

#define CRASHPAD_SIMULATE_CRASH_AND_DEFER_PROCESSING_AT_PATH(path)      \
  do {                                                                  \
    crashpad::NativeCPUContext cpu_context;                             \
    crashpad::CaptureContext(&cpu_context);                             \
    crashpad::CrashpadClient::DumpWithoutCrashAndDeferProcessingAtPath( \
        &cpu_context, path);                                            \
  } while (false)

#endif  // CRASHPAD_CLIENT_SIMULATE_CRASH_IOS_H_
