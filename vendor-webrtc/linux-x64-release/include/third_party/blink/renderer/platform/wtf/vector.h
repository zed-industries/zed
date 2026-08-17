/*
 *  Copyright (C) 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Library General Public
 *  License as published by the Free Software Foundation; either
 *  version 2 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Library General Public License for more details.
 *
 *  You should have received a copy of the GNU Library General Public License
 *  along with this library; see the file COPYING.LIB.  If not, write to
 *  the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 *  Boston, MA 02110-1301, USA.
 *
 */

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_VECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_VECTOR_H_

#include <string.h>

#include <algorithm>
#include <concepts>
#include <functional>
#include <initializer_list>
#include <iterator>
#include <ranges>
#include <type_traits>
#include <utility>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/checked_iterators.h"
#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "base/numerics/safe_conversions.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partition_allocator.h"
#include "third_party/blink/renderer/platform/wtf/assertions.h"
#include "third_party/blink/renderer/platform/wtf/atomic_operations.h"
#include "third_party/blink/renderer/platform/wtf/construct_traits.h"
#include "third_party/blink/renderer/platform/wtf/container_annotations.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"  // For default Vector template parameters.
#include "third_party/blink/renderer/platform/wtf/hash_table_deleted_value_type.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

// For ASAN builds, disable inline buffers completely as they cause various
// issues.
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
#define INLINE_CAPACITY 0
#else
#define INLINE_CAPACITY InlineCapacity
#endif

namespace WTF {
template <typename T, wtf_size_t InlineCapacity, typename Allocator>
class Vector;
}

namespace WTF {

#if defined(MEMORY_TOOL_REPLACES_ALLOCATOR)
// The allocation pool for nodes is one big chunk that ASAN has no insight
// into, so it can cloak errors. Make it as small as possible to force nodes
// to be allocated individually where ASAN can see them.
static const wtf_size_t kInitialVectorSize = 1;
#else
static const wtf_size_t kInitialVectorSize = 4;
#endif

template <typename T, wtf_size_t inlineBuffer, typename Allocator>
class Deque;

//
// Vector Traits
//

// Bunch of traits for Vector are defined here, with which you can customize
// Vector's behavior. In most cases the default traits are appropriate, so you
// usually don't have to specialize those traits by yourself.
//
// The behavior of the implementation below can be controlled by VectorTraits.
// If you want to change the behavior of your type, take a look at VectorTraits
// (defined in VectorTraits.h), too.

// Tracing assumes the entire backing store is safe to access. To guarantee
// that, tracing a backing store starts by marking the whole backing store
// capacity as accessible. With concurrent marking enabled, annotating size
// changes could conflict with marking the whole store as accessible, causing
// a race.
#if defined(ADDRESS_SANITIZER)
#define MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, buffer, capacity,      \
                                           old_size, new_size)               \
  if (Allocator::kIsGarbageCollected && Allocator::IsIncrementalMarking()) { \
    ANNOTATE_CHANGE_SIZE(buffer, capacity, 0, capacity);                     \
  } else {                                                                   \
    ANNOTATE_CHANGE_SIZE(buffer, capacity, old_size, new_size)               \
  }
#define MARKING_AWARE_ANNOTATE_NEW_BUFFER(Allocator, buffer, capacity, size) \
  if (Allocator::kIsGarbageCollected && Allocator::IsIncrementalMarking()) { \
    ANNOTATE_NEW_BUFFER(buffer, capacity, capacity);                         \
  } else {                                                                   \
    ANNOTATE_NEW_BUFFER(buffer, capacity, size)                              \
  }
#else
#define MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, buffer, capacity, \
                                           old_size, new_size)          \
  ANNOTATE_CHANGE_SIZE(buffer, capacity, old_size, new_size)
#define MARKING_AWARE_ANNOTATE_NEW_BUFFER(Allocator, buffer, capacity, size) \
  ANNOTATE_NEW_BUFFER(buffer, capacity, size)
#endif  // defined(ADDRESS_SANITIZER)

template <typename T>
struct VectorElementComparer {
  STATIC_ONLY(VectorElementComparer);
  template <typename U>
  static bool CompareElement(const T& left, const U& right) {
    return left == right;
  }
};

template <typename T>
struct VectorElementComparer<std::unique_ptr<T>> {
  STATIC_ONLY(VectorElementComparer);
  template <typename U>
  static bool CompareElement(const std::unique_ptr<T>& left, const U& right) {
    return left.get() == right;
  }
};

// `VectorOperationOrigin` tracks the origin of a vector operation. This is
// needed for the Vector specialization of `HeapAllocator` which is used for
// garbage-collected objects.
//
// The general idea is that during construction of a Vector write barriers can
// be omitted as objects are allocated unmarked and the GC would thus still
// process such objects. Conservative GC is unaffected and would find the
// objects through the stack scan.
//
// This usually applies to storage in the object itself, i.e., inline capacity.
// For Vector it even applies to out-of-line backings as long as those also omit
// the write barrier as they only are referred to from the Vector itself.
enum class VectorOperationOrigin {
  // A regular modification that's always safe.
  kRegularModification,
  // A modification from a constructor that's only safe when being in
  // construction and also requires that the backing stores is modified (set)
  // with the same origin.
  kConstruction,
};

// A collection of all the traits used by Vector. This is basically an
// implementation detail of Vector, and you probably don't want to change this.
// If you want to customize Vector's behavior, you should specialize
// VectorTraits instead (defined in VectorTraits.h).
template <typename T, typename Allocator>
struct VectorTypeOperations {
  STATIC_ONLY(VectorTypeOperations);

  using ConstructTraits = WTF::ConstructTraits<T, VectorTraits<T>, Allocator>;

  static void Destruct(T* begin, T* end) {
    if constexpr (VectorTraits<T>::kNeedsDestruction) {
      for (T* cur = begin; cur != end; ++cur)
        cur->~T();
    }
  }

  static void Initialize(T* begin, T* end) {
    if constexpr (VectorTraits<T>::kCanInitializeWithMemset) {
      size_t size =
          reinterpret_cast<char*>(end) - reinterpret_cast<char*>(begin);
      if constexpr (!Allocator::kIsGarbageCollected || !IsTraceable<T>::value) {
        if (size != 0) {
          // NOLINTNEXTLINE(bugprone-undefined-memory-manipulation)
          memset(begin, 0, size);
        }
      } else {
        AtomicMemzero(begin, size);
      }
    } else {
      for (T* cur = begin; cur != end; ++cur)
        ConstructTraits::Construct(cur);
    }
  }

  static void Move(T* const src,
                   T* const src_end,
                   T* const dst,
                   VectorOperationOrigin origin) {
    if (!src || !dst) [[unlikely]] {
      return;
    }
    if constexpr (!VectorTraits<T>::kCanMoveWithMemcpy) {
      if (origin == VectorOperationOrigin::kConstruction) {
        for (T *s = src, *d = dst; s != src_end; ++s, ++d) {
          ConstructTraits::Construct(d, std::move(*s));
          s->~T();
        }
      } else {
        for (T *s = src, *d = dst; s != src_end; ++s, ++d) {
          ConstructTraits::ConstructAndNotifyElement(d, std::move(*s));
          s->~T();
        }
      }
    } else if constexpr (Allocator::kIsGarbageCollected &&
                         IsTraceable<T>::value) {
      static_assert(VectorTraits<T>::kCanMoveWithMemcpy);
      AtomicWriteMemcpy(dst, src,
                        reinterpret_cast<const char*>(src_end) -
                            reinterpret_cast<const char*>(src));
      if (origin != VectorOperationOrigin::kConstruction) {
        // SAFETY: TODO(359904345): VectorTypeOperations should operate on spans.
        base::span<T> UNSAFE_BUFFERS(
            elements(dst, static_cast<size_t>(src_end - src)));
        ConstructTraits::NotifyNewElements(elements);
      }
    } else {
      static_assert(VectorTraits<T>::kCanMoveWithMemcpy);
      // NOLINTNEXTLINE(bugprone-undefined-memory-manipulation)
      memcpy(dst, src,
             reinterpret_cast<const char*>(src_end) -
                 reinterpret_cast<const char*>(src));
    }
  }

  static void MoveOverlapping(T* const src,
                              T* const src_end,
                              T* const dst,
                              VectorOperationOrigin origin) {
    if (!src || !dst) [[unlikely]] {
      return;
    }
    if constexpr (!VectorTraits<T>::kCanMoveWithMemcpy) {
      if (dst < src) {
        Move(src, src_end, dst, origin);
      } else if (dst > src) {
        T* s = src_end - 1;
        T* d = dst + (s - src);
        if (origin == VectorOperationOrigin::kConstruction) {
          for (; s >= src; --s, --d) {
            ConstructTraits::Construct(d, std::move(*s));
            s->~T();
          }
        } else {
          for (; s >= src; --s, --d) {
            ConstructTraits::ConstructAndNotifyElement(d, std::move(*s));
            s->~T();
          }
        }
      }
    } else if constexpr (Allocator::kIsGarbageCollected &&
                         IsTraceable<T>::value) {
      static_assert(VectorTraits<T>::kCanMoveWithMemcpy);
      if (dst < src) {
        for (T *s = src, *d = dst; s < src_end; ++s, ++d)
          AtomicWriteMemcpy<sizeof(T), alignof(T)>(d, s);
      } else if (dst > src) {
        T* s = src_end - 1;
        T* d = dst + (s - src);
        for (; s >= src; --s, --d)
          AtomicWriteMemcpy<sizeof(T), alignof(T)>(d, s);
      }
      if (origin != VectorOperationOrigin::kConstruction) {
        // SAFETY: TODO(359904345): VectorTypeOperations should operate on spans.
        base::span<T> UNSAFE_BUFFERS(
            elements(dst, static_cast<size_t>(src_end - src)));
        ConstructTraits::NotifyNewElements(elements);
      }
    } else {
      static_assert(VectorTraits<T>::kCanMoveWithMemcpy);
      // NOLINTNEXTLINE(bugprone-undefined-memory-manipulation)
      memmove(dst, src,
              reinterpret_cast<const char*>(src_end) -
                  reinterpret_cast<const char*>(src));
    }
  }

  static void Swap(T* const src,
                   T* const src_end,
                   T* const dst,
                   VectorOperationOrigin src_origin) {
    if constexpr (!VectorTraits<T>::kCanMoveWithMemcpy) {
      std::swap_ranges(src, src_end, dst);
    } else if constexpr (Allocator::kIsGarbageCollected &&
                         IsTraceable<T>::value) {
      static_assert(VectorTraits<T>::kCanMoveWithMemcpy);
      constexpr size_t boundary = std::max(alignof(T), sizeof(size_t));
      alignas(boundary) char buf[sizeof(T)];
      for (T *s = src, *d = dst; s < src_end; ++s, ++d) {
        // NOLINTNEXTLINE(bugprone-undefined-memory-manipulation)
        memcpy(buf, d, sizeof(T));
        AtomicWriteMemcpy<sizeof(T), alignof(T)>(d, s);
        AtomicWriteMemcpy<sizeof(T), alignof(T)>(s, buf);
      }
      const size_t len = src_end - src;
      if (src_origin != VectorOperationOrigin::kConstruction) {
        // SAFETY: TODO(359904345): VectorTypeOperations should operate on spans.
        base::span<T> UNSAFE_BUFFERS(elements(src, len));
        ConstructTraits::NotifyNewElements(elements);
      }
      // SAFETY: TODO(359904345): VectorTypeOperations should operate on spans.
      base::span<T> UNSAFE_BUFFERS(elements(dst, len));
      ConstructTraits::NotifyNewElements(elements);
    } else {
      static_assert(VectorTraits<T>::kCanMoveWithMemcpy);
      std::swap_ranges(reinterpret_cast<char*>(src),
                       reinterpret_cast<char*>(src_end),
                       reinterpret_cast<char*>(dst));
    }
  }

  static void Copy(const T* src,
                   const T* src_end,
                   T* dst,
                   VectorOperationOrigin origin) {
    if constexpr (!VectorTraits<T>::kCanCopyWithMemcpy) {
      std::copy(src, src_end, dst);
    } else if constexpr (Allocator::kIsGarbageCollected &&
                         IsTraceable<T>::value) {
      static_assert(VectorTraits<T>::kCanCopyWithMemcpy);
      AtomicWriteMemcpy(dst, src,
                        reinterpret_cast<const char*>(src_end) -
                            reinterpret_cast<const char*>(src));
      if (origin != VectorOperationOrigin::kConstruction) {
        // SAFETY: TODO(359904345): VectorTypeOperations should operate on spans.
        base::span<T> UNSAFE_BUFFERS(
            elements(dst, static_cast<size_t>(src_end - src)));
        ConstructTraits::NotifyNewElements(elements);
      }
    } else {
      static_assert(VectorTraits<T>::kCanCopyWithMemcpy);
      // NOLINTNEXTLINE(bugprone-undefined-memory-manipulation)
      if (src != src_end) {
        memcpy(dst, src,
               reinterpret_cast<const char*>(src_end) -
                   reinterpret_cast<const char*>(src));
      }
    }
  }

  template <typename U>
  static void UninitializedCopy(const U* src,
                                const U* src_end,
                                T* dst,
                                VectorOperationOrigin origin) {
    if (!dst || !src) [[unlikely]] {
      return;
    }
    if constexpr (std::is_same_v<T, U> && VectorTraits<T>::kCanCopyWithMemcpy) {
      Copy(src, src_end, dst, origin);
    } else {
      UninitializedTransform(src, src_end, dst, origin, std::identity());
    }
  }

  template <typename InputIterator, typename Proj>
  static void UninitializedTransform(InputIterator src,
                                     InputIterator src_end,
                                     T* dst,
                                     VectorOperationOrigin origin,
                                     Proj proj) {
    if (origin == VectorOperationOrigin::kConstruction) {
      while (src != src_end) {
        ConstructTraits::Construct(
            dst, std::invoke(proj, std::forward<decltype(*src)>(*src)));
        ++dst;
        ++src;
      }
    } else {
      while (src != src_end) {
        ConstructTraits::ConstructAndNotifyElement(
            dst, std::invoke(proj, std::forward<decltype(*src)>(*src)));
        ++dst;
        ++src;
      }
    }
  }

  static void UninitializedFill(T* dst,
                                T* dst_end,
                                const T& val,
                                VectorOperationOrigin origin) {
    if (!dst) [[unlikely]] {
      return;
    }
    if constexpr (VectorTraits<T>::kCanFillWithMemset) {
      static_assert(sizeof(T) == sizeof(char), "size of type should be one");
      static_assert(!Allocator::kIsGarbageCollected,
                    "memset is unsupported for garbage-collected vectors.");
      memset(dst, static_cast<unsigned char>(val), dst_end - dst);
    } else if (origin == VectorOperationOrigin::kConstruction) {
      while (dst != dst_end) {
        ConstructTraits::Construct(dst, T(val));
        ++dst;
      }
    } else {
      while (dst != dst_end) {
        ConstructTraits::ConstructAndNotifyElement(dst, T(val));
        ++dst;
      }
    }
  }

  static bool Compare(const T* a, const T* b, size_t size) {
    DCHECK(a);
    DCHECK(b);
    if constexpr (VectorTraits<T>::kCanCompareWithMemcmp)
      return memcmp(a, b, sizeof(T) * size) == 0;
    else
      return std::equal(a, a + size, b);
  }

  template <typename U>
  static bool CompareElement(const T& left, U&& right) {
    return VectorElementComparer<T>::CompareElement(left,
                                                    std::forward<U>(right));
  }
};

//
// VectorBuffer
//

// VectorBuffer is an implementation detail of Vector and Deque. It manages
// Vector's underlying buffer, and does operations like allocation or
// expansion.
//
// Not meant for general consumption.

template <typename T, typename Allocator>
class VectorBufferBase {
  DISALLOW_NEW();

 public:
  VectorBufferBase(VectorBufferBase&&) = default;
  VectorBufferBase& operator=(VectorBufferBase&&) = default;

  void AllocateBuffer(wtf_size_t new_capacity, VectorOperationOrigin origin) {
    AllocateBufferNoBarrier(new_capacity);
    if (origin != VectorOperationOrigin::kConstruction) {
      Allocator::BackingWriteBarrier(&buffer_);
    }
  }

  size_t AllocationSize(size_t capacity) const {
    return Allocator::template QuantizedSize<T>(capacity);
  }

  T* Buffer() { return buffer_; }
  const T* Buffer() const { return buffer_; }
  wtf_size_t capacity() const { return capacity_; }

  void ClearUnusedSlots(T* from, T* to) {
    if constexpr (NeedsToClearUnusedSlots()) {
      AtomicMemzero(reinterpret_cast<void*>(from), sizeof(T) * (to - from));
    }
  }

  void CheckUnusedSlots(const T* from, const T* to) {
#if DCHECK_IS_ON() && !defined(ANNOTATE_CONTIGUOUS_CONTAINER)
    if constexpr (NeedsToClearUnusedSlots()) {
      const unsigned char* unused_area =
          reinterpret_cast<const unsigned char*>(from);
      const unsigned char* end_address =
          reinterpret_cast<const unsigned char*>(to);
      DCHECK_GE(end_address, unused_area);
      for (; unused_area != end_address; ++unused_area)
        DCHECK(!*unused_area);
    }
#endif
  }

  void AcquireBuffer(VectorBufferBase&& other) {
    AsAtomicPtr(&buffer_)->store(other.buffer_, std::memory_order_relaxed);
    Allocator::BackingWriteBarrier(&buffer_);
    capacity_ = other.capacity_;
  }

  // |end| is exclusive, a la STL.
  struct OffsetRange final {
    OffsetRange() : begin(0), end(0) {}
    explicit OffsetRange(wtf_size_t begin, wtf_size_t end)
        : begin(begin), end(end) {
      DCHECK_LE(begin, end);
    }
    bool empty() const { return begin == end; }
    wtf_size_t begin;
    wtf_size_t end;
  };

 protected:
  static VectorBufferBase AllocateTemporaryBuffer(wtf_size_t capacity) {
    VectorBufferBase buffer;
    buffer.AllocateBuffer(capacity, VectorOperationOrigin::kConstruction);
    return buffer;
  }

  VectorBufferBase() : buffer_(nullptr), capacity_(0) {}

  VectorBufferBase(T* buffer, wtf_size_t capacity)
      : buffer_(buffer), capacity_(capacity) {}

  VectorBufferBase(HashTableDeletedValueType value)
      : buffer_(reinterpret_cast<T*>(-1)) {}

  bool IsHashTableDeletedValue() const {
    return buffer_ == reinterpret_cast<T*>(-1);
  }

  const T* BufferSafe() const {
    return AsAtomicPtr(&buffer_)->load(std::memory_order_relaxed);
  }

  void SwapBuffers(VectorBufferBase& other, VectorOperationOrigin this_origin) {
    AtomicWriteSwap(buffer_, other.buffer_);
    std::swap(capacity_, other.capacity_);
    std::swap(size_, other.size_);
    if (this_origin != VectorOperationOrigin::kConstruction) {
      Allocator::BackingWriteBarrier(&buffer_);
    }
    Allocator::BackingWriteBarrier(&other.buffer_);
  }

  T* buffer_;
  wtf_size_t capacity_;
  wtf_size_t size_;

 private:
  static constexpr bool NeedsToClearUnusedSlots() {
    // Tracing and finalization access all slots of a vector backing. In case
    // there's work to be done there unused slots should be cleared.
    return Allocator::kIsGarbageCollected &&
           (IsTraceable<T>::value || VectorTraits<T>::kNeedsDestruction);
  }

  void AllocateBufferNoBarrier(wtf_size_t new_capacity) {
    DCHECK(new_capacity);
    DCHECK_LE(new_capacity,
              Allocator::template MaxElementCountInBackingStore<T>());
    size_t size_to_allocate = AllocationSize(new_capacity);
    AsAtomicPtr(&buffer_)->store(
        Allocator::template AllocateVectorBacking<T>(size_to_allocate),
        std::memory_order_relaxed);
    capacity_ = static_cast<wtf_size_t>(size_to_allocate / sizeof(T));
  }
};

template <typename T,
          wtf_size_t InlineCapacity,
          typename Allocator = PartitionAllocator>
class VectorBuffer;

template <typename T, typename Allocator>
class VectorBuffer<T, 0, Allocator> : protected VectorBufferBase<T, Allocator> {
 private:
  using Base = VectorBufferBase<T, Allocator>;

 public:
  using OffsetRange = typename Base::OffsetRange;

  VectorBuffer() = default;

  explicit VectorBuffer(wtf_size_t capacity) {
    // Calling malloc(0) might take a lock and may actually do an allocation
    // on some systems.
    if (capacity) {
      AllocateBuffer(capacity, VectorOperationOrigin::kConstruction);
    }
  }

  explicit VectorBuffer(HashTableDeletedValueType value) : Base(value) {}

  void Destruct() {
    DeallocateBuffer(buffer_);
    buffer_ = nullptr;
  }

  void DeallocateBuffer(T* buffer_to_deallocate) {
    Allocator::FreeVectorBacking(buffer_to_deallocate);
  }

  bool ExpandBuffer(wtf_size_t new_capacity) {
    size_t size_to_allocate = AllocationSize(new_capacity);
    if (buffer_ && Allocator::ExpandVectorBacking(buffer_, size_to_allocate)) {
      capacity_ = static_cast<wtf_size_t>(size_to_allocate / sizeof(T));
      return true;
    }
    return false;
  }

  inline bool ShrinkBuffer(wtf_size_t new_capacity) {
    DCHECK(buffer_);
    DCHECK_LT(new_capacity, capacity());
    size_t size_to_allocate = AllocationSize(new_capacity);
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
    ANNOTATE_DELETE_BUFFER(buffer_, capacity_, size_);
#endif
    bool succeeded = false;
    if (Allocator::ShrinkVectorBacking(buffer_, AllocationSize(capacity()),
                                       size_to_allocate)) {
      capacity_ = static_cast<wtf_size_t>(size_to_allocate / sizeof(T));
      succeeded = true;
    }
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
    MARKING_AWARE_ANNOTATE_NEW_BUFFER(Allocator, buffer_, capacity_, size_);
#endif
    return succeeded;
  }

  void ResetBufferPointer() {
    AsAtomicPtr(&buffer_)->store(nullptr, std::memory_order_relaxed);
    capacity_ = 0;
  }

  // See the other specialization for the meaning of |thisHole| and |otherHole|.
  // They are irrelevant in this case.
  void SwapVectorBuffer(VectorBuffer<T, 0, Allocator>& other,
                        OffsetRange this_hole,
                        OffsetRange other_hole,
                        VectorOperationOrigin this_origin) {
    Base::SwapBuffers(other, this_origin);
  }

  using Base::AllocateBuffer;
  using Base::AllocationSize;

  using Base::Buffer;
  using Base::capacity;

  using Base::ClearUnusedSlots;
  using Base::CheckUnusedSlots;

  bool HasOutOfLineBuffer() const {
    // When InlineCapacity is 0 we have an out of line buffer if we have a
    // buffer.
    return IsOutOfLineBuffer(Buffer());
  }

  T** BufferSlot() { return &buffer_; }
  const T* const* BufferSlot() const { return &buffer_; }

 protected:
  using Base::BufferSafe;

  using Base::size_;

  bool IsOutOfLineBuffer(const T* buffer) const { return buffer; }

 private:
  using Base::buffer_;
  using Base::capacity_;
};

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
class VectorBuffer : protected VectorBufferBase<T, Allocator> {
 private:
  using Base = VectorBufferBase<T, Allocator>;

 public:
  using OffsetRange = typename Base::OffsetRange;

  VectorBuffer() : Base(InlineBuffer(), InlineCapacity) { InitInlinedBuffer(); }

  explicit VectorBuffer(HashTableDeletedValueType value) : Base(value) {
    InitInlinedBuffer();
  }
  bool IsHashTableDeletedValue() const {
    return Base::IsHashTableDeletedValue();
  }

  explicit VectorBuffer(wtf_size_t capacity)
      : Base(InlineBuffer(), InlineCapacity) {
    InitInlinedBuffer();
    if (capacity > InlineCapacity) {
      Base::AllocateBuffer(capacity, VectorOperationOrigin::kConstruction);
    }
  }

  VectorBuffer(const VectorBuffer&) = delete;
  VectorBuffer& operator=(const VectorBuffer&) = delete;

  void Destruct() {
    DeallocateBuffer(buffer_);
    buffer_ = nullptr;
  }

  NOINLINE void ReallyDeallocateBuffer(T* buffer_to_deallocate) {
    Allocator::FreeVectorBacking(buffer_to_deallocate);
  }

  void DeallocateBuffer(T* buffer_to_deallocate) {
    if (buffer_to_deallocate != InlineBuffer()) [[unlikely]] {
      ReallyDeallocateBuffer(buffer_to_deallocate);
    }
  }

  bool ExpandBuffer(wtf_size_t new_capacity) {
    DCHECK_GT(new_capacity, InlineCapacity);
    if (buffer_ == InlineBuffer())
      return false;

    size_t size_to_allocate = AllocationSize(new_capacity);
    if (buffer_ && Allocator::ExpandVectorBacking(buffer_, size_to_allocate)) {
      capacity_ = static_cast<wtf_size_t>(size_to_allocate / sizeof(T));
      return true;
    }
    return false;
  }

  inline bool ShrinkBuffer(wtf_size_t new_capacity) {
    DCHECK(buffer_);
    DCHECK_LT(new_capacity, capacity());
    if (new_capacity <= InlineCapacity) {
      // We need to switch to inlineBuffer.  Vector::ShrinkCapacity will
      // handle it.
      return false;
    }
    DCHECK_NE(buffer_, InlineBuffer());
    size_t new_size = AllocationSize(new_capacity);
    bool succeeded = false;
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
    ANNOTATE_DELETE_BUFFER(buffer_, capacity_, size_);
#endif
    if (Allocator::ShrinkVectorBacking(buffer_, AllocationSize(capacity()),
                                       new_size)) {
      capacity_ = static_cast<wtf_size_t>(new_size / sizeof(T));
      succeeded = true;
    }
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
    MARKING_AWARE_ANNOTATE_NEW_BUFFER(Allocator, buffer_, capacity_, size_);
#endif
    return succeeded;
  }

  void ResetBufferPointer() {
    AsAtomicPtr(&buffer_)->store(InlineBuffer(), std::memory_order_relaxed);
    capacity_ = InlineCapacity;
  }

  void AllocateBuffer(wtf_size_t new_capacity, VectorOperationOrigin origin) {
    // FIXME: This should DCHECK(!buffer_) to catch misuse/leaks.
    if (new_capacity > InlineCapacity) {
      Base::AllocateBuffer(new_capacity, origin);
    } else {
      ResetBufferPointer();
    }
  }

  size_t AllocationSize(size_t capacity) const {
    if (capacity <= InlineCapacity) {
      return kInlineBufferSize;
    }
    return Base::AllocationSize(capacity);
  }

  // Swap two vector buffers, both of which have the same non-zero inline
  // capacity.
  //
  // If the data is in an out-of-line buffer, we can just pass the pointers
  // across the two buffers.  If the data is in an inline buffer, we need to
  // either swap or move each element, depending on whether each slot is
  // occupied or not.
  //
  // Further complication comes from the fact that VectorBuffer is also used as
  // the backing store of a Deque.  Deque allocates the objects like a ring
  // buffer, so there may be a "hole" (unallocated region) in the middle of the
  // buffer. This function assumes elements in a range [buffer_, buffer_ +
  // size_) are all allocated except for elements within |thisHole|. The same
  // applies for |other.buffer_| and |otherHole|.
  void SwapVectorBuffer(VectorBuffer<T, InlineCapacity, Allocator>& other,
                        OffsetRange this_hole,
                        OffsetRange other_hole,
                        VectorOperationOrigin this_origin) {
    using TypeOperations = VectorTypeOperations<T, Allocator>;

    if (Buffer() != InlineBuffer() && other.Buffer() != other.InlineBuffer()) {
      Base::SwapBuffers(other, this_origin);
      return;
    }

    Allocator::EnterGCForbiddenScope();

    // Otherwise, we at least need to move some elements from one inline buffer
    // to another.
    //
    // Terminology: "source" is a place from which elements are copied, and
    // "destination" is a place to which elements are copied. thisSource or
    // otherSource can be empty (represented by nullptr) when this range or
    // other range is in an out-of-line buffer.
    //
    // We first record which range needs to get moved and where elements in such
    // a range will go. Elements in an inline buffer will go to the other
    // buffer's inline buffer. Elements in an out-of-line buffer won't move,
    // because we can just swap pointers of out-of-line buffers.
    T* this_source_begin = nullptr;
    wtf_size_t this_source_size = 0;
    T* this_destination_begin = nullptr;
    if (Buffer() == InlineBuffer()) {
      this_source_begin = Buffer();
      this_source_size = size_;
      this_destination_begin = other.InlineBuffer();
      if (!this_hole.empty()) {  // Sanity check.
        DCHECK_LT(this_hole.begin, this_hole.end);
        DCHECK_LE(this_hole.end, this_source_size);
      }
    } else {
      // We don't need the hole information for an out-of-line buffer.
      this_hole.begin = this_hole.end = 0;
    }
    T* other_source_begin = nullptr;
    wtf_size_t other_source_size = 0;
    T* other_destination_begin = nullptr;
    if (other.Buffer() == other.InlineBuffer()) {
      other_source_begin = other.Buffer();
      other_source_size = other.size_;
      other_destination_begin = InlineBuffer();
      if (!other_hole.empty()) {
        DCHECK_LT(other_hole.begin, other_hole.end);
        DCHECK_LE(other_hole.end, other_source_size);
      }
    } else {
      other_hole.begin = other_hole.end = 0;
    }

    // Next, we mutate members and do other bookkeeping. We do pointer swapping
    // (for out-of-line buffers) here if we can. From now on, don't assume
    // buffer() or capacity() maintains their original values.
    std::swap(capacity_, other.capacity_);
    if (this_source_begin &&
        !other_source_begin) {  // Our buffer is inline, theirs is not.
      DCHECK_EQ(Buffer(), InlineBuffer());
      DCHECK_NE(other.Buffer(), other.InlineBuffer());
      ANNOTATE_DELETE_BUFFER(buffer_, InlineCapacity, size_);
      AsAtomicPtr(&buffer_)->store(other.Buffer(), std::memory_order_relaxed);
      AsAtomicPtr(&other.buffer_)
          ->store(other.InlineBuffer(), std::memory_order_relaxed);
      std::swap(size_, other.size_);
      MARKING_AWARE_ANNOTATE_NEW_BUFFER(Allocator, other.buffer_,
                                        InlineCapacity, other.size_);
      if (this_origin != VectorOperationOrigin::kConstruction) {
        Allocator::BackingWriteBarrier(&buffer_);
      }
    } else if (!this_source_begin &&
               other_source_begin) {  // Their buffer is inline, ours is not.
      DCHECK_NE(Buffer(), InlineBuffer());
      DCHECK_EQ(other.Buffer(), other.InlineBuffer());
      ANNOTATE_DELETE_BUFFER(other.buffer_, InlineCapacity, other.size_);
      AsAtomicPtr(&other.buffer_)->store(Buffer(), std::memory_order_relaxed);
      AsAtomicPtr(&buffer_)->store(InlineBuffer(), std::memory_order_relaxed);
      std::swap(size_, other.size_);
      MARKING_AWARE_ANNOTATE_NEW_BUFFER(Allocator, buffer_, InlineCapacity,
                                        size_);
      Allocator::BackingWriteBarrier(&other.buffer_);
    } else {  // Both buffers are inline.
      DCHECK(this_source_begin);
      DCHECK(other_source_begin);
      DCHECK_EQ(Buffer(), InlineBuffer());
      DCHECK_EQ(other.Buffer(), other.InlineBuffer());
      MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, buffer_, InlineCapacity,
                                         size_, other.size_);
      MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, other.buffer_,
                                         InlineCapacity, other.size_, size_);
      std::swap(size_, other.size_);
    }

    // We are ready to move elements. We determine an action for each "section",
    // which is a contiguous range such that all elements in the range are
    // treated similarly.
    wtf_size_t section_begin = 0;
    while (section_begin < InlineCapacity) {
      // To determine the end of this section, we list up all the boundaries
      // where the "occupiedness" may change.
      wtf_size_t section_end = InlineCapacity;
      if (this_source_begin && section_begin < this_source_size)
        section_end = std::min(section_end, this_source_size);
      if (!this_hole.empty() && section_begin < this_hole.begin)
        section_end = std::min(section_end, this_hole.begin);
      if (!this_hole.empty() && section_begin < this_hole.end)
        section_end = std::min(section_end, this_hole.end);
      if (other_source_begin && section_begin < other_source_size)
        section_end = std::min(section_end, other_source_size);
      if (!other_hole.empty() && section_begin < other_hole.begin)
        section_end = std::min(section_end, other_hole.begin);
      if (!other_hole.empty() && section_begin < other_hole.end)
        section_end = std::min(section_end, other_hole.end);

      DCHECK_LT(section_begin, section_end);

      // Is the |sectionBegin|-th element of |thisSource| occupied?
      bool this_occupied = false;
      if (this_source_begin && section_begin < this_source_size) {
        // Yes, it's occupied, unless the position is in a hole.
        if (this_hole.empty() || section_begin < this_hole.begin ||
            section_begin >= this_hole.end)
          this_occupied = true;
      }
      bool other_occupied = false;
      if (other_source_begin && section_begin < other_source_size) {
        if (other_hole.empty() || section_begin < other_hole.begin ||
            section_begin >= other_hole.end)
          other_occupied = true;
      }

      if (this_occupied && other_occupied) {
        // Both occupied; swap them. In this case, one's destination must be the
        // other's source (i.e. both ranges are in inline buffers).
        DCHECK_EQ(this_destination_begin, other_source_begin);
        DCHECK_EQ(other_destination_begin, this_source_begin);
        TypeOperations::Swap(this_source_begin + section_begin,
                             this_source_begin + section_end,
                             other_source_begin + section_begin, this_origin);
      } else if (this_occupied) {
        // Move from ours to theirs.
        TypeOperations::Move(this_source_begin + section_begin,
                             this_source_begin + section_end,
                             this_destination_begin + section_begin,
                             VectorOperationOrigin::kRegularModification);
        Base::ClearUnusedSlots(this_source_begin + section_begin,
                               this_source_begin + section_end);
      } else if (other_occupied) {
        // Move from theirs to ours.
        TypeOperations::Move(other_source_begin + section_begin,
                             other_source_begin + section_end,
                             other_destination_begin + section_begin,
                             this_origin);
        Base::ClearUnusedSlots(other_source_begin + section_begin,
                               other_source_begin + section_end);
      } else {
        // Both empty; nothing to do.
      }

      section_begin = section_end;
    }

    Allocator::LeaveGCForbiddenScope();
  }

  using Base::Buffer;
  using Base::capacity;

  bool HasOutOfLineBuffer() const { return IsOutOfLineBuffer(Buffer()); }

  T** BufferSlot() { return &buffer_; }
  const T* const* BufferSlot() const { return &buffer_; }

 protected:
  using Base::BufferSafe;

  using Base::size_;

  bool IsOutOfLineBuffer(const T* buffer) const {
    return buffer && buffer != InlineBuffer();
  }

 private:
  using Base::buffer_;
  using Base::capacity_;

  static const wtf_size_t kInlineBufferSize = InlineCapacity * sizeof(T);
  T* InlineBuffer() { return unsafe_reinterpret_cast_ptr<T*>(inline_buffer_); }
  const T* InlineBuffer() const {
    return unsafe_reinterpret_cast_ptr<const T*>(inline_buffer_);
  }

  void InitInlinedBuffer() {
    if (Allocator::kIsGarbageCollected) {
      memset(&inline_buffer_, 0, kInlineBufferSize);
    }
  }

  alignas(T) char inline_buffer_[kInlineBufferSize];
  template <typename U, wtf_size_t inlineBuffer, typename V>
  friend class Deque;
};

// UncheckedIteraotr<T> is just a wrapper of a T pointer with no bounds
// checking, and the default iterator implementation of WTF::Vector.
template <typename T>
class UncheckedIterator {
 public:
  using difference_type = std::ptrdiff_t;
  using value_type = std::remove_cv_t<T>;
  using pointer = T*;
  using reference = T&;
  using iterator_category = std::contiguous_iterator_tag;
  using iterator_concept = std::contiguous_iterator_tag;

  constexpr UncheckedIterator() = default;
  explicit UncheckedIterator(T* cur) : current_(cur) {}
  UncheckedIterator(const UncheckedIterator& other) = default;
  // Allow implicit conversion from a base::CheckedContiguousIterator<T>.
  // NOLINTNEXTLINE(google-explicit-constructor)
  UncheckedIterator(const base::CheckedContiguousIterator<T>& other)
      : current_(base::to_address(other)) {}
  ~UncheckedIterator() = default;

  UncheckedIterator& operator=(const UncheckedIterator& other) = default;

  friend constexpr bool operator==(const UncheckedIterator& lhs,
                                   const UncheckedIterator& rhs) {
    return lhs.current_ == rhs.current_;
  }
  friend auto operator<=>(const UncheckedIterator& lhs,
                          const UncheckedIterator& rhs) {
    return lhs.current_ <=> rhs.current_;
  }

  UNSAFE_BUFFER_USAGE UncheckedIterator& operator++() {
    ++current_;
    return *this;
  }
  UNSAFE_BUFFER_USAGE UncheckedIterator operator++(int) {
    auto old = *this;
    ++current_;
    return old;
  }
  UNSAFE_BUFFER_USAGE UncheckedIterator& operator--() {
    --current_;
    return *this;
  }
  UNSAFE_BUFFER_USAGE UncheckedIterator operator--(int) {
    auto old = *this;
    --current_;
    return old;
  }
  UNSAFE_BUFFER_USAGE UncheckedIterator& operator+=(difference_type rhs) {
    current_ += rhs;
    return *this;
  }
  UNSAFE_BUFFER_USAGE UncheckedIterator operator+(difference_type rhs) const {
    auto it = *this;
    it += rhs;
    return it;
  }
  UNSAFE_BUFFER_USAGE friend UncheckedIterator operator+(
      difference_type lhs,
      const UncheckedIterator& rhs) {
    return rhs + lhs;
  }
  UNSAFE_BUFFER_USAGE UncheckedIterator& operator-=(difference_type rhs) {
    current_ -= rhs;
    return *this;
  }
  UNSAFE_BUFFER_USAGE UncheckedIterator operator-(difference_type rhs) const {
    auto it = *this;
    it -= rhs;
    return it;
  }
  friend difference_type operator-(const UncheckedIterator& lhs,
                                   const UncheckedIterator& rhs) {
    return lhs.current_ - rhs.current_;
  }

  T& operator*() const { return *current_; }
  T* operator->() const { return current_; }
  UNSAFE_BUFFER_USAGE T& operator[](difference_type rhs) const {
    return current_[rhs];
  }

  friend std::ostream& operator<<(std::ostream& out,
                                  const UncheckedIterator& rhs) {
    return out << "UncheckedIterator {current_:" << rhs.current_ << "}";
  }

 private:
  // Allow current_ access from UncheckedIterator<U>.
  template <typename>
  friend class UncheckedIterator;

  T* current_ = nullptr;
};

//
// Vector
//

// Vector is a container that works just like std::vector. WTF's Vector has
// several extra functionalities: inline buffer, behavior customization via
// traits, and Oilpan support. Those are explained in the sections below.
//
// Vector is the most basic container, which stores its element in a contiguous
// buffer. The buffer is expanded automatically when necessary. The elements
// are automatically moved to the new buffer. This event is called a
// reallocation. A reallocation takes O(N)-time (N = number of elements), but
// its occurrences are rare, so its time cost should not be significant,
// compared to the time cost of other operations to the vector.
//
// Time complexity of key operations is as follows:
//
//     * Indexed access -- O(1)
//     * Insertion or removal of an element at the end -- amortized O(1)
//     * Other insertion or removal -- O(N)
//     * Swapping with another vector -- O(1)
//
// 1. Iterator invalidation semantics
//
// Vector provides STL-compatible iterators and reverse iterators. Iterators
// are _invalidated_ on certain occasions. Reading an invalidated iterator
// causes undefined behavior.
//
// Iterators are invalidated on the following situations:
//
//     * When a reallocation happens on a vector, all the iterators for that
//       vector will be invalidated.
//     * Some member functions invalidate part of the existing iterators for
//       the vector; see comments on the individual functions.
//     * [Oilpan only] Heap compaction invalidates all the iterators for any
//       HeapVectors. This means you can only store an iterator on stack, as
//       a local variable.
//
// In this context, pointers or references to an element of a Vector are
// essentially equivalent to iterators, in that they also become invalid
// whenever corresponding iterators are invalidated.
//
// 2. Inline buffer
//
// Vectors may have an _inline buffer_. An inline buffer is a storage area
// that is contained in the vector itself, along with other metadata like
// size_. It is used as a storage space when the vector's elements fit in
// that space. If the inline buffer becomes full and further space is
// necessary, an out-of-line buffer is allocated in the heap, and it will
// take over the role of the inline buffer.
//
// The existence of an inline buffer is indicated by non-zero |InlineCapacity|
// template argument. The value represents the number of elements that can be
// stored in the inline buffer. Zero |InlineCapacity| means the vector has no
// inline buffer.
//
// An inline buffer increases the size of the Vector instances, and, in trade
// for that, it gives you several performance benefits, as long as the number
// of elements do not exceed |InlineCapacity|:
//
//     * No heap allocation will be made.
//     * Memory locality will improve.
//
// Generally, having an inline buffer is useful for vectors that (1) are
// frequently accessed or modified, and (2) contain only a few elements at
// most.
//
// 3. Behavior customization
//
// You usually do not need to customize Vector's behavior, since the default
// behavior is appropriate for normal usage. The behavior is controlled by
// VectorTypeOperations traits template above. Read VectorTypeOperations
// and VectorTraits if you want to change the behavior for your types (i.e.
// if you really want faster vector operations).
//
// The default traits basically do the following:
//
//     * Skip constructor call and fill zeros with memset for simple types;
//     * Skip destructor call for simple types;
//     * Copy or move by memcpy for simple types; and
//     * Customize the comparisons for smart pointer types, so you can look
//       up a std::unique_ptr<T> element with a raw pointer, for instance.
//
// 4. Oilpan
//
// If you want to store garbage collected objects in Vector, (1) use HeapVector
// (defined in HeapAllocator.h) instead of Vector, and (2) make sure your
// garbage-collected type is wrapped with Member, like:
//
//     HeapVector<Member<Node>> nodes;
//
// Unlike normal garbage-collected objects, a HeapVector object itself is
// NOT a garbage-collected object, but its backing buffer is allocated in
// Oilpan heap, and it may still carry garbage-collected objects.
//
// Even though a HeapVector object is not garbage-collected, you still need
// to trace it, if you stored it in your class. Also, you can allocate it
// as a local variable. This is useful when you want to build a vector locally
// and put it in an on-heap vector with swap().
//
// Also, heap compaction, which may happen at any time when Blink code is not
// running (i.e. Blink code does not appear in the call stack), may invalidate
// existing iterators for any HeapVectors. So, essentially, you should always
// allocate an iterator on stack (as a local variable), and you should not
// store iterators in another heap object.

// In general, Vector requires destruction.
template <typename T, wtf_size_t InlineCapacity, bool isGced>
inline constexpr bool kVectorNeedsDestructor = true;

// For garbage collection, Vector does not require destruction when there's no
// inline capacity.
template <typename T>
inline constexpr bool kVectorNeedsDestructor<T, 0, true> = false;

// For garbage collection, a Vector with inline capacity conditionally requires
// destruction based on whether the element type itself requires destruction.
//
// However, for now, always return true, as there are many uses of on-stack
// HeapVector with inline capacity that require eager clearing for performance.
//
// Ideally, there should be a different representation for on-stack usages
// which would allow eager clearing for all uses of Vector from stack and avoid
// destructors on heap.
template <typename T, wtf_size_t InlineCapacity>
inline constexpr bool kVectorNeedsDestructor<T, InlineCapacity, true> = true;

template <typename T,
          wtf_size_t InlineCapacity,
          typename Allocator,
          typename Range,
          typename Proj>
concept VectorCanAssignFromRange =
    std::ranges::input_range<Range> && std::ranges::sized_range<Range> &&
    std::indirectly_unary_invocable<Proj, std::ranges::iterator_t<Range>> &&
    // This prevents accidental fallback from the more efficient code paths.
    (!std::is_base_of_v<Vector<T, InlineCapacity, Allocator>,
                        std::decay_t<Range>> ||
     !std::is_same_v<Proj, std::identity>);

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
class Vector : private VectorBuffer<T, INLINE_CAPACITY, Allocator> {
  USE_ALLOCATOR(Vector, Allocator);
  using Base = VectorBuffer<T, INLINE_CAPACITY, Allocator>;
  using TypeOperations = VectorTypeOperations<T, Allocator>;
  using OffsetRange = typename Base::OffsetRange;

 public:
  using ValueType = T;
  using value_type = T;
  using size_type = wtf_size_t;
  using reference = value_type&;
  using const_reference = const value_type&;
  using pointer = value_type*;
  using const_pointer = const value_type*;

  // TODO(crbug.com/355003172): We should try using
  // base::CheckedContiguousIterator instead of UncheckedIterator.
  using iterator = UncheckedIterator<T>;
  using const_iterator = UncheckedIterator<const T>;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;

  static constexpr bool SupportsInlineCapacity() { return INLINE_CAPACITY > 0; }

  // Create an empty vector.
  inline Vector();
  // Create a vector containing the specified number of default-initialized
  // elements. Requires T to have a default constructor.
  inline explicit Vector(wtf_size_t);
  // Create a vector containing the specified number of elements, each of which
  // is copy initialized from the specified value.
  inline Vector(wtf_size_t, const T&);

  // HashTable support
  Vector(HashTableDeletedValueType value) : Base(value) {}
  bool IsHashTableDeletedValue() const {
    return Base::IsHashTableDeletedValue();
  }

  // Copying.
  Vector(const Vector&);
  template <wtf_size_t otherCapacity>
  explicit Vector(const Vector<T, otherCapacity, Allocator>&);

  Vector& operator=(const Vector&);
  template <wtf_size_t otherCapacity>
  Vector& operator=(const Vector<T, otherCapacity, Allocator>&);

  // Creates a vector with elements copied or moved from an input and sized
  // range, with optional projection. To move elements, use
  // base::RangeAsRvalues(std::move(range)) as the first parameter.
  template <typename Range, typename Proj = std::identity>
    requires VectorCanAssignFromRange<T, InlineCapacity, Allocator, Range, Proj>
  explicit Vector(Range&&, Proj = {});

  // Replaces the vector with elements copied or moved from an input and sized
  // range. To move elements, use base::RangeAsRvalues(std::move(range)) as the
  // first parameter.
  template <typename Range, typename Proj = std::identity>
    requires VectorCanAssignFromRange<T, InlineCapacity, Allocator, Range, Proj>
  void assign(Range&&, Proj = {});

  // Moving.
  Vector(Vector&&);
  Vector& operator=(Vector&&);

  // Construct with an initializer list. You can do e.g.
  //     Vector<int> v({1, 2, 3});
  // or
  //     v = {4, 5, 6};
  Vector(std::initializer_list<T> elements);
  Vector& operator=(std::initializer_list<T> elements);

  // Basic inquiry about the vector's state.
  //
  // capacity() is the maximum number of elements that the Vector can hold
  // without a reallocation. It can be zero.
  wtf_size_t size() const { return size_; }
  wtf_size_t capacity() const { return Base::capacity(); }
  size_t CapacityInBytes() const { return Base::AllocationSize(capacity()); }
  bool empty() const { return !size(); }

  // at() and operator[]: Obtain the reference of the element that is located
  // at the given index. The reference may be invalidated on a reallocation.
  //
  // at() can be used in cases like:
  //     pointerToVector->at(1);
  // instead of:
  //     (*pointerToVector)[1];
  T& at(wtf_size_t i) {
    CHECK_LT(i, size());
    return Base::Buffer()[i];
  }
  const T& at(wtf_size_t i) const {
    CHECK_LT(i, size());
    return Base::Buffer()[i];
  }

  T& operator[](wtf_size_t i) { return at(i); }
  const T& operator[](wtf_size_t i) const { return at(i); }

  // Returns a base::span representing the whole data.
  // The base::span is valid until this Vector is modified.
  explicit operator base::span<T>() { return {data(), size()}; }
  explicit operator base::span<const T>() { return {data(), size()}; }

  // Return a pointer to the front of the backing buffer. Those pointers get
  // invalidated on a reallocation.
  T* data() { return Base::Buffer(); }
  const T* data() const { return Base::Buffer(); }

  // Iterators and reverse iterators. They are invalidated on a reallocation.
  //
  // When working with a subrange of a Vector, use base::span to represent
  // the range instead of a pair of iterators.
  //
  // If iterators are required for an api, prefer CheckedBegin() and
  // CheckedEnd() as they include bounds checks when the compiler can not
  // verify the code won't have a security bug with adversarial states
  // otherwise.
  //
  // These functions were primarily left unchecked for backward compat with
  // std sort algorithms. Use of the iterators that involves manually adjusting
  // their positions would require UNSAFE_BUFFERS and the code should satisfy
  // the requirements of UNSAFE_BUFFERS. See the macro definition in
  // https://source.chromium.org/chromium/chromium/src/+/main:base/compiler_specific.h
  // for more.
  iterator begin() { return iterator(data()); }
  iterator end() { return iterator(DataEnd()); }
  const_iterator begin() const { return const_iterator(data()); }
  const_iterator end() const { return const_iterator(DataEnd()); }

  reverse_iterator rbegin() { return reverse_iterator(end()); }
  reverse_iterator rend() { return reverse_iterator(begin()); }
  const_reverse_iterator rbegin() const {
    return const_reverse_iterator(end());
  }
  const_reverse_iterator rend() const {
    return const_reverse_iterator(begin());
  }

  // Checked iterators.
  // These iterators have runtime CHECK()s for incorrect operations. So
  // they are safer and slower than begin() and end().

  base::CheckedContiguousIterator<T> CheckedBegin() {
    return base::CheckedContiguousIterator<T>(data(), DataEnd());
  }
  base::CheckedContiguousIterator<T> CheckedEnd() {
    auto* e = DataEnd();
    return base::CheckedContiguousIterator<T>(data(), e, e);
  }
  base::CheckedContiguousIterator<const T> CheckedBegin() const {
    return base::CheckedContiguousIterator<const T>(data(), DataEnd());
  }
  base::CheckedContiguousIterator<const T> CheckedEnd() const {
    auto* e = DataEnd();
    return base::CheckedContiguousIterator<const T>(data(), e, e);
  }

  // Quick access to the first and the last element. It is invalid to call
  // these functions when the vector is empty.
  T& front() { return at(0); }
  const T& front() const { return at(0); }
  T& back() { return at(size() - 1); }
  const T& back() const { return at(size() - 1); }

  // Searching.
  //
  // Comparisons are done in terms of compareElement(), which is usually
  // operator==(). find() and reverseFind() returns an index of the element
  // that is found first. If no match is found, kNotFound will be returned.
  template <typename U>
  bool Contains(const U&) const;
  template <typename U>
  wtf_size_t Find(const U&) const;
  template <typename U>
  wtf_size_t ReverseFind(const U&) const;

  // Resize the vector to the specified size.
  //
  // These three functions are essentially similar. They differ in that
  // (1) Shrink() has a DCHECK to make sure the specified size is not more than
  //     size();
  // (2) Grow() has a DCHECK to make sure the specified size is not less than
  //     size();
  // (3) Grow() and resize() can be called only if T has a default constructor.
  //
  // When a vector shrinks, the extra elements in the back will be destructed.
  // All the iterators pointing to a to-be-destructed element will be
  // invalidated.
  //
  // When a vector grows, new elements will be added in the back, and they
  // will be default-initialized. A reallocation may happen in this case.
  void Shrink(wtf_size_t);
  void Grow(wtf_size_t);
  void resize(wtf_size_t);

  // Increase the capacity of the vector to at least |newCapacity|. The
  // elements in the vector are not affected. This function does not shrink
  // the size of the backing buffer, even if |newCapacity| is small. This
  // function may cause a reallocation.
  void reserve(wtf_size_t new_capacity);

  // This is similar to reserve() but must be called immediately after
  // the vector is default-constructed.
  void ReserveInitialCapacity(wtf_size_t initial_capacity);

  // Shrink the backing buffer to |new_capacity|. This function may cause a
  // reallocation.
  void ShrinkCapacity(wtf_size_t new_capacity);

  // Shrink the backing buffer so it can contain exactly |size()| elements.
  // This function may cause a reallocation.
  void shrink_to_fit() { ShrinkCapacity(size()); }

  // Shrink the backing buffer if at least 50% of the vector's capacity is
  // unused. If it shrinks, the new buffer contains roughly 25% of unused
  // space. This function may cause a reallocation.
  void ShrinkToReasonableCapacity() {
    if (size() * 2 < capacity())
      ShrinkCapacity(size() + size() / 4 + 1);
  }

  // Remove all the elements. This function actually releases the backing
  // buffer, thus any iterators will get invalidated (including begin()).
  REINITIALIZES_AFTER_MOVE void clear() { ShrinkCapacity(0); }

  // Insertion to the back. All of these functions except uncheckedAppend() may
  // cause a reallocation.
  //
  // push_back(value)
  //     Insert a single element to the back.
  // emplace_back(args...)
  //     Insert a single element constructed as T(args...) to the back. The
  //     element is constructed directly on the backing buffer with placement
  //     new.
  // Append(buffer, size)
  // AppendVector(vector)
  // AppendRange(begin, end)
  // AppendSpan(span)
  //     Insert multiple elements represented by (1) |buffer| and |size|
  //     (for append), (2) |vector| (for AppendVector), (3) a pair of
  //     iterators (for AppendRange), or (4) |span| (for AppendSpan) to the
  //     back. Except for AppendRange, the elements will be copied. For
  //     AppendRange, the elements will be copied or moved depending on the
  //     iterators. For example, the elements will be moved if the iterators
  //     are from std::make_move_iterator().
  // UncheckedAppend(value)
  //     Insert a single element like push_back(), but this function assumes
  //     the vector has enough capacity such that it can store the new element
  //     without a reallocation. Using this function could improve the
  //     performance when you append many elements repeatedly.
  template <typename U>
  void push_back(U&&);
  template <typename... Args>
  T& emplace_back(Args&&...);
  ALWAYS_INLINE T& emplace_back() {
    Grow(size_ + 1);
    return back();
  }
  template <typename U>
  void Append(const U*, wtf_size_t);
  template <typename U, wtf_size_t otherCapacity, typename V>
  void AppendVector(const Vector<U, otherCapacity, V>&);
  template <typename Iterator>
  void AppendRange(Iterator begin, Iterator end);
  template <typename U, size_t N, typename Ptr>
  void AppendSpan(base::span<U, N, Ptr>);
  template <typename U>
  void UncheckedAppend(U&&);

  // Insertion to an arbitrary position. All of these functions will take
  // O(size())-time. All of the elements after |position| will be moved to
  // the new locations. |position| must be no more than size(). All of these
  // functions may cause a reallocation. In any case, all the iterators
  // pointing to an element after |position| will be invalidated.
  //
  // insert(position, value)
  //     Insert a single element at |position|, where |position| is an index.
  // insert(position, buffer, size)
  // InsertVector(position, vector)
  //     Insert multiple elements represented by either |buffer| and |size|
  //     or |vector| at |position|. The elements will be copied.
  // InsertAt(position, value)
  //     Insert a single element at |position|, where |position| is an iterator.
  // InsertAt(position, buffer, size)
  //     Insert multiple elements represented by either |buffer| and |size|
  //     or |vector| at |position|. The elements will be copied.
  template <typename U>
  void insert(wtf_size_t position, U&&);
  template <typename U>
  void insert(wtf_size_t position, const U*, wtf_size_t);
  template <typename U>
  void InsertAt(iterator position, U&&);
  template <typename U>
  void InsertAt(iterator position, const U*, wtf_size_t);
  template <typename U, wtf_size_t otherCapacity, typename OtherAllocator>
  void InsertVector(wtf_size_t position,
                    const Vector<U, otherCapacity, OtherAllocator>&);

  // Insertion to the front. All of these functions will take O(size())-time.
  // All of the elements in the vector will be moved to the new locations.
  // All of these functions may cause a reallocation. In any case, all the
  // iterators pointing to any element in the vector will be invalidated.
  //
  // push_front(value)
  //     Insert a single element to the front.
  // push_front(buffer, size)
  // PrependVector(vector)
  //     Insert multiple elements represented by either |buffer| and |size| or
  //     |vector| to the front. The elements will be copied.
  template <typename U>
  void push_front(U&&);
  template <typename U>
  void push_front(const U*, wtf_size_t);
  template <typename U, wtf_size_t otherCapacity, typename OtherAllocator>
  void PrependVector(const Vector<U, otherCapacity, OtherAllocator>&);

  // Remove an element or elements at the specified position. These functions
  // take O(size())-time. All of the elements after the removed ones will be
  // moved to the new locations. All the iterators pointing to any element
  // after |position| will be invalidated.
  void EraseAt(wtf_size_t position);
  void EraseAt(wtf_size_t position, wtf_size_t length);
  iterator erase(iterator position);
  iterator erase(iterator first, iterator last);
  // This is to prevent compilation of deprecated calls like 'vector.erase(0)'.
  void erase(std::nullptr_t) = delete;

  // Remove the last element. Unlike remove(), (1) this function is fast, and
  // (2) only iterators pointing to the last element will be invalidated. Other
  // references will remain valid.
  void pop_back() {
    DCHECK(!empty());
    Shrink(size() - 1);
  }

  // Filling the vector with the same value. If the vector has shrinked or
  // growed as a result of this call, those events may invalidate some
  // iterators. See comments for shrink() and grow().
  //
  // Fill(value, size) will resize the Vector to |size|, and then copy-assign
  // or copy-initialize all the elements.
  //
  // Fill(value) is a synonym for Fill(value, size()).
  //
  // The implementation of Fill uses std::fill which is not yet supported for
  // garbage collected vectors.
  void Fill(const T&, wtf_size_t)
    requires(!Allocator::kIsGarbageCollected);
  void Fill(const T& val)
    requires(!Allocator::kIsGarbageCollected)
  {
    Fill(val, size());
  }

  // Swap two vectors quickly.
  void swap(Vector& other) {
    Base::SwapVectorBuffer(other, OffsetRange(), OffsetRange(),
                           VectorOperationOrigin::kRegularModification);
  }

  // Reverse the contents.
  void Reverse();

  // Maximum element count supported; allocating a vector
  // buffer with a larger count will fail.
  static size_t MaxCapacity() {
    return Allocator::template MaxElementCountInBackingStore<T>();
  }

  ~Vector()
    requires(!kVectorNeedsDestructor<T,
                                     INLINE_CAPACITY,
                                     Allocator::kIsGarbageCollected>)
  = default;
  ~Vector()
    requires(kVectorNeedsDestructor<T,
                                    INLINE_CAPACITY,
                                    Allocator::kIsGarbageCollected>)
  {
    static_assert(!Allocator::kIsGarbageCollected || INLINE_CAPACITY,
                  "GarbageCollected collections without inline capacity cannot "
                  "be finalized.");
    if (!INLINE_CAPACITY) {
      if (!Base::Buffer()) [[likely]] {
        return;
      }
    }
    ANNOTATE_DELETE_BUFFER(data(), capacity(), size_);
    if (size_) [[likely]] {
      if (!Allocator::kIsGarbageCollected || !this->HasOutOfLineBuffer()) {
        TypeOperations::Destruct(data(), DataEnd());
        size_ = 0;  // Partial protection against use-after-free.
      }
    }

    // For garbage collected vector HeapAllocator::BackingFree() will bail out
    // during sweeping.
    Base::Destruct();
  }

  void Trace(auto visitor) const
    requires Allocator::kIsGarbageCollected;

 protected:
  using Base::CheckUnusedSlots;
  using Base::ClearUnusedSlots;

  T** GetBufferSlot() { return Base::BufferSlot(); }
  const T* const* GetBufferSlot() const { return Base::BufferSlot(); }

 private:
  template <typename, wtf_size_t, typename>
  friend class Vector;
  // Point the next of the last item. We must not dereference the return value.
  T* DataEnd() { return data() + size(); }
  const T* DataEnd() const { return data() + size(); }

  void ExpandCapacity(wtf_size_t new_min_capacity);
  T* ExpandCapacity(wtf_size_t new_min_capacity, T*);
  T* ExpandCapacity(wtf_size_t new_min_capacity, const T* data) {
    return ExpandCapacity(new_min_capacity, const_cast<T*>(data));
  }

  template <typename U>
  U* ExpandCapacity(wtf_size_t new_min_capacity, U*);
  template <typename U>
  NOINLINE PRESERVE_MOST void AppendSlowCase(U&&);

  bool HasInlineBuffer() const {
    return INLINE_CAPACITY && !this->HasOutOfLineBuffer();
  }

  void ReallocateBuffer(wtf_size_t);

  void SwapForMove(Vector&& other, VectorOperationOrigin this_origin) {
    Base::SwapVectorBuffer(other, OffsetRange(), OffsetRange(), this_origin);
  }

  using Base::AllocateBuffer;
  using Base::AllocationSize;
  using Base::Buffer;
  using Base::BufferSafe;
  using Base::size_;
  using Base::SwapVectorBuffer;

  struct TypeConstraints {
    constexpr TypeConstraints() {
      // This condition is relied upon by TraceCollectionIfEnabled.
      static_assert(!IsWeak<T>::value);
      static_assert(!IsStackAllocatedTypeV<T>);
      static_assert(!std::is_polymorphic_v<T> ||
                        !VectorTraits<T>::kCanInitializeWithMemset,
                    "Cannot initialize with memset if there is a vtable.");
      static_assert(Allocator::kIsGarbageCollected || !IsDisallowNew<T> ||
                        !IsTraceable<T>::value,
                    "Cannot put DISALLOW_NEW() objects that have trace methods "
                    "into an off-heap Vector.");
      static_assert(
          Allocator::kIsGarbageCollected || !IsMemberType<T>::value,
          "Cannot put Member into an off-heap Vector. Use HeapVector instead.");
      static_assert(
          Allocator::kIsGarbageCollected || !IsWeakMemberType<T>::value,
          "WeakMember is not allowed in Vector nor HeapVector.");
      static_assert(
          Allocator::kIsGarbageCollected || !IsPointerToGarbageCollectedType<T>,
          "Cannot put raw pointers to garbage-collected classes into an "
          "off-heap Vector.  Use HeapVector<Member<T>> instead.");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

//
// Vector out-of-line implementation
//

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline Vector<T, InlineCapacity, Allocator>::Vector() {
  ANNOTATE_NEW_BUFFER(data(), capacity(), 0);
  size_ = 0;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline Vector<T, InlineCapacity, Allocator>::Vector(wtf_size_t size)
    : Base(size) {
  ANNOTATE_NEW_BUFFER(data(), capacity(), size);
  size_ = size;
  TypeOperations::Initialize(data(), DataEnd());
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline Vector<T, InlineCapacity, Allocator>::Vector(wtf_size_t size,
                                                    const T& val)
    : Base(size) {
  ANNOTATE_NEW_BUFFER(data(), capacity(), size);
  size_ = size;
  TypeOperations::UninitializedFill(data(), DataEnd(), val,
                                    VectorOperationOrigin::kConstruction);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
Vector<T, InlineCapacity, Allocator>::Vector(const Vector& other)
    : Base(other.capacity()) {
  ANNOTATE_NEW_BUFFER(data(), capacity(), other.size());
  size_ = other.size();
  TypeOperations::UninitializedCopy(other.data(), other.DataEnd(), data(),
                                    VectorOperationOrigin::kConstruction);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <wtf_size_t otherCapacity>
Vector<T, InlineCapacity, Allocator>::Vector(
    const Vector<T, otherCapacity, Allocator>& other)
    : Base(other.capacity()) {
  ANNOTATE_NEW_BUFFER(data(), capacity(), other.size());
  size_ = other.size();
  TypeOperations::UninitializedCopy(other.data(), other.DataEnd(), data(),
                                    VectorOperationOrigin::kConstruction);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename Range, typename Proj>
  requires VectorCanAssignFromRange<T, InlineCapacity, Allocator, Range, Proj>
Vector<T, InlineCapacity, Allocator>::Vector(Range&& other, Proj proj)
    : Base(std::ranges::size(other)) {
  // Note that `size(other)` may become smaller if `other` is a hash table
  // with WeakMember keys and `Base(size(other))` above caused GC which
  // removed some entries from `other`, see crbug.com/40448463. This won't
  // cause problems as long as we won't use the old `size(other)` in the
  // following code.
  ANNOTATE_NEW_BUFFER(data(), capacity(), std::ranges::size(other));
  TypeOperations::UninitializedTransform(
      std::ranges::begin(other), std::ranges::end(other), data(),
      VectorOperationOrigin::kConstruction, std::move(proj));
  size_ = std::ranges::size(other);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
Vector<T, InlineCapacity, Allocator>&
Vector<T, InlineCapacity, Allocator>::operator=(
    const Vector<T, InlineCapacity, Allocator>& other) {
  if (&other == this) [[unlikely]] {
    return *this;
  }

  if (size() > other.size()) {
    Shrink(other.size());
  } else if (other.size() > capacity()) {
    clear();
    reserve(other.size());
    DCHECK(data());
  }

  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     other.size());
  TypeOperations::Copy(other.data(), other.data() + size(), data(),
                       VectorOperationOrigin::kRegularModification);
  TypeOperations::UninitializedCopy(
      other.data() + size(), other.DataEnd(), DataEnd(),
      VectorOperationOrigin::kRegularModification);
  size_ = other.size();

  return *this;
}

inline bool TypelessPointersAreEqual(const void* a, const void* b) {
  return a == b;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <wtf_size_t otherCapacity>
Vector<T, InlineCapacity, Allocator>&
Vector<T, InlineCapacity, Allocator>::operator=(
    const Vector<T, otherCapacity, Allocator>& other) {
  // If the inline capacities match, we should call the more specific
  // template.  If the inline capacities don't match, the two objects
  // shouldn't be allocated the same address.
  DCHECK(!TypelessPointersAreEqual(&other, this));

  if (size() > other.size()) {
    Shrink(other.size());
  } else if (other.size() > capacity()) {
    clear();
    reserve(other.size());
    DCHECK(data());
  }

  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     other.size());
  TypeOperations::Copy(other.data(), other.data() + size(), data(),
                       VectorOperationOrigin::kRegularModification);
  TypeOperations::UninitializedCopy(
      other.data() + size(), other.DataEnd(), DataEnd(),
      VectorOperationOrigin::kRegularModification);
  size_ = other.size();

  return *this;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename Range, typename Proj>
  requires VectorCanAssignFromRange<T, InlineCapacity, Allocator, Range, Proj>
void Vector<T, InlineCapacity, Allocator>::assign(Range&& other, Proj proj) {
  if (std::ranges::size(other) > capacity()) {
    clear();
    reserve(std::ranges::size(other));
    // Note that `size(other)` may become smaller if `other` is a hash table
    // with `WeakMember` keys and `reserve` caused GC which removed some
    // entries from `other`, see crbug.com/40448463. This won't cause problems
    // as long as we won't use the old `size(other)` in the following code.
  } else {
    if (std::ranges::size(other) < size()) {
      Shrink(std::ranges::size(other));
    }
    TypeOperations::Destruct(data(), DataEnd());
  }

  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     std::ranges::size(other));
  TypeOperations::UninitializedTransform(
      std::ranges::begin(other), std::ranges::end(other), data(),
      VectorOperationOrigin::kRegularModification, std::move(proj));
  size_ = std::ranges::size(other);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
Vector<T, InlineCapacity, Allocator>::Vector(
    Vector<T, InlineCapacity, Allocator>&& other) {
  size_ = 0;
  // It's a little weird to implement a move constructor using swap but this
  // way we don't have to add a move constructor to VectorBuffer.
  SwapForMove(std::move(other), VectorOperationOrigin::kConstruction);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
Vector<T, InlineCapacity, Allocator>&
Vector<T, InlineCapacity, Allocator>::operator=(
    Vector<T, InlineCapacity, Allocator>&& other) {
  // Explicitly clearing allows the backing to be freed
  // immediately. In the non-garbage-collected case this is
  // often just slightly moving it earlier as the old backing
  // would otherwise be freed in the destructor. For the
  // garbage-collected case this allows for freeing the backing
  // right away without introducing GC pressure.
  clear();
  SwapForMove(std::move(other), VectorOperationOrigin::kRegularModification);
  return *this;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
Vector<T, InlineCapacity, Allocator>::Vector(std::initializer_list<T> elements)
    : Base(base::checked_cast<wtf_size_t>(elements.size())) {
  ANNOTATE_NEW_BUFFER(data(), capacity(), elements.size());
  size_ = static_cast<wtf_size_t>(elements.size());
  TypeOperations::UninitializedCopy(elements.begin(), elements.end(), data(),
                                    VectorOperationOrigin::kConstruction);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
Vector<T, InlineCapacity, Allocator>&
Vector<T, InlineCapacity, Allocator>::operator=(
    std::initializer_list<T> elements) {
  wtf_size_t input_size = base::checked_cast<wtf_size_t>(elements.size());
  if (size() > input_size) {
    Shrink(input_size);
  } else if (input_size > capacity()) {
    clear();
    reserve(input_size);
    DCHECK(data());
  }

  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     input_size);
  TypeOperations::Copy(elements.begin(), elements.begin() + size_, data(),
                       VectorOperationOrigin::kRegularModification);
  TypeOperations::UninitializedCopy(
      elements.begin() + size_, elements.end(), DataEnd(),
      VectorOperationOrigin::kRegularModification);
  size_ = input_size;

  return *this;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
bool Vector<T, InlineCapacity, Allocator>::Contains(const U& value) const {
  // Do not reuse Find because the compiler will generate extra code to
  // handle finding the kNotFound-th element in the array.  kNotFound is part
  // of wtf_size_t, but not used as an index due to runtime restrictions.  See
  // kNotFound.
  const T* b = data();
  const T* e = DataEnd();
  for (const T* iter = b; iter < e; ++iter) {
    if (TypeOperations::CompareElement(*iter, value)) {
      return true;
    }
  }
  return false;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
wtf_size_t Vector<T, InlineCapacity, Allocator>::Find(const U& value) const {
  const T* b = data();
  const T* e = DataEnd();
  for (const T* iter = b; iter < e; ++iter) {
    if (TypeOperations::CompareElement(*iter, value))
      return static_cast<wtf_size_t>(iter - b);
  }
  return kNotFound;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
wtf_size_t Vector<T, InlineCapacity, Allocator>::ReverseFind(
    const U& value) const {
  const T* b = data();
  const T* iter = DataEnd();
  while (iter > b) {
    --iter;
    if (TypeOperations::CompareElement(*iter, value))
      return static_cast<wtf_size_t>(iter - b);
  }
  return kNotFound;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
void Vector<T, InlineCapacity, Allocator>::Fill(const T& val,
                                                wtf_size_t new_size)
  requires(!Allocator::kIsGarbageCollected)
{
  if (size() > new_size) {
    Shrink(new_size);
  } else if (new_size > capacity()) {
    clear();
    reserve(new_size);
    DCHECK(data());
  }

  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     new_size);
  std::fill(begin(), end(), val);
  TypeOperations::UninitializedFill(
      DataEnd(), data() + new_size, val,
      VectorOperationOrigin::kRegularModification);
  size_ = new_size;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
void Vector<T, InlineCapacity, Allocator>::ExpandCapacity(
    wtf_size_t new_min_capacity) {
  wtf_size_t old_capacity = capacity();
  wtf_size_t expanded_capacity = old_capacity;
  // We use a more aggressive expansion strategy for Vectors with inline
  // storage.  This is because they are more likely to be on the stack, so the
  // risk of heap bloat is minimized.  Furthermore, exceeding the inline
  // capacity limit is not supposed to happen in the common case and may
  // indicate a pathological condition or microbenchmark.
  if (INLINE_CAPACITY) {
    expanded_capacity *= 2;
    // Check for integer overflow, which could happen in the 32-bit build.
    CHECK_GT(expanded_capacity, old_capacity);
  } else {
    // This cannot integer overflow.
    // On 64-bit, the "expanded" integer is 32-bit, and any encroachment
    // above 2^32 will fail allocation in allocateBuffer().  On 32-bit,
    // there's not enough address space to hold the old and new buffers.  In
    // addition, our underlying allocator is supposed to always fail on >
    // (2^31 - 1) allocations.
    expanded_capacity += (expanded_capacity / 4) + 1;
  }
  reserve(std::max(new_min_capacity,
                   std::max(kInitialVectorSize, expanded_capacity)));
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
T* Vector<T, InlineCapacity, Allocator>::ExpandCapacity(
    wtf_size_t new_min_capacity,
    T* ptr) {
  if (ptr < data() || ptr >= DataEnd()) {
    ExpandCapacity(new_min_capacity);
    return ptr;
  }
  size_t index = ptr - data();
  ExpandCapacity(new_min_capacity);
  return data() + index;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
inline U* Vector<T, InlineCapacity, Allocator>::ExpandCapacity(
    wtf_size_t new_min_capacity,
    U* ptr) {
  ExpandCapacity(new_min_capacity);
  return ptr;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline void Vector<T, InlineCapacity, Allocator>::resize(wtf_size_t size) {
  if (size <= size_) {
    TypeOperations::Destruct(data() + size, DataEnd());
    ClearUnusedSlots(data() + size, DataEnd());
    MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                       size);
  } else {
    if (size > capacity())
      ExpandCapacity(size);
    MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                       size);
    TypeOperations::Initialize(DataEnd(), data() + size);
  }

  size_ = size;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
void Vector<T, InlineCapacity, Allocator>::Shrink(wtf_size_t size) {
  CHECK_LE(size, size_);
  TypeOperations::Destruct(data() + size, DataEnd());
  ClearUnusedSlots(data() + size, DataEnd());
  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     size);
  size_ = size;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
void Vector<T, InlineCapacity, Allocator>::Grow(wtf_size_t size) {
  DCHECK_GE(size, size_);
  if (size > capacity())
    ExpandCapacity(size);
  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     size);
  TypeOperations::Initialize(DataEnd(), data() + size);
  size_ = size;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
void Vector<T, InlineCapacity, Allocator>::reserve(wtf_size_t new_capacity) {
  if (new_capacity <= capacity()) [[unlikely]] {
    return;
  }
  if (!data()) {
    Base::AllocateBuffer(new_capacity,
                         VectorOperationOrigin::kRegularModification);
    return;
  }

  if constexpr (Allocator::kIsGarbageCollected) {
    wtf_size_t old_capacity = capacity();
    // Unpoison container annotations. Note that in the case of sizeof(T) < 8,
    // size_ = 1, old_capacity = 1, this may leave behind state in ASAN's shadow
    // memory. The additional transition after expanding ensures that this state
    // is cleared.
    //
    // Details see
    //   https://github.com/llvm-mirror/compiler-rt/blob/master/lib/asan/asan_poisoning.cpp#L354
    MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), old_capacity, size_,
                                       old_capacity);
    if (Base::ExpandBuffer(new_capacity)) {
      // The following transition clears out old ASAN shadow memory state in the
      // case mentioned above.
      new_capacity = capacity();
      DCHECK_LE(old_capacity, new_capacity);
      ANNOTATE_CHANGE_SIZE(data(), new_capacity, old_capacity, new_capacity);
      // Finally, assuming new capacity, re-poison with the used size.
      ANNOTATE_CHANGE_SIZE(data(), new_capacity, new_capacity, size_);
      return;
    }
    // In case expansion failed, there's no need to adjust container
    // annotations, as the buffer is freed right away.
  }

  // Reallocating a backing buffer may resurrect a dead object.
  CHECK(Allocator::IsAllocationAllowed());

  ReallocateBuffer(new_capacity);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline void Vector<T, InlineCapacity, Allocator>::ReserveInitialCapacity(
    wtf_size_t initial_capacity) {
  DCHECK(!size_);
  DCHECK(capacity() == INLINE_CAPACITY);
  if (initial_capacity > INLINE_CAPACITY) {
    ANNOTATE_DELETE_BUFFER(data(), capacity(), size_);
    // The following uses `kRegularModification` as it's not guaranteed that the
    // Vector has not been published to the object graph after finishing the
    // constructor.
    Base::AllocateBuffer(initial_capacity,
                         VectorOperationOrigin::kRegularModification);
    MARKING_AWARE_ANNOTATE_NEW_BUFFER(Allocator, data(), capacity(), size_);
  }
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
void Vector<T, InlineCapacity, Allocator>::ShrinkCapacity(
    wtf_size_t new_capacity) {
  if (new_capacity >= capacity())
    return;

  if (new_capacity < size())
    Shrink(new_capacity);

  T* old_buffer = data();
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
  wtf_size_t old_capacity = capacity();
#endif
  if (new_capacity > 0) {
    if (Base::ShrinkBuffer(new_capacity)) {
      return;
    }

    if (!Allocator::IsAllocationAllowed())
      return;

    ReallocateBuffer(new_capacity);
    return;
  }
  Base::ResetBufferPointer();
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
  if (old_buffer != data()) {
    MARKING_AWARE_ANNOTATE_NEW_BUFFER(Allocator, data(), capacity(), size_);
    ANNOTATE_DELETE_BUFFER(old_buffer, old_capacity, size_);
  }
#endif
  Base::DeallocateBuffer(old_buffer);
}

// Templatizing these is better than just letting the conversion happen
// implicitly.
template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
ALWAYS_INLINE void Vector<T, InlineCapacity, Allocator>::push_back(U&& val) {
  DCHECK(Allocator::IsAllocationAllowed());
  if (size() != capacity()) [[likely]] {
    MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                       size_ + 1);
    ConstructTraits<T, VectorTraits<T>, Allocator>::ConstructAndNotifyElement(
        DataEnd(), std::forward<U>(val));
    ++size_;
    return;
  }

  AppendSlowCase(std::forward<U>(val));
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename... Args>
ALWAYS_INLINE T& Vector<T, InlineCapacity, Allocator>::emplace_back(
    Args&&... args) {
  DCHECK(Allocator::IsAllocationAllowed());
  if (size() == capacity()) [[unlikely]] {
    ExpandCapacity(size() + 1);
  }

  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     size_ + 1);
  T* t =
      ConstructTraits<T, VectorTraits<T>, Allocator>::ConstructAndNotifyElement(
          DataEnd(), std::forward<Args>(args)...);
  ++size_;
  return *t;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
void Vector<T, InlineCapacity, Allocator>::Append(const U* data,
                                                  wtf_size_t data_size) {
  DCHECK(Allocator::IsAllocationAllowed());
  wtf_size_t new_size = size_ + data_size;
  if (new_size > capacity()) {
    data = ExpandCapacity(new_size, data);
    DCHECK(this->data());
  }
  CHECK_GE(new_size, size_);
  T* dest = DataEnd();
  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, this->data(), capacity(), size_,
                                     new_size);
  TypeOperations::UninitializedCopy(
      data, &data[data_size], dest,
      VectorOperationOrigin::kRegularModification);
  size_ = new_size;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
NOINLINE PRESERVE_MOST void
Vector<T, InlineCapacity, Allocator>::AppendSlowCase(U&& val) {
  DCHECK_EQ(size(), capacity());

  typename std::remove_reference<U>::type* ptr = &val;
  ptr = ExpandCapacity(size() + 1, ptr);
  DCHECK(data());

  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     size_ + 1);
  ConstructTraits<T, VectorTraits<T>, Allocator>::ConstructAndNotifyElement(
      DataEnd(), std::forward<U>(*ptr));
  ++size_;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U, wtf_size_t otherCapacity, typename OtherAllocator>
inline void Vector<T, InlineCapacity, Allocator>::AppendVector(
    const Vector<U, otherCapacity, OtherAllocator>& val) {
  Append(val.data(), val.size());
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename Iterator>
void Vector<T, InlineCapacity, Allocator>::AppendRange(Iterator begin,
                                                       Iterator end) {
  for (Iterator it = begin; it != end; ++it)
    push_back(*it);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U, size_t N, typename Ptr>
void Vector<T, InlineCapacity, Allocator>::AppendSpan(
    base::span<U, N, Ptr> data) {
  Append(data.data(), base::checked_cast<wtf_size_t>(data.size()));
}

// This version of append saves a branch in the case where you know that the
// vector's capacity is large enough for the append to succeed.
template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
ALWAYS_INLINE void Vector<T, InlineCapacity, Allocator>::UncheckedAppend(
    U&& val) {
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
  // Vectors in ASAN builds don't have InlineCapacity.
  push_back(std::forward<U>(val));
#else
  DCHECK_LT(size(), capacity());
  ConstructTraits<T, VectorTraits<T>, Allocator>::ConstructAndNotifyElement(
      DataEnd(), std::forward<U>(val));
  ++size_;
#endif
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
inline void Vector<T, InlineCapacity, Allocator>::insert(wtf_size_t position,
                                                         U&& val) {
  DCHECK(Allocator::IsAllocationAllowed());
  CHECK_LE(position, size());
  typename std::remove_reference<U>::type* data = &val;
  if (size() == capacity()) {
    data = ExpandCapacity(size() + 1, data);
    DCHECK(this->data());
  }
  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, this->data(), capacity(), size_,
                                     size_ + 1);
  T* spot = this->data() + position;
  TypeOperations::MoveOverlapping(spot, DataEnd(), spot + 1,
                                  VectorOperationOrigin::kRegularModification);
  ConstructTraits<T, VectorTraits<T>, Allocator>::ConstructAndNotifyElement(
      spot, std::forward<U>(*data));
  ++size_;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
void Vector<T, InlineCapacity, Allocator>::insert(wtf_size_t position,
                                                  const U* data,
                                                  wtf_size_t data_size) {
  DCHECK(Allocator::IsAllocationAllowed());
  CHECK_LE(position, size());
  wtf_size_t new_size = size_ + data_size;
  if (new_size > capacity()) {
    data = ExpandCapacity(new_size, data);
    DCHECK(this->data());
  }
  CHECK_GE(new_size, size_);
  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, this->data(), capacity(), size_,
                                     new_size);
  T* spot = this->data() + position;
  TypeOperations::MoveOverlapping(spot, DataEnd(), spot + data_size,
                                  VectorOperationOrigin::kRegularModification);
  TypeOperations::UninitializedCopy(
      data, &data[data_size], spot,
      VectorOperationOrigin::kRegularModification);
  size_ = new_size;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
void Vector<T, InlineCapacity, Allocator>::InsertAt(Vector::iterator position,
                                                    U&& val) {
  insert(base::checked_cast<wtf_size_t>(position - begin()), val);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
void Vector<T, InlineCapacity, Allocator>::InsertAt(Vector::iterator position,
                                                    const U* data,
                                                    wtf_size_t data_size) {
  insert(base::checked_cast<wtf_size_t>(position - begin()), data, data_size);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U, wtf_size_t otherCapacity, typename OtherAllocator>
inline void Vector<T, InlineCapacity, Allocator>::InsertVector(
    wtf_size_t position,
    const Vector<U, otherCapacity, OtherAllocator>& val) {
  insert(position, val.data(), val.size());
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
inline void Vector<T, InlineCapacity, Allocator>::push_front(U&& val) {
  insert(0, std::forward<U>(val));
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U>
void Vector<T, InlineCapacity, Allocator>::push_front(const U* data,
                                                      wtf_size_t data_size) {
  insert(0, data, data_size);
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
template <typename U, wtf_size_t otherCapacity, typename OtherAllocator>
inline void Vector<T, InlineCapacity, Allocator>::PrependVector(
    const Vector<U, otherCapacity, OtherAllocator>& val) {
  insert(0, val.data(), val.size());
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline void Vector<T, InlineCapacity, Allocator>::EraseAt(wtf_size_t position) {
  CHECK_LT(position, size());
  T* spot = data() + position;
  spot->~T();
  TypeOperations::MoveOverlapping(spot + 1, DataEnd(), spot,
                                  VectorOperationOrigin::kRegularModification);
  ClearUnusedSlots(DataEnd() - 1, DataEnd());
  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     size_ - 1);
  --size_;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline auto Vector<T, InlineCapacity, Allocator>::erase(iterator position)
    -> iterator {
  wtf_size_t index = static_cast<wtf_size_t>(position - begin());
  EraseAt(index);
  return begin() + index;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline auto Vector<T, InlineCapacity, Allocator>::erase(iterator first,
                                                        iterator last)
    -> iterator {
  DCHECK_LE(first, last);
  const wtf_size_t index = static_cast<wtf_size_t>(first - begin());
  const wtf_size_t diff = static_cast<wtf_size_t>(std::distance(first, last));
  EraseAt(index, diff);
  return begin() + index;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline void Vector<T, InlineCapacity, Allocator>::EraseAt(wtf_size_t position,
                                                          wtf_size_t length) {
  SECURITY_DCHECK(position <= size());
  if (!length)
    return;
  CHECK_LE(position + length, size());
  T* begin_spot = data() + position;
  T* end_spot = begin_spot + length;
  TypeOperations::Destruct(begin_spot, end_spot);
  TypeOperations::MoveOverlapping(end_spot, DataEnd(), begin_spot,
                                  VectorOperationOrigin::kRegularModification);
  ClearUnusedSlots(DataEnd() - length, DataEnd());
  MARKING_AWARE_ANNOTATE_CHANGE_SIZE(Allocator, data(), capacity(), size_,
                                     size_ - length);
  size_ -= length;
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline void Vector<T, InlineCapacity, Allocator>::Reverse() {
  for (wtf_size_t i = 0; i < size_ / 2; ++i)
    std::swap(at(i), at(size_ - 1 - i));
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
inline void swap(Vector<T, InlineCapacity, Allocator>& a,
                 Vector<T, InlineCapacity, Allocator>& b) {
  a.Swap(b);
}

template <typename T,
          wtf_size_t InlineCapacityA,
          wtf_size_t InlineCapacityB,
          typename Allocator>
bool operator==(const Vector<T, InlineCapacityA, Allocator>& a,
                const Vector<T, InlineCapacityB, Allocator>& b) {
  if (a.size() != b.size())
    return false;
  if (a.empty())
    return true;
  return VectorTypeOperations<T, Allocator>::Compare(a.data(), b.data(),
                                                     a.size());
}

template <typename T,
          wtf_size_t InlineCapacityA,
          wtf_size_t InlineCapacityB,
          typename Allocator>
inline bool operator!=(const Vector<T, InlineCapacityA, Allocator>& a,
                       const Vector<T, InlineCapacityB, Allocator>& b) {
  return !(a == b);
}

namespace internal {
template <typename Allocator, typename VisitorDispatcher, typename T>
void TraceInlinedBuffer(VisitorDispatcher visitor,
                        const T* buffer_begin,
                        size_t capacity) {
  const T* buffer_end = buffer_begin + capacity;
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
  // Vector can trace unused slots (which are already zeroed out).
  ANNOTATE_CHANGE_SIZE(buffer_begin, capacity, 0, capacity);
#endif  // ANNOTATE_CONTIGUOUS_CONTAINER
  for (const T* buffer_entry = buffer_begin; buffer_entry != buffer_end;
       buffer_entry++) {
    Allocator::template Trace<T, VectorTraits<T>>(visitor, *buffer_entry);
  }
}

template <typename Allocator,
          typename VisitorDispatcher,
          typename T,
          wtf_size_t InlineCapacity>
void DeferredTraceImpl(VisitorDispatcher visitor, const void* object) {
  internal::TraceInlinedBuffer<Allocator>(
      visitor, reinterpret_cast<const T*>(object), InlineCapacity);
}

}  // namespace internal

// Only defined for HeapAllocator. Used when visiting vector object.
template <typename T, wtf_size_t InlineCapacity, typename Allocator>
void Vector<T, InlineCapacity, Allocator>::Trace(auto visitor) const
  requires Allocator::kIsGarbageCollected
{
  static_assert(Allocator::kIsGarbageCollected,
                "Garbage collector must be enabled.");

  const T* buffer = BufferSafe();

  if (!buffer) {
    // Register the slot for heap compaction.
    Allocator::TraceVectorBacking(visitor, static_cast<T*>(nullptr),
                                  Base::BufferSlot());
    return;
  }

  if (Base::IsOutOfLineBuffer(buffer)) {
    Allocator::TraceVectorBacking(visitor, buffer, Base::BufferSlot());
  } else {
    // We should not visit inline buffers, but we still need to register the
    // slot for heap compaction. So, we pass nullptr to this method.
    Allocator::TraceVectorBacking(visitor, static_cast<T*>(nullptr),
                                  Base::BufferSlot());

    // Bail out for concurrent marking.
    if (!VectorTraits<T>::kCanTraceConcurrently) {
      if (Allocator::DeferTraceToMutatorThreadIfConcurrent(
              visitor, buffer,
              internal::DeferredTraceImpl<Allocator, decltype(visitor), T,
                                          InlineCapacity>,
              InlineCapacity * sizeof(T))) {
        return;
      }
    }

    // Inline buffer requires tracing immediately.
    internal::TraceInlinedBuffer<Allocator>(visitor, buffer, InlineCapacity);
  }
}

template <typename T, wtf_size_t InlineCapacity, typename Allocator>
void Vector<T, InlineCapacity, Allocator>::ReallocateBuffer(
    wtf_size_t new_capacity) {
  if (new_capacity <= INLINE_CAPACITY) {
    if (HasInlineBuffer()) {
      Base::ResetBufferPointer();
      return;
    }
    // Shrinking to inline buffer from out-of-line one.
    T *old_begin = data(), *old_end = DataEnd();
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
    const wtf_size_t old_capacity = capacity();
#endif
    Base::ResetBufferPointer();
    TypeOperations::Move(old_begin, old_end, data(),
                         VectorOperationOrigin::kRegularModification);
    ClearUnusedSlots(old_begin, old_end);
    ANNOTATE_DELETE_BUFFER(old_begin, old_capacity, size_);
    Base::DeallocateBuffer(old_begin);
    return;
  }
  // Shrinking/resizing to out-of-line buffer.
  VectorBufferBase<T, Allocator> temp_buffer =
      Base::AllocateTemporaryBuffer(new_capacity);
  ANNOTATE_NEW_BUFFER(temp_buffer.Buffer(), temp_buffer.capacity(), size_);
  // If there was a new out-of-line buffer allocated, there is no need in
  // calling write barriers for entries in that backing store as it is still
  // white.
  TypeOperations::Move(data(), DataEnd(), temp_buffer.Buffer(),
                       VectorOperationOrigin::kConstruction);
  ClearUnusedSlots(data(), DataEnd());
  ANNOTATE_DELETE_BUFFER(data(), capacity(), size_);
  Base::DeallocateBuffer(data());
  Base::AcquireBuffer(std::move(temp_buffer));
}

// Erase/EraseIf are based on C++20's uniform container erasure API:
// - https://eel.is/c++draft/libraryindex#:erase
// - https://eel.is/c++draft/libraryindex#:erase_if
template <typename T,
          wtf_size_t inline_capacity,
          typename Allocator,
          typename U>
wtf_size_t Erase(Vector<T, inline_capacity, Allocator>& v, const U& value) {
  auto it = std::remove(v.begin(), v.end(), value);
  wtf_size_t removed = base::checked_cast<wtf_size_t>(v.end() - it);
  v.erase(it, v.end());
  return removed;
}
template <typename T,
          wtf_size_t inline_capacity,
          typename Allocator,
          typename Pred>
wtf_size_t EraseIf(Vector<T, inline_capacity, Allocator>& v, Pred pred) {
  auto it = std::remove_if(v.begin(), v.end(), pred);
  wtf_size_t removed = base::checked_cast<wtf_size_t>(v.end() - it);
  v.erase(it, v.end());
  return removed;
}

// The WTF version of base::ToVector. This is more convenient to use than
// Vector::Vector(range[, proj]) in some cases, e.g. when a temporary vector is
// needed and the desired result type is the same as the deducted return type.
// See Vector::Vector(range, proj) and Vector::assign() about copying vs moving.
template <typename Range, typename Proj = std::identity>
  requires std::ranges::sized_range<Range> && std::ranges::input_range<Range> &&
           std::indirectly_unary_invocable<Proj, std::ranges::iterator_t<Range>>
auto ToVector(Range&& range, Proj proj = {}) {
  using ProjectedType =
      std::projected<std::ranges::iterator_t<Range>, Proj>::value_type;
  return Vector<ProjectedType>(std::forward<Range>(range), std::move(proj));
}

}  // namespace WTF

using WTF::Vector;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_VECTOR_H_
