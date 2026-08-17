// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_MEMORY_TAGGING_H_
#define BASE_ALLOCATOR_DISPATCHER_MEMORY_TAGGING_H_

#include "partition_alloc/tagging.h"

namespace base::allocator::dispatcher {
// The various modes of Arm's MTE extension. The enum values should match their
// pendants in partition_alloc::TagViolationReportingMode, otherwise the below
// conversion function would involve a translation table or conditional jumps.
enum class MTEMode {
  // Default settings
  kUndefined,
  // MTE explicitly disabled.
  kDisabled,
  // Precise tag violation reports, higher overhead. Good for unittests
  // and security critical threads.
  kSynchronous,
  // Imprecise tag violation reports (async mode). Lower overhead.
  kAsynchronous,
};

constexpr MTEMode ConvertToMTEMode(
    partition_alloc::TagViolationReportingMode pa_mte_reporting_mode) {
  switch (pa_mte_reporting_mode) {
    case partition_alloc::TagViolationReportingMode::kUndefined:
      return MTEMode::kUndefined;
    case partition_alloc::TagViolationReportingMode::kDisabled:
      return MTEMode::kDisabled;
    case partition_alloc::TagViolationReportingMode::kSynchronous:
      return MTEMode::kSynchronous;
    case partition_alloc::TagViolationReportingMode::kAsynchronous:
      return MTEMode::kAsynchronous;
  }
}

}  // namespace base::allocator::dispatcher

#endif  // BASE_ALLOCATOR_DISPATCHER_MEMORY_TAGGING_H_
