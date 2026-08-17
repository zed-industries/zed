// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_PUFF_READER_H_
#define SRC_PUFF_READER_H_

#include <cstddef>
#include <cstdint>

#include "puffin/src/include/puffin/common.h"
#include "puffin/src/puff_data.h"

namespace puffin {

// An abstract class for reading data from a puffed buffer. Data can be
// literals, lengths, distances, or metadata. Extensions of this class can
// define how the puffed data should reside in the puffed buffer.
class PuffReaderInterface {
 public:
  virtual ~PuffReaderInterface() = default;

  // Retrieves the next puff data available in the puffed buffer. Similar to
  // |PuffWriterInterface.Insert()| This function does not check for validity of
  // data.
  //
  // |data|  OUT  The next data available in the puffed buffer.
  virtual bool GetNext(PuffData* data) = 0;

  // Returns the number of bytes left in the puff buffer.
  virtual size_t BytesLeft() const = 0;
};

class BufferPuffReader : public PuffReaderInterface {
 public:
  // Sets the parameters of puff buffer.
  //
  // |puff_buf|  IN  The input puffed stream. It is owned by the caller and must
  //                 be valid during the lifetime of the object.
  // |puff_size| IN  The size of the puffed stream.
  BufferPuffReader(const uint8_t* puff_buf, size_t puff_size)
      : puff_buf_in_(puff_buf), puff_size_(puff_size) {}

  ~BufferPuffReader() override = default;

  bool GetNext(PuffData* pd) override;
  size_t BytesLeft() const override;

 private:
  // The pointer to the puffed stream. This should not be deallocated.
  const uint8_t* puff_buf_in_;

  // The size of the puffed buffer.
  size_t puff_size_;

  // Index to the offset of the next data in the puff buffer.
  size_t index_{0};

  // State when reading from the puffed buffer.
  enum class State {
    kReadingLenDist = 0,
    kReadingBlockMetadata,
  } state_{State::kReadingBlockMetadata};

  DISALLOW_COPY_AND_ASSIGN(BufferPuffReader);
};

}  // namespace puffin

#endif  // SRC_PUFF_READER_H_
