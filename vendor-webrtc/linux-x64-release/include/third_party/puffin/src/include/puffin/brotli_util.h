// Copyright 2021 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_INCLUDE_PUFFIN_BROTLI_UTIL_H_
#define SRC_INCLUDE_PUFFIN_BROTLI_UTIL_H_

#include <stdint.h>

#include <vector>

#include "puffin/stream.h"

namespace puffin {

// Use brotli to compress |input_size| bytes of data, and write the result to
// |output_stream|.
bool BrotliEncode(const uint8_t* input,
                  size_t input_size,
                  UniqueStreamPtr output_stream);
// Similar to above function, and writes to a output buffer.
bool BrotliEncode(const uint8_t* input,
                  size_t input_size,
                  std::vector<uint8_t>* output);

// Similar to the above, but quality controls how well the compression is
// |quality| should be between 0 and 11
bool BrotliEncode(const uint8_t* input,
                  size_t input_size,
                  UniqueStreamPtr output_stream,
                  int quality);

// Decompress |input_size| bytes of data with brotli, and write the result to
// |output_stream|.
bool BrotliDecode(const uint8_t* input,
                  size_t input_size,
                  UniqueStreamPtr output_stream);
// Similar to above function, and writes to a output buffer.
bool BrotliDecode(const uint8_t* input,
                  size_t input_size,
                  std::vector<uint8_t>* output);
}  // namespace puffin

#endif  // SRC_INCLUDE_PUFFIN_BROTLI_UTIL_H_
