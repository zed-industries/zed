/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_TLV_TRAIT_H_
#define NET_DCSCTP_PACKET_TLV_TRAIT_H_

#include <stdint.h>
#include <string.h>

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "api/array_view.h"
#include "net/dcsctp/packet/bounded_byte_reader.h"
#include "net/dcsctp/packet/bounded_byte_writer.h"

namespace dcsctp {
namespace tlv_trait_impl {
// Logging functions, only to be used by TLVTrait, which is a templated class.
void ReportInvalidSize(size_t actual_size, size_t expected_size);
void ReportInvalidType(int actual_type, int expected_type);
void ReportInvalidFixedLengthField(size_t value, size_t expected);
void ReportInvalidVariableLengthField(size_t value, size_t available);
void ReportInvalidPadding(size_t padding_bytes);
void ReportInvalidLengthMultiple(size_t length, size_t alignment);
}  // namespace tlv_trait_impl

// Various entities in SCTP are padded data blocks, with a type and length
// field at fixed offsets, all stored in a 4-byte header.
//
// See e.g. https://tools.ietf.org/html/rfc4960#section-3.2 and
// https://tools.ietf.org/html/rfc4960#section-3.2.1
//
// These are helper classes for writing and parsing that data, which in SCTP is
// called Type-Length-Value, or TLV.
//
// This templated class is configurable - a struct passed in as template
// parameter with the following expected members:
//   * kType                    - The type field's value
//   * kTypeSizeInBytes         - The type field's width in bytes.
//                                Either 1 or 2.
//   * kHeaderSize              - The fixed size header
//   * kVariableLengthAlignment - The size alignment on the variable data. Set
//                                to zero (0) if no variable data is used.
//
// This class is to be used as a trait
// (https://en.wikipedia.org/wiki/Trait_(computer_programming)) that adds a few
// public and protected members and which a class inherits from when it
// represents a type-length-value object.
template <typename Config>
class TLVTrait {
 private:
  static constexpr size_t kTlvHeaderSize = 4;

 protected:
  static constexpr size_t kHeaderSize = Config::kHeaderSize;

  static_assert(Config::kTypeSizeInBytes == 1 || Config::kTypeSizeInBytes == 2,
                "kTypeSizeInBytes must be 1 or 2");
  static_assert(Config::kHeaderSize >= kTlvHeaderSize,
                "HeaderSize must be >= 4 bytes");
  static_assert((Config::kHeaderSize % 4 == 0),
                "kHeaderSize must be an even multiple of 4 bytes");
  static_assert((Config::kVariableLengthAlignment == 0 ||
                 Config::kVariableLengthAlignment == 1 ||
                 Config::kVariableLengthAlignment == 2 ||
                 Config::kVariableLengthAlignment == 4 ||
                 Config::kVariableLengthAlignment == 8),
                "kVariableLengthAlignment must be an allowed value");

  // Validates the data with regards to size, alignment and type.
  // If valid, returns a bounded buffer.
  static std::optional<BoundedByteReader<Config::kHeaderSize>> ParseTLV(
      webrtc::ArrayView<const uint8_t> data) {
    if (data.size() < Config::kHeaderSize) {
      tlv_trait_impl::ReportInvalidSize(data.size(), Config::kHeaderSize);
      return std::nullopt;
    }
    BoundedByteReader<kTlvHeaderSize> tlv_header(data);

    const int type = (Config::kTypeSizeInBytes == 1)
                         ? tlv_header.template Load8<0>()
                         : tlv_header.template Load16<0>();

    if (type != Config::kType) {
      tlv_trait_impl::ReportInvalidType(type, Config::kType);
      return std::nullopt;
    }
    const uint16_t length = tlv_header.template Load16<2>();
    if (Config::kVariableLengthAlignment == 0) {
      // Don't expect any variable length data at all.
      if (length != Config::kHeaderSize || data.size() != Config::kHeaderSize) {
        tlv_trait_impl::ReportInvalidFixedLengthField(length,
                                                      Config::kHeaderSize);
        return std::nullopt;
      }
    } else {
      // Expect variable length data - verify its size alignment.
      if (length > data.size() || length < Config::kHeaderSize) {
        tlv_trait_impl::ReportInvalidVariableLengthField(length, data.size());
        return std::nullopt;
      }
      const size_t padding = data.size() - length;
      if (padding > 3) {
        // https://tools.ietf.org/html/rfc4960#section-3.2
        // "This padding MUST NOT be more than 3 bytes in total"
        tlv_trait_impl::ReportInvalidPadding(padding);
        return std::nullopt;
      }
      if (!ValidateLengthAlignment(length, Config::kVariableLengthAlignment)) {
        tlv_trait_impl::ReportInvalidLengthMultiple(
            length, Config::kVariableLengthAlignment);
        return std::nullopt;
      }
    }
    return BoundedByteReader<Config::kHeaderSize>(data.subview(0, length));
  }

  // Allocates space for data with a static header size, as defined by
  // `Config::kHeaderSize` and a variable footer, as defined by `variable_size`
  // (which may be 0) and writes the type and length in the header.
  static BoundedByteWriter<Config::kHeaderSize> AllocateTLV(
      std::vector<uint8_t>& out,
      size_t variable_size = 0) {
    const size_t offset = out.size();
    const size_t size = Config::kHeaderSize + variable_size;
    out.resize(offset + size);

    BoundedByteWriter<kTlvHeaderSize> tlv_header(
        webrtc::ArrayView<uint8_t>(out.data() + offset, kTlvHeaderSize));
    if (Config::kTypeSizeInBytes == 1) {
      tlv_header.template Store8<0>(static_cast<uint8_t>(Config::kType));
    } else {
      tlv_header.template Store16<0>(Config::kType);
    }
    tlv_header.template Store16<2>(size);

    return BoundedByteWriter<Config::kHeaderSize>(
        webrtc::ArrayView<uint8_t>(out.data() + offset, size));
  }

 private:
  static bool ValidateLengthAlignment(uint16_t length, size_t alignment) {
    // This is to avoid MSVC believing there could be a "mod by zero", when it
    // certainly can't.
    if (alignment == 0) {
      return true;
    }
    return (length % alignment) == 0;
  }
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_TLV_TRAIT_H_
