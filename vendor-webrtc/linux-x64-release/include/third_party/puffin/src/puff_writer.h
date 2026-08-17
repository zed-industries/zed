// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_PUFF_WRITER_H_
#define SRC_PUFF_WRITER_H_

#include <cstddef>
#include <cstdint>

#include "puffin/src/include/puffin/common.h"
#include "puffin/src/puff_data.h"

namespace puffin {

// An abstract class for writing data into a puffed buffer. Data can be
// literals, lengths, distances, or metadata. Extensions of this class can
// define how the puffed data should reside in the puffed buffer.
class PuffWriterInterface {
 public:
  virtual ~PuffWriterInterface() = default;

  // Inserts data. This function does not need to check for the validity of data
  // . e.g. length > 285, etc.
  //
  // |pd|          IN   The data to put into the puffed buffer. |pd.type|
  //                    defines the type of the data.
  // Returns false if it fails.
  virtual bool Insert(const PuffData& pd) = 0;

  // Fluesh any buffer or internal state to the output.
  // Returns false if it fails.
  virtual bool Flush() = 0;

  // Returns the number of bytes processed and written into the puff buffer.
  virtual size_t Size() = 0;
};

class BufferPuffWriter : public PuffWriterInterface {
 public:
  // Sets the parameters of puff buffer.
  //
  // |puff_buf|  IN  The input puffed stream. It is owned by the caller and must
  //                 be valid during the lifetime of the object.
  // |puff_size| IN  The size of the puffed stream.
  BufferPuffWriter(uint8_t* puff_buf, size_t puff_size)
      : puff_buf_out_(puff_buf), puff_size_(puff_size) {}

  ~BufferPuffWriter() override = default;

  bool Insert(const PuffData& pd) override;
  bool Flush() override;
  size_t Size() override;

 private:
  // Flushes the literals into the output and resets the state.
  bool FlushLiterals();

  // The pointer to the puffed stream. This should not be deallocated.
  uint8_t* puff_buf_out_;

  // The size of the puffed buffer.
  size_t puff_size_;

  // The offset to the next data in the buffer.
  size_t index_{0};

  // Marks where the length of data should be written after the |index_| has
  // moved forward.
  size_t len_index_{0};

  // The number of literals currently been written (or cached).
  size_t cur_literals_length_{0};

  // States when writing into the puffed buffer.
  enum class State {
    kWritingNonLiteral = 0,
    kWritingSmallLiteral,
    kWritingLargeLiteral,
  } state_{State::kWritingNonLiteral};

  DISALLOW_COPY_AND_ASSIGN(BufferPuffWriter);
};

}  // namespace puffin

#endif  // SRC_PUFF_WRITER_H_
