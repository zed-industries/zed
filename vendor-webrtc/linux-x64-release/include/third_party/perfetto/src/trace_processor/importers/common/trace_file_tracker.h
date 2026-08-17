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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACE_FILE_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACE_FILE_TRACKER_H_

#include <string>
#include <vector>

#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/util/trace_type.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

// This class keeps track of the file currently being parsed and metadata about
// it. Files can be nested into other files (zip or gzip files) and this class
// also keeps track of those relations.
class TraceFileTracker {
 public:
  explicit TraceFileTracker(TraceProcessorContext* context)
      : context_(context) {}

  tables::TraceFileTable::Id AddFile(const std::string& name);
  tables::TraceFileTable::Id AddFile() { return AddFileImpl(kNullStringId); }
  void SetSize(tables::TraceFileTable::Id id, uint64_t size);
  void StartParsing(tables::TraceFileTable::Id id, TraceType trace_type);
  void DoneParsing(tables::TraceFileTable::Id id, size_t size);

 private:
  tables::TraceFileTable::Id AddFileImpl(StringId name);

  TraceProcessorContext* const context_;
  size_t processing_order_ = 0;
  std::vector<tables::TraceFileTable::Id> parsing_stack_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACE_FILE_TRACKER_H_
