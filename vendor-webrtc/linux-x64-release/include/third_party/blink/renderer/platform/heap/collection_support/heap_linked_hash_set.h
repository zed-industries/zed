// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_LINKED_HASH_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_LINKED_HASH_SET_H_

#include "third_party/blink/renderer/platform/heap/collection_support/utils.h"
// Needs heap_vector.h for VectorTraits of Member and WeakMember.
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/heap_allocator_impl.h"
#include "third_party/blink/renderer/platform/wtf/linked_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/size_assertions.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace blink {

template <internal::HeapCollectionType CollectionType,
          typename ValueArg,
          typename TraitsArg = HashTraits<ValueArg>>
class BasicHeapLinkedHashSet final
    : public std::conditional_t<
          CollectionType == internal::HeapCollectionType::kGCed,
          GarbageCollected<
              BasicHeapLinkedHashSet<CollectionType, ValueArg, TraitsArg>>,
          internal::DisallowNewBaseForHeapCollections>,
      public LinkedHashSet<ValueArg, TraitsArg, HeapAllocator> {
 public:
  BasicHeapLinkedHashSet() = default;

  void Trace(Visitor* v) const {
    LinkedHashSet<ValueArg, TraitsArg, HeapAllocator>::Trace(v);
  }

 private:
  struct TypeConstraints {
    constexpr TypeConstraints() {
      static_assert(
          WTF::IsMemberOrWeakMemberType<ValueArg>::value,
          "BasicHeapLinkedHashSet supports only Member and WeakMember.");
      static_assert(std::is_trivially_destructible_v<BasicHeapLinkedHashSet>,
                    "BasicHeapLinkedHashSet must be trivially destructible.");
      static_assert(WTF::IsTraceable<ValueArg>::value,
                    "For sets without traceable elements, use LinkedHashSet<> "
                    "instead of BasicHeapLinkedHashSet<>.");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

// On-stack for in-field version of WTF::LinkedHashSet for referring to
// GarbageCollected objects.
template <typename T, typename Traits = HashTraits<T>>
using HeapLinkedHashSet =
    BasicHeapLinkedHashSet<internal::HeapCollectionType::kDisallowNew,
                           T,
                           Traits>;

static_assert(WTF::IsDisallowNew<HeapLinkedHashSet<int>>);
ASSERT_SIZE(LinkedHashSet<int>, HeapLinkedHashSet<int>);

// GCed version of WTF::LinkedHashSet for referring to GarbageCollected objects.
template <typename T, typename Traits = HashTraits<T>>
using GCedHeapLinkedHashSet =
    BasicHeapLinkedHashSet<internal::HeapCollectionType::kGCed, T, Traits>;

static_assert(!WTF::IsDisallowNew<GCedHeapLinkedHashSet<int>>);
ASSERT_SIZE(LinkedHashSet<int>, GCedHeapLinkedHashSet<int>);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_LINKED_HASH_SET_H_
