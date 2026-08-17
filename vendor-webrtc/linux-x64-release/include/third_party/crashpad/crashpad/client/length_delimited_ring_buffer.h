// Copyright 2023 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_CLIENT_LENGTH_DELIMITED_RING_BUFFER_H_
#define CRASHPAD_CLIENT_LENGTH_DELIMITED_RING_BUFFER_H_

#include <stdint.h>
#include <string.h>

#include <algorithm>
#include <array>
#include <limits>
#include <optional>
#include <type_traits>
#include <vector>

#include "base/numerics/safe_math.h"

namespace crashpad {

//! \brief Capacity of a `RingBufferData`, in bytes.
using RingBufferCapacity = uint32_t;

namespace internal {

//! \brief Default capacity of `RingBufferData`, in bytes.
inline constexpr RingBufferCapacity kDefaultRingBufferDataCapacity = 8192;

//! \brief A tuple holding the current range of bytes which can be read from or
//!     have been written to.
struct Range final {
  //! \brief The offset into a `RingBufferData` at which a `Range` begins.
  using Offset = uint32_t;

  //! \brief The length inside a `RingBufferData` of a `Range` of data.
  using Length = uint32_t;

  Offset offset;
  Length length;
};

// This struct is persisted to disk, so its size must not change.
static_assert(sizeof(Range) == 8,
              "struct Range is not packed on this platform");

//! \brief The number of bits encoded in each byte of a Base 128-encoded varint.
inline constexpr int kBase128ByteValueBits = 7;

//! \!brief Calculates the length in bytes of `value` encoded using
//!    little-endian Base 128 varint encoding.
//! \sa https://developers.google.com/protocol-buffers/docs/encoding#varints
//!
//! `LengthDelimitedRingBufferWriter` uses varint-encoded delimiters to enable
//! zero-copy deserialization of the ringbuffer's contents when storing
//! protobufs inside the ringbuffer, e.g. via
//! `google::protobuf::util::ParseDelimitedFromZeroCopyStream()` or similar.
//!
//! \sa
//! https://github.com/protocolbuffers/protobuf/blob/3202b9da88ceb75b65bbabaf4033c95e872f828d/src/google/protobuf/util/delimited_message_util.h#L85
//! \sa
//! https://github.com/protocolbuffers/protobuf/blob/8bd49dea5e167a389d94b71d24c981d8f9fa0c99/src/google/protobuf/io/zero_copy_stream_impl_lite.h#L68
//! \sa
//! https://github.com/protocolbuffers/protobuf/blob/8bd49dea5e167a389d94b71d24c981d8f9fa0c99/src/google/protobuf/io/coded_stream.h#L171
//!
//! \!param[in] value Value to be encoded in Base 128 varint encoding.
//! \!return The length in bytes of `value` in Base 128 varint encoding.
template <typename IntegerType>
constexpr Range::Length Base128VarintEncodedLength(IntegerType value) {
  static_assert(std::is_unsigned<IntegerType>::value);

  Range::Length size = 1;
  while (value >= 0x80) {
    value >>= kBase128ByteValueBits;
    size++;
  }
  return size;
}

// Note that std::array capacity is a size_t, not a RingBufferCapacity.
template <size_t ArrayCapacity>
using RingBufferArray = std::array<uint8_t, ArrayCapacity>;

//! \return The size of the `RingBufferArray` as a `Range::Length`.
template <size_t ArrayCapacity>
constexpr Range::Length RingBufferArraySize(
    const RingBufferArray<ArrayCapacity>& ring_buffer_data) {
  static_assert(ArrayCapacity <= std::numeric_limits<Range::Length>::max());
  return static_cast<Range::Length>(ring_buffer_data.size());
}

//! \brief Reads data from the ring buffer into a target buffer.
//! \param[in] ring_buffer_data The ring buffer to read.
//! \param[in,out] ring_buffer_read_range The range of the data available
//!     to read. Upon return, set to the remaining range of data available
//!     to read, if any.
//! \param[in] target_buffer Buffer into which data will be written.
//! \param[in] target_buffer_length Number of bytes to write into
//!     `target_buffer`.
//!
//! \return `true` if the read succeeded, `false` otherwise. On success, updates
//!     `ring_buffer_read_range` to reflect the bytes consumed.
//!
//! The bytes can wrap around the end of the ring buffer, in which case the read
//! continues at the beginning of the ring buffer (if the ring buffer is long
//! enough).
template <typename RingBufferArrayType>
bool ReadBytesFromRingBuffer(const RingBufferArrayType& ring_buffer_data,
                             internal::Range& ring_buffer_read_range,
                             uint8_t* target_buffer,
                             Range::Length target_buffer_length) {
  if (target_buffer_length > ring_buffer_read_range.length) {
    return false;
  }
  if (target_buffer_length == 0) {
    return true;
  }
  const Range::Length initial_read_length = std::min(
      target_buffer_length,
      RingBufferArraySize(ring_buffer_data) - ring_buffer_read_range.offset);
  memcpy(target_buffer,
         &ring_buffer_data[ring_buffer_read_range.offset],
         initial_read_length);
  if (initial_read_length < target_buffer_length) {
    const Range::Length remaining_read_length =
        target_buffer_length - initial_read_length;
    memcpy(target_buffer + initial_read_length,
           &ring_buffer_data[0],
           remaining_read_length);
  }
  ring_buffer_read_range.offset =
      (ring_buffer_read_range.offset + target_buffer_length) %
      RingBufferArraySize(ring_buffer_data);
  ring_buffer_read_range.length -= target_buffer_length;
  return true;
}

//! \brief Reads a single little-endian Base 128 varint-encoded integer from
//!     the ring buffer.
//! \param[in] ring_buffer_data The ring buffer to read.
//! \param[in,out] ring_buffer_read_range The range of the data available
//!     to read. Upon return, set to the remaining range of data available
//!     to read, if any.
//! \param[out] result Upon success, set to the decoded value read from the
//!     buffer.
//!
//! \return The length in bytes of the varint if the read succeeded,
//!    `std::nullopt` otherwise. On success, updates `ring_buffer_read_range`
//!    to reflect the bytes available to read.
//!
//! The varint can wrap around the end of the ring buffer, in which case the
//! read continues at the beginning of the ring buffer (if the ring buffer is
//! long enough).
template <typename RingBufferArrayType, typename IntegerType>
std::optional<Range::Length> ReadBase128VarintFromRingBuffer(
    const RingBufferArrayType& ring_buffer_data,
    internal::Range& ring_buffer_read_range,
    IntegerType& result) {
  static_assert(std::is_unsigned<IntegerType>::value);

  result = 0;
  uint8_t cur_varint_byte = 0;
  constexpr uint8_t kValueMask = 0x7f;
  constexpr uint8_t kContinuationMask = 0x80;
  Range::Length length = 0;
  do {
    if (!ReadBytesFromRingBuffer(
            ring_buffer_data, ring_buffer_read_range, &cur_varint_byte, 1)) {
      // No capacity remaining in `ring_buffer_read_range` to read the varint.
      return std::nullopt;
    }
    IntegerType cur_varint_value =
        static_cast<IntegerType>(cur_varint_byte & kValueMask);

    // This is equivalent to:
    //
    // result |= (cur_varint_value << (length * kBase128ByteValueBits));
    //
    // but checks the result at each step for overflow, which handles two types
    // of invalid input:
    //
    // 1) Too many bytes with kContinuationMask set (e.g., trying to encode 6
    //    bytes worth of data in a 32-bit value)
    // 2) Too many bits in the final byte (e.g., the 5th byte for a 32-bit value
    //    has bits 33 and 34 set)
    IntegerType next_result_bits;
    if (!base::CheckLsh(cur_varint_value, length * kBase128ByteValueBits)
             .AssignIfValid(&next_result_bits)) {
      return std::nullopt;
    }
    result |= next_result_bits;
    ++length;
  } while ((cur_varint_byte & kContinuationMask) == kContinuationMask);
  return length;
}

//! \brief Writes data from the source buffer into the ring buffer.
//! \param[in] source_buffer Buffer from which data will be read.
//! \param[in] source_buffer_length The length in bytes of `source_buffer`.
//! \param[in] ring_buffer_data The ring buffer into which data will be read.
//! \param[in,out] ring_buffer_write_range The range of the data available
//!     to write. Upon return, set to the remaining range of data available
//!     to write, if any.
//!
//! \return `true` if write read succeeded, `false` otherwise. On success,
//! updates
//!     `ring_buffer_write_range` to reflect the bytes written.
//!
//! The bytes can wrap around the end of the ring buffer, in which case the
//! write continues at the beginning of the ring buffer (if the ring buffer is
//! long enough).
template <typename RingBufferArrayType>
bool WriteBytesToRingBuffer(const uint8_t* const source_buffer,
                            Range::Length source_buffer_length,
                            RingBufferArrayType& ring_buffer_data,
                            internal::Range& ring_buffer_write_range) {
  const Range::Length ring_buffer_bytes_remaining =
      RingBufferArraySize(ring_buffer_data) - ring_buffer_write_range.length;
  if (source_buffer_length > ring_buffer_bytes_remaining) {
    return false;
  }
  const Range::Length initial_write_length = std::min(
      source_buffer_length,
      RingBufferArraySize(ring_buffer_data) - ring_buffer_write_range.offset);
  memcpy(&ring_buffer_data[ring_buffer_write_range.offset],
         source_buffer,
         initial_write_length);
  if (initial_write_length < source_buffer_length) {
    const Range::Length remaining_write_length =
        source_buffer_length - initial_write_length;
    memcpy(&ring_buffer_data[0],
           source_buffer + initial_write_length,
           remaining_write_length);
  }
  ring_buffer_write_range.offset =
      (ring_buffer_write_range.offset + source_buffer_length) %
      RingBufferArraySize(ring_buffer_data);
  ring_buffer_write_range.length -= source_buffer_length;
  return true;
}

//! \brief Writes a single Base 128 varint-encoded little-endian unsigned
//!     integer into the ring buffer.
//! \param[in] value The value to encode and write into the ring buffer.
//! \param[in] ring_buffer_data The ring buffer into which to write.
//! \param[in,out] ring_buffer_write_range The range of the data available
//!     to write. Upon return, set to the remaining range of data available
//!     to write, if any.
//!
//! \return The length in bytes of the varint if the write succeeded,
//!    `std::nullopt` otherwise. On success, updates `write_buffer_read_range`
//!    to reflect the range available to write, if any.
//!
//! The varint can wrap around the end of the ring buffer, in which case the
//! write continues at the beginning of the ring buffer (if the ring buffer is
//! long enough).
template <typename RingBufferArrayType, typename IntegerType>
std::optional<int> WriteBase128VarintToRingBuffer(
    IntegerType value,
    RingBufferArrayType& ring_buffer_data,
    internal::Range& ring_buffer_write_range) {
  static_assert(std::is_unsigned<IntegerType>::value);

  uint8_t cur_varint_byte;
  constexpr uint8_t kValueMask = 0x7f;
  constexpr uint8_t kContinuationMask = 0x80;

  // Every varint encodes to at least 1 byte of data.
  int length = 1;

  while (value > kValueMask) {
    cur_varint_byte =
        (static_cast<uint8_t>(value) & kValueMask) | kContinuationMask;
    if (!WriteBytesToRingBuffer(
            &cur_varint_byte, 1, ring_buffer_data, ring_buffer_write_range)) {
      return std::nullopt;
    }
    value >>= kBase128ByteValueBits;
    ++length;
  }
  cur_varint_byte = static_cast<uint8_t>(value);
  if (!WriteBytesToRingBuffer(
          &cur_varint_byte, 1, ring_buffer_data, ring_buffer_write_range)) {
    return std::nullopt;
  }
  return length;
}

}  // namespace internal

//! \brief Storage for a ring buffer which can hold up to
//! `RingBufferCapacity`
//!     bytes of Base 128-varint delimited variable-length items.
//!
//! This struct contains a header immediately followed by the ring buffer
//! data. The current read offset and length are stored in `header.data_range`.
//!
//! The structure of this object is:
//!
//! `|magic|version|data_offset|data_length|ring_buffer_data|`
//!
//! To write data to this object, see `LengthDelimitedRingBufferWriter`.
//! To read data from this object, see `LengthDelimitedRingBufferReader`.
//!
//! The bytes of this structure are suitable for direct serialization from
//! memory to disk, e.g. as a crashpad::Annotation.
template <RingBufferCapacity Capacity>
struct RingBufferData final {
  RingBufferData() = default;
  RingBufferData(RingBufferData&) = delete;
  RingBufferData& operator=(RingBufferData&) = delete;

  //! \brief The type of the array holding the data in this object.
  using RingBufferArrayType = internal::RingBufferArray<Capacity>;

  //! \brief The type of the size in bytes of operations on this object.
  using SizeType = internal::Range::Length;

  //! \brief Attempts to overwrite the contents of this object by deserializing
  //!     the buffer into this object.
  //! \param[in] buffer The bytes to deserialize into this object.
  //! \param[in] length The length in bytes of `buffer`.
  //!
  //! \return `true` if the buffer was a valid RingBufferData and this object
  //!     has enough capacity to store its bytes, `false` otherwise.
  bool DeserializeFromBuffer(const void* buffer, SizeType length) {
    if (length < sizeof(header) || length > sizeof(header) + sizeof(data)) {
      return false;
    }
    const Header* other_header = reinterpret_cast<const Header*>(buffer);
    if (other_header->magic != kMagic || other_header->version != kVersion) {
      return false;
    }
    header.data_range = other_header->data_range;
    const uint8_t* other_ring_buffer_bytes =
        reinterpret_cast<const uint8_t*>(buffer) + sizeof(*other_header);
    const SizeType other_ring_buffer_len = length - sizeof(*other_header);
    memcpy(&data[0], other_ring_buffer_bytes, other_ring_buffer_len);
    return true;
  }

  //! \return The current length in bytes of the data written to the ring
  //!     buffer.
  SizeType GetRingBufferLength() const {
    internal::Range data_range = header.data_range;
    return sizeof(header) + std::min(internal::RingBufferArraySize(data),
                                     data_range.offset + data_range.length);
  }

  //! \brief Resets the state of the ring buffer (e.g., for testing).
  void ResetForTesting() { header.data_range = {0, 0}; }

  //! \brief The magic signature of the ring buffer.
  static constexpr uint32_t kMagic = 0xcab00d1e;
  //! \brief The version of the ring buffer.
  static constexpr uint32_t kVersion = 1;

  //! \brief A header containing metadata preceding the ring buffer data.
  struct Header final {
    Header() : magic(kMagic), version(kVersion), data_range({0, 0}) {}

    //! \brief The fixed magic value identifying this as a ring buffer.
    const uint32_t magic;

    //! \brief The version of this ring buffer data.
    const uint32_t version;

    //! \brief The range of readable data in the ring buffer.
    internal::Range data_range;
  };

  //! \brief The header containing ring buffer metadata.
  Header header;

  //! \brief The bytes of the ring buffer data.
  RingBufferArrayType data;

  // This struct is persisted to disk, so its size must not change.
  static_assert(sizeof(Header) == 16);
  static_assert(Capacity <= std::numeric_limits<uint32_t>::max());
};

// Ensure the ring buffer is packed correctly at its default capacity.
static_assert(
    sizeof(RingBufferData<internal::kDefaultRingBufferDataCapacity>) ==
    16 + internal::kDefaultRingBufferDataCapacity);

// Allow just `RingBufferData foo;` to be declared without template arguments
// using C++17 class template argument deduction.
template <
    RingBufferCapacity Capacity = internal::kDefaultRingBufferDataCapacity>
RingBufferData() -> RingBufferData<Capacity>;

//! \brief Reads variable-length data buffers from a `RingBufferData`,
//!     delimited by Base128 varint-encoded length delimiters.
//!
//! Holds a reference to a `RingBufferData` with the capacity to hold
//! `RingBufferDataType::size()` bytes of variable-length buffers each
//! preceded by its length (encoded as a Base128 length varint).
//!
//! Provides reading capabilities via `Pop()`.
template <typename RingBufferDataType>
class LengthDelimitedRingBufferReader final {
 public:
  //! \brief Constructs a reader which holds a reference to `ring_buffer`.
  //! \param[in] ring_buffer The ring buffer from which data will be read.
  //!     This object must outlive the lifetime of `ring_buffer`.
  constexpr explicit LengthDelimitedRingBufferReader(
      RingBufferDataType& ring_buffer)
      : ring_buffer_(ring_buffer),
        data_range_(ring_buffer_.header.data_range) {}

  LengthDelimitedRingBufferReader(const LengthDelimitedRingBufferReader&) =
      delete;
  LengthDelimitedRingBufferReader& operator=(
      const LengthDelimitedRingBufferReader&) = delete;

  //! \brief Pops off the next buffer from the front of the ring buffer.
  //!
  //! \param[in] target_buffer On success, the buffer into which data will
  //!     be read.
  //! \return On success, returns `true` and advances `ring_buffer.data_range`
  //!     past the end of the buffer read. Otherwise, returns `false`.
  bool Pop(std::vector<uint8_t>& target_buffer) {
    return PopWithRange(target_buffer, data_range_);
  }

  //! \brief Resets the state of the reader (e.g., for testing).
  void ResetForTesting() { data_range_ = {0, 0}; }

 private:
  //! \brief Pops off the next buffer from the front of the ring buffer.
  //! \param[in] target_buffer On success, the buffer into which data will
  //!     be read.
  //! \param[in,out] data_range The range of data available to read.
  //!     On success, updated to the remaining range avilable to read.
  //! \return On success, returns `true` and advances `ring_buffer.data_range`
  //!     past the end of the buffer read. Otherwise, returns `false`.
  bool PopWithRange(std::vector<uint8_t>& target_buffer,
                    internal::Range& data_range) {
    internal::Range::Length buffer_length;
    if (!ReadBase128VarintFromRingBuffer(
            ring_buffer_.data, data_range, buffer_length)) {
      return false;
    }
    if (buffer_length == 0) {
      // A zero-length buffer means the buffer was truncated in the middle of a
      // Push().
      return false;
    }
    const auto previous_target_buffer_size = target_buffer.size();
    target_buffer.resize(previous_target_buffer_size + buffer_length);
    if (!ReadBytesFromRingBuffer(ring_buffer_.data,
                                 data_range,
                                 &target_buffer[previous_target_buffer_size],
                                 buffer_length)) {
      return false;
    }
    return true;
  }

  //! \brief Reference to the ring buffer from which data is read.
  const RingBufferDataType& ring_buffer_;
  //! \brief Range of data currently available to read.
  internal::Range data_range_;
};

// Allow just `LengthDelimitedRingBufferReader reader(foo);` to be declared
// without template arguments using C++17 class template argument deduction.
template <typename RingBufferDataType>
LengthDelimitedRingBufferReader(RingBufferDataType&)
    -> LengthDelimitedRingBufferReader<RingBufferDataType>;

//! \brief Writes variable-length data buffers to a `RingBufferData`,
//!     delimited by Base128 varint-encoded length delimiters.
//!
//! Holds a reference to a `RingBufferData` with the capacity to hold
//! `RingBufferDataType::size()` bytes of variable-length buffers each
//! preceded by its length (encoded as a Base128 length varint).
//!
//! Provides writing capabilities via `Push()`.
template <typename RingBufferDataType>
class LengthDelimitedRingBufferWriter final {
 public:
  //! \brief Constructs a writer which holds a reference to `ring_buffer`.
  //! \param[in] ring_buffer The ring buffer into which data will be written.
  //!     This object must outlive the lifetime of `ring_buffer`.
  constexpr explicit LengthDelimitedRingBufferWriter(
      RingBufferDataType& ring_buffer)
      : ring_buffer_(ring_buffer), ring_buffer_write_offset_(0) {}

  LengthDelimitedRingBufferWriter(const LengthDelimitedRingBufferWriter&) =
      delete;
  LengthDelimitedRingBufferWriter& operator=(
      const LengthDelimitedRingBufferWriter&) = delete;

  //! \brief Writes data to the ring buffer.
  //!
  //! If there is not enough room remaining in the ring buffer to store the new
  //! data, old data will be removed from the ring buffer in FIFO order until
  //! there is room for the new data.
  //!
  //! \param[in] buffer The data to be written.
  //! \param[in] buffer_length The lengh of `buffer`, in bytes.
  //! \return On success, returns `true`, updates `ring_buffer.data_range`
  //!     to reflect the remaining data available to read, and updates
  //!     `ring_buffer_write_offset_` to reflec the current write positionl.
  //!     Otherwise, returns `false`.
  bool Push(const void* const buffer,
            typename RingBufferDataType::SizeType buffer_length) {
    if (buffer_length == 0) {
      // Pushing a zero-length buffer is not allowed
      // (`LengthDelimitedRingBufferWriter` reserves that to represent a
      // temporarily truncated item below).
      return false;
    }
    const internal::Range::Length buffer_varint_encoded_length =
        internal::Base128VarintEncodedLength(buffer_length);
    const internal::Range::Length bytes_needed =
        buffer_varint_encoded_length + buffer_length;
    if (bytes_needed > ring_buffer_.data.size()) {
      return false;
    }
    // If needed, move the readable region forward one buffer at a time to make
    // room for `buffer_length` bytes of new data.
    auto readable_data_range = ring_buffer_.header.data_range;
    internal::Range::Length bytes_available =
        internal::RingBufferArraySize(ring_buffer_.data) -
        readable_data_range.length;
    while (bytes_available < bytes_needed) {
      internal::Range::Length bytes_to_skip;
      auto varint_length = ReadBase128VarintFromRingBuffer(
          ring_buffer_.data, readable_data_range, bytes_to_skip);
      if (!varint_length.has_value()) {
        return false;
      }
      // Skip past the next entry including its prepended varint length.
      readable_data_range.offset =
          (readable_data_range.offset + bytes_to_skip) %
          internal::RingBufferArraySize(ring_buffer_.data);
      readable_data_range.length -= bytes_to_skip;
      bytes_available += varint_length.value() + bytes_to_skip;
    }
    // Write the varint containing `buffer_length` to the current write
    // position.
    internal::Range write_range = {
        ring_buffer_write_offset_,
        bytes_needed,
    };

    internal::WriteBase128VarintToRingBuffer(
        buffer_length, ring_buffer_.data, write_range);
    // Next, write the bytes from `buffer`.
    internal::WriteBytesToRingBuffer(
        reinterpret_cast<const uint8_t* const>(buffer),
        buffer_length,
        ring_buffer_.data,
        write_range);
    // Finally, update the write position and read data range taking into
    // account any items skipped to make room plus the new buffer's varint
    // length and the new buffer's length.
    ring_buffer_write_offset_ = write_range.offset;
    const internal::Range final_data_range = {
        readable_data_range.offset,
        readable_data_range.length + bytes_needed,
    };
    ring_buffer_.header.data_range = final_data_range;
    return true;
  }

  //! \brief Resets the state of the ring buffer and writer (e.g., for testing).
  void ResetForTesting() {
    ring_buffer_.ResetForTesting();
    ring_buffer_write_offset_ = 0;
  }

 private:
  //! \brief Reference to the ring buffer from which data is written.
  RingBufferDataType& ring_buffer_;

  // \brief Current write position next time `Push()` is invoked.
  internal::Range::Offset ring_buffer_write_offset_;
};

// Allow just `LengthDelimitedRingBufferWriter writer(foo);` to be declared
// without template arguments using C++17 class template argument deduction.
template <typename RingBufferDataType>
LengthDelimitedRingBufferWriter(RingBufferDataType&)
    -> LengthDelimitedRingBufferWriter<RingBufferDataType>;

}  // namespace crashpad

#endif  // CRASHPAD_CLIENT_LENGTH_DELIMITED_RING_BUFFER_H_
