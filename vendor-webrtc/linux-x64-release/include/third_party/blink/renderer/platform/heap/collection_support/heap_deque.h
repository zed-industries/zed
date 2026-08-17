// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_DEQUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_DEQUE_H_

// Include heap_vector.h to also make general VectorTraits available.
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/collection_support/utils.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/heap_allocator_impl.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace blink {

template <internal::HeapCollectionType CollectionType, typename T>
class BasicHeapDeque final
    : public std::conditional_t<
          CollectionType == internal::HeapCollectionType::kGCed,
          GarbageCollected<BasicHeapDeque<CollectionType, T>>,
          internal::DisallowNewBaseForHeapCollections>,
      public Deque<T, 0, HeapAllocator> {
 public:
  BasicHeapDeque() = default;

  explicit BasicHeapDeque(wtf_size_t size) : Deque<T, 0, HeapAllocator>(size) {}

  BasicHeapDeque(wtf_size_t size, const T& val)
      : Deque<T, 0, HeapAllocator>(size, val) {}

  BasicHeapDeque(const BasicHeapDeque<CollectionType, T>& other)
      : Deque<T, 0, HeapAllocator>(other) {}

  BasicHeapDeque& operator=(const BasicHeapDeque& other) {
    Deque<T, 0, HeapAllocator>::operator=(other);
    return *this;
  }

  BasicHeapDeque(BasicHeapDeque&& other) noexcept
      : Deque<T, 0, HeapAllocator>(std::move(other)) {}

  BasicHeapDeque& operator=(BasicHeapDeque&& other) noexcept {
    Deque<T, 0, HeapAllocator>::operator=(std::move(other));
    return *this;
  }

  void Trace(Visitor* visitor) const {
    Deque<T, 0, HeapAllocator>::Trace(visitor);
  }

 private:
  struct TypeConstraints {
    constexpr TypeConstraints() {
      static_assert(WTF::IsMemberType<T>::value,
                    "BasicHeapDeque supports only Member.");
      static_assert(std::is_trivially_destructible_v<BasicHeapDeque>,
                    "BasicHeapDeque must be trivially destructible.");
      static_assert(
          WTF::IsTraceable<T>::value,
          "For deques without traceable elements, use Deque<> instead "
          "of HeapDeque<>");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

// On-stack for in-field version of WTF::Deque for referring to GarbageCollected
// or DISALLOW_NEW() objects with Trace() methods.
template <typename T>
using HeapDeque = BasicHeapDeque<internal::HeapCollectionType::kDisallowNew, T>;

static_assert(WTF::IsDisallowNew<HeapDeque<int>>);
ASSERT_SIZE(Deque<int>, HeapDeque<int>);

// GCed version of WTF::Deque for referring to GarbageCollected or
// DISALLOW_NEW() objects with Trace() methods.
template <typename T>
using GCedHeapDeque = BasicHeapDeque<internal::HeapCollectionType::kGCed, T>;

static_assert(!WTF::IsDisallowNew<GCedHeapDeque<int>>);
ASSERT_SIZE(Deque<int>, GCedHeapDeque<int>);

}  // namespace blink

namespace WTF {

template <typename T>
struct VectorTraits<blink::HeapDeque<T>>
    : VectorTraitsBase<blink::HeapDeque<T>> {
  STATIC_ONLY(VectorTraits);
  static const bool kNeedsDestruction = false;
  static const bool kCanInitializeWithMemset = true;
  static const bool kCanClearUnusedSlotsWithMemset = true;
  static const bool kCanMoveWithMemcpy = true;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_DEQUE_H_
