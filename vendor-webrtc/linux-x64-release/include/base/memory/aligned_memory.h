// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_ALIGNED_MEMORY_H_
#define BASE_MEMORY_ALIGNED_MEMORY_H_

#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <bit>
#include <ostream>

#include "base/base_export.h"
#include "base/check.h"
#include "base/containers/heap_array.h"
#include "base/containers/span.h"
#include "build/build_config.h"

#if defined(COMPILER_MSVC)
#include <malloc.h>
#else
#include <stdlib.h>
#endif

// A runtime sized aligned allocation for objects of type `T` with a runtime
// sized alignment:
//
//   base::AlignedHeapArray<float> array = base::AlignedUninit<float>(
//       size, alignment);
//   CHECK(reinterpret_cast<uintptr_t>(array.data()) % alignment == 0);
//
// A runtime sized aligned allocation for objects of type `T` but represented as
// a char array, along with a span accessing that memory as `T*` for in-place
// construction:
//
//   auto [a, s] = base::AlignedUninitCharArray<float>(size, alignment);
//   base::AlignedHeapArray<char> array = std::move(a);
//   base::span<float> span = s;
//   CHECK(reinterpret_cast<uintptr_t>(array.data()) % alignment == 0);
//   CHECK(reinterpret_cast<uintptr_t>(span.data()) % alignment == 0);
//
// With manual memory management, a runtime sized aligned allocation can be
// created:
//
//   float* my_array = static_cast<float*>(AlignedAlloc(size, alignment));
//   CHECK(reinterpret_cast<uintptr_t>(my_array) % alignment == 0);
//   memset(my_array, 0, size);  // fills entire object.
//
//   // ... later, to release the memory:
//   AlignedFree(my_array);

namespace base {

// Allocate memory of size `size` aligned to `alignment`.
//
// Prefer `AlignedUninit()` to make a `base::HeapArray` that has a runtime-sized
// alignment.
//
// When the caller will be managing the lifetimes of the objects in the array
// with in-place construction and destruction, `AlignedUninitCharArray()`
// provides safe ownership of the memory and access to memory aligned for `T` as
// `span<T>`.
//
// TODO(https://crbug.com/40255447): Convert usage to / convert to use
// `std::aligned_alloc` to the extent that it can be done (since
// `std::aligned_alloc` can't be used on Windows). When that happens, note that
// `std::aligned_alloc` requires the `size` parameter be an integral multiple of
// `alignment` while this implementation does not.
BASE_EXPORT void* AlignedAlloc(size_t size, size_t alignment);

// Deallocate memory allocated by `AlignedAlloc`.
inline void AlignedFree(void* ptr) {
#if defined(COMPILER_MSVC)
  _aligned_free(ptr);
#else
  free(ptr);
#endif
}

// Deleter for use with unique_ptr. E.g., use as
//   std::unique_ptr<Foo, base::AlignedFreeDeleter> foo;
struct AlignedFreeDeleter {
  inline void operator()(void* ptr) const { AlignedFree(ptr); }
};

template <class T>
using AlignedHeapArray = HeapArray<T, AlignedFreeDeleter>;

// Constructs a `base::AlignedHeapArray<T>` that is sized to hold `capacity`
// many objects of type `T` and is aligned to `alignment`. The memory is
// uninitialized.
//
// The `alignment` defaults to `alignof(T)` and can be omitted, but the
// alignment used will always be at least the alignment of a pointer.
template <class T>
AlignedHeapArray<T> AlignedUninit(size_t capacity,
                                  size_t alignment = alignof(T)) {
  alignment = std::max(alignment, alignof(void*));
  CHECK_GE(alignment, alignof(T));
  CHECK_LE(capacity, SIZE_MAX / sizeof(T));
  const size_t bytes = capacity * sizeof(T);
  // SAFETY: AlignedAlloc() allocates `bytes` many chars, which has room for
  // `capacity` many `T` objects by construction, so we specify `capacity` as
  // the size of the `HeapArray<T>`.
  return UNSAFE_BUFFERS(HeapArray<T, AlignedFreeDeleter>::FromOwningPointer(
      static_cast<T*>(AlignedAlloc(bytes, alignment)), capacity));
}

// Constructs a AlignedHeapArray<char> that is sized to hold `capacity` many
// objects of type `T` and is aligned to `alignment`.
//
// The `alignment` defaults to `alignof(T)` and can be omitted, but the
// alignment used will always be at least the alignment of a pointer.
//
// Returns a pair of `AlignedHeapArray<char>` and a `span<T>` for the entire
// _uninitialized_ `AlignedHeapArray`.
//
// It is up to the caller to construct objects of type `T` in the array
// in-place, and to destruct them before destroying the `AlignedHeapArray`.
//
// Note that using `span[index]` to make a reference to uninitialized memory is
// undefined behaviour. In-place construction must use the unsafe `span.data() +
// index` to avoid constructing a reference.
template <class T>
auto AlignedUninitCharArray(size_t capacity, size_t alignment = alignof(T)) {
  alignment = std::max(alignment, alignof(void*));
  CHECK_GE(alignment, alignof(T));
  CHECK_LE(capacity, SIZE_MAX / sizeof(T));
  const size_t bytes = capacity * sizeof(T);
  // SAFETY: AlignedAlloc() allocates `bytes` many chars, and we give the same
  // `bytes` as the size for `HeapArray`.
  auto uninit_array =
      UNSAFE_BUFFERS(HeapArray<char, AlignedFreeDeleter>::FromOwningPointer(
          static_cast<char*>(AlignedAlloc(bytes, alignment)), bytes));
  // SAFETY: `uninit_array` holds `capacity * sizeof(T)` bytes, so it has room
  // for `capacity` many objects of type `T`.
  auto uninit_span =
      UNSAFE_BUFFERS(span(reinterpret_cast<T*>(uninit_array.data()), capacity));
  return std::make_pair(std::move(uninit_array), std::move(uninit_span));
}

#ifdef __has_builtin
#define SUPPORTS_BUILTIN_IS_ALIGNED (__has_builtin(__builtin_is_aligned))
#else
#define SUPPORTS_BUILTIN_IS_ALIGNED 0
#endif

inline bool IsAligned(uintptr_t val, size_t alignment) {
  // If the compiler supports builtin alignment checks prefer them.
#if SUPPORTS_BUILTIN_IS_ALIGNED
  return __builtin_is_aligned(val, alignment);
#else
  DCHECK(std::has_single_bit(alignment)) << alignment << " is not a power of 2";
  return (val & (alignment - 1)) == 0;
#endif
}

#undef SUPPORTS_BUILTIN_IS_ALIGNED

inline bool IsAligned(const void* val, size_t alignment) {
  return IsAligned(reinterpret_cast<uintptr_t>(val), alignment);
}

}  // namespace base

#endif  // BASE_MEMORY_ALIGNED_MEMORY_H_
