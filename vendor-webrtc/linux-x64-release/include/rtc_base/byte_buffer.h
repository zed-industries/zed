/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_BYTE_BUFFER_H_
#define RTC_BASE_BYTE_BUFFER_H_

#include <stddef.h>
#include <stdint.h>

#include <string>

#include "absl/base/attributes.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "rtc_base/buffer.h"
#include "rtc_base/byte_order.h"

// Reads/Writes from/to buffer using network byte order (big endian)
namespace webrtc {

template <class BufferClassT>
class ByteBufferWriterT {
  using value_type = typename BufferClassT::value_type;

 public:
  ByteBufferWriterT() { Construct(nullptr, kDefaultCapacity); }
  ByteBufferWriterT(const value_type* bytes, size_t len) {
    Construct(bytes, len);
  }

  ByteBufferWriterT(const ByteBufferWriterT&) = delete;
  ByteBufferWriterT& operator=(const ByteBufferWriterT&) = delete;

  const value_type* Data() const { return buffer_.data(); }
  size_t Length() const { return buffer_.size(); }
  size_t Capacity() const { return buffer_.capacity(); }
  ArrayView<const value_type> DataView() const {
    return MakeArrayView(Data(), Length());
  }
  // Accessor that returns a string_view, independent of underlying type.
  // Intended to provide access for existing users that expect char*
  // when the underlying type changes to uint8_t.
  // TODO(bugs.webrtc.org/15665): Delete when users are converted.
  absl::string_view DataAsStringView() const {
    return absl::string_view(reinterpret_cast<const char*>(Data()), Length());
  }
  const char* DataAsCharPointer() const {
    return reinterpret_cast<const char*>(Data());
  }

  // Write value to the buffer. Resizes the buffer when it is
  // neccessary.
  void WriteUInt8(uint8_t val) {
    WriteBytesInternal(reinterpret_cast<const value_type*>(&val), 1);
  }
  void WriteUInt16(uint16_t val) {
    uint16_t v = webrtc::HostToNetwork16(val);
    WriteBytesInternal(reinterpret_cast<const value_type*>(&v), 2);
  }
  void WriteUInt24(uint32_t val) {
    uint32_t v = webrtc::HostToNetwork32(val);
    value_type* start = reinterpret_cast<value_type*>(&v);
    ++start;
    WriteBytesInternal(start, 3);
  }
  void WriteUInt32(uint32_t val) {
    uint32_t v = webrtc::HostToNetwork32(val);
    WriteBytesInternal(reinterpret_cast<const value_type*>(&v), 4);
  }
  void WriteUInt64(uint64_t val) {
    uint64_t v = webrtc::HostToNetwork64(val);
    WriteBytesInternal(reinterpret_cast<const value_type*>(&v), 8);
  }
  // Serializes an unsigned varint in the format described by
  // https://developers.google.com/protocol-buffers/docs/encoding#varints
  // with the caveat that integers are 64-bit, not 128-bit.
  void WriteUVarint(uint64_t val) {
    while (val >= 0x80) {
      // Write 7 bits at a time, then set the msb to a continuation byte
      // (msb=1).
      value_type byte = static_cast<value_type>(val) | 0x80;
      WriteBytesInternal(&byte, 1);
      val >>= 7;
    }
    value_type last_byte = static_cast<value_type>(val);
    WriteBytesInternal(&last_byte, 1);
  }
  void WriteString(absl::string_view val) {
    WriteBytesInternal(reinterpret_cast<const value_type*>(val.data()),
                       val.size());
  }
  // Write an array of bytes (uint8_t)
  [[deprecated("issues.webrtc.org/4225170 - use Write(ArrayView)")]]
  void WriteBytes(const uint8_t* val, size_t len) {
    WriteBytesInternal(reinterpret_cast<const value_type*>(val), len);
  }

  void Write(ArrayView<const value_type> data) {
    WriteBytesInternal(data.data(), data.size());
  }

  // Reserves the given number of bytes and returns a value_type* that can be
  // written into. Useful for functions that require a value_type* buffer and
  // not a ByteBufferWriter.
  value_type* ReserveWriteBuffer(size_t len) {
    buffer_.SetSize(buffer_.size() + len);
    return buffer_.data();
  }

  // Resize the buffer to the specified `size`.
  void Resize(size_t size) { buffer_.SetSize(size); }

  // Clears the contents of the buffer. After this, Length() will be 0.
  void Clear() { buffer_.Clear(); }

  BufferClassT Extract() && { return std::move(buffer_); }

 private:
  static constexpr size_t kDefaultCapacity = 4096;

  void Construct(const value_type* bytes, size_t size) {
    if (bytes) {
      buffer_.AppendData(bytes, size);
    } else {
      buffer_.EnsureCapacity(size);
    }
  }

  void WriteBytesInternal(const value_type* val, size_t len) {
    buffer_.AppendData(val, len);
  }

  BufferClassT buffer_;

  // There are sensible ways to define these, but they aren't needed in our code
  // base.
};

class ByteBufferWriter : public ByteBufferWriterT<BufferT<uint8_t>> {
 public:
  ByteBufferWriter();
  ByteBufferWriter(const uint8_t* bytes, size_t len);

  ByteBufferWriter(const ByteBufferWriter&) = delete;
  ByteBufferWriter& operator=(const ByteBufferWriter&) = delete;
};

// The ByteBufferReader references the passed data, i.e. the pointer must be
// valid during the lifetime of the reader.
class ByteBufferReader {
 public:
  explicit ByteBufferReader(
      ArrayView<const uint8_t> bytes ABSL_ATTRIBUTE_LIFETIME_BOUND);

  explicit ByteBufferReader(const ByteBufferWriter& buf);

  ByteBufferReader(const ByteBufferReader&) = delete;
  ByteBufferReader& operator=(const ByteBufferReader&) = delete;

  const uint8_t* Data() const { return bytes_ + start_; }
  // Returns number of unprocessed bytes.
  size_t Length() const { return end_ - start_; }
  // Returns a view of the unprocessed data. Does not move current position.
  ArrayView<const uint8_t> DataView() const {
    return ArrayView<const uint8_t>(bytes_ + start_, end_ - start_);
  }

  // Read a next value from the buffer. Return false if there isn't
  // enough data left for the specified type.
  bool ReadUInt8(uint8_t* val);
  bool ReadUInt16(uint16_t* val);
  bool ReadUInt24(uint32_t* val);
  bool ReadUInt32(uint32_t* val);
  bool ReadUInt64(uint64_t* val);
  bool ReadUVarint(uint64_t* val);
  // Copies the val.size() next bytes into val.data().
  bool ReadBytes(ArrayView<uint8_t> val);
  // Appends next `len` bytes from the buffer to `val`. Returns false
  // if there is less than `len` bytes left.
  bool ReadString(std::string* val, size_t len);
  // Same as `ReadString` except that the returned string_view will point into
  // the internal buffer (no additional buffer allocation).
  bool ReadStringView(absl::string_view* val, size_t len);

  // Moves current position `size` bytes forward. Returns false if
  // there is less than `size` bytes left in the buffer. Consume doesn't
  // permanently remove data, so remembered read positions are still valid
  // after this call.
  bool Consume(size_t size);

 private:
  void Construct(const uint8_t* bytes, size_t size);
  bool ReadBytes(uint8_t* val, size_t len);

  const uint8_t* bytes_;
  size_t size_;
  size_t start_;
  size_t end_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::ByteBufferReader;
using ::webrtc::ByteBufferWriter;
using ::webrtc::ByteBufferWriterT;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_BYTE_BUFFER_H_
