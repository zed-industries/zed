// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_TABLE_BACKING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_TABLE_BACKING_H_

#include <type_traits>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/heap/collection_support/utils.h"
#include "third_party/blink/renderer/platform/heap/custom_spaces.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/thread_state_storage.h"
#include "third_party/blink/renderer/platform/heap/trace_traits.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/hash_table.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/key_value_pair.h"
#include "third_party/blink/renderer/platform/wtf/sanitizers.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "v8/include/cppgc/custom-space.h"
#include "v8/include/cppgc/explicit-management.h"
#include "v8/include/cppgc/object-size-trait.h"

namespace blink {

template <typename Table>
class HeapHashTableBacking final
    : public GarbageCollected<HeapHashTableBacking<Table>> {
  using ClassType = HeapHashTableBacking<Table>;
  using ValueType = typename Table::ValueType;

 public:
  // Although the HeapHashTableBacking is fully constructed, the array resulting
  // from ToArray may not be fully constructed as the elements of the array are
  // not initialized and may have null vtable pointers. Null vtable pointer
  // violates CFI for polymorphic types.
  ALWAYS_INLINE NO_SANITIZE_UNRELATED_CAST static ValueType* ToArray(
      ClassType* backing) {
    return reinterpret_cast<ValueType*>(backing);
  }

  ALWAYS_INLINE static ClassType* FromArray(ValueType* array) {
    return reinterpret_cast<ClassType*>(array);
  }

  void Free(cppgc::HeapHandle& heap_handle) {
    cppgc::subtle::FreeUnreferencedObject(heap_handle, *this);
  }

  bool Resize(size_t new_size) {
    return cppgc::subtle::Resize(*this, GetAdditionalBytes(new_size));
  }

  ~HeapHashTableBacking()
    requires(std::is_trivially_destructible_v<typename Table::ValueType>)
  = default;
  ~HeapHashTableBacking()
    requires(!std::is_trivially_destructible_v<typename Table::ValueType>);

 private:
  static cppgc::AdditionalBytes GetAdditionalBytes(size_t wanted_array_size) {
    // HHTB is an empty class that's purely used with inline storage. Since its
    // sizeof(HHTB) == 1, we need to subtract its size to avoid wasting storage.
    static_assert(sizeof(ClassType) == 1, "Class declaration changed");
    DCHECK_GE(wanted_array_size, sizeof(ClassType));
    return cppgc::AdditionalBytes{wanted_array_size - sizeof(ClassType)};
  }
};

template <typename Table>
HeapHashTableBacking<Table>::~HeapHashTableBacking()
  requires(!std::is_trivially_destructible_v<typename Table::ValueType>)
{
  using Value = typename Table::ValueType;
  static_assert(
      !std::is_trivially_destructible<Value>::value,
      "Finalization of trivially destructible classes should not happen.");
  const size_t object_size =
      cppgc::subtle::ObjectSizeTrait<HeapHashTableBacking<Table>>::GetSize(
          *this);
  const size_t length = object_size / sizeof(Value);
  Value* table = reinterpret_cast<Value*>(this);
  for (unsigned i = 0; i < length; ++i) {
    if (!Table::IsEmptyOrDeletedBucket(table[i]))
      table[i].~Value();
  }
}

template <typename Table>
struct ThreadingTrait<HeapHashTableBacking<Table>> {
  STATIC_ONLY(ThreadingTrait);
  using Key = typename Table::KeyType;
  using Value = typename Table::ValueType;
  static constexpr ThreadAffinity kAffinity =
      (ThreadingTrait<Key>::kAffinity == kMainThreadOnly) &&
              (ThreadingTrait<Value>::kAffinity == kMainThreadOnly)
          ? kMainThreadOnly
          : kAnyThread;
};

template <typename First, typename Second>
struct ThreadingTrait<WTF::KeyValuePair<First, Second>> {
  STATIC_ONLY(ThreadingTrait);
  static constexpr ThreadAffinity kAffinity =
      (ThreadingTrait<First>::kAffinity == kMainThreadOnly) &&
              (ThreadingTrait<Second>::kAffinity == kMainThreadOnly)
          ? kMainThreadOnly
          : kAnyThread;
};

namespace internal {

template <typename Table>
struct CompactionTraits<blink::HeapHashTableBacking<Table>> {
  static constexpr bool SupportsCompaction() {
    using ValueTraits = typename Table::ValueTraits;
    return ValueTraits::kSupportsCompaction;
  }
};
}  // namespace internal
}  // namespace blink

namespace WTF {

namespace internal {

// ConcurrentBucket is a wrapper for HashTable buckets for concurrent marking.
// It is used to provide a snapshot view of the bucket key and guarantee
// that the same key is used for checking empty/deleted buckets and tracing.
template <typename T>
class ConcurrentBucket {
  using KeyExtractionCallback = void (*)(const T&, void*);

 public:
  using BucketType = T;

  ConcurrentBucket(const T& t, KeyExtractionCallback extract_key) {
    extract_key(t, &buf_);
  }

  // for HashTable that don't use KeyValuePair (i.e. *HashSets), the key
  // and the value are the same.
  const T* key() const { return reinterpret_cast<const T*>(&buf_); }
  const T* value() const { return key(); }
  const T* bucket() const { return key(); }

 private:
  // Alignment is needed for atomic accesses to |buf_| and to assure |buf_|
  // can be accessed the same as objects of type T
  static constexpr size_t boundary = std::max(alignof(T), sizeof(size_t));
  alignas(boundary) char buf_[sizeof(T)];
};

template <typename Key, typename Value>
class ConcurrentBucket<KeyValuePair<Key, Value>> {
  using KeyExtractionCallback = void (*)(const KeyValuePair<Key, Value>&,
                                         void*);

 public:
  using BucketType = ConcurrentBucket;

  ConcurrentBucket(const KeyValuePair<Key, Value>& pair,
                   KeyExtractionCallback extract_key)
      : value_(&pair.value) {
    extract_key(pair, &buf_);
  }

  const Key* key() const { return reinterpret_cast<const Key*>(&buf_); }
  const Value* value() const { return value_; }
  const ConcurrentBucket* bucket() const { return this; }

 private:
  // Alignment is needed for atomic accesses to |buf_| and to assure |buf_|
  // can be accessed the same as objects of type Key
  static constexpr size_t boundary = std::max(alignof(Key), sizeof(size_t));
  alignas(boundary) char buf_[sizeof(Key)];
  const Value* value_;
};

}  // namespace internal

template <WTF::WeakHandlingFlag weak_handling, typename Table>
struct TraceHashTableBackingInCollectionTrait {
  using Value = typename Table::ValueType;
  using Traits = typename Table::ValueTraits;
  using Extractor = typename Table::ExtractorType;

  static void Trace(blink::Visitor* visitor, const void* self) {
    static_assert(IsTraceable<Value>::value || WTF::IsWeak<Value>::value,
                  "Table should not be traced");
    const Value* array = reinterpret_cast<const Value*>(self);
    const size_t length =
        cppgc::subtle::
            ObjectSizeTrait<const blink::HeapHashTableBacking<Table>>::GetSize(
                *reinterpret_cast<const blink::HeapHashTableBacking<Table>*>(
                    self)) /
        sizeof(Value);
    for (size_t i = 0; i < length; ++i) {
      if constexpr (Traits::kCanTraceConcurrently) {
        internal::ConcurrentBucket<Value> concurrent_bucket(
            array[i], Extractor::ExtractKeyToMemory);
        if (!WTF::IsHashTraitsEmptyOrDeletedValue<
                typename Table::KeyTraitsType>(*concurrent_bucket.key())) {
          blink::TraceCollectionIfEnabled<
              weak_handling,
              typename internal::ConcurrentBucket<Value>::BucketType,
              Traits>::Trace(visitor, concurrent_bucket.bucket());
        }
      } else {
        // Use single-threaded tracing in case we don't support concurrent
        // tracing. For GC semantics this could use the `ConcurrentBucket` as
        // well. We simply use the bucket in the data structure though to avoid
        // copying possibly ASAN-poisened fields. Such fields can exist in keys
        // in form of an `std::string` that uses container annotations to detect
        // OOB. A side effect is that we also avoid copying the key.
        if (!WTF::IsHashTraitsEmptyOrDeletedValue<
                typename Table::KeyTraitsType>(
                Extractor::ExtractKey(array[i]))) {
          blink::TraceCollectionIfEnabled<weak_handling, Value, Traits>::Trace(
              visitor, &array[i]);
        }
      }
    }
  }
};

template <typename Table>
struct TraceInCollectionTrait<kNoWeakHandling,
                              blink::HeapHashTableBacking<Table>,
                              void> {
  static void Trace(blink::Visitor* visitor, const void* self) {
    TraceHashTableBackingInCollectionTrait<kNoWeakHandling, Table>::Trace(
        visitor, self);
  }
};

template <typename Table>
struct TraceInCollectionTrait<kWeakHandling,
                              blink::HeapHashTableBacking<Table>,
                              void> {
  static void Trace(blink::Visitor* visitor, const void* self) {
    TraceHashTableBackingInCollectionTrait<kWeakHandling, Table>::Trace(visitor,
                                                                        self);
  }
};

template <typename Key, typename Value, typename Traits>
struct TraceInCollectionTrait<
    kNoWeakHandling,
    internal::ConcurrentBucket<KeyValuePair<Key, Value>>,
    Traits> {
  static void Trace(
      blink::Visitor* visitor,
      const internal::ConcurrentBucket<KeyValuePair<Key, Value>>& self) {
    blink::internal::KeyValuePairInCollectionTrait<kNoWeakHandling, Key, Value,
                                                   Traits>::Trace(visitor,
                                                                  self.key(),
                                                                  self.value());
  }
};

template <typename Key, typename Value, typename Traits>
struct TraceInCollectionTrait<
    kWeakHandling,
    internal::ConcurrentBucket<KeyValuePair<Key, Value>>,
    Traits> {
  static void Trace(
      blink::Visitor* visitor,
      const internal::ConcurrentBucket<KeyValuePair<Key, Value>>& self) {
    blink::internal::KeyValuePairInCollectionTrait<kWeakHandling, Key, Value,
                                                   Traits>::Trace(visitor,
                                                                  self.key(),
                                                                  self.value());
  }
};

template <typename T>
struct IsWeak<internal::ConcurrentBucket<T>> : IsWeak<T> {};

template <typename T>
struct IsTraceable<internal::ConcurrentBucket<T>> : IsTraceable<T> {};

}  // namespace WTF

namespace cppgc {

template <typename Table>
struct SpaceTrait<
    blink::HeapHashTableBacking<Table>,
    std::enable_if_t<blink::internal::CompactionTraits<
        blink::HeapHashTableBacking<Table>>::SupportsCompaction()>> {
  using Space = blink::CompactableHeapHashTableBackingSpace;
};

// Custom allocation accounts for inlined storage of the actual elements of the
// backing table.
template <typename Table>
class MakeGarbageCollectedTrait<blink::HeapHashTableBacking<Table>>
    : public MakeGarbageCollectedTraitBase<blink::HeapHashTableBacking<Table>> {
 public:
  using Backing = blink::HeapHashTableBacking<Table>;

  template <typename... Args>
  static Backing* Call(AllocationHandle& handle, size_t num_elements) {
    static_assert(
        !std::is_polymorphic<blink::HeapHashTableBacking<Table>>::value,
        "HeapHashTableBacking must not be polymorphic as it is converted to a "
        "raw array of buckets for certain operation");
    CHECK_GT(num_elements, 0u);
    // Allocate automatically considers the custom space via SpaceTrait.
    void* memory = MakeGarbageCollectedTraitBase<Backing>::Allocate(
        handle, sizeof(typename Table::ValueType) * num_elements);
    Backing* object = ::new (memory) Backing();
    MakeGarbageCollectedTraitBase<Backing>::MarkObjectAsFullyConstructed(
        object);
    return object;
  }
};

template <typename Table>
struct TraceTrait<blink::HeapHashTableBacking<Table>> {
  using Backing = blink::HeapHashTableBacking<Table>;
  using Traits = typename Table::ValueTraits;
  using ValueType = typename Table::ValueTraits::TraitType;

  static TraceDescriptor GetTraceDescriptor(const void* self) {
    return {self, Trace<WTF::kNoWeakHandling>};
  }

  static TraceDescriptor GetWeakTraceDescriptor(const void* self) {
    return GetWeakTraceDescriptorImpl<ValueType>::GetWeakTraceDescriptor(self);
  }

  template <WTF::WeakHandlingFlag weak_handling = WTF::kNoWeakHandling>
  static void Trace(Visitor* visitor, const void* self) {
    if (!Traits::kCanTraceConcurrently && self) {
      if (visitor->DeferTraceToMutatorThreadIfConcurrent(
              self, &Trace<weak_handling>,
              cppgc::subtle::ObjectSizeTrait<const Backing>::GetSize(
                  *reinterpret_cast<const Backing*>(self)))) {
        return;
      }
    }

    static_assert(
        WTF::IsTraceable<ValueType>::value || WTF::IsWeak<ValueType>::value,
        "T should not be traced");
    WTF::TraceInCollectionTrait<weak_handling, Backing, void>::Trace(visitor,
                                                                     self);
  }

 private:
  // Default setting for HashTable is without weak trace descriptor.
  template <typename ValueType>
  struct GetWeakTraceDescriptorImpl {
    static TraceDescriptor GetWeakTraceDescriptor(const void* self) {
      return {self, nullptr};
    }
  };

  // Specialization for WTF::KeyValuePair, which is default bucket storage type.
  template <typename K, typename V>
  struct GetWeakTraceDescriptorImpl<WTF::KeyValuePair<K, V>> {
    static TraceDescriptor GetWeakTraceDescriptor(const void* backing) {
      return GetWeakTraceDescriptorKVPImpl<K, V>::GetWeakTraceDescriptor(
          backing);
    }

    // Default setting for KVP without ephemeron semantics.
    template <typename KeyType,
              typename ValueType,
              bool ephemeron_semantics = (WTF::IsWeak<KeyType>::value &&
                                          !WTF::IsWeak<ValueType>::value &&
                                          WTF::IsTraceable<ValueType>::value) ||
                                         (WTF::IsWeak<ValueType>::value &&
                                          !WTF::IsWeak<KeyType>::value &&
                                          WTF::IsTraceable<KeyType>::value)>
    struct GetWeakTraceDescriptorKVPImpl {
      static TraceDescriptor GetWeakTraceDescriptor(const void* backing) {
        return {backing, nullptr};
      }
    };

    // Specialization for KVP with ephemeron semantics.
    template <typename KeyType, typename ValueType>
    struct GetWeakTraceDescriptorKVPImpl<KeyType, ValueType, true> {
      static TraceDescriptor GetWeakTraceDescriptor(const void* backing) {
        return {backing, Trace<WTF::kWeakHandling>};
      }
    };
  };
};

}  // namespace cppgc

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_COLLECTION_SUPPORT_HEAP_HASH_TABLE_BACKING_H_
