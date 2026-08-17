// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_SCOPED_CLEAR_LAST_ERROR_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_SCOPED_CLEAR_LAST_ERROR_H_

#include <cerrno>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base {

// ScopedClearLastError stores and resets the value of thread local error codes
// (errno, GetLastError()), and restores them in the destructor. This is useful
// to avoid side effects on these values in instrumentation functions that
// interact with the OS.

// Common implementation of ScopedClearLastError for all platforms. Use
// ScopedClearLastError instead.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) ScopedClearLastErrorBase {
 public:
  ScopedClearLastErrorBase() : last_errno_(errno) { errno = 0; }
  ScopedClearLastErrorBase(const ScopedClearLastErrorBase&) = delete;
  ScopedClearLastErrorBase& operator=(const ScopedClearLastErrorBase&) = delete;
  ~ScopedClearLastErrorBase() { errno = last_errno_; }

 private:
  const int last_errno_;
};

#if PA_BUILDFLAG(IS_WIN)

// Windows specific implementation of ScopedClearLastError.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) ScopedClearLastError
    : public ScopedClearLastErrorBase {
 public:
  ScopedClearLastError();
  ScopedClearLastError(const ScopedClearLastError&) = delete;
  ScopedClearLastError& operator=(const ScopedClearLastError&) = delete;
  ~ScopedClearLastError();

 private:
  const unsigned long last_system_error_;
};

#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)

using ScopedClearLastError = ScopedClearLastErrorBase;

#endif  // PA_BUILDFLAG(IS_WIN)

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_SCOPED_CLEAR_LAST_ERROR_H_
