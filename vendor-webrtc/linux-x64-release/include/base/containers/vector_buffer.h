// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/390223051): Remove C-library calls to fix the errors.
#pragma allow_unsafe_libc_calls
#endif

#ifndef BASE_CONTAINERS_VECTOR_BUFFER_H_
#define BASE_CONTAINERS_VECTOR_BUFFER_H_

#include <stdlib.h>
#include <string.h>

#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/numerics/checked_math.h"

namespace base::internal {

// Internal implementation detail of base/containers.
//
// Implements a vector-like buffer that holds a certain capacity of T. Unlike
// std::vector, VectorBuffer never constructs or destructs its arguments, and
// can't change sizes. But it does implement templates to assist in efficient
// moving and destruction of those items manually.
//
// In particular, the destructor function does not iterate over the items if
// there is no destructor. Moves should be implemented as a memcpy/memmove for
// trivially copyable objects (POD) otherwise, it should be a std::move if
// possible, and as a last resort it falls back to a copy. This behavior is
// similar to std::vector.
//
// No special consideration is done for noexcept move constructors since
// we compile without exceptions.
//
// The current API does not support moving overlapping ranges.
template <typename T>
class VectorBuffer {
 public:
  constexpr VectorBuffer() = default;

#if defined(__clang__) && !defined(__native_client__)
  // This constructor converts an uninitialized void* to a T* which triggers
  // clang Control Flow Integrity. Since this is as-designed, disable.
  __attribute__((no_sanitize("cfi-unrelated-cast", "vptr")))
#endif
  VectorBuffer(size_t count)
      : buffer_(reinterpret_cast<T*>(
            malloc(CheckMul(sizeof(T), count).ValueOrDie()))),
        capacity_(count) {
  }
  VectorBuffer(VectorBuffer&& other) noexcept
      : buffer_(other.buffer_), capacity_(other.capacity_) {
    other.buffer_ = nullptr;
    other.capacity_ = 0;
  }

  VectorBuffer(const VectorBuffer&) = delete;
  VectorBuffer& operator=(const VectorBuffer&) = delete;

  ~VectorBuffer() { free(buffer_); }

  VectorBuffer& operator=(VectorBuffer&& other) {
    free(buffer_);
    buffer_ = other.buffer_;
    capacity_ = other.capacity_;

    other.buffer_ = nullptr;
    other.capacity_ = 0;
    return *this;
  }

  size_t capacity() const { return capacity_; }

  T& operator[](size_t i) {
    // TODO(crbug.com/40565371): Some call sites (at least circular_deque.h) are
    // calling this with `i == capacity_` as a way of getting `end()`. Therefore
    // we have to allow this for now (`i <= capacity_`), until we fix those call
    // sites to use real iterators. This comment applies here and to `const T&
    // operator[]`, below.
    CHECK_LT(i, capacity_);
    // SAFETY: `capacity_` is the size of the array pointed to by `buffer_`,
    // which `i` is less than, so the dereference is inside the allocation.
    return UNSAFE_BUFFERS(buffer_[i]);
  }

  const T& operator[](size_t i) const {
    CHECK_LT(i, capacity_);
    // SAFETY: `capacity_` is the size of the array pointed to by `buffer_`,
    // which `i` is less than, so the dereference is inside the allocation.
    return UNSAFE_BUFFERS(buffer_[i]);
  }

  const T* data() const { return buffer_; }
  T* data() { return buffer_; }

  T* begin() { return buffer_; }
  T* end() {
    // SAFETY: `capacity_` is the size of the array pointed to by `buffer_`.
    return UNSAFE_BUFFERS(buffer_ + capacity_);
  }

  span<T> as_span() {
    // SAFETY: The `buffer_` array's size is `capacity_` so this gives the
    // pointer to the start and one-past-the-end of the `buffer_`.
    return UNSAFE_BUFFERS(span(buffer_, buffer_ + capacity_));
  }

  span<T> subspan(size_t index) { return as_span().subspan(index); }

  span<T> subspan(size_t index, size_t size) {
    return as_span().subspan(index, size);
  }

  T* get_at(size_t index) { return as_span().get_at(index); }

  // DestructRange ------------------------------------------------------------

  static void DestructRange(span<T> range) {
    // Trivially destructible objects need not have their destructors called.
    if constexpr (!std::is_trivially_destructible_v<T>) {
      for (T& t : range) {
        t.~T();
      }
    }
  }

  // MoveRange ----------------------------------------------------------------

  template <typename T2>
  static inline constexpr bool is_trivially_copyable_or_relocatable =
      std::is_trivially_copyable_v<T2> || IS_TRIVIALLY_RELOCATABLE(T2);

  // Moves or copies elements from the `from` span to the `to` span. Uses memcpy
  // when possible. The memory in `to` must be uninitialized and the ranges must
  // not overlap.
  //
  // The objects in `from` are destroyed afterward.
  static void MoveConstructRange(span<T> from, span<T> to) {
    CHECK(!RangesOverlap(from, to));
    CHECK_EQ(from.size(), to.size());

    if constexpr (is_trivially_copyable_or_relocatable<T>) {
      // We can't use span::copy_from() as it tries to call copy constructors,
      // and fails to compile on move-only trivially-relocatable types.
      memcpy(to.data(), from.data(), to.size_bytes());
      // Destructors are skipped because they are trivial or should be elided in
      // trivial relocation (https://reviews.llvm.org/D114732).
    } else {
      for (size_t i = 0; i < from.size(); ++i) {
        T* to_pointer = to.subspan(i).data();
        if constexpr (std::move_constructible<T>) {
          new (to_pointer) T(std::move(from[i]));
        } else {
          new (to_pointer) T(from[i]);
        }
        from[i].~T();
      }
    }
  }

 private:
  static bool RangesOverlap(span<T> a, span<T> b) {
    const auto a_start = reinterpret_cast<uintptr_t>(a.data());
    const auto a_end = reinterpret_cast<uintptr_t>(a.data()) + a.size();
    const auto b_start = reinterpret_cast<uintptr_t>(b.data());
    const auto b_end = reinterpret_cast<uintptr_t>(b.data()) + b.size();
    return a_end > b_start && a_start < b_end;
  }

  // `buffer_` is not a raw_ptr<...> for performance reasons (based on analysis
  // of sampling profiler data and tab_search:top100:2020).
  RAW_PTR_EXCLUSION T* buffer_ = nullptr;
  size_t capacity_ = 0;
};

}  // namespace base::internal

#endif  // BASE_CONTAINERS_VECTOR_BUFFER_H_
