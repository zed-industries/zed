// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_STACK_SAMPLING_PROFILER_JAVA_TEST_UTIL_H_
#define BASE_PROFILER_STACK_SAMPLING_PROFILER_JAVA_TEST_UTIL_H_

#include "base/functional/callback_forward.h"
#include "base/profiler/stack_sampling_profiler_test_util.h"

namespace base {

// Wrapper to call |TestSupport.callWithJavaFunction| java function from native.
// Returns address range of InvokeCallbackFunction defined by Native.
FunctionAddressRange callWithJavaFunction(OnceClosure closure);

}  // namespace base

#endif  // BASE_PROFILER_STACK_SAMPLING_PROFILER_JAVA_TEST_UTIL_H_
