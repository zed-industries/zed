// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_ALLOCATOR_PARTITION_ALLOCATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_ALLOCATOR_PARTITION_ALLOCATOR_H_

// This is the allocator that is used for allocations that are not on the
// traced, garbage collected heap. It uses FastMalloc for collections,
// but uses the PartitionAlloc for the backing store of the collections.

#include <string.h>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

class WTF_EXPORT PartitionAllocator {
 public:
  static constexpr bool kIsGarbageCollected = false;

  template <typename T>
  static size_t MaxElementCountInBackingStore() {
    return partition_alloc::MaxDirectMapped() / sizeof(T);
  }

  template <typename T>
  static size_t QuantizedSize(size_t count) {
    CHECK_LE(count, MaxElementCountInBackingStore<T>());
    return WTF::Partitions::BufferPotentialCapacity(count * sizeof(T));
  }

  template <typename T>
  static T* AllocateVectorBacking(size_t size) {
    return reinterpret_cast<T*>(
        AllocateBacking(size, WTF_HEAP_PROFILER_TYPE_NAME(T)));
  }

  static void FreeVectorBacking(void* address) { FreeBacking(address); }

  static inline bool ExpandVectorBacking(void*, size_t) { return false; }

  static inline bool ShrinkVectorBacking(void* address,
                                         size_t quantized_current_size,
                                         size_t quantized_shrunk_size) {
    // Optimization: if we're downsizing inside the same allocator bucket,
    // we can skip reallocation.
    return quantized_current_size == quantized_shrunk_size;
  }

  template <typename T, typename HashTable>
  static T* AllocateHashTableBacking(size_t size) {
    return reinterpret_cast<T*>(
        AllocateBacking(size, WTF_HEAP_PROFILER_TYPE_NAME(T)));
  }

  template <typename T, typename HashTable>
  static T* AllocateZeroedHashTableBacking(size_t size) {
    void* result = AllocateBacking(size, WTF_HEAP_PROFILER_TYPE_NAME(T));
    UNSAFE_TODO(memset(result, 0, size));
    return reinterpret_cast<T*>(result);
  }

  template <typename T, typename HashTable>
  static void FreeHashTableBacking(void* address) {
    FreeBacking(address);
  }

  template <typename Return, typename Metadata>
  static Return Malloc(size_t size, const char* type_name) {
    return reinterpret_cast<Return>(
        WTF::Partitions::FastMalloc(size, type_name));
  }

  template <typename T, typename HashTable>
  static inline bool ExpandHashTableBacking(void*, size_t) {
    return false;
  }
  template <typename Traits>
  static inline bool CanReuseHashTableDeletedBucket() {
    return true;
  }

  static void Free(void* address) { WTF::Partitions::FastFree(address); }
  template <typename T>
  static void* NewArray(size_t bytes) {
    return Malloc<void*, void>(bytes, WTF_HEAP_PROFILER_TYPE_NAME(T));
  }
  static void DeleteArray(void* ptr) {
    Free(ptr);  // Not the system free, the one from this class.
  }

  template <typename T>
  static void TraceBackingStoreIfMarked(T*) {}
  template <typename T>
  static void BackingWriteBarrier(T**) {}

  static bool IsAllocationAllowed() { return true; }
  static bool IsIncrementalMarking() { return false; }

  static void EnterGCForbiddenScope() {}
  static void LeaveGCForbiddenScope() {}

  template <typename T, typename Traits>
  static void NotifyNewObject(T* object) {}

  template <typename T, typename Traits>
  static void NotifyNewObjects(base::span<T>) {}

 private:
  static void* AllocateBacking(size_t, const char* type_name);
  static void FreeBacking(void*);
};

// Specializations for heap profiling, so type profiling of |char| is possible
// even in official builds (because |char| makes up a large portion of the
// heap.)
template <>
WTF_EXPORT char* PartitionAllocator::AllocateVectorBacking<char>(size_t);

}  // namespace WTF

#define USE_ALLOCATOR(ClassName, Allocator)                     \
 public:                                                        \
  void* operator new(size_t size) {                             \
    return Allocator::template Malloc<void*, ClassName>(        \
        size, WTF_HEAP_PROFILER_TYPE_NAME(ClassName));          \
  }                                                             \
  void operator delete(void* p) {                               \
    Allocator::Free(p);                                         \
  }                                                             \
  void* operator new[](size_t size) {                           \
    return Allocator::template NewArray<ClassName>(size);       \
  }                                                             \
  void operator delete[](void* p) {                             \
    Allocator::DeleteArray(p);                                  \
  }                                                             \
  void* operator new(size_t, WTF::NotNullTag, void* location) { \
    DCHECK(location);                                           \
    return location;                                            \
  }                                                             \
  void* operator new(size_t, void* location) {                  \
    return location;                                            \
  }                                                             \
                                                                \
 private:                                                       \
  typedef int __thisIsHereToForceASemicolonAfterThisMacro

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_ALLOCATOR_PARTITION_ALLOCATOR_H_
