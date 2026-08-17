// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_BIT_READER_H_
#define SRC_BIT_READER_H_

#include <cstddef>
#include <cstdint>

#include "puffin/src/include/puffin/common.h"

namespace puffin {

// An abstract class for reading bits from a deflate stream. It can be used
// either for the beginning of the deflate stream or for any place inside the
// deflate stream. For more information on the pattern of reading, refer to
// RFC1951 in https://www.ietf.org/rfc/rfc1951.txt
class BitReaderInterface {
 public:
  virtual ~BitReaderInterface() = default;

  // Caches at least |nbits| starting from the next available bit (next bit that
  // will be read in |ReadBits|) in the cache. The maximum of number of bits
  // that can be cached is implementation dependent.
  //
  // |nbits| IN  The number of bits to see if available in the input.
  virtual bool CacheBits(size_t nbits) = 0;

  // Reads |nbits| from the cached input. Users should call |CacheBits| with
  // greater than or equal to |nbits| bits before calling this function.
  //
  // |nbits| IN  The number of bits to read from the cache.
  // Returns the read bits as an unsigned integer.
  virtual uint32_t ReadBits(size_t nbits) = 0;

  // Drops |nbits| from the input cache. Users should be careful that |nbits|
  // does not exceed the number of bits in the cache.
  //
  // |nbits| IN  The number of bits to drop from the cache.
  virtual void DropBits(size_t nbits) = 0;

  // TODO(*): Add ReadAndDropBits(uint32_t nbits); Because it is a common
  // pattern.

  // Returns an unsigned byte equal to the unread bits in the first cached
  // byte. This function should not advance the bit pointer in any way. A call
  // to |SkipBoundaryBits| should do the advancement.
  virtual uint8_t ReadBoundaryBits() = 0;

  // Moves the current bit pointer to the beginning of the next byte and returns
  // the number of bits skipped.
  virtual size_t SkipBoundaryBits() = 0;

  // Populates a function that allows reading from the byte that has the next
  // avilable bit for reading. This function clears all the bits that have been
  // cached previously. As a consequence the next |CacheBits| starts reading
  // from a byte boundary. The returned functin can only read |length| bytes. It
  // might be necessary to call |ReadBoundaryBits| and |SkipBoundaryBits| before
  // this function.
  virtual bool GetByteReaderFn(
      size_t length,
      std::function<bool(uint8_t* buffer, size_t count)>* read_fn) = 0;

  // Returns the number of bytes read till now. This size includes the last
  // partially read byte.
  virtual size_t Offset() const = 0;

  // Returns the number of bits read (dropped) till now.
  virtual uint64_t OffsetInBits() const = 0;

  // Returns the number of bits remaining to be cached.
  virtual uint64_t BitsRemaining() const = 0;
};

// A raw buffer implementation of |BitReaderInterface|.
class BufferBitReader : public BitReaderInterface {
 public:
  // Sets the beginning of the buffer that the users wants to read.
  //
  // |in_buf|  IN  The input buffer
  // |in_size| IN  The size of the input buffer
  BufferBitReader(const uint8_t* in_buf, size_t in_size)
      : in_buf_(in_buf), in_size_(in_size) {}

  ~BufferBitReader() override = default;

  // Can only cache up to 32 bits.
  bool CacheBits(size_t nbits) override;
  uint32_t ReadBits(size_t nbits) override;
  void DropBits(size_t nbits) override;
  uint8_t ReadBoundaryBits() override;
  size_t SkipBoundaryBits() override;
  bool GetByteReaderFn(
      size_t length,
      std::function<bool(uint8_t* buffer, size_t count)>* read_fn) override;
  size_t Offset() const override;
  uint64_t OffsetInBits() const override;
  uint64_t BitsRemaining() const override;

 private:
  const uint8_t* in_buf_;    // The input buffer.
  uint64_t in_size_;         // The number of bytes in |in_buf_|.
  uint64_t index_{0};        // The index to the next byte to be read.
  uint32_t in_cache_{0};     // The temporary buffer to put input data into.
  size_t in_cache_bits_{0};  // The number of bits available in |in_cache_|.

  DISALLOW_COPY_AND_ASSIGN(BufferBitReader);
};

}  // namespace puffin

#endif  // SRC_BIT_READER_H_
