// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_BUFFER_ITERATOR_H_
#define BASE_CONTAINERS_BUFFER_ITERATOR_H_

#include <string.h>

#include <concepts>
#include <optional>

#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/memory/raw_span.h"
#include "base/numerics/checked_math.h"

namespace base {

// BufferIterator is a bounds-checked container utility to access variable-
// length, heterogeneous structures contained within a buffer. If the data are
// homogeneous, use base::span<> instead.
//
// After being created with a weakly-owned buffer, BufferIterator returns
// pointers to structured data within the buffer. After each method call that
// returns data in the buffer, the iterator position is advanced by the byte
// size of the object (or span of objects) returned. If there are not enough
// bytes remaining in the buffer to return the requested object(s), a nullptr
// or empty span is returned.
//
// This class is similar to base::Pickle, which should be preferred for
// serializing to disk. Pickle versions its header and does not support writing
// structures, which are problematic for serialization due to struct padding and
// version shear concerns.
//
// Example usage:
//
//    std::vector<uint8_t> buffer(4096);
//    if (!ReadSomeData(&buffer, buffer.size())) {
//      LOG(ERROR) << "Failed to read data.";
//      return false;
//    }
//
//    BufferIterator<uint8_t> iterator(buffer);
//    uint32_t* num_items = iterator.Object<uint32_t>();
//    if (!num_items) {
//      LOG(ERROR) << "No num_items field."
//      return false;
//    }
//
//    base::span<const item_struct> items =
//        iterator.Span<item_struct>(*num_items);
//    if (items.size() != *num_items) {
//      LOG(ERROR) << "Not enough items.";
//      return false;
//    }
//
//    // ... validate the objects in |items|.
template <typename B>
class BufferIterator {
 public:
  static_assert(std::same_as<std::remove_const_t<B>, char> ||
                    std::same_as<std::remove_const_t<B>, unsigned char>,
                "Underlying buffer type must be char-type.");
  // Constructs an empty BufferIterator that will always return null pointers.
  BufferIterator() = default;

  // Constructs a BufferIterator over the `buffer` span, that will return
  // pointers into the span.
  explicit BufferIterator(span<B> buffer)
      : buffer_(buffer), remaining_(buffer) {}

  // TODO(crbug.com/40284755): Move all callers to use spans and remove this.
  UNSAFE_BUFFER_USAGE BufferIterator(B* data, size_t size)
      : BufferIterator(
            // TODO(crbug.com/40284755): Remove this constructor entirely,
            // callers should provide a span. There's no way to know that the
            // size is correct here.
            UNSAFE_BUFFERS(span(data, size))) {}

  // Copies out an object. As compared to using `Object`, this avoids potential
  // unaligned access which may be undefined behavior.
  template <typename T>
    requires(std::is_trivially_copyable_v<T>)
  std::optional<T> CopyObject() {
    std::optional<T> t;
    if (remaining_.size() >= sizeof(T)) {
      auto [source, remain] = remaining_.template split_at<sizeof(T)>();
      byte_span_from_ref(t.emplace()).copy_from(as_bytes(source));
      remaining_ = remain;
    }
    return t;
  }

  // Returns a const pointer to an object of type T in the buffer at the current
  // position.
  //
  // # Safety
  // Note that the buffer's current position must be aligned for the type T
  // or using the pointer will cause Undefined Behaviour. Generally prefer
  // `CopyObject` as it avoids this problem entirely.
  // TODO(danakj): We should probably CHECK this instead of allowing UB into
  // production.
  template <typename T>
    requires(std::is_trivially_copyable_v<T>)
  const T* Object() {
    return MutableObject<const T>();
  }

  // Returns a pointer to a mutable structure T in the buffer at the current
  // position. On success, the iterator position is advanced by sizeof(T). If
  // there are not sizeof(T) bytes remaining in the buffer, returns nullptr.
  //
  // # Safety
  // Note that the buffer's current position must be aligned for the type T or
  // using the pointer will cause Undefined Behaviour. Generally prefer
  // `CopyObject` as it avoids this problem entirely.
  // TODO(danakj): We should probably CHECK this instead of allowing UB into
  // production.
  template <typename T>
    requires(std::is_trivially_copyable_v<T>)
  T* MutableObject() {
    T* t = nullptr;
    if (remaining_.size() >= sizeof(T)) {
      auto [source, remain] = remaining_.template split_at<sizeof(T)>();
      // TODO(danakj): This is UB without creating a lifetime for the object in
      // the compiler, which we can not do before C++23:
      // https://en.cppreference.com/w/cpp/memory/start_lifetime_as
      t = reinterpret_cast<T*>(source.data());
      remaining_ = remain;
    }
    return t;
  }

  // Returns a span of |count| T objects in the buffer at the current position.
  // On success, the iterator position is advanced by |sizeof(T) * count|. If
  // there are not enough bytes remaining in the buffer to fulfill the request,
  // returns an empty span.
  //
  // # Safety
  // Note that the buffer's current position must be aligned for the type T or
  // using the span will cause Undefined Behaviour.
  // TODO(danakj): We should probably CHECK this instead of allowing UB into
  // production.
  template <typename T>
    requires(std::is_trivially_copyable_v<T>)
  span<T> MutableSpan(size_t count) {
    size_t byte_size;
    if (!CheckMul(sizeof(T), count).AssignIfValid(&byte_size)) {
      return span<T>();
    }
    if (byte_size > remaining_.size()) {
      return span<T>();
    }
    auto [lhs, rhs] = remaining_.split_at(byte_size);
    remaining_ = rhs;
    // SAFETY: The byte size of `span<T>` with size `count` is `count *
    // sizeof(T)` which is exactly `byte_size`, the byte size of `lhs`.
    //
    // TODO(danakj): This is UB without creating a lifetime for the object in
    // the compiler, which we can not do before C++23:
    // https://en.cppreference.com/w/cpp/memory/start_lifetime_as
    return UNSAFE_BUFFERS(span<T>(reinterpret_cast<T*>(lhs.data()), count));
  }

  // An overload for when the size is known at compile time. The result will be
  // a fixed-size span.
  template <typename T, size_t N>
    requires(N <= std::numeric_limits<size_t>::max() / sizeof(T) &&
             std::is_trivially_copyable_v<T>)
  std::optional<span<T, N>> MutableSpan() {
    constexpr size_t byte_size =
        N * sizeof(T);  // Overflow is checked by `requires`.
    if (byte_size > remaining_.size()) {
      return std::nullopt;
    }
    auto [lhs, rhs] = remaining_.split_at(byte_size);
    remaining_ = rhs;
    // SAFETY: The byte size of `span<T>` with size `count` is `count *
    // sizeof(T)` which is exactly `byte_size`, the byte size of `lhs`.
    //
    // TODO(danakj): This is UB without creating a lifetime for the object in
    // the compiler, which we can not do before C++23:
    // https://en.cppreference.com/w/cpp/memory/start_lifetime_as
    return UNSAFE_BUFFERS(span<T, N>(reinterpret_cast<T*>(lhs.data()), N));
  }

  // Returns a span to |count| const objects of type T in the buffer at the
  // current position.
  //
  // # Safety
  // Note that the buffer's current position must be aligned for the type T or
  // using the span will cause Undefined Behaviour.
  // TODO(danakj): We should probably CHECK this instead of allowing UB into
  // production.
  template <typename T>
    requires(std::is_trivially_copyable_v<T>)
  span<const T> Span(size_t count) {
    return MutableSpan<const T>(count);
  }

  // An overload for when the size is known at compile time. The result will be
  // a fixed-size span.
  template <typename T, size_t N>
    requires(N <= std::numeric_limits<size_t>::max() / sizeof(T) &&
             std::is_trivially_copyable_v<T>)
  std::optional<span<const T, N>> Span() {
    return MutableSpan<const T, N>();
  }

  // Resets the iterator position to the absolute offset |to|.
  void Seek(size_t to) { remaining_ = buffer_.subspan(to); }

  // Limits the remaining data to the specified size.
  // Seeking to an absolute offset reverses this.
  void TruncateTo(size_t size) { remaining_ = remaining_.first(size); }

  // Returns the total size of the underlying buffer.
  size_t total_size() const { return buffer_.size(); }

  // Returns the current position in the buffer.
  size_t position() const {
    // SAFETY: `remaining_` is a subspan always constructed from `buffer_` (or
    // from itself) so its `data()` pointer is always inside `buffer_`. This
    // means the subtraction is well-defined and the result is always
    // non-negative.
    return static_cast<size_t>(
        UNSAFE_BUFFERS(remaining_.data() - buffer_.data()));
  }

 private:
  // The original buffer that the iterator was constructed with.
  const raw_span<B> buffer_;
  // A subspan of `buffer_` containing the remaining bytes to iterate over.
  raw_span<B> remaining_;
  // Copy and assign allowed.
};

}  // namespace base

#endif  // BASE_CONTAINERS_BUFFER_ITERATOR_H_
