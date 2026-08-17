// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_SPAN_READER_H_
#define BASE_CONTAINERS_SPAN_READER_H_

#include <concepts>
#include <optional>

#include "base/containers/span.h"
#include "base/memory/stack_allocated.h"
#include "base/numerics/byte_conversions.h"
#include "base/numerics/safe_conversions.h"

namespace base {

// A Reader to consume elements from the front of a span dynamically.
//
// SpanReader is used to split off prefix spans from a larger span, reporting
// errors if there's not enough room left (instead of crashing, as would happen
// with span directly).
template <class T>
class SpanReader {
  STACK_ALLOCATED();

 public:
  // Construct SpanReader from a span.
  explicit SpanReader(span<T> buf) : buf_(buf), original_size_(buf_.size()) {}

  // Returns a span over the next `n` objects, if there are enough objects left.
  // Otherwise, it returns nullopt and does nothing.
  std::optional<span<T>> Read(StrictNumeric<size_t> n) {
    if (n > remaining()) {
      return std::nullopt;
    }
    auto [lhs, rhs] = buf_.split_at(n);
    buf_ = rhs;
    return lhs;
  }

  // Returns a fixed-size span over the next `N` objects, if there are enough
  // objects left. Otherwise, it returns nullopt and does nothing.
  template <size_t N>
  std::optional<span<T, N>> Read() {
    if (N > remaining()) {
      return std::nullopt;
    }
    auto [lhs, rhs] = buf_.template split_at<N>();
    buf_ = rhs;
    return lhs;
  }

  // Returns true and writes a span over the next `n` objects into `out`, if
  // there are enough objects left. Otherwise, it returns false and does
  // nothing.
  bool ReadInto(StrictNumeric<size_t> n, span<T>& out) {
    if (n > remaining()) {
      return false;
    }
    auto [lhs, rhs] = buf_.split_at(n);
    out = lhs;
    buf_ = rhs;
    return true;
  }

  // Returns true and copies objects into `out`, if there are enough objects
  // left to fill `out`. Otherwise, it returns false and does nothing.
  bool ReadCopy(span<std::remove_const_t<T>> out) {
    if (out.size() > remaining()) {
      return false;
    }
    auto [lhs, rhs] = buf_.split_at(out.size());
    out.copy_from(lhs);
    buf_ = rhs;
    return true;
  }

  // Returns true and skips over the next `n` objects, if there are enough
  // objects left. Otherwise, it returns false and does nothing.
  std::optional<span<T>> Skip(StrictNumeric<size_t> n) {
    if (n > remaining()) {
      return std::nullopt;
    }
    auto [lhs, rhs] = buf_.split_at(n);
    buf_ = rhs;
    return lhs;
  }

  // For a SpanReader over bytes, we can read integer values directly from those
  // bytes as a memcpy. Returns true if there was room remaining and the bytes
  // were read.
  //
  // These treat the bytes from the buffer as being in big endian order.
  bool ReadU8BigEndian(uint8_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<1>([&](auto buf) { value = U8FromBigEndian(buf); });
  }
  bool ReadU16BigEndian(uint16_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<2>([&](auto buf) { value = U16FromBigEndian(buf); });
  }
  bool ReadU32BigEndian(uint32_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<4>([&](auto buf) { value = U32FromBigEndian(buf); });
  }
  bool ReadU64BigEndian(uint64_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<8>([&](auto buf) { value = U64FromBigEndian(buf); });
  }

  // For a SpanReader over bytes, we can read integer values directly from those
  // bytes as a memcpy. Returns true if there was room remaining and the bytes
  // were read.
  //
  // These treat the bytes from the buffer as being in little endian order.
  bool ReadU8LittleEndian(uint8_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<1>([&](auto buf) { value = U8FromLittleEndian(buf); });
  }
  bool ReadU16LittleEndian(uint16_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<2>([&](auto buf) { value = U16FromLittleEndian(buf); });
  }
  bool ReadU32LittleEndian(uint32_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<4>([&](auto buf) { value = U32FromLittleEndian(buf); });
  }
  bool ReadU64LittleEndian(uint64_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<8>([&](auto buf) { value = U64FromLittleEndian(buf); });
  }

  // For a SpanReader over bytes, we can read integer values directly from those
  // bytes as a memcpy. Returns true if there was room remaining and the bytes
  // were read.
  //
  // These treat the bytes from the buffer as being in native endian order. Note
  // that this is almost never what you want to do. Native ordering only makes
  // sense for byte buffers that are only meant to stay in memory and never be
  // written to the disk or network.
  bool ReadU8NativeEndian(uint8_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<1>([&](auto buf) { value = U8FromNativeEndian(buf); });
  }
  bool ReadU16NativeEndian(uint16_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<2>([&](auto buf) { value = U16FromNativeEndian(buf); });
  }
  bool ReadU32NativeEndian(uint32_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<4>([&](auto buf) { value = U32FromNativeEndian(buf); });
  }
  bool ReadU64NativeEndian(uint64_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<8>([&](auto buf) { value = U64FromNativeEndian(buf); });
  }

  // For a SpanReader over bytes, we can read integer values directly from those
  // bytes as a memcpy. Returns true if there was room remaining and the bytes
  // were read.
  //
  // These treat the bytes from the buffer as being in big endian order.
  bool ReadI8BigEndian(int8_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<1>([&](auto buf) { value = I8FromBigEndian(buf); });
  }
  bool ReadI16BigEndian(int16_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<2>([&](auto buf) { value = I16FromBigEndian(buf); });
  }
  bool ReadI32BigEndian(int32_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<4>([&](auto buf) { value = I32FromBigEndian(buf); });
  }
  bool ReadI64BigEndian(int64_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<8>([&](auto buf) { value = I64FromBigEndian(buf); });
  }

  // For a SpanReader over bytes, we can read integer values directly from those
  // bytes as a memcpy. Returns true if there was room remaining and the bytes
  // were read.
  //
  // These treat the bytes from the buffer as being in little endian order.
  bool ReadI8LittleEndian(int8_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<1>([&](auto buf) { value = I8FromLittleEndian(buf); });
  }
  bool ReadI16LittleEndian(int16_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<2>([&](auto buf) { value = I16FromLittleEndian(buf); });
  }
  bool ReadI32LittleEndian(int32_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<4>([&](auto buf) { value = I32FromLittleEndian(buf); });
  }
  bool ReadI64LittleEndian(int64_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<8>([&](auto buf) { value = I64FromLittleEndian(buf); });
  }

  // For a SpanReader over bytes, we can read integer values directly from those
  // bytes as a memcpy. Returns true if there was room remaining and the bytes
  // were read.
  //
  // These treat the bytes from the buffer as being in native endian order. Note
  // that this is almost never what you want to do. Native ordering only makes
  // sense for byte buffers that are only meant to stay in memory and never be
  // written to the disk or network.
  bool ReadI8NativeEndian(int8_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<1>([&](auto buf) { value = I8FromNativeEndian(buf); });
  }
  bool ReadI16NativeEndian(int16_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<2>([&](auto buf) { value = I16FromNativeEndian(buf); });
  }
  bool ReadI32NativeEndian(int32_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<4>([&](auto buf) { value = I32FromNativeEndian(buf); });
  }
  bool ReadI64NativeEndian(int64_t& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<8>([&](auto buf) { value = I64FromNativeEndian(buf); });
  }

  // For a SpanReader over bytes, reads one byte and returns it as a `char`,
  // which may be signed or unsigned depending on the platform. Returns true if
  // there was room remaining and the byte was read.
  bool ReadChar(char& value)
    requires(std::same_as<std::remove_const_t<T>, uint8_t>)
  {
    return ReadAnd<1>([&](auto buf) { value = static_cast<char>(buf[0u]); });
  }

  // Returns the number of objects remaining to be read from the original span.
  size_t remaining() const { return buf_.size(); }
  // Returns the objects that have not yet been read, as a span.
  span<T> remaining_span() const { return buf_; }

  // Returns the number of objects read (or skipped) in the original span.
  size_t num_read() const { return original_size_ - buf_.size(); }

 private:
  template <size_t N, class F>
    requires(std::invocable<F, span<T, N>>)
  bool ReadAnd(F f) {
    auto buf = Read<N>();
    if (buf.has_value()) {
      f(*buf);
    }
    return buf.has_value();
  }

  span<T> buf_;
  size_t original_size_;
};

template <typename ElementType, size_t Extent, typename InternalPtrType>
SpanReader(span<ElementType, Extent, InternalPtrType>)
    -> SpanReader<ElementType>;

}  // namespace base

#endif  // BASE_CONTAINERS_SPAN_READER_H_
