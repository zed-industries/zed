// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_PUFF_DATA_H_
#define SRC_PUFF_DATA_H_

#include <cstddef>
#include <cstdint>
#include <functional>

namespace puffin {

// Data structure that is exchanged between the |PuffWriterInterface|,
// |PuffReaderInterface|, |Puffer|, and |Huffer|.
struct PuffData {
  enum class Type {
    // Used for reading/writing only one literal.
    kLiteral,

    // Used for reading/writing literals.
    kLiterals,

    // Used for reading/writing length/distance pairs.
    kLenDist,

    // Used for reading/writing a buffer as the Huffman table. The
    // implementations should copy the data (located in |metadata|) into puff
    // buffer.
    kBlockMetadata,

    // Used for reading/writing an end of block symbol. End of block can
    // contain the unused bits of data at the end of a deflate stream.
    kEndOfBlock,
  } type;

  // A function that once set, can read raw bytes from whatever its parameters
  // are set. This function reads |count| bytes from |buffer| and advances its
  // read offset forward. The next call to this function will start reading
  // after the last read byte. It returns false if it cannot read or the |count|
  // is larger than what is availabe in the buffer.
  // Used by:
  // PuffData::Type::kLiterals
  std::function<bool(uint8_t* buffer, size_t count)> read_fn;

  // Used by:
  // PuffData::Type::kBlockMetadata
  // PuffData::Type::kEndOfBlock
  // PuffData::Type::kLiterals
  // PuffData::Type::kLenDist
  size_t length;

  // Used by:
  // PuffData::Type::kEndOfBlock
  // PuffData::Type::kLenDist
  size_t distance;

  // Used by:
  // PuffData::Type::kEndOfBlock
  // PuffData::Type::kLiteral
  uint8_t byte;

  // 1: Header size.
  // 3: Lengths of next three arrays.
  // 286: Maximum number of literals/lengths codes lengths.
  // 30: Maximum number of distance codes lengths.
  // 19: Maximum number of header code lengths.
  // Used by:
  // PuffData::Type::kBlockMetadata
  uint8_t block_metadata[1 + 3 + 286 + 30 + 19];
};

// The headers for differentiating literals from length/distance pairs.
constexpr uint8_t kLiteralsHeader = 0x00;
constexpr uint8_t kLenDistHeader = 0x80;

}  // namespace puffin

#endif  // SRC_PUFF_DATA_H_
