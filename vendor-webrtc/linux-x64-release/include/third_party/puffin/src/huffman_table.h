// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_HUFFMAN_TABLE_H_
#define SRC_HUFFMAN_TABLE_H_

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include "puffin/src/bit_reader.h"
#include "puffin/src/bit_writer.h"
#include "puffin/src/include/puffin/common.h"
#include "puffin/src/logging.h"

namespace puffin {

// Maximum Huffman code length based on RFC1951.
constexpr size_t kMaxHuffmanBits = 15;

// Permutations of input Huffman code lengths (used only to read
// |dynamic_code_lens_|).
extern const uint8_t kPermutations[];

// The bases of each alphabet which is added to the integer value of extra
// bits that comes after the Huffman code in the input to create the given
// length value. The last element is a guard.
extern const uint16_t kLengthBases[];

// Number of extra bits that comes after the associating Huffman code.
extern const uint8_t kLengthExtraBits[];

// same as |kLengthBases| except for the the distances instead of lengths.
// The last element is a guard.
extern const uint16_t kDistanceBases[];

// Same as |kLengthExtraBits| except for distances instead of lengths.
extern const uint8_t kDistanceExtraBits[];

class HuffmanTable {
 public:
  HuffmanTable();
  virtual ~HuffmanTable() = default;

  // Checks the lengths of Huffman length arrays for correctness
  //
  // |num_lit_len|  IN  The number of literal/lengths code lengths
  // |num_distance| IN  The number of distance code lengths
  // |num_codes|    IN  The number of code lengths for reading Huffman table.
  inline bool CheckHuffmanArrayLengths(size_t num_lit_len,
                                       size_t num_distance,
                                       size_t num_codes) {
    if (num_lit_len > 286 || num_distance > 30 || num_codes > 19) {
      return false;
    }
    return true;
  }

  // Returns the maximum number of bits used in the current literal/length
  // Huffman codes.
  inline size_t LitLenMaxBits() { return lit_len_max_bits_; }

  // Returns the maximum number of bits used in the current distance Huffman
  // codes.
  inline size_t DistanceMaxBits() { return distance_max_bits_; }

  // Returns the alphabet associated with the set of input bits for the code
  // length array.
  //
  // |bits|     IN   The input Huffman bits read from the deflate stream.
  // |alphabet| OUT  The alphabet associated with the given |bits|.
  // |nbits|    OUT  The number of bits in the Huffman code of alphabet.
  // Returns true if there is an alphabet associated with |bits|.
  inline bool CodeAlphabet(uint32_t bits, uint16_t* alphabet, size_t* nbits) {
    auto hc = code_hcodes_[bits];
    TEST_AND_RETURN_FALSE(hc & 0x8000);
    *alphabet = hc & 0x7FFF;
    *nbits = code_lens_[*alphabet];
    return true;
  }

  // Returns the alphabet associated with the set of input bits for the
  // literal/length code length array.
  //
  // |bits|     IN   The input Huffman bits read from the deflate stream.
  // |alphabet| OUT  The alphabet associated with the given |bits|.
  // |nbits|    OUT  The number of bits in the Huffman code of the |alphabet|.
  // Returns true if there is an alphabet associated with |bits|.
  inline bool LitLenAlphabet(uint32_t bits, uint16_t* alphabet, size_t* nbits) {
    auto hc = lit_len_hcodes_[bits];
    TEST_AND_RETURN_FALSE(hc & 0x8000);
    *alphabet = hc & 0x7FFF;
    *nbits = lit_len_lens_[*alphabet];
    return true;
  }

  // Returns the alphabet associated with the set of input bits for the
  // distance code length array.
  //
  // |bits|     IN   The input Huffman bits read from the deflate stream.
  // |alphabet| OUT  The alphabet associated with the given |bits|.
  // |nbits|    OUT  The number of bits in the Huffman code of the |alphabet|.
  // Returns true if there is an alphabet associated with |bits|.
  inline bool DistanceAlphabet(uint32_t bits,
                               uint16_t* alphabet,
                               size_t* nbits) {
    auto hc = distance_hcodes_[bits];
    TEST_AND_RETURN_FALSE(hc & 0x8000);
    *alphabet = hc & 0x7FFF;
    *nbits = distance_lens_[*alphabet];
    return true;
  }

  // Returns the Huffman code of a give alphabet for Huffman table codes.
  //
  // |alphabet| IN   The alphabet.
  // |huffman|  OUT  The Huffman code for |alphabet|.
  // |nbits|    OUT  The maximum number of bits in the Huffman code of the
  //                 |alphabet|.
  inline bool CodeHuffman(uint16_t alphabet, uint16_t* huffman, size_t* nbits) {
    TEST_AND_RETURN_FALSE(alphabet < code_lens_.size());
    *huffman = code_rcodes_[alphabet];
    *nbits = code_lens_[alphabet];
    return true;
  }

  // Returns the Huffman code of a give alphabet for literal/length codes.
  //
  // |alphabet| IN   The alphabet.
  // |huffman|  OUT  The Huffman code for |alphabet|.
  // |nbits|    OUT  The maximum number of bits in the Huffman code of the
  //                 |alphabet|.
  inline bool LitLenHuffman(uint16_t alphabet,
                            uint16_t* huffman,
                            size_t* nbits) {
    TEST_AND_RETURN_FALSE(alphabet < lit_len_lens_.size());
    *huffman = lit_len_rcodes_[alphabet];
    *nbits = lit_len_lens_[alphabet];
    return true;
  }

  inline bool EndOfBlockBitLength(size_t* nbits) {
    TEST_AND_RETURN_FALSE(256 < lit_len_lens_.size());
    *nbits = lit_len_lens_[256];
    return true;
  }

  // Returns the Huffman code of a give alphabet for distance codes.
  //
  // |alphabet| IN   The alphabet.
  // |huffman|  OUT  The Huffman code for |alphabet|.
  // |nbits|    OUT  The maximum number of bits in the Huffman code of the
  //                 |alphabet|.
  inline bool DistanceHuffman(uint16_t alphabet,
                              uint16_t* huffman,
                              size_t* nbits) {
    TEST_AND_RETURN_FALSE(alphabet < distance_lens_.size());
    *huffman = distance_rcodes_[alphabet];
    *nbits = distance_lens_[alphabet];
    return true;
  }

  // This populates the object with fixed huffman table parameters.
  // TODO(ahassani): Revamp the use of this function to be initiliazed once in
  // the lifetime of the program and only one instance needed.
  bool BuildFixedHuffmanTable();

  // This functions first reads the Huffman code length arrays from the input
  // deflate stream, then builds both literal/length and distance Huffman
  // code arrays. It also writes the Huffman table into the puffed stream.
  //
  // |br|      IN      The |BitReaderInterface| reading the deflate stream.
  // |buffer|  OUT     The object to write the Huffman table.
  // |length|  IN/OUT  The length available in the |buffer| and in return it
  //                   will be the length of Huffman table data written into
  //                   the |buffer|.
  bool BuildDynamicHuffmanTable(BitReaderInterface* br,
                                uint8_t* buffer,
                                size_t* length);

  // This functions first reads the Huffman code length arrays from the input
  // puffed |buffer|, then builds both literal/length and distance Huffman code
  // arrays. It also writes the coded Huffman table arrays into the deflate
  // stream.
  //
  // |buffer| IN      The array to read the Huffman table from.
  // |length| IN      The length available in the |buffer|.
  // |bw|     IN/OUT  The |BitWriterInterface| for writing into the deflate
  //                  stream.
  bool BuildDynamicHuffmanTable(const uint8_t* buffer,
                                size_t length,
                                BitWriterInterface* bw);

 protected:
  // Initializes the Huffman codes from an array of lengths.
  //
  // |lens|     IN   The input array of code lengths.
  // |max_bits| OUT  The maximum number of bits used for the Huffman codes.
  bool InitHuffmanCodes(const Buffer& lens, size_t* max_bits);

  // Creates the Huffman code to alphabet array.
  // |lens|     IN   The input array of code lengths.
  // |hcodes|   OUT  The Huffman to alphabet array.
  // |max_bits| OUT  The maximum number of bits used for the Huffman codes.
  bool BuildHuffmanCodes(const Buffer& lens,
                         std::vector<uint16_t>* hcodes,
                         size_t* max_bits);

  // Creates the alphabet to Huffman code array.
  // |lens|     IN   The input array of code lengths.
  // |rcodes|   OUT  The Huffman to Huffman array.
  // |max_bits| OUT  The maximum number of bits used for the Huffman codes.
  bool BuildHuffmanReverseCodes(const Buffer& lens,
                                std::vector<uint16_t>* rcodes,
                                size_t* max_bits);

  // Reads a specific Huffman code length array from input. At the same time
  // writes the array into the puffed stream. The Huffman code length array is
  // either the literal/lengths or distance codes.
  //
  // |br|        IN      The |BitReaderInterface| for reading the deflate
  //                      stream.
  // |buffer|    OUT     The array to write the Huffman table.
  // |length|    IN/OUT  The length available in the |buffer| and in return it
  //                     will be the length of data written into the |buffer|.
  // |max_bits|  IN      The maximum number of bits in the Huffman codes.
  // |num_codes| IN      The size of the Huffman code length array in the input.
  // |lens|      OUT     The resulting Huffman code length array.
  bool BuildHuffmanCodeLengths(BitReaderInterface* br,
                               uint8_t* buffer,
                               size_t* length,
                               size_t max_bits,
                               size_t num_codes,
                               Buffer* lens);

  // Similar to |BuildHuffmanCodeLengths| but for reading from puffed buffer and
  // writing into deflate stream.
  //
  // |buffer|    IN      The array to read the Huffman table from.
  // |length|    IN      The length available in the |buffer|.
  // |bw|        IN/OUT  The |BitWriterInterface| for writing into the deflate
  //                     stream.
  // |num_codes| IN      Number of Huffman code lengths to read from the
  //                     |buffer|.
  // |lens|      OUT     The Huffman code lengths array.
  bool BuildHuffmanCodeLengths(const uint8_t* buffer,
                               size_t* length,
                               BitWriterInterface* bw,
                               size_t num_codes,
                               Buffer* lens);

 private:
  // A utility struct used to create Huffman codes.
  struct CodeIndexPair {
    uint16_t code;   // The Huffman code
    uint16_t index;  // The alphabet
  };
  // A vector with maximum size of 286 that is only uses as temporary space for
  // building Huffman codes.
  std::vector<CodeIndexPair> codeindexpairs_;

  // Used in building Huffman codes for literals/lengths and distances.
  std::vector<uint8_t> lit_len_lens_;
  std::vector<uint16_t> lit_len_hcodes_;
  std::vector<uint16_t> lit_len_rcodes_;
  size_t lit_len_max_bits_;
  std::vector<uint8_t> distance_lens_;
  std::vector<uint16_t> distance_hcodes_;
  std::vector<uint16_t> distance_rcodes_;
  size_t distance_max_bits_;

  // The reason for keeping a temporary buffer here is to avoid reallocing each
  // time.
  std::vector<uint8_t> tmp_lens_;

  // Used in building Huffman codes for reading and decoding literal/length and
  // distance Huffman code length arrays.
  std::vector<uint8_t> code_lens_;
  std::vector<uint16_t> code_hcodes_;
  std::vector<uint16_t> code_rcodes_;
  size_t code_max_bits_;

  bool initialized_{false};

  DISALLOW_COPY_AND_ASSIGN(HuffmanTable);
};

// The type of a block in a deflate stream.
enum class BlockType : uint8_t {
  kUncompressed = 0x00,
  kFixed = 0x01,
  kDynamic = 0x02,
};

std::string BlockTypeToString(BlockType type);

}  // namespace puffin

#endif  // SRC_HUFFMAN_TABLE_H_
