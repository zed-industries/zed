/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_H265_H265_COMMON_H_
#define COMMON_VIDEO_H265_H265_COMMON_H_

#include <memory>
#include <vector>

#include "common_video/h265/h265_inline.h"
#include "rtc_base/buffer.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

namespace H265 {
// The size of a full NALU start sequence {0 0 0 1}, used for the first NALU
// of an access unit, and for SPS and PPS blocks.
constexpr size_t kNaluLongStartSequenceSize = 4;

// The size of a shortened NALU start sequence {0 0 1}, that may be used if
// not the first NALU of an access unit or an SPS or PPS block.
constexpr size_t kNaluShortStartSequenceSize = 3;

// The size of the NALU header byte (2).
constexpr size_t kNaluHeaderSize = 2;

// Type description of 0-40 is defined in Table7-1 of the H.265 spec
// Type desciption of 48-49 is defined in section 4.4.2 and 4.4.3 of RFC7798
enum NaluType : uint8_t {
  kTrailN = 0,
  kTrailR = 1,
  kTsaN = 2,
  kTsaR = 3,
  kStsaN = 4,
  kStsaR = 5,
  kRadlN = 6,
  kRadlR = 7,
  kRaslN = 8,
  kRaslR = 9,
  kBlaWLp = 16,
  kBlaWRadl = 17,
  kBlaNLp = 18,
  kIdrWRadl = 19,
  kIdrNLp = 20,
  kCra = 21,
  kRsvIrapVcl23 = 23,
  kRsvVcl31 = 31,
  kVps = 32,
  kSps = 33,
  kPps = 34,
  kAud = 35,
  kPrefixSei = 39,
  kSuffixSei = 40,
  // Aggregation packets, refer to section 4.4.2 in RFC 7798.
  kAp = 48,
  // Fragmentation units, refer to section 4.4.3 in RFC 7798.
  kFu = 49,
  // PACI packets, refer to section 4.4.4 in RFC 7798.
  kPaci = 50
};

// Slice type definition. See table 7-7 of the H.265 spec
enum SliceType : uint8_t { kB = 0, kP = 1, kI = 2 };

struct NaluIndex {
  // Start index of NALU, including start sequence.
  size_t start_offset = 0;
  // Start index of NALU payload, typically type header.
  size_t payload_start_offset = 0;
  // Length of NALU payload, in bytes, counting from payload_start_offset.
  size_t payload_size = 0;
};

// Returns a vector of the NALU indices in the given buffer.
RTC_EXPORT std::vector<NaluIndex> FindNaluIndices(
    ArrayView<const uint8_t> buffer);

// TODO: bugs.webrtc.org/42225170 - Deprecate.
inline std::vector<NaluIndex> FindNaluIndices(const uint8_t* buffer,
                                              size_t buffer_size) {
  return FindNaluIndices(MakeArrayView(buffer, buffer_size));
}

// Get the NAL type from the header byte immediately following start sequence.
RTC_EXPORT NaluType ParseNaluType(uint8_t data);

// Methods for parsing and writing RBSP. See section 7.4.2 of the H.265 spec.
//
// The following sequences are illegal, and need to be escaped when encoding:
// 00 00 00 -> 00 00 03 00
// 00 00 01 -> 00 00 03 01
// 00 00 02 -> 00 00 03 02
// And things in the source that look like the emulation byte pattern (00 00 03)
// need to have an extra emulation byte added, so it's removed when decoding:
// 00 00 03 -> 00 00 03 03
//
// Decoding is simply a matter of finding any 00 00 03 sequence and removing
// the 03 emulation byte.

// Parse the given data and remove any emulation byte escaping.
std::vector<uint8_t> ParseRbsp(ArrayView<const uint8_t> data);

// TODO: bugs.webrtc.org/42225170 - Deprecate.
inline std::vector<uint8_t> ParseRbsp(const uint8_t* data, size_t length) {
  return ParseRbsp(MakeArrayView(data, length));
}

// Write the given data to the destination buffer, inserting and emulation
// bytes in order to escape any data the could be interpreted as a start
// sequence.
void WriteRbsp(ArrayView<const uint8_t> bytes, Buffer* destination);

// TODO: bugs.webrtc.org/42225170 -  Deprecate.
inline void WriteRbsp(const uint8_t* bytes,
                      size_t length,
                      Buffer* destination) {
  WriteRbsp(MakeArrayView(bytes, length), destination);
}

uint32_t Log2Ceiling(uint32_t value);

}  // namespace H265
}  // namespace webrtc

#endif  // COMMON_VIDEO_H265_H265_COMMON_H_
