// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_SET_H_

#include "third_party/blink/renderer/platform/heap/collection_support/utils.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/heap_allocator_impl.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/size_assertions.h"

namespace blink {

template <internal::HeapCollectionType CollectionType,
          typename ValueArg,
          typename TraitsArg = HashTraits<ValueArg>>
class BasicHeapHashSet final
    : public std::conditional_t<
          CollectionType == internal::HeapCollectionType::kGCed,
          GarbageCollected<
              BasicHeapHashSet<CollectionType, ValueArg, TraitsArg>>,
          internal::DisallowNewBaseForHeapCollections>,
      public HashSet<ValueArg, TraitsArg, HeapAllocator> {
 public:
  BasicHeapHashSet() = default;

  BasicHeapHashSet(const BasicHeapHashSet& other)
      : HashSet<ValueArg, TraitsArg, HeapAllocator>(other) {}

  BasicHeapHashSet& operator=(const BasicHeapHashSet& other) {
    HashSet<ValueArg, TraitsArg, HeapAllocator>::operator=(other);
    return *this;
  }

  template <internal::HeapCollectionType OtherCollectionType>
  BasicHeapHashSet(
      const BasicHeapHashSet<OtherCollectionType, ValueArg, TraitsArg>& other)
      : HashSet<ValueArg, TraitsArg, HeapAllocator>(other) {}

  BasicHeapHashSet(BasicHeapHashSet&& other)
      : HashSet<ValueArg, TraitsArg, HeapAllocator>(std::move(other)) {}

  BasicHeapHashSet& operator=(BasicHeapHashSet&& other) noexcept {
    HashSet<ValueArg, TraitsArg, HeapAllocator>::operator=(std::move(other));
    return *this;
  }

  template <internal::HeapCollectionType OtherCollectionType>
  BasicHeapHashSet(
      BasicHeapHashSet<OtherCollectionType, ValueArg, TraitsArg>&& other)
      : HashSet<ValueArg, TraitsArg, HeapAllocator>(std::move(other)) {}

  void Trace(Visitor* visitor) const {
    HashSet<ValueArg, TraitsArg, HeapAllocator>::Trace(visitor);
  }

 private:
  struct TypeConstraints {
    constexpr TypeConstraints() {
      static_assert(WTF::IsMemberOrWeakMemberType<ValueArg>::value,
                    "BasicHeapHashSet supports only Member and WeakMember.");
      static_assert(std::is_trivially_destructible_v<BasicHeapHashSet>,
                    "BasicHeapHashSet must be trivially destructible.");
      static_assert(WTF::IsTraceable<ValueArg>::value,
                    "For hash sets without traceable elements, use HashSet<> "
                    "instead of BasicHeapHashSet<>.");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

// On-stack for in-field version of WTF::HashSet for referring to
// GarbageCollected objects.
template <typename T, typename Traits = HashTraits<T>>
using HeapHashSet =
    BasicHeapHashSet<internal::HeapCollectionType::kDisallowNew, T, Traits>;

static_assert(WTF::IsDisallowNew<HeapHashSet<int>>);
ASSERT_SIZE(HashSet<int>, HeapHashSet<int>);

// GCed version of WTF::HashSet for referring to GarbageCollected objects.
template <typename T, typename Traits = HashTraits<T>>
using GCedHeapHashSet =
    BasicHeapHashSet<internal::HeapCollectionType::kGCed, T, Traits>;

static_assert(!WTF::IsDisallowNew<GCedHeapHashSet<int>>);
ASSERT_SIZE(HashSet<int>, GCedHeapHashSet<int>);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_SET_H_
