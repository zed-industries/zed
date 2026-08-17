// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_FOR_TESTING_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_FOR_TESTING_H_

#include "partition_alloc/partition_alloc.h"

namespace partition_alloc {
namespace internal {

constexpr bool AllowLeaks = true;
constexpr bool DisallowLeaks = false;

// A subclass of PartitionAllocator for testing. It will free all resources,
// i.e. allocated memory, memory inside freelist, and so on, when destructing
// it or when manually invoking reset().
// If need to check if there are any memory allocated but not freed yet,
// use allow_leaks=false. We will see CHECK failure inside reset() if any
// leak is detected. Otherwise (e.g. intentional leaks), use allow_leaks=true.
template <bool allow_leaks>
struct PartitionAllocatorForTesting : public PartitionAllocator {
  PartitionAllocatorForTesting() : PartitionAllocator() {}

  explicit PartitionAllocatorForTesting(PartitionOptions opts)
      : PartitionAllocator(opts) {}

  ~PartitionAllocatorForTesting() { reset(); }

  PA_ALWAYS_INLINE void reset() {
    PartitionAllocator::root()->ResetForTesting(allow_leaks);
  }
};

}  // namespace internal

using PartitionAllocatorForTesting =
    internal::PartitionAllocatorForTesting<internal::DisallowLeaks>;

using PartitionAllocatorAllowLeaksForTesting =
    internal::PartitionAllocatorForTesting<internal::AllowLeaks>;

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_FOR_TESTING_H_
