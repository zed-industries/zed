/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_SYS_TYPES_H_
#define INCLUDE_PERFETTO_EXT_BASE_SYS_TYPES_H_

// This headers deals with sys types commonly used in the codebase that are
// missing on Windows.

#include <sys/types.h>  // IWYU pragma: export
#include <cstdint>

#include "perfetto/base/build_config.h"

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)

#if !PERFETTO_BUILDFLAG(PERFETTO_COMPILER_GCC)
// MinGW has these. clang-cl and MSVC, which use just the Windows SDK, don't.
using uid_t = int;
using pid_t = int;
#endif  // !GCC

#if defined(_WIN64)
using ssize_t = int64_t;
#else
using ssize_t = long;
#endif  // _WIN64

#endif  // OS_WIN

#if PERFETTO_BUILDFLAG(PERFETTO_OS_ANDROID) && !defined(AID_SHELL)
// From libcutils' android_filesystem_config.h .
#define AID_SHELL 2000
#endif

namespace perfetto {
namespace base {

// The machine ID used in the tracing core.
using MachineID = uint32_t;
// The default value reserved for the host trace.
constexpr MachineID kDefaultMachineID = 0;

constexpr uid_t kInvalidUid = static_cast<uid_t>(-1);
constexpr pid_t kInvalidPid = static_cast<pid_t>(-1);

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_SYS_TYPES_H_
