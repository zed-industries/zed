/* Copyright 2014 Google Inc. All Rights Reserved.

   Distributed under MIT license.
   See file LICENSE for detail or copy at https://opensource.org/licenses/MIT
*/

/* Common definition for WOFF2 encoding/decoding */

#ifndef WOFF2_WOFF2_COMMON_H_
#define WOFF2_WOFF2_COMMON_H_

#include <stddef.h>
#include <inttypes.h>

#include <string>

namespace woff2 {

static const uint32_t kWoff2Signature = 0x774f4632;  // "wOF2"

// Leave the first byte open to store flag_byte
const unsigned int kWoff2FlagsTransform = 1 << 8;

// TrueType Collection ID string: 'ttcf'
static const uint32_t kTtcFontFlavor = 0x74746366;

static const size_t kSfntHeaderSize = 12;
static const size_t kSfntEntrySize = 16;

struct Point {
  int x;
  int y;
  bool on_curve;
};

struct Table {
  uint32_t tag;
  uint32_t flags;
  uint32_t src_offset;
  uint32_t src_length;

  uint32_t transform_length;

  uint32_t dst_offset;
  uint32_t dst_length;
  const uint8_t* dst_data;

  bool operator<(const Table& other) const {
    return tag < other.tag;
  }
};


// Size of the collection header. 0 if version indicates this isn't a
// collection. Ref http://www.microsoft.com/typography/otspec/otff.htm,
// True Type Collections
size_t CollectionHeaderSize(uint32_t header_version, uint32_t num_fonts);

// Compute checksum over size bytes of buf
uint32_t ComputeULongSum(const uint8_t* buf, size_t size);

} // namespace woff2

#endif  // WOFF2_WOFF2_COMMON_H_
