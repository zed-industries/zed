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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ELF_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ELF_TRACKER_H_

#include <optional>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/tables/etm_tables_py.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/build_id.h"

namespace perfetto::trace_processor::etm {

class ElfTracker : public Destructible {
 public:
  static ElfTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->elf_tracker) {
      context->elf_tracker.reset(new ElfTracker(context));
    }
    return static_cast<ElfTracker*>(context->elf_tracker.get());
  }

  ~ElfTracker() override;

  bool ProcessFile(tables::FileTable::Id file_id, const TraceBlobView& content);
  std::optional<tables::ElfFileTable::Id> FindBuildId(
      const BuildId& build_id) const {
    auto it = files_by_build_id_.Find(build_id);
    return it ? std::make_optional(*it) : std::nullopt;
  }

 private:
  explicit ElfTracker(TraceProcessorContext* context) : context_(context) {}
  TraceProcessorContext* context_;
  base::FlatHashMap<BuildId, tables::ElfFileTable::Id> files_by_build_id_;
};

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ELF_TRACKER_H_
