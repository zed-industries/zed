// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_HEAP_ARRAY_H_
#define BASE_CONTAINERS_HEAP_ARRAY_H_

#include <stddef.h>

#include <memory>
#include <type_traits>
#include <utility>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"

namespace base {

// HeapArray<T> is a replacement for std::unique_ptr<T[]> that keeps track
// of its size. It is intended to provide easy conversion to span<T> for most
// usage, but it also provides bounds-checked indexing.
//
// By default, elements in the array are either value-initialized (i.e. zeroed
// for primitive types) when the array is created using the WithSize()
// static method, or uninitialized when the array is created via the Uninit()
// static method.
template <typename T, typename Deleter = void>
class TRIVIAL_ABI GSL_OWNER HeapArray {
 public:
  static_assert(!std::is_const_v<T>, "HeapArray cannot hold const types");
  static_assert(!std::is_reference_v<T>,
                "HeapArray cannot hold reference types");

  using iterator = base::span<T>::iterator;
  using const_iterator = base::span<const T>::iterator;
  // We don't put this default value in the template parameter list to allow the
  // static_assert on is_reference_v to give a nicer error message.
  using deleter_type = std::
      conditional_t<std::is_void_v<Deleter>, std::default_delete<T[]>, Deleter>;

  // Allocates initialized memory capable of holding `size` elements. No memory
  // is allocated for zero-sized arrays.
  static HeapArray WithSize(size_t size)
    requires(std::constructible_from<T>)
  {
    if (!size) {
      return HeapArray();
    }
    return HeapArray(std::unique_ptr<T[], deleter_type>(new T[size]()), size);
  }

  // Allocates uninitialized memory capable of holding `size` elements. T must
  // be trivially constructible and destructible. No memory is allocated for
  // zero-sized arrays.
  static HeapArray Uninit(size_t size)
    requires(std::is_trivially_constructible_v<T> &&
             std::is_trivially_destructible_v<T>)
  {
    if (!size) {
      return HeapArray();
    }
    return HeapArray(std::unique_ptr<T[], deleter_type>(new T[size]), size);
  }

  static HeapArray CopiedFrom(base::span<const T> that) {
    auto result = HeapArray::Uninit(that.size());
    result.copy_from(that);
    return result;
  }

  // Constructs a HeapArray from an existing pointer, taking ownership of the
  // pointer.
  //
  // # Safety
  // The pointer must be correctly aligned for type `T` and able to be deleted
  // through the `deleter_type`, which defaults to the `delete[]` operation. The
  // `ptr` must point to an array of at least `size` many elements. If these are
  // not met, then Undefined Behaviour can result.
  //
  // # Checks
  // When the `size` is zero, the `ptr` must be null.
  UNSAFE_BUFFER_USAGE static HeapArray FromOwningPointer(T* ptr, size_t size) {
    if (!size) {
      CHECK_EQ(ptr, nullptr);
      return HeapArray();
    }
    return HeapArray(std::unique_ptr<T[], deleter_type>(ptr), size);
  }

  // Constructs an empty array and does not allocate any memory.
  HeapArray()
    requires(std::constructible_from<T>)
  = default;

  // Move-only type since the memory is owned.
  HeapArray(const HeapArray&) = delete;
  HeapArray& operator=(const HeapArray&) = delete;

  // Move-construction leaves the moved-from object empty and containing
  // no allocated memory.
  HeapArray(HeapArray&& that)
      : data_(std::move(that.data_)), size_(std::exchange(that.size_, 0u)) {}

  // Move-assigment leaves the moved-from object empty and containing
  // no allocated memory.
  HeapArray& operator=(HeapArray&& that) {
    data_ = std::move(that.data_);
    size_ = std::exchange(that.size_, 0u);
    return *this;
  }
  ~HeapArray() = default;

  bool empty() const { return size_ == 0u; }
  size_t size() const { return size_; }

  // Prefer span-based methods below over data() where possible. The data()
  // method exists primarily to allow implicit constructions of spans.
  // Returns nullptr for a zero-sized (or moved-from) array.
  T* data() LIFETIME_BOUND { return data_.get(); }
  const T* data() const LIFETIME_BOUND { return data_.get(); }

  iterator begin() LIFETIME_BOUND { return as_span().begin(); }
  const_iterator begin() const LIFETIME_BOUND { return as_span().begin(); }

  iterator end() LIFETIME_BOUND { return as_span().end(); }
  const_iterator end() const LIFETIME_BOUND { return as_span().end(); }

  ALWAYS_INLINE T& operator[](size_t idx) LIFETIME_BOUND {
    CHECK_LT(idx, size_);
    // SAFETY: bounds checked above.
    return UNSAFE_BUFFERS(data_.get()[idx]);
  }
  ALWAYS_INLINE const T& operator[](size_t idx) const LIFETIME_BOUND {
    CHECK_LT(idx, size_);
    // SAFETY: bounds checked above.
    return UNSAFE_BUFFERS(data_.get()[idx]);
  }

  // Access the HeapArray via spans. Note that span<T> is implicilty
  // constructible from HeapArray<T>, so an explicit call to .as_span() is
  // most useful, say, when the compiler can't deduce a template
  // argument type.
  ALWAYS_INLINE base::span<T> as_span() LIFETIME_BOUND {
    // SAFETY: `size_` is the number of elements in the `data_` allocation` at
    // all times.
    return UNSAFE_BUFFERS(base::span<T>(data_.get(), size_));
  }
  ALWAYS_INLINE base::span<const T> as_span() const LIFETIME_BOUND {
    // SAFETY: `size_` is the number of elements in the `data_` allocation` at
    // all times.
    return UNSAFE_BUFFERS(base::span<const T>(data_.get(), size_));
  }

  // Convenience method to copy the contents of a span<> into the entire array.
  // Hard CHECK occurs in span<>::copy_from() if the HeapArray and the span
  // have different sizes.
  void copy_from(base::span<const T> other) { as_span().copy_from(other); }

  // Convenience method to copy the contents of a span<> into the start of the
  // array. Hard CHECK occurs in span<>::copy_prefix_from() if the HeapArray
  // isn't large enough to contain the entire span.
  void copy_prefix_from(base::span<const T> other) {
    as_span().copy_prefix_from(other);
  }

  // Convenience methods to slice the vector into spans.
  // Returns a span over the HeapArray starting at `offset` of `count` elements.
  // If `count` is unspecified, all remaining elements are included. A CHECK()
  // occurs if any of the parameters results in an out-of-range position in
  // the HeapArray.
  base::span<T> subspan(size_t offset) LIFETIME_BOUND {
    return as_span().subspan(offset);
  }
  base::span<const T> subspan(size_t offset) const LIFETIME_BOUND {
    return as_span().subspan(offset);
  }
  base::span<T> subspan(size_t offset, size_t count) LIFETIME_BOUND {
    return as_span().subspan(offset, count);
  }
  base::span<const T> subspan(size_t offset,
                              size_t count) const LIFETIME_BOUND {
    return as_span().subspan(offset, count);
  }

  // Returns a span over the first `count` elements of the HeapArray. A CHECK()
  // occurs if the `count` is larger than size of the HeapArray.
  base::span<T> first(size_t count) LIFETIME_BOUND {
    return as_span().first(count);
  }
  base::span<const T> first(size_t count) const LIFETIME_BOUND {
    return as_span().first(count);
  }

  // Returns a span over the last `count` elements of the HeapArray. A CHECK()
  // occurs if the `count` is larger than size of the HeapArray.
  base::span<T> last(size_t count) LIFETIME_BOUND {
    return as_span().last(count);
  }
  base::span<const T> last(size_t count) const LIFETIME_BOUND {
    return as_span().last(count);
  }

  // Leaks the memory in the HeapArray so that it will never be freed, and
  // consumes the HeapArray, returning an unowning span that points to the
  // memory.
  base::span<T> leak() && {
    HeapArray<T> dropped = std::move(*this);
    T* leaked = dropped.data_.release();
    // SAFETY: The `size_` is the number of elements in the allocation in
    // `data_` at all times, which is renamed as `leaked` here.
    return UNSAFE_BUFFERS(span(leaked, dropped.size_));
  }

  // Allows construction of a smaller HeapArray from an existing HeapArray w/o
  // reallocations or copies. Note: The original allocation is preserved, so
  // CopiedFrom() should be preferred for significant size reductions.
  base::HeapArray<T> take_first(size_t reduced_size) && {
    CHECK_LE(reduced_size, size_);
    size_ = 0u;
    if (!reduced_size) {
      data_.reset();
    }
    return base::HeapArray(std::move(data_), reduced_size);
  }

  // Delete the memory previously obtained from leak(). Argument is a pointer
  // rather than a span to facilitate use by callers that have lost track of
  // size information, as may happen when being passed through a C-style
  // function callback. The void* argument type makes its signature compatible
  // with typical void (*cb)(void*) C-style deletion callback.
  static void DeleteLeakedData(void* ptr) {
    // Memory is freed by unique ptr going out of scope.
    std::unique_ptr<T[], deleter_type> deleter(static_cast<T*>(ptr));
  }

 private:
  HeapArray(std::unique_ptr<T[], deleter_type> data, size_t size)
      : data_(std::move(data)), size_(size) {}

  std::unique_ptr<T[], deleter_type> data_;
  size_t size_ = 0u;
};

}  // namespace base

#endif  // BASE_CONTAINERS_HEAP_ARRAY_H_
