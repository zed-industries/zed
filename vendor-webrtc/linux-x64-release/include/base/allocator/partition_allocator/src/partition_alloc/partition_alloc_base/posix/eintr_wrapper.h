// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This provides a wrapper around system calls which may be interrupted by a
// signal and return EINTR. See man 7 signal.
// To prevent long-lasting loops (which would likely be a bug, such as a signal
// that should be masked) to go unnoticed, there is a limit after which the
// caller will nonetheless see an EINTR in Debug builds.
//
// On Windows and Fuchsia, this wrapper does nothing because there are no
// signals.
//
// Don't wrap close calls in WrapEINTR. Use IGNORE_EINTR macro if the return
// value of close is significant. See http://crbug.com/269623.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_POSIX_EINTR_WRAPPER_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_POSIX_EINTR_WRAPPER_H_

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(IS_POSIX)
#include <cerrno>
#include <utility>
#endif

namespace partition_alloc {
#if PA_BUILDFLAG(IS_POSIX)

template <typename Fn>
inline auto WrapEINTR(Fn fn) {
  return [fn](auto&&... args) {
    int out = -1;
#if !PA_BUILDFLAG(IS_DEBUG)
    while (true)
#else
    for (int retry_count = 0; retry_count < 100; ++retry_count)
#endif
    {
      out = fn(std::forward<decltype(args)>(args)...);
      if (out != -1 || errno != EINTR) {
        return out;
      }
    }
    return out;
  };
}

#else  // !PA_BUILDFLAG(IS_POSIX)

template <typename Fn>
inline auto WrapEINTR(Fn fn) {
  return fn;
}

#endif  // !PA_BUILDFLAG(IS_POSIX)

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_POSIX_EINTR_WRAPPER_H_
