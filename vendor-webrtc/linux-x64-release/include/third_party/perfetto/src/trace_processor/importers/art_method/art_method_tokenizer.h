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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ART_METHOD_ART_METHOD_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ART_METHOD_ART_METHOD_TOKENIZER_H_

#include <cstddef>
#include <cstdint>
#include <limits>
#include <optional>
#include <string>
#include <string_view>
#include <variant>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"
#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/trace_blob_view_reader.h"

namespace perfetto::trace_processor::art_method {

class ArtMethodTokenizer : public ChunkedTraceReader {
 public:
  explicit ArtMethodTokenizer(TraceProcessorContext*);
  ~ArtMethodTokenizer() override;

  base::Status Parse(TraceBlobView) override;
  base::Status NotifyEndOfFile() override;

 private:
  using Iterator = util::TraceBlobViewReader::Iterator;
  struct Method {
    StringId name;
    std::optional<StringId> pathname;
    std::optional<uint32_t> line_number;
  };
  struct Thread {
    StringId comm;
    bool comm_used;
  };
  struct Detect {};
  struct NonStreaming {
    base::Status Parse();
    base::Status NotifyEndOfFile() const;

    base::StatusOr<bool> ParseHeaderStart(Iterator&);
    base::StatusOr<bool> ParseHeaderVersion(Iterator&);
    base::StatusOr<bool> ParseHeaderOptions(Iterator&);
    base::StatusOr<bool> ParseHeaderThreads(Iterator&);
    base::StatusOr<bool> ParseHeaderMethods(Iterator&);
    base::StatusOr<bool> ParseDataHeader(Iterator&);

    base::Status ParseHeaderSectionLine(std::string_view);

    ArtMethodTokenizer* tokenizer_;
    enum {
      kHeaderStart,
      kHeaderVersion,
      kHeaderOptions,
      kHeaderThreads,
      kHeaderMethods,
      kDataHeader,
      kData,
    } mode_ = kHeaderStart;
  };
  struct Streaming {
    base::Status Parse();
    base::Status NotifyEndOfFile();

    base::StatusOr<bool> ParseHeaderStart(Iterator&);
    base::StatusOr<bool> ParseData(Iterator&);
    base::Status ParseSummary(std::string_view) const;

    ArtMethodTokenizer* tokenizer_;
    enum {
      kHeaderStart,
      kData,
      kSummaryDone,
      kDone,
    } mode_ = kHeaderStart;
    size_t it_offset_ = 0;
  };
  using SubParser = std::variant<Detect, NonStreaming, Streaming>;

  [[nodiscard]] base::Status ParseMethodLine(std::string_view);
  [[nodiscard]] base::Status ParseOptionLine(std::string_view);
  [[nodiscard]] base::Status ParseThread(uint32_t tid, const std::string&);
  [[nodiscard]] base::Status ParseRecord(uint32_t tid, const TraceBlobView&);

  TraceProcessorContext* const context_;
  util::TraceBlobViewReader reader_;

  SubParser sub_parser_;
  enum {
    kWall,
    kDual,
  } clock_ = kWall;

  uint32_t version_ = std::numeric_limits<uint32_t>::max();
  int64_t ts_ = std::numeric_limits<int64_t>::max();
  uint32_t record_size_ = std::numeric_limits<uint32_t>::max();
  base::FlatHashMap<uint32_t, Method> method_map_;
  base::FlatHashMap<uint32_t, Thread> thread_map_;
};

}  // namespace perfetto::trace_processor::art_method

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ART_METHOD_ART_METHOD_TOKENIZER_H_
