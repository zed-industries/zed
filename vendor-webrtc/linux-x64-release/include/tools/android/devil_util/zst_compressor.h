// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ANDROID_DEVIL_UTIL_ZST_COMPRESSOR_H_
#define TOOLS_ANDROID_DEVIL_UTIL_ZST_COMPRESSOR_H_

#include <ostream>

// Forward declarations.
struct ZSTD_inBuffer_s;
typedef ZSTD_inBuffer_s ZSTD_inBuffer;
struct ZSTD_outBuffer_s;
typedef ZSTD_outBuffer_s ZSTD_outBuffer;
struct ZSTD_CCtx_s;
typedef ZSTD_CCtx_s ZSTD_CCtx;

class ZstCompressor {
 public:
  // The user is responsible for closing the output_stream if necessary.
  explicit ZstCompressor(std::ostream& output_stream, int compression_level);
  ~ZstCompressor();

  // Get the size of input buffer (which will be passed to CompressStreaming())
  // that is recommended by zstd. It is OK to use a different input buffer size.
  size_t GetRecommendedInputBufferSize();

  struct UncompressedContent {
    char* buffer;
    size_t size;
  };

  // Compress the input and write the result to output_stream. Repeatedly call
  // this function until you have passed everything that need to be compressed.
  // If this is the last chunk of input that needs to be compressed, set
  // last_chunk to true, otherwise set last_chunk to false.
  // The user is responsible for freeing the input.buffer if needed.
  void CompressStreaming(UncompressedContent input, bool last_chunk);

 private:
  // The output stream where we are writing the compressed content to.
  std::ostream& output_stream_;
  size_t input_buffer_recommended_size_;
  // A struct containing the input buffer, the size of the contents of input
  // buffer, and the position where the zstd library function stopped reading.
  // The position field is updated by the zstd library functions.
  ZSTD_inBuffer* input_struct_;
  // A buffer where the compressed content is placed into.
  char* output_buffer_;
  // The size of the output buffer. Indicates how large the buffer is.
  size_t output_buffer_size_;
  // A struct containing the output buffer, the size of the output buffer,
  // and the position where the zstd library function stopped writing.
  // The position field is updated by the zstd library functions.
  ZSTD_outBuffer* output_struct_;
  // A context object needed by the zstd library functions.
  ZSTD_CCtx* ctx_;
};

#endif  // TOOLS_ANDROID_DEVIL_UTIL_ZST_COMPRESSOR_H_
