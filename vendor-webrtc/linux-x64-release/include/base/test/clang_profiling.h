// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_CLANG_PROFILING_H_
#define BASE_TEST_CLANG_PROFILING_H_

#include "base/base_export.h"
#include "base/clang_profiling_buildflags.h"

#if !BUILDFLAG(CLANG_PROFILING)
#error "Clang profiling can only be used if CLANG_PROFILING macro is defined"
#endif

namespace base {

// Write out the accumulated code profiling profile to the configured file.
// This is used internally by e.g. base::Process and FATAL logging, to cause
// profiling information to be stored even when performing an "immediate" exit
// (or triggering a debug crash), where the automatic at-exit writer will not
// be invoked.
// This call is thread-safe, and will write profiling data at-most-once.
BASE_EXPORT void WriteClangProfilingProfile();

}  // namespace base

#endif  // BASE_TEST_CLANG_PROFILING_H_
