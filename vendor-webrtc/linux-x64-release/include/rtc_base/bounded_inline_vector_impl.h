/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BOUNDED_INLINE_VECTOR_IMPL_H_
#define RTC_BASE_BOUNDED_INLINE_VECTOR_IMPL_H_

#include <stdint.h>

#include <cstring>
#include <memory>
#include <type_traits>
#include <utility>

namespace webrtc {
namespace bounded_inline_vector_impl {

template <bool...>
struct BoolPack;

// Tests if all its parameters (x0, x1, ..., xn) are true. The implementation
// checks whether (x0, x1, ..., xn, true) == (true, x0, x1, ..., xn), which is
// true iff true == x0 && x0 == x1 && x1 == x2 ... && xn-1 == xn && xn == true.
template <bool... Bs>
using AllTrue = std::is_same<BoolPack<Bs..., true>, BoolPack<true, Bs...>>;

template <typename To, typename... Froms>
using AllConvertible = AllTrue<std::is_convertible<Froms, To>::value...>;

// Initializes part of an uninitialized array. Unlike normal array
// initialization, does not zero the remaining array elements. Caller is
// responsible for ensuring that there is enough space in `data`.
template <typename T>
void InitializeElements(T* data) {}
template <typename T, typename U, typename... Us>
void InitializeElements(T* data, U&& element, Us&&... elements) {
  // Placement new, because we construct a new object in uninitialized memory.
  ::new (data) T(std::forward<U>(element));
  InitializeElements(data + 1, std::forward<Us>(elements)...);
}

// Default initializes uninitialized array elements.
template <typename T>
void DefaultInitializeElements(T* data, int size) {
  std::uninitialized_default_construct_n(data, size);
}

// Copies from source to uninitialized destination. Caller is responsible for
// ensuring that there is enough space in `dst_data`.
template <typename T>
void CopyElements(const T* src_data, int src_size, T* dst_data, int* dst_size) {
  if /*constexpr*/ (std::is_trivially_copy_constructible<T>::value) {
    std::memcpy(dst_data, src_data, src_size * sizeof(T));
  } else {
    std::uninitialized_copy_n(src_data, src_size, dst_data);
  }
  *dst_size = src_size;
}

// Moves from source to uninitialized destination. Caller is responsible for
// ensuring that there is enough space in `dst_data`.
template <typename T>
void MoveElements(T* src_data, int src_size, T* dst_data, int* dst_size) {
  if /*constexpr*/ (std::is_trivially_move_constructible<T>::value) {
    std::memcpy(dst_data, src_data, src_size * sizeof(T));
  } else {
    std::uninitialized_move_n(src_data, src_size, dst_data);
  }
  *dst_size = src_size;
}

// Destroys elements, leaving them uninitialized.
template <typename T>
void DestroyElements(T* data, int size) {
  if /*constexpr*/ (!std::is_trivially_destructible<T>::value) {
    for (int i = 0; i < size; ++i) {
      data[i].~T();
    }
  }
}

// If elements are trivial and the total capacity is at most this many bytes,
// copy everything instead of just the elements that are in use; this is more
// efficient, and makes BoundedInlineVector trivially copyable.
static constexpr int kSmallSize = 64;

// Storage implementations.
//
// There are diferent Storage structs for diferent kinds of element types. The
// common contract is the following:
//
//   * They have public `size` variables and `data` array members.
//
//   * Their owner is responsible for enforcing the invariant that the first
//     `size` elements in `data` are initialized, and the remaining elements are
//     not initialized.
//
//   * They implement default construction, construction with one or more
//     elements, copy/move construction, copy/move assignment, and destruction;
//     the owner must ensure that the invariant holds whenever these operations
//     occur.

// Storage implementation for nontrivial element types.
template <typename T,
          int fixed_capacity,
          bool is_trivial = std::is_trivial<T>::value,
          bool is_small = (sizeof(T) * fixed_capacity <= kSmallSize)>
struct Storage {
  static_assert(!std::is_trivial<T>::value, "");

  template <
      typename... Ts,
      typename std::enable_if_t<AllConvertible<T, Ts...>::value>* = nullptr>
  explicit Storage(Ts&&... elements) : size(sizeof...(Ts)) {
    InitializeElements(data, std::forward<Ts>(elements)...);
  }

  Storage(const Storage& other) {
    CopyElements(other.data, other.size, data, &size);
  }

  Storage(Storage&& other) {
    MoveElements(other.data, other.size, data, &size);
  }

  Storage& operator=(const Storage& other) {
    if (this != &other) {
      DestroyElements(data, size);
      CopyElements(other.data, other.size, data, &size);
    }
    return *this;
  }

  Storage& operator=(Storage&& other) {
    DestroyElements(data, size);
    size = 0;  // Needed in case of self assignment.
    MoveElements(other.data, other.size, data, &size);
    return *this;
  }

  ~Storage() { DestroyElements(data, size); }

  int size;
  union {
    // Since this array is in a union, we get to construct and destroy it
    // manually.
    T data[fixed_capacity];  // NOLINT(runtime/arrays)
  };
};

// Storage implementation for trivial element types when the capacity is small
// enough that we can cheaply copy everything.
template <typename T, int fixed_capacity>
struct Storage<T, fixed_capacity, /*is_trivial=*/true, /*is_small=*/true> {
  static_assert(std::is_trivial<T>::value, "");
  static_assert(sizeof(T) * fixed_capacity <= kSmallSize, "");

  template <
      typename... Ts,
      typename std::enable_if_t<AllConvertible<T, Ts...>::value>* = nullptr>
  explicit Storage(Ts&&... elements) : size(sizeof...(Ts)) {
    InitializeElements(data, std::forward<Ts>(elements)...);
  }

  Storage(const Storage&) = default;
  Storage& operator=(const Storage&) = default;
  ~Storage() = default;

  int size;
  T data[fixed_capacity];  // NOLINT(runtime/arrays)
};

// Storage implementation for trivial element types when the capacity is large
// enough that we want to avoid copying uninitialized elements.
template <typename T, int fixed_capacity>
struct Storage<T, fixed_capacity, /*is_trivial=*/true, /*is_small=*/false> {
  static_assert(std::is_trivial<T>::value, "");
  static_assert(sizeof(T) * fixed_capacity > kSmallSize, "");

  template <
      typename... Ts,
      typename std::enable_if_t<AllConvertible<T, Ts...>::value>* = nullptr>
  explicit Storage(Ts&&... elements) : size(sizeof...(Ts)) {
    InitializeElements(data, std::forward<Ts>(elements)...);
  }

  Storage(const Storage& other) : size(other.size) {
    std::memcpy(data, other.data, other.size * sizeof(T));
  }

  Storage& operator=(const Storage& other) {
    if (this != &other) {
      size = other.size;
      std::memcpy(data, other.data, other.size * sizeof(T));
    }
    return *this;
  }

  ~Storage() = default;

  int size;
  union {
    T data[fixed_capacity];  // NOLINT(runtime/arrays)
  };
};

}  // namespace bounded_inline_vector_impl
}  // namespace webrtc

#endif  // RTC_BASE_BOUNDED_INLINE_VECTOR_IMPL_H_
