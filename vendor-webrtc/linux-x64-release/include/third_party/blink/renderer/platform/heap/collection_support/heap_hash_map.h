// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_MAP_H_

#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/collection_support/utils.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/heap_allocator_impl.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/size_assertions.h"

namespace blink {

template <internal::HeapCollectionType CollectionType,
          typename KeyArg,
          typename MappedArg,
          typename KeyTraitsArg = HashTraits<KeyArg>,
          typename MappedTraitsArg = HashTraits<MappedArg>>
class BasicHeapHashMap final
    : public std::conditional_t<
          CollectionType == internal::HeapCollectionType::kGCed,
          GarbageCollected<BasicHeapHashMap<CollectionType,
                                            KeyArg,
                                            MappedArg,
                                            KeyTraitsArg,
                                            MappedTraitsArg>>,
          internal::DisallowNewBaseForHeapCollections>,
      public HashMap<KeyArg,
                     MappedArg,
                     KeyTraitsArg,
                     MappedTraitsArg,
                     HeapAllocator> {
 public:
  BasicHeapHashMap() = default;

  void Trace(Visitor* visitor) const {
    HashMap<KeyArg, MappedArg, KeyTraitsArg, MappedTraitsArg,
            HeapAllocator>::Trace(visitor);
  }

 private:
  template <typename T>
  static constexpr bool IsValidNonTraceableType() {
    return !WTF::IsTraceable<T>::value &&
           !WTF::IsPointerToGarbageCollectedType<T>;
  }

  struct TypeConstraints {
    constexpr TypeConstraints() {
      static_assert(std::is_trivially_destructible_v<BasicHeapHashMap>,
                    "BasicHeapHashMap must be trivially destructible.");
      static_assert(
          WTF::IsTraceable<KeyArg>::value || WTF::IsTraceable<MappedArg>::value,
          "For hash maps without traceable elements, use HashMap<> "
          "instead of BasicHeapHashMap<>.");
      static_assert(WTF::IsMemberOrWeakMemberType<KeyArg>::value ||
                        IsValidNonTraceableType<KeyArg>(),
                    "BasicHeapHashMap supports only Member, WeakMember and "
                    "non-traceable types as keys.");
      static_assert(
          WTF::IsMemberOrWeakMemberType<MappedArg>::value ||
              WTF::IsTraceable<MappedArg>::value ||
              IsValidNonTraceableType<MappedArg>() ||
              WTF::IsSubclassOfTemplate<MappedArg, v8::TracedReference>::value,
          "BasicHeapHashMap supports only Member, WeakMember, "
          "TraceWrapperV8Reference, objects with Trace(), and "
          "non-traceable types as values.");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

// On-stack for in-field version of WTF::HashSet for referring to
// GarbageCollected objects.
template <typename KeyArg,
          typename MappedArg,
          typename KeyTraitsArg = HashTraits<KeyArg>,
          typename MappedTraitsArg = HashTraits<MappedArg>>
using HeapHashMap = BasicHeapHashMap<internal::HeapCollectionType::kDisallowNew,
                                     KeyArg,
                                     MappedArg,
                                     KeyTraitsArg,
                                     MappedTraitsArg>;

static_assert(WTF::IsDisallowNew<HeapHashMap<int, int>>);
#define COMMA ,
ASSERT_SIZE(HashMap<int COMMA int>, HeapHashMap<int COMMA int>);
#undef COMMA

// GCed version of WTF::HashSet for referring to GarbageCollected objects.
template <typename KeyArg,
          typename MappedArg,
          typename KeyTraitsArg = HashTraits<KeyArg>,
          typename MappedTraitsArg = HashTraits<MappedArg>>
using GCedHeapHashMap = BasicHeapHashMap<internal::HeapCollectionType::kGCed,
                                         KeyArg,
                                         MappedArg,
                                         KeyTraitsArg,
                                         MappedTraitsArg>;

static_assert(!WTF::IsDisallowNew<GCedHeapHashMap<int, int>>);
#define COMMA ,
ASSERT_SIZE(HashMap<int COMMA int>, GCedHeapHashMap<int COMMA int>);
#undef COMMA

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_MAP_H_
