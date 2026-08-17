/*
 * Copyright (C) 2025 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_FLEX_VECTOR_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_FLEX_VECTOR_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>

#include "perfetto/base/logging.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/dataframe/impl/slab.h"

namespace perfetto::trace_processor::dataframe::impl {

// A dynamically resizable vector with aligned memory allocation.
//
// FlexVector provides a vector-like container optimized for
// performance-critical operations. It offers significant advantages over
// std::vector:
// 1. Custom memory alignment guarantees for better SIMD performance
// 2. No initialization of elements (avoids constructors for better performance)
// 3. Only works with trivially copyable types for simpler memory management
// 4. Explicit control over memory growth policies
//
// Features:
// - Automatic capacity growth (doubles in capacity when full)
// - Memory alignment for efficient SIMD operations
// - Simple API similar to std::vector but for trivially copyable types only
//
// Performance characteristics:
// - Ensures power-of-two capacity for efficient modulo operations
// - Uses aligned memory for better memory access patterns
// - Provides fast element access with bounds checking in debug mode
//
// Usage example:
//   auto vec = FlexVector<int>::CreateWithCapacity(8);
//   for (int i = 0; i < 20; ++i) {
//     vec.push_back(i);  // Will automatically resize when needed
//   }
template <typename T, uint64_t kAlignment = std::max<uint64_t>(alignof(T), 64)>
class FlexVector {
 public:
  static_assert(std::is_trivially_copyable_v<T>,
                "FlexVector elements must be trivially copyable");
  static_assert(alignof(T) <= kAlignment,
                "Alignment must be at least as strict as element alignment");
  static_assert(internal::IsPowerOfTwo(kAlignment),
                "Alignment must be a power of two");

  // Default constructor creates an empty vector.
  FlexVector() = default;

  // Allocates a new FlexVector with the specified initial capacity.
  //
  // capacity: Initial capacity (number of elements).
  static FlexVector<T, kAlignment> CreateWithCapacity(uint64_t capacity) {
    return FlexVector(capacity == 0 ? 0 : NextPowerOfTwo(capacity), 0);
  }

  // Allocates a new FlexVector with the specified initial size. The values
  // are *not* initialized; this is the main reason why this class exists vs
  // std::vector.
  //
  // size: Initial size (number of elements).
  static FlexVector<T, kAlignment> CreateWithSize(uint64_t size) {
    return FlexVector(size == 0 ? 0 : NextPowerOfTwo(size), size);
  }

  // Adds an element to the end of the vector, automatically resizing if needed.
  //
  // value: The value to append.
  PERFETTO_ALWAYS_INLINE void push_back(T value) {
    PERFETTO_DCHECK(internal::IsPowerOfTwo(capacity()));
    PERFETTO_DCHECK(size_ <= capacity());
    if (PERFETTO_UNLIKELY(size_ == capacity())) {
      // Grow by doubling, at least to capacity 64
      uint64_t new_capacity = std::max<uint64_t>(capacity() * 2, 64ul);
      Slab<T, kAlignment> new_slab = Slab<T, kAlignment>::Alloc(new_capacity);
      if (slab_.size() > 0) {
        // Copy from the original slab data
        memcpy(new_slab.data(), slab_.data(), size_ * sizeof(T));
      }
      slab_ = std::move(new_slab);
    }
    slab_[size_++] = value;
  }

  // Provides indexed access to elements with bounds checking in debug mode.
  PERFETTO_ALWAYS_INLINE T& operator[](uint64_t i) {
    PERFETTO_DCHECK(i < size_);
    return slab_.data()[i];
  }

  PERFETTO_ALWAYS_INLINE const T& operator[](uint64_t i) const {
    PERFETTO_DCHECK(i < size_);
    return slab_.data()[i];
  }

  // Access to the underlying data and size.
  PERFETTO_ALWAYS_INLINE T* data() { return slab_.data(); }
  PERFETTO_ALWAYS_INLINE const T* data() const { return slab_.data(); }
  PERFETTO_ALWAYS_INLINE uint64_t size() const { return size_; }
  PERFETTO_ALWAYS_INLINE bool empty() const { return size() == 0; }

  // Iterators for range-based for loops.
  PERFETTO_ALWAYS_INLINE const T* begin() const { return slab_.data(); }
  PERFETTO_ALWAYS_INLINE const T* end() const { return slab_.data() + size_; }
  PERFETTO_ALWAYS_INLINE T* begin() { return slab_.data(); }
  PERFETTO_ALWAYS_INLINE T* end() { return slab_.data() + size_; }

  PERFETTO_ALWAYS_INLINE const T& back() const {
    PERFETTO_DCHECK(!empty());
    return slab_.data()[size_ - 1];
  }
  PERFETTO_ALWAYS_INLINE T& back() {
    PERFETTO_DCHECK(!empty());
    return slab_.data()[size_ - 1];
  }

  static constexpr uint64_t NextPowerOfTwo(uint64_t x) {
    uint64_t n = x - 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    return n + 1;
  }

  // Returns the current capacity (maximum size without reallocation).
  PERFETTO_ALWAYS_INLINE uint64_t capacity() const { return slab_.size(); }

 private:
  // Constructor used by Alloc.
  explicit FlexVector(uint64_t capacity, uint64_t size)
      : slab_(Slab<T, kAlignment>::Alloc(capacity)), size_(size) {}

  // The underlying memory slab.
  Slab<T, kAlignment> slab_;

  // Current number of elements.
  uint64_t size_ = 0;
};

}  // namespace perfetto::trace_processor::dataframe::impl

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_FLEX_VECTOR_H_
