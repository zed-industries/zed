/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_BASE_PLATFORM_HANDLE_H_
#define INCLUDE_PERFETTO_BASE_PLATFORM_HANDLE_H_

#include <stdint.h>

#include "perfetto/base/build_config.h"

namespace perfetto {
namespace base {

// PlatformHandle should be used only for types that are HANDLE(s) in Windows.
// It should NOT be used to blanket-replace "int fd" in the codebase.
// Windows has two types of "handles", which, in UNIX-land, both map to int:
// 1. File handles returned by the posix-compatibility API like _open().
//    These are just int(s) and should stay such, because all the posix-like API
//    in Windows.h take an int, not a HANDLE.
// 2. Handles returned by old-school WINAPI like CreateFile, CreateEvent etc.
//    These are proper HANDLE(s). PlatformHandle should be used here.
//
// On Windows, sockets have their own type (SOCKET) which is neither a HANDLE
// nor an int. However Windows SOCKET(s) can have an event HANDLE attached
// to them (which in Perfetto is a PlatformHandle), and that can be used in
// WaitForMultipleObjects, hence in base::TaskRunner.AddFileDescriptorWatch().
// On POSIX OSes, a SocketHandle is really just an int (a file descriptor).
#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
// Windows.h typedefs HANDLE to void*, and SOCKET to uintptr_t. We use their
// types to avoid leaking Windows.h through our headers.
using PlatformHandle = void*;
using SocketHandle = uintptr_t;

// On Windows both nullptr and 0xffff... (INVALID_HANDLE_VALUE) are invalid.
struct PlatformHandleChecker {
  static inline bool IsValid(PlatformHandle h) {
    return h && h != reinterpret_cast<PlatformHandle>(-1);
  }
};
#else
using PlatformHandle = int;
using SocketHandle = int;
struct PlatformHandleChecker {
  static inline bool IsValid(PlatformHandle h) { return h >= 0; }
};
#endif

// The definition of this lives in base/file_utils.cc (to avoid creating an
// extra build edge for a one liner). This is really an alias for close() (UNIX)
// CloseHandle() (Windows). THe indirection layer is just to avoid leaking
// system headers like Windows.h through perfetto headers.
// Thre return value is always UNIX-style: 0 on success, -1 on failure.
int ClosePlatformHandle(PlatformHandle);

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_BASE_PLATFORM_HANDLE_H_
