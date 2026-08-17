// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_SAMPLING_PROFILER_THREAD_TOKEN_H_
#define BASE_PROFILER_SAMPLING_PROFILER_THREAD_TOKEN_H_

#include <optional>

#include "base/base_export.h"
#include "base/threading/platform_thread.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_APPLE)
#include <pthread.h>
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
#include <stdint.h>
#endif

namespace base {

// SamplingProfilerThreadToken represents the thread identifier(s) required by
// sampling profiler to operate on a thread. PlatformThreadId is needed for all
// platforms, while Android and Mac also require a pthread_t to pass to pthread
// functions used to obtain the stack base address.
struct SamplingProfilerThreadToken {
  PlatformThreadId id;
#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_APPLE)
  pthread_t pthread_id;
#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
  // Due to the sandbox, we can only retrieve the stack base address for the
  // current thread. We must grab it during
  // GetSamplingProfilerCurrentThreadToken() and not try to get it later.
  std::optional<uintptr_t> stack_base_address;
#endif
};

BASE_EXPORT SamplingProfilerThreadToken GetSamplingProfilerCurrentThreadToken();

}  // namespace base

#endif  // BASE_PROFILER_SAMPLING_PROFILER_THREAD_TOKEN_H_
