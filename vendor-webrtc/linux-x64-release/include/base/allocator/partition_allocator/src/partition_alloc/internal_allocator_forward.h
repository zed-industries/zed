// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_INTERNAL_ALLOCATOR_FORWARD_H_
#define PARTITION_ALLOC_INTERNAL_ALLOCATOR_FORWARD_H_

#include <new>
#include <type_traits>

#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_forward.h"

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
class InternalAllocator {
 public:
  using value_type = T;
  using is_always_equal = std::true_type;

  InternalAllocator() = default;

  template <typename U>
  InternalAllocator(const InternalAllocator<U>&) {}  // NOLINT

  template <typename U>
  InternalAllocator& operator=(const InternalAllocator<U>&) {
    return *this;
  }

  template <typename U>
  bool operator==(const InternalAllocator<U>&) {
    // InternalAllocator<T> can free allocations made by InternalAllocator<U>.
    return true;
  }

  value_type* allocate(std::size_t count);

  void deallocate(value_type* ptr, std::size_t);
};

// Inherit this to make a class allocated on the internal partition.
struct PA_COMPONENT_EXPORT(PARTITION_ALLOC) InternalPartitionAllocated {
  static void* operator new(size_t count);
  static void* operator new(size_t count, std::align_val_t alignment);
  // Though we do not forward placement new, we need to define this explicitly
  // to allow it.
  static void* operator new(std::size_t, void* ptr) { return ptr; }
  static void operator delete(void* ptr);
  static void operator delete(void* ptr, std::align_val_t);
};

// Create an object on heap in the internal partition.
template <typename T, typename... Args>
T* ConstructAtInternalPartition(Args&&... args);

// Destroy an object on heap in the internal partition.
template <typename T>
void DestroyAtInternalPartition(T* ptr);

// A deleter for `std::unique_ptr<T>`.
template <typename T>
struct InternalPartitionDeleter final {
  void operator()(T* ptr) const { DestroyAtInternalPartition(ptr); }
};

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_INTERNAL_ALLOCATOR_FORWARD_H_
