// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_BIT_WRITER_H_
#define SRC_BIT_WRITER_H_

#include <cstddef>
#include <cstdint>

#include "puffin/src/include/puffin/common.h"

namespace puffin {
// An abstract class for writing bits into a deflate stream. For more
// information on the pattern of writing, refer to RFC1951 at
// https://www.ietf.org/rfc/rfc1951.txt
class BitWriterInterface {
 public:
  virtual ~BitWriterInterface() = default;

  // Puts least significant |nbits| bits of |bits| into the cache and flush it
  // if necessary. If it returns false (e.g. not enough output buffer), then it
  // may write part of |bits| into the output which will be unknown.
  //
  // |nbits| IN  The number of bits to write in the output.
  // |bits|  IN  The bit values to write into the output.
  virtual bool WriteBits(size_t nbits, uint32_t bits) = 0;

  // It first flushes the cache and then puts the |nbytes| bytes from |buffer|
  // into the output buffer. User should make sure there that the number of bits
  // written into the |BitWriter| before this call is a multiplication of
  // eight. Otherwise it is errornous. This can be achieved by calling
  // |WriteBoundaryBits| or |WriteBits| (if the user is tracking the number of
  // bits written).
  //
  // |nbytes|  IN  The number of bytes to read using |read_fn| and write into
  //               the output.
  // |read_fn| IN  A function to read bytes from.
  virtual bool WriteBytes(
      size_t nbytes,
      const std::function<bool(uint8_t* buffer, size_t count)>& read_fn) = 0;

  // Puts enough least-significant bits from |bits| into output until the
  // beginning of the next Byte is reached. The number of bits to write into
  // output will be determined by how many bits are needed to reach the
  // boundary.
  //
  // |bits| IN  The value of boundary bits.
  virtual bool WriteBoundaryBits(uint8_t bits) = 0;

  // Flushes the cache into the output buffer. It writes 0 for extra bits that
  // comes without data at the end.
  //
  // Returns false if it fails to flush.
  virtual bool Flush() = 0;

  // Returns the number of bytes written to the ouput including the cached
  // bytes.
  virtual size_t Size() const = 0;
};

// A raw buffer implementation of |BitWriterInterface|.
class BufferBitWriter : public BitWriterInterface {
 public:
  // Sets the beginning of the buffer that the users wants to write into.
  //
  // |out_buf|  IN  The output buffer
  // |out_size| IN  The size of the output buffer
  BufferBitWriter(uint8_t* out_buf, size_t out_size)
      : out_buf_(out_buf), out_size_(out_size) {}

  ~BufferBitWriter() override = default;

  bool WriteBits(size_t nbits, uint32_t bits) override;
  bool WriteBytes(size_t nbytes,
                  const std::function<bool(uint8_t* buffer, size_t count)>&
                      read_fn) override;
  bool WriteBoundaryBits(uint8_t bits) override;
  bool Flush() override;
  size_t Size() const override;

 private:
  // The output buffer.
  uint8_t* out_buf_;

  // The number of bytes in |out_buf_|.
  uint64_t out_size_;

  // The index to the next byte to write into.
  uint64_t index_{0};

  // A temporary buffer to keep the bits going out.
  uint32_t out_holder_{0};

  // The number of bits in |out_holder_|.
  uint8_t out_holder_bits_{0};

  DISALLOW_COPY_AND_ASSIGN(BufferBitWriter);
};

}  // namespace puffin

#endif  // SRC_BIT_WRITER_H_
