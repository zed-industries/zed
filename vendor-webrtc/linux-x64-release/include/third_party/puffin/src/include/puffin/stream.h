// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_INCLUDE_PUFFIN_STREAM_H_
#define SRC_INCLUDE_PUFFIN_STREAM_H_

#include <stddef.h>

#include <memory>

#include "puffin/common.h"

namespace puffin {

// The base stream interface used by Puffin for all operations. This interface
// is designed to be as simple as possible.
class StreamInterface {
 public:
  virtual ~StreamInterface() = default;

  // Returns the size of the stream.
  virtual bool GetSize(uint64_t* size) = 0;

  // Returns the current offset in the stream where next read or write will
  // happen.
  virtual bool GetOffset(uint64_t* offset) = 0;

  // Sets the offset in the stream for the next read or write. On error
  // returns |false|.
  virtual bool Seek(uint64_t offset) = 0;

  // Reads |length| bytes of data into |buffer|. On error, returns |false|.
  virtual bool Read(void* buffer, size_t length) = 0;

  // Writes |length| bytes of data into |buffer|. On error, returns |false|.
  virtual bool Write(const void* buffer, size_t length) = 0;

  // Closes the stream and cleans up all associated resources. On error, returns
  // |false|.
  virtual bool Close() = 0;
};

using UniqueStreamPtr = std::unique_ptr<StreamInterface>;

}  // namespace puffin

#endif  // SRC_INCLUDE_PUFFIN_STREAM_H_
