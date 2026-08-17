// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_NOTREACHED_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_NOTREACHED_H_

#include "partition_alloc/partition_alloc_base/check.h"

// PA_NOTREACHED() annotates paths that are supposed to be unreachable. They
// crash if they are ever hit.
#if PA_BASE_CHECK_WILL_STREAM()
// PartitionAlloc uses async-signal-safe RawCheckFailure() for error reporting.
// Async-signal-safe functions are guaranteed to not allocate as otherwise they
// could operate with inconsistent allocator state.
#define PA_NOTREACHED()                                  \
  ::partition_alloc::internal::logging::RawCheckFailure( \
      __FILE__ "(" PA_STRINGIFY(__LINE__) ") PA_NOTREACHED() hit.")
#else
#define PA_NOTREACHED() PA_IMMEDIATE_CRASH()
#endif  // CHECK_WILL_STREAM()

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_NOTREACHED_H_
