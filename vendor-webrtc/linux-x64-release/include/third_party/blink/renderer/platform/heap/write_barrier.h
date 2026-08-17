// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_WRITE_BARRIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_WRITE_BARRIER_H_

#include <type_traits>

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "v8/include/cppgc/heap-consistency.h"
#include "v8/include/cppgc/member.h"

namespace blink {

class WriteBarrier final {
  STATIC_ONLY(WriteBarrier);

  using HeapConsistency = cppgc::subtle::HeapConsistency;

 public:
  template <typename T>
  ALWAYS_INLINE static void DispatchForObject(T* element) {
    static_assert(!WTF::IsMemberOrWeakMemberType<std::decay_t<T>>::value,
                  "Member and WeakMember should use the other overload.");
    HeapConsistency::WriteBarrierParams params;
    switch (HeapConsistency::GetWriteBarrierType(element, *element, params)) {
      case HeapConsistency::WriteBarrierType::kMarking:
        HeapConsistency::DijkstraWriteBarrier(params, *element);
        break;
      case HeapConsistency::WriteBarrierType::kGenerational:
        HeapConsistency::GenerationalBarrier(params, element);
        break;
      case HeapConsistency::WriteBarrierType::kNone:
        break;
    }
  }

  // Cannot refer to blink::Member and friends here due to cyclic includes.
  template <typename T,
            typename WeaknessTag,
            typename StorageType,
            typename WriteBarrierPolicy,
            typename CheckingPolicy>
  ALWAYS_INLINE static void DispatchForObject(
      cppgc::internal::BasicMember<T,
                                   WeaknessTag,
                                   StorageType,
                                   WriteBarrierPolicy,
                                   CheckingPolicy>* element) {
    HeapConsistency::WriteBarrierParams params;
    switch (HeapConsistency::GetWriteBarrierType(*element, params)) {
      case HeapConsistency::WriteBarrierType::kMarking:
        HeapConsistency::DijkstraWriteBarrier(params, element->Get());
        break;
      case HeapConsistency::WriteBarrierType::kGenerational:
        HeapConsistency::GenerationalBarrier(params, element);
        break;
      case HeapConsistency::WriteBarrierType::kNone:
        break;
    }
  }

  // Cannot refer to blink::Member and friends here due to cyclic includes.
  template <typename T,
            typename WeaknessTag,
            typename StorageType,
            typename WriteBarrierPolicy,
            typename CheckingPolicy>
  ALWAYS_INLINE static bool IsWriteBarrierNeeded(
      cppgc::internal::BasicMember<T,
                                   WeaknessTag,
                                   StorageType,
                                   WriteBarrierPolicy,
                                   CheckingPolicy>* element) {
    HeapConsistency::WriteBarrierParams params;
    return HeapConsistency::GetWriteBarrierType(*element, params) !=
           HeapConsistency::WriteBarrierType::kNone;
  }

  static void DispatchForUncompressedSlot(void* slot, void* value) {
    HeapConsistency::WriteBarrierParams params;
    switch (HeapConsistency::GetWriteBarrierType(slot, value, params)) {
      case HeapConsistency::WriteBarrierType::kMarking:
        HeapConsistency::DijkstraWriteBarrier(params, value);
        break;
      case HeapConsistency::WriteBarrierType::kGenerational:
        HeapConsistency::GenerationalBarrierForUncompressedSlot(params, slot);
        break;
      case HeapConsistency::WriteBarrierType::kNone:
        break;
    }
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_WRITE_BARRIER_H_
