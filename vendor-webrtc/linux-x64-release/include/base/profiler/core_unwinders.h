// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_CORE_UNWINDERS_H_
#define BASE_PROFILER_CORE_UNWINDERS_H_

#include "base/base_export.h"
#include "base/profiler/stack_sampling_profiler.h"
#include "build/build_config.h"

static_assert(
    !BUILDFLAG(IS_ANDROID),
    "Android platform is not supported by CreateCoreUnwindersFactory()");

namespace base {

BASE_EXPORT StackSamplingProfiler::UnwindersFactory
CreateCoreUnwindersFactory();

}  // namespace base

#endif  // BASE_PROFILER_CORE_UNWINDERS_H_
