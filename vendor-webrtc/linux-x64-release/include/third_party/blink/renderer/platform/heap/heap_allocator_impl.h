// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_ALLOCATOR_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_ALLOCATOR_IMPL_H_

#include "base/bits.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_table_backing.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector_backing.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/thread_state_storage.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/heap/write_barrier.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partition_allocator.h"
#include "v8/include/cppgc/explicit-management.h"
#include "v8/include/cppgc/heap-consistency.h"
#include "v8/include/cppgc/internal/api-constants.h"
#include "v8/include/cppgc/trace-trait.h"
#include "v8/include/cppgc/visitor.h"

namespace blink {

template <typename T>
void GenerationalBarrierForBacking(
    const cppgc::subtle::HeapConsistency::WriteBarrierParams& params,
    T* slot_in_backing);

template <typename K, typename V>
void GenerationalBarrierForBacking(
    const cppgc::subtle::HeapConsistency::WriteBarrierParams& params,
    std::pair<K, V>* slot_in_backing);

template <typename K, typename V>
void GenerationalBarrierForBacking(
    const cppgc::subtle::HeapConsistency::WriteBarrierParams& params,
    WTF::KeyValuePair<K, V>* slot_in_backing);

class PLATFORM_EXPORT HeapAllocator {
  STATIC_ONLY(HeapAllocator);

 public:
  using HeapConsistency = cppgc::subtle::HeapConsistency;
  using LivenessBroker = blink::LivenessBroker;
  using TraceCallback = cppgc::TraceCallback;
  using WeakCallback = cppgc::WeakCallback;

  static constexpr bool kIsGarbageCollected = true;

  template <typename T>
  static size_t MaxElementCountInBackingStore() {
    // Oilpan doesn't have a limit for supported capacity and instead supports
    // arbitrary sized allocations. Delegate to PA to keep limits in sync which
    // may be enforced for security reasons. E.g. PA may cap the limit below
    // 32-bit sizes to avoid integer overflows in old code.
    return WTF::PartitionAllocator::MaxElementCountInBackingStore<T>();
  }

  template <typename T>
  static size_t QuantizedSize(size_t count) {
    CHECK_LE(count, MaxElementCountInBackingStore<T>());
    // Oilpan's internal size is independent of MaxElementCountInBackingStore()
    // and the required size to match capacity needs. Align the size by Oilpan
    // allocation granularity. This also helps us with ASAN API for container
    // annotation, which requires 8-byte alignment for the range.
    return base::bits::AlignUp<size_t>(
        count * sizeof(T),
        cppgc::internal::api_constants::kAllocationGranularity);
  }

  template <typename T>
  static T* AllocateVectorBacking(size_t size) {
    return HeapVectorBacking<T>::ToArray(
        MakeGarbageCollected<HeapVectorBacking<T>>(size / sizeof(T)));
  }

  template <typename T>
  static void FreeVectorBacking(T* array) {
    if (!array)
      return;

    HeapVectorBacking<T>::FromArray(array)->Free(
        ThreadStateStorageFor<ThreadingTrait<T>::kAffinity>::GetState()
            ->heap_handle());
  }

  template <typename T>
  static bool ExpandVectorBacking(T* array, size_t new_size) {
    DCHECK(array);
    return HeapVectorBacking<T>::FromArray(array)->Resize(new_size);
  }

  template <typename T>
  static bool ShrinkVectorBacking(T* array, size_t, size_t new_size) {
    DCHECK(array);
    return HeapVectorBacking<T>::FromArray(array)->Resize(new_size);
  }

  template <typename T, typename HashTable>
  static T* AllocateHashTableBacking(size_t size) {
    static_assert(sizeof(T) == sizeof(typename HashTable::ValueType),
                  "T must match ValueType.");
    return HeapHashTableBacking<HashTable>::ToArray(
        MakeGarbageCollected<HeapHashTableBacking<HashTable>>(size /
                                                              sizeof(T)));
  }

  template <typename T, typename HashTable>
  static T* AllocateZeroedHashTableBacking(size_t size) {
    return AllocateHashTableBacking<T, HashTable>(size);
  }

  template <typename T, typename HashTable>
  static void FreeHashTableBacking(T* array) {
    if (!array)
      return;

    HeapHashTableBacking<HashTable>::FromArray(array)->Free(
        ThreadStateStorageFor<ThreadingTrait<
            HeapHashTableBacking<HashTable>>::kAffinity>::GetState()
            ->heap_handle());
  }

  template <typename T, typename HashTable>
  static bool ExpandHashTableBacking(T* array, size_t new_size) {
    DCHECK(array);
    return HeapHashTableBacking<HashTable>::FromArray(array)->Resize(new_size);
  }

  static bool IsAllocationAllowed() {
    return cppgc::subtle::DisallowGarbageCollectionScope::
        IsGarbageCollectionAllowed(
            ThreadStateStorage::Current()->heap_handle());
  }

  static bool IsIncrementalMarking() {
    auto& heap_handle = ThreadStateStorage::Current()->heap_handle();
    return cppgc::subtle::HeapState::IsMarking(heap_handle) &&
           !cppgc::subtle::HeapState::IsInAtomicPause(heap_handle);
  }

  static void EnterGCForbiddenScope() {
    cppgc::subtle::NoGarbageCollectionScope::Enter(
        ThreadStateStorage::Current()->heap_handle());
  }

  static void LeaveGCForbiddenScope() {
    cppgc::subtle::NoGarbageCollectionScope::Leave(
        ThreadStateStorage::Current()->heap_handle());
  }

  template <typename Traits>
  static bool CanReuseHashTableDeletedBucket() {
    if (Traits::kEmptyValueIsZero || !Traits::kCanTraceConcurrently)
      return true;
    return !IsIncrementalMarking();
  }

  template <typename T>
  static void BackingWriteBarrier(T** backing_pointer_slot) {
    WriteBarrier::DispatchForUncompressedSlot(backing_pointer_slot,
                                              *backing_pointer_slot);
  }

  template <typename T>
  static void TraceBackingStoreIfMarked(T* object) {
    HeapConsistency::WriteBarrierParams params;
    if (HeapConsistency::GetWriteBarrierType(object, params) ==
        HeapConsistency::WriteBarrierType::kMarking) {
      HeapConsistency::SteeleWriteBarrier(params, object);
    }
  }

  template <typename T, typename Traits>
  static void NotifyNewObject(T* slot_in_backing) {
    HeapConsistency::WriteBarrierParams params;
    // `slot_in_backing` points into a backing store and T is not necessarily a
    // garbage collected type but may be kept inline.
    switch (HeapConsistency::GetWriteBarrierType(
        slot_in_backing, params, []() -> cppgc::HeapHandle& {
          return ThreadStateStorageFor<ThreadingTrait<T>::kAffinity>::GetState()
              ->heap_handle();
        })) {
      case HeapConsistency::WriteBarrierType::kMarking:
        HeapConsistency::DijkstraWriteBarrierRange(
            params, slot_in_backing, sizeof(T), 1,
            TraceCollectionIfEnabled<WTF::kNoWeakHandling, T, Traits>::Trace);
        break;
      case HeapConsistency::WriteBarrierType::kGenerational:
        GenerationalBarrierForBacking(params, slot_in_backing);
        break;
      case HeapConsistency::WriteBarrierType::kNone:
        break;
      default:
        break;  // TODO(1056170): Remove default case when API is stable.
    }
  }

  template <typename T, typename Traits>
  static void NotifyNewObjects(base::span<T> objects) {
    T* first_element = &objects.front();
    size_t length = objects.size();
    HeapConsistency::WriteBarrierParams params;
    // `first_element` points into a backing store and T is not necessarily a
    // garbage collected type but may be kept inline.
    switch (HeapConsistency::GetWriteBarrierType(
        first_element, params, []() -> cppgc::HeapHandle& {
          return ThreadStateStorageFor<ThreadingTrait<T>::kAffinity>::GetState()
              ->heap_handle();
        })) {
      case HeapConsistency::WriteBarrierType::kMarking:
        HeapConsistency::DijkstraWriteBarrierRange(
            params, first_element, sizeof(T), length,
            TraceCollectionIfEnabled<WTF::kNoWeakHandling, T, Traits>::Trace);
        break;
      case HeapConsistency::WriteBarrierType::kGenerational:
        GenerationalBarrierForBacking(params, first_element);
        break;
      case HeapConsistency::WriteBarrierType::kNone:
        break;
      default:
        break;  // TODO(1056170): Remove default case when API is stable.
    }
  }

  template <typename T, typename Traits>
  static void Trace(Visitor* visitor, const T& t) {
    TraceCollectionIfEnabled<WTF::kWeakHandlingTrait<T>, T, Traits>::Trace(
        visitor, &t);
  }

  template <typename T>
  static void TraceVectorBacking(Visitor* visitor,
                                 const T* backing,
                                 const T* const* backing_slot) {
    using BackingType = HeapVectorBacking<T>;

    if constexpr (BackingType::TraitsType::kCanMoveWithMemcpy) {
      visitor->RegisterMovableReference(const_cast<const BackingType**>(
          reinterpret_cast<const BackingType* const*>(backing_slot)));
    }
    visitor->TraceStrongContainer(
        reinterpret_cast<const BackingType*>(backing));
  }

  template <typename T, typename HashTable>
  static void TraceHashTableBackingStrongly(Visitor* visitor,
                                            const T* backing,
                                            const T* const* backing_slot) {
    if constexpr (internal::CompactionTraits<
                      HeapHashTableBacking<HashTable>>::SupportsCompaction()) {
      visitor->RegisterMovableReference(
          const_cast<const HeapHashTableBacking<HashTable>**>(
              reinterpret_cast<const HeapHashTableBacking<HashTable>* const*>(
                  backing_slot)));
    }
    visitor->TraceStrongContainer(
        reinterpret_cast<const HeapHashTableBacking<HashTable>*>(backing));
  }

  template <typename T, typename HashTable>
  static void TraceHashTableBackingWeakly(Visitor* visitor,
                                          const T* backing,
                                          const T* const* backing_slot,
                                          WeakCallback callback,
                                          const void* parameter) {
    if constexpr (internal::CompactionTraits<
                      HeapHashTableBacking<HashTable>>::SupportsCompaction()) {
      visitor->RegisterMovableReference(
          const_cast<const HeapHashTableBacking<HashTable>**>(
              reinterpret_cast<const HeapHashTableBacking<HashTable>* const*>(
                  backing_slot)));
    }
    visitor->TraceWeakContainer(
        reinterpret_cast<const HeapHashTableBacking<HashTable>*>(backing),
        callback, parameter);
  }

  static bool DeferTraceToMutatorThreadIfConcurrent(Visitor* visitor,
                                                    const void* object,
                                                    TraceCallback callback,
                                                    size_t deferred_size) {
    return visitor->DeferTraceToMutatorThreadIfConcurrent(object, callback,
                                                          deferred_size);
  }
};

template <typename T>
void GenerationalBarrierForBacking(
    const cppgc::subtle::HeapConsistency::WriteBarrierParams& params,
    T* slot_in_backing) {
  if constexpr (WTF::IsMemberOrWeakMemberType<std::decay_t<T>>::value) {
    // TODO(1029379): Provide Member::GetSlot() and call it here.
    cppgc::subtle::HeapConsistency::GenerationalBarrier(params,
                                                        slot_in_backing);
  } else if constexpr (WTF::IsTraceable<std::decay_t<T>>::value) {
    cppgc::subtle::HeapConsistency::GenerationalBarrierForSourceObject(
        params, slot_in_backing);
  }
}

template <typename K, typename V>
void GenerationalBarrierForBacking(
    const cppgc::subtle::HeapConsistency::WriteBarrierParams& params,
    std::pair<K, V>* slot_in_backing) {
  GenerationalBarrierForBacking(params, &slot_in_backing->first);
  GenerationalBarrierForBacking(params, &slot_in_backing->second);
}

template <typename K, typename V>
void GenerationalBarrierForBacking(
    const cppgc::subtle::HeapConsistency::WriteBarrierParams& params,
    WTF::KeyValuePair<K, V>* slot_in_backing) {
  GenerationalBarrierForBacking(params, &slot_in_backing->key);
  GenerationalBarrierForBacking(params, &slot_in_backing->value);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_ALLOCATOR_IMPL_H_
