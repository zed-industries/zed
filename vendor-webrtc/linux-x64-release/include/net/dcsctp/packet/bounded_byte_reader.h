/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef NET_DCSCTP_PACKET_BOUNDED_BYTE_READER_H_
#define NET_DCSCTP_PACKET_BOUNDED_BYTE_READER_H_

#include <cstdint>

#include "api/array_view.h"

namespace dcsctp {

// TODO(boivie): These generic functions - and possibly this entire class -
// could be a candidate to have added to rtc_base/. They should use compiler
// intrinsics as well.
namespace internal {
// Loads a 8-bit unsigned word at `data`.
inline uint8_t LoadBigEndian8(const uint8_t* data) {
  return data[0];
}

// Loads a 16-bit unsigned word at `data`.
inline uint16_t LoadBigEndian16(const uint8_t* data) {
  return (data[0] << 8) | data[1];
}

// Loads a 32-bit unsigned word at `data`.
inline uint32_t LoadBigEndian32(const uint8_t* data) {
  return (data[0] << 24) | (data[1] << 16) | (data[2] << 8) | data[3];
}
}  // namespace internal

// BoundedByteReader wraps an ArrayView and divides it into two parts; A fixed
// size - which is the template parameter - and a variable size, which is what
// remains in `data` after the `FixedSize`.
//
// The BoundedByteReader provides methods to load/read big endian numbers from
// the FixedSize portion of the buffer, and these are read with static bounds
// checking, to avoid out-of-bounds accesses without a run-time penalty.
//
// The variable sized portion can either be used to create sub-readers, which
// themselves would provide compile-time bounds-checking, or the entire variable
// sized portion can be retrieved as an ArrayView.
template <int FixedSize>
class BoundedByteReader {
 public:
  explicit BoundedByteReader(webrtc::ArrayView<const uint8_t> data)
      : data_(data) {
    RTC_CHECK(data.size() >= FixedSize);
  }

  template <size_t offset>
  uint8_t Load8() const {
    static_assert(offset + sizeof(uint8_t) <= FixedSize, "Out-of-bounds");
    return internal::LoadBigEndian8(&data_[offset]);
  }

  template <size_t offset>
  uint16_t Load16() const {
    static_assert(offset + sizeof(uint16_t) <= FixedSize, "Out-of-bounds");
    static_assert((offset % sizeof(uint16_t)) == 0, "Unaligned access");
    return internal::LoadBigEndian16(&data_[offset]);
  }

  template <size_t offset>
  uint32_t Load32() const {
    static_assert(offset + sizeof(uint32_t) <= FixedSize, "Out-of-bounds");
    static_assert((offset % sizeof(uint32_t)) == 0, "Unaligned access");
    return internal::LoadBigEndian32(&data_[offset]);
  }

  template <size_t SubSize>
  BoundedByteReader<SubSize> sub_reader(size_t variable_offset) const {
    RTC_CHECK(FixedSize + variable_offset + SubSize <= data_.size());

    webrtc::ArrayView<const uint8_t> sub_span =
        data_.subview(FixedSize + variable_offset, SubSize);
    return BoundedByteReader<SubSize>(sub_span);
  }

  size_t variable_data_size() const { return data_.size() - FixedSize; }

  webrtc::ArrayView<const uint8_t> variable_data() const {
    return data_.subview(FixedSize, data_.size() - FixedSize);
  }

 private:
  const webrtc::ArrayView<const uint8_t> data_;
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_BOUNDED_BYTE_READER_H_
