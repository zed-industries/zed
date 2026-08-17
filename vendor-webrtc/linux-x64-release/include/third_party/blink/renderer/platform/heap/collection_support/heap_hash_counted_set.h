// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_COUNTED_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_COUNTED_SET_H_

#include "third_party/blink/renderer/platform/heap/collection_support/utils.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/heap_allocator_impl.h"
#include "third_party/blink/renderer/platform/wtf/hash_counted_set.h"
#include "third_party/blink/renderer/platform/wtf/size_assertions.h"

namespace blink {

template <internal::HeapCollectionType CollectionType,
          typename Value,
          typename Traits = HashTraits<Value>>
class BasicHeapHashCountedSet final
    : public std::conditional_t<
          CollectionType == internal::HeapCollectionType::kGCed,
          GarbageCollected<
              BasicHeapHashCountedSet<CollectionType, Value, Traits>>,
          internal::DisallowNewBaseForHeapCollections>,
      public HashCountedSet<Value, Traits, HeapAllocator> {
 public:
  BasicHeapHashCountedSet() = default;

  void Trace(Visitor* visitor) const {
    HashCountedSet<Value, Traits, HeapAllocator>::Trace(visitor);
  }

 private:
  struct TypeConstraints {
    constexpr TypeConstraints() {
      static_assert(WTF::IsMemberOrWeakMemberType<Value>::value,
                    "HeapHashCountedSet supports only Member and WeakMember.");
      static_assert(
          std::is_trivially_destructible<BasicHeapHashCountedSet>::value,
          "HeapHashCountedSet must be trivially destructible.");
      static_assert(WTF::IsTraceable<Value>::value,
                    "For counted sets without traceable elements, use "
                    "HashCountedSet<> instead of HeapHashCountedSet<>.");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

// On-stack for in-field version of WTF::HashCountedSet for referring to
// GarbageCollected or DISALLOW_NEW() objects with Trace() methods.
template <typename T, typename Traits = HashTraits<T>>
using HeapHashCountedSet =
    BasicHeapHashCountedSet<internal::HeapCollectionType::kDisallowNew,
                            T,
                            Traits>;

static_assert(WTF::IsDisallowNew<HeapHashCountedSet<int>>);
ASSERT_SIZE(HashCountedSet<int>, HeapHashCountedSet<int>);

// GCed version of WTF::HashCountedSet for referring to GarbageCollected or
// DISALLOW_NEW() objects with Trace() methods.
template <typename T, typename Traits = HashTraits<T>>
using GCedHeapHashCountedSet =
    BasicHeapHashCountedSet<internal::HeapCollectionType::kGCed, T, Traits>;

static_assert(!WTF::IsDisallowNew<GCedHeapHashCountedSet<int>>);
ASSERT_SIZE(HashCountedSet<int>, GCedHeapHashCountedSet<int>);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_COUNTED_SET_H_
