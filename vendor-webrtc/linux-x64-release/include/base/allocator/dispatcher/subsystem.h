// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_SUBSYSTEM_H_
#define BASE_ALLOCATOR_DISPATCHER_SUBSYSTEM_H_

namespace base::allocator::dispatcher {

// Identifiers for the memory subsystem handling the allocation. Some observers
// require more detailed information on who is performing the allocation, i.e.
// SamplingHeapProfiler.
enum class AllocationSubsystem {
  // Allocation is handled by PartitionAllocator.
  kPartitionAllocator = 1,
  // Allocation is handled by AllocatorShims.
  kAllocatorShim = 2,
  // Represents a simulated allocation event during testing and is used to
  // filter out these allocations from real ones.
  //
  // Included for backward compatibility, this value becomes obsolete once the
  // old allocation hooks are removed from PoissonAllocationSampler.
  kManualForTesting = 3,
};
}  // namespace base::allocator::dispatcher

#endif  // BASE_ALLOCATOR_DISPATCHER_SUBSYSTEM_H_
