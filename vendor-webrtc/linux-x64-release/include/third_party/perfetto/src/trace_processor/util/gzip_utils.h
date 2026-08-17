/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_UTIL_GZIP_UTILS_H_
#define SRC_TRACE_PROCESSOR_UTIL_GZIP_UTILS_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

#include "perfetto/base/build_config.h"

struct z_stream_s;

namespace perfetto::trace_processor::util {

// Returns whether gzip related functioanlity is supported with the current
// build flags.
constexpr bool IsGzipSupported() {
#if PERFETTO_BUILDFLAG(PERFETTO_ZLIB)
  return true;
#else
  return false;
#endif
}

// Usage: To decompress in a streaming way, there are two ways of using it:
// 1. [Commonly used] - Feed the sequence of mem-blocks in 'FeedAndExtract' one
//    by one. Output will be produced in given output_consumer, which is simply
//    a callback. On each 'FeedAndExtract', output_consumer could get invoked
//    any number of times, based on how much partial output is available.

// 2. [Uncommon ; Discouraged] - Feed the sequence of mem-blocks one by one, by
//    calling 'Feed'. For each time 'Feed' is called, client should call
//    'ExtractOutput' again and again to extract the partially available output,
//    until there in no more output to extract. Also see 'ResultCode' enum.
class GzipDecompressor {
 public:
  enum class ResultCode {
    // 'kOk' means nothing bad happened so far, but continue doing what you
    // were doing.
    kOk,
    // While calling 'ExtractOutput' repeatedly, if we get 'kEof', it means
    // we have extracted all the partially available data and we are also
    // done, i.e. there is no need to feed more input.
    kEof,
    // Some error. Possibly invalid compressed stream or corrupted data.
    kError,
    // While calling 'ExtractOutput' repeatedly, if we get 'kNeedsMoreInput',
    // it means we have extracted all the partially available data, but we are
    // not done yet. We need to call the 'Feed' to feed the next input
    // mem-block and go through the ExtractOutput loop again.
    kNeedsMoreInput,
  };
  struct Result {
    // The return code of the decompression.
    ResultCode ret;

    // The amount of bytes written to output.
    // Valid in all cases except |ResultCode::kError|.
    size_t bytes_written;
  };
  enum class InputMode {
    // The input stream contains a gzip header. This is for the common case of
    // decompressing .gz files.
    kGzip = 0,

    // A raw deflate stream. This is for the case of uncompressing files from
    // a .zip archive, where the compression type is specified in the zip file
    // entry, rather than in the stream header.
    kRawDeflate = 1,
  };

  explicit GzipDecompressor(InputMode = InputMode::kGzip);

  // Feed the next mem-block.
  void Feed(const uint8_t* data, size_t size);

  // Feed the next mem-block and extract output in the callback consumer.
  // callback can get invoked multiple times if there are multiple
  // mem-blocks to output.
  //
  // Note the output of this function is guaranteed *not* to be kOk.
  template <typename Callback = void(const uint8_t* ptr, size_t size)>
  ResultCode FeedAndExtract(const uint8_t* data,
                            size_t size,
                            const Callback& output_consumer) {
    Feed(data, size);
    uint8_t buffer[4096];
    Result result;
    do {
      result = ExtractOutput(buffer, sizeof(buffer));
      if (result.ret != ResultCode::kError && result.bytes_written > 0) {
        output_consumer(buffer, result.bytes_written);
      }
    } while (result.ret == ResultCode::kOk);
    return result.ret;
  }

  // Extract the newly available partial output. On each 'Feed', this method
  // should be called repeatedly until there is no more data to output
  // i.e. (either 'kEof' or 'kNeedsMoreInput').
  Result ExtractOutput(uint8_t* out, size_t out_capacity);

  // Sets the state of the decompressor to reuse with other gzip streams.
  // This is almost like constructing a new 'GzipDecompressor' object
  // but without paying the cost of internal memory allocation.
  void Reset();

  // Decompress the entire mem-block and return decompressed mem-block.
  // This is used for decompressing small strings or small files
  // which doesn't require streaming decompression.
  static std::vector<uint8_t> DecompressFully(const uint8_t* data, size_t len);

  // Returns the amount of input bytes left unprocessed.
  size_t AvailIn() const;

 private:
  struct Deleter {
    void operator()(z_stream_s*) const;
  };
  std::unique_ptr<z_stream_s, Deleter> z_stream_;
};

}  // namespace perfetto::trace_processor::util

#endif  // SRC_TRACE_PROCESSOR_UTIL_GZIP_UTILS_H_
