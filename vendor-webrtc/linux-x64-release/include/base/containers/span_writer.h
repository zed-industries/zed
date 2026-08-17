// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_SPAN_WRITER_H_
#define BASE_CONTAINERS_SPAN_WRITER_H_

#include <optional>

#include "base/containers/span.h"
#include "base/memory/raw_span.h"
#include "base/numerics/byte_conversions.h"

namespace base {

// A Writer to write into and consume elements from the front of a span
// dynamically.
//
// SpanWriter is used to split off prefix spans from a larger span, reporting
// errors if there's not enough room left (instead of crashing, as would happen
// with span directly).
template <typename T>
class SpanWriter {
  static_assert(!std::is_const_v<T>,
                "SpanWriter needs mutable access to its buffer");

 public:
  // Construct SpanWriter that writes to `buf`.
  constexpr explicit SpanWriter(span<T> buf)
      : buf_(buf), original_size_(buf_.size()) {}

  // Returns true and writes the span `data` into the front of the inner span,
  // if there is enough room left. Otherwise, it returns false and does
  // nothing.
  constexpr bool Write(span<const T> data) {
    if (data.size() > remaining()) {
      return false;
    }
    auto [lhs, rhs] = buf_.split_at(data.size());
    lhs.copy_from(data);
    buf_ = rhs;
    return true;
  }

  // Returns true and writes `value` into the front of the inner span if there
  // is space remaining. Otherwise, it returns false and does nothing.
  template <typename V>
    requires(std::same_as<T, std::remove_cvref_t<V>>)
  bool Write(V&& value) {
    if (!remaining()) {
      return false;
    }
    buf_[0] = std::forward<V>(value);
    buf_ = buf_.last(remaining() - 1);
    return true;
  }

  // Skips over the next `n` objects, and returns a span that points to the
  // skipped objects, if there are enough objects left. Otherwise, it returns
  // nullopt and does nothing.
  constexpr std::optional<span<T>> Skip(StrictNumeric<size_t> n) {
    if (n > remaining()) {
      return std::nullopt;
    }
    auto [lhs, rhs] = buf_.split_at(n);
    buf_ = rhs;
    return lhs;
  }
  template <size_t N>
  constexpr std::optional<span<T, N>> Skip() {
    if (N > remaining()) {
      return std::nullopt;
    }
    auto [lhs, rhs] = buf_.template split_at<N>();
    buf_ = rhs;
    return lhs;
  }

  // For a SpanWriter over bytes, we can write integer values directly to those
  // bytes as a memcpy. Returns true if there was room remaining and the bytes
  // were written. The macros below implement the following methods:
  //
  // bool WriteU8BigEndian(uint8_t)
  // bool WriteU16BigEndian(uint16_t)
  // bool WriteU32BigEndian(uint32_t)
  // bool WriteU64BigEndian(uint64_t)
  // bool WriteU8LittleEndian(uint8_t)
  // bool WriteU16LittleEndian(uint16_t)
  // bool WriteU32LittleEndian(uint32_t)
  // bool WriteU64LittleEndian(uint64_t)
  // bool WriteU8NativeEndian(uint8_t)
  // bool WriteU16NativeEndian(uint16_t)
  // bool WriteU32NativeEndian(uint32_t)
  // bool WriteU64NativeEndian(uint64_t)
  // bool WriteI8BigEndian(int8_t)
  // bool WriteI16BigEndian(int16_t)
  // bool WriteI32BigEndian(int32_t)
  // bool WriteI64BigEndian(int64_t)
  // bool WriteI8LittleEndian(int8_t)
  // bool WriteI16LittleEndian(int16_t)
  // bool WriteI32LittleEndian(int32_t)
  // bool WriteI64LittleEndian(int64_t)
  // bool WriteI8NativeEndian(int8_t)
  // bool WriteI16NativeEndian(int16_t)
  // bool WriteI32NativeEndian(int32_t)
  // bool WriteI64NativeEndian(int64_t)
  //
  // Note that "native" order is almost never what you want; it only makes sense
  // for byte buffers that stay in memory and are never written to the disk or
  // network.
#define BASE_SPANWRITER_WRITE(signchar, bitsize, endian, typeprefix) \
  constexpr bool Write##signchar##bitsize##endian##Endian(           \
      typeprefix##int##bitsize##_t value)                            \
    requires(std::same_as<T, uint8_t>)                               \
  {                                                                  \
    return Write(signchar##bitsize##To##endian##Endian(value));      \
  }
#define BASE_SPANWRITER_WRITE_BOTH_SIGNS(bitsize, endian) \
  BASE_SPANWRITER_WRITE(U, bitsize, endian, u)            \
  BASE_SPANWRITER_WRITE(I, bitsize, endian, )
#define BASE_SPANWRITER_WRITE_BOTH_SIGNS_ALL_SIZES(endian) \
  BASE_SPANWRITER_WRITE_BOTH_SIGNS(8, endian)              \
  BASE_SPANWRITER_WRITE_BOTH_SIGNS(16, endian)             \
  BASE_SPANWRITER_WRITE_BOTH_SIGNS(32, endian)             \
  BASE_SPANWRITER_WRITE_BOTH_SIGNS(64, endian)

  BASE_SPANWRITER_WRITE_BOTH_SIGNS_ALL_SIZES(Big)
  BASE_SPANWRITER_WRITE_BOTH_SIGNS_ALL_SIZES(Little)
  BASE_SPANWRITER_WRITE_BOTH_SIGNS_ALL_SIZES(Native)

#undef BASE_SPANWRITER_WRITE_BOTH_SIGNS_ALL_SIZES
#undef BASE_SPANWRITER_WRITE_BOTH_SIGNS
#undef BASE_SPANWRITER_WRITE

  // Returns the remaining not-yet-written-to object count.
  constexpr size_t remaining() const { return buf_.size(); }

  // Returns the remaining not-yet-written-to objects.
  constexpr span<T> remaining_span() const { return buf_; }

  // Returns the number of objects already written (or skipped).
  constexpr size_t num_written() const { return original_size_ - buf_.size(); }

 private:
  raw_span<T> buf_;
  size_t original_size_;
};

template <class T, size_t N>
SpanWriter(span<T, N>) -> SpanWriter<T>;

}  // namespace base

#endif  // BASE_CONTAINERS_SPAN_WRITER_H_
