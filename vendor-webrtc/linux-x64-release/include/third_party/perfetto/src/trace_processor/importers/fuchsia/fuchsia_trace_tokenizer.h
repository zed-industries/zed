/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FUCHSIA_FUCHSIA_TRACE_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FUCHSIA_FUCHSIA_TRACE_TOKENIZER_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>

#include "perfetto/base/status.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"
#include "src/trace_processor/importers/fuchsia/fuchsia_record.h"
#include "src/trace_processor/importers/proto/proto_trace_reader.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/sched_tables_py.h"
#include "src/trace_processor/util/trace_type.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

// The Fuchsia trace format is documented at
// https://fuchsia.googlesource.com/fuchsia/+/HEAD/docs/development/tracing/trace-format/README.md
class FuchsiaTraceTokenizer : public ChunkedTraceReader {
 public:
  static constexpr TraceType kTraceType = TraceType::kFuchsiaTraceType;
  explicit FuchsiaTraceTokenizer(TraceProcessorContext*);
  ~FuchsiaTraceTokenizer() override;

  // ChunkedTraceReader implementation
  base::Status Parse(TraceBlobView) override;
  base::Status NotifyEndOfFile() override;

 private:
  struct ProviderInfo {
    std::string name;

    std::unordered_map<uint64_t, StringId> string_table;
    std::unordered_map<uint64_t, FuchsiaThreadInfo> thread_table;

    // Returns a StringId for the given FXT string ref id.
    StringId GetString(uint64_t string_ref) {
      auto search = string_table.find(string_ref);
      if (search != string_table.end()) {
        return search->second;
      }
      return kNullStringId;
    }

    // Returns a FuchsiaThreadInfo for the given FXT thread ref id.
    FuchsiaThreadInfo GetThread(uint64_t thread_ref) {
      auto search = thread_table.find(thread_ref);
      if (search != thread_table.end()) {
        return search->second;
      }
      return {0, 0};
    }

    uint64_t ticks_per_second = 1000000000;
  };

  void ParseRecord(TraceBlobView);
  void RegisterProvider(uint32_t, std::string);

  TraceProcessorContext* const context_;
  std::vector<uint8_t> leftover_bytes_;

  // Proto reader creates state that the blobs it emits reference, so the
  // proto_reader needs to live for as long as the tokenizer.
  ProtoTraceReader proto_reader_;
  std::vector<uint8_t> proto_trace_data_;

  std::unordered_map<uint32_t, std::unique_ptr<ProviderInfo>> providers_;
  ProviderInfo* current_provider_;

  // Interned string ids for record arguments.
  StringId process_id_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FUCHSIA_FUCHSIA_TRACE_TOKENIZER_H_
