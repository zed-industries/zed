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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_SLICE_TRANSLATION_TABLE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_SLICE_TRANSLATION_TABLE_H_

#include <cstdint>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto {
namespace trace_processor {

// Tracks and stores slice translation rules. It allows Trace Processor
// to for example deobfuscate slice names.
class SliceTranslationTable {
 public:
  SliceTranslationTable(TraceStorage* storage);

  // If the name is not mapped to anything, assumes that no translation is
  // necessry, and returns the raw_name.
  StringId TranslateName(StringId raw_name) const {
    const auto* mapped_name = raw_to_deobfuscated_name_.Find(raw_name);
    return mapped_name ? *mapped_name : raw_name;
  }

  void AddNameTranslationRule(base::StringView raw,
                              base::StringView deobfuscated) {
    const StringId raw_id = storage_->InternString(raw);
    const StringId deobfuscated_id = storage_->InternString(deobfuscated);
    raw_to_deobfuscated_name_[raw_id] = deobfuscated_id;
  }

 private:
  TraceStorage* storage_;
  base::FlatHashMap<StringId, StringId> raw_to_deobfuscated_name_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_SLICE_TRANSLATION_TABLE_H_
