// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_ATOMIC_OPERATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_ATOMIC_OPERATIONS_H_

#include <atomic>
#include <cstddef>
#include <type_traits>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

// TOOD(omerkatz): Replace these casts with std::atomic_ref (C++20) once it
// becomes available.
template <typename T>
ALWAYS_INLINE std::atomic<T>* AsAtomicPtr(T* t) {
  return reinterpret_cast<std::atomic<T>*>(t);
}

template <typename T>
ALWAYS_INLINE const std::atomic<T>* AsAtomicPtr(const T* t) {
  return reinterpret_cast<const std::atomic<T>*>(t);
}

// Copies |bytes| bytes from |from| to |to| using atomic reads. Assumes |to|
// and |from| are size_t-aligned (or halfword-aligned for 64-bit platforms) and
// point to buffers of size |bytes|. Note that atomicity is guaranteed only per
// word/halfword, not for the entire |bytes| bytes as a whole. The function
// copies elements one by one, so overlapping regions are not supported.
WTF_EXPORT void AtomicReadMemcpy(void* to, const void* from, size_t bytes);

namespace internal {

template <size_t bytes, typename AlignmentType>
ALWAYS_INLINE void AtomicReadMemcpyAligned(void* to, const void* from) {
  static constexpr size_t kAlignment = sizeof(AlignmentType);

  DCHECK_EQ(0u, reinterpret_cast<uintptr_t>(to) & (kAlignment - 1));
  DCHECK_EQ(0u, reinterpret_cast<uintptr_t>(from) & (kAlignment - 1));

#if defined(ARCH_CPU_64_BITS)
  if constexpr (bytes == sizeof(uint32_t)) {
    *reinterpret_cast<uint32_t*>(to) =
        AsAtomicPtr(reinterpret_cast<const uint32_t*>(from))
            ->load(std::memory_order_relaxed);
    return;
  }
#endif  // defined(ARCH_CPU_64_BITS)

  if constexpr (bytes % kAlignment == 0 && bytes >= kAlignment &&
                bytes <= 3 * kAlignment) {
    *reinterpret_cast<AlignmentType*>(to) =
        AsAtomicPtr(reinterpret_cast<const AlignmentType*>(from))
            ->load(std::memory_order_relaxed);
    if constexpr (bytes >= 2 * kAlignment) {
      *(reinterpret_cast<AlignmentType*>(to) + 1) =
          AsAtomicPtr(reinterpret_cast<const AlignmentType*>(from) + 1)
              ->load(std::memory_order_relaxed);
    }
    if constexpr (bytes == 3 * kAlignment) {
      *(reinterpret_cast<AlignmentType*>(to) + 2) =
          AsAtomicPtr(reinterpret_cast<const AlignmentType*>(from) + 2)
              ->load(std::memory_order_relaxed);
    }
  } else {
    AtomicReadMemcpy(to, from, bytes);
  }
}

}  // namespace internal

template <size_t bytes, size_t alignment>
ALWAYS_INLINE void AtomicReadMemcpy(void* to, const void* from) {
  static_assert(bytes > 0, "Number of copied bytes should be greater than 0");

  if constexpr (alignment == sizeof(size_t)) {
    internal::AtomicReadMemcpyAligned<bytes, size_t>(to, from);
  } else if constexpr (alignment == sizeof(uint32_t)) {
    internal::AtomicReadMemcpyAligned<bytes, uint32_t>(to, from);
  } else {
    AtomicReadMemcpy(to, from, bytes);
  }
}

// Copies |bytes| bytes from |from| to |to| using atomic writes. Assumes |to|
// and |from| are size_t-aligned (or halfword-aligned for 64-bit platforms) and
// point to buffers of size |bytes|. Note that atomicity is guaranteed only per
// word/halfword, not for the entire |bytes| bytes as a whole. The function
// copies elements one by one, so overlapping regions are not supported.
WTF_EXPORT void AtomicWriteMemcpy(void* to, const void* from, size_t bytes);

namespace internal {

template <size_t bytes, typename AlignmentType>
ALWAYS_INLINE void AtomicWriteMemcpyAligned(void* to, const void* from) {
  static constexpr size_t kAlignment = sizeof(AlignmentType);

  DCHECK_EQ(0u, reinterpret_cast<uintptr_t>(to) & (kAlignment - 1));
  DCHECK_EQ(0u, reinterpret_cast<uintptr_t>(from) & (kAlignment - 1));

#if defined(ARCH_CPU_64_BITS)
  if constexpr (bytes == sizeof(uint32_t)) {
    AsAtomicPtr(reinterpret_cast<uint32_t*>(to))
        ->store(*reinterpret_cast<const uint32_t*>(from),
                std::memory_order_relaxed);
    return;
  }
#endif  // defined(ARCH_CPU_64_BITS)

  if constexpr (bytes % kAlignment == 0 && bytes >= kAlignment &&
                bytes <= 3 * kAlignment) {
    AsAtomicPtr(reinterpret_cast<AlignmentType*>(to))
        ->store(*reinterpret_cast<const AlignmentType*>(from),
                std::memory_order_relaxed);
    if constexpr (bytes >= 2 * kAlignment) {
      AsAtomicPtr(reinterpret_cast<AlignmentType*>(to) + 1)
          ->store(*(reinterpret_cast<const AlignmentType*>(from) + 1),
                  std::memory_order_relaxed);
    }
    if constexpr (bytes == 3 * kAlignment) {
      AsAtomicPtr(reinterpret_cast<AlignmentType*>(to) + 2)
          ->store(*(reinterpret_cast<const AlignmentType*>(from) + 2),
                  std::memory_order_relaxed);
    }
  } else {
    AtomicWriteMemcpy(to, from, bytes);
  }
}

}  // namespace internal

template <size_t bytes, size_t alignment>
ALWAYS_INLINE void AtomicWriteMemcpy(void* to, const void* from) {
  static_assert(bytes > 0, "Number of copied bytes should be greater than 0");
  if constexpr (alignment == sizeof(size_t)) {
    internal::AtomicWriteMemcpyAligned<bytes, size_t>(to, from);
  } else if constexpr (alignment == sizeof(uint32_t)) {
    internal::AtomicWriteMemcpyAligned<bytes, uint32_t>(to, from);
  } else {
    AtomicWriteMemcpy(to, from, bytes);
  }
}

// Set the first |bytes| bytes of |buf| to 0 using atomic writes. Assumes |buf|
// is size_t-aligned (or halfword-aligned for 64-bit platforms) and points to a
// buffer of size at least |bytes|. Note that atomicity is guaranteed only per
// word/halfword, not for the entire |bytes| bytes as a whole.
WTF_EXPORT void AtomicMemzero(void* buf, size_t bytes);

namespace internal {

template <size_t bytes, typename AlignmentType>
ALWAYS_INLINE void AtomicMemzeroAligned(void* buf) {
  static constexpr size_t kAlignment = sizeof(AlignmentType);

  DCHECK_EQ(0u, reinterpret_cast<size_t>(buf) & (kAlignment - 1));

#if defined(ARCH_CPU_64_BITS)
  if constexpr (bytes == sizeof(uint32_t)) {
    AsAtomicPtr(reinterpret_cast<uint32_t*>(buf))
        ->store(0, std::memory_order_relaxed);
    return;
  }
#endif  // defined(ARCH_CPU_64_BITS)

  if constexpr (bytes % kAlignment == 0 && bytes >= kAlignment &&
                bytes <= 3 * kAlignment) {
    AsAtomicPtr(reinterpret_cast<AlignmentType*>(buf))
        ->store(0, std::memory_order_relaxed);
    if constexpr (bytes >= 2 * kAlignment) {
      AsAtomicPtr(reinterpret_cast<AlignmentType*>(buf) + 1)
          ->store(0, std::memory_order_relaxed);
    }
    if constexpr (bytes == 3 * kAlignment) {
      AsAtomicPtr(reinterpret_cast<AlignmentType*>(buf) + 2)
          ->store(0, std::memory_order_relaxed);
    }
  } else {
    AtomicMemzero(buf, bytes);
  }
}

}  // namespace internal

template <size_t bytes, size_t alignment>
ALWAYS_INLINE void AtomicMemzero(void* buf) {
  static_assert(bytes > 0, "Number of copied bytes should be greater than 0");
  if constexpr (alignment == sizeof(size_t)) {
    internal::AtomicMemzeroAligned<bytes, size_t>(buf);
  } else if constexpr (alignment == sizeof(uint32_t)) {
    internal::AtomicMemzeroAligned<bytes, uint32_t>(buf);
  } else {
    AtomicMemzero(buf, bytes);
  }
}

// Swaps values using atomic writes.
template <typename T>
ALWAYS_INLINE void AtomicWriteSwap(T& lhs, T& rhs) {
  T tmp_val = rhs;
  AsAtomicPtr(&rhs)->store(lhs, std::memory_order_relaxed);
  AsAtomicPtr(&lhs)->store(tmp_val, std::memory_order_relaxed);
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_ATOMIC_OPERATIONS_H_
