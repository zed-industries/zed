/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_CHUNKED_LINE_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_CHUNKED_LINE_READER_H_

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"

namespace perfetto ::trace_processor {

// Adapter on top of `ChunkedTraceReader` that performs line by line parsing.
class ChunkedLineReader : public ChunkedTraceReader {
 public:
  base::Status Parse(TraceBlobView) final;
  base::Status NotifyEndOfFile() final;

  // Called for each line in the input. Each line is terminated by a '\n'
  // character. The new line character will be included the `line`.
  virtual base::Status ParseLine(base::StringView line) = 0;

  // Similar to `NotifyEndOfFile` but this also provides any leftovers. That
  // would heppen if the last line in a stream is not terminated by the newline
  // character.
  virtual void EndOfStream(base::StringView leftovers) = 0;

 private:
  base::StatusOr<TraceBlobView> SpliceLoop(TraceBlobView data);
  base::Status OnLine(const TraceBlobView& data);

  // Buffers any leftovers from a previous call to `Parse`;
  TraceBlobView buffer_;
};

}  // namespace perfetto::trace_processor
#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_CHUNKED_LINE_READER_H_
