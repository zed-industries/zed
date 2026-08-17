/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BOUNDED_INLINE_VECTOR_H_
#define RTC_BASE_BOUNDED_INLINE_VECTOR_H_

#include <stdint.h>

#include <memory>
#include <type_traits>
#include <utility>

#include "rtc_base/bounded_inline_vector_impl.h"
#include "rtc_base/checks.h"

namespace webrtc {

// A small std::vector-like type whose capacity is a compile-time constant. It
// stores all data inline and never heap allocates (beyond what its element type
// requires). Trying to grow it beyond its constant capacity is an error.
//
// TODO(bugs.webrtc.org/11391): Comparison operators.
// TODO(bugs.webrtc.org/11391): Methods for adding and deleting elements.
template <typename T, int fixed_capacity>
class BoundedInlineVector {
  static_assert(!std::is_const<T>::value, "T may not be const");
  static_assert(fixed_capacity > 0, "Capacity must be strictly positive");

 public:
  using size_type = int;
  using value_type = T;
  using const_iterator = const T*;

  BoundedInlineVector() = default;
  BoundedInlineVector(const BoundedInlineVector&) = default;
  BoundedInlineVector(BoundedInlineVector&&) = default;
  BoundedInlineVector& operator=(const BoundedInlineVector&) = default;
  BoundedInlineVector& operator=(BoundedInlineVector&&) = default;
  ~BoundedInlineVector() = default;

  // This constructor is implicit, to make it possible to write e.g.
  //
  //   BoundedInlineVector<double, 7> x = {2.72, 3.14};
  //
  // and
  //
  //   BoundedInlineVector<double, 7> GetConstants() {
  //     return {2.72, 3.14};
  //   }
  template <typename... Ts,
            typename std::enable_if_t<
                bounded_inline_vector_impl::AllConvertible<T, Ts...>::value>* =
                nullptr>
  BoundedInlineVector(Ts&&... elements)  // NOLINT(runtime/explicit)
      : storage_(std::forward<Ts>(elements)...) {
    static_assert(sizeof...(Ts) <= fixed_capacity, "");
  }

  template <
      int other_capacity,
      typename std::enable_if_t<other_capacity != fixed_capacity>* = nullptr>
  BoundedInlineVector(const BoundedInlineVector<T, other_capacity>& other) {
    RTC_DCHECK_LE(other.size(), fixed_capacity);
    bounded_inline_vector_impl::CopyElements(other.data(), other.size(),
                                             storage_.data, &storage_.size);
  }

  template <
      int other_capacity,
      typename std::enable_if_t<other_capacity != fixed_capacity>* = nullptr>
  BoundedInlineVector(BoundedInlineVector<T, other_capacity>&& other) {
    RTC_DCHECK_LE(other.size(), fixed_capacity);
    bounded_inline_vector_impl::MoveElements(other.data(), other.size(),
                                             storage_.data, &storage_.size);
  }

  template <
      int other_capacity,
      typename std::enable_if_t<other_capacity != fixed_capacity>* = nullptr>
  BoundedInlineVector& operator=(
      const BoundedInlineVector<T, other_capacity>& other) {
    bounded_inline_vector_impl::DestroyElements(storage_.data, storage_.size);
    RTC_DCHECK_LE(other.size(), fixed_capacity);
    bounded_inline_vector_impl::CopyElements(other.data(), other.size(),
                                             storage_.data, &storage_.size);
    return *this;
  }

  template <
      int other_capacity,
      typename std::enable_if_t<other_capacity != fixed_capacity>* = nullptr>
  BoundedInlineVector& operator=(
      BoundedInlineVector<T, other_capacity>&& other) {
    bounded_inline_vector_impl::DestroyElements(storage_.data, storage_.size);
    RTC_DCHECK_LE(other.size(), fixed_capacity);
    bounded_inline_vector_impl::MoveElements(other.data(), other.size(),
                                             storage_.data, &storage_.size);
    return *this;
  }

  bool empty() const { return storage_.size == 0; }
  int size() const { return storage_.size; }
  constexpr int capacity() const { return fixed_capacity; }

  // Resizes the BoundedInlineVector to the given size, which must not exceed
  // its constant capacity. If the size is increased, the added elements are
  // default constructed.
  void resize(int new_size) {
    RTC_DCHECK_GE(new_size, 0);
    RTC_DCHECK_LE(new_size, fixed_capacity);
    if (new_size > storage_.size) {
      bounded_inline_vector_impl::DefaultInitializeElements(
          storage_.data + storage_.size, new_size - storage_.size);
    } else if (new_size < storage_.size) {
      bounded_inline_vector_impl::DestroyElements(storage_.data + new_size,
                                                  storage_.size - new_size);
    }
    storage_.size = new_size;
  }

  const T* data() const { return storage_.data; }
  T* data() { return storage_.data; }

  const T& operator[](int index) const {
    RTC_DCHECK_GE(index, 0);
    RTC_DCHECK_LT(index, storage_.size);
    return storage_.data[index];
  }
  T& operator[](int index) {
    RTC_DCHECK_GE(index, 0);
    RTC_DCHECK_LT(index, storage_.size);
    return storage_.data[index];
  }

  T* begin() { return storage_.data; }
  T* end() { return storage_.data + storage_.size; }
  const T* begin() const { return storage_.data; }
  const T* end() const { return storage_.data + storage_.size; }
  const T* cbegin() const { return storage_.data; }
  const T* cend() const { return storage_.data + storage_.size; }

 private:
  bounded_inline_vector_impl::Storage<T, fixed_capacity> storage_;
};

}  // namespace webrtc

#endif  // RTC_BASE_BOUNDED_INLINE_VECTOR_H_
