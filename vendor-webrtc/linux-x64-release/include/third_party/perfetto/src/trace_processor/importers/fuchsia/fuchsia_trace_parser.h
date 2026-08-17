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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FUCHSIA_FUCHSIA_TRACE_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FUCHSIA_FUCHSIA_TRACE_PARSER_H_

#include <cstdint>
#include <functional>
#include <optional>
#include <unordered_map>
#include <vector>

#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/importers/fuchsia/fuchsia_record.h"
#include "src/trace_processor/importers/fuchsia/fuchsia_trace_utils.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/sched_tables_py.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class FuchsiaTraceParser : public FuchsiaRecordParser {
 public:
  explicit FuchsiaTraceParser(TraceProcessorContext*);
  ~FuchsiaTraceParser() override;

  // Tracks the state for updating sched slice and thread state tables.
  struct Thread {
    explicit Thread(uint64_t tid) : info{0, tid} {}

    FuchsiaThreadInfo info;
    int64_t last_ts{0};
    std::optional<tables::SchedSliceTable::RowNumber> last_slice_row;
    std::optional<tables::ThreadStateTable::RowNumber> last_state_row;
  };

  void ParseFuchsiaRecord(int64_t timestamp, FuchsiaRecord fr) override;

  // Allocates or returns an existing Thread instance for the given tid.
  Thread& GetThread(uint64_t tid) {
    auto search = threads_.find(tid);
    if (search != threads_.end()) {
      return search->second;
    }
    auto result = threads_.emplace(tid, tid);
    return result.first->second;
  }

  struct Arg {
    StringId name;
    fuchsia_trace_utils::ArgValue value;
  };

  // Utility to parse record arguments. Exposed here to provide consistent
  // parsing between trace parsing and tokenization.
  //
  // Returns an empty optional on error, otherwise a vector containing zero or
  // more arguments.
  static std::optional<std::vector<Arg>> ParseArgs(
      fuchsia_trace_utils::RecordCursor& cursor,
      uint32_t n_args,
      std::function<StringId(base::StringView string)> intern_string,
      std::function<StringId(uint32_t index)> get_string);

 private:
  void SwitchFrom(Thread* thread,
                  int64_t ts,
                  uint32_t cpu,
                  uint32_t thread_state);
  void SwitchTo(Thread* thread, int64_t ts, uint32_t cpu, int32_t weight);
  void Wake(Thread* thread, int64_t ts, uint32_t cpu);

  StringId IdForOutgoingThreadState(uint32_t state);

  TraceProcessorContext* const context_;

  // Interned string ids for record arguments.
  const StringId weight_id_;
  const StringId incoming_weight_id_;
  const StringId outgoing_weight_id_;

  // Interned string ids for the relevant thread states.
  const StringId running_string_id_;
  const StringId runnable_string_id_;
  const StringId waking_string_id_;
  const StringId blocked_string_id_;
  const StringId suspended_string_id_;
  const StringId exit_dying_string_id_;
  const StringId exit_dead_string_id_;

  // Map from tid to Thread.
  std::unordered_map<uint64_t, Thread> threads_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FUCHSIA_FUCHSIA_TRACE_PARSER_H_
