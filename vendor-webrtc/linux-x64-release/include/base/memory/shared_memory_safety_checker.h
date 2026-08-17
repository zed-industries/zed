// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_SHARED_MEMORY_SAFETY_CHECKER_H_
#define BASE_MEMORY_SHARED_MEMORY_SAFETY_CHECKER_H_

#include <array>
#include <atomic>
#include <type_traits>

#include "base/containers/span.h"

namespace base::subtle {

// Constraints on types that can be copied across memory spaces. This is a
// non-exhaustive list and further constraints may be added in the future.

// `kIsAllowed` is true unless T is known to be dangerous over shared memory.
template <typename T>
struct SharedMemorySafetyChecker {
  // Copying non-trivially-copyable objects across memory spaces is dangerous.
  // This check isn't a separate specialization because many types that match
  // other specializations are also trivially copyable, introducing ambiguity.
  static constexpr bool kIsAllowed = std::is_trivially_copyable_v<T>;
};

// Pointers can't be shared across memory spaces.
template <typename T>
  requires(std::is_pointer_v<T> || std::is_member_pointer_v<T>)
struct SharedMemorySafetyChecker<T> {
  static constexpr bool kIsAllowed = false;
};

// Spans can't be shared across memory spaces.
template <typename ElementType, size_t Extent, typename InternalPtrType>
struct SharedMemorySafetyChecker<span<ElementType, Extent, InternalPtrType>> {
  static constexpr bool kIsAllowed = false;
};

// Atomics are dangerous to share across memory spaces unless they're lock-free.
template <typename T>
struct SharedMemorySafetyChecker<std::atomic<T>> {
  static constexpr bool kIsAllowed = std::atomic<T>::is_always_lock_free &&
                                     SharedMemorySafetyChecker<T>::kIsAllowed;
};

// Each element of an array must itself be safe. Although arrays aren't outright
// banned, prefer to use GetMemoryAsSpan<T> for array-like access.
template <typename T, size_t N>
struct SharedMemorySafetyChecker<T[N]> {
  static constexpr bool kIsAllowed = SharedMemorySafetyChecker<T>::kIsAllowed;
};

template <typename T, size_t N>
struct SharedMemorySafetyChecker<std::array<T, N>> {
  static constexpr bool kIsAllowed = SharedMemorySafetyChecker<T>::kIsAllowed;
};

template <typename T>
concept AllowedOverSharedMemory =
    SharedMemorySafetyChecker<std::remove_cvref_t<T>>::kIsAllowed;

// Convenience alias for atomics that are safe to share across memory spaces.
template <typename T>
  requires AllowedOverSharedMemory<std::atomic<T>>
using SharedAtomic = std::atomic<T>;

}  // namespace base::subtle

#endif  // BASE_MEMORY_SHARED_MEMORY_SAFETY_CHECKER_H_
