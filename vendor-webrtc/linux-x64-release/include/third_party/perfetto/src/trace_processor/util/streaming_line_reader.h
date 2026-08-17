/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_UTIL_STREAMING_LINE_READER_H_
#define SRC_TRACE_PROCESSOR_UTIL_STREAMING_LINE_READER_H_

#include <cstddef>
#include <functional>
#include <vector>

#include "perfetto/ext/base/string_view.h"

namespace perfetto::trace_processor::util {

// A streaming line tokenizer for efficiently processing large text files on a
// line-by-line basis. It's designed to be used in conjunction with ZipReader to
// stream lines out of a compressed file (think of a bugreport) without having
// to decompress the whole file in memory upfront.
// Internally it deals with the necessary buffering and line-merging across
// different chunks.
// Usage:
// - The caller should pass a callback into the ctor. The callback is invoked
//   whenever a batch of lines has been tokenized. This happens after calls to
//   either BeginWrite()+EndWrite() or Tokenize(). In order to avoid too much
//   virtual dispatch overhead, the callback argument is a vector of lines, not
//   a single line.
// - The caller can call either:
//   - Tokenize(whole input): this exist to avoid a copy in the case of
//     non-compressed (STORE) files in zip archive.
//   - A sequence of BeginWrite() + EndWrite() as follows:
//     - BeginWrite(n) guarantees that the caller can write at least `n` char.
//       `n` is typically the decompression buffer passed to zlib.
//     - The caller writes at most `n` bytes into the pointer returned above.
//     - The caller calls EndWrite(m) passing the number of bytes actually
//       written (`m` <= `n`);
// NOTE:
// This implementation slightly diverges from base::StringSplitter as follows:
// 1. It does NOT skip empty lines. SS coalesces empty tokens, this doesn't.
// 2. it won't output the last line unless it terminates with a \n. SS doesn't
//    tell the difference between "foo\nbar" and "foo\nbar\n". This is
//    fundamental for streaming, where we cannot tell upfront if we got the end.
class StreamingLineReader {
 public:
  // Note: the lifetime of the lines passed in the vector argument is valid only
  // for the duration of the callback. Don't retain the StringView(s) passed.
  using LinesCallback =
      std::function<void(const std::vector<base::StringView>&)>;

  explicit StreamingLineReader(LinesCallback);
  ~StreamingLineReader();

  // This can be used when the whole input is known upfront and we just need
  // splitting. This exist mostly for convenience when processing uncompressed
  // (STORE) files in zip archives. If you just need a tokenizer outside of the
  // context of a zip file, you are better off just using base::StringSplitter.
  size_t Tokenize(base::StringView input);

  // Reserves `write_buf_size` bytes into the internal buffer. The caller is
  // expected to write at most `write_buf_size` on the returned pointer and
  // then call EndWrite().
  char* BeginWrite(size_t write_buf_size);

  // Finishes the write reporting the number of bytes actually written, which
  // must be <= `write_buf_size`. If one or more lines can be tokenized, this
  // will cause one or more calls to the LinesCallback.
  void EndWrite(size_t size_written);

 private:
  std::vector<char> buf_;
  LinesCallback lines_callback_;
  size_t size_before_write_ = 0;
};

}  // namespace perfetto::trace_processor::util

#endif  // SRC_TRACE_PROCESSOR_UTIL_STREAMING_LINE_READER_H_
