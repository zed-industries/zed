/*
 * Copyright (C) 2017 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_BASE_SCOPED_FILE_H_
#define INCLUDE_PERFETTO_EXT_BASE_SCOPED_FILE_H_

#include "perfetto/base/build_config.h"

#include <stdio.h>

#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
#include <dirent.h>  // For DIR* / opendir().
#endif

#include <string>

#include "perfetto/base/export.h"
#include "perfetto/base/logging.h"
#include "perfetto/base/platform_handle.h"

namespace perfetto {
namespace base {

namespace internal {
// Used for the most common cases of ScopedResource where there is only one
// invalid value.
template <typename T, T InvalidValue>
struct DefaultValidityChecker {
  static bool IsValid(T t) { return t != InvalidValue; }
};
}  // namespace internal

// RAII classes for auto-releasing fds and dirs.
// if T is a pointer type, InvalidValue must be nullptr. Doing otherwise
// causes weird unexpected behaviors (See https://godbolt.org/z/5nGMW4).
template <typename T,
          int (*CloseFunction)(T),
          T InvalidValue,
          bool CheckClose = true,
          class Checker = internal::DefaultValidityChecker<T, InvalidValue>>
class ScopedResource {
 public:
  using ValidityChecker = Checker;
  static constexpr T kInvalid = InvalidValue;

  explicit ScopedResource(T t = InvalidValue) : t_(t) {}
  ScopedResource(ScopedResource&& other) noexcept {
    t_ = other.t_;
    other.t_ = InvalidValue;
  }
  ScopedResource& operator=(ScopedResource&& other) {
    reset(other.t_);
    other.t_ = InvalidValue;
    return *this;
  }
  T get() const { return t_; }
  T operator*() const { return t_; }
  explicit operator bool() const { return Checker::IsValid(t_); }
  void reset(T r = InvalidValue) {
    if (Checker::IsValid(t_)) {
      int res = CloseFunction(t_);
      if (CheckClose)
        PERFETTO_CHECK(res == 0);
    }
    t_ = r;
  }
  T release() {
    T t = t_;
    t_ = InvalidValue;
    return t;
  }
  ~ScopedResource() { reset(InvalidValue); }

 private:
  ScopedResource(const ScopedResource&) = delete;
  ScopedResource& operator=(const ScopedResource&) = delete;
  T t_;
};

// Declared in file_utils.h. Forward declared to avoid #include cycles.
int PERFETTO_EXPORT_COMPONENT CloseFile(int fd);

// Use this for file resources obtained via open() and similar APIs.
using ScopedFile = ScopedResource<int, CloseFile, -1>;
using ScopedFstream = ScopedResource<FILE*, fclose, nullptr>;

// Use this for resources that are HANDLE on Windows. See comments in
// platform_handle.h
#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
using ScopedPlatformHandle = ScopedResource<PlatformHandle,
                                            ClosePlatformHandle,
                                            /*InvalidValue=*/nullptr,
                                            /*CheckClose=*/true,
                                            PlatformHandleChecker>;
#else
// On non-windows systems we alias ScopedPlatformHandle to ScopedFile because
// they are really the same. This is to allow assignments between the two in
// Linux-specific code paths that predate ScopedPlatformHandle.
static_assert(std::is_same<int, PlatformHandle>::value, "");
using ScopedPlatformHandle = ScopedFile;

// DIR* does not exist on Windows.
using ScopedDir = ScopedResource<DIR*, closedir, nullptr>;
#endif

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_SCOPED_FILE_H_
