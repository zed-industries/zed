// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_INITIALIZATION_UTIL_H_
#define BASE_TASK_THREAD_POOL_INITIALIZATION_UTIL_H_

#include <stddef.h>

#include "base/base_export.h"

namespace base {

// Computes a value that may be used as the maximum number of threads in a
// ThreadGroup. Developers may use other methods to choose this maximum.
BASE_EXPORT size_t
RecommendedMaxNumberOfThreadsInThreadGroup(size_t min,
                                           size_t max,
                                           double cores_multiplier,
                                           int offset);

}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_INITIALIZATION_UTIL_H_
