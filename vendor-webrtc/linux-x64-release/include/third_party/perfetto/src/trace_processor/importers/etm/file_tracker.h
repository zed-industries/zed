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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_FILE_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_FILE_TRACKER_H_

#include <optional>
#include <string>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/etm_tables_py.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor::etm {

class FileTracker : public Destructible {
 public:
  static FileTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->file_tracker) {
      context->file_tracker.reset(new FileTracker(context));
    }
    return static_cast<FileTracker*>(context->file_tracker.get());
  }

  ~FileTracker() override;

  base::Status AddFile(const std::string& name, TraceBlobView data);
  TraceBlobView GetContent(tables::FileTable::Id id) const {
    PERFETTO_DCHECK(id.value < file_content_.size());
    return file_content_[id.value].copy();
  }

 private:
  explicit FileTracker(TraceProcessorContext* context) : context_(context) {}
  void IndexFileType(tables::FileTable::Id file_id,
                     const TraceBlobView& content);
  TraceProcessorContext* const context_;
  // Indexed by `tables::FileTable::Id`
  std::vector<TraceBlobView> file_content_;

  base::FlatHashMap<StringId, tables::FileTable::Id> files_by_path_;
};

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_FILE_TRACKER_H_
