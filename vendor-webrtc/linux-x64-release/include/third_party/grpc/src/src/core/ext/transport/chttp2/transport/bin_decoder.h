//
//
// Copyright 2016 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_BIN_DECODER_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_BIN_DECODER_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

struct grpc_base64_decode_context {
  // input/output:
  const uint8_t* input_cur;
  const uint8_t* input_end;
  uint8_t* output_cur;
  uint8_t* output_end;
  // Indicate if the decoder should handle the tail of input data
  bool contains_tail;
};

// base64 decode a grpc_base64_decode_context util either input_end is reached
// or output_end is reached. When input_end is reached, (input_end - input_cur)
// is less than 4. When output_end is reached, (output_end - output_cur) is less
// than 3. Returns false if decoding is failed.
bool grpc_base64_decode_partial(struct grpc_base64_decode_context* ctx);

// base64 decode a slice with pad chars. Returns a new slice, does not take
// ownership of the input. Returns an empty slice if decoding is failed.
grpc_slice grpc_chttp2_base64_decode(const grpc_slice& input);

// base64 decode a slice without pad chars, data length is needed. Returns a new
// slice, does not take ownership of the input. Returns an empty slice if
// decoding is failed.
grpc_slice grpc_chttp2_base64_decode_with_length(const grpc_slice& input,
                                                 size_t output_length);

// Infer the length of decoded data from encoded data.
size_t grpc_chttp2_base64_infer_length_after_decode(const grpc_slice& slice);

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_BIN_DECODER_H
