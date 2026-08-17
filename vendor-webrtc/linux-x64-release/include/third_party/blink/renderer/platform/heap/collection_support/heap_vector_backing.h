// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_VECTOR_BACKING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_VECTOR_BACKING_H_

#include <type_traits>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/heap/collection_support/utils.h"
#include "third_party/blink/renderer/platform/heap/custom_spaces.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/thread_state_storage.h"
#include "third_party/blink/renderer/platform/heap/trace_traits.h"
#include "third_party/blink/renderer/platform/wtf/container_annotations.h"
#include "third_party/blink/renderer/platform/wtf/sanitizers.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"
#include "v8/include/cppgc/allocation.h"
#include "v8/include/cppgc/custom-space.h"
#include "v8/include/cppgc/explicit-management.h"
#include "v8/include/cppgc/object-size-trait.h"
#include "v8/include/cppgc/trace-trait.h"
#include "v8/include/cppgc/visitor.h"

namespace blink {
namespace internal {

inline bool VTableInitialized(const void* object_payload) {
  return !!(*reinterpret_cast<const size_t*>(object_payload));
}

}  // namespace internal

template <typename T, typename Traits = WTF::VectorTraits<T>>
class HeapVectorBacking final
    : public GarbageCollected<HeapVectorBacking<T, Traits>> {
 public:
  using ClassType = HeapVectorBacking<T, Traits>;
  using TraitsType = Traits;

  // Although the HeapVectorBacking is fully constructed, the array resulting
  // from ToArray may not be fully constructed as the elements of the array are
  // not initialized and may have null vtable pointers. Null vtable pointer
  // violates CFI for polymorphic types.
  ALWAYS_INLINE NO_SANITIZE_UNRELATED_CAST static T* ToArray(
      ClassType* backing) {
    return reinterpret_cast<T*>(backing);
  }

  ALWAYS_INLINE static ClassType* FromArray(T* payload) {
    return reinterpret_cast<ClassType*>(payload);
  }

  void Free(cppgc::HeapHandle& heap_handle) {
    cppgc::subtle::FreeUnreferencedObject(heap_handle, *this);
  }

  bool Resize(size_t new_size) {
    return cppgc::subtle::Resize(*this, GetAdditionalBytes(new_size));
  }

  ~HeapVectorBacking()
    requires(!Traits::kNeedsDestruction)
  = default;
  ~HeapVectorBacking()
    requires(Traits::kNeedsDestruction);

 private:
  static cppgc::AdditionalBytes GetAdditionalBytes(size_t wanted_array_size) {
    // HVB is an empty class that's purely used with inline storage. Since its
    // sizeof(HVB) == 1, we need to subtract its size to avoid wasting storage.
    static_assert(sizeof(ClassType) == 1, "Class declaration changed");
    DCHECK_GE(wanted_array_size, sizeof(ClassType));
    return cppgc::AdditionalBytes{wanted_array_size - sizeof(ClassType)};
  }
};

template <typename T, typename Traits>
HeapVectorBacking<T, Traits>::~HeapVectorBacking()
  requires(Traits::kNeedsDestruction)
{
  static_assert(
      Traits::kCanClearUnusedSlotsWithMemset || std::is_polymorphic<T>::value,
      "HeapVectorBacking doesn't support objects that cannot be cleared as "
      "unused with memset or don't have a vtable");
  static_assert(
      !std::is_trivially_destructible<T>::value,
      "Finalization of trivially destructible classes should not happen.");
  const size_t object_size =
      cppgc::subtle::ObjectSizeTrait<HeapVectorBacking<T, Traits>>::GetSize(
          *this);
  const size_t length = object_size / sizeof(T);
  using ByteBuffer = uint8_t*;
  ByteBuffer payload = reinterpret_cast<ByteBuffer>(this);
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
  ANNOTATE_CHANGE_SIZE(payload, length * sizeof(T), 0, length * sizeof(T));
#endif  // ANNOTATE_CONTIGUOUS_CONTAINER
  // HeapVectorBacking calls finalizers for unused slots and expects them to be
  // no-ops.
  if (std::is_polymorphic<T>::value) {
    for (size_t i = 0; i < length; ++i) {
      ByteBuffer element = payload + i * sizeof(T);
      if (internal::VTableInitialized(element))
        reinterpret_cast<T*>(element)->~T();
    }
  } else {
    T* buffer = reinterpret_cast<T*>(payload);
    for (size_t i = 0; i < length; ++i)
      buffer[i].~T();
  }
}

template <typename T, typename Traits>
struct ThreadingTrait<HeapVectorBacking<T, Traits>> {
  STATIC_ONLY(ThreadingTrait);
  static constexpr ThreadAffinity kAffinity = ThreadingTrait<T>::kAffinity;
};

namespace internal {
template <typename T>
struct CompactionTraits<blink::HeapVectorBacking<T>> {
  static constexpr bool SupportsCompaction() {
    return blink::HeapVectorBacking<T>::TraitsType::kCanMoveWithMemcpy;
  }
};
}  // namespace internal
}  // namespace blink

namespace WTF {

// This trace method is used for all HeapVectorBacking objects. On-stack objects
// are found and dispatched using conservative stack scanning. HeapVector (i.e.
// Vector) dispatches all regular on-heap backings to this method.
template <typename T, typename Traits>
struct TraceInCollectionTrait<kNoWeakHandling,
                              blink::HeapVectorBacking<T, Traits>,
                              void> {
  using Backing = blink::HeapVectorBacking<T, Traits>;

  static void Trace(blink::Visitor* visitor, const void* self) {
    // HeapVectorBacking does not know the exact size of the vector
    // and just knows the capacity of the vector. Due to the constraint,
    // HeapVectorBacking can support only the following objects:
    //
    // - An object that has a vtable. In this case, HeapVectorBacking
    //   traces only slots that are not zeroed out. This is because if
    //   the object has a vtable, the zeroed slot means that it is
    //   an unused slot (Remember that the unused slots are guaranteed
    //   to be zeroed out by VectorUnusedSlotClearer).
    //
    // - An object that can be initialized with memset. In this case,
    //   HeapVectorBacking traces all slots including unused slots.
    //   This is fine because the fact that the object can be initialized
    //   with memset indicates that it is safe to treat the zerod slot
    //   as a valid object.
    static_assert(
        Traits::kCanClearUnusedSlotsWithMemset || std::is_polymorphic<T>::value,
        "HeapVectorBacking doesn't support objects that cannot be "
        "cleared as unused with memset.");

    // Bail out early if the contents are not actually traceable.
    if constexpr (!IsTraceable<T>::value) {
      return;
    }

    const T* array = reinterpret_cast<const T*>(self);
    const size_t length =
        cppgc::subtle::ObjectSizeTrait<const Backing>::GetSize(
            *reinterpret_cast<const Backing*>(self)) /
        sizeof(T);
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
    // As commented above, HeapVectorBacking can trace unused slots (which are
    // already zeroed out).
    ANNOTATE_CHANGE_SIZE(array, length, 0, length);
#endif  // ANNOTATE_CONTIGUOUS_CONTAINER
    if constexpr (IsTraceable<T>::value) {
      for (unsigned i = 0; i < length; ++i) {
        if (!std::is_polymorphic_v<T> ||
            blink::internal::VTableInitialized(&array[i])) {
          visitor->Trace(array[i]);
        }
      }
    }
  }
};

}  // namespace WTF

namespace cppgc {

// The space trait rewires allocations for HeapVector with `kCanMoveWithMemcpy`
// into a space supporting compaction.
template <typename T>
struct SpaceTrait<blink::HeapVectorBacking<T>,
                  std::enable_if_t<blink::internal::CompactionTraits<
                      blink::HeapVectorBacking<T>>::SupportsCompaction()>> {
  using Space = blink::CompactableHeapVectorBackingSpace;
};

// Custom allocation accounts for inlined storage of the actual elements of the
// backing array.
template <typename T>
class MakeGarbageCollectedTrait<blink::HeapVectorBacking<T>>
    : public MakeGarbageCollectedTraitBase<blink::HeapVectorBacking<T>> {
  using Backing = blink::HeapVectorBacking<T>;

 public:
  template <typename... Args>
  static Backing* Call(AllocationHandle& handle, size_t num_elements) {
    static_assert(!std::is_polymorphic<blink::HeapVectorBacking<T>>::value,
                  "HeapVectorBacking must not be polymorphic as it is "
                  "converted to a raw array of buckets for certain operation");
    CHECK_GT(num_elements, 0u);
    // Allocate automatically considers the custom space via SpaceTrait.
    void* memory = MakeGarbageCollectedTraitBase<Backing>::Allocate(
        handle, sizeof(T) * num_elements);
    Backing* object = ::new (memory) Backing();
    MakeGarbageCollectedTraitBase<Backing>::MarkObjectAsFullyConstructed(
        object);
    return object;
  }
};

template <typename T, typename Traits>
struct TraceTrait<blink::HeapVectorBacking<T, Traits>> {
  using Backing = blink::HeapVectorBacking<T, Traits>;

  static TraceDescriptor GetTraceDescriptor(const void* self) {
    return {self, Trace};
  }

  static void Trace(Visitor* visitor, const void* self) {
    static_assert(!WTF::IsWeak<T>::value,
                  "Weakness is not supported in HeapVector and HeapDeque");

    // Early bailout for non-traceable types. `GetTraceDescriptor()` doesn't
    // support returning a null callback, so this is the best we can do for now.
    if (!WTF::IsTraceable<T>::value) {
      return;
    }

    if (!Traits::kCanTraceConcurrently && self) {
      if (visitor->DeferTraceToMutatorThreadIfConcurrent(
              self, &Trace,
              cppgc::subtle::ObjectSizeTrait<const Backing>::GetSize(
                  *reinterpret_cast<const Backing*>(self)))) {
        return;
      }
    }

    WTF::TraceInCollectionTrait<WTF::kNoWeakHandling,
                                blink::HeapVectorBacking<T, Traits>,
                                void>::Trace(visitor, self);
  }
};

}  // namespace cppgc

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_VECTOR_BACKING_H_
