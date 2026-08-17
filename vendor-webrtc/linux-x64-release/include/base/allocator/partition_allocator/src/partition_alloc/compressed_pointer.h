// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_COMPRESSED_POINTER_H_
#define PARTITION_ALLOC_COMPRESSED_POINTER_H_

#include <climits>
#include <type_traits>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_address_space.h"
#include "partition_alloc/partition_alloc_base/bits.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

#if PA_BUILDFLAG(ENABLE_POINTER_COMPRESSION)

#if !PA_BUILDFLAG(GLUE_CORE_POOLS)
#error "Pointer compression only works with glued pools"
#endif
#if PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
#error "Pointer compression currently supports constant pool size"
#endif

#endif  // PA_BUILDFLAG(ENABLE_POINTER_COMPRESSION)

namespace partition_alloc {

namespace internal {

template <typename T1, typename T2>
constexpr bool IsDecayedSame =
    std::is_same_v<std::decay_t<T1>, std::decay_t<T2>>;

#if PA_BUILDFLAG(ENABLE_POINTER_COMPRESSION)

// Pointer compression works by storing only the 'useful' 32-bit part of the
// pointer. The other half (the base) is stored in a global variable
// (CompressedPointerBaseGlobal::g_base_), which is used on decompression. To
// support fast branchless decompression of nullptr, we use the most significant
// bit in the compressed pointer to leverage sign-extension (for non-nullptr
// pointers, the most significant bit is set, whereas for nullptr it's not).
// Using this bit and supporting heaps larger than 4GB relies on having
// alignment bits in pointers. Assuming that all pointers point to at least
// 8-byte alignment objects, pointer compression can support heaps of size <=
// 16GB.
// ((3 alignment bits) = (1 bit for sign-extension) + (2 bits for 16GB heap)).
//
// Example: heap base: 0x4b0'ffffffff
//  - g_base: 0x4b3'ffffffff (lower 34 bits set)
//  - normal pointer: 0x4b2'a08b6480
//    - compression:
//      - shift right by 3:        0x96'54116c90
//      - truncate:                   0x54116c90
//      - mark MSB:                   0xd4116c90
//    - decompression:
//      - sign-extend:       0xffffffff'd4116c90
//      - shift left by 3:   0xfffffffe'a08b6480
//      - 'and' with g_base: 0x000004b2'a08b6480
//
//  - nullptr: 0x00000000'00000000
//    - compression:
//      - shift right by 3:  0x00000000'00000000
//      - truncate:                   0x00000000
//      - (don't mark MSB for nullptr)
//    - decompression:
//      - sign-extend:       0x00000000'00000000
//      - shift left by 3:   0x00000000'00000000
//      - 'and' with g_base: 0x00000000'00000000
//
// Pointer compression relies on having both the regular and the BRP pool (core
// pools) 'glued', so that the same base could be used for both. For simplicity,
// the configurations with dynamically selected pool size are not supported.
// However, they can be at the cost of performing an extra load for
// core-pools-shift-size on both compression and decompression.

class CompressedPointerBaseGlobal final {
 public:
  static constexpr size_t kUsefulBits =
      base::bits::CountrZero(PartitionAddressSpace::CorePoolsSize());
  static_assert(kUsefulBits >= sizeof(uint32_t) * CHAR_BIT);
  static constexpr size_t kBitsToShift =
      kUsefulBits - sizeof(uint32_t) * CHAR_BIT;

  CompressedPointerBaseGlobal() = delete;

  // Attribute const allows the compiler to assume that
  // CompressedPointerBaseGlobal::g_base_ doesn't change (e.g. across calls) and
  // thereby avoid redundant loads.
  PA_ALWAYS_INLINE __attribute__((const)) static uintptr_t Get() {
    PA_DCHECK(IsBaseConsistent());
    return g_base_.base;
  }

  PA_ALWAYS_INLINE static bool IsSet() {
    PA_DCHECK(IsBaseConsistent());
    return (g_base_.base & ~kUsefulBitsMask) != 0;
  }

 private:
  static constexpr uintptr_t kUsefulBitsMask =
      PartitionAddressSpace::CorePoolsSize() - 1;

  PA_CONSTINIT static union alignas(kPartitionCachelineSize)
      PA_COMPONENT_EXPORT(PARTITION_ALLOC) Base {
    uintptr_t base;
    char cache_line[kPartitionCachelineSize];
  } g_base_;

  PA_ALWAYS_INLINE static bool IsBaseConsistent() {
    return kUsefulBitsMask == (g_base_.base & kUsefulBitsMask);
  }

  static void SetBase(uintptr_t base);
  static void ResetBaseForTesting();

  friend class PartitionAddressSpace;
};

#endif  // PA_BUILDFLAG(ENABLE_POINTER_COMPRESSION)

}  // namespace internal

#if PA_BUILDFLAG(ENABLE_POINTER_COMPRESSION)

template <typename T>
class PA_TRIVIAL_ABI CompressedPointer final {
 public:
  using UnderlyingType = uint32_t;

  PA_ALWAYS_INLINE constexpr CompressedPointer() = default;
  PA_ALWAYS_INLINE explicit CompressedPointer(T* ptr) : value_(Compress(ptr)) {}
  PA_ALWAYS_INLINE constexpr explicit CompressedPointer(std::nullptr_t)
      : value_(0u) {}

  PA_ALWAYS_INLINE constexpr CompressedPointer(const CompressedPointer&) =
      default;
  PA_ALWAYS_INLINE constexpr CompressedPointer(
      CompressedPointer&& other) noexcept = default;

  template <typename U,
            std::enable_if_t<std::is_convertible_v<U*, T*>>* = nullptr>
  PA_ALWAYS_INLINE constexpr CompressedPointer(
      const CompressedPointer<U>& other) {
    if constexpr (internal::IsDecayedSame<T, U>) {
      // When pointers have the same type modulo constness, avoid the
      // compress-decompress round.
      value_ = other.value_;
    } else {
      // When the types are different, perform the round, because the pointer
      // may need to be adjusted.
      // TODO(crbug.com/40243421): Avoid the cycle here.
      value_ = Compress(other.get());
    }
  }

  template <typename U,
            std::enable_if_t<std::is_convertible_v<U*, T*>>* = nullptr>
  PA_ALWAYS_INLINE constexpr CompressedPointer(
      CompressedPointer<U>&& other) noexcept
      : CompressedPointer(other) {}

  ~CompressedPointer() = default;

  PA_ALWAYS_INLINE constexpr CompressedPointer& operator=(
      const CompressedPointer&) = default;
  PA_ALWAYS_INLINE constexpr CompressedPointer& operator=(
      CompressedPointer&& other) noexcept = default;

  template <typename U,
            std::enable_if_t<std::is_convertible_v<U*, T*>>* = nullptr>
  PA_ALWAYS_INLINE constexpr CompressedPointer& operator=(
      const CompressedPointer<U>& other) {
    CompressedPointer copy(other);
    value_ = copy.value_;
    return *this;
  }

  template <typename U,
            std::enable_if_t<std::is_convertible_v<U*, T*>>* = nullptr>
  PA_ALWAYS_INLINE constexpr CompressedPointer& operator=(
      CompressedPointer<U>&& other) noexcept {
    *this = other;
    return *this;
  }

  // Don't perform compression when assigning to nullptr.
  PA_ALWAYS_INLINE constexpr CompressedPointer& operator=(std::nullptr_t) {
    value_ = 0u;
    return *this;
  }

  PA_ALWAYS_INLINE T* get() const { return Decompress(value_); }

  PA_ALWAYS_INLINE constexpr bool is_nonnull() const { return value_; }

  PA_ALWAYS_INLINE constexpr UnderlyingType GetAsIntegral() const {
    return value_;
  }

  PA_ALWAYS_INLINE constexpr explicit operator bool() const {
    return is_nonnull();
  }

  template <typename U = T,
            std::enable_if_t<!std::is_void_v<std::remove_cv_t<U>>>* = nullptr>
  PA_ALWAYS_INLINE U& operator*() const {
    PA_DCHECK(is_nonnull());
    return *get();
  }

  PA_ALWAYS_INLINE T* operator->() const {
    PA_DCHECK(is_nonnull());
    return get();
  }

  PA_ALWAYS_INLINE constexpr void swap(CompressedPointer& other) {
    std::swap(value_, other.value_);
  }

 private:
  template <typename>
  friend class CompressedPointer;

  static constexpr size_t kBitsForSignExtension = 1;
  static constexpr size_t kOverallBitsToShift =
      internal::CompressedPointerBaseGlobal::kBitsToShift +
      kBitsForSignExtension;

  PA_ALWAYS_INLINE static UnderlyingType Compress(T* ptr) {
    static constexpr size_t kMinimalRequiredAlignment = 8;
    static_assert((1 << kOverallBitsToShift) == kMinimalRequiredAlignment);

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
    PA_DCHECK(reinterpret_cast<uintptr_t>(ptr) % kMinimalRequiredAlignment ==
              0);
    PA_DCHECK(internal::CompressedPointerBaseGlobal::IsSet());

    const uintptr_t base = internal::CompressedPointerBaseGlobal::Get();
    static constexpr size_t kCorePoolsBaseMask =
        ~(internal::PartitionAddressSpace::CorePoolsSize() - 1);
    PA_DCHECK(!ptr ||
              (base & kCorePoolsBaseMask) ==
                  (reinterpret_cast<uintptr_t>(ptr) & kCorePoolsBaseMask));
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

    const auto uptr = reinterpret_cast<uintptr_t>(ptr);
    // Shift the pointer and truncate.
    auto compressed = static_cast<UnderlyingType>(uptr >> kOverallBitsToShift);
    // If the pointer is non-null, mark the most-significant-bit to sign-extend
    // it on decompression. Assuming compression is a significantly less
    // frequent operation, we let more work here in favor of faster
    // decompression.
    // TODO(crbug.com/40243421): Avoid this by overreserving the heap.
    if (compressed) {
      compressed |= (1u << (sizeof(uint32_t) * CHAR_BIT - 1));
    }

    return compressed;
  }

  PA_ALWAYS_INLINE static T* Decompress(UnderlyingType ptr) {
    PA_DCHECK(internal::CompressedPointerBaseGlobal::IsSet());
    const uintptr_t base = internal::CompressedPointerBaseGlobal::Get();
    // Treat compressed pointer as signed and cast it to uint64_t, which will
    // sign-extend it. Then, shift the result by one. It's important to shift
    // the already unsigned value, as otherwise it would result in undefined
    // behavior.
    const uint64_t mask = static_cast<uint64_t>(static_cast<int32_t>(ptr))
                          << (kOverallBitsToShift);
    return reinterpret_cast<T*>(mask & base);
  }

  UnderlyingType value_;
};

template <typename T>
PA_ALWAYS_INLINE constexpr void swap(CompressedPointer<T>& a,
                                     CompressedPointer<T>& b) {
  a.swap(b);
}

// operators==.
template <typename T, typename U>
PA_ALWAYS_INLINE bool operator==(CompressedPointer<T> a,
                                 CompressedPointer<U> b) {
  if constexpr (internal::IsDecayedSame<T, U>) {
    // When pointers have the same type modulo constness, simply compare
    // compressed values.
    return a.GetAsIntegral() == b.GetAsIntegral();
  } else {
    // When the types are different, compare decompressed pointers, because the
    // pointers may need to be adjusted.
    // TODO(crbug.com/40243421): Avoid decompression here.
    return a.get() == b.get();
  }
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator==(CompressedPointer<T> a, U* b) {
  // Do compression, since it is less expensive.
  return a == static_cast<CompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator==(T* a, CompressedPointer<U> b) {
  return b == a;
}

template <typename T>
PA_ALWAYS_INLINE constexpr bool operator==(CompressedPointer<T> a,
                                           std::nullptr_t) {
  return !a.is_nonnull();
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator==(std::nullptr_t,
                                           CompressedPointer<U> b) {
  return b == nullptr;
}

// operators!=.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator!=(CompressedPointer<T> a,
                                           CompressedPointer<U> b) {
  return !(a == b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator!=(CompressedPointer<T> a, U* b) {
  // Do compression, since it is less expensive.
  return a != static_cast<CompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator!=(T* a, CompressedPointer<U> b) {
  return b != a;
}

template <typename T>
PA_ALWAYS_INLINE constexpr bool operator!=(CompressedPointer<T> a,
                                           std::nullptr_t) {
  return a.is_nonnull();
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator!=(std::nullptr_t,
                                           CompressedPointer<U> b) {
  return b != nullptr;
}

// operators<.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<(CompressedPointer<T> a,
                                          CompressedPointer<U> b) {
  if constexpr (internal::IsDecayedSame<T, U>) {
    // When pointers have the same type modulo constness, simply compare
    // compressed values.
    return a.GetAsIntegral() < b.GetAsIntegral();
  } else {
    // When the types are different, compare decompressed pointers, because the
    // pointers may need to be adjusted.
    // TODO(crbug.com/40243421): Avoid decompression here.
    return a.get() < b.get();
  }
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<(CompressedPointer<T> a, U* b) {
  // Do compression, since it is less expensive.
  return a < static_cast<CompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<(T* a, CompressedPointer<U> b) {
  // Do compression, since it is less expensive.
  return static_cast<CompressedPointer<T>>(a) < b;
}

// operators<=.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<=(CompressedPointer<T> a,
                                           CompressedPointer<U> b) {
  if constexpr (internal::IsDecayedSame<T, U>) {
    // When pointers have the same type modulo constness, simply compare
    // compressed values.
    return a.GetAsIntegral() <= b.GetAsIntegral();
  } else {
    // When the types are different, compare decompressed pointers, because the
    // pointers may need to be adjusted.
    // TODO(crbug.com/40243421): Avoid decompression here.
    return a.get() <= b.get();
  }
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<=(CompressedPointer<T> a, U* b) {
  // Do compression, since it is less expensive.
  return a <= static_cast<CompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<=(T* a, CompressedPointer<U> b) {
  // Do compression, since it is less expensive.
  return static_cast<CompressedPointer<T>>(a) <= b;
}

// operators>.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>(CompressedPointer<T> a,
                                          CompressedPointer<U> b) {
  return !(a <= b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>(CompressedPointer<T> a, U* b) {
  // Do compression, since it is less expensive.
  return a > static_cast<CompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>(T* a, CompressedPointer<U> b) {
  // Do compression, since it is less expensive.
  return static_cast<CompressedPointer<T>>(a) > b;
}

// operators>=.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>=(CompressedPointer<T> a,
                                           CompressedPointer<U> b) {
  return !(a < b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>=(CompressedPointer<T> a, U* b) {
  // Do compression, since it is less expensive.
  return a >= static_cast<CompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>=(T* a, CompressedPointer<U> b) {
  // Do compression, since it is less expensive.
  return static_cast<CompressedPointer<T>>(a) >= b;
}

#endif  // PA_BUILDFLAG(ENABLE_POINTER_COMPRESSION)

// Simple wrapper over the raw pointer.
template <typename T>
class PA_TRIVIAL_ABI UncompressedPointer final {
 public:
  PA_ALWAYS_INLINE constexpr UncompressedPointer() = default;
  PA_ALWAYS_INLINE constexpr explicit UncompressedPointer(T* ptr) : ptr_(ptr) {}
  PA_ALWAYS_INLINE constexpr explicit UncompressedPointer(std::nullptr_t)
      : ptr_(nullptr) {}

  PA_ALWAYS_INLINE constexpr UncompressedPointer(const UncompressedPointer&) =
      default;
  PA_ALWAYS_INLINE constexpr UncompressedPointer(
      UncompressedPointer&& other) noexcept = default;

  template <typename U,
            std::enable_if_t<std::is_convertible_v<U*, T*>>* = nullptr>
  PA_ALWAYS_INLINE constexpr explicit UncompressedPointer(
      const UncompressedPointer<U>& other)
      : ptr_(other.ptr_) {}

  template <typename U,
            std::enable_if_t<std::is_convertible_v<U*, T*>>* = nullptr>
  PA_ALWAYS_INLINE constexpr explicit UncompressedPointer(
      UncompressedPointer<U>&& other) noexcept
      : ptr_(std::move(other.ptr_)) {}

  ~UncompressedPointer() = default;

  PA_ALWAYS_INLINE constexpr UncompressedPointer& operator=(
      const UncompressedPointer&) = default;
  PA_ALWAYS_INLINE constexpr UncompressedPointer& operator=(
      UncompressedPointer&& other) noexcept = default;

  template <typename U,
            std::enable_if_t<std::is_convertible_v<U*, T*>>* = nullptr>
  PA_ALWAYS_INLINE constexpr UncompressedPointer& operator=(
      const UncompressedPointer<U>& other) {
    ptr_ = other.ptr_;
    return *this;
  }

  template <typename U,
            std::enable_if_t<std::is_convertible_v<U*, T*>>* = nullptr>
  PA_ALWAYS_INLINE constexpr UncompressedPointer& operator=(
      UncompressedPointer<U>&& other) noexcept {
    ptr_ = std::move(other.ptr_);
    return *this;
  }

  PA_ALWAYS_INLINE constexpr UncompressedPointer& operator=(std::nullptr_t) {
    ptr_ = nullptr;
    return *this;
  }

  PA_ALWAYS_INLINE constexpr T* get() const { return ptr_; }

  PA_ALWAYS_INLINE constexpr bool is_nonnull() const { return ptr_; }

  PA_ALWAYS_INLINE constexpr explicit operator bool() const {
    return is_nonnull();
  }

  template <typename U = T,
            std::enable_if_t<!std::is_void_v<std::remove_cv_t<U>>>* = nullptr>
  PA_ALWAYS_INLINE constexpr U& operator*() const {
    PA_DCHECK(is_nonnull());
    return *get();
  }

  PA_ALWAYS_INLINE constexpr T* operator->() const {
    PA_DCHECK(is_nonnull());
    return get();
  }

  PA_ALWAYS_INLINE constexpr void swap(UncompressedPointer& other) {
    std::swap(ptr_, other.ptr_);
  }

 private:
  template <typename>
  friend class UncompressedPointer;

  T* ptr_;
};

template <typename T>
PA_ALWAYS_INLINE constexpr void swap(UncompressedPointer<T>& a,
                                     UncompressedPointer<T>& b) {
  a.swap(b);
}

// operators==.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator==(UncompressedPointer<T> a,
                                           UncompressedPointer<U> b) {
  return a.get() == b.get();
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator==(UncompressedPointer<T> a, U* b) {
  return a == static_cast<UncompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator==(T* a, UncompressedPointer<U> b) {
  return b == a;
}

template <typename T>
PA_ALWAYS_INLINE constexpr bool operator==(UncompressedPointer<T> a,
                                           std::nullptr_t) {
  return !a.is_nonnull();
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator==(std::nullptr_t,
                                           UncompressedPointer<U> b) {
  return b == nullptr;
}

// operators!=.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator!=(UncompressedPointer<T> a,
                                           UncompressedPointer<U> b) {
  return !(a == b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator!=(UncompressedPointer<T> a, U* b) {
  return a != static_cast<UncompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator!=(T* a, UncompressedPointer<U> b) {
  return b != a;
}

template <typename T>
PA_ALWAYS_INLINE constexpr bool operator!=(UncompressedPointer<T> a,
                                           std::nullptr_t) {
  return a.is_nonnull();
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator!=(std::nullptr_t,
                                           UncompressedPointer<U> b) {
  return b != nullptr;
}

// operators<.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<(UncompressedPointer<T> a,
                                          UncompressedPointer<U> b) {
  return a.get() < b.get();
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<(UncompressedPointer<T> a, U* b) {
  return a < static_cast<UncompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<(T* a, UncompressedPointer<U> b) {
  return static_cast<UncompressedPointer<T>>(a) < b;
}

// operators<=.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<=(UncompressedPointer<T> a,
                                           UncompressedPointer<U> b) {
  return a.get() <= b.get();
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<=(UncompressedPointer<T> a, U* b) {
  return a <= static_cast<UncompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator<=(T* a, UncompressedPointer<U> b) {
  return static_cast<UncompressedPointer<T>>(a) <= b;
}

// operators>.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>(UncompressedPointer<T> a,
                                          UncompressedPointer<U> b) {
  return !(a <= b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>(UncompressedPointer<T> a, U* b) {
  return a > static_cast<UncompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>(T* a, UncompressedPointer<U> b) {
  return static_cast<UncompressedPointer<T>>(a) > b;
}

// operators>=.
template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>=(UncompressedPointer<T> a,
                                           UncompressedPointer<U> b) {
  return !(a < b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>=(UncompressedPointer<T> a, U* b) {
  return a >= static_cast<UncompressedPointer<U>>(b);
}

template <typename T, typename U>
PA_ALWAYS_INLINE constexpr bool operator>=(T* a, UncompressedPointer<U> b) {
  return static_cast<UncompressedPointer<T>>(a) >= b;
}

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_COMPRESSED_POINTER_H_
