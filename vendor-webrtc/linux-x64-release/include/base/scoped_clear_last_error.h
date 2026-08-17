// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SCOPED_CLEAR_LAST_ERROR_H_
#define BASE_SCOPED_CLEAR_LAST_ERROR_H_

#include <errno.h>

#include "base/base_export.h"
#include "build/build_config.h"

namespace base {

// ScopedClearLastError stores and resets the value of thread local error codes
// (errno, GetLastError()), and restores them in the destructor. This is useful
// to avoid side effects on these values in instrumentation functions that
// interact with the OS.

// Common implementation of ScopedClearLastError for all platforms. Use
// ScopedClearLastError instead.
class BASE_EXPORT ScopedClearLastErrorBase {
 public:
  ScopedClearLastErrorBase() : last_errno_(errno) { errno = 0; }
  ScopedClearLastErrorBase(const ScopedClearLastErrorBase&) = delete;
  ScopedClearLastErrorBase& operator=(const ScopedClearLastErrorBase&) = delete;
  ~ScopedClearLastErrorBase() { errno = last_errno_; }

 private:
  const int last_errno_;
};

#if BUILDFLAG(IS_WIN)

// Windows specific implementation of ScopedClearLastError.
class BASE_EXPORT ScopedClearLastError : public ScopedClearLastErrorBase {
 public:
  ScopedClearLastError();
  ScopedClearLastError(const ScopedClearLastError&) = delete;
  ScopedClearLastError& operator=(const ScopedClearLastError&) = delete;
  ~ScopedClearLastError();

 private:
  const unsigned long last_system_error_;
};

#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)

using ScopedClearLastError = ScopedClearLastErrorBase;

#endif  // BUILDFLAG(IS_WIN)

}  // namespace base

#endif  // BASE_SCOPED_CLEAR_LAST_ERROR_H_
