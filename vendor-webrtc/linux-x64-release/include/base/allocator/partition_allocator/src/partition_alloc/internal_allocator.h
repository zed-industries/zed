// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_INTERNAL_ALLOCATOR_H_
#define PARTITION_ALLOC_INTERNAL_ALLOCATOR_H_

#include <new>
#include <type_traits>

#include "partition_alloc/internal_allocator_forward.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_root.h"

// Internal Allocator can be used to get heap allocations required to
// implement PartitionAlloc's feature.
// As Internal Allocator being PartitionAlloc with minimal configuration,
// it is not allowed to use this allocator for PA's core implementation to avoid
// reentrancy issues. Also don't use this when satisfying the very first PA-E
// allocation of the process.

namespace partition_alloc::internal {

PA_COMPONENT_EXPORT(PARTITION_ALLOC)
PartitionRoot& InternalAllocatorRoot();

// A class that meets C++ named requirements, Allocator.
template <typename T>
typename InternalAllocator<T>::value_type* InternalAllocator<T>::allocate(
    std::size_t count) {
  PA_CHECK(count <=
           std::numeric_limits<std::size_t>::max() / sizeof(value_type));
  return static_cast<value_type*>(
      InternalAllocatorRoot().Alloc<AllocFlags::kNoHooks>(count *
                                                          sizeof(value_type)));
}
template <typename T>
void InternalAllocator<T>::deallocate(value_type* ptr, std::size_t) {
  InternalAllocatorRoot().Free<FreeFlags::kNoHooks>(ptr);
}

// Create an object on heap in the internal partition.
template <typename T, typename... Args>
T* ConstructAtInternalPartition(Args&&... args) {
  auto* memory = static_cast<T*>(
      InternalAllocatorRoot().Alloc<AllocFlags::kNoHooks>(sizeof(T)));
  return new (memory) T(std::forward<Args>(args)...);
}

// Destroy an object on heap in the internal partition.
template <typename T>
void DestroyAtInternalPartition(T* ptr) {
  // Destroying an array is not supported.
  static_assert(!std::is_array_v<T>);
  ptr->~T();
  InternalAllocatorRoot().Free<FreeFlags::kNoHooks>(ptr);
}

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_INTERNAL_ALLOCATOR_H_
