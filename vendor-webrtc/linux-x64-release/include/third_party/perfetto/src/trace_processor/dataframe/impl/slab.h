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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_SLAB_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_SLAB_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <type_traits>

#include "perfetto/ext/base/utils.h"
#include "perfetto/public/compiler.h"

namespace perfetto::trace_processor::dataframe::impl {
namespace internal {
static constexpr bool IsPowerOfTwo(size_t n) {
  return (n & (n - 1)) == 0;
}
}  // namespace internal

// A memory-aligned contiguous block of trivially constructible and destructible
// elements. Basically just a thin wrapper around a std::unique_ptr and a size
// but with enforced alignment and additional compile-time checks.
//
// This class enforces several important constraints:
// - Elements must be trivially constructible and destructible
// - The alignment must be at least as strict as the element's alignment
// - The alignment must be a power of two
//
// Usage example:
//   auto slab = Slab<float>::Alloc(1024);  // Allocates space for 1024 floats
//   for (size_t i = 0; i < slab.size(); ++i) {
//     slab[i] = static_cast<float>(i);
//   }
template <typename T, size_t kAlignment = std::max<size_t>(alignof(T), 64)>
class Slab {
 public:
  static_assert(std::is_trivially_constructible_v<T>,
                "Slab elements must be trivially constructible");
  static_assert(std::is_trivially_destructible_v<T>,
                "Slab elements must be trivially destructible");
  static_assert(alignof(T) <= kAlignment,
                "Alignment must be at least as strict as element alignment");
  static_assert(internal::IsPowerOfTwo(kAlignment),
                "Alignment must be a power of two");

  using value_type = T;
  using const_iterator = const T*;

  // Default constructor creates an empty slab.
  Slab() = default;

  // Move operations are supported.
  Slab(Slab&&) = default;
  Slab& operator=(Slab&&) = default;

  // Copy operations are deleted to avoid accidental copies.
  Slab(const Slab&) = delete;
  Slab& operator=(const Slab&) = delete;

  // Allocates a new slab with the specified number of elements.
  //
  // size: Number of elements to allocate space for.
  // Returns a new Slab object with the requested capacity.
  static Slab<T, kAlignment> Alloc(uint64_t size) {
    return Slab(
        static_cast<T*>(base::AlignedAlloc(kAlignment, size * sizeof(T))),
        size);
  }

  // Returns a pointer to the underlying data.
  PERFETTO_ALWAYS_INLINE const T* data() const { return data_.get(); }
  PERFETTO_ALWAYS_INLINE T* data() { return data_.get(); }

  // Returns the number of elements in the slab.
  PERFETTO_ALWAYS_INLINE uint64_t size() const { return size_; }

  // Returns iterators for range-based for loops.
  PERFETTO_ALWAYS_INLINE T* begin() const { return data_.get(); }
  PERFETTO_ALWAYS_INLINE T* end() const { return data_.get() + size_; }

  // Provides indexed access to elements.
  PERFETTO_ALWAYS_INLINE T& operator[](uint64_t i) const { return data_[i]; }

 private:
  // Constructor used by Alloc.
  Slab(T* data, uint64_t size) : data_(data), size_(size) {}

  // Aligned unique pointer that holds the allocated memory.
  base::AlignedUniquePtr<T[]> data_;

  // Number of elements in the slab.
  uint64_t size_ = 0;
};

}  // namespace perfetto::trace_processor::dataframe::impl

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_SLAB_H_
