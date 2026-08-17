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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_TO_FTRACE_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_TO_FTRACE_H_

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/string_writer.h"
#include "src/trace_processor/perfetto_sql/intrinsics/functions/sql_function.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class SystraceSerializer {
 public:
  using ScopedCString = std::unique_ptr<char, void (*)(void*)>;

  explicit SystraceSerializer(TraceProcessorContext* context);

  ScopedCString SerializeToString(uint32_t raw_row);

 private:
  using StringIdMap =
      base::FlatHashMap<StringId, std::vector<std::optional<uint32_t>>>;

  void SerializePrefix(uint32_t raw_row, base::StringWriter* writer);

  StringIdMap proto_id_to_arg_index_by_event_;
  const TraceStorage* storage_ = nullptr;
  TraceProcessorContext* context_ = nullptr;
};

struct ToFtrace : public SqlFunction {
  struct Context {
    const TraceStorage* storage;
    SystraceSerializer serializer;
  };

  static base::Status Run(Context*,
                          size_t argc,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors& destructors);
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_TO_FTRACE_H_
